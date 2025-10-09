# MPOOL_3A: Implement TextEmbedding Capability Module

**PREFIX**: MPOOL (Model Pool)

## OBJECTIVE

Implement `pool/capabilities/text_embedding.rs` with TextEmbedding-specific Request/Response structs, worker loop, global pool instance, and API methods. This instantiates the generic Pool<T> for TextEmbeddingCapable trait.

## CONTEXT

TextEmbedding has 2 methods: `embed()` and `batch_embed()`. Each gets dedicated Request/Response structs and channel. Worker loop uses crossbeam select! for multi-channel handling. Global TEXT_EMBEDDING_POOL singleton is lazy-initialized.

## SUBTASK 1: Create Capability Module Files

**Create files**:
- `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/mod.rs`
- `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/text_embedding.rs`

## SUBTASK 2: Implement Request/Response Structs

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/text_embedding.rs`

**Implementation**:
```rust
use crossbeam::channel::{Sender, Receiver};
use crate::pool::core::PoolError;

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
```

**Why**: Type-safe request passing (Pattern A from MODEL_POOL.md).

## SUBTASK 3: Implement Worker Handle Extension

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/text_embedding.rs`

**Implementation**:
```rust
use crate::pool::core::WorkerHandle as CoreHandle;
use crossbeam::channel::Sender;

/// TextEmbedding-specific worker handle with channels
pub struct TextEmbeddingWorkerHandle {
    pub core: CoreHandle,
    pub embed_tx: Sender<EmbedRequest>,
    pub batch_embed_tx: Sender<BatchEmbedRequest>,
}
```

**Why**: Capability-specific channel storage (Scenario 3).

## SUBTASK 4: Implement Worker Loop

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/text_embedding.rs`

**Implementation**:
```rust
use crate::capability::traits::TextEmbeddingCapable;
use crossbeam::channel::Receiver;
use crossbeam::select;

/// Worker loop for TextEmbedding models
///
/// Processes requests from 2 channels:
/// - embed_rx: Single text embedding
/// - batch_embed_rx: Batch text embedding
///
/// Worker owns model exclusively, processes requests until shutdown.
pub fn text_embedding_worker<T: TextEmbeddingCapable>(
    mut model: T,
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
```

**Why**: Generic worker loop over TextEmbeddingCapable trait (Scenario 3, workers self-schedule).

## SUBTASK 5: Implement Global Pool Instance

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/text_embedding.rs`

**Implementation**:
```rust
use once_cell::sync::Lazy;
use crate::pool::core::{Pool, PoolConfig};
use crate::capability::traits::TextEmbeddingCapable;

/// Global TextEmbedding pool instance
static TEXT_EMBEDDING_POOL: Lazy<Pool<dyn TextEmbeddingCapable>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

/// Access global TextEmbedding pool
pub fn text_embedding_pool() -> &'static Pool<dyn TextEmbeddingCapable> {
    &TEXT_EMBEDDING_POOL
}
```

**Why**: Singleton pool instance per MODEL_POOL.md architecture.

## SUBTASK 6: Implement Pool API Methods

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/text_embedding.rs`

**Implementation**:
```rust
use std::time::Duration;
use crossbeam::channel;

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
        // Check memory
        crate::pool::core::worker::check_memory_available(
            &model_loader()?,
            self.total_memory_mb(),
            1,
        )?;

        // Create channels
        let (embed_tx, embed_rx) = channel::unbounded();
        let (batch_embed_tx, batch_embed_rx) = channel::unbounded();
        let (shutdown_tx, shutdown_rx) = channel::unbounded();

        // Spawn worker thread
        let worker_id = self.next_worker_id();
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

        // Register worker
        let handle = TextEmbeddingWorkerHandle {
            core: WorkerHandle::new(worker_id),
            embed_tx,
            batch_embed_tx,
        };
        self.register_worker(registry_key.to_string(), handle);

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

        // Get workers
        let workers = self.workers.get(registry_key)
            .ok_or_else(|| PoolError::NoWorkers(format!("No workers for {}", registry_key)))?;

        // Find least busy worker
        let worker = workers.iter()
            .min_by_key(|w| w.core.pending_requests.load(std::sync::atomic::Ordering::Acquire))
            .ok_or_else(|| PoolError::NoWorkers("No workers available".to_string()))?;

        // Send request
        worker.core.pending_requests.fetch_add(1, std::sync::atomic::Ordering::Release);
        worker.core.touch();

        let (response_tx, response_rx) = channel::bounded(0);
        worker.embed_tx.send(EmbedRequest {
            text: text.to_string(),
            task,
            response: response_tx,
        }).map_err(|e| PoolError::SendError(e.to_string()))?;

        // Wait for response with timeout
        let timeout = Duration::from_secs(self.config().request_timeout_secs);
        let result = response_rx.recv_timeout(timeout)
            .map_err(|e| PoolError::RecvError(e.to_string()))?;

        worker.core.pending_requests.fetch_sub(1, std::sync::atomic::Ordering::Release);

        result
    }

    /// Batch embed texts using pooled worker
    pub fn batch_embed_text(
        &self,
        registry_key: &str,
        texts: &[String],
        task: Option<String>,
    ) -> Result<Vec<Vec<f32>>, PoolError> {
        // Similar pattern to embed_text but uses batch channel
        // Implementation follows same structure
        todo!("Implement following embed_text pattern")
    }
}
```

**Why**: Pool API for spawning workers and routing requests (Scenario 2, Scenario 3).

## SUBTASK 7: Wire Up Module Exports

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/mod.rs`

