# MPOOL_2A: Implement Pool Core Infrastructure

**PREFIX**: MPOOL (Model Pool)

## OBJECTIVE

Implement generic `Pool<T>` struct and supporting types in `pool/core/` module. This provides the foundational infrastructure for managing worker pools across all capability traits.

## CONTEXT

Pool is generic over capability traits (`T: TextEmbeddingCapable`, etc.). Written once, instantiated 5 times (one per capability). Handles worker registration, memory tracking, and request routing. This is pure infrastructure - zero model-specific logic.

## SUBTASK 1: Create Module Structure

**Create directories**:
```bash
mkdir -p /Volumes/samsung_t9/paraphym/packages/candle/src/pool/core
mkdir -p /Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities
```

**Create files**:
- `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/mod.rs`
- `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/core/mod.rs`
- `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/core/pool.rs`
- `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/core/types.rs`
- `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/core/error.rs`

**Why**: Organized module structure per MODEL_POOL.md "Module Structure" section.

## SUBTASK 2: Implement PoolError Enum

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/core/error.rs`

**Implementation**:
```rust
use std::fmt;

#[derive(Debug, Clone)]
pub enum PoolError {
    NoWorkers(String),           // No workers spawned for registry_key
    Timeout(String),             // Request timed out after N seconds
    SendError(String),           // Failed to send request to worker
    RecvError(String),           // Failed to receive response from worker
    ModelError(String),          // Model inference error
    ShuttingDown(String),        // Pool shutting down, rejecting requests
    MemoryExhausted(String),     // Cannot spawn worker, 80% limit reached
    SpawnFailed(String),         // Worker thread spawn failed
}

impl fmt::Display for PoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoWorkers(msg) => write!(f, "No workers available: {}", msg),
            Self::Timeout(msg) => write!(f, "Request timeout: {}", msg),
            Self::SendError(msg) => write!(f, "Channel send error: {}", msg),
            Self::RecvError(msg) => write!(f, "Channel recv error: {}", msg),
            Self::ModelError(msg) => write!(f, "Model error: {}", msg),
            Self::ShuttingDown(msg) => write!(f, "Shutting down: {}", msg),
            Self::MemoryExhausted(msg) => write!(f, "Memory exhausted: {}", msg),
            Self::SpawnFailed(msg) => write!(f, "Worker spawn failed: {}", msg),
        }
    }
}

impl std::error::Error for PoolError {}
```

**Why**: Typed error handling for all pool operations (Scenario 6).

## SUBTASK 3: Implement Pool Types

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/core/types.rs`

**Implementation**:
```rust
use std::sync::atomic::{AtomicU64, AtomicUsize};
use std::sync::Arc;

/// Configuration for pool behavior
#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub request_timeout_secs: u64,      // Default: 30
    pub shutdown_timeout_secs: u64,     // Default: 5
    pub maintenance_interval_secs: u64, // Default: 60 (1 minute)
    pub cooldown_idle_minutes: u64,     // Default: 1
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            request_timeout_secs: 30,
            shutdown_timeout_secs: 5,
            maintenance_interval_secs: 60,
            cooldown_idle_minutes: 1,
        }
    }
}

/// Metrics tracked per pool
#[derive(Debug, Default)]
pub struct PoolMetrics {
    pub total_requests: AtomicUsize,
    pub total_timeouts: AtomicUsize,
    pub total_errors: AtomicUsize,
    pub workers_spawned: AtomicUsize,
    pub workers_evicted: AtomicUsize,
}

/// Handle to a worker thread (capability-specific channels defined in capabilities/)
#[derive(Debug)]
pub struct WorkerHandle {
    pub pending_requests: Arc<AtomicUsize>,
    pub last_used: Arc<AtomicU64>,
    pub worker_id: usize,
}

impl WorkerHandle {
    pub fn new(worker_id: usize) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            pending_requests: Arc::new(AtomicUsize::new(0)),
            last_used: Arc::new(AtomicU64::new(now)),
            worker_id,
        }
    }

    pub fn touch(&self) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.last_used.store(now, std::sync::atomic::Ordering::Release);
    }
}
```

**Why**: Shared types used across all pool implementations (Scenario 1, 5, 7).

## SUBTASK 4: Implement Generic Pool<T> Struct

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/core/pool.rs`

**Implementation**:
```rust
use dashmap::DashMap;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

use super::types::{PoolConfig, PoolMetrics, WorkerHandle};
use super::error::PoolError;

