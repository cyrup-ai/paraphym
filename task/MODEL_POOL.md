# Model Pool Implementation Plan

## SCOPE

This pool supports ALL capability traits in the system, not just text embedding. The architecture is fully generic across capability types.

### Supported Capability Traits

**1. TextToTextCapable**
- Method: `fn prompt(&self, prompt: CandlePrompt, params: &CandleCompletionParams) -> AsyncStream<CandleCompletionChunk>`
- Request: `PromptRequest { prompt: CandlePrompt, params: CandleCompletionParams, response: Sender<AsyncStream<CandleCompletionChunk>> }`
- Response: `AsyncStream<CandleCompletionChunk>` (streaming)
- Pattern: Caller awaits Future, receives stream, consumes chunks

**2. TextEmbeddingCapable**
- Methods:
  - `fn embed(&self, text: &str, task: Option<String>) -> Result<Vec<f32>, BoxError>`
  - `fn batch_embed(&self, texts: &[String], task: Option<String>) -> Result<Vec<Vec<f32>>, BoxError>`
- Requests:
  - `EmbedRequest { text: String, task: Option<String>, response: Sender<Result<Vec<f32>, PoolError>> }`
  - `BatchEmbedRequest { texts: Vec<String>, task: Option<String>, response: Sender<Result<Vec<Vec<f32>>, PoolError>> }`
- Response: `Result<Vec<f32>>` or `Result<Vec<Vec<f32>>>` (immediate)
- Pattern: Caller awaits Future, receives result directly

**3. ImageEmbeddingCapable**
- Methods:
  - `fn embed_image(&self, image_path: &str) -> Result<Vec<f32>, BoxError>`
  - `fn embed_image_url(&self, url: &str) -> Result<Vec<f32>, BoxError>`
  - `fn embed_image_base64(&self, base64_data: &str) -> Result<Vec<f32>, BoxError>`
  - `fn batch_embed_images(&self, image_paths: Vec<&str>) -> Result<Vec<Vec<f32>>, BoxError>`
- Requests:
  - `EmbedImageRequest { image_path: String, response: Sender<Result<Vec<f32>, PoolError>> }`
  - `EmbedImageUrlRequest { url: String, response: Sender<Result<Vec<f32>, PoolError>> }`
  - `EmbedImageBase64Request { base64_data: String, response: Sender<Result<Vec<f32>, PoolError>> }`
  - `BatchEmbedImagesRequest { image_paths: Vec<String>, response: Sender<Result<Vec<Vec<f32>>, PoolError>> }`
- Response: `Result<Vec<f32>>` or `Result<Vec<Vec<f32>>>` (immediate)
- Pattern: Caller awaits Future, receives result directly

**4. VisionCapable**
- Methods:
  - `fn describe_image(&self, image_path: &str, query: &str) -> AsyncStream<CandleStringChunk>`
  - `fn describe_url(&self, url: &str, query: &str) -> AsyncStream<CandleStringChunk>`
- Requests:
  - `DescribeImageRequest { image_path: String, query: String, response: Sender<AsyncStream<CandleStringChunk>> }`
  - `DescribeUrlRequest { url: String, query: String, response: Sender<AsyncStream<CandleStringChunk>> }`
- Response: `AsyncStream<CandleStringChunk>` (streaming)
- Pattern: Caller awaits Future, receives stream, consumes chunks

**5. TextToImageCapable**
- Method: `fn generate_image(&self, prompt: &str, config: &ImageGenerationConfig, device: &Device) -> AsyncStream<ImageGenerationChunk>`
- Request: `GenerateImageRequest { prompt: String, config: ImageGenerationConfig, device: Device, response: Sender<AsyncStream<ImageGenerationChunk>> }`
- Response: `AsyncStream<ImageGenerationChunk>` (streaming)
- Pattern: Caller awaits Future, receives stream, consumes chunks

### Request/Response Pattern

