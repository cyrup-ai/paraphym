# Model Pool Implementation Plan

## CRITICAL DESIGN PRINCIPLE

**pool.rs CONTAINS ZERO MODEL-SPECIFIC LOGIC**

This cannot be emphasized enough:

- ❌ **NO** GteQwen-specific code in pool.rs
- ❌ **NO** JinaBert-specific code in pool.rs  
- ❌ **NO** NvEmbed-specific code in pool.rs
- ❌ **NO** Phi4-specific code in pool.rs
- ❌ **NO** KimiK2-specific code in pool.rs
- ❌ **NO** Qwen3Coder-specific code in pool.rs
- ❌ **NO** ClipVision-specific code in pool.rs
- ❌ **NO** LLaVA-specific code in pool.rs
- ❌ **NO** FLUX-specific code in pool.rs
- ❌ **NO** StableDiffusion-specific code in pool.rs
- ❌ **NO** knowledge of any specific model's existence

**pool.rs is 100% GENERIC over capability traits:**

- ✅ Generic over `T: TextEmbeddingCapable`
- ✅ Generic over `T: TextToTextCapable`
- ✅ Generic over `T: ImageEmbeddingCapable`
- ✅ Generic over `T: VisionCapable`
- ✅ Generic over `T: TextToImageCapable`

**ALL model-specific logic lives in:**
- `capability/{type}/{model_name}.rs` - Individual model implementations
- `capability/registry.rs` - Enum dispatch and model loading closures

**pool.rs only knows about:**
- Capability trait methods (embed, batch_embed, prompt, describe_image, generate_image)
- Request/Response structs (EmbedRequest, PromptRequest, etc.)
- Worker lifecycle (spawn, route, evict)
- Memory management (track usage, enforce limits)
- Channel communication (send requests, receive responses)

**Zero knowledge of:**
- Tokenizer specifics
- Model architectures
- Weight file formats (safetensors, GGUF, etc.)
- HuggingFace repos
- Specific model configurations

This is a **hard requirement**. Any PR that adds model-specific logic to pool.rs will be rejected.

## SCOPE

This pool supports ALL capability traits in the system, not just text embedding. The architecture is fully generic across capability types.

### Pool Architecture

**5 Global Pool Instances**: One pool per capability trait, each a global singleton.

**Multi-Capability Design**:
- Pool implementation is **generic** over capability trait: `Pool<T>`
- Written once in `pool/core/pool.rs`
- Instantiated 5 times as global singletons
- Each pool manages multiple models of that capability type

**Lifecycle**:
- **Initialization**: Lazy initialization on first use
- **Ownership**: Global statics, live for application lifetime
- **Shared**: All agents, conversations, and requests use same pool instances
- **Thread-safe**: DashMap and atomic counters for concurrent access

**Implementation Pattern**:
```rust
use once_cell::sync::Lazy;
use std::marker::PhantomData;

// Generic pool struct (written once)
pub struct Pool<T> {
    workers: DashMap<String, Vec<WorkerHandle>>,
    config: PoolConfig,
    next_worker_id: AtomicUsize,
    metrics: PoolMetrics,
    _phantom: PhantomData<T>,
}

// 5 global pool instances (one per capability trait)
static TEXT_EMBEDDING_POOL: Lazy<Pool<dyn TextEmbeddingCapable>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

static TEXT_TO_TEXT_POOL: Lazy<Pool<dyn TextToTextCapable>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

static IMAGE_EMBEDDING_POOL: Lazy<Pool<dyn ImageEmbeddingCapable>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

static VISION_POOL: Lazy<Pool<dyn VisionCapable>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

static TEXT_TO_IMAGE_POOL: Lazy<Pool<dyn TextToImageCapable>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

// Accessor functions
pub fn text_embedding_pool() -> &'static Pool<dyn TextEmbeddingCapable> {
    &TEXT_EMBEDDING_POOL
}

pub fn text_to_text_pool() -> &'static Pool<dyn TextToTextCapable> {
    &TEXT_TO_TEXT_POOL
}

// ... etc for other pools
```

