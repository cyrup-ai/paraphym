use crossbeam::channel::{bounded, Receiver, Sender};
use crossbeam::select;
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, AtomicU64, AtomicU32, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use ystream::{AsyncStream, spawn_stream};

use crate::pool::core::{Pool, PoolConfig, PoolError, WorkerHandle};
use crate::pool::core::types::{select_worker_power_of_two, HealthPing, HealthPong};
use crate::pool::core::memory_governor::AllocationGuard;
use crate::capability::traits::VisionCapable;
use crate::domain::context::CandleStringChunk;

/// Request for describe_image() operation (streaming response)
pub struct DescribeImageRequest {
    pub image_path: String,
    pub query: String,
    pub response: Sender<Result<AsyncStream<CandleStringChunk>, PoolError>>,
}

/// Request for describe_url() operation (streaming response)
pub struct DescribeUrlRequest {
    pub url: String,
    pub query: String,
    pub response: Sender<Result<AsyncStream<CandleStringChunk>, PoolError>>,
}

/// Vision-specific worker handle with channels
pub struct VisionWorkerHandle {
    pub core: WorkerHandle,
    pub describe_image_tx: Sender<DescribeImageRequest>,
    pub describe_url_tx: Sender<DescribeUrlRequest>,
    pub shutdown_tx: Sender<()>,
    pub registry_key: String,  // Added to enable cleanup on drop
}

impl crate::pool::core::types::PoolWorkerHandle for VisionWorkerHandle {
    fn core(&self) -> &crate::pool::core::WorkerHandle {
        &self.core
    }
    
    fn core_mut(&mut self) -> &mut crate::pool::core::WorkerHandle {
        &mut self.core
    }
    
    fn registry_key(&self) -> &str {
        &self.registry_key
    }
}

impl std::ops::Deref for VisionWorkerHandle {
    type Target = WorkerHandle;
    
    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

/// Worker loop for Vision models
///
/// Processes streaming requests. Worker calls trait method which
/// returns AsyncStream<CandleStringChunk>. Stream is sent back to caller
/// who forwards chunks to end user.
pub fn vision_worker<T: VisionCapable>(
    model: T,
    describe_image_rx: Receiver<DescribeImageRequest>,
    describe_url_rx: Receiver<DescribeUrlRequest>,
    shutdown_rx: Receiver<()>,
    health_rx: Receiver<HealthPing>,
    health_tx: Sender<HealthPong>,
    worker_id: usize,
    state: Arc<AtomicU32>,
) {
    use std::time::{Duration, SystemTime};
    use crate::pool::core::worker_state::WorkerState;
    
    // Track last activity for idle detection
    let mut last_activity = SystemTime::now();
    let idle_threshold = Duration::from_secs(300); // 5 minutes
    
    loop {
        // Check for idle timeout (Ready → Idle after 5 minutes of inactivity)
        if let Ok(elapsed) = last_activity.elapsed() {
            if elapsed > idle_threshold {
                let current_state = WorkerState::from(state.load(std::sync::atomic::Ordering::Acquire));
                if matches!(current_state, WorkerState::Ready) {
                    state.store(WorkerState::Idle as u32, std::sync::atomic::Ordering::Release);
                }
            }
        }
        
        select! {
            recv(describe_image_rx) -> req => {
                if let Ok(req) = req {
                    // Transition: Ready/Idle → Processing
                    state.store(WorkerState::Processing as u32, std::sync::atomic::Ordering::Release);
                    
                    let stream = model.describe_image(&req.image_path, &req.query);
                    let _ = req.response.send(Ok(stream));
                    
                    // Transition: Processing → Ready
                    state.store(WorkerState::Ready as u32, std::sync::atomic::Ordering::Release);
                    last_activity = SystemTime::now();
                }
            }
            recv(describe_url_rx) -> req => {
                if let Ok(req) = req {
                    // Transition: Ready/Idle → Processing
                    state.store(WorkerState::Processing as u32, std::sync::atomic::Ordering::Release);
                    
                    let stream = model.describe_url(&req.url, &req.query);
                    let _ = req.response.send(Ok(stream));
                    
                    // Transition: Processing → Ready
                    state.store(WorkerState::Ready as u32, std::sync::atomic::Ordering::Release);
                    last_activity = SystemTime::now();
                }
            }
            recv(health_rx) -> ping => {
                if ping.is_ok() {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(0);
                    
                    let pong = HealthPong {
                        worker_id,
                        timestamp: now,
                        queue_depth: describe_image_rx.len() + describe_url_rx.len(),
                    };
                    
                    let _ = health_tx.send(pong);
                }
            }
            recv(shutdown_rx) -> _ => {
                log::info!("Vision worker {} shutting down", worker_id);
                // Transition: Ready/Idle → Evicting
                state.store(WorkerState::Evicting as u32, std::sync::atomic::Ordering::Release);
                break;
            }
        }
    }
}

/// Global Vision pool instance
static VISION_POOL: Lazy<Pool<VisionWorkerHandle>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

/// Access global Vision pool
pub fn vision_pool() -> &'static Pool<VisionWorkerHandle> {
    &VISION_POOL
}

