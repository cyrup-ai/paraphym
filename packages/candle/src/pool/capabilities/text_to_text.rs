use crossbeam::channel::{unbounded, Receiver, Sender};
use crossbeam::select;
use once_cell::sync::Lazy;
use dashmap::DashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use ystream::{AsyncStream, spawn_stream};

use crate::pool::core::{Pool, PoolConfig, PoolError, WorkerHandle, query_system_memory_mb};
use crate::pool::core::types::{select_worker_power_of_two, HealthPing, HealthPong};
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
pub struct TextToTextWorkerHandle {
    pub core: WorkerHandle,
    pub prompt_tx: Sender<PromptRequest>,
    pub shutdown_tx: Sender<()>,
    pub registry_key: String,  // Added to enable cleanup on drop
}

impl std::ops::Deref for TextToTextWorkerHandle {
    type Target = WorkerHandle;
    
    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl Drop for TextToTextWorkerHandle {
    fn drop(&mut self) {
        // Clean up from global storage when handle is dropped
        // This prevents memory leak when workers are evicted
        if let Some(mut workers) = TEXT_TO_TEXT_WORKERS.get_mut(&self.registry_key) {
            workers.retain(|w| w.core.worker_id != self.core.worker_id);
            log::debug!(
                "Cleaned up TextToText worker {} for {} from global storage",
                self.core.worker_id,
                self.registry_key
            );
        }
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
) {
    loop {
        select! {
            recv(prompt_rx) -> req => {
                if let Ok(req) = req {
                    // Model method returns AsyncStream directly
                    let stream = model.prompt(req.prompt, &req.params);
                    let _ = req.response.send(Ok(stream));
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
                break;
            }
        }
    }
}

/// Global storage for TextToText worker handles with channels
static TEXT_TO_TEXT_WORKERS: Lazy<DashMap<String, Vec<TextToTextWorkerHandle>>> = 
    Lazy::new(DashMap::new);

/// Global TextToText pool instance
static TEXT_TO_TEXT_POOL: Lazy<Pool<dyn TextToTextCapable>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

/// Access global TextToText pool
pub fn text_to_text_pool() -> &'static Pool<dyn TextToTextCapable> {
    &TEXT_TO_TEXT_POOL
}

impl Pool<dyn TextToTextCapable> {
    /// Spawn worker for TextToText model
    pub fn spawn_text_to_text_worker<T, F>(
        &self,
        registry_key: &str,
        model_loader: F,
        per_worker_mb: usize,
    ) -> Result<(), PoolError>
    where
        T: TextToTextCapable + Send + 'static,
        F: FnOnce() -> Result<T, PoolError> + Send + 'static,
    {
        // Check memory availability
        let current_memory = self.total_memory_mb();
        let total_system_mb = query_system_memory_mb();
        let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;
        
        if current_memory + per_worker_mb > memory_limit_mb {
            return Err(PoolError::MemoryExhausted(format!(
                "Cannot spawn worker ({} MB). Current: {} MB, Limit: {} MB (80% of {})",
                per_worker_mb, current_memory, memory_limit_mb, total_system_mb
            )));
        }

        // Create channels
        let (prompt_tx, prompt_rx) = unbounded();
        let (shutdown_tx, shutdown_rx) = unbounded();
        let (health_tx_worker, health_rx_worker) = unbounded::<HealthPing>();
        let (health_tx_main, health_rx_main) = unbounded::<HealthPong>();

        // Get worker ID before moving into thread
        let worker_id = self.next_worker_id();
        let registry_key_clone = registry_key.to_string();

        // Clone channels for worker thread
        let health_rx_worker_clone = health_rx_worker.clone();
        let health_tx_main_clone = health_tx_main.clone();

        // Spawn worker thread
        std::thread::spawn(move || {
            let model = match model_loader() {
                Ok(m) => m,
                Err(e) => {
                    log::error!("TextToText worker {} model loading failed: {}", worker_id, e);
                    return;
                }
            };

            text_to_text_worker(
                model,
                prompt_rx,
                shutdown_rx,
                health_rx_worker_clone,
                health_tx_main_clone,
                worker_id,
            );
        });

        // Create handles
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        let pending_requests = Arc::new(AtomicUsize::new(0));
        let last_used = Arc::new(AtomicU64::new(now));
        
        // Register with pool core
        let pool_handle = WorkerHandle {
            pending_requests: Arc::clone(&pending_requests),
            last_used: Arc::clone(&last_used),
            worker_id,
            shutdown_tx: shutdown_tx.clone(),
            per_worker_mb,
            health_tx: health_tx_worker.clone(),
            health_rx: health_rx_main.clone(),
        };
        self.register_worker(registry_key.to_string(), pool_handle);

        // Store capability-specific handle
        let full_handle = TextToTextWorkerHandle {
            core: WorkerHandle {
                pending_requests: Arc::clone(&pending_requests),
                last_used: Arc::clone(&last_used),
                worker_id,
                shutdown_tx: shutdown_tx.clone(),
                per_worker_mb,
                health_tx: health_tx_worker,
                health_rx: health_rx_main,
            },
            prompt_tx,
            shutdown_tx,
            registry_key: registry_key_clone.clone(),  // Store for cleanup on drop
        };

        TEXT_TO_TEXT_WORKERS
            .entry(registry_key_clone)
            .or_insert_with(Vec::new)
            .push(full_handle);

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
            let (response_tx, response_rx) = crossbeam::channel::unbounded();
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
                Ok(Ok(stream)) => stream,
                Ok(Err(e)) => {
                    ystream::emit!(sender, CandleCompletionChunk::Error(
                        format!("Worker error: {}", e)
                    ));
                    worker.core.pending_requests.fetch_sub(1, Ordering::Release);
                    return;
                }
                Err(e) => {
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