**Per-Pool Storage**:
Each pool's DashMap stores workers for multiple models of that capability:
- `TEXT_EMBEDDING_POOL`: Contains workers for all embedding models (registry_key -> workers)
- `TEXT_TO_TEXT_POOL`: Contains workers for all text generation models (registry_key -> workers)
- `IMAGE_EMBEDDING_POOL`: Contains workers for all image embedding models (registry_key -> workers)
- `VISION_POOL`: Contains workers for all vision models (registry_key -> workers)
- `TEXT_TO_IMAGE_POOL`: Contains workers for all image generation models (registry_key -> workers)

**Example**: TEXT_EMBEDDING_POOL DashMap contents after use:
```
"dunzhang/stella_en_1.5B_v5" -> [WorkerHandle, WorkerHandle]  // 2 Stella workers
"Alibaba-NLP/gte-Qwen2-1.5B-instruct" -> [WorkerHandle, WorkerHandle, WorkerHandle]  // 3 GTE-Qwen workers
"nvidia/NV-Embed-v2" -> [WorkerHandle]  // 1 NvEmbed worker
```

**Rationale**:
- One generic implementation, multiple instantiations (DRY principle)
- Each pool coordinates memory for its capability type
- Simplifies integration (capability-specific accessors)
- Workers persist across agent/conversation boundaries
- Natural fit for long-running CLI/server processes

### Module Structure

**Submodule Organization**:
```
pool/
  mod.rs                      - Module exports
  core/
    mod.rs                    - Core module exports
    pool.rs                   - Generic Pool<T> implementation
    worker.rs                 - Generic worker spawn/loop functions  
    types.rs                  - WorkerHandle, PoolConfig, PoolMetrics
    error.rs                  - PoolError enum
  capabilities/
    mod.rs                    - Capability module exports
    text_embedding.rs         - TextEmbedding Request/Response + global pool
    text_to_text.rs           - TextToText Request/Response + global pool
    image_embedding.rs        - ImageEmbedding Request/Response + global pool
    vision.rs                 - Vision Request/Response + global pool
    text_to_image.rs          - TextToImage Request/Response + global pool
  maintenance.rs              - Maintenance thread (eviction, memory monitoring)
```

**File Responsibilities**:

**core/pool.rs**: Generic `Pool<T>` struct and methods (spawn_worker, send_request, etc.)

**core/worker.rs**: Generic worker loop functions per capability trait

**core/types.rs**: Shared types (WorkerHandle, PoolConfig, PoolMetrics)

**core/error.rs**: PoolError enum

**capabilities/text_embedding.rs**:
- `EmbedRequest` / `BatchEmbedRequest` structs
- `TEXT_EMBEDDING_POOL` global instance
- `text_embedding_pool()` accessor function
- `text_embedding_worker()` specialized worker loop

**capabilities/text_to_text.rs**:
- `PromptRequest` struct
- `TEXT_TO_TEXT_POOL` global instance
- `text_to_text_pool()` accessor function
- `text_to_text_worker()` specialized worker loop

**capabilities/{others}.rs**: Same pattern for ImageEmbedding, Vision, TextToImage

**maintenance.rs**: Background thread for eviction and memory monitoring across all pools

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
- **ONE shared channel per model** - all workers for that model pull from same channel
- Workers **self-schedule** by pulling from shared queue (first available worker gets next request)
- No dispatch logic needed - crossbeam channel handles worker selection automatically
- Each capability type has specific request channels:
  - TextEmbedding: embed_rx, batch_embed_rx
  - TextToText: prompt_rx
  - ImageEmbedding: embed_image_rx, embed_image_url_rx, embed_image_base64_rx, batch_embed_images_rx
  - Vision: describe_image_rx, describe_url_rx
  - TextToImage: generate_image_rx
- All channels are **unbounded** (crossbeam::channel::unbounded)

**Example**: GteQwen model with 3 workers
```
Client → [embed_request] → SharedQueue → Worker #1 (busy, not listening)
                                       → Worker #2 (calls recv(), gets request!)
                                       → Worker #3 (busy, not listening)
```
Workers self-schedule - no load balancing dispatch needed.

