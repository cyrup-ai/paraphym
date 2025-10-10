use crossbeam::channel::{Sender, Receiver, bounded, unbounded};
use crossbeam::select;
use once_cell::sync::Lazy;
use dashmap::DashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};

use crate::pool::core::{Pool, PoolConfig, PoolError, WorkerHandle, query_system_memory_mb};
use crate::pool::core::types::{HealthPing, HealthPong, select_worker_power_of_two};
use crate::capability::traits::ImageEmbeddingCapable;

/// Request for embed_image() operation
pub struct EmbedImageRequest {
    pub image_path: String,
    pub response: Sender<Result<Vec<f32>, PoolError>>,
}

/// Request for embed_image_url() operation
pub struct EmbedImageUrlRequest {
    pub url: String,
    pub response: Sender<Result<Vec<f32>, PoolError>>,
}

/// Request for embed_image_base64() operation
pub struct EmbedImageBase64Request {
    pub base64_data: String,
    pub response: Sender<Result<Vec<f32>, PoolError>>,
}

/// Request for batch_embed_images() operation
pub struct BatchEmbedImagesRequest {
    pub image_paths: Vec<String>,
    pub response: Sender<Result<Vec<Vec<f32>>, PoolError>>,
}

/// ImageEmbedding-specific worker handle with channels
pub struct ImageEmbeddingWorkerHandle {
    pub core: WorkerHandle,
    pub embed_image_tx: Sender<EmbedImageRequest>,
    pub embed_image_url_tx: Sender<EmbedImageUrlRequest>,
    pub embed_image_base64_tx: Sender<EmbedImageBase64Request>,
    pub batch_embed_images_tx: Sender<BatchEmbedImagesRequest>,
    pub shutdown_tx: Sender<()>,
    pub registry_key: String,  // Added to enable cleanup on drop
}

impl std::ops::Deref for ImageEmbeddingWorkerHandle {
    type Target = WorkerHandle;
    
    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl Drop for ImageEmbeddingWorkerHandle {
    fn drop(&mut self) {
        // Clean up from global storage when handle is dropped
        // This prevents memory leak when workers are evicted
        if let Some(mut workers) = IMAGE_EMBEDDING_WORKERS.get_mut(&self.registry_key) {
            workers.retain(|w| w.core.worker_id != self.core.worker_id);
            log::debug!(
                "Cleaned up ImageEmbedding worker {} for {} from global storage",
                self.core.worker_id,
                self.registry_key
            );
        }
    }
}

/// Worker loop for ImageEmbedding models
///
/// Processes requests from 4 channels:
/// - embed_image_rx: Single image embedding from path
/// - embed_image_url_rx: Single image embedding from URL
/// - embed_image_base64_rx: Single image embedding from base64
/// - batch_embed_images_rx: Batch image embedding
///
/// Worker owns model exclusively, processes requests until shutdown.
pub fn image_embedding_worker<T: ImageEmbeddingCapable>(
    model: T,
    embed_image_rx: Receiver<EmbedImageRequest>,
    embed_image_url_rx: Receiver<EmbedImageUrlRequest>,
    embed_image_base64_rx: Receiver<EmbedImageBase64Request>,
    batch_embed_images_rx: Receiver<BatchEmbedImagesRequest>,
    shutdown_rx: Receiver<()>,
    health_rx: Receiver<HealthPing>,
    health_tx: Sender<HealthPong>,
    worker_id: usize,
) {
    loop {
        select! {
            recv(embed_image_rx) -> req => {
                if let Ok(req) = req {
                    let result = model.embed_image(&req.image_path)
                        .map_err(|e| PoolError::ModelError(e.to_string()));
                    let _ = req.response.send(result);
                }
            }
            recv(embed_image_url_rx) -> req => {
                if let Ok(req) = req {
                    let result = model.embed_image_url(&req.url)
                        .map_err(|e| PoolError::ModelError(e.to_string()));
                    let _ = req.response.send(result);
                }
            }
            recv(embed_image_base64_rx) -> req => {
                if let Ok(req) = req {
                    let result = model.embed_image_base64(&req.base64_data)
                        .map_err(|e| PoolError::ModelError(e.to_string()));
                    let _ = req.response.send(result);
                }
            }
            recv(batch_embed_images_rx) -> req => {
                if let Ok(req) = req {
                    let paths: Vec<&str> = req.image_paths.iter().map(|s| s.as_str()).collect();
                    let result = model.batch_embed_images(paths)
                        .map_err(|e| PoolError::ModelError(e.to_string()));
                    let _ = req.response.send(result);
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
                        queue_depth: embed_image_rx.len() + embed_image_url_rx.len() + embed_image_base64_rx.len() + batch_embed_images_rx.len(),
                    };
                    
                    let _ = health_tx.send(pong);
                }
            }
            recv(shutdown_rx) -> _ => {
                log::info!("ImageEmbedding worker {} shutting down", worker_id);
                break;
            }
        }
    }
}

