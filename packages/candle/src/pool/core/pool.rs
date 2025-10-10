use dashmap::DashMap;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::types::{PoolConfig, PoolMetrics, WorkerHandle, SpawnGuard, PoolWorkerHandle};
use super::error::PoolError;
use super::worker_state::{CircuitBreaker, CircuitBreakerConfig};
use super::memory_governor::MemoryGovernor;

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

    /// Circuit breakers per model (prevents cascade failures)
    circuit_breakers: DashMap<String, Arc<CircuitBreaker>>,

    /// Memory governor for system-wide coordination
    pub memory_governor: Arc<MemoryGovernor>,
}

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
            circuit_breakers: DashMap::new(),
            memory_governor: Arc::new(MemoryGovernor::new(0.80)),
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
    pub fn register_worker(&self, registry_key: String, handle: W) {
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
    pub fn workers(&self) -> &DashMap<String, Vec<W>> {
        &self.workers
    }

    /// Get or create circuit breaker for model
    ///
    /// Returns a circuit breaker configured with default thresholds:
    /// - Opens after 5 consecutive failures
    /// - Tries half-open after 60s timeout
    /// - Closes after 3 successful requests in half-open state
    pub fn get_circuit_breaker(&self, registry_key: &str) -> Arc<CircuitBreaker> {
        self.circuit_breakers
            .entry(registry_key.to_string())
            .or_insert_with(|| {
                Arc::new(CircuitBreaker::new(CircuitBreakerConfig {
                    failure_threshold: 5,
                    success_threshold: 3,
                    timeout: Duration::from_secs(60),
                    half_open_requests: 3,
                }))
            })
            .clone()
    }

    /// Validate workers and remove dead ones
    ///
    /// Checks each worker's health via is_alive() and removes workers
    /// that don't respond. Updates memory tracking and metrics.
    ///
    /// Returns the number of workers removed.
    pub fn validate_workers(&self, registry_key: &str) -> usize {
        use crate::pool::core::worker_state::WorkerState;
        
        let mut removed_count = 0;
        
        if let Some(mut workers_guard) = self.workers.get_mut(registry_key) {
            workers_guard.retain(|worker| {
                let state = worker.core().get_state();
                
                // Remove dead/failed workers immediately
                if matches!(state, WorkerState::Dead | WorkerState::Failed) {
                    log::warn!(
                        "Removing {:?} worker {} for {}",
                        state,
                        worker.core().worker_id,
                        registry_key
                    );
                    
                    self.remove_memory(worker.core().per_worker_mb);
                    let _ = worker.core().shutdown_tx.send(());
                    removed_count += 1;
                    
                    false // Remove
                }
                // Also check health for workers that should be alive
                else if matches!(state, WorkerState::Ready | WorkerState::Processing | WorkerState::Idle) {
                    // Only do health check for workers claiming to be active
                    if !worker.core().is_alive() {
                        log::warn!(
                            "Removing unresponsive worker {} for {} (state: {:?})",
                            worker.core().worker_id,
                            registry_key,
                            state
                        );
                        
                        worker.core().set_state(WorkerState::Dead);
                        self.remove_memory(worker.core().per_worker_mb);
                        let _ = worker.core().shutdown_tx.send(());
                        removed_count += 1;
                        
                        false // Remove
                    } else {
                        true // Keep
                    }
                }
                else {
                    // Keep workers in Spawning/Loading states
                    true
                }
            });
            
            if removed_count > 0 {
                log::warn!("Removed {} workers for {}", removed_count, registry_key);
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
            workers.iter().any(|w| w.core().is_alive())
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
                .filter(|(_, w)| w.core().is_alive())  // Only alive workers
                .min_by_key(|(_, w)| w.core().pending_requests.load(Ordering::Acquire))
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
