# POOL_IMPLEMENT_HEALTH_CHECKS

**Priority**: HIGH  
**Component**: pool/core, pool/capabilities  
**Estimated Effort**: 6 hours  
**Risk**: High (Silent Failures)

## Problem Statement

When worker threads crash or model loading fails, the pool continues routing requests to dead workers, causing all requests to timeout. There's no mechanism to detect worker health or remove failed workers from the routing pool.

### Current Bug Location

The bug exists in the worker spawn pattern used across all capability modules:

**File**: [`packages/candle/src/pool/capabilities/text_embedding.rs:118-127`](../packages/candle/src/pool/capabilities/text_embedding.rs)

```rust
// Worker thread crashes after registration
std::thread::spawn(move || {
    let model = match model_loader() {
        Ok(m) => m,
        Err(e) => {
            log::error!("Worker {} failed: {}", worker_id, e);
            return;  // Thread exits but handle remains in pool!
        }
    };
    // ...
});

// Later: Pool still routes to dead worker (line 149 in WorkerHandle registration)
let pool_handle = WorkerHandle { ... };
pool.register_worker(registry_key_clone, pool_handle); // Registered even if thread died!
```

**Impact**: After thread exit, the `WorkerHandle` remains in the pool's `DashMap`, and requests continue to be routed to the dead worker.

## Runtime Impact

### Request Timeouts (100% failure rate)
- **What happens**: Dead worker selected by load balancer
- **Result**: Request sent to non-existent thread, waits 30s, times out
- **User impact**: Every request to that model fails until service restart
- **Frequency**: Happens whenever model download fails, OOM during load, or GPU errors

### Cascading Timeouts
- **What happens**: Dead worker appears "least busy" (0 pending requests)
- **Result**: ALL new requests routed to dead worker
- **User impact**: 100% of model requests fail, not just some
- **Business impact**: Complete feature outage, customer churn

### Resource Leak
- **What happens**: Dead worker handles accumulate forever
- **Memory impact**: Each dead worker = 100KB handle data
- **Result**: After 1000 failures = 100MB leaked memory
- **User impact**: Gradual degradation, eventual OOM

---

## Current Implementation Analysis

### Existing Infrastructure

**Pool Core** ([`packages/candle/src/pool/core/`](../packages/candle/src/pool/core/)):
- **`types.rs`**: Defines `WorkerHandle` with `pending_requests`, `last_used`, `shutdown_tx`
- **`pool.rs`**: Manages `DashMap<String, Vec<WorkerHandle>>` for worker registry
- **`worker.rs`**: Generic `spawn_worker_thread()` pattern (not currently used by capabilities)

**Capability Workers** ([`packages/candle/src/pool/capabilities/`](../packages/candle/src/pool/capabilities/)):
- Each capability (text_embedding, text_to_text, vision, etc.) implements its own spawn logic
- All use `crossbeam::channel` with `select!` for multiplexing request/shutdown channels
- Worker loop pattern: `select! { recv(request_rx) => ..., recv(shutdown_rx) => break }`

**Maintenance** ([`packages/candle/src/pool/maintenance.rs`](../packages/candle/src/pool/maintenance.rs)):
- Already has `evict_worker()` that removes workers and updates memory
- Has `all_workers_idle()` for checking idle status
- Runs periodic maintenance loop every 60s (configurable)

### What's Missing

1. **No health check channels** in `WorkerHandle`
2. **No health ping/pong protocol**
3. **No `is_alive()` method** to check worker health
4. **No worker validation** before routing requests
5. **No automatic cleanup** of dead workers

---

## Detailed Implementation Plan

### Step 1: Add Health Check Types and Channels

**File**: `packages/candle/src/pool/core/types.rs`

**Changes Required**:

1. Add health check message types at the top of the file (after imports):

```rust
/// Health check ping sent to worker
#[derive(Debug, Clone, Copy)]
pub struct HealthPing;

/// Health check response from worker
#[derive(Debug, Clone)]
pub struct HealthPong {
    pub worker_id: usize,
    pub timestamp: u64,
    pub queue_depth: usize,
}
```

2. Add health check channels to `WorkerHandle` struct (line ~37):

