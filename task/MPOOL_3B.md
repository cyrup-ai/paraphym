# MPOOL_3B: Implement TextToText Capability Module

**PREFIX**: MPOOL (Model Pool)

## OBJECTIVE

Implement `pool/capabilities/text_to_text.rs` with TextToText-specific Request/Response structs, worker loop returning AsyncStream, global pool instance, and API methods. This instantiates Pool<T> for TextToTextCapable trait.

## CONTEXT

TextToText has `prompt()` method returning AsyncStream (streaming response). Worker loop uses ystream AsyncStream::with_channel() pattern. Reference: kimi_k2.rs:83-96 for canonical sync method returning AsyncStream.

## SUBTASK 1: Create Module File

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/text_to_text.rs`

## SUBTASK 2: Implement Request/Response Structs

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/text_to_text.rs`

**Implementation**:
```rust
use crossbeam::channel::Sender;
use crate::pool::core::PoolError;
use crate::domain::chat::CandlePrompt;
use crate::core::completion::CandleCompletionParams;
use ystream::AsyncStream;
use crate::domain::chat::message::CandleCompletionChunk;

/// Request for prompt() operation (streaming response)
pub struct PromptRequest {
    pub prompt: CandlePrompt,
    pub params: CandleCompletionParams,
    pub response: Sender<Result<AsyncStream<CandleCompletionChunk>, PoolError>>,
}
```

**Why**: Type-safe request passing for streaming operations (Pattern B from MODEL_POOL.md).

## SUBTASK 3: Implement Worker Handle Extension

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/text_to_text.rs`

**Implementation**:
```rust
use crate::pool::core::WorkerHandle as CoreHandle;
use crossbeam::channel::Sender;

/// TextToText-specific worker handle with channel
pub struct TextToTextWorkerHandle {
    pub core: CoreHandle,
    pub prompt_tx: Sender<PromptRequest>,
}
```

**Why**: Single channel for prompt requests (Scenario 3).

## SUBTASK 4: Implement Worker Loop

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/text_to_text.rs`

**Implementation**:
```rust
use crate::capability::traits::TextToTextCapable;
use crossbeam::channel::Receiver;
use crossbeam::select;

/// Worker loop for TextToText models
///
/// Processes streaming prompt requests. Worker calls trait method which
/// returns AsyncStream<CandleCompletionChunk>. Stream is sent back to caller
/// who forwards chunks to end user.
pub fn text_to_text_worker<T: TextToTextCapable>(
    model: T,
    prompt_rx: Receiver<PromptRequest>,
    shutdown_rx: Receiver<()>,
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
            recv(shutdown_rx) -> _ => {
                log::info!("TextToText worker shutting down");
                break;
            }
        }
    }
}
```

**Why**: Generic worker loop over TextToTextCapable trait, returns streams (Scenario 3).

## SUBTASK 5: Implement Global Pool Instance

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/text_to_text.rs`

**Implementation**:
```rust
use once_cell::sync::Lazy;
use crate::pool::core::{Pool, PoolConfig};
use crate::capability::traits::TextToTextCapable;

/// Global TextToText pool instance
static TEXT_TO_TEXT_POOL: Lazy<Pool<dyn TextToTextCapable>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

/// Access global TextToText pool
pub fn text_to_text_pool() -> &'static Pool<dyn TextToTextCapable> {
    &TEXT_TO_TEXT_POOL
}
```

**Why**: Singleton pool instance per MODEL_POOL.md architecture.

## SUBTASK 6: Implement Pool API Methods

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/text_to_text.rs`