**Canonical Example**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/text_to_text/kimi_k2.rs:83-96`

```rust
// Sync trait method returns AsyncStream
fn prompt(&self, prompt: CandlePrompt, params: &CandleCompletionParams)
    -> AsyncStream<CandleCompletionChunk>
{
    // Return AsyncStream using with_channel pattern
    AsyncStream::with_channel(move |sender| {
        // Send request to pool worker
        // Worker processes in background thread
        // emit!(sender, chunk) for each chunk
    })
}
```

**Pattern Reference**: `/Volumes/samsung_t9/ystream/examples/with_channel_pattern.rs`

**For ALL capabilities:**
1. Caller invokes **synchronous** trait method (e.g., `model.embed()`, `model.prompt()`)
2. Trait impl calls pool, wrapping in `AsyncStream::with_channel()` or blocking on oneshot
3. Pool creates request with oneshot::channel for response
4. Request sent to worker via unbounded crossbeam channel
5. Worker thread picks up request from channel
6. Worker processes using loaded model (already in memory)
7. Worker sends response back via oneshot channel

**Two response patterns:**

**Pattern A: Immediate Results** (TextEmbedding, ImageEmbedding)
```rust
fn embed(&self, text: &str, task: Option<String>) -> Result<Vec<f32>, BoxError> {
    let (response_tx, response_rx) = oneshot::channel();
    let request = EmbedRequest { text: text.to_string(), task, response: response_tx };

    pool.send_to_worker(registry_key, request)?;

    // Block on response with timeout
    response_rx.recv_timeout(Duration::from_secs(30))?
}
```

**Pattern B: Streaming Results** (TextToText, Vision, TextToImage)
```rust
fn prompt(&self, prompt: CandlePrompt, params: &CandleCompletionParams)
    -> AsyncStream<CandleCompletionChunk>
{
    AsyncStream::with_channel(move |sender| {
        let (response_tx, response_rx) = oneshot::channel();
        let request = PromptRequest { prompt, params, response: response_tx };

        pool.send_to_worker(registry_key, request).ok();

        // Receive stream from worker, forward chunks to caller
        if let Ok(worker_stream) = response_rx.recv_timeout(Duration::from_secs(30)) {
            for chunk in worker_stream {
                emit!(sender, chunk);
            }
        }
    })
}
```

**Key insights**:
- All traits are **synchronous** (no async fn, no .await)
- Immediate results: Block on oneshot channel, return Result directly
- Streaming results: Use `AsyncStream::with_channel()`, forward chunks from worker
- Worker owns loaded model exclusively (no Arc<Mutex<>>)
- Pool routes requests to workers via unbounded channels
- Timeout on response receive provides backpressure visibility

## Problem Statement

Current text embedding models (GTE-Qwen, Jina-BERT, NvEmbed) reload from disk on EVERY inference call:
- Load tokenizer (I/O)
- Read config.json (I/O)
- Map safetensor files (I/O)
- Create Model instance with weights (GPU/CPU memory allocation)
- Run inference
- **Discard everything**

This is extremely slow. We need a pool to keep loaded models in memory and reuse them.

## Architecture Overview

### Core Components

1. **pool.rs** - Generic worker pool infrastructure
   - Generic over `TextEmbeddingCapable` trait
   - Request/Response structs for each operation
   - Worker lifecycle management
   - Load balancing by queue depth
   - LRU eviction of idle workers
   - Zero knowledge of specific models (GteQwen, JinaBert, NvEmbed)

2. **registry.rs** - Model-specific integration
   - Provides model loading closures to pool
   - Calls pool's generic methods
   - Handles TextEmbeddingModel enum dispatch

3. **Worker threads** - Generic over T: TextEmbeddingCapable
   - Load model once during spawn
   - Process requests in loop
   - Owned exclusively by thread (no locks)

## Detailed Design

### Request/Response Structs (pool.rs)

Each operation gets dedicated types:

```rust
pub struct EmbedRequest {
    pub text: String,
    pub task: Option<String>,
    pub response: Sender<Result<Vec<f32>, PoolError>>,
}

