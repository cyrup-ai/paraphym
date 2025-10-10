use dashmap::DashMap;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

use super::types::{PoolConfig, PoolMetrics, WorkerHandle};

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
}
