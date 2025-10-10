use std::sync::atomic::{AtomicU64, AtomicUsize};
use std::sync::Arc;
use crossbeam::channel::Sender;

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
    pub shutdown_tx: Sender<()>,
    pub per_worker_mb: usize,
}

impl WorkerHandle {
    pub fn new(worker_id: usize, shutdown_tx: Sender<()>, per_worker_mb: usize) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            pending_requests: Arc::new(AtomicUsize::new(0)),
            last_used: Arc::new(AtomicU64::new(now)),
            worker_id,
            shutdown_tx,
            per_worker_mb,
        }
    }

    pub fn touch(&self) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        self.last_used.store(now, std::sync::atomic::Ordering::Release);
    }
}