**Implementation**:
```rust
use std::time::Duration;
use crossbeam::channel;

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
        // Check memory (similar to text_embedding)
        crate::pool::core::worker::check_memory_available(
            &model_loader()?,
            self.total_memory_mb(),
            1,
        )?;

        // Create channels
        let (prompt_tx, prompt_rx) = channel::unbounded();
        let (shutdown_tx, shutdown_rx) = channel::unbounded();

        // Spawn worker thread
        let worker_id = self.next_worker_id();
        std::thread::spawn(move || {
            let model = match model_loader() {
                Ok(m) => m,
                Err(e) => {
                    log::error!("TextToText worker {} model loading failed: {}", worker_id, e);
                    return;
                }
            };

            text_to_text_worker(model, prompt_rx, shutdown_rx);
        });

        // Register worker
        let handle = TextToTextWorkerHandle {
            core: WorkerHandle::new(worker_id),
            prompt_tx,
        };
        self.register_worker(registry_key.to_string(), handle);

        // Update memory tracking
        self.add_memory(per_worker_mb);

        Ok(())
    }

    /// Generate completion using pooled worker (returns AsyncStream)
    pub fn prompt(
        &self,
        registry_key: &str,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> AsyncStream<CandleCompletionChunk> {
        use ystream::AsyncStream;

        AsyncStream::with_channel(move |sender| {
            // Check shutdown
            if self.is_shutting_down() {
                let _ = sender.send(CandleCompletionChunk::Error(
                    "Pool shutting down".to_string()
                ));
                return;
            }

            // Get workers
            let workers = match self.workers.get(registry_key) {
                Some(w) => w,
                None => {
                    let _ = sender.send(CandleCompletionChunk::Error(
                        format!("No workers for {}", registry_key)
                    ));
                    return;
                }
            };

            // Find least busy worker
            let worker = match workers.iter()
                .min_by_key(|w| w.core.pending_requests.load(std::sync::atomic::Ordering::Acquire))
            {
                Some(w) => w,
                None => {
                    let _ = sender.send(CandleCompletionChunk::Error(
                        "No workers available".to_string()
                    ));
                    return;
                }
            };

            // Send request
            worker.core.pending_requests.fetch_add(1, std::sync::atomic::Ordering::Release);
            worker.core.touch();

            let (response_tx, response_rx) = channel::bounded(0);
            if let Err(e) = worker.prompt_tx.send(PromptRequest {
                prompt,
                params,
                response: response_tx,
            }) {
                let _ = sender.send(CandleCompletionChunk::Error(e.to_string()));
                return;
            }

            // Wait for stream with timeout
            let timeout = Duration::from_secs(self.config().request_timeout_secs);
            let stream = match response_rx.recv_timeout(timeout) {
                Ok(Ok(s)) => s,
                Ok(Err(e)) => {
                    let _ = sender.send(CandleCompletionChunk::Error(e.to_string()));
                    return;
                }
                Err(e) => {
                    let _ = sender.send(CandleCompletionChunk::Error(e.to_string()));
                    return;
                }
            };

            // Forward chunks from worker stream to caller
            while let Some(chunk) = stream.next().await {
                emit!(sender, chunk);
            }

            worker.core.pending_requests.fetch_sub(1, std::sync::atomic::Ordering::Release);
        })
    }
}
```

**Why**: Pool API for spawning workers and routing requests, returns AsyncStream (Pattern B from MODEL_POOL.md).

## SUBTASK 7: Wire Up Module Exports

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/mod.rs`

```rust
pub mod text_embedding;
pub mod text_to_text;  // NEW

pub use text_embedding::text_embedding_pool;
pub use text_to_text::text_to_text_pool;  // NEW
```

## DEFINITION OF DONE

- [ ] `text_to_text.rs` file created
- [ ] `PromptRequest` struct implemented
- [ ] `TextToTextWorkerHandle` struct implemented
- [ ] `text_to_text_worker()` loop implemented with crossbeam select!
- [ ] Global `TEXT_TO_TEXT_POOL` instance created
- [ ] `text_to_text_pool()` accessor function implemented
- [ ] `spawn_text_to_text_worker()` method implemented
- [ ] `prompt()` method implemented returning AsyncStream
- [ ] Module exports configured
- [ ] Code compiles with `cargo check`

## DEPENDENCIES

**Requires**: MPOOL_2A (Pool<T>), MPOOL_2B (worker helpers)

**Blocks**: MPOOL_5 (registry integration)

**Parallel with**: MPOOL_3A (TextEmbedding), MPOOL_3C (other capabilities)

## RESEARCH NOTES

**Streaming Pattern** (from MODEL_POOL.md Pattern B):
```rust
fn prompt(&self, prompt: CandlePrompt, params: &CandleCompletionParams)
    -> AsyncStream<CandleCompletionChunk>
{
    AsyncStream::with_channel(move |sender| {
        let (response_tx, response_rx) = oneshot::channel();
        let request = PromptRequest { prompt, params, response: response_tx };

        pool.send_to_worker(registry_key, request).ok();

        if let Ok(worker_stream) = response_rx.recv_timeout(Duration::from_secs(30)) {
            for chunk in worker_stream {
                emit!(sender, chunk);
            }
        }
    })
}
```

**Canonical Example** (kimi_k2.rs:83-96):
```rust
fn prompt(&self, prompt: CandlePrompt, params: &CandleCompletionParams)
    -> AsyncStream<CandleCompletionChunk>
{
    let gguf_file_path = match self.huggingface_file("*.gguf") {
        Ok(p) => p,
        Err(e) => {
            return AsyncStream::with_channel(move |sender| {
                let _ = sender.send(CandleCompletionChunk::Error(/*...*/));
            });
        }
    };
    // ... validation, then engine.coordinate_generation()
}
```

## CONSTRAINTS

- **NO TESTS**: Do not write any test code. Tests are handled by separate team.
- **NO BENCHMARKS**: Do not write any benchmark code. Benchmarks are handled by separate team.
- **GENERIC TRAIT**: Worker loop works for ANY TextToTextCapable implementation.