impl Pool<VisionWorkerHandle> {
    /// Spawn worker for Vision model
    pub fn spawn_vision_worker<T, F>(
        &self,
        registry_key: &str,
        model_loader: F,
        per_worker_mb: usize,
        allocation_guard: AllocationGuard,
    ) -> Result<(), PoolError>
    where
        T: VisionCapable + Send + 'static,
        F: FnOnce() -> Result<T, PoolError> + Send + 'static,
    {

        // Create BOUNDED channels (prevent OOM)
        let (describe_image_tx, describe_image_rx) = bounded(self.config().vision_queue_capacity);
        let (describe_url_tx, describe_url_rx) = bounded(self.config().vision_queue_capacity);
        let (shutdown_tx, shutdown_rx) = bounded(1);
        let (health_tx_worker, health_rx_worker) = bounded::<HealthPing>(1);
        let (health_tx_main, health_rx_main) = bounded::<HealthPong>(1);

        // Get worker ID before moving into thread
        let worker_id = self.next_worker_id();
        let registry_key_clone = registry_key.to_string();

        // Clone channels for worker thread
        let health_rx_worker_clone = health_rx_worker.clone();
        let health_tx_main_clone = health_tx_main.clone();
        let per_worker_mb_clone = per_worker_mb;
        
        // Create state before spawning thread so we can clone it
        use std::sync::atomic::AtomicU32;
        let state = Arc::new(AtomicU32::new(0)); // Spawning state
        let state_clone = Arc::clone(&state);

        // Spawn worker thread
        std::thread::spawn(move || {
            use crate::pool::core::worker_state::WorkerState;
            
            // Guard held by worker thread - will drop on exit
            let _memory_guard = allocation_guard;
            
            // Transition: Spawning → Loading
            state_clone.store(WorkerState::Loading as u32, std::sync::atomic::Ordering::Release);
            
            let model = match model_loader() {
                Ok(m) => {
                    log::info!("Vision worker {} ready", worker_id);
                    // Transition: Loading → Ready
                    state_clone.store(WorkerState::Ready as u32, std::sync::atomic::Ordering::Release);
                    m
                }
                Err(e) => {
                    log::error!("Vision worker {} model loading failed: {}", worker_id, e);
                    // Transition: Loading → Failed
                    state_clone.store(WorkerState::Failed as u32, std::sync::atomic::Ordering::Release);
                    
                    // Clean up memory tracking (CRITICAL FIX)
                    // This prevents memory leak when model loading fails
                    vision_pool().remove_memory(per_worker_mb_clone);
                    
                    return;
                }
            };

            vision_worker(
                model,
                describe_image_rx,
                describe_url_rx,
                shutdown_rx,
                health_rx_worker_clone,
                health_tx_main_clone,
                worker_id,
                Arc::clone(&state_clone),
            );
            
            // Transition: Ready → Dead (when worker loop exits)
            state_clone.store(WorkerState::Dead as u32, std::sync::atomic::Ordering::Release);
        });

        // Create handles (state already created above before spawning)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        let pending_requests = Arc::new(AtomicUsize::new(0));
        let last_used = Arc::new(AtomicU64::new(now));
        
        // Store capability-specific handle
        let full_handle = VisionWorkerHandle {
            core: WorkerHandle {
                pending_requests,
                last_used,
                worker_id,
                shutdown_tx: shutdown_tx.clone(),
                per_worker_mb,
                health_tx: health_tx_worker,
                health_rx: health_rx_main,
                state,
            },
            describe_image_tx,
            describe_url_tx,
            shutdown_tx,
            registry_key: registry_key_clone.clone(),
        };

        // Single registration point - no duplication
        self.register_worker(registry_key.to_string(), full_handle);

        // Update memory tracking
        self.add_memory(per_worker_mb);

        Ok(())
    }