### Scenario 4: Memory Footprint Calculation

**Context**: Dynamic worker limits (Scenario 1) require knowing per-model memory usage to calculate `max_workers = (available_memory * 0.80) / per_model_memory`.

**Decision**: Add `est_memory_allocation_mb: usize` field to CandleModelInfo (required, non-optional)

**Implementation**:
1. Add field to CandleModelInfo struct in `domain/model/info.rs`
2. Download HuggingFace model cards for all 12 models
3. Convert to markdown, store in `docs/models/{capability}/{MODEL_NAME}.md`
4. Extract memory allocation specs from model cards
5. Populate `est_memory_allocation_mb` in each model's static MODEL_INFO (REQUIRED for all models)
6. Pool reads via `model.info().est_memory_allocation_mb`

**Models Requiring Documentation** (see task/MODEL_CARDS.md):
- TextEmbedding (5): Stella, BERT, GteQwen, JinaBert, NvEmbed
- TextToText (3): KimiK2, Qwen3Coder, Phi4Reasoning (✓ already done)
- ImageEmbedding (1): ClipVision
- Vision (1): LLaVA
- TextToImage (2): FluxSchnell, StableDiffusion35Turbo

**Memory Specs Source**: HuggingFace model cards (official specifications from model authors)

**Memory Accounting Formula**:
```rust
// Simple formula - use est_memory_allocation_mb directly
let per_worker_memory_mb = model.info().est_memory_allocation_mb;

// Memory check before spawning worker
let current_usage_mb = pool.total_memory_used.load(Ordering::Acquire);
let total_system_memory_mb = query_system_memory();
let memory_limit_mb = (total_system_memory_mb as f64 * 0.80) as usize;

if current_usage_mb + per_worker_memory_mb <= memory_limit_mb {
    // Safe to spawn worker
    spawn_worker();
    pool.total_memory_used.fetch_add(per_worker_memory_mb, Ordering::Release);
} else {
    // At memory limit, queue request instead
}
```

**No Overhead Multiplier**:
- Estimates are already padded/rounded up during documentation phase (task/MODEL_CARDS.md)
- Pool uses `est_memory_allocation_mb` as-is without additional multipliers
- Conservative estimates built into documented values

**Rationale**:
- Required field = compile-time guarantee all models have memory specs
- Static field = zero runtime overhead
- Manual calculation = most accurate (from official model cards)
- Documented in markdown = auditable, maintainable
- No fallbacks or defaults = explicit memory management
- Padding in estimates = no runtime overhead multiplication needed

### Scenario 5: Worker Lifecycle - Warmup and Cooldown

**Context**: Workers consume memory. Need policies for scaling up (warmup) and scaling down (cooldown) based on load.

**Decision**: Gradual scale-up and scale-down with 1-minute intervals

**Warmup (Scale-Up) Policy**:

1. **Lazy Activation** (app starts with 0 workers for ALL models):
   - No workers spawned at startup
   - Registry has 500+ models registered as metadata only
   - Zero memory usage until first request

2. **Cold Start** (first request for Model X arrives, Model X has 0 workers):
   - **Spawn 2 workers for Model X specifically** (asymmetric with cooldown)
   - Memory check: `current_memory_usage + (2 * model_X_memory) <= 0.80 * total_memory`
   - If memory sufficient:
     - Spawn worker #1 to handle request (blocking for this request)
     - Spawn worker #2 as "warm" worker (background)
     - Result: Model X now has 2 workers
   - If memory insufficient for 2 workers:
     - Spawn only worker #1, skip warm worker
     - Result: Model X has 1 worker (degraded cold start)
   - **Other models still have 0 workers**

2. **Warm Expansion** (all workers busy, new request arrives):
   - **Gradual spawn: +1 worker** (symmetric with cooldown)
   - Memory check: `current_memory_usage + model_memory <= 0.80 * total_memory`
   - If memory sufficient:
     - Spawn +1 worker in background
     - Request routes to new worker when ready
   - If memory insufficient:
     - Request queues in unbounded channel
     - No new worker spawned