/// Global storage for ImageEmbedding worker handles with channels
static IMAGE_EMBEDDING_WORKERS: Lazy<DashMap<String, Vec<ImageEmbeddingWorkerHandle>>> = Lazy::new(DashMap::new);

/// Global ImageEmbedding pool instance
static IMAGE_EMBEDDING_POOL: Lazy<Pool<dyn ImageEmbeddingCapable>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

/// Access global ImageEmbedding pool
pub fn image_embedding_pool() -> &'static Pool<dyn ImageEmbeddingCapable> {
    &IMAGE_EMBEDDING_POOL
}

impl Pool<dyn ImageEmbeddingCapable> {
    /// Spawn worker for ImageEmbedding model
    pub fn spawn_image_embedding_worker<T, F>(
        &self,
        registry_key: &str,
        model_loader: F,
        per_worker_mb: usize,
    ) -> Result<(), PoolError>
    where
        T: ImageEmbeddingCapable + Send + 'static,
        F: FnOnce() -> Result<T, PoolError> + Send + 'static,
    {
        // Memory check
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
        let (embed_image_tx, embed_image_rx) = unbounded();
        let (embed_image_url_tx, embed_image_url_rx) = unbounded();
        let (embed_image_base64_tx, embed_image_base64_rx) = unbounded();
        let (batch_embed_images_tx, batch_embed_images_rx) = unbounded();
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
                    log::error!("ImageEmbedding worker {} model loading failed: {}", worker_id, e);
                    return;
                }
            };

            image_embedding_worker(
                model,
                embed_image_rx,
                embed_image_url_rx,
                embed_image_base64_rx,
                batch_embed_images_rx,
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
        let full_handle = ImageEmbeddingWorkerHandle {
            core: WorkerHandle {
                pending_requests: Arc::clone(&pending_requests),
                last_used: Arc::clone(&last_used),
                worker_id,
                shutdown_tx: shutdown_tx.clone(),
                per_worker_mb,
                health_tx: health_tx_worker,
                health_rx: health_rx_main,
            },
            embed_image_tx,
            embed_image_url_tx,
            embed_image_base64_tx,
            batch_embed_images_tx,
            shutdown_tx,
            registry_key: registry_key_clone.clone(),  // Store for cleanup on drop
        };

        IMAGE_EMBEDDING_WORKERS
            .entry(registry_key_clone)
            .or_insert_with(Vec::new)
            .push(full_handle);

        // Update memory tracking
        self.add_memory(per_worker_mb);

        Ok(())
    }

    /// Embed image using pooled worker
    pub fn embed_image(
        &self,
        registry_key: &str,
        image_path: &str,
    ) -> Result<Vec<f32>, PoolError> {
        // Check shutdown
        if self.is_shutting_down() {
            return Err(PoolError::ShuttingDown("Pool shutting down".to_string()));
        }

        // Get workers from IMAGE_EMBEDDING_WORKERS map
        let workers = IMAGE_EMBEDDING_WORKERS.get(registry_key)
            .ok_or_else(|| PoolError::NoWorkers(format!("No workers for {}", registry_key)))?;

        if workers.is_empty() {
            return Err(PoolError::NoWorkers("No workers available".to_string()));
        }

        // Find alive worker with least load using Power of Two Choices (O(1))
        let alive_workers: Vec<_> = workers.iter().filter(|w| w.is_alive()).collect();
        let worker = select_worker_power_of_two(&alive_workers, |w| &w.core)
            .ok_or_else(|| PoolError::NoWorkers(format!("No alive workers for {}", registry_key)))?;

        // Send request
        worker.core.pending_requests.fetch_add(1, Ordering::Release);
        worker.core.touch();

        let (response_tx, response_rx) = bounded(0);
        worker.embed_image_tx.send(EmbedImageRequest {
            image_path: image_path.to_string(),
            response: response_tx,
        }).map_err(|e| PoolError::SendError(e.to_string()))?;

        // Wait for response with timeout
        let timeout = Duration::from_secs(self.config().request_timeout_secs);
        let result = response_rx.recv_timeout(timeout)
            .map_err(|e| PoolError::Timeout(e.to_string()))?;

        worker.core.pending_requests.fetch_sub(1, Ordering::Release);

        result
    }

    /// Embed image from URL using pooled worker
    pub fn embed_image_url(
        &self,
        registry_key: &str,
        url: &str,
    ) -> Result<Vec<f32>, PoolError> {
        // Check shutdown
        if self.is_shutting_down() {
            return Err(PoolError::ShuttingDown("Pool shutting down".to_string()));
        }

        // Get workers from IMAGE_EMBEDDING_WORKERS map
        let workers = IMAGE_EMBEDDING_WORKERS.get(registry_key)
            .ok_or_else(|| PoolError::NoWorkers(format!("No workers for {}", registry_key)))?;

        if workers.is_empty() {
            return Err(PoolError::NoWorkers("No workers available".to_string()));
        }

        // Find alive worker with least load using Power of Two Choices (O(1))
        let alive_workers: Vec<_> = workers.iter().filter(|w| w.is_alive()).collect();
        let worker = select_worker_power_of_two(&alive_workers, |w| &w.core)
            .ok_or_else(|| PoolError::NoWorkers(format!("No alive workers for {}", registry_key)))?;

        // Send request
        worker.core.pending_requests.fetch_add(1, Ordering::Release);
        worker.core.touch();

        let (response_tx, response_rx) = bounded(0);
        worker.embed_image_url_tx.send(EmbedImageUrlRequest {
            url: url.to_string(),
            response: response_tx,
        }).map_err(|e| PoolError::SendError(e.to_string()))?;

        // Wait for response with timeout
        let timeout = Duration::from_secs(self.config().request_timeout_secs);
        let result = response_rx.recv_timeout(timeout)
            .map_err(|e| PoolError::Timeout(e.to_string()))?;

        worker.core.pending_requests.fetch_sub(1, Ordering::Release);

        result
    }

    /// Embed image from base64 data using pooled worker
    pub fn embed_image_base64(
        &self,
        registry_key: &str,
        base64_data: &str,
    ) -> Result<Vec<f32>, PoolError> {
        // Check shutdown
        if self.is_shutting_down() {
            return Err(PoolError::ShuttingDown("Pool shutting down".to_string()));
        }

        // Get workers from IMAGE_EMBEDDING_WORKERS map
        let workers = IMAGE_EMBEDDING_WORKERS.get(registry_key)
            .ok_or_else(|| PoolError::NoWorkers(format!("No workers for {}", registry_key)))?;

        if workers.is_empty() {
            return Err(PoolError::NoWorkers("No workers available".to_string()));
        }

        // Find alive worker with least load using Power of Two Choices (O(1))
        let alive_workers: Vec<_> = workers.iter().filter(|w| w.is_alive()).collect();
        let worker = select_worker_power_of_two(&alive_workers, |w| &w.core)
            .ok_or_else(|| PoolError::NoWorkers(format!("No alive workers for {}", registry_key)))?;

        // Send request
        worker.core.pending_requests.fetch_add(1, Ordering::Release);
        worker.core.touch();

        let (response_tx, response_rx) = bounded(0);
        worker.embed_image_base64_tx.send(EmbedImageBase64Request {
            base64_data: base64_data.to_string(),
            response: response_tx,
        }).map_err(|e| PoolError::SendError(e.to_string()))?;

        // Wait for response with timeout
        let timeout = Duration::from_secs(self.config().request_timeout_secs);
        let result = response_rx.recv_timeout(timeout)
            .map_err(|e| PoolError::Timeout(e.to_string()))?;

        worker.core.pending_requests.fetch_sub(1, Ordering::Release);

        result
    }

    /// Batch embed images using pooled worker
    pub fn batch_embed_images(
        &self,
        registry_key: &str,
        image_paths: &[String],
    ) -> Result<Vec<Vec<f32>>, PoolError> {
        // Check shutdown
        if self.is_shutting_down() {
            return Err(PoolError::ShuttingDown("Pool shutting down".to_string()));
        }

        // Get workers from IMAGE_EMBEDDING_WORKERS map
        let workers = IMAGE_EMBEDDING_WORKERS.get(registry_key)
            .ok_or_else(|| PoolError::NoWorkers(format!("No workers for {}", registry_key)))?;

        if workers.is_empty() {
            return Err(PoolError::NoWorkers("No workers available".to_string()));
        }

        // Find alive worker with least load using Power of Two Choices (O(1))
        let alive_workers: Vec<_> = workers.iter().filter(|w| w.is_alive()).collect();
        let worker = select_worker_power_of_two(&alive_workers, |w| &w.core)
            .ok_or_else(|| PoolError::NoWorkers(format!("No alive workers for {}", registry_key)))?;

        // Send request
        worker.core.pending_requests.fetch_add(1, Ordering::Release);
        worker.core.touch();

        let (response_tx, response_rx) = bounded(0);
        worker.batch_embed_images_tx.send(BatchEmbedImagesRequest {
            image_paths: image_paths.to_vec(),
            response: response_tx,
        }).map_err(|e| PoolError::SendError(e.to_string()))?;

        // Wait for response with timeout
        let timeout = Duration::from_secs(self.config().request_timeout_secs);
        let result = response_rx.recv_timeout(timeout)
            .map_err(|e| PoolError::Timeout(e.to_string()))?;

        worker.core.pending_requests.fetch_sub(1, Ordering::Release);

        result
    }
}


