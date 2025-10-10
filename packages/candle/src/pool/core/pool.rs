use dashmap::DashMap;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::types::{PoolConfig, PoolMetrics, WorkerHandle, SpawnGuard};
use super::error::PoolError;

/// Generic worker pool for capability trait T
pub struct Pool<T: ?Sized> {
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

    /// Track models currently spawning workers (prevents race conditions)
    spawning_in_progress: DashMap<String, Arc<AtomicBool>>,

    /// Phantom data for generic type
    _phantom: PhantomData<T>,
}

impl<T: ?Sized> Pool<T> {
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

    /// Get workers map (for maintenance operations)
    pub fn workers(&self) -> &DashMap<String, Vec<WorkerHandle>> {
        &self.workers
    }

    /// Validate workers and remove dead ones
    ///
    /// Checks each worker's health via is_alive() and removes workers
    /// that don't respond. Updates memory tracking and metrics.
    ///
    /// Returns the number of workers removed.
    pub fn validate_workers(&self, registry_key: &str) -> usize {
        // Collect dead workers first to avoid TOCTOU race
        let mut dead_workers = Vec::new();
        
        // First pass: identify dead workers without holding lock
        if let Some(workers_guard) = self.workers.get(registry_key) {
            for (idx, worker) in workers_guard.iter().enumerate() {
                if !worker.is_alive() {
                    dead_workers.push((idx, worker.worker_id, worker.per_worker_mb));
                }
            }
        }
        
        // Early return if no dead workers
        if dead_workers.is_empty() {
            return 0;
        }
        
        let mut removed_count = 0;
        
        // Second pass: remove dead workers with proper locking
        if let Some(mut workers_guard) = self.workers.get_mut(registry_key) {
            // Remove in reverse order to maintain indices
            for (idx, worker_id, per_worker_mb) in dead_workers.iter().rev() {
                // Double-check index is still valid
                if *idx < workers_guard.len() {
                    // Verify it's the same worker (in case of concurrent modifications)
                    if workers_guard[*idx].worker_id == *worker_id {
                        let worker = workers_guard.remove(*idx);
                        
                        log::warn!(
                            "Removing dead worker {} for {} (no health response)",
                            worker_id,
                            registry_key
                        );
                        
                        // Update memory tracking
                        self.remove_memory(*per_worker_mb);
                        
                        // Send shutdown signal (may fail if worker already dead)
                        let _ = worker.shutdown_tx.send(());
                        
                        removed_count += 1;
                    }
                }
            }
            
            if removed_count > 0 {
                log::warn!(
                    "Removed {} dead workers for {}",
                    removed_count,
                    registry_key
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

    /// Try to acquire exclusive spawn lock for a model
    /// 
    /// Returns Some(guard) if this thread won the race to spawn workers.
    /// Returns None if another thread is already spawning workers.
    /// 
    /// Uses compare-and-swap for lock-free synchronization.
    pub fn try_acquire_spawn_lock(&self, registry_key: &str) -> Option<SpawnGuard> {
        // Get or create atomic flag for this model
        let flag = self.spawning_in_progress
            .entry(registry_key.to_string())
            .or_insert_with(|| Arc::new(AtomicBool::new(false)))
            .value()
            .clone();
        
        // Try to claim spawn lock using compare-exchange
        // If flag is false (not spawning), set to true (spawning) and return guard
        // If flag is true (already spawning), return None
        match flag.compare_exchange(
            false,                    // Expected: not spawning
            true,                     // Desired: now spawning
            Ordering::AcqRel,         // Success ordering
            Ordering::Acquire,        // Failure ordering
        ) {
            Ok(_) => {
                log::debug!("Acquired spawn lock for {}", registry_key);
                Some(SpawnGuard::new(flag, registry_key.to_string()))
            },
            Err(_) => {
                log::debug!("Spawn lock busy for {} (another thread spawning)", registry_key);
                None
            },
        }
    }
    
    /// Wait for workers to become available (blocking)
    /// 
    /// Called by threads that lose the spawn race. Polls until:
    /// - Workers become available (success)
    /// - Spawning thread releases lock without creating workers (spawn failed)
    /// - Timeout exceeded (spawn timeout)
    pub fn wait_for_workers(&self, registry_key: &str, timeout: Duration) -> Result<(), PoolError> {
        let start = Instant::now();
        
        loop {
            // Check if workers are ready
            if self.has_workers(registry_key) {
                log::debug!("Workers ready for {}", registry_key);
                return Ok(());
            }
            
            // Check if spawning thread released lock (spawn completed or failed)
            if let Some(flag) = self.spawning_in_progress.get(registry_key) {
                if !flag.load(Ordering::Acquire) {
                    // Spawning finished but no workers available = spawn failed
                    return Err(PoolError::SpawnFailed(format!(
                        "Worker spawning completed for {} but no workers available. \
                         Check logs for model loading errors.",
                        registry_key
                    )));
                }
            }
            
            // Check timeout
            if start.elapsed() > timeout {
                return Err(PoolError::SpawnTimeout(format!(
                    "Timed out after {:?} waiting for {} workers to spawn",
                    timeout, registry_key
                )));
            }
            
            // Sleep briefly before next poll (50ms balances latency vs CPU)
            std::thread::sleep(Duration::from_millis(50));
        }
    }
}