**Cooldown (Scale-Down) Policy**:

1. **Maintenance Thread** checks every 1 minute
2. For each model's worker pool:
   - If **all workers** idle for 1+ minute: Evict LRU worker
   - If **any worker** received request: Reset cooldown timer
3. Evict 1 worker per model per minute until reaching 0
4. "Idle" = `pending_requests.load() == 0` for worker
5. LRU = worker with oldest `last_used` timestamp (AtomicU64)

**Example Timeline** (for one specific model, e.g., GteQwen):
```
t=0:00  App starts. GteQwen: 0 workers. All 500 models: 0 workers.
t=0:05  FIRST GteQwen request arrives → lazy activation triggers
        Spawn worker #1 (blocking) + worker #2 (warm) = 2 workers for GteQwen only
        All other models still: 0 workers
t=0:10  Both GteQwen workers busy, new request → spawn worker #3 = 3 workers
t=0:15  All 3 GteQwen workers busy, new request → spawn worker #4 = 4 workers
t=5:00  Last GteQwen request completes, all 4 workers idle
t=6:00  All idle 1 min → evict 1 worker (3 remain)
t=7:00  All idle 1 min → evict 1 worker (2 remain)
t=7:30  NEW GTEQWEN REQUEST → cooldown resets, 2 workers stay
t=8:30  Request done, 2 workers idle
t=9:30  All idle 1 min → evict 1 worker (1 remains)
t=10:30 All idle 1 min → evict last worker (0 remain, back to cold state)

Note: If Stella model is never used, it stays at 0 workers forever.
```

**Key Insights**:
- **Asymmetry at 0 only**: Cold start spawns 2 workers immediately (0→2)
- **Symmetric after cold start**: +1 worker per busy request, -1 worker per idle minute (2→3→4 / 4→3→2)
- **Memory bounded**: All scaling checks `current + new <= 0.80 * total`
- **80% memory limit**: 20% headroom for OS, buffers, safety margin
- **Lazy activation**: Only spawn workers for models actually used
- **Gradual cooldown**: 1 worker per minute prevents thrashing
- **Complete unload**: Scales back to 0 for unused models (frees all memory)

**Rationale**:
- Warm worker reduces latency for subsequent requests (no cold start)
- Gradual cooldown prevents premature eviction (1 min = reasonable idle time)
- Scales to 0 frees memory completely for unused models
- LRU eviction keeps most active workers alive

---

## Scenario 6: Error Handling

**Context**: Various failures can occur during model loading and inference. Need clear error handling strategy for each type.

### 6.1: Network Errors (During Model Download)

**Scenario**: Worker attempts to load model via `huggingface_file("model.safetensors")` but HuggingFace API is unreachable or times out.

**Decision**: Retry logic in `huggingface_file()`, pool waits on Future

**Behavior**:
- Download retry/timeout handled deep in `huggingface_file(registry_key)` implementation
- Worker thread blocks on Future while download attempts continue
- Pool request timeout (default 30s, configurable) applies to entire operation
- If timeout expires: Return error to end user with clear message

**Error Message Format**:
```
"Timed out acquiring pool worker for model 'microsoft/phi-4' after 30s. 
Cause: Network connectivity issues downloading model files from HuggingFace."
```

**Rationale**:
- Network errors are transient and retriable
- Pool layer doesn't need network-specific logic (abstracted by Future)
- Clear error messages help users diagnose connectivity issues
- Timeout prevents indefinite hangs

### 6.2: Out of Memory (OOM) Errors

**Scenario**: System runs out of memory despite 80% memory limit checks.

**Decision**: Should NEVER happen (prevention-based design), fatal if it does

**Behavior**:
- **Prevention**: All worker spawns check `current + new <= 0.80 * total_memory`
- **20% headroom**: Accounts for OS overhead, buffers, measurement errors
- **If OOM occurs anyway**: Fatal error, app crashes with diagnostic message

