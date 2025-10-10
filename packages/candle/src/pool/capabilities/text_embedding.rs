use crossbeam::channel::{Sender, Receiver, bounded, unbounded};
use crossbeam::select;
use once_cell::sync::Lazy;
use dashmap::DashMap;
use std::time::Duration;
use std::sync::atomic::Ordering;

use crate::pool::core::{Pool, PoolConfig, PoolError, WorkerHandle, query_system_memory_mb};
use crate::capability::traits::TextEmbeddingCapable;

/// Request for embed() operation
pub struct EmbedRequest {
    pub text: String,
    pub task: Option<String>,
    pub response: Sender<Result<Vec<f32>, PoolError>>,
}

/// Request for batch_embed() operation
pub struct BatchEmbedRequest {
    pub texts: Vec<String>,
    pub task: Option<String>,
    pub response: Sender<Result<Vec<Vec<f32>>, PoolError>>,
}

/// TextEmbedding-specific worker handle with channels
pub struct TextEmbeddingWorkerHandle {
    pub core: WorkerHandle,
    pub embed_tx: Sender<EmbedRequest>,
    pub batch_embed_tx: Sender<BatchEmbedRequest>,
    pub shutdown_tx: Sender<()>,
}

/// Worker loop for TextEmbedding models
///
/// Processes requests from 2 channels:
/// - embed_rx: Single text embedding
/// - batch_embed_rx: Batch text embedding
///
/// Worker owns model exclusively, processes requests until shutdown.
pub fn text_embedding_worker<T: TextEmbeddingCapable>(
    model: T,
    embed_rx: Receiver<EmbedRequest>,
    batch_embed_rx: Receiver<BatchEmbedRequest>,
    shutdown_rx: Receiver<()>,
) {
    loop {
        select! {
            recv(embed_rx) -> req => {
                if let Ok(req) = req {
                    let result = model.embed(&req.text, req.task)
                        .map_err(|e| PoolError::ModelError(e.to_string()));
                    let _ = req.response.send(result);
                }
            }
            recv(batch_embed_rx) -> req => {
                if let Ok(req) = req {
                    let result = model.batch_embed(&req.texts, req.task)
                        .map_err(|e| PoolError::ModelError(e.to_string()));
                    let _ = req.response.send(result);
                }
            }
            recv(shutdown_rx) -> _ => {
                log::info!("TextEmbedding worker shutting down");
                break;
            }
        }
    }
}

/// Global storage for TextEmbedding worker handles with channels
static TEXT_EMBEDDING_WORKERS: Lazy<DashMap<String, Vec<TextEmbeddingWorkerHandle>>> = Lazy::new(DashMap::new);

/// Global TextEmbedding pool instance  
static TEXT_EMBEDDING_POOL: Lazy<Pool<dyn TextEmbeddingCapable>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

/// Access global TextEmbedding pool
pub fn text_embedding_pool() -> &'static Pool<dyn TextEmbeddingCapable> {
    &TEXT_EMBEDDING_POOL
}

