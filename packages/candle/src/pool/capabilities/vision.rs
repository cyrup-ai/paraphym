use crossbeam::channel::{unbounded, Receiver, Sender};
use crossbeam::select;
use once_cell::sync::Lazy;
use dashmap::DashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use ystream::{AsyncStream, spawn_stream};

use crate::pool::core::{Pool, PoolConfig, PoolError, WorkerHandle, query_system_memory_mb};
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
) {
    loop {
        select! {
            recv(describe_image_rx) -> req => {
                if let Ok(req) = req {
                    let stream = model.describe_image(&req.image_path, &req.query);
                    let _ = req.response.send(Ok(stream));
                }
            }
            recv(describe_url_rx) -> req => {
                if let Ok(req) = req {
                    let stream = model.describe_url(&req.url, &req.query);
                    let _ = req.response.send(Ok(stream));
                }
            }
            recv(shutdown_rx) -> _ => {
                log::info!("Vision worker shutting down");
                break;
            }
        }
    }
}

/// Global storage for Vision worker handles with channels
static VISION_WORKERS: Lazy<DashMap<String, Vec<VisionWorkerHandle>>> = 
    Lazy::new(DashMap::new);

/// Global Vision pool instance
static VISION_POOL: Lazy<Pool<dyn VisionCapable>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

/// Access global Vision pool
pub fn vision_pool() -> &'static Pool<dyn VisionCapable> {
    &VISION_POOL
}

impl Pool<dyn VisionCapable> {
    /// Spawn worker for Vision model
    pub fn spawn_vision_worker<T, F>(
        &self,
        registry_key: &str,
        model_loader: F,
        per_worker_mb: usize,
    ) -> Result<(), PoolError>
    where
        T: VisionCapable + Send + 'static,
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
        let (describe_image_tx, describe_image_rx) = unbounded();
        let (describe_url_tx, describe_url_rx) = unbounded();
        let (shutdown_tx, shutdown_rx) = unbounded();

        // Get worker ID before moving into thread
        let worker_id = self.next_worker_id();
        let registry_key_clone = registry_key.to_string();

        // Spawn worker thread
        std::thread::spawn(move || {
            let model = match model_loader() {
                Ok(m) => m,
                Err(e) => {
                    log::error!("Vision worker {} model loading failed: {}", worker_id, e);
                    return;
                }
            };

            vision_worker(model, describe_image_rx, describe_url_rx, shutdown_rx);
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
        };
        self.register_worker(registry_key.to_string(), pool_handle);

        // Store capability-specific handle
        let full_handle = VisionWorkerHandle {
            core: WorkerHandle {
                pending_requests,
                last_used,
                worker_id,
                shutdown_tx: shutdown_tx.clone(),
                per_worker_mb,
            },
            describe_image_tx,
            describe_url_tx,
            shutdown_tx,
        };

        VISION_WORKERS
            .entry(registry_key_clone)
            .or_insert_with(Vec::new)
            .push(full_handle);

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

            // Get workers from global worker storage
            let workers = match VISION_WORKERS.get(&registry_key) {
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

            // Find least busy worker
            let worker = match workers.iter()
                .min_by_key(|w| w.core.pending_requests.load(Ordering::Acquire))
            {
                Some(w) => w,
                None => {
                    ystream::emit!(sender, CandleStringChunk(
                        "No workers available".to_string()
                    ));
                    return;
                }
            };

            // Track request
            worker.core.pending_requests.fetch_add(1, Ordering::Release);
            worker.core.touch();

            // Send request to worker
            let (response_tx, response_rx) = crossbeam::channel::unbounded();
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
                Ok(Ok(stream)) => stream,
                Ok(Err(e)) => {
                    ystream::emit!(sender, CandleStringChunk(
                        format!("Worker error: {}", e)
                    ));
                    worker.core.pending_requests.fetch_sub(1, Ordering::Release);
                    return;
                }
                Err(e) => {
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

            // Get workers from global worker storage
            let workers = match VISION_WORKERS.get(&registry_key) {
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

            // Find least busy worker
            let worker = match workers.iter()
                .min_by_key(|w| w.core.pending_requests.load(Ordering::Acquire))
            {
                Some(w) => w,
                None => {
                    ystream::emit!(sender, CandleStringChunk(
                        "No workers available".to_string()
                    ));
                    return;
                }
            };

            // Track request
            worker.core.pending_requests.fetch_add(1, Ordering::Release);
            worker.core.touch();

            // Send request to worker
            let (response_tx, response_rx) = crossbeam::channel::unbounded();
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
                Ok(Ok(stream)) => stream,
                Ok(Err(e)) => {
                    ystream::emit!(sender, CandleStringChunk(
                        format!("Worker error: {}", e)
                    ));
                    worker.core.pending_requests.fetch_sub(1, Ordering::Release);
                    return;
                }
                Err(e) => {
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