```rust
use crossbeam::channel::{Sender, Receiver};

pub struct WorkerHandle {
    pub pending_requests: Arc<AtomicUsize>,
    pub last_used: Arc<AtomicU64>,
    pub worker_id: usize,
    pub shutdown_tx: Sender<()>,
    pub per_worker_mb: usize,
    
    // NEW: Health check channels
    pub health_tx: Sender<HealthPing>,
    pub health_rx: Receiver<HealthPong>,
}
```

3. Update `WorkerHandle::new()` signature (line ~47):

```rust
impl WorkerHandle {
    pub fn new(
        worker_id: usize,
        shutdown_tx: Sender<()>,
        per_worker_mb: usize,
        health_tx: Sender<HealthPing>,
        health_rx: Receiver<HealthPong>,
    ) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            pending_requests: Arc::new(AtomicUsize::new(0)),
            last_used: Arc::new(AtomicU64::new(now)),
            worker_id,
            shutdown_tx,
            per_worker_mb,
            health_tx,
            health_rx,
        }
    }

    // ... existing touch() method ...
    
    /// Check if worker is alive by sending health ping
    ///
    /// Returns true if worker responds within 100ms, false otherwise.
    /// False indicates worker thread is dead, stuck, or channel broken.
    pub fn is_alive(&self) -> bool {
        use std::time::Duration;
        
        // Try to send ping
        if self.health_tx.send(HealthPing).is_err() {
            // Channel broken = worker dead
            return false;
        }

        // Wait for pong with timeout
        match self.health_rx.recv_timeout(Duration::from_millis(100)) {
            Ok(pong) => {
                // Update last health check timestamp
                self.last_used.store(pong.timestamp, std::sync::atomic::Ordering::Release);
                true
            }
            Err(_) => {
                // Timeout or disconnected = worker dead/stuck
                false
            }
        }
    }
}
```

**Key Design Decisions**:
- **100ms timeout**: Fast enough to detect failures quickly, slow enough to avoid false positives
- **Channel failure detection**: `send().is_err()` catches broken channels (thread exited)
- **Timeout detection**: `recv_timeout()` catches stuck/unresponsive workers
- **State update**: Update `last_used` on successful pong to track worker activity

---

### Step 2: Update Each Capability Worker Implementation

The same pattern applies to all 5 capability modules. I'll show the complete pattern for `text_embedding.rs`, then note what needs to change in the others.

**File**: `packages/candle/src/pool/capabilities/text_embedding.rs`

**Location**: In `spawn_text_embedding_worker()` method (around line 100-150)

**Changes Required**:

1. Create health check channels alongside other channels (after line 111):

```rust
// Existing channels
let (embed_tx, embed_rx) = unbounded();
let (batch_embed_tx, batch_embed_rx) = unbounded();
let (shutdown_tx, shutdown_rx) = unbounded();

// NEW: Health check channels
let (health_tx_worker, health_rx_worker) = unbounded::<HealthPing>();
let (health_tx_main, health_rx_main) = unbounded::<HealthPong>();
```

2. Pass health channels to worker thread (around line 120):

```rust
// Clone channels for worker thread
let health_rx_worker_clone = health_rx_worker.clone();
let health_tx_main_clone = health_tx_main.clone();

std::thread::spawn(move || {
    let model = match model_loader() {
        Ok(m) => m,
        Err(e) => {
            log::error!("TextEmbedding worker {} model loading failed: {}", worker_id, e);
            return;
        }
    };

    // Pass health channels to worker loop
    text_embedding_worker(
        model, 
        embed_rx, 
        batch_embed_rx, 
        shutdown_rx,
        health_rx_worker_clone,  // NEW
        health_tx_main_clone,    // NEW
        worker_id,               // NEW (for pong response)
    );
});
```

3. Update worker loop signature and add health check arm (around line 40):