    /// Describe image using pooled worker (returns AsyncStream)
    pub fn describe_image(
        &self,
        registry_key: &str,
        image_path: &str,
        query: &str,
    ) -> AsyncStream<CandleStringChunk> {
        // Clone for move into closure
        let registry_key = registry_key.to_string();
        let image_path = image_path.to_string();
        let query = query.to_string();
        let is_shutting_down = self.is_shutting_down();
        let request_timeout_secs = self.config().request_timeout_secs;

        spawn_stream(move |sender| {
            // Check shutdown
            if is_shutting_down {
                ystream::emit!(sender, CandleStringChunk(
                    "Pool shutting down".to_string()
                ));
                return;
            }

            // Get circuit breaker for this model and check state
            let pool = vision_pool();
            let circuit = pool.get_circuit_breaker(&registry_key);
            
            if !circuit.can_request() {
                ystream::emit!(sender, CandleStringChunk(
                    format!("Circuit breaker open for {}", registry_key)
                ));
                // Update metrics
                pool.metrics().circuit_rejections.fetch_add(1, Ordering::Relaxed);
                return;
            }

            // Get workers from pool
            let workers = match pool.workers().get(&registry_key) {
                Some(w) => w,
                None => {
                    ystream::emit!(sender, CandleStringChunk(
                        format!("No workers for {}", registry_key)
                    ));
                    return;
                }
            };

            if workers.is_empty() {
                ystream::emit!(sender, CandleStringChunk(
                    "No workers available".to_string()
                ));
                return;
            }

            // Find alive worker with least load using Power of Two Choices (O(1))
            let alive_workers: Vec<_> = workers
                .iter()
                .filter(|w| w.core.is_alive())
                .collect();
            
            let worker = match select_worker_power_of_two(&alive_workers, |w| &w.core) {
                Some(w) => w,
                None => {
                    ystream::emit!(sender, CandleStringChunk(
                        format!("No alive workers for {}", registry_key)
                    ));
                    return;
                }
            };

            // Track request
            worker.core.pending_requests.fetch_add(1, Ordering::Release);
            worker.core.touch();

            // Send request to worker
            let (response_tx, response_rx) = crossbeam::channel::bounded(1);
            if let Err(e) = worker.describe_image_tx.send(DescribeImageRequest {
                image_path,
                query,
                response: response_tx,
            }) {
                ystream::emit!(sender, CandleStringChunk(
                    format!("Failed to send request: {}", e)
                ));
                worker.core.pending_requests.fetch_sub(1, Ordering::Release);
                return;
            }

            // Wait for worker's AsyncStream with timeout
            let timeout = Duration::from_secs(request_timeout_secs);
            let worker_stream = match response_rx.recv_timeout(timeout) {
                Ok(Ok(stream)) => {
                    // Record success on circuit breaker
                    circuit.record_success();
                    stream
                }
                Ok(Err(e)) => {
                    // Record failure on circuit breaker
                    circuit.record_failure();
                    pool.metrics().total_errors.fetch_add(1, Ordering::Relaxed);
                    
                    ystream::emit!(sender, CandleStringChunk(
                        format!("Worker error: {}", e)
                    ));
                    worker.core.pending_requests.fetch_sub(1, Ordering::Release);
                    return;
                }
                Err(e) => {
                    // Record timeout as failure
                    circuit.record_failure();
                    pool.metrics().total_timeouts.fetch_add(1, Ordering::Relaxed);
                    
                    ystream::emit!(sender, CandleStringChunk(
                        format!("Request timeout: {}", e)
                    ));
                    worker.core.pending_requests.fetch_sub(1, Ordering::Release);
                    return;
                }
            };

            // Forward chunks from worker stream to caller as they arrive
            // We're already in a background thread, so blocking iteration is fine
            // into_iter() gives us blocking iteration over the stream
            for chunk in worker_stream {
                ystream::emit!(sender, chunk);
            }
            
            worker.core.pending_requests.fetch_sub(1, Ordering::Release);
        })
    }