pub struct BatchEmbedRequest {
    pub texts: Vec<String>,
    pub task: Option<String>,
    pub response: Sender<Result<Vec<Vec<f32>>, PoolError>>,
}
```

### Worker Handle (pool.rs)

```rust
struct WorkerHandle {
    embed_tx: Sender<EmbedRequest>,
    batch_embed_tx: Sender<BatchEmbedRequest>,
    pending_requests: Arc<AtomicUsize>,
    last_used: Arc<AtomicU64>,
    worker_id: usize,
}
```

### Model Pool Structure (pool.rs)

```rust
pub struct ModelPool {
    // Generic storage: registry_key -> Vec<WorkerHandle>
    workers: DashMap<String, Vec<WorkerHandle>>,
    config: PoolConfig,
    next_worker_id: AtomicUsize,
    metrics: PoolMetrics,
}
```

### Generic Worker Function (pool.rs)

```rust
fn text_embedding_worker<T: TextEmbeddingCapable + Send + 'static>(
    model: T,
    embed_rx: Receiver<EmbedRequest>,
    batch_rx: Receiver<BatchEmbedRequest>,
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
            recv(batch_rx) -> req => {
                if let Ok(req) = req {
                    let result = model.batch_embed(&req.texts, req.task)
                        .map_err(|e| PoolError::ModelError(e.to_string()));
                    let _ = req.response.send(result);
                }
            }
        }
    }
}
```

### Pool Methods (pool.rs)

```rust
impl ModelPool {
    /// Spawn worker for text embedding model
    pub fn spawn_text_embedding_worker<T, F>(
        &self,
        registry_key: &str,
        model_loader: F,
    ) -> Result<(), PoolError>
    where
        T: TextEmbeddingCapable + Send + 'static,
        F: FnOnce() -> Result<T, PoolError> + Send + 'static,
    {
        // Create channels
        let (embed_tx, embed_rx) = channel();
        let (batch_tx, batch_rx) = channel();

        // Spawn worker thread
        std::thread::spawn(move || {
            let model = model_loader().expect("Model loading failed");
            text_embedding_worker(model, embed_rx, batch_rx);
        });

        // Create handle and register
        let handle = WorkerHandle {
            embed_tx,
            batch_embed_tx: batch_tx,
            pending_requests: Arc::new(AtomicUsize::new(0)),
            last_used: Arc::new(AtomicU64::new(now())),
            worker_id: self.next_worker_id.fetch_add(1, Ordering::Relaxed),
        };

        self.workers.entry(registry_key.to_string())
            .or_insert_with(Vec::new)
            .push(handle);

        Ok(())
    }

    /// Embed text using pooled worker
    pub fn embed_text(
        &self,
        registry_key: &str,
        text: &str,
        task: Option<String>,
    ) -> Result<Vec<f32>, PoolError> {
        // Get or create workers for this model
        let workers_ref = self.workers.get(registry_key)
            .ok_or_else(|| PoolError::NoWorkers(registry_key.to_string()))?;

        // Load balance: find worker with minimum queue depth
        let worker = workers_ref.iter()
            .min_by_key(|w| w.pending_requests.load(Ordering::Acquire))
            .ok_or(PoolError::NoWorkers(registry_key.to_string()))?;

        // Update metrics
        worker.pending_requests.fetch_add(1, Ordering::Release);
        worker.touch();

        // Send request
        let (response_tx, response_rx) = sync_channel(0);
        worker.embed_tx.send(EmbedRequest {
            text: text.to_string(),
            task,
            response: response_tx,
        })?;

        // Wait for response with timeout
        let result = response_rx.recv_timeout(Duration::from_millis(self.config.timeout_ms))?;

        // Decrement pending
        worker.pending_requests.fetch_sub(1, Ordering::Release);

        result
    }

    /// Batch embed texts using pooled worker
    pub fn batch_embed_text(
        &self,
        registry_key: &str,
        texts: &[String],
        task: Option<String>,
    ) -> Result<Vec<Vec<f32>>, PoolError> {
        // Similar to embed_text but uses batch channel
        // ...
    }
}
```

### Registry Integration (registry.rs)

For GteQwen match arm:

```rust
impl TextEmbeddingCapable for TextEmbeddingModel {
    fn embed(&self, text: &str, task: Option<String>) -> Result<Vec<f32>, Box<dyn Error>> {
        match self {
            Self::GteQwen(m) => {
                let registry_key = m.info().registry_key;
                let pool = pool::global_pool();

                // Ensure worker exists
                if !pool.has_workers(registry_key) {
                    let m_clone = m.clone();
                    pool.spawn_text_embedding_worker(registry_key, move || {
                        load_gte_qwen_model(&m_clone)
                    })?;
                }

                // Use pool
                pool.embed_text(registry_key, text, task)
                    .map_err(|e| Box::new(e) as Box<dyn Error>)
            }
            // ... other variants
        }
    }
}
```

Where `load_gte_qwen_model()` is a helper that extracts the loading logic from lines 178-249 of current gte_qwen.rs and returns a struct that implements TextEmbeddingCapable with the loaded state.

### Loaded Model Wrapper

Create a struct that holds the loaded state and implements TextEmbeddingCapable:

```rust
struct LoadedGteQwenModel {
    model: Model,
    tokenizer: Tokenizer,
    device: Device,
    config: Config,
}

