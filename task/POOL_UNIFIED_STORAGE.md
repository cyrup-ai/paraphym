# POOL_UNIFIED_STORAGE

**Priority**: CRITICAL
**Component**: pool/core, pool/capabilities
**Estimated Effort**: 2-3 days
**Risk**: High (breaking change)
**Dependencies**: None

## Problem Statement

Current implementation has **DUAL STORAGE** causing memory leaks:

1. **`Pool.workers`**: `DashMap<String, Vec<WorkerHandle>>` - stores core handles only
2. **Global static DashMaps**: `TEXT_EMBEDDING_WORKERS`, `TEXT_TO_TEXT_WORKERS`, `IMAGE_EMBEDDING_WORKERS`, `VISION_WORKERS`, `TEXT_TO_IMAGE_WORKERS` - stores full handles with channels

### The Memory Leak Mechanism

When workers are evicted via [`maintenance.rs::evict_worker()`](../packages/candle/src/pool/maintenance.rs#L70-L105):

1. Worker is removed from `Pool.workers` (line 81: `workers_guard.remove(worker_idx)`)
2. Shutdown signal is sent (line 87-93)
3. Memory is decremented (line 95: `pool.remove_memory(per_worker_mb)`)
4. **BUT**: Full handle in global DashMap is never removed!

The `Drop` trait in [`TextEmbeddingWorkerHandle`](../packages/candle/src/pool/capabilities/text_embedding.rs#L44-L54) attempts cleanup, but it never executes because:
- `Pool.workers` stores only `WorkerHandle` (core)
- Global DashMap stores `TextEmbeddingWorkerHandle` (full)
- Eviction drops the core, but full handle remains in global map forever
- Channels, threads, and memory references leak

### Current Architecture (Broken)

```rust
// pool/core/pool.rs:11-13
pub struct Pool<T: ?Sized> {
    workers: DashMap<String, Vec<WorkerHandle>>,  // ← Core handles only
    // ...
}

// pool/capabilities/text_embedding.rs:117
static TEXT_EMBEDDING_WORKERS: Lazy<DashMap<String, Vec<TextEmbeddingWorkerHandle>>> 
    = Lazy::new(DashMap::new);  // ← Full handles with channels

// pool/capabilities/text_embedding.rs:235-242 (spawn_text_embedding_worker)
self.register_worker(registry_key.to_string(), pool_handle);  // ← Core to Pool
TEXT_EMBEDDING_WORKERS
    .entry(registry_key_clone)
    .or_insert_with(Vec::new)
    .push(full_handle);  // ← Full to global static (LEAK SOURCE)
```

### Why Two Storage Locations?

The split exists because:
- **`WorkerHandle`** ([types.rs:45-60](../packages/candle/src/pool/core/types.rs#L45-L60)) contains: `pending_requests`, `last_used`, `worker_id`, `shutdown_tx`, `per_worker_mb`, `health_tx`, `health_rx`
- **Capability-specific handles** contain: `core: WorkerHandle` + capability channels (`embed_tx`, `batch_embed_tx`, etc.)
- **`Pool` operations** (eviction, health checks) only need core fields
- **Request routing** ([text_embedding.rs:257](../packages/candle/src/pool/capabilities/text_embedding.rs#L257)) needs capability-specific channels to send requests

The dual storage was an architectural shortcut that causes memory leaks.

## Solution Design

### Core Principle: Single Source of Truth

**Store capability-specific worker handles directly in `Pool<T>.workers`**, eliminating global static DashMaps entirely.

### Design Approach: Generic Over Worker Handle Type

Instead of `Pool<dyn TextEmbeddingCapable>` storing generic `WorkerHandle`, make `Pool<W>` generic over the worker handle type:

```rust
// New approach
pub struct Pool<W> {
    workers: DashMap<String, Vec<W>>,  // W = TextEmbeddingWorkerHandle, etc.
    config: PoolConfig,
    // ...
}

// Each capability has its own pool type
static TEXT_EMBEDDING_POOL: Lazy<Pool<TextEmbeddingWorkerHandle>> = 
    Lazy::new(|| Pool::new(PoolConfig::default()));
```

This requires adding a trait to abstract common operations:

```rust
pub trait PoolWorkerHandle: Send + Sync {
    fn core(&self) -> &WorkerHandle;
    fn core_mut(&mut self) -> &mut WorkerHandle;
    fn registry_key(&self) -> &str;
}
```

## Implementation Steps

### Step 1: Create PoolWorkerHandle Trait

**File**: `packages/candle/src/pool/core/types.rs`

**Add after line 199** (after `SpawnGuard` impl):

```rust
/// Trait for capability-specific worker handles
/// 
/// All worker handles (TextEmbeddingWorkerHandle, TextToTextWorkerHandle, etc.)
/// implement this trait to provide unified access to core WorkerHandle fields.
pub trait PoolWorkerHandle: Send + Sync + 'static {
    /// Access core WorkerHandle (pending_requests, last_used, etc.)
    fn core(&self) -> &WorkerHandle;
    
    /// Mutable access to core WorkerHandle
    fn core_mut(&mut self) -> &mut WorkerHandle;
    
    /// Registry key for this worker (model identifier)
    fn registry_key(&self) -> &str;
}
```

### Step 2: Update Pool Struct

**File**: `packages/candle/src/pool/core/pool.rs`

**Replace lines 10-32** (entire `Pool` struct):

```rust
/// Generic worker pool for capability-specific worker handles
pub struct Pool<W: PoolWorkerHandle> {
    /// Map of registry_key -> Vec<W> where W is capability-specific handle
    /// (TextEmbeddingWorkerHandle, TextToTextWorkerHandle, etc.)
    workers: DashMap<String, Vec<W>>,

    /// Pool configuration
    config: PoolConfig,

    /// Total memory used by all workers (in MB)
    total_memory_used: Arc<AtomicUsize>,

    /// Next worker ID for unique identification
    next_worker_id: AtomicUsize,

    /// Pool metrics
    metrics: PoolMetrics,

    /// Shutdown flag
    shutting_down: Arc<AtomicBool>,

    /// Track models currently spawning workers (prevents race conditions)
    spawning_in_progress: DashMap<String, Arc<AtomicBool>>,
}
```

**Update lines 34-50** (Pool::new, has_workers, next_worker_id):

```rust
impl<W: PoolWorkerHandle> Pool<W> {
    /// Create new pool with config
    pub fn new(config: PoolConfig) -> Self {
        Self {
            workers: DashMap::new(),
            config,
            total_memory_used: Arc::new(AtomicUsize::new(0)),
            next_worker_id: AtomicUsize::new(0),
            metrics: PoolMetrics::default(),
            shutting_down: Arc::new(AtomicBool::new(false)),
            spawning_in_progress: DashMap::new(),
        }
    }

    /// Check if workers exist for registry_key
    pub fn has_workers(&self, registry_key: &str) -> bool {
        self.workers.get(registry_key).map(|w| !w.is_empty()).unwrap_or(false)
    }
```

**Update line 58** (register_worker signature):

```rust
    /// Register worker handle for registry_key
    pub fn register_worker(&self, registry_key: String, handle: W) {
        self.workers.entry(registry_key).or_insert_with(Vec::new).push(handle);
    }
```

**Update lines 118-137** (validate_workers to use .core()):

```rust
    pub fn validate_workers(&self, registry_key: &str) -> usize {
        let mut dead_workers = Vec::new();
        
        if let Some(workers_guard) = self.workers.get(registry_key) {
            for (idx, worker) in workers_guard.iter().enumerate() {
                if !worker.core().is_alive() {  // ← Use .core()
                    dead_workers.push((idx, worker.core().worker_id, worker.core().per_worker_mb));
                }
            }
        }
        
        if dead_workers.is_empty() {
            return 0;
        }
        
        let mut removed_count = 0;
        
        if let Some(mut workers_guard) = self.workers.get_mut(registry_key) {
            for (idx, worker_id, per_worker_mb) in dead_workers.iter().rev() {
                if *idx < workers_guard.len() {
                    if workers_guard[*idx].core().worker_id == *worker_id {  // ← Use .core()
                        let worker = workers_guard.remove(*idx);
                        
                        log::warn!(
                            "Removing dead worker {} for {} (no health response)",
                            worker_id,
                            registry_key
                        );
                        
                        self.remove_memory(*per_worker_mb);
                        let _ = worker.core().shutdown_tx.send(());  // ← Use .core()
                        removed_count += 1;
                    }
                }
            }
            
            if removed_count > 0 {
                log::warn!("Removed {} dead workers for {}", removed_count, registry_key);
                self.metrics.workers_evicted.fetch_add(removed_count, Ordering::Release);
            }
        }
        
        removed_count
    }
```

**Update lines 179-190** (has_alive_workers, get_alive_worker to use .core()):

```rust
    pub fn has_alive_workers(&self, registry_key: &str) -> bool {
        if let Some(workers) = self.workers.get(registry_key) {
            workers.iter().any(|w| w.core().is_alive())  // ← Use .core()
        } else {
            false
        }
    }

    pub fn get_alive_worker(&self, registry_key: &str) -> Option<usize> {
        if let Some(workers) = self.workers.get(registry_key) {
            workers
                .iter()
                .enumerate()
                .filter(|(_, w)| w.core().is_alive())  // ← Use .core()
                .min_by_key(|(_, w)| w.core().pending_requests.load(Ordering::Acquire))  // ← Use .core()
                .map(|(idx, _)| idx)
        } else {
            None
        }
    }
```

**Update line 106** (workers() return type):

```rust
    pub fn workers(&self) -> &DashMap<String, Vec<W>> {
        &self.workers
    }
```

### Step 3: Update TextEmbeddingWorkerHandle

**File**: `packages/candle/src/pool/capabilities/text_embedding.rs`

**Add after line 30** (after struct definition):

```rust
impl crate::pool::core::types::PoolWorkerHandle for TextEmbeddingWorkerHandle {
    fn core(&self) -> &crate::pool::core::WorkerHandle {
        &self.core
    }
    
    fn core_mut(&mut self) -> &mut crate::pool::core::WorkerHandle {
        &mut self.core
    }
    
    fn registry_key(&self) -> &str {
        &self.registry_key
    }
}
```

**DELETE lines 44-54** (entire Drop implementation - no longer needed!):

```rust
// DELETE THIS ENTIRE BLOCK
impl Drop for TextEmbeddingWorkerHandle {
    fn drop(&mut self) {
        // This is no longer needed - workers are in Pool only
    }
}
```

**DELETE lines 116-118** (global static DashMap):

```rust
// DELETE THIS LINE
static TEXT_EMBEDDING_WORKERS: Lazy<DashMap<String, Vec<TextEmbeddingWorkerHandle>>> = Lazy::new(DashMap::new);
```

**UPDATE line 120** (pool type):

```rust
static TEXT_EMBEDDING_POOL: Lazy<Pool<TextEmbeddingWorkerHandle>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});
```

**UPDATE line 125** (pool accessor return type):

```rust
pub fn text_embedding_pool() -> &'static Pool<TextEmbeddingWorkerHandle> {
    &TEXT_EMBEDDING_POOL
}
```

**UPDATE line 127** (impl block):

```rust
impl Pool<TextEmbeddingWorkerHandle> {
```

**UPDATE lines 235-247** (remove dual registration):

Replace:
```rust
        self.register_worker(registry_key.to_string(), pool_handle);

        let full_handle = TextEmbeddingWorkerHandle { /* ... */ };

        TEXT_EMBEDDING_WORKERS
            .entry(registry_key_clone)
            .or_insert_with(Vec::new)
            .push(full_handle);
```

With:
```rust
        let full_handle = TextEmbeddingWorkerHandle {
            core: WorkerHandle {
                pending_requests,
                last_used,
                worker_id,
                shutdown_tx: shutdown_tx.clone(),
                per_worker_mb,
                health_tx: health_tx_worker,
                health_rx: health_rx_main,
            },
            embed_tx,
            batch_embed_tx,
            shutdown_tx,
            registry_key: registry_key_clone,
        };

        // Single registration point - no duplication
        self.register_worker(registry_key.to_string(), full_handle);
```

**UPDATE lines 257-277** (embed_text method - access workers from pool):

Replace:
```rust
        let workers = TEXT_EMBEDDING_WORKERS.get(registry_key)
            .ok_or_else(|| PoolError::NoWorkers(format!("No workers for {}", registry_key)))?;
```

With:
```rust
        let workers = self.workers.get(registry_key)
            .ok_or_else(|| PoolError::NoWorkers(format!("No workers for {}", registry_key)))?;
```

Same change for lines 264-275 (worker selection code remains identical).

**UPDATE lines 300-330** (batch_embed_text method - same changes):

Replace global map access with pool access:
```rust
        let workers = self.workers.get(registry_key)
            .ok_or_else(|| PoolError::NoWorkers(format!("No workers for {}", registry_key)))?;
```

### Step 4: Repeat for All Other Capabilities

Apply the same pattern to:

**File**: `packages/candle/src/pool/capabilities/text_to_text.rs`
- Add `PoolWorkerHandle` impl for `TextToTextWorkerHandle` (after line 33)
- Delete `Drop` impl (lines 43-53)
- Delete `static TEXT_TO_TEXT_WORKERS` (line 102-103)
- Change pool type to `Pool<TextToTextWorkerHandle>` (line 106)
- Change impl to `impl Pool<TextToTextWorkerHandle>` (line 114)
- Remove dual registration in `spawn_text_to_text_worker` (lines ~170-180)
- Change worker access from global map to `self.workers` (lines ~195+)

**File**: `packages/candle/src/pool/capabilities/image_embedding.rs`
- Add `PoolWorkerHandle` impl for `ImageEmbeddingWorkerHandle`
- Delete `Drop` impl
- Delete `static IMAGE_EMBEDDING_WORKERS` (line 147)
- Change pool type to `Pool<ImageEmbeddingWorkerHandle>` (line 150)
- Change impl to `impl Pool<ImageEmbeddingWorkerHandle>` (line 158)
- Remove dual registration in spawn method
- Change worker access from global map to `self.workers`

**File**: `packages/candle/src/pool/capabilities/vision.rs`
- Add `PoolWorkerHandle` impl for `VisionWorkerHandle`
- Delete `Drop` impl
- Delete `static VISION_WORKERS` (line 114)
- Change pool type to `Pool<VisionWorkerHandle>`
- Change impl to `impl Pool<VisionWorkerHandle>`
- Remove dual registration in spawn method
- Change worker access from global map to `self.workers`

**File**: `packages/candle/src/pool/capabilities/text_to_image.rs`
- Add `PoolWorkerHandle` impl for `TextToImageWorkerHandle`
- Delete `Drop` impl
- Delete `static TEXT_TO_IMAGE_WORKERS` (line 101)
- Change pool type to `Pool<TextToImageWorkerHandle>`
- Change impl to `impl Pool<TextToImageWorkerHandle>`
- Remove dual registration in spawn method
- Change worker access from global map to `self.workers`

### Step 5: Update Maintenance Code

**File**: `packages/candle/src/pool/maintenance.rs`

**UPDATE lines 70-105** (evict_worker function to use .core()):

```rust
fn evict_worker<W: crate::pool::core::types::PoolWorkerHandle>(
    pool: &Pool<W>,
    registry_key: &str,
    worker_idx: usize,
    per_worker_mb: usize,
) -> Result<(), String> {
    let mut workers_guard = pool
        .workers()
        .get_mut(registry_key)
        .ok_or_else(|| format!("No workers for {}", registry_key))?;

    if worker_idx >= workers_guard.len() {
        return Err(format!(
            "Worker index {} out of bounds (len: {})",
            worker_idx,
            workers_guard.len()
        ));
    }

    let worker = workers_guard.remove(worker_idx);
    let remaining_count = workers_guard.len();
    drop(workers_guard);

    // Send shutdown signal
    if let Err(e) = worker.core().shutdown_tx.send(()) {  // ← Use .core()
        log::warn!(
            "Failed to send shutdown signal to worker {}: {}",
            worker.core().worker_id,  // ← Use .core()
            e
        );
    }

    pool.remove_memory(per_worker_mb);
    pool.metrics().workers_evicted.fetch_add(1, Ordering::Release);

    log::info!(
        "Evicted worker {} from {} (idle cooldown), {} workers remain",
        worker.core().worker_id,  // ← Use .core()
        registry_key,
        remaining_count
    );

    Ok(())
}
```

**UPDATE lines 17-34** (all_workers_idle function to use .core()):

```rust
fn all_workers_idle<W: crate::pool::core::types::PoolWorkerHandle>(
    workers: &[W],
    idle_threshold_secs: u64,
) -> bool {
    if workers.is_empty() {
        return false;
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    workers.iter().all(|w| {
        let core = w.core();  // ← Use .core()
        let pending = core.pending_requests.load(Ordering::Acquire);
        let last_used = core.last_used.load(Ordering::Acquire);
        let idle_duration = now.saturating_sub(last_used);

        pending == 0 && idle_duration >= idle_threshold_secs
    })
}
```

**UPDATE lines 43-51** (find_lru_worker function to use .core()):

```rust
fn find_lru_worker<W: crate::pool::core::types::PoolWorkerHandle>(
    workers: &[W],
) -> Option<usize> {
    workers
        .iter()
        .enumerate()
        .min_by_key(|(_, w)| w.core().last_used.load(Ordering::Acquire))  // ← Use .core()
        .map(|(idx, _)| idx)
}
```

### Step 6: Update Type Exports

**File**: `packages/candle/src/pool/core/types.rs`

**Add to exports** (top of file after existing use statements):

```rust
pub use super::types::PoolWorkerHandle;  // Add to existing exports
```

**File**: `packages/candle/src/pool/core/mod.rs`

**Add export** (after existing exports):

```rust
pub use types::PoolWorkerHandle;
```

## Verification of Solution

After implementation, verify memory leak is fixed:

```rust
// Test code (not in production)
fn test_no_memory_leak() {
    let pool = text_embedding_pool();
    
    // Spawn workers
    pool.spawn_text_embedding_worker("test-model", || Ok(model), 1000)?;
    assert_eq!(pool.total_memory_mb(), 1000);
    
    // Evict workers (via maintenance or manual)
    evict_worker(pool, "test-model", 0, 1000)?;
    
    // Memory should be freed
    assert_eq!(pool.total_memory_mb(), 0);
    
    // Global static should be empty (doesn't exist anymore!)
    // No dangling references in memory
}
```

## Definition of Done

- [ ] `PoolWorkerHandle` trait created in `types.rs`
- [ ] `Pool<W>` updated to be generic over worker handle type
- [ ] All 5 capability worker handles implement `PoolWorkerHandle`
- [ ] All 5 global static DashMaps (`TEXT_EMBEDDING_WORKERS`, etc.) deleted
- [ ] All 5 pool types updated: `Pool<TextEmbeddingWorkerHandle>`, etc.
- [ ] All spawn methods register only to pool, not to global map
- [ ] All request routing methods access `self.workers` instead of global map
- [ ] Maintenance code updated to use `.core()` accessor
- [ ] All `Drop` impls for worker handles deleted
- [ ] Code compiles without errors
- [ ] Existing pool integration tests pass
- [ ] Memory leak eliminated (verified via manual spawn/evict cycle)

## Files Changed Summary

| File | Changes |
|------|---------|
| `pool/core/types.rs` | Add `PoolWorkerHandle` trait |
| `pool/core/pool.rs` | Change `Pool<T>` to `Pool<W: PoolWorkerHandle>`, update all methods to use `.core()` |
| `pool/maintenance.rs` | Update function signatures to generic `<W: PoolWorkerHandle>`, use `.core()` |
| `pool/capabilities/text_embedding.rs` | Implement `PoolWorkerHandle`, delete global static, update pool type, fix spawn/routing |
| `pool/capabilities/text_to_text.rs` | Same as text_embedding |
| `pool/capabilities/image_embedding.rs` | Same as text_embedding |
| `pool/capabilities/vision.rs` | Same as text_embedding |
| `pool/capabilities/text_to_image.rs` | Same as text_embedding |

**Lines Changed**: ~150 lines across 8 files
**Lines Deleted**: ~50 lines (global statics + Drop impls)
**Net Change**: +100 lines

## Key Architectural Insight

The root cause was **type erasure**: storing `WorkerHandle` (core) in `Pool` while needing capability-specific handles (full) for request routing. The solution is **no type erasure**: store the full capability-specific handle directly via generics.

**Before** (memory leak):
```
Pool<dyn TextEmbeddingCapable>.workers: Vec<WorkerHandle>  ← Core only
TEXT_EMBEDDING_WORKERS: Vec<TextEmbeddingWorkerHandle>      ← Full with channels
↑ Eviction drops core, full remains forever = LEAK
```

**After** (no leak):
```
Pool<TextEmbeddingWorkerHandle>.workers: Vec<TextEmbeddingWorkerHandle>  ← Full with channels
↑ Eviction drops full handle, channels closed, memory freed = CLEAN
```

The generic `Pool<W: PoolWorkerHandle>` allows each capability to have a properly-typed pool while sharing common pool logic through the `PoolWorkerHandle` trait's `.core()` accessor.

## References

- Current Pool implementation: [`packages/candle/src/pool/core/pool.rs`](../packages/candle/src/pool/core/pool.rs)
- Worker handle types: [`packages/candle/src/pool/core/types.rs`](../packages/candle/src/pool/core/types.rs)
- TextEmbedding capability (example): [`packages/candle/src/pool/capabilities/text_embedding.rs`](../packages/candle/src/pool/capabilities/text_embedding.rs)
- Maintenance logic: [`packages/candle/src/pool/maintenance.rs`](../packages/candle/src/pool/maintenance.rs)
- ULTIMATE_SOLUTION (alternative approach): [`packages/candle/src/pool/ULTIMATE_SOLUTION.md`](../packages/candle/src/pool/ULTIMATE_SOLUTION.md)