    /// Describe image from URL using pooled worker (returns AsyncStream)
    pub fn describe_url(
        &self,
        registry_key: &str,
        url: &str,
        query: &str,
    ) -> AsyncStream<CandleStringChunk> {
        // Clone for move into closure
        let registry_key = registry_key.to_string();
        let url = url.to_string();
        let query = query.to_string();
        let is_shutting_down = self.is_shutting_down();
        let request_timeout_secs = self.config().request_timeout_secs;

        spawn_stream(move |sender| {
            // Check shutdown
            if is_shutting_down {
                ystream::emit!(sender, CandleStringChunk(
                    "Pool shutting down".to_string()
                ));
                return;
            }

            // Get circuit breaker for this model and check state
            let pool = vision_pool();
            let circuit = pool.get_circuit_breaker(&registry_key);
            
            if !circuit.can_request() {
                ystream::emit!(sender, CandleStringChunk(
                    format!("Circuit breaker open for {}", registry_key)
                ));
                // Update metrics
                pool.metrics().circuit_rejections.fetch_add(1, Ordering::Relaxed);
                return;
            }

            // Get workers from pool
            let workers = match pool.workers().get(&registry_key) {
                Some(w) => w,
                None => {
                    ystream::emit!(sender, CandleStringChunk(
                        format!("No workers for {}", registry_key)
                    ));
                    return;
                }
            };

            if workers.is_empty() {
                ystream::emit!(sender, CandleStringChunk(
                    "No workers available".to_string()
                ));
                return;
            }

            // Find alive worker with least load using Power of Two Choices (O(1))
            let alive_workers: Vec<_> = workers
                .iter()
                .filter(|w| w.core.is_alive())
                .collect();
            
            let worker = match select_worker_power_of_two(&alive_workers, |w| &w.core) {
                Some(w) => w,
                None => {
                    ystream::emit!(sender, CandleStringChunk(
                        format!("No alive workers for {}", registry_key)
                    ));
                    return;
                }
            };

            // Track request
            worker.core.pending_requests.fetch_add(1, Ordering::Release);
            worker.core.touch();

            // Send request to worker
            let (response_tx, response_rx) = crossbeam::channel::bounded(1);
            if let Err(e) = worker.describe_url_tx.send(DescribeUrlRequest {
                url,
                query,
                response: response_tx,
            }) {
                ystream::emit!(sender, CandleStringChunk(
                    format!("Failed to send request: {}", e)
                ));
                worker.core.pending_requests.fetch_sub(1, Ordering::Release);
                return;
            }

            // Wait for worker's AsyncStream with timeout
            let timeout = Duration::from_secs(request_timeout_secs);
            let worker_stream = match response_rx.recv_timeout(timeout) {
                Ok(Ok(stream)) => {
                    // Record success on circuit breaker
                    circuit.record_success();
                    stream
                }
                Ok(Err(e)) => {
                    // Record failure on circuit breaker
                    circuit.record_failure();
                    pool.metrics().total_errors.fetch_add(1, Ordering::Relaxed);
                    
                    ystream::emit!(sender, CandleStringChunk(
                        format!("Worker error: {}", e)
                    ));
                    worker.core.pending_requests.fetch_sub(1, Ordering::Release);
                    return;
                }
                Err(e) => {
                    // Record timeout as failure
                    circuit.record_failure();
                    pool.metrics().total_timeouts.fetch_add(1, Ordering::Relaxed);
                    
                    ystream::emit!(sender, CandleStringChunk(
                        format!("Request timeout: {}", e)
                    ));
                    worker.core.pending_requests.fetch_sub(1, Ordering::Release);
                    return;
                }
            };

            // Forward chunks from worker stream to caller as they arrive
            // We're already in a background thread, so blocking iteration is fine
            // into_iter() gives us blocking iteration over the stream
            for chunk in worker_stream {
                ystream::emit!(sender, chunk);
            }
            
            worker.core.pending_requests.fetch_sub(1, Ordering::Release);
        })
    }
}


