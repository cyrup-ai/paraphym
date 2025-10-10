use std::sync::atomic::{AtomicU64, AtomicUsize};
use std::sync::Arc;
use crossbeam::channel::{Sender, Receiver};

/// Health check ping sent to worker
#[derive(Debug, Clone, Copy)]
pub struct HealthPing;

/// Health check response from worker
#[derive(Debug, Clone)]
pub struct HealthPong {
    pub worker_id: usize,
    pub timestamp: u64,
    pub queue_depth: usize,
}

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
    pub health_tx: Sender<HealthPing>,
    pub health_rx: Receiver<HealthPong>,
}

impl WorkerHandle {
    pub fn new(
        worker_id: usize,
        shutdown_tx: Sender<()>,
        per_worker_mb: usize,
        health_tx: Sender<HealthPing>,
        health_rx: Receiver<HealthPong>,
    ) -> Self {
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
            health_tx,
            health_rx,
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

    /// Check if worker is alive by sending health ping
    ///
    /// Returns true if worker responds within 100ms, false otherwise.
    /// False indicates worker thread is dead, stuck, or channel broken.
    pub fn is_alive(&self) -> bool {
        use std::time::Duration;
        
        // Try to send ping
        if self.health_tx.send(HealthPing).is_err() {
            // Channel broken = worker dead
            return false;
        }

        // Wait for pong with timeout
        match self.health_rx.recv_timeout(Duration::from_millis(100)) {
            Ok(pong) => {
                // Update last health check timestamp
                self.last_used.store(pong.timestamp, std::sync::atomic::Ordering::Release);
                true
            }
            Err(_) => {
                // Timeout or disconnected = worker dead/stuck
                false
            }
        }
    }
}

/// Select worker using Power of Two Choices algorithm (O(1) instead of O(n))
///
/// Algorithm:
/// - 0 workers: None
/// - 1 worker: Return that worker
/// - 2+ workers: Sample 2 random workers, return least loaded
///
/// This achieves O(log log n) load imbalance vs O(log n) for random selection,
/// while maintaining O(1) time complexity vs O(n) for full scan.
///
/// # Performance
/// - 2 atomic loads instead of N
/// - No cache thrashing from full iteration
/// - Scalable to 100+ workers with no degradation
///
/// # Usage
/// Works with any worker handle type that has a `core: WorkerHandle` field:
/// ```ignore
/// let worker = select_worker_power_of_two(&workers, |w| &w.core)?;
/// ```
pub fn select_worker_power_of_two<'a, T, F>(workers: &'a [T], get_core: F) -> Option<&'a T>
where
    F: Fn(&'a T) -> &'a WorkerHandle,
{
    match workers.len() {
        0 => None,
        1 => Some(&workers[0]),
        len => {
            // Sample 2 random indices
            let idx1 = fastrand::usize(..len);
            let mut idx2 = fastrand::usize(..len);
            
            // Ensure idx2 != idx1 (unlikely but possible)
            while idx2 == idx1 && len > 1 {
                idx2 = fastrand::usize(..len);
            }
            
            let w1 = &workers[idx1];
            let w2 = &workers[idx2];
            
            // Compare pending requests (only 2 atomic loads!)
            let core1 = get_core(w1);
            let core2 = get_core(w2);
            let load1 = core1.pending_requests.load(std::sync::atomic::Ordering::Acquire);
            let load2 = core2.pending_requests.load(std::sync::atomic::Ordering::Acquire);
            
            // Return least loaded
            if load1 <= load2 {
                Some(w1)
            } else {
                Some(w2)
            }
        }
    }
}

/// RAII guard that prevents duplicate worker spawning
/// 
/// Automatically releases spawn lock when dropped, even if panic occurs.
/// Only one thread can hold a SpawnGuard for a given registry_key at a time.
pub struct SpawnGuard {
    flag: Arc<std::sync::atomic::AtomicBool>,
    registry_key: String,
}

impl SpawnGuard {
    pub(crate) fn new(flag: Arc<std::sync::atomic::AtomicBool>, registry_key: String) -> Self {
        Self { flag, registry_key }
    }
}

impl Drop for SpawnGuard {
    fn drop(&mut self) {
        // Release spawn lock when guard is dropped
        self.flag.store(false, std::sync::atomic::Ordering::Release);
        log::debug!("Released spawn lock for {}", self.registry_key);
    }
}