/// Generic worker pool for capability trait T
pub struct Pool<T> {
    /// Map of registry_key -> Vec<WorkerHandle>
    workers: DashMap<String, Vec<WorkerHandle>>,

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

    /// Phantom data for generic type
    _phantom: PhantomData<T>,
}

impl<T> Pool<T> {
    /// Create new pool with config
    pub fn new(config: PoolConfig) -> Self {
        Self {
            workers: DashMap::new(),
            config,
            total_memory_used: Arc::new(AtomicUsize::new(0)),
            next_worker_id: AtomicUsize::new(0),
            metrics: PoolMetrics::default(),
            shutting_down: Arc::new(AtomicBool::new(false)),
            _phantom: PhantomData,
        }
    }

    /// Check if workers exist for registry_key
    pub fn has_workers(&self, registry_key: &str) -> bool {
        self.workers.get(registry_key).map(|w| !w.is_empty()).unwrap_or(false)
    }

    /// Get next worker ID
    pub fn next_worker_id(&self) -> usize {
        self.next_worker_id.fetch_add(1, Ordering::Relaxed)
    }

    /// Register worker handle for registry_key
    pub fn register_worker(&self, registry_key: String, handle: WorkerHandle) {
        self.workers.entry(registry_key).or_insert_with(Vec::new).push(handle);
    }

    /// Get total memory used
    pub fn total_memory_mb(&self) -> usize {
        self.total_memory_used.load(Ordering::Acquire)
    }

    /// Add memory usage
    pub fn add_memory(&self, mb: usize) {
        self.total_memory_used.fetch_add(mb, Ordering::Release);
    }

    /// Remove memory usage
    pub fn remove_memory(&self, mb: usize) {
        self.total_memory_used.fetch_sub(mb, Ordering::Release);
    }

    /// Check if shutting down
    pub fn is_shutting_down(&self) -> bool {
        self.shutting_down.load(Ordering::Acquire)
    }

    /// Begin shutdown
    pub fn begin_shutdown(&self) {
        self.shutting_down.store(true, Ordering::Release);
    }

    /// Get config
    pub fn config(&self) -> &PoolConfig {
        &self.config
    }

    /// Get metrics
    pub fn metrics(&self) -> &PoolMetrics {
        &self.metrics
    }
}
```

**Why**: Generic foundation for all 5 pool instances (Architecture section).

## SUBTASK 5: Wire Up Module Exports

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/core/mod.rs`

```rust
pub mod pool;
pub mod types;
pub mod error;

pub use pool::Pool;
pub use types::{PoolConfig, PoolMetrics, WorkerHandle};
pub use error::PoolError;
```

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/mod.rs`

```rust
pub mod core;
pub mod capabilities;

pub use core::{Pool, PoolConfig, PoolError};
```

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/lib.rs` (add):

```rust
pub mod pool;
```

**Why**: Make pool types accessible to other modules.

## DEFINITION OF DONE

- [ ] Module structure created (`pool/core/`, `pool/capabilities/`)
- [ ] `PoolError` enum implemented with all 8 variants
- [ ] `PoolConfig`, `PoolMetrics`, `WorkerHandle` types implemented
- [ ] Generic `Pool<T>` struct implemented with core methods
- [ ] Module exports configured (`mod.rs` files)
- [ ] Code compiles with `cargo check`
- [ ] No model-specific logic in any file (100% generic)

## DEPENDENCIES

**Requires**: MPOOL_1 (Pool reads `est_memory_allocation_mb` field)

**Blocks**: MPOOL_2B (worker implementations), MPOOL_3A/B/C (capability-specific pools)

## RESEARCH NOTES

**Pool Architecture** (from MODEL_POOL.md):
- 5 pool instances (one per capability trait)
- Generic `Pool<T>` written once, instantiated 5 times
- Each pool's DashMap stores: `registry_key -> Vec<WorkerHandle>`

**Key Design Principles**:
- Zero model-specific code (GteQwen, Phi4, etc. never mentioned)
- Generic over capability traits
- Thread-safe (DashMap, atomics)
- Workers own models exclusively (no Arc<Mutex<>>)

## CONSTRAINTS

- **NO TESTS**: Do not write any test code. Tests are handled by separate team.
- **NO BENCHMARKS**: Do not write any benchmark code. Benchmarks are handled by separate team.
- **GENERIC ONLY**: No model-specific imports or logic. Pool must work for ANY model implementing trait T.
