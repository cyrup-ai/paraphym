use crossbeam::channel::{bounded, Receiver, Sender};
use crossbeam::select;
use once_cell::sync::Lazy;
use dashmap::DashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, AtomicU64, AtomicU32, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use ystream::{AsyncStream, spawn_stream};

use crate::pool::core::{Pool, PoolConfig, PoolError, WorkerHandle};
use crate::pool::core::types::{select_worker_power_of_two, HealthPing, HealthPong};
use crate::pool::core::memory_governor::AllocationGuard;
use crate::capability::traits::TextToTextCapable;
use crate::domain::prompt::CandlePrompt;
use crate::domain::completion::CandleCompletionParams;
use crate::domain::context::chunk::CandleCompletionChunk;

/// Request for prompt() operation (streaming response)
pub struct PromptRequest {
    pub prompt: CandlePrompt,
    pub params: CandleCompletionParams,
    pub response: Sender<Result<AsyncStream<CandleCompletionChunk>, PoolError>>,
}

/// TextToText-specific worker handle with channel
#[derive(Clone)]
pub struct TextToTextWorkerHandle {
    pub core: WorkerHandle,
    pub prompt_tx: Sender<PromptRequest>,
    pub shutdown_tx: Sender<()>,
    pub registry_key: String,  // Added to enable cleanup on drop
}

impl crate::pool::core::types::PoolWorkerHandle for TextToTextWorkerHandle {
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

impl std::ops::Deref for TextToTextWorkerHandle {
    type Target = WorkerHandle;
    
    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

/// Worker loop for TextToText models
///
/// Processes streaming prompt requests. Worker calls trait method which
/// returns AsyncStream<CandleCompletionChunk>. Stream is sent back to caller
/// who forwards chunks to end user.
pub fn text_to_text_worker<T: TextToTextCapable>(
    model: T,
    prompt_rx: Receiver<PromptRequest>,
    shutdown_rx: Receiver<()>,
    health_rx: Receiver<HealthPing>,
    health_tx: Sender<HealthPong>,
    worker_id: usize,
    registry_key: String,
    state: Arc<AtomicU32>,
) {
    use std::time::Duration;
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
            recv(prompt_rx) -> req => {
                if let Ok(req) = req {
                    // Transition: Ready/Idle → Processing
                    state.store(WorkerState::Processing as u32, std::sync::atomic::Ordering::Release);
                    
                    // Model method returns AsyncStream directly
                    let stream = model.prompt(req.prompt, &req.params);
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
                        queue_depth: prompt_rx.len(),
                    };
                    
                    let _ = health_tx.send(pong);
                }
            }
            recv(shutdown_rx) -> _ => {
                log::info!("TextToText worker {} shutting down", worker_id);
                // Transition: Ready/Idle → Evicting
                state.store(WorkerState::Evicting as u32, std::sync::atomic::Ordering::Release);
                break;
            }
        }
    }
}

/// Global TextToText pool instance
static TEXT_TO_TEXT_POOL: Lazy<Pool<TextToTextWorkerHandle>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

/// Access global TextToText pool
pub fn text_to_text_pool() -> &'static Pool<TextToTextWorkerHandle> {
    &TEXT_TO_TEXT_POOL
}

