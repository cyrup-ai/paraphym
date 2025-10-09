# MPOOL_6A: Implement Maintenance Thread (Eviction & Memory Monitoring)

**PREFIX**: MPOOL (Model Pool)

## OBJECTIVE

Implement background maintenance thread that runs every 1 minute to evict idle workers and monitor memory pressure. This implements the cooldown policy from Scenario 5.

## CONTEXT

Maintenance thread responsibilities:
- Check all pools for idle workers (no requests for 1+ minute)
- Evict LRU worker per idle model per minute
- Scales workers back to 0 gradually (4→3→2→1→0)
- Monitors system memory pressure
- Coordinates across all 5 pool instances

## SUBTASK 1: Create Maintenance Module File

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/maintenance.rs`

## SUBTASK 2: Implement Worker Eviction Logic

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/maintenance.rs`

**Implementation**:
```rust
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::sync::atomic::Ordering;
use crate::pool::core::Pool;

/// Check if all workers for a model are idle
fn all_workers_idle(workers: &[WorkerHandle], idle_threshold_secs: u64) -> bool {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    workers.iter().all(|w| {
        let pending = w.pending_requests.load(Ordering::Acquire);
        let last_used = w.last_used.load(Ordering::Acquire);
        let idle_duration = now.saturating_sub(last_used);

        pending == 0 && idle_duration >= idle_threshold_secs
    })
}

/// Find least recently used (LRU) worker
fn find_lru_worker(workers: &[WorkerHandle]) -> Option<usize> {
    workers.iter()
        .enumerate()
        .min_by_key(|(_, w)| w.last_used.load(Ordering::Acquire))
        .map(|(idx, _)| idx)
}

/// Evict worker from pool
fn evict_worker<T>(
    pool: &Pool<T>,
    registry_key: &str,
    worker_idx: usize,
    per_worker_mb: usize,
) -> Result<(), String> {
    let mut workers = pool.workers.get_mut(registry_key)
        .ok_or_else(|| format!("No workers for {}", registry_key))?;

    if worker_idx >= workers.len() {
        return Err("Worker index out of bounds".to_string());
    }

    // Remove worker
    let worker = workers.remove(worker_idx);

    // Send shutdown signal to worker thread
    // (worker loop receives signal and breaks)

    // Update memory tracking
    pool.remove_memory(per_worker_mb);

    log::info!(
        "Evicted worker {} from {} (idle cooldown), {} workers remain",
        worker.worker_id,
        registry_key,
        workers.len()
    );

    Ok(())
}
```

**Why**: Core eviction logic per Scenario 5 cooldown policy.

## SUBTASK 3: Implement Maintenance Loop

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/maintenance.rs`

**Implementation**:
```rust
use std::thread;
use crate::pool::capabilities::{
    text_embedding_pool,
    text_to_text_pool,
    image_embedding_pool,
    vision_pool,
    text_to_image_pool,
};

/// Start maintenance thread for all pools
///
/// Runs every 1 minute:
/// - Check each pool for idle workers
/// - Evict 1 LRU worker per idle model
/// - Monitor system memory pressure
/// - Log eviction events
pub fn start_maintenance_thread() -> thread::JoinHandle<()> {
    thread::Builder::new()
        .name("pool-maintenance".to_string())
        .spawn(move || {
            let interval = Duration::from_secs(60); // 1 minute
            let idle_threshold = 60; // 1 minute

            loop {
                thread::sleep(interval);

                // Check if shutting down
                if text_embedding_pool().is_shutting_down() {
                    log::info!("Maintenance thread shutting down");
                    break;
                }

                // Process TextEmbedding pool
                process_pool_maintenance(
                    text_embedding_pool(),
                    idle_threshold,
                    "TextEmbedding"
                );

                // Process TextToText pool
                process_pool_maintenance(
                    text_to_text_pool(),
                    idle_threshold,
                    "TextToText"
                );

                // Process ImageEmbedding pool
                process_pool_maintenance(
                    image_embedding_pool(),
                    idle_threshold,
                    "ImageEmbedding"
                );

                // Process Vision pool
                process_pool_maintenance(
                    vision_pool(),
                    idle_threshold,
                    "Vision"
                );

                // Process TextToImage pool
                process_pool_maintenance(
                    text_to_image_pool(),
                    idle_threshold,
                    "TextToImage"
                );

                // Log memory usage
                log_memory_usage();
            }
        })
        .expect("Failed to spawn maintenance thread")
}

