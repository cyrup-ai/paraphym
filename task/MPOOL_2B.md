# MPOOL_2B: Implement Core Worker Functions

**PREFIX**: MPOOL (Model Pool)

## OBJECTIVE

Implement generic worker spawn and loop functions in `pool/core/worker.rs`. These functions handle worker thread lifecycle, request processing, and channel communication patterns for all capability traits.

## CONTEXT

Workers are generic over capability traits. Each worker owns a loaded model exclusively and processes requests in an infinite loop. Worker loops use crossbeam channel select! for multi-channel request handling. This module provides the generic patterns - capability-specific details are in pool/capabilities/.

## SUBTASK 1: Create Worker Module File

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/core/worker.rs`

## SUBTASK 2: Implement Generic Worker Spawn Helper

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/core/worker.rs`

**Implementation**:
```rust
use std::thread;
use crate::pool::core::PoolError;

/// Spawn worker thread with model loader closure
///
/// Generic pattern for all capability types:
/// 1. Worker thread spawned
/// 2. Model loaded via closure (happens in worker thread)
/// 3. Worker loop starts, processes requests until shutdown
/// 4. Worker owns model exclusively (no Arc<Mutex<>>)
pub fn spawn_worker_thread<T, F>(
    model_loader: F,
    worker_name: String,
) -> Result<thread::JoinHandle<()>, PoolError>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, PoolError> + Send + 'static,
{
    thread::Builder::new()
        .name(worker_name)
        .spawn(move || {
            // Load model in worker thread
            let model = match model_loader() {
                Ok(m) => m,
                Err(e) => {
                    log::error!("Worker model loading failed: {}", e);
                    return;
                }
            };

            // Worker loop implemented by capability-specific modules
            // (model passed to capability-specific worker loop)
        })
        .map_err(|e| PoolError::SpawnFailed(format!("Thread spawn failed: {}", e)))
}
```

**Why**: Provides generic worker spawn pattern used by all capability types (Scenario 2, 5).

## SUBTASK 3: Implement Memory Check Helper

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/core/worker.rs`

**Implementation**:
```rust
use crate::domain::model::traits::CandleModel;

/// Check if spawning N workers would exceed 80% memory limit
///
/// Formula (from Scenario 4):
/// ```
/// current_usage_mb + (num_workers * per_worker_mb) <= 0.80 * total_system_mb
/// ```
pub fn check_memory_available<T: CandleModel>(
    model: &T,
    current_usage_mb: usize,
    num_workers: usize,
) -> Result<(), PoolError> {
    let per_worker_mb = model.info().est_memory_allocation_mb;
    let required_mb = num_workers * per_worker_mb;

    // Query system memory
    let total_system_mb = query_system_memory_mb();
    let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;

    if current_usage_mb + required_mb > memory_limit_mb {
        return Err(PoolError::MemoryExhausted(format!(
            "Cannot spawn {} workers ({} MB). Current: {} MB, Limit: {} MB (80% of {})",
            num_workers, required_mb, current_usage_mb, memory_limit_mb, total_system_mb
        )));
    }

    Ok(())
}

/// Query total system memory in MB
fn query_system_memory_mb() -> usize {
    use sysinfo::{System, SystemExt};
    let mut sys = System::new_all();
    sys.refresh_memory();
    (sys.total_memory() / 1024 / 1024) as usize
}
```

**Why**: Memory accounting for 80% limit enforcement (Scenario 1, Scenario 4).

## SUBTASK 4: Add Dependency

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml`

**Add to dependencies**:
```toml
[dependencies]
# ... existing dependencies ...
sysinfo = "0.30"
```

**Why**: Need sysinfo crate for system memory queries.

## SUBTASK 5: Wire Up Module Export

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/core/mod.rs`

**Update**:
```rust
pub mod pool;
pub mod types;
pub mod error;
pub mod worker;  // NEW

pub use pool::Pool;
pub use types::{PoolConfig, PoolMetrics, WorkerHandle};
pub use error::PoolError;
pub use worker::{spawn_worker_thread, check_memory_available};  // NEW
```

**Why**: Make worker helpers accessible to capability modules.

## SUBTASK 6: Document Worker Loop Pattern

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/core/worker.rs`

**Add module-level documentation**:
```rust
//! Generic worker spawn and lifecycle helpers
//!
//! # Worker Pattern
//!
//! Workers are generic over capability traits. Each worker:
//! 1. Owns loaded model exclusively (no Arc<Mutex<>>)
//! 2. Processes requests from shared crossbeam channels
//! 3. Self-schedules via channel recv() (first available gets request)
//! 4. Runs infinite loop until shutdown signal
//!
//! # Capability-Specific Implementation
//!
//! Worker loops are implemented in `pool/capabilities/{trait_name}.rs`:
//! - TextEmbedding: embed_rx, batch_embed_rx channels
//! - TextToText: prompt_rx channel
//! - ImageEmbedding: embed_image_rx, embed_image_url_rx, etc.
//! - Vision: describe_image_rx, describe_url_rx
//! - TextToImage: generate_image_rx channel
//!
//! # Memory Management
//!
//! Before spawning worker, check memory with `check_memory_available()`.
//! After spawn: pool.add_memory(per_worker_mb)
//! After eviction: pool.remove_memory(per_worker_mb)
//!
//! # References
//!
//! - MODEL_POOL.md Scenario 1: Dynamic worker limits
//! - MODEL_POOL.md Scenario 4: Memory footprint calculation
//! - MODEL_POOL.md Scenario 5: Worker lifecycle
```

**Why**: Clear documentation for capability module implementers.

## DEFINITION OF DONE

- [ ] `worker.rs` file created
- [ ] `spawn_worker_thread()` generic helper implemented
- [ ] `check_memory_available()` memory accounting helper implemented
- [ ] `query_system_memory_mb()` system memory query implemented
- [ ] `sysinfo` dependency added to Cargo.toml
- [ ] Module exports updated
- [ ] Module documentation added
- [ ] Code compiles with `cargo check`

## DEPENDENCIES

**Requires**: MPOOL_1 (worker reads `est_memory_allocation_mb`), MPOOL_2A (uses Pool, PoolError)

**Blocks**: MPOOL_3A/B/C (capability modules use these helpers)

## RESEARCH NOTES

**Worker Lifecycle** (from MODEL_POOL.md Scenario 5):
- Cold start: 0→2 workers (asymmetric)
- Warm expansion: +1 worker (symmetric)
- Cooldown: -1 worker per idle minute
- Memory bounded: All spawns check 80% limit

**Memory Formula** (from Scenario 4):
```rust
let per_worker_memory_mb = model.info().est_memory_allocation_mb;
let current_usage_mb = pool.total_memory_used.load(Ordering::Acquire);
let total_system_memory_mb = query_system_memory();
let memory_limit_mb = (total_system_memory_mb as f64 * 0.80) as usize;

if current_usage_mb + per_worker_memory_mb <= memory_limit_mb {
    spawn_worker();
}
```

**Key Design Principles**:
- Workers own models exclusively (no locks)
- Generic spawn pattern for all capability traits
- Memory checks prevent OOM (20% headroom)

## CONSTRAINTS

- **NO TESTS**: Do not write any test code. Tests are handled by separate team.
- **NO BENCHMARKS**: Do not write any benchmark code. Benchmarks are handled by separate team.
- **GENERIC ONLY**: No model-specific logic. Works for any capability trait.