```rust
pub fn text_embedding_worker<T: TextEmbeddingCapable>(
    model: T,
    embed_rx: Receiver<EmbedRequest>,
    batch_embed_rx: Receiver<BatchEmbedRequest>,
    shutdown_rx: Receiver<()>,
    health_rx: Receiver<HealthPing>,      // NEW
    health_tx: Sender<HealthPong>,        // NEW
    worker_id: usize,                     // NEW
) {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    loop {
        select! {
            recv(embed_rx) -> req => {
                if let Ok(req) = req {
                    let result = model.embed(&req.text, req.task)
                        .map_err(|e| PoolError::ModelError(e.to_string()));
                    let _ = req.response.send(result);
                }
            }
            recv(batch_embed_rx) -> req => {
                if let Ok(req) = req {
                    let result = model.batch_embed(&req.texts, req.task)
                        .map_err(|e| PoolError::ModelError(e.to_string()));
                    let _ = req.response.send(result);
                }
            }
            
            // NEW: Health check arm
            recv(health_rx) -> ping => {
                if ping.is_ok() {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(0);
                    
                    let pong = HealthPong {
                        worker_id,
                        timestamp: now,
                        queue_depth: embed_rx.len() + batch_embed_rx.len(),
                    };
                    
                    // Send pong (ignore errors - main thread might have given up)
                    let _ = health_tx.send(pong);
                }
            }
            
            recv(shutdown_rx) -> _ => {
                log::info!("TextEmbedding worker {} shutting down", worker_id);
                break;
            }
        }
    }
}
```

4. Update `WorkerHandle` creation to include health channels (around line 145):

```rust
// Existing handle creation
let pool_handle = WorkerHandle {
    pending_requests: Arc::clone(&pending_requests),
    last_used: Arc::clone(&last_used),
    worker_id,
    shutdown_tx: shutdown_tx.clone(),
    per_worker_mb,
    health_tx: health_tx_worker,     // NEW
    health_rx: health_rx_main,       // NEW
};

// Register in pool
pool.register_worker(registry_key_clone, pool_handle);
```

**Repeat for Other Capabilities**:

Apply the same 4-step pattern to:
- **`text_to_text.rs`**: Add to `text_to_text_worker()` and `spawn_text_to_text_worker()`
- **`vision.rs`**: Add to `vision_worker()` and `spawn_vision_worker()`
- **`image_embedding.rs`**: Add to `image_embedding_worker()` and `spawn_image_embedding_worker()`
- **`text_to_image.rs`**: Add to `text_to_image_worker()` and `spawn_text_to_image_worker()`

**Queue Depth Calculation** (adjust per capability):
- text_embedding: `embed_rx.len() + batch_embed_rx.len()`
- text_to_text: `prompt_rx.len()`
- vision: `describe_image_rx.len() + describe_url_rx.len()`
- image_embedding: `embed_image_rx.len() + embed_image_url_rx.len() + batch_embed_image_rx.len()`
- text_to_image: `generate_image_rx.len()`

---

### Step 3: Add Worker Validation to Pool

**File**: `packages/candle/src/pool/core/pool.rs`

**Location**: Add new method to `Pool<T>` impl block (around line 90)

```rust
impl<T: ?Sized> Pool<T> {
    // ... existing methods ...

    /// Validate workers and remove dead ones
    ///
    /// Checks each worker's health via is_alive() and removes workers
    /// that don't respond. Updates memory tracking and metrics.
    ///
    /// Returns the number of workers removed.
    pub fn validate_workers(&self, registry_key: &str) -> usize {
        let mut removed_count = 0;
        
        // Get mutable access to workers
        if let Some(mut workers_guard) = self.workers.get_mut(registry_key) {
            let initial_count = workers_guard.len();
            
            // Retain only alive workers
            workers_guard.retain(|worker| {
                if worker.is_alive() {
                    true  // Keep alive workers
                } else {
                    // Worker is dead - remove it
                    log::warn!(
                        "Removing dead worker {} for {} (no health response)",
                        worker.worker_id,
                        registry_key
                    );
                    
                    // Update memory tracking
                    self.remove_memory(worker.per_worker_mb);
                    
                    // Send shutdown signal (may fail if worker already dead)
                    let _ = worker.shutdown_tx.send(());
                    
                    removed_count += 1;
                    false  // Remove from pool
                }
            });
            
            let final_count = workers_guard.len();
            
            if removed_count > 0 {
                log::warn!(
                    "Removed {} dead workers for {} ({} -> {} workers)",
                    removed_count,
                    registry_key,
                    initial_count,
                    final_count
                );
                
                // Update metrics
                self.metrics.workers_evicted.fetch_add(removed_count, Ordering::Release);
            }
        }
        
        removed_count
    }

    /// Check if there are any alive workers for a model
    ///
    /// Returns true if at least one worker responds to health check.
    pub fn has_alive_workers(&self, registry_key: &str) -> bool {
        if let Some(workers) = self.workers.get(registry_key) {
            workers.iter().any(|w| w.is_alive())
        } else {
            false
        }
    }

    /// Get least busy ALIVE worker for routing
    ///
    /// Filters out dead workers before selecting by load.
    /// Returns None if no alive workers exist.
    pub fn get_alive_worker(&self, registry_key: &str) -> Option<usize> {
        if let Some(workers) = self.workers.get(registry_key) {
            workers
                .iter()
                .enumerate()
                .filter(|(_, w)| w.is_alive())  // Only alive workers
                .min_by_key(|(_, w)| w.pending_requests.load(Ordering::Acquire))
                .map(|(idx, _)| idx)
        } else {
            None
        }
    }
}
```