impl Pool<TextToTextWorkerHandle> {
    /// Spawn worker for TextToText model
    pub fn spawn_text_to_text_worker<T, F>(
        &self,
        registry_key: &str,
        model_loader: F,
        per_worker_mb: usize,
        allocation_guard: AllocationGuard,
    ) -> Result<(), PoolError>
    where
        T: TextToTextCapable + Send + 'static,
        F: FnOnce() -> Result<T, PoolError> + Send + 'static,
    {

        // Create BOUNDED channels (prevent OOM)
        let (prompt_tx, prompt_rx) = bounded(self.config().prompt_queue_capacity);
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
            
            // Load model
            let model = match model_loader() {
                Ok(m) => {
                    log::info!("TextToText worker {} ready", worker_id);
                    // Transition: Loading → Ready
                    state_clone.store(WorkerState::Ready as u32, std::sync::atomic::Ordering::Release);
                    m
                }
                Err(e) => {
                    log::error!("TextToText worker {} failed: {}", worker_id, e);
                    // Transition: Loading → Failed
                    state_clone.store(WorkerState::Failed as u32, std::sync::atomic::Ordering::Release);
                    
                    // Clean up memory tracking
                    // This prevents memory leak when model loading fails
                    text_to_text_pool().remove_memory(per_worker_mb_clone);
                    
                    return; // Exit thread without running worker loop
                }
            };

            text_to_text_worker(
                model,
                prompt_rx,
                shutdown_rx,
                health_rx_worker_clone,
                health_tx_main_clone,
                worker_id,
                registry_key_clone.clone(),
                Arc::clone(&state_clone),
            );
            
            // Transition: Ready → Dead (when worker loop exits)
            state_clone.store(WorkerState::Dead as u32, std::sync::atomic::Ordering::Release);
        });

        // Create handles
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        let pending_requests = Arc::new(AtomicUsize::new(0));
        let last_used = Arc::new(AtomicU64::new(now));
        
        // Store capability-specific handle (state already created above before spawning)
        let full_handle = TextToTextWorkerHandle {
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
            prompt_tx,
            shutdown_tx,
            registry_key: registry_key_clone.clone(),
        };

        // Single registration point - no duplication
        self.register_worker(registry_key.to_string(), full_handle);

        // Update memory tracking
        self.add_memory(per_worker_mb);

        Ok(())
    }

    /// Generate completion using pooled worker (returns AsyncStream)
    pub fn prompt(
        &self,
        registry_key: &str,
        prompt: CandlePrompt,
        params: CandleCompletionParams,
    ) -> AsyncStream<CandleCompletionChunk> {
        // Clone for move into closure
        let registry_key = registry_key.to_string();
        let is_shutting_down = self.is_shutting_down();
        let request_timeout_secs = self.config().request_timeout_secs;

        spawn_stream(move |sender| {
            // Check shutdown
            if is_shutting_down {
                ystream::emit!(sender, CandleCompletionChunk::Error(
                    "Pool shutting down".to_string()
                ));
                return;
            }

            // Get circuit breaker for this model and check state
            let pool = text_to_text_pool();
            let circuit = pool.get_circuit_breaker(&registry_key);
            
            if !circuit.can_request() {
                ystream::emit!(sender, CandleCompletionChunk::Error(
                    format!("Circuit breaker open for {}", registry_key)
                ));
                // Update metrics
                pool.metrics().circuit_rejections.fetch_add(1, Ordering::Relaxed);
                return;
            }

            // Get workers from global worker storage
            let workers = match TEXT_TO_TEXT_WORKERS.get(&registry_key) {
                Some(w) => w,
                None => {
                    ystream::emit!(sender, CandleCompletionChunk::Error(
                        format!("No workers for {}", registry_key)
                    ));
                    return;
                }
            };

            if workers.is_empty() {
                ystream::emit!(sender, CandleCompletionChunk::Error(
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
                    ystream::emit!(sender, CandleCompletionChunk::Error(
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
            if let Err(e) = worker.prompt_tx.send(PromptRequest {
                prompt,
                params,
                response: response_tx,
            }) {
                ystream::emit!(sender, CandleCompletionChunk::Error(
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
                    
                    ystream::emit!(sender, CandleCompletionChunk::Error(
                        format!("Worker error: {}", e)
                    ));
                    worker.core.pending_requests.fetch_sub(1, Ordering::Release);
                    return;
                }
                Err(e) => {
                    // Record timeout as failure
                    circuit.record_failure();
                    pool.metrics().total_timeouts.fetch_add(1, Ordering::Relaxed);
                    
                    ystream::emit!(sender, CandleCompletionChunk::Error(
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


