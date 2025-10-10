//! Generic cold-start helper for eliminating duplication in registry.rs
//!
//! This module provides `ensure_workers_spawned()` which encapsulates the
//! decision logic for spawning workers that was previously duplicated 42+ times.

use super::{Pool, PoolError, SpawnGuard, query_system_memory_mb};
use std::time::Duration;

/// Ensure workers are spawned for a model (cold-start helper with race protection)
///
/// Encapsulates the decision logic:
/// 1. Try to acquire spawn lock (prevents race conditions)
/// 2. If lock acquired: spawn workers and release lock
/// 3. If lock busy: wait for other thread to complete spawning
///
/// # Parameters
/// - `pool`: Pool instance (must have has_workers and total_memory_mb methods)
/// - `registry_key`: Model registry key
/// - `per_worker_mb`: Memory per worker (from model.info().est_memory_allocation_mb)
/// - `spawn_fn`: Closure that spawns ONE worker, called N times
///
/// # Returns
/// - `Ok(())` if workers already exist or spawned successfully
/// - `Err(PoolError)` if memory exhausted, spawn failed, or timeout
///
/// # Example Usage
/// ```rust,ignore
/// ensure_workers_spawned(
///     pool,
///     registry_key,
///     per_worker_mb,
///     |_worker_idx| {
///         let m_clone = m.clone();
///         pool.spawn_text_embedding_worker(
///             registry_key,
///             move || LoadedGteQwenModel::load(&m_clone)
///                 .map_err(|e| PoolError::SpawnFailed(e.to_string())),
///             per_worker_mb,
///         )
///     }
/// )?;
/// ```
pub fn ensure_workers_spawned<P, F>(
    pool: &P,
    registry_key: &str,
    per_worker_mb: usize,
    spawn_fn: F,
) -> Result<(), PoolError>
where
    P: HasWorkers + TotalMemory + SpawnLock,
    F: Fn(usize) -> Result<(), PoolError>,
{
    // 1. Try to acquire spawn lock (prevents race conditions)
    if let Some(_guard) = pool.try_acquire_spawn_lock(registry_key) {
        // Double-check workers don't exist (another thread may have spawned before lock)
        if pool.has_workers(registry_key) {
            return Ok(());
        }

        // 2. Calculate memory availability
        let current_mb = pool.total_memory_mb();
        let total_system_mb = query_system_memory_mb();
        let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;

        // 3. Decide worker count based on available memory
        let workers_to_spawn = if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
            2  // Ideal: spawn 2 workers for parallel processing
        } else if current_mb + per_worker_mb <= memory_limit_mb {
            1  // Degraded: only 1 worker fits
        } else {
            return Err(PoolError::MemoryExhausted(format!(
                "Cannot spawn workers for {}. Need {} MB, only {} MB available (80% limit)",
                registry_key, per_worker_mb, memory_limit_mb.saturating_sub(current_mb)
            )));
        };

        // 4. Spawn N workers by calling spawn_fn in loop
        for worker_idx in 0..workers_to_spawn {
            spawn_fn(worker_idx)?;
        }

        // _guard drops here, releasing spawn lock
        Ok(())
    } else {
        // Another thread is spawning - wait for it to complete (30s timeout)
        pool.wait_for_workers(registry_key, Duration::from_secs(30))
    }
}

/// Trait for pools that can check worker existence
pub trait HasWorkers {
    fn has_workers(&self, registry_key: &str) -> bool;
}

/// Trait for pools that track total memory
pub trait TotalMemory {
    fn total_memory_mb(&self) -> usize;
}

/// Trait for pools that support spawn locking
pub trait SpawnLock {
    fn try_acquire_spawn_lock(&self, registry_key: &str) -> Option<SpawnGuard>;
    fn wait_for_workers(&self, registry_key: &str, timeout: Duration) -> Result<(), PoolError>;
}

// Implement traits for Pool<T>
impl<T: ?Sized> HasWorkers for Pool<T> {
    fn has_workers(&self, registry_key: &str) -> bool {
        Pool::has_workers(self, registry_key)
    }
}

impl<T: ?Sized> TotalMemory for Pool<T> {
    fn total_memory_mb(&self) -> usize {
        Pool::total_memory_mb(self)
    }
}

impl<T: ?Sized> SpawnLock for Pool<T> {
    fn try_acquire_spawn_lock(&self, registry_key: &str) -> Option<SpawnGuard> {
        Pool::try_acquire_spawn_lock(self, registry_key)
    }
    
    fn wait_for_workers(&self, registry_key: &str, timeout: Duration) -> Result<(), PoolError> {
        Pool::wait_for_workers(self, registry_key, timeout)
    }
}
