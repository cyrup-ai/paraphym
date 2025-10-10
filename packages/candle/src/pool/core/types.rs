use std::sync::atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering};
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
    pub max_workers_per_model: usize,   // Default: 4 (adaptive scaling limit)
    
    // Channel capacities (bounded to prevent OOM)
    pub embed_queue_capacity: usize,          // Default: 100
    pub batch_queue_capacity: usize,          // Default: 50
    pub prompt_queue_capacity: usize,         // Default: 100 (text_to_text)
    pub image_gen_queue_capacity: usize,      // Default: 20  (text_to_image)
    pub vision_queue_capacity: usize,         // Default: 50  (vision)
    pub image_embed_queue_capacity: usize,    // Default: 50  (image_embedding)
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            request_timeout_secs: 30,
            shutdown_timeout_secs: 5,
            maintenance_interval_secs: 60,
            cooldown_idle_minutes: 1,
            max_workers_per_model: 4,
            
            // Channel capacities (bounded to prevent OOM)
            embed_queue_capacity: 100,
            batch_queue_capacity: 50,
            prompt_queue_capacity: 100,
            image_gen_queue_capacity: 20,  // Image gen is slower, smaller queue
            vision_queue_capacity: 50,
            image_embed_queue_capacity: 50,
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
    pub circuit_rejections: AtomicUsize,
}

/// Handle to a worker thread (capability-specific channels defined in capabilities/)
#[derive(Debug, Clone)]
pub struct WorkerHandle {
    pub pending_requests: Arc<AtomicUsize>,
    pub last_used: Arc<AtomicU64>,
    pub worker_id: usize,
    pub shutdown_tx: Sender<()>,
    pub per_worker_mb: usize,
    pub health_tx: Sender<HealthPing>,
    pub health_rx: Receiver<HealthPong>,
    
    // NEW: Add state tracking
    pub state: Arc<AtomicU32>,  // WorkerState as u32
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
            state: Arc::new(AtomicU32::new(0)), // Start in Spawning state
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
    
    /// Get current worker state
    pub fn get_state(&self) -> crate::pool::core::worker_state::WorkerState {
        use crate::pool::core::worker_state::WorkerState;
        let state_val = self.state.load(Ordering::Acquire);
        WorkerState::from(state_val)
    }
    
    /// Set worker state (atomic)
    pub fn set_state(&self, new_state: crate::pool::core::worker_state::WorkerState) {
        self.state.store(new_state as u32, Ordering::Release);
    }
    
    /// Check if worker can accept requests
    pub fn can_accept_requests(&self) -> bool {
        use crate::pool::core::worker_state::WorkerState;
        matches!(
            self.get_state(),
            WorkerState::Ready | WorkerState::Processing | WorkerState::Idle
        )
    }
    
    /// Check if worker should be evicted
    pub fn is_evictable(&self) -> bool {
        use crate::pool::core::worker_state::WorkerState;
        matches!(
            self.get_state(),
            WorkerState::Ready | WorkerState::Idle
        )
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

/// Trait for capability-specific worker handles
/// 
/// All worker handles (TextEmbeddingWorkerHandle, TextToTextWorkerHandle, etc.)
/// implement this trait to provide unified access to core WorkerHandle fields.
pub trait PoolWorkerHandle: Send + Sync + 'static {
    /// Access core WorkerHandle (pending_requests, last_used, etc.)
    fn core(&self) -> &WorkerHandle;
    
    /// Mutable access to core WorkerHandle
    fn core_mut(&mut self) -> &mut WorkerHandle;
    
    /// Registry key for this worker (model identifier)
    fn registry_key(&self) -> &str;
}
