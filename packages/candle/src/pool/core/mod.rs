pub mod pool;
pub mod types;
pub mod error;
pub mod worker;
pub mod memory;
pub mod spawn;

pub use pool::Pool;
pub use types::{PoolConfig, PoolMetrics, WorkerHandle, SpawnGuard};
pub use error::PoolError;
pub use worker::{spawn_worker_thread, check_memory_available};
pub use memory::query_system_memory_mb;
pub use spawn::{ensure_workers_spawned, HasWorkers, TotalMemory, SpawnLock};