impl Pool<dyn TextEmbeddingCapable> {
    /// Spawn worker for TextEmbedding model
    pub fn spawn_text_embedding_worker<T, F>(
        &self,
        registry_key: &str,
        model_loader: F,
        per_worker_mb: usize,
    ) -> Result<(), PoolError>
    where
        T: TextEmbeddingCapable + Send + 'static,
        F: FnOnce() -> Result<T, PoolError> + Send + 'static,
    {
        // Inline memory check (can't call model_loader twice)
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
        let (embed_tx, embed_rx) = unbounded();
        let (batch_embed_tx, batch_embed_rx) = unbounded();
        let (shutdown_tx, shutdown_rx) = unbounded();

        // Get worker ID before moving into thread
        let worker_id = self.next_worker_id();
        let registry_key_clone = registry_key.to_string();

        // Spawn worker thread
        std::thread::spawn(move || {
            let model = match model_loader() {
                Ok(m) => m,
                Err(e) => {
                    log::error!("TextEmbedding worker {} model loading failed: {}", worker_id, e);
                    return;
                }
            };

            text_embedding_worker(model, embed_rx, batch_embed_rx, shutdown_rx);
        });

        // Register worker handles
        // Create shared Arc values for worker handle
        use std::sync::Arc;
        use std::sync::atomic::AtomicUsize;
        use std::sync::atomic::AtomicU64;
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        let pending_requests = Arc::new(AtomicUsize::new(0));
        let last_used = Arc::new(AtomicU64::new(now));
        
        // Create handle for pool registration
        let pool_handle = WorkerHandle {
            pending_requests: Arc::clone(&pending_requests),
            last_used: Arc::clone(&last_used),
            worker_id,
            shutdown_tx: shutdown_tx.clone(),
            per_worker_mb,
        };
        self.register_worker(registry_key.to_string(), pool_handle);

        // Create full handle with channels
        let full_handle = TextEmbeddingWorkerHandle {
            core: WorkerHandle {
                pending_requests,
                last_used,
                worker_id,
                shutdown_tx: shutdown_tx.clone(),
                per_worker_mb,
            },
            embed_tx,
            batch_embed_tx,
            shutdown_tx,
        };

        TEXT_EMBEDDING_WORKERS
            .entry(registry_key_clone)
            .or_insert_with(Vec::new)
            .push(full_handle);

        // Update memory tracking
        self.add_memory(per_worker_mb);

        Ok(())
    }

    /// Embed text using pooled worker
    pub fn embed_text(
        &self,
        registry_key: &str,
        text: &str,
        task: Option<String>,
    ) -> Result<Vec<f32>, PoolError> {
        // Check shutdown
        if self.is_shutting_down() {
            return Err(PoolError::ShuttingDown("Pool shutting down".to_string()));
        }

        // Get workers from TEXT_EMBEDDING_WORKERS map
        let workers = TEXT_EMBEDDING_WORKERS.get(registry_key)
            .ok_or_else(|| PoolError::NoWorkers(format!("No workers for {}", registry_key)))?;

        if workers.is_empty() {
            return Err(PoolError::NoWorkers("No workers available".to_string()));
        }

        // Find least busy worker
        let worker = workers.iter()
            .min_by_key(|w| w.core.pending_requests.load(Ordering::Acquire))
            .ok_or_else(|| PoolError::NoWorkers("No workers available".to_string()))?;

        // Send request
        worker.core.pending_requests.fetch_add(1, Ordering::Release);
        worker.core.touch();

        let (response_tx, response_rx) = bounded(0);
        worker.embed_tx.send(EmbedRequest {
            text: text.to_string(),
            task,
            response: response_tx,
        }).map_err(|e| PoolError::SendError(e.to_string()))?;

        // Wait for response with timeout
        let timeout = Duration::from_secs(self.config().request_timeout_secs);
        let result = response_rx.recv_timeout(timeout)
            .map_err(|e| PoolError::Timeout(e.to_string()))?;

        worker.core.pending_requests.fetch_sub(1, Ordering::Release);

        result
    }

    /// Batch embed texts using pooled worker
    pub fn batch_embed_text(
        &self,
        registry_key: &str,
        texts: &[String],
        task: Option<String>,
    ) -> Result<Vec<Vec<f32>>, PoolError> {
        // Check shutdown
        if self.is_shutting_down() {
            return Err(PoolError::ShuttingDown("Pool shutting down".to_string()));
        }

        // Get workers from TEXT_EMBEDDING_WORKERS map
        let workers = TEXT_EMBEDDING_WORKERS.get(registry_key)
            .ok_or_else(|| PoolError::NoWorkers(format!("No workers for {}", registry_key)))?;

        if workers.is_empty() {
            return Err(PoolError::NoWorkers("No workers available".to_string()));
        }

        // Find least busy worker
        let worker = workers.iter()
            .min_by_key(|w| w.core.pending_requests.load(Ordering::Acquire))
            .ok_or_else(|| PoolError::NoWorkers("No workers available".to_string()))?;

        // Send request
        worker.core.pending_requests.fetch_add(1, Ordering::Release);
        worker.core.touch();

        let (response_tx, response_rx) = bounded(0);
        worker.batch_embed_tx.send(BatchEmbedRequest {
            texts: texts.to_vec(),
            task,
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


