use crossbeam::channel::{unbounded, Receiver, Sender};
use crossbeam::select;
use once_cell::sync::Lazy;
use dashmap::DashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use ystream::{AsyncStream, spawn_stream};
use candle_core::Device;

use crate::pool::core::{Pool, PoolConfig, PoolError, WorkerHandle, query_system_memory_mb};
use crate::pool::core::types::{HealthPing, HealthPong, select_worker_power_of_two};
use crate::capability::traits::TextToImageCapable;
use crate::domain::image_generation::{ImageGenerationChunk, ImageGenerationConfig};

/// Request for generate_image() operation (streaming response)
pub struct GenerateImageRequest {
    pub prompt: String,
    pub config: ImageGenerationConfig,
    pub device: Device,
    pub response: Sender<Result<AsyncStream<ImageGenerationChunk>, PoolError>>,
}

/// TextToImage-specific worker handle with channel
pub struct TextToImageWorkerHandle {
    pub core: WorkerHandle,
    pub generate_image_tx: Sender<GenerateImageRequest>,
    pub shutdown_tx: Sender<()>,
    pub registry_key: String,  // Added to enable cleanup on drop
}

impl std::ops::Deref for TextToImageWorkerHandle {
    type Target = WorkerHandle;
    
    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl Drop for TextToImageWorkerHandle {
    fn drop(&mut self) {
        // Clean up from global storage when handle is dropped
        // This prevents memory leak when workers are evicted
        if let Some(mut workers) = TEXT_TO_IMAGE_WORKERS.get_mut(&self.registry_key) {
            workers.retain(|w| w.core.worker_id != self.core.worker_id);
            log::debug!(
                "Cleaned up TextToImage worker {} for {} from global storage",
                self.core.worker_id,
                self.registry_key
            );
        }
    }
}

/// Worker loop for TextToImage models
///
/// Processes streaming requests. Worker calls trait method which
/// returns AsyncStream<ImageGenerationChunk>. Stream is sent back to caller
/// who forwards chunks to end user.
pub fn text_to_image_worker<T: TextToImageCapable>(
    model: T,
    generate_image_rx: Receiver<GenerateImageRequest>,
    shutdown_rx: Receiver<()>,
    health_rx: Receiver<HealthPing>,
    health_tx: Sender<HealthPong>,
    worker_id: usize,
) {
    loop {
        select! {
            recv(generate_image_rx) -> req => {
                if let Ok(req) = req {
                    let stream = model.generate_image(&req.prompt, &req.config, &req.device);
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
                        queue_depth: generate_image_rx.len(),
                    };
                    
                    let _ = health_tx.send(pong);
                }
            }
            recv(shutdown_rx) -> _ => {
                log::info!("TextToImage worker {} shutting down", worker_id);
                break;
            }
        }
    }
}

/// Global storage for TextToImage worker handles with channels
static TEXT_TO_IMAGE_WORKERS: Lazy<DashMap<String, Vec<TextToImageWorkerHandle>>> = 
    Lazy::new(DashMap::new);

/// Global TextToImage pool instance
static TEXT_TO_IMAGE_POOL: Lazy<Pool<dyn TextToImageCapable>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

/// Access global TextToImage pool
pub fn text_to_image_pool() -> &'static Pool<dyn TextToImageCapable> {
    &TEXT_TO_IMAGE_POOL
}

impl Pool<dyn TextToImageCapable> {
    /// Spawn worker for TextToImage model
    pub fn spawn_text_to_image_worker<T, F>(
        &self,
        registry_key: &str,
        model_loader: F,
        per_worker_mb: usize,
    ) -> Result<(), PoolError>
    where
        T: TextToImageCapable + Send + 'static,
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
        let (generate_image_tx, generate_image_rx) = unbounded();
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
                    log::error!("TextToImage worker {} model loading failed: {}", worker_id, e);
                    return;
                }
            };

            text_to_image_worker(
                model,
                generate_image_rx,
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
        let full_handle = TextToImageWorkerHandle {
            core: WorkerHandle {
                pending_requests: Arc::clone(&pending_requests),
                last_used: Arc::clone(&last_used),
                worker_id,
                shutdown_tx: shutdown_tx.clone(),
                per_worker_mb,
                health_tx: health_tx_worker,
                health_rx: health_rx_main,
            },
            generate_image_tx,
            shutdown_tx,
            registry_key: registry_key_clone.clone(),  // Store for cleanup on drop
        };

        TEXT_TO_IMAGE_WORKERS
            .entry(registry_key_clone)
            .or_insert_with(Vec::new)
            .push(full_handle);

        // Update memory tracking
        self.add_memory(per_worker_mb);

        Ok(())
    }

    /// Generate image using pooled worker (returns AsyncStream)
    pub fn generate_image(
        &self,
        registry_key: &str,
        prompt: &str,
        config: &ImageGenerationConfig,
        device: &Device,
    ) -> AsyncStream<ImageGenerationChunk> {
        // Clone for move into closure
        let registry_key = registry_key.to_string();
        let prompt = prompt.to_string();
        let config = config.clone();
        let device = device.clone();
        let is_shutting_down = self.is_shutting_down();
        let request_timeout_secs = self.config().request_timeout_secs;

        spawn_stream(move |sender| {
            // Check shutdown
            if is_shutting_down {
                ystream::emit!(sender, ImageGenerationChunk::Error(
                    "Pool shutting down".to_string()
                ));
                return;
            }

            // Get workers from global worker storage
            let workers = match TEXT_TO_IMAGE_WORKERS.get(&registry_key) {
                Some(w) => w,
                None => {
                    ystream::emit!(sender, ImageGenerationChunk::Error(
                        format!("No workers for {}", registry_key)
                    ));
                    return;
                }
            };

            if workers.is_empty() {
                ystream::emit!(sender, ImageGenerationChunk::Error(
                    "No workers available".to_string()
                ));
                return;
            }

            // Find alive worker with least load using Power of Two Choices (O(1))
            let alive_workers: Vec<_> = workers.iter().filter(|w| w.is_alive()).collect();
            let worker = match select_worker_power_of_two(&alive_workers, |w| &w.core) {
                Some(w) => w,
                None => {
                    ystream::emit!(sender, ImageGenerationChunk::Error(
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
            if let Err(e) = worker.generate_image_tx.send(GenerateImageRequest {
                prompt,
                config,
                device,
                response: response_tx,
            }) {
                ystream::emit!(sender, ImageGenerationChunk::Error(
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
                    ystream::emit!(sender, ImageGenerationChunk::Error(
                        format!("Worker error: {}", e)
                    ));
                    worker.core.pending_requests.fetch_sub(1, Ordering::Release);
                    return;
                }
                Err(e) => {
                    ystream::emit!(sender, ImageGenerationChunk::Error(
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


