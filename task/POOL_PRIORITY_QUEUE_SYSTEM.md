# POOL_PRIORITY_QUEUE_SYSTEM

**Priority**: HIGH
**Component**: pool/core
**Estimated Effort**: 2 days
**Risk**: Medium
**Dependencies**: POOL_UNIFIED_STORAGE

## Problem Statement

Current implementation issues:
- Unbounded channels cause OOM under load
- No request prioritization (FIFO only)
- No request deduplication
- No deadline support
- No backpressure mechanism

## Solution Design

### Priority-Based Request Queue

```rust
// pool/core/queue.rs
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::time::Instant;
use crossbeam::channel::{bounded, Sender, Receiver};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Critical = 0,   // System health checks, auth
    High = 1,       // User-facing requests
    Normal = 2,     // Default priority
    Low = 3,        // Background tasks
    Batch = 4,      // Bulk operations
}

pub struct PriorityRequest<T> {
    pub id: u64,
    pub priority: Priority,
    pub payload: T,
    pub enqueued_at: Instant,
    pub deadline: Option<Instant>,
    pub response_tx: Sender<Result<Response, PoolError>>,
}

// Implement Ord for heap ordering (higher priority first)
impl<T> Ord for PriorityRequest<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
            .then_with(|| other.enqueued_at.cmp(&self.enqueued_at))
    }
}

pub struct PriorityQueue<T: Send> {
    heap: Arc<Mutex<BinaryHeap<PriorityRequest<T>>>>,
    capacity: usize,
    semaphore: Arc<Semaphore>,  // For backpressure
    metrics: QueueMetrics,
}
```

### Bounded Channel Replacement

Replace all unbounded channels with bounded + backpressure:

```rust
impl Pool<dyn TextEmbeddingCapable> {
    pub fn spawn_text_embedding_worker<T, F>(...) -> Result<(), PoolError> {
        // BEFORE: let (embed_tx, embed_rx) = unbounded();
        // AFTER: Bounded with configurable capacity
        let (embed_tx, embed_rx) = bounded(self.config.queue_capacity);
        let (batch_embed_tx, batch_embed_rx) = bounded(self.config.batch_queue_capacity);
        
        // Priority queue for high-priority requests
        let priority_queue = PriorityQueue::new(self.config.priority_queue_capacity);
        
        // ... rest of spawn logic
    }
}
```

### Request Deduplication

```rust
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use xxhash_rust::xxh3::Xxh3;

pub struct DedupCache<T: Hash> {
    cache: Arc<RwLock<HashMap<u64, DedupEntry<T>>>>,
    ttl: Duration,
}

struct DedupEntry<T> {
    request: Arc<T>,
    result: Option<Arc<Result<Response, PoolError>>>,
    waiters: Vec<Sender<Result<Response, PoolError>>>,
    created_at: Instant,
}

impl<T: Hash + Clone> DedupCache<T> {
    pub fn deduplicate(&self, request: &T) -> DedupResult {
        let hash = self.hash_request(request);
        
        let mut cache = self.cache.write();
        
        // Check if request exists and is still valid
        if let Some(entry) = cache.get_mut(&hash) {
            if entry.created_at.elapsed() < self.ttl {
                // Request in flight or cached
                if let Some(result) = &entry.result {
                    return DedupResult::Cached(result.clone());
                } else {
                    // Add to waiters
                    let (tx, rx) = bounded(1);
                    entry.waiters.push(tx);
                    return DedupResult::Waiting(rx);
                }
            }
        }
        
        // New request
        cache.insert(hash, DedupEntry {
            request: Arc::new(request.clone()),
            result: None,
            waiters: Vec::new(),
            created_at: Instant::now(),
        });
        
        DedupResult::New(hash)
    }
    
    fn hash_request(&self, request: &T) -> u64 {
        let mut hasher = Xxh3::new();
        request.hash(&mut hasher);
        hasher.finish()
    }
}
```

### Worker Loop with Priority

```rust
fn priority_aware_worker_loop<T: TextEmbeddingCapable>(
    model: T,
    normal_rx: Receiver<EmbedRequest>,
    priority_queue: Arc<PriorityQueue<EmbedRequest>>,
    shutdown_rx: Receiver<()>,
) {
    loop {
        // Check priority queue first
        if let Some(priority_req) = priority_queue.try_dequeue() {
            // Check deadline
            if let Some(deadline) = priority_req.deadline {
                if Instant::now() > deadline {
                    let _ = priority_req.response_tx.send(Err(PoolError::DeadlineExpired));
                    continue;
                }
            }
            
            // Process high-priority request
            let result = model.embed(&priority_req.payload.text, priority_req.payload.task);
            let _ = priority_req.response_tx.send(result.map_err(|e| PoolError::ModelError(e)));
            continue;
        }
        
        // Then check normal queue with timeout
        select! {
            recv(normal_rx) -> req => {
                if let Ok(req) = req {
                    let result = model.embed(&req.text, req.task);
                    let _ = req.response_tx.send(result);
                }
            }
            recv(shutdown_rx) -> _ => {
                break;
            }
            default(Duration::from_millis(10)) => {
                // Prevents busy waiting
            }
        }
    }
}
```

