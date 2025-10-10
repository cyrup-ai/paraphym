# POOL_UNIFIED_STORAGE

**Priority**: CRITICAL
**Component**: pool/core, pool/capabilities
**Estimated Effort**: 2-3 days
**Risk**: High (breaking change)
**Dependencies**: None

## Problem Statement

Current implementation has DUAL STORAGE causing memory leaks:
1. `Pool.workers`: DashMap<String, Vec<WorkerHandle>>
2. Global static DashMaps: TEXT_EMBEDDING_WORKERS, etc.

When workers are evicted, they're removed from Pool.workers but remain in global DashMaps forever. The Drop trait doesn't help because handles in global maps keep them alive.

## Solution Design

### Phase 1: Unified Worker Registry

Create a single, unified worker registry that eliminates dual storage:

```rust
// pool/core/registry.rs
pub struct UnifiedWorkerRegistry<C> {
    // Single storage location - no duplication
    workers: Arc<DashMap<String, WorkerGroup<C>>>,
}

pub struct WorkerGroup<C> {
    registry_key: String,
    workers: Vec<Worker<C>>,
    channels: ChannelGroup<C>,
    metrics: WorkerGroupMetrics,
}

pub struct Worker<C> {
    handle: WorkerHandle,
    channels: WorkerChannels<C>,
    state: Arc<AtomicU32>,
}
```

### Phase 2: Channel Management

Consolidate channel management into the Worker struct:

```rust
pub struct WorkerChannels<C> {
    // Capability-specific channels
    request_tx: Sender<C::Request>,
    response_rx: Receiver<C::Response>,
    
    // Common channels
    shutdown_tx: Sender<()>,
    health: HealthChannel,
}

// Capability trait for channel types
pub trait PoolCapability {
    type Request: Send + 'static;
    type Response: Send + 'static;
}
```

### Phase 3: Registry Integration

Replace all global statics with registry access:

```rust
impl Pool<dyn TextEmbeddingCapable> {
    fn get_worker(&self, registry_key: &str) -> Option<&Worker<TextEmbedding>> {
        self.registry.get_worker(registry_key)
    }
    
    fn embed_text(&self, registry_key: &str, text: &str) -> Result<Vec<f32>, PoolError> {
        let worker = self.get_worker(registry_key)?;
        // Direct channel access - no intermediate lookup
        worker.channels.request_tx.send(EmbedRequest { text })?;
        worker.channels.response_rx.recv_timeout(timeout)
    }
}
```

## Implementation Steps

### Step 1: Create UnifiedWorkerRegistry (pool/core/registry.rs)

```rust
use dashmap::DashMap;
use std::sync::Arc;

pub struct UnifiedWorkerRegistry<C: PoolCapability> {
    workers: Arc<DashMap<String, WorkerGroup<C>>>,
    
    pub fn register(&self, registry_key: String, worker: Worker<C>) {
        self.workers.entry(registry_key)
            .or_insert_with(|| WorkerGroup::new(registry_key))
            .workers.push(worker);
    }
    
    pub fn remove(&self, registry_key: &str, worker_id: usize) -> Option<Worker<C>> {
        self.workers.get_mut(registry_key)?
            .workers.remove(worker_id)
    }
    
    pub fn get_least_loaded(&self, registry_key: &str) -> Option<&Worker<C>> {
        let group = self.workers.get(registry_key)?;
        // Power of Two Choices selection
        select_worker_power_of_two(&group.workers)
    }
}
```

### Step 2: Update Pool struct (pool/core/pool.rs)

```rust
pub struct Pool<T: ?Sized> {
    // REMOVE: workers: DashMap<String, Vec<WorkerHandle>>,
    // ADD: Single unified registry
    registry: Arc<UnifiedWorkerRegistry<T::Capability>>,
    
    config: PoolConfig,
    total_memory_used: Arc<AtomicUsize>,
    metrics: PoolMetrics,
    shutting_down: Arc<AtomicBool>,
}
```

### Step 3: Remove Global DashMaps

DELETE these from all capability files:
- `static TEXT_EMBEDDING_WORKERS: Lazy<DashMap<...>>`
- `static TEXT_TO_TEXT_WORKERS: Lazy<DashMap<...>>`
- `static IMAGE_EMBEDDING_WORKERS: Lazy<DashMap<...>>`
- `static VISION_WORKERS: Lazy<DashMap<...>>`
- `static TEXT_TO_IMAGE_WORKERS: Lazy<DashMap<...>>`

### Step 4: Update spawn_worker methods

```rust
impl Pool<dyn TextEmbeddingCapable> {
    pub fn spawn_text_embedding_worker<T, F>(...) -> Result<(), PoolError> {
        // ... spawn thread ...
        
        let worker = Worker {
            handle: WorkerHandle { ... },
            channels: WorkerChannels { ... },
            state: Arc::new(AtomicU32::new(0)),
        };
        
        // Single registration point - no duplication
        self.registry.register(registry_key.to_string(), worker);
        
        // No more TEXT_EMBEDDING_WORKERS.insert()
    }
}
```

### Step 5: Update request routing

```rust
pub fn embed_text(&self, registry_key: &str, text: &str) -> Result<Vec<f32>, PoolError> {
    // Direct registry access - no global lookup
    let worker = self.registry.get_least_loaded(registry_key)
        .ok_or_else(|| PoolError::NoWorkers(registry_key.to_string()))?;
    
    // Direct channel access
    worker.channels.request_tx.send(EmbedRequest { text, response: tx })?;
    rx.recv_timeout(self.config.request_timeout)
}
```

## Acceptance Criteria

- [ ] UnifiedWorkerRegistry implemented with single storage point
- [ ] All global static DashMaps removed
- [ ] Pool uses registry for all worker operations
- [ ] No Drop trait needed - workers cleaned up on removal
- [ ] Zero duplicate storage locations
- [ ] All tests pass with no memory leaks
- [ ] Benchmarks show equal or better performance

## Testing Strategy

1. **Memory Leak Test**: Spawn/evict 1000 workers, verify memory returns to baseline
2. **Concurrent Access Test**: 100 threads accessing registry simultaneously
3. **Performance Test**: Measure lookup latency vs dual storage
4. **Integration Test**: Full request flow through unified registry

## Migration Guide

For existing code using the pool:

```rust
// Before (with memory leak):
let model = registry::get::<TextEmbeddingModel>("model")?;
let embedding = model.embed("text", None)?;

// After (no change to API):
let model = registry::get::<TextEmbeddingModel>("model")?;
let embedding = model.embed("text", None)?;  // Same API, no leak
```

## Risk Mitigation

- Keep old implementation behind feature flag during migration
- Add comprehensive tests before removing old code
- Monitor memory usage in staging environment
- Have rollback plan ready

## Success Metrics

- Memory leak eliminated (0 bytes leaked per spawn/evict cycle)
- Request latency unchanged or improved
- Code complexity reduced (fewer lines, clearer flow)
- No API changes required for users