**Fatal Error Message**:
```
"FATAL: Out of memory error occurred despite pool memory checks. 
This indicates a critical bug in memory accounting. 
Current tracked usage: X MB, System total: Y MB, Limit: Z MB (80%)
Please report this bug with system details."
```

**Rationale**:
- OOM should be impossible if memory accounting is correct
- If it happens, it's a critical bug that needs immediate attention
- Crashing is safer than continuing with corrupted state
- Cannot gracefully recover from OOM (models may be partially loaded)
- Fatal crash with diagnostic info helps identify accounting bugs

### 6.3: Corrupted Safetensor Files

**Scenario**: Downloaded model files are corrupted (checksum mismatch, truncated, invalid format).

**Decision**: Automatic corruption detection and cleanup in `huggingface_file()`

**Behavior**:
- **Checksum validation**: `huggingface_file()` verifies SHA256 checksums from HuggingFace manifest
- **Auto-cleanup on corruption**: Delete corrupted files, re-download clean copies
- **Pool perspective**: Worker waits on Future like network errors
- **Timeout applies**: Default 30s includes validation and re-download time
- **If cleanup fails**: Return clear error message

**Error Message Format** (if re-download fails):
```
"Failed to load model 'nvidia/NV-Embed-v2' after detecting file corruption. 
Attempted auto-cleanup and re-download but operation failed.
Please check disk space and file permissions in cache directory."
```

**Rationale**:
- End users should never manually fix corrupted files
- Automatic recovery provides best UX
- Checksums detect corruption reliably
- Re-download is safest fix (ensures clean state)
- Pool stays generic (corruption handling in model loading layer)

### 6.4: Missing Required Files

**Scenario**: Required model files are missing from cache directory (tokenizer.json, config.json, safetensor files).

**Decision**: Treat as download error - retry logic in `huggingface_file()`, pool waits on Future

**Behavior**:
- `huggingface_file("tokenizer.json")` checks cache, downloads if missing
- `huggingface_file("config.json")` checks cache, downloads if missing
- Missing files trigger automatic download from HuggingFace
- Worker thread waits on Future during download
- Pool request timeout (default 30s) applies to entire operation
- If timeout expires: Return error to end user

**Error Message Format**:
```
"Timed out acquiring pool worker for model 'Alibaba-NLP/gte-Qwen2-1.5B-instruct' after 30s.
Cause: Missing required files triggered automatic download which did not complete in time."
```

**Rationale**:
- Missing files are recoverable via automatic download
- Same handling as network errors (abstracted by Future)
- Pool layer doesn't distinguish missing vs corrupted vs network errors
- All handled transparently in model loading layer

### 6.5: Disk I/O Errors

**Scenario**: Disk I/O errors during model loading (read errors, permission denied, disk full, filesystem corruption).

**Decision**: Fatal panic with diagnostic message, app crashes

**Fatal Error Message**:
```
"FATAL: Disk I/O error during model loading.
Model: nvidia/NV-Embed-v2
Operation: Memory-mapping safetensor files
Error: Permission denied: /Users/user/.cache/huggingface/hub/model.safetensors
This indicates a system-level issue with disk access or permissions.
Please check disk health, available space, and file permissions."
```

**Rationale**:
- Disk I/O errors indicate serious system problems (hardware failure, permissions, full disk)
- Cannot recover gracefully from filesystem corruption
- Crashing is safer than attempting to continue with partial/corrupted state
- Diagnostic message helps users identify system-level issues
- These are rare errors that require human intervention

### 6.6: Inference Errors

**Scenario**: Errors during model inference execution (tensor shape mismatch, NaN values, dimension errors, worker panic).

**Decision**: Fatal panic with diagnostic message, app crashes

**Fatal Error Message**:
```
"FATAL: Inference error in model 'microsoft/phi-4'.
This indicates a critical bug in model configuration or untested model implementation.
Error: Tensor shape mismatch - expected [1, 512, 4096], got [1, 512, 2048]
ModelInfo configuration may be incorrect or model was never tested with examples.
Please report this bug with full error details."
```

