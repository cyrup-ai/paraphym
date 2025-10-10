pub mod pool;
pub mod types;
pub mod error;
pub mod worker;
pub mod memory;
pub mod memory_governor;
pub mod spawn;
pub mod worker_state;

pub use pool::Pool;
pub use types::{PoolConfig, PoolMetrics, WorkerHandle, SpawnGuard, PoolWorkerHandle};
pub use error::PoolError;
pub use worker::{spawn_worker_thread, check_memory_available};
pub use memory::query_system_memory_mb;
pub use memory_governor::{MemoryGovernor, AllocationGuard, MemoryError, EvictionCandidate, MemoryPressure};
pub use spawn::{ensure_workers_spawned, ensure_workers_spawned_adaptive, HasWorkers, MemoryGovernorAccess, SpawnLock, WorkerMetrics};
pub use worker_state::{WorkerState, UnifiedWorkerHandle, CircuitBreaker, HealthCheck, HealthStatus};