impl TextEmbeddingCapable for LoadedGteQwenModel {
    fn embed(&self, text: &str, task: Option<String>) -> Result<Vec<f32>, Box<dyn Error>> {
        // Use self.model, self.tokenizer (already loaded!)
        // No I/O, just inference
        CandleGteQwenEmbeddingModel::forward_pass_with_task(
            &self.tokenizer,
            &mut self.model,
            &self.device,
            &[text],
            task.as_deref(),
        )?.into_iter().next().ok_or("No embeddings")
    }

    fn batch_embed(&self, texts: &[String], task: Option<String>) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
        // Similar - use loaded state
    }

    // ... other trait methods
}
```

## Key Architectural Decisions

1. **Pool is generic over TextEmbeddingCapable trait** - works for any embedding model
2. **Request/Response structs per operation** - type-safe, enables parameterization
3. **Dedicated channels per operation** - cleaner than enum variants
4. **Worker spawning via closure** - registry provides type-specific loading, pool manages lifecycle
5. **LoadedModel wrapper** - holds mutable state (Model, Tokenizer), implements trait
6. **DashMap<String, Vec<WorkerHandle>>** - one generic storage for all models
7. **Load balancing by queue depth** - minimize latency
8. **std::sync channels** - blocking, matches synchronous trait methods
9. **No Arc<Mutex<>>** - workers own models exclusively
10. **No model-specific code in pool.rs** - fully generic

## Implementation Phases

### Phase 1: Pool Infrastructure
- PoolError type
- PoolConfig struct
- EmbedRequest / BatchEmbedRequest structs
- WorkerHandle struct
- ModelPool struct
- global_pool() function

### Phase 2: Generic Worker
- text_embedding_worker() function
- spawn_text_embedding_worker() method
- embed_text() method
- batch_embed_text() method

### Phase 3: Loaded Model Wrappers
- Extract loading logic from gte_qwen.rs
- Create LoadedGteQwenModel struct
- Implement TextEmbeddingCapable for loaded wrapper
- Repeat for JinaBert and NvEmbed

### Phase 4: Registry Integration
- Update GteQwen match arm to use pool
- Update JinaBert match arm
- Update NvEmbed match arm

### Phase 5: Advanced Features
- Load balancing with spawn-on-demand
- LRU eviction
- Metrics
- Session support

## Success Criteria

1. Models load once and stay in memory
2. Multiple concurrent requests work correctly
3. Pool spawns additional workers when overloaded
4. Idle workers are evicted to free memory
5. No Arc<Mutex<>> patterns
6. No unwrap() or expect() in src/
7. Code compiles without warnings
8. Performance: 10-100x faster than disk loading per request

---

## Scenario Decisions

### Scenario 1: All Workers Busy - What Happens?

**Decision:** Dynamic worker limits based on available memory

**Behavior:**
1. Calculate `max_workers` per model dynamically:
   ```
   available_memory = query_system_memory()
   usable_memory = available_memory * 0.80  // 80% safety margin
   per_model_memory = calculate_from_model_info(registry_key)
   max_workers = usable_memory / per_model_memory
   ```

2. When request arrives and all workers busy:
   - If `current_workers < max_workers`: **Spawn new worker**
   - If `current_workers >= max_workers`: **Queue and wait** (requests block in channel)

3. Memory calculation from ModelInfo fields:
   - `vocab_size` - tokenizer vocabulary
   - `embedding_dimension` - model hidden size
   - Model weight files size (from safetensors)
   - dtype multiplier (F32=4 bytes, F16=2 bytes, BF16=2 bytes)
   - Overhead for activations, KV cache, etc.

**Implementation notes:**
- Pool tracks total memory used: `Arc<AtomicUsize>`
- Before spawning worker: check `total_used + model_footprint <= usable_memory`
- After spawning: `total_used += model_footprint`
- After eviction: `total_used -= model_footprint`
- No hard-coded limits - adapts to available resources

**Safety:**
- 80% limit prevents OOM
- Graceful degradation: queuing instead of crashing
- Conservative estimates: better to queue than OOM

**Maintenance Thread:**
- Background thread runs every N seconds (e.g., 10s)
- Recalculates `max_workers` based on current system memory
- If memory pressure increased (max_workers decreased):
  - Triggers aggressive eviction of idle workers
  - May need to evict least-recently-used active workers if critical
- If memory freed up (max_workers increased):
  - Allows new worker spawns on next request
- Updates pool-wide limits dynamically
- Coordinates across all model types in the pool
- Logs memory pressure events for observability

### Scenario 2: Lazy Activation - Only Load Used Models

**CRITICAL REQUIREMENT:** Registry will eventually have hundreds/thousands of model variants. We CANNOT load them all into memory.

**Decision:** Lazy activation on first request + eager expansion

**Behavior:**
1. **Cold start (first request for model X):**
   - No workers exist for this registry_key
   - Pool spawns first worker (slow - loads from disk)
   - Request waits for first worker to load and process
   - After first worker loads: **immediately evaluate spawning additional workers**
   - If `available_memory` allows: spawn workers 2, 3, ... up to calculated `max_workers`
   - This happens async - first request doesn't wait for all workers

2. **Warm state (subsequent requests):**
   - Workers already exist
   - Fast path - route to least-busy worker

3. **Unused models:**
   - If model Y registered but never invoked: **ZERO workers spawned**
   - No memory wasted on unused models
   - Registry is just metadata (CandleModelInfo)

**Implementation:**
- `embed_text()` checks: `if !self.workers.contains_key(registry_key)`
- If missing: call `activate_model(registry_key, model_loader)`
- `activate_model()`:
  1. Spawn first worker (blocking for this request)
  2. Spawn background task to spawn additional workers up to limit
  3. Return first worker handle

**Example:**
- Registry has 500 models registered
- User only uses: GteQwen, Stella, BERT
- Pool only activates those 3 models
- 497 other models stay as metadata only
- Memory footprint: 3 models worth, not 500

### Scenario 3: Request Dispatch When All Workers Busy

**Context**: Request arrives for model X. All workers busy (pending_requests > 0). Already at max_workers limit (can't spawn more).

**Decision**: Unbounded queue with configurable timeout

**Behavior:**
1. Worker channels are **unbounded** (crossbeam::channel::unbounded)
2. Request **blocks** in channel queue until worker available
3. **Timeout** on receive operation (default: 30 seconds, configurable)
4. If timeout expires: return `PoolError::Timeout` to caller
5. **No request rejection** - queue can grow as large as needed
6. Memory grows with queue depth - this is intentional design

**Configuration:**
- Timeout configured via agent builder pattern:
  ```rust
  agent_builder
      .model(text_model)
      .pool_timeout(Duration::from_secs(30))  // override default
  ```
- Alternative: agent_role defaults include timeout setting
- Stored in PoolConfig per-pool or per-request

**Rationale:**
- Unbounded queue prevents dropping legitimate requests
- Timeout provides user feedback when system overloaded
- Configurable timeout allows different use cases:
  - CLI: longer timeout (60s) - user is waiting
  - API: shorter timeout (10s) - fail fast for retry
  - Batch: very long (300s) - throughput over latency

**Future Enhancement (Phase 5):**
- Maintenance thread monitors queue depth across all models
- When depth > threshold (e.g., 50 requests):
  - Log warning: "Model X queue depth: 73 requests"
  - Optionally: Emit user-visible message via callback
  - Message example: "System under heavy load, request queued..."
- Helps users understand why requests are slow

**Implementation Notes:**
- Use `recv_timeout()` for blocking with timeout on response receiver
- Each worker has capability-specific request channels:
  - TextEmbedding worker: embed_rx, batch_embed_rx
  - TextToText worker: prompt_rx
  - ImageEmbedding worker: embed_image_rx, embed_image_url_rx, embed_image_base64_rx, batch_embed_images_rx
  - Vision worker: describe_image_rx, describe_url_rx
  - TextToImage worker: generate_image_rx
- All channels are **unbounded** (crossbeam::channel::unbounded)
- Load balancer sends to worker with min queue depth
- Queue depth = pending_requests counter (atomic, incremented on send, decremented on response)