**Rationale**:
- Inference errors indicate fundamental bugs in ModelInfo configuration
- All models MUST be tested with examples before deployment
- If inference fails, it means developers never validated the model
- Crashing prevents silent data corruption or incorrect results
- Fatal crash forces immediate bug fix rather than masking issues
- Cannot trust model state after inference failure (may be corrupted)

---

## Scenario 7: Graceful Shutdown

**Context**: Application exits (Ctrl+C, normal shutdown). Need to handle in-flight requests and queued work.

**Decision**: Timeout-based drain (wait up to N seconds, then force exit)

**Behavior**:

1. **Shutdown Signal Received** (SIGINT, SIGTERM):
   - Set global shutdown flag (AtomicBool)
   - Stop accepting new requests to all 5 pools
   - Start drain timer (default: 5 seconds, configurable)

2. **Drain Period** (0 to N seconds):
   - **In-flight requests**: Workers finish processing current requests
   - **Queued requests**: Workers continue pulling from channels and processing
   - **New requests**: Immediately return `PoolError::ShuttingDown`
   - **Maintenance thread**: Stops spawning new workers, continues monitoring

3. **Timeout Reached** (after N seconds):
   - **Force exit**: Drop all remaining queued requests
   - **Worker threads**: Send shutdown signal via dedicated channel
   - **In-flight requests**: Workers interrupted, may return partial results or errors
   - **Main thread**: Exits with status code 0

4. **Clean Exit Before Timeout**:
   - If all queues empty and all workers idle before timeout: exit immediately
   - Log: "Graceful shutdown complete (X.Xs, Y requests drained)"

**Configuration**:
```rust
// In PoolConfig
pub struct PoolConfig {
    pub shutdown_timeout_secs: u64,  // Default: 5
    // ... other fields
}
```

**Error Response During Shutdown**:
```rust
PoolError::ShuttingDown {
    message: "Pool is shutting down, cannot accept new requests"
}
```

**Shutdown Hook Integration**:
```rust
// In main() or runner
use tokio::signal;

tokio::spawn(async {
    signal::ctrl_c().await.ok();
    eprintln!("Shutdown signal received, draining pools...");
    pool::begin_shutdown();
});
```

**Rationale**:
- Timeout prevents indefinite hangs on shutdown
- Draining in-flight work provides better UX (complete visible operations)
- Dropping queued work after timeout is acceptable (user already signaled exit)
- Configurable timeout allows different use cases (CLI vs server)
- Clean logging helps users understand what was completed vs dropped

---

## Integration Points

### Registry Integration Point

**Where pool integration happens**: `capability/registry.rs`

When users access models via:
- `registry::get<TextEmbeddingModel>("registry_key")` 
- Type-specific accessors from registry

The **registry enum dispatch** is where pool integration slides in:

```rust
// registry.rs - TextEmbeddingModel enum
impl TextEmbeddingCapable for TextEmbeddingModel {
    fn embed(&self, text: &str, task: Option<String>) -> Result<Vec<f32>, BoxError> {
        match self {
            Self::GteQwen(m) => {
                let registry_key = m.info().registry_key;
                
                // POOL INTEGRATION POINT - call pool instead of model directly
                let pool = pool::text_embedding_pool();
                
                // Lazy worker spawn if needed
                if !pool.has_workers(registry_key) {
                    let m_clone = m.clone();
                    pool.spawn_worker(registry_key, move || {
                        // Load model once for worker
                        create_loaded_model(&m_clone)
                    })?;
                }
                
                // Route through pool
                pool.embed_text(registry_key, text, task)
            }
            Self::JinaBert(m) => { /* same pattern */ }
            Self::NvEmbed(m) => { /* same pattern */ }
            Self::Stella(m) => { /* same pattern */ }
            Self::Bert(m) => { /* same pattern */ }
        }
    }
}
```

**User's perspective**: Nothing changes
```rust
// User code (unchanged)
let model = registry::get<TextEmbeddingModel>("dunzhang/stella_en_1.5B_v5")?;
let embedding = model.embed("hello world", None)?;  // Transparently uses pool
```

**Key insight**: Pool integration is **invisible** to users. Registry enum dispatch intercepts trait method calls and routes through pool.