```rust
pub mod text_embedding;

pub use text_embedding::text_embedding_pool;
```

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/mod.rs`

```rust
pub mod core;
pub mod capabilities;

pub use core::{Pool, PoolConfig, PoolError};
pub use capabilities::text_embedding_pool;  // NEW
```

## DEFINITION OF DONE

- [ ] `text_embedding.rs` file created
- [ ] `EmbedRequest`, `BatchEmbedRequest` structs implemented
- [ ] `TextEmbeddingWorkerHandle` struct implemented
- [ ] `text_embedding_worker()` loop implemented with crossbeam select!
- [ ] Global `TEXT_EMBEDDING_POOL` instance created
- [ ] `text_embedding_pool()` accessor function implemented
- [ ] `spawn_text_embedding_worker()` method implemented
- [ ] `embed_text()` method implemented
- [ ] `batch_embed_text()` method implemented
- [ ] Module exports configured
- [ ] Code compiles with `cargo check`

## DEPENDENCIES

**Requires**: MPOOL_2A (Pool<T>), MPOOL_2B (worker helpers)

**Blocks**: MPOOL_4A/B (LoadedModel wrappers call these APIs), MPOOL_5 (registry integration)

## RESEARCH NOTES

**Request Pattern** (from MODEL_POOL.md Pattern A):
```rust
fn embed(&self, text: &str, task: Option<String>) -> Result<Vec<f32>, BoxError> {
    let (response_tx, response_rx) = oneshot::channel();
    let request = EmbedRequest { text: text.to_string(), task, response: response_tx };

    pool.send_to_worker(registry_key, request)?;
    response_rx.recv_timeout(Duration::from_secs(30))?
}
```

**Self-Scheduling** (from Scenario 3):
```
Client → [embed_request] → SharedQueue → Worker #1 (busy, not listening)
                                       → Worker #2 (calls recv(), gets request!)
                                       → Worker #3 (busy, not listening)
```

Workers self-schedule - no dispatch logic needed.

## CONSTRAINTS

- **NO TESTS**: Do not write any test code. Tests are handled by separate team.
- **NO BENCHMARKS**: Do not write any benchmark code. Benchmarks are handled by separate team.
- **GENERIC TRAIT**: Worker loop works for ANY TextEmbeddingCapable implementation.