/// Process maintenance for one pool
fn process_pool_maintenance<T>(
    pool: &Pool<T>,
    idle_threshold_secs: u64,
    pool_name: &str,
) {
    // Iterate over all models in pool
    for entry in pool.workers.iter() {
        let registry_key = entry.key();
        let workers = entry.value();

        // Check if all workers idle
        if all_workers_idle(workers, idle_threshold_secs) && !workers.is_empty() {
            // Find LRU worker
            if let Some(lru_idx) = find_lru_worker(workers) {
                // Get memory footprint (need to read from model info)
                // This requires accessing the model - skip for now
                // TODO: Store per_worker_mb in WorkerHandle

                log::info!(
                    "{} pool: All workers idle for {}, evicting LRU worker",
                    pool_name,
                    registry_key
                );

                // Evict worker
                drop(entry); // Release DashMap lock
                // evict_worker(pool, registry_key, lru_idx, per_worker_mb).ok();
            }
        }
    }
}

/// Log current memory usage across all pools
fn log_memory_usage() {
    let text_embedding_mb = text_embedding_pool().total_memory_mb();
    let text_to_text_mb = text_to_text_pool().total_memory_mb();
    let image_embedding_mb = image_embedding_pool().total_memory_mb();
    let vision_mb = vision_pool().total_memory_mb();
    let text_to_image_mb = text_to_image_pool().total_memory_mb();

    let total_mb = text_embedding_mb + text_to_text_mb + image_embedding_mb + vision_mb + text_to_image_mb;

    log::debug!(
        "Pool memory usage: {} MB (TextEmbedding: {}, TextToText: {}, ImageEmbedding: {}, Vision: {}, TextToImage: {})",
        total_mb,
        text_embedding_mb,
        text_to_text_mb,
        image_embedding_mb,
        vision_mb,
        text_to_image_mb
    );
}
```

**Why**: Background thread coordinates eviction across all pools (Scenario 5).

## SUBTASK 4: Add Maintenance Thread Initialization

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/mod.rs`

**Add**:
```rust
pub mod core;
pub mod capabilities;
pub mod maintenance;  // NEW

pub use core::{Pool, PoolConfig, PoolError};
pub use capabilities::{
    text_embedding_pool,
    text_to_text_pool,
    image_embedding_pool,
    vision_pool,
    text_to_image_pool,
};
pub use maintenance::start_maintenance_thread;  // NEW

use once_cell::sync::Lazy;

/// Global maintenance thread handle
static MAINTENANCE_THREAD: Lazy<std::thread::JoinHandle<()>> = Lazy::new(|| {
    start_maintenance_thread()
});

/// Initialize maintenance thread (call once at startup)
pub fn init_maintenance() {
    // Force lazy initialization
    let _ = &*MAINTENANCE_THREAD;
}
```

**Why**: Lazy initialization ensures maintenance thread starts when pools are first used.

## SUBTASK 5: Add Shutdown Signal Channel to WorkerHandle

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/core/types.rs`

**Modify WorkerHandle**:
```rust
use crossbeam::channel::Sender;

#[derive(Debug)]
pub struct WorkerHandle {
    pub pending_requests: Arc<AtomicUsize>,
    pub last_used: Arc<AtomicU64>,
    pub worker_id: usize,
    pub shutdown_tx: Sender<()>,  // NEW: Signal worker to shutdown
}
```

**Why**: Need channel to signal worker thread to exit during eviction.

## DEFINITION OF DONE

- [ ] `maintenance.rs` file created
- [ ] `all_workers_idle()` function implemented
- [ ] `find_lru_worker()` function implemented
- [ ] `evict_worker()` function implemented
- [ ] `start_maintenance_thread()` function implemented
- [ ] `process_pool_maintenance()` function implemented
- [ ] `log_memory_usage()` function implemented
- [ ] Maintenance thread initialization added to pool/mod.rs
- [ ] `shutdown_tx` channel added to WorkerHandle
- [ ] Code compiles with `cargo check`

## DEPENDENCIES

**Requires**: MPOOL_2A (Pool<T>), MPOOL_3A/B/C (pool accessors)

**Blocks**: None (maintenance is enhancement, not blocker)

**Parallel with**: MPOOL_6B (graceful shutdown)

## RESEARCH NOTES

**Cooldown Policy** (from Scenario 5):
```
t=5:00  Last request completes, all 4 workers idle
t=6:00  All idle 1 min → evict 1 worker (3 remain)
t=7:00  All idle 1 min → evict 1 worker (2 remain)
t=7:30  NEW REQUEST → cooldown resets, 2 workers stay
t=8:30  Request done, 2 workers idle
t=9:30  All idle 1 min → evict 1 worker (1 remains)
t=10:30 All idle 1 min → evict last worker (0 remain, back to cold state)
```

**Key Rules**:
- Evict 1 worker per model per minute
- Only evict if ALL workers idle for 1+ minute
- If ANY worker received request: reset cooldown timer
- LRU = worker with oldest last_used timestamp
- Scales back to 0 (complete unload)

## CONSTRAINTS

- **NO TESTS**: Do not write any test code. Tests are handled by separate team.
- **NO BENCHMARKS**: Do not write any benchmark code. Benchmarks are handled by separate team.
- **GRADUAL EVICTION**: 1 worker per model per minute (prevents thrashing).
- **COORDINATED**: Single maintenance thread for all 5 pools.