### Model Modification Requirements

**Current broken pattern** (TextEmbedding models):

```rust
// gte_qwen.rs, jina_bert.rs, nvembed.rs, stella.rs, bert.rs
impl TextEmbeddingCapable for CandleGteQwenEmbeddingModel {
    fn embed(&self, text: &str, task: Option<String>) -> Result<Vec<f32>, BoxError> {
        // Load EVERYTHING from disk
        let tokenizer = Tokenizer::from_file(&tokenizer_path)?;  // I/O
        let config = serde_json::from_str(&config_str)?;         // I/O
        let vb = VarBuilder::from_mmaped_safetensors(...)?;      // I/O
        let model = Model::new(&config, vb)?;                    // GPU memory
        
        // Do inference ONCE
        let embeddings = forward_pass(&tokenizer, &model, ...)?;
        
        // DISCARD EVERYTHING (goes out of scope)
        Ok(embeddings)
    }
}
```

**New required pattern** (LoadedModel wrapper):

```rust
// NEW: gte_qwen.rs adds LoadedGteQwenModel struct
struct LoadedGteQwenModel {
    tokenizer: Tokenizer,      // STAYS IN MEMORY
    model: Model,              // STAYS IN MEMORY
    device: Device,            // STAYS IN MEMORY
    config: Config,            // STAYS IN MEMORY
}

impl TextEmbeddingCapable for LoadedGteQwenModel {
    fn embed(&self, text: &str, task: Option<String>) -> Result<Vec<f32>, BoxError> {
        // NO I/O - everything already loaded in self
        let embeddings = forward_pass_with_task(
            &self.tokenizer,  // Use loaded tokenizer
            &mut self.model,  // Use loaded model
            &self.device,
            &[text],
            task.as_deref(),
        )?;
        
        Ok(embeddings.into_iter().next().unwrap())
    }
}

// NEW: Factory function to create loaded model
fn create_loaded_gte_qwen_model(base_model: &CandleGteQwenEmbeddingModel) 
    -> Result<LoadedGteQwenModel, BoxError> 
{
    // Extract loading logic from current embed() method (lines 178-249)
    let tokenizer = Tokenizer::from_file(&tokenizer_path)?;
    let config = serde_json::from_str(&config_str)?;
    let vb = VarBuilder::from_mmaped_safetensors(...)?;
    let model = Model::new(&config, vb)?;
    let device = detect_best_device()?;
    
    Ok(LoadedGteQwenModel {
        tokenizer,
        model,
        device,
        config,
    })
}
```

**Worker uses LoadedModel**:

```rust
// Worker thread (lives forever, processes many requests)
fn text_embedding_worker<T: TextEmbeddingCapable>(
    model: T,  // This is LoadedGteQwenModel, owns all state
    embed_rx: Receiver<EmbedRequest>,
) {
    loop {
        if let Ok(req) = embed_rx.recv() {
            // Model already loaded, just do inference
            let result = model.embed(&req.text, req.task);
            let _ = req.response.send(result);
        }
    }
}
```

**Models requiring modification**:
1. `capability/text_embedding/gte_qwen.rs` - Extract loading logic, create LoadedGteQwenModel
2. `capability/text_embedding/jina_bert.rs` - Extract loading logic, create LoadedJinaBertModel
3. `capability/text_embedding/nvembed.rs` - Extract loading logic, create LoadedNvEmbedModel
4. `capability/text_embedding/stella.rs` - Extract loading logic, create LoadedStellaModel
5. `capability/text_embedding/bert.rs` - Extract loading logic, create LoadedBertModel

**Models NOT requiring modification**:
- `text_to_text/kimi_k2.rs` - Already stores state (model_path, engine, etc.)
- `text_to_text/qwen3_coder.rs` - Already stores state
- `text_to_text/phi4_reasoning.rs` - Already stores state
- All other capabilities - Don't have the "load-per-call" problem

**Critical requirement**: LoadedModel must implement `TextEmbeddingCapable` trait so workers can call trait methods generically.