### Backpressure Mechanism

```rust
impl<T> Pool<T> {
    pub fn submit_request_with_backpressure(
        &self,
        request: Request,
        priority: Priority,
        timeout: Duration,
    ) -> Result<Response, PoolError> {
        // Acquire permit (blocks if at capacity)
        let _permit = self.request_semaphore
            .try_acquire_for(timeout)
            .map_err(|_| PoolError::QueueFull)?;
        
        // Check queue depth
        if self.get_queue_depth() > self.config.max_queue_depth {
            return Err(PoolError::Overloaded);
        }
        
        // Submit based on priority
        match priority {
            Priority::Critical | Priority::High => {
                self.priority_queue.enqueue(request, priority)
            }
            _ => {
                self.normal_queue.send_timeout(request, timeout)
                    .map_err(|_| PoolError::Timeout)
            }
        }
    }
}
```

### Request Coalescing

```rust
pub struct CoalescingBuffer<T> {
    buffer: Arc<RwLock<HashMap<String, Vec<T>>>>,
    flush_interval: Duration,
    max_batch_size: usize,
}

impl<T: Clone> CoalescingBuffer<T> {
    pub fn add(&self, key: String, request: T) -> CoalescingResult {
        let mut buffer = self.buffer.write();
        
        let batch = buffer.entry(key.clone()).or_insert_with(Vec::new);
        batch.push(request);
        
        if batch.len() >= self.max_batch_size {
            // Flush immediately
            let batch = buffer.remove(&key).unwrap();
            CoalescingResult::FlushNow(batch)
        } else {
            // Will flush on timer
            CoalescingResult::Buffered
        }
    }
    
    fn start_flush_timer(&self) {
        let buffer = self.buffer.clone();
        let interval = self.flush_interval;
        
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(interval);
                
                let mut buffer = buffer.write();
                for (_, batch) in buffer.drain() {
                    // Send batch to workers
                }
            }
        });
    }
}
```

## Implementation Steps

1. **Create queue.rs** with PriorityQueue implementation
2. **Replace unbounded channels** with bounded in all spawn methods
3. **Add DedupCache** to Pool struct
4. **Update worker loops** to handle priority
5. **Add backpressure semaphore** to Pool
6. **Implement request coalescing** for batch operations
7. **Add deadline checking** in worker loops
8. **Update metrics** to track queue depths and priorities

## Configuration

```rust
pub struct QueueConfig {
    pub queue_capacity: usize,              // Default: 1000
    pub batch_queue_capacity: usize,        // Default: 100
    pub priority_queue_capacity: usize,     // Default: 100
    pub max_queue_depth: usize,             // Default: 5000
    pub enable_deduplication: bool,         // Default: true
    pub dedup_ttl: Duration,               // Default: 100ms
    pub enable_coalescing: bool,           // Default: true
    pub coalesce_interval: Duration,       // Default: 50ms
    pub coalesce_max_batch: usize,         // Default: 32
}
```

## Acceptance Criteria

- [ ] All channels bounded with configurable capacity
- [ ] Priority queue implementation working
- [ ] Request deduplication eliminates duplicates
- [ ] Deadline support with automatic expiry
- [ ] Backpressure prevents OOM
- [ ] Coalescing batches similar requests
- [ ] Metrics track queue depths by priority
- [ ] No API changes for existing users

## Testing Strategy

1. **OOM Prevention Test**: Submit 1M requests, verify bounded memory
2. **Priority Test**: Verify high-priority processed first
3. **Deduplication Test**: Submit duplicates, verify single processing
4. **Deadline Test**: Submit with deadline, verify expiry
5. **Backpressure Test**: Overwhelm system, verify graceful degradation
6. **Coalescing Test**: Submit batchable requests, verify batching

## Performance Targets

- Queue operations: < 1μs
- Priority selection: O(log n)
- Dedup lookup: O(1) average
- Backpressure response: < 10ms
- Zero allocations in hot path

## Migration Guide

```rust
// Users can optionally specify priority
let embedding = pool.embed_with_priority(
    "text",
    Priority::High,
    Some(Duration::from_secs(5)), // deadline
)?;

// Default API unchanged
let embedding = pool.embed("text")?; // Uses Priority::Normal
```