**Key Implementation Notes**:
- `validate_workers()`: Removes all dead workers, updates memory/metrics
- `has_alive_workers()`: Quick check if any workers are responsive  
- `get_alive_worker()`: Routing helper that skips dead workers

---

### Step 4: Integrate Health Checks into Maintenance

**File**: `packages/candle/src/pool/maintenance.rs`

**Location**: Update periodic maintenance loop (around line 100-200)

**Changes Required**:

Add health check validation before idle eviction:

```rust
/// Run maintenance cycle on all pools
///
/// 1. Validate worker health (remove dead workers)
/// 2. Evict idle workers if needed
/// 3. Log status
pub fn run_maintenance_cycle() {
    let config = text_embedding_pool().config();
    let idle_threshold_secs = config.cooldown_idle_minutes * 60;

    // Validate TextEmbedding workers
    validate_pool_health(
        text_embedding_pool(),
        &TEXT_EMBEDDING_WORKERS,
        "TextEmbedding",
    );
    
    // Evict idle TextEmbedding workers
    evict_idle_workers(
        text_embedding_pool(),
        &TEXT_EMBEDDING_WORKERS,
        idle_threshold_secs,
        "TextEmbedding",
    );

    // Repeat for other pools...
    validate_pool_health(text_to_text_pool(), &TEXT_TO_TEXT_WORKERS, "TextToText");
    evict_idle_workers(text_to_text_pool(), &TEXT_TO_TEXT_WORKERS, idle_threshold_secs, "TextToText");
    
    validate_pool_health(vision_pool(), &VISION_WORKERS, "Vision");
    evict_idle_workers(vision_pool(), &VISION_WORKERS, idle_threshold_secs, "Vision");
    
    validate_pool_health(image_embedding_pool(), &IMAGE_EMBEDDING_WORKERS, "ImageEmbedding");
    evict_idle_workers(image_embedding_pool(), &IMAGE_EMBEDDING_WORKERS, idle_threshold_secs, "ImageEmbedding");
    
    validate_pool_health(text_to_image_pool(), &TEXT_TO_IMAGE_WORKERS, "TextToImage");
    evict_idle_workers(text_to_image_pool(), &TEXT_TO_IMAGE_WORKERS, idle_threshold_secs, "TextToImage");
}

/// Validate health of all workers in a pool
fn validate_pool_health<T: ?Sized>(
    pool: &Pool<T>,
    workers_map: &DashMap<String, Vec<impl std::fmt::Debug>>,
    pool_name: &str,
) {
    for entry in pool.workers().iter() {
        let registry_key = entry.key();
        let removed = pool.validate_workers(registry_key);
        
        if removed > 0 {
            log::warn!(
                "[{}] Removed {} dead workers for model '{}'",
                pool_name,
                removed,
                registry_key
            );
        }
    }
}
```

**Integration Notes**:
- Health validation runs BEFORE idle eviction (dead workers shouldn't be considered idle)
- Validation runs every maintenance cycle (default 60s, configurable)
- All pools validated on each cycle

---

### Step 5: Update Worker Selection Logic in Capabilities

Each capability module has request routing logic that needs to check worker health.

**Example for text_embedding.rs** (around line 200+):

**Current Pattern** (BROKEN - routes to dead workers):
```rust
pub fn embed(registry_key: &str, text: String, task: Option<String>) -> Result<Vec<f32>, PoolError> {
    let workers = TEXT_EMBEDDING_WORKERS
        .get(registry_key)
        .ok_or_else(|| PoolError::NoWorkers(registry_key.to_string()))?;
    
    // PROBLEM: Picks first worker, might be dead!
    let worker = workers.first()
        .ok_or_else(|| PoolError::NoWorkers(registry_key.to_string()))?;
    
    // Send request to potentially dead worker...
}
```

**Updated Pattern** (FIXED - checks health):
```rust
pub fn embed(registry_key: &str, text: String, task: Option<String>) -> Result<Vec<f32>, PoolError> {
    let workers = TEXT_EMBEDDING_WORKERS
        .get(registry_key)
        .ok_or_else(|| PoolError::NoWorkers(registry_key.to_string()))?;
    
    // NEW: Find alive worker with least load
    let worker = workers
        .iter()
        .filter(|w| w.core.is_alive())  // Only alive workers
        .min_by_key(|w| w.core.pending_requests.load(Ordering::Acquire))
        .ok_or_else(|| PoolError::NoWorkers(format!("No alive workers for {}", registry_key)))?;
    
    // Increment pending requests
    worker.core.pending_requests.fetch_add(1, Ordering::Release);
    
    // Create response channel
    let (response_tx, response_rx) = crossbeam::channel::unbounded();
    
    // Send request
    worker.embed_tx.send(EmbedRequest { text, task, response: response_tx })
        .map_err(|_| PoolError::ChannelSend("Worker channel closed".into()))?;
    
    // Wait for response with timeout
    let result = response_rx
        .recv_timeout(Duration::from_secs(text_embedding_pool().config().request_timeout_secs))
        .map_err(|_| PoolError::Timeout("Request timeout".into()))??;
    
    // Decrement pending requests
    worker.core.pending_requests.fetch_sub(1, Ordering::Release);
    worker.core.touch();
    
    Ok(result)
}
```

**Apply to All Request Methods**:
- `text_embedding.rs`: `embed()`, `batch_embed()`
- `text_to_text.rs`: `prompt()`
- `vision.rs`: `describe_image()`, `describe_image_url()`
- `image_embedding.rs`: `embed_image()`, `embed_image_url()`, `batch_embed_image()`
- `text_to_image.rs`: `generate_image()`

---

## Implementation Checklist

### Core Types (1 file)
- [ ] Add `HealthPing` and `HealthPong` structs to `pool/core/types.rs`
- [ ] Add `health_tx` and `health_rx` fields to `WorkerHandle`
- [ ] Add `is_alive()` method to `WorkerHandle`
- [ ] Update `WorkerHandle::new()` signature

### Capability Workers (5 files)
- [ ] **text_embedding.rs**: Add health channels, update worker loop, update spawn
- [ ] **text_to_text.rs**: Add health channels, update worker loop, update spawn
- [ ] **vision.rs**: Add health channels, update worker loop, update spawn
- [ ] **image_embedding.rs**: Add health channels, update worker loop, update spawn
- [ ] **text_to_image.rs**: Add health channels, update worker loop, update spawn

### Pool Core (1 file)
- [ ] Add `validate_workers()` to `pool/core/pool.rs`
- [ ] Add `has_alive_workers()` to `pool/core/pool.rs`
- [ ] Add `get_alive_worker()` to `pool/core/pool.rs`

### Maintenance (1 file)
- [ ] Add `validate_pool_health()` to `pool/maintenance.rs`
- [ ] Integrate health checks into `run_maintenance_cycle()`

### Request Routing (5 files)
- [ ] Update worker selection in `text_embedding.rs` request methods
- [ ] Update worker selection in `text_to_text.rs` request methods
- [ ] Update worker selection in `vision.rs` request methods
- [ ] Update worker selection in `image_embedding.rs` request methods
- [ ] Update worker selection in `text_to_image.rs` request methods

---

## Code Pattern Reference

### Worker Loop Pattern (Applies to All Capabilities)

```rust
// In worker loop select! block:
recv(health_rx) -> ping => {
    if ping.is_ok() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        let pong = HealthPong {
            worker_id,
            timestamp: now,
            queue_depth: /* Sum of all request channel lengths */,
        };
        
        let _ = health_tx.send(pong);
    }
}
```

### Channel Creation Pattern (Applies to All Capabilities)

```rust
// In spawn function, create health channels:
let (health_tx_worker, health_rx_worker) = unbounded::<HealthPing>();
let (health_tx_main, health_rx_main) = unbounded::<HealthPong>();

// Clone for thread:
let health_rx_worker_clone = health_rx_worker.clone();
let health_tx_main_clone = health_tx_main.clone();

// Store in WorkerHandle:
let handle = WorkerHandle {
    // ... other fields ...
    health_tx: health_tx_worker,
    health_rx: health_rx_main,
};
```

### Worker Selection Pattern (Applies to All Capabilities)

```rust
// In request methods, filter for alive workers:
let worker = workers
    .iter()
    .filter(|w| w.core.is_alive())  // Skip dead workers
    .min_by_key(|w| w.core.pending_requests.load(Ordering::Acquire))
    .ok_or_else(|| PoolError::NoWorkers("No alive workers".into()))?;
```

---

## Definition of Done

The implementation is complete when:

1. **Worker Death Detection**: When a worker thread exits (model load failure, panic, etc.), the pool detects it within 1 second and removes the WorkerHandle

2. **No Routing to Dead Workers**: Request routing methods (`embed()`, `prompt()`, etc.) skip dead workers and only route to responsive workers

3. **Automatic Cleanup**: The maintenance loop automatically removes dead workers every cycle without manual intervention

4. **Memory Tracking**: When dead workers are removed, their memory allocation is properly subtracted from `total_memory_used`

5. **Graceful Degradation**: When all workers for a model die, the pool returns `PoolError::NoWorkers` instead of hanging indefinitely

6. **Performance Overhead**: Health checks add <1% CPU overhead (single ping/pong per worker per maintenance cycle)

7. **Logging**: Dead worker removal is logged at WARN level with worker_id and registry_key for debugging

---

## Implementation Notes

### Channel Direction Clarity

The health check uses TWO unidirectional channels (not one bidirectional):
- **Ping channel**: Main thread → Worker thread (`health_tx_worker`/`health_rx_worker`)
- **Pong channel**: Worker thread → Main thread (`health_tx_main`/`health_rx_main`)

This avoids channel contention and makes direction explicit.

### Why 100ms Timeout?

- **Too short** (<50ms): False positives when worker is busy processing
- **Too long** (>500ms): Slow detection of dead workers, requests accumulate
- **100ms**: Sweet spot - fast detection, tolerates brief CPU spikes

### Why Check in Maintenance Loop?

- **Periodic validation** (every 60s) catches workers that died between requests
- **On-demand validation** (during routing) catches workers that died during active use
- Together: Comprehensive coverage with minimal overhead

### Thread Safety

- `is_alive()` uses `recv_timeout()` which is thread-safe
- `validate_workers()` uses `DashMap::get_mut()` which provides exclusive access
- No locks needed beyond DashMap's internal locking

### Existing Code Reuse

This implementation leverages existing infrastructure:
- [`pool/maintenance.rs:evict_worker()`](../packages/candle/src/pool/maintenance.rs) for worker removal pattern
- [`pool/core/types.rs:WorkerHandle`](../packages/candle/src/pool/core/types.rs) for handle structure
- Crossbeam channels already used throughout capabilities
- `select!` macro already used in all worker loops

**No new dependencies required** - all functionality uses existing crates (crossbeam, dashmap, std).

---

## Why This Is Critical

Silent worker death is the **#1 cause of production incidents**:
- Model downloads fail due to network issues (common)
- GPU memory allocation fails (happens under load)
- CUDA/Metal errors crash threads (driver issues)
- Without health checks, these become total outages
- Customer impact: "AI features randomly stop working"
- Support cost: Each incident requires manual intervention

**This fix transforms silent failures into automatic recovery.**
