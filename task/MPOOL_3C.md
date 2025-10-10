# MPOOL_3C: Implement ImageEmbedding, Vision, TextToImage Capability Modules

**PREFIX**: MPOOL (Model Pool)

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

## OBJECTIVE

Implement remaining 3 capability modules: `image_embedding.rs`, `vision.rs`, and `text_to_image.rs`. These follow same patterns as text_embedding and text_to_text but are combined into one task as they have fewer models using them.

## EXISTING CODE RESEARCH

### Trait Signatures (from capability/traits.rs)

**ImageEmbeddingCapable** (4 methods, all synchronous):
```rust
pub trait ImageEmbeddingCapable: CandleModel {
    fn embed_image(&self, image_path: &str) -> Result<Vec<f32>, BoxError>;
    fn embed_image_url(&self, url: &str) -> Result<Vec<f32>, BoxError>;
    fn embed_image_base64(&self, base64_data: &str) -> Result<Vec<f32>, BoxError>;
    fn batch_embed_images(&self, image_paths: Vec<&str>) -> Result<Vec<Vec<f32>>, BoxError>;
    fn embedding_dimension(&self) -> usize;
}
```

**VisionCapable** (2 methods, both streaming):
```rust
pub trait VisionCapable: CandleModel {
    fn describe_image(&self, image_path: &str, query: &str) -> AsyncStream<CandleStringChunk>;
    fn describe_url(&self, url: &str, query: &str) -> AsyncStream<CandleStringChunk>;
}
```

**TextToImageCapable** (1 method, streaming):
```rust
pub trait TextToImageCapable: CandleModel {
    fn generate_image(&self, prompt: &str, config: &ImageGenerationConfig, device: &Device) -> AsyncStream<ImageGenerationChunk>;
}
```

### Existing Model Implementations

**ImageEmbedding**:
- `ClipVisionEmbeddingModel` in `capability/image_embedding/clip_vision_embedding.rs` (lines 220-257)
- Implements ImageEmbeddingCapable with all 4 methods
- Supports 512D (ViT-Base) and 768D (ViT-Large) embeddings

**Vision**:
- `LLaVAModel` in `capability/vision/llava.rs` (lines 1-720)
- Implements VisionCapable with 2 streaming methods
- Returns AsyncStream<CandleStringChunk> using AsyncStream::with_channel pattern
- Uses dedicated thread for non-Send model, channels for communication

**TextToImage**:
- `FluxSchnell` in `capability/text_to_image/flux_schnell.rs` (lines 1-559)
- Implements TextToImageCapable with generate_image method
- Returns AsyncStream<ImageGenerationChunk> using AsyncStream::with_channel pattern
- 4-step diffusion with dual text encoding (T5-XXL + CLIP-L)

### Pattern References

**Pattern A** (Immediate Results) - Reference: `pool/capabilities/text_embedding.rs`
- Synchronous request/response using `bounded(0)` channels
- Worker calls trait method, sends Result back via response channel
- Caller uses `recv_timeout` to wait for result
- Example:
  ```rust
  let (response_tx, response_rx) = bounded(0);
  worker.embed_tx.send(EmbedRequest { text, task, response: response_tx })?;
  let result = response_rx.recv_timeout(timeout)?;
  ```

**Pattern B** (Streaming Results) - Reference: `pool/capabilities/text_to_text.rs`
- Worker trait method returns AsyncStream directly
- Worker sends stream back via response channel
- Caller wraps in spawn_stream, uses blocking `for chunk in worker_stream` iteration
- Example (text_to_text.rs lines 247-252):
  ```rust
  // Forward chunks from worker stream to caller as they arrive
  for chunk in worker_stream {
      ystream::emit!(sender, chunk);
  }
  ```

## CONTEXT

- **ImageEmbedding**: 4 methods - all immediate results (Pattern A)
- **Vision**: 2 methods - both streaming results (Pattern B)  
- **TextToImage**: 1 method - streaming results (Pattern B)

## SUBTASK 1: Implement ImageEmbedding Module

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/image_embedding.rs`

### Architecture

- **4 Request Types**: EmbedImageRequest, EmbedImageUrlRequest, EmbedImageBase64Request, BatchEmbedImagesRequest
- **4 Channels**: embed_image_tx, embed_image_url_tx, embed_image_base64_tx, batch_embed_images_tx
- **Pattern A**: Synchronous request/response using bounded(0) channels
- **Global Storage**: `DashMap<String, Vec<ImageEmbeddingWorkerHandle>>`
- **Global Pool**: Lazy<Pool<dyn ImageEmbeddingCapable>>

### Implementation Structure

```rust
use crossbeam::channel::{Sender, Receiver, bounded, unbounded};
use crossbeam::select;
use once_cell::sync::Lazy;
use dashmap::DashMap;
use std::time::Duration;
use std::sync::atomic::Ordering;

use crate::pool::core::{Pool, PoolConfig, PoolError, WorkerHandle};
use crate::capability::traits::ImageEmbeddingCapable;

// 4 Request structs (one per method)
pub struct EmbedImageRequest {
    pub image_path: String,
    pub response: Sender<Result<Vec<f32>, PoolError>>,
}

pub struct EmbedImageUrlRequest {
    pub url: String,
    pub response: Sender<Result<Vec<f32>, PoolError>>,
}

pub struct EmbedImageBase64Request {
    pub base64_data: String,
    pub response: Sender<Result<Vec<f32>, PoolError>>,
}

pub struct BatchEmbedImagesRequest {
    pub image_paths: Vec<String>,
    pub response: Sender<Result<Vec<Vec<f32>>, PoolError>>,
}

// Worker handle with 4 channels + shutdown
pub struct ImageEmbeddingWorkerHandle {
    pub core: WorkerHandle,
    pub embed_image_tx: Sender<EmbedImageRequest>,
    pub embed_image_url_tx: Sender<EmbedImageUrlRequest>,
    pub embed_image_base64_tx: Sender<EmbedImageBase64Request>,
    pub batch_embed_images_tx: Sender<BatchEmbedImagesRequest>,
    pub shutdown_tx: Sender<()>,
}

// Worker loop with 4 channels + shutdown (Pattern A)
pub fn image_embedding_worker<T: ImageEmbeddingCapable>(
    model: T,
    embed_image_rx: Receiver<EmbedImageRequest>,
    embed_image_url_rx: Receiver<EmbedImageUrlRequest>,
    embed_image_base64_rx: Receiver<EmbedImageBase64Request>,
    batch_embed_images_rx: Receiver<BatchEmbedImagesRequest>,
    shutdown_rx: Receiver<()>,
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
            recv(shutdown_rx) -> _ => {
                log::info!("ImageEmbedding worker shutting down");
                break;
            }
        }
    }
}

// Global storage and pool (copy from text_embedding.rs pattern)
static IMAGE_EMBEDDING_WORKERS: Lazy<DashMap<String, Vec<ImageEmbeddingWorkerHandle>>> = Lazy::new(DashMap::new);

static IMAGE_EMBEDDING_POOL: Lazy<Pool<dyn ImageEmbeddingCapable>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

pub fn image_embedding_pool() -> &'static Pool<dyn ImageEmbeddingCapable> {
    &IMAGE_EMBEDDING_POOL
}

impl Pool<dyn ImageEmbeddingCapable> {
    // spawn_image_embedding_worker<T, F>(...) - copy text_embedding spawn pattern, create 4 channels
    // embed_image(...) - copy embed_text pattern, use embed_image_tx channel
    // embed_image_url(...) - copy embed_text pattern, use embed_image_url_tx channel  
    // embed_image_base64(...) - copy embed_text pattern, use embed_image_base64_tx channel
    // batch_embed_images(...) - copy batch_embed_text pattern, use batch_embed_images_tx channel
}

// Helper: query_system_memory_mb() - copy from text_embedding.rs
```

**Key Implementation Details**:
1. Memory check before spawning (80% system memory limit)
2. Use `bounded(0)` for synchronous request/response
3. Find least-busy worker using `pending_requests.load(Ordering::Acquire)`
4. Use `recv_timeout` with config.request_timeout_secs
5. Track pending requests: fetch_add before send, fetch_sub after receive
6. Touch worker (update last_used timestamp)

## SUBTASK 2: Implement Vision Module

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/vision.rs`

### Architecture

- **2 Request Types**: DescribeImageRequest, DescribeUrlRequest
- **2 Channels**: describe_image_tx, describe_url_tx
- **Pattern B**: Streaming results using AsyncStream<CandleStringChunk>
- **Global Storage**: `DashMap<String, Vec<VisionWorkerHandle>>`
- **Global Pool**: Lazy<Pool<dyn VisionCapable>>

### Implementation Structure

```rust
use crossbeam::channel::{unbounded, Receiver, Sender};
use crossbeam::select;
use once_cell::sync::Lazy;
use dashmap::DashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::time::Duration;
use ystream::{AsyncStream, spawn_stream};

use crate::pool::core::{Pool, PoolConfig, PoolError, WorkerHandle};
use crate::capability::traits::VisionCapable;
use crate::domain::context::CandleStringChunk;

// 2 Request structs (both streaming)
pub struct DescribeImageRequest {
    pub image_path: String,
    pub query: String,
    pub response: Sender<Result<AsyncStream<CandleStringChunk>, PoolError>>,
}

pub struct DescribeUrlRequest {
    pub url: String,
    pub query: String,
    pub response: Sender<Result<AsyncStream<CandleStringChunk>, PoolError>>,
}

// Worker handle with 2 channels + shutdown
pub struct VisionWorkerHandle {
    pub core: WorkerHandle,
    pub describe_image_tx: Sender<DescribeImageRequest>,
    pub describe_url_tx: Sender<DescribeUrlRequest>,
    pub shutdown_tx: Sender<()>,
}

// Worker loop with 2 channels + shutdown (Pattern B)
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

// Global storage and pool (copy from text_to_text.rs pattern)
static VISION_WORKERS: Lazy<DashMap<String, Vec<VisionWorkerHandle>>> = Lazy::new(DashMap::new);

static VISION_POOL: Lazy<Pool<dyn VisionCapable>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

pub fn vision_pool() -> &'static Pool<dyn VisionCapable> {
    &VISION_POOL
}

impl Pool<dyn VisionCapable> {
    // spawn_vision_worker<T, F>(...) - copy text_to_text spawn pattern, create 2 channels
    // describe_image(...) -> AsyncStream<CandleStringChunk> - copy text_to_text prompt pattern
    // describe_url(...) -> AsyncStream<CandleStringChunk> - copy text_to_text prompt pattern
}

// Helper: query_system_memory_mb() - copy from text_embedding.rs
```

**Key Implementation Details**:
1. Use `spawn_stream` to wrap worker stream forwarding
2. Worker returns AsyncStream directly from trait method
3. Use blocking `for chunk in worker_stream` iteration (NOT runtime.spawn)
4. Forward chunks using `ystream::emit!(sender, chunk)`
5. Memory check and least-busy selection same as ImageEmbedding
6. Use `recv_timeout` to wait for worker's AsyncStream response

## SUBTASK 3: Implement TextToImage Module

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/text_to_image.rs`

### Architecture

- **1 Request Type**: GenerateImageRequest
- **1 Channel**: generate_image_tx
- **Pattern B**: Streaming results using AsyncStream<ImageGenerationChunk>
- **Global Storage**: `DashMap<String, Vec<TextToImageWorkerHandle>>`
- **Global Pool**: Lazy<Pool<dyn TextToImageCapable>>

### Implementation Structure

```rust
use crossbeam::channel::{unbounded, Receiver, Sender};
use crossbeam::select;
use once_cell::sync::Lazy;
use dashmap::DashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::time::Duration;
use ystream::{AsyncStream, spawn_stream};
use candle_core::Device;

use crate::pool::core::{Pool, PoolConfig, PoolError, WorkerHandle};
use crate::capability::traits::TextToImageCapable;
use crate::domain::image_generation::{ImageGenerationChunk, ImageGenerationConfig};

// 1 Request struct (streaming)
pub struct GenerateImageRequest {
    pub prompt: String,
    pub config: ImageGenerationConfig,
    pub device: Device,
    pub response: Sender<Result<AsyncStream<ImageGenerationChunk>, PoolError>>,
}

// Worker handle with 1 channel + shutdown
pub struct TextToImageWorkerHandle {
    pub core: WorkerHandle,
    pub generate_image_tx: Sender<GenerateImageRequest>,
    pub shutdown_tx: Sender<()>,
}

// Worker loop with 1 channel + shutdown (Pattern B)
pub fn text_to_image_worker<T: TextToImageCapable>(
    model: T,
    generate_image_rx: Receiver<GenerateImageRequest>,
    shutdown_rx: Receiver<()>,
) {
    loop {
        select! {
            recv(generate_image_rx) -> req => {
                if let Ok(req) = req {
                    let stream = model.generate_image(&req.prompt, &req.config, &req.device);
                    let _ = req.response.send(Ok(stream));
                }
            }
            recv(shutdown_rx) -> _ => {
                log::info!("TextToImage worker shutting down");
                break;
            }
        }
    }
}

// Global storage and pool (copy from text_to_text.rs pattern)
static TEXT_TO_IMAGE_WORKERS: Lazy<DashMap<String, Vec<TextToImageWorkerHandle>>> = Lazy::new(DashMap::new);

static TEXT_TO_IMAGE_POOL: Lazy<Pool<dyn TextToImageCapable>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

pub fn text_to_image_pool() -> &'static Pool<dyn TextToImageCapable> {
    &TEXT_TO_IMAGE_POOL
}

impl Pool<dyn TextToImageCapable> {
    // spawn_text_to_image_worker<T, F>(...) - copy text_to_text spawn pattern, create 1 channel
    // generate_image(...) -> AsyncStream<ImageGenerationChunk> - copy text_to_text prompt pattern
}

// Helper: query_system_memory_mb() - copy from text_embedding.rs
```

**Key Implementation Details**:
1. Same Pattern B as Vision module
2. Different chunk type: ImageGenerationChunk instead of CandleStringChunk
3. Device parameter must be cloned/passed through request
4. ImageGenerationConfig contains width, height, steps, guidance_scale, etc.

## SUBTASK 4: Wire Up Module Exports

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/mod.rs`

Add to existing exports:

```rust
pub mod text_embedding;
pub mod text_to_text;
pub mod image_embedding;  // NEW
pub mod vision;           // NEW
pub mod text_to_image;    // NEW

pub use text_embedding::text_embedding_pool;
pub use text_to_text::text_to_text_pool;
pub use image_embedding::image_embedding_pool;  // NEW
pub use vision::vision_pool;                    // NEW
pub use text_to_image::text_to_image_pool;      // NEW
```

## IMPLEMENTATION CHECKLIST

**ImageEmbedding Module** (`image_embedding.rs`):
- [ ] Define 4 Request structs with response channels
- [ ] Define ImageEmbeddingWorkerHandle with 4 channels + shutdown
- [ ] Implement image_embedding_worker loop with 4 select! arms + shutdown
- [ ] Create IMAGE_EMBEDDING_WORKERS DashMap storage
- [ ] Create IMAGE_EMBEDDING_POOL Lazy static
- [ ] Implement image_embedding_pool() accessor
- [ ] Implement spawn_image_embedding_worker() (follow text_embedding spawn pattern)
- [ ] Implement embed_image() API method (Pattern A: bounded(0), recv_timeout)
- [ ] Implement embed_image_url() API method (Pattern A)
- [ ] Implement embed_image_base64() API method (Pattern A)
- [ ] Implement batch_embed_images() API method (Pattern A)
- [ ] Add query_system_memory_mb() helper

**Vision Module** (`vision.rs`):
- [ ] Define 2 Request structs with AsyncStream response channels
- [ ] Define VisionWorkerHandle with 2 channels + shutdown
- [ ] Implement vision_worker loop with 2 select! arms + shutdown
- [ ] Create VISION_WORKERS DashMap storage
- [ ] Create VISION_POOL Lazy static
- [ ] Implement vision_pool() accessor
- [ ] Implement spawn_vision_worker() (follow text_to_text spawn pattern)
- [ ] Implement describe_image() API method (Pattern B: spawn_stream, blocking iteration)
- [ ] Implement describe_url() API method (Pattern B)
- [ ] Add query_system_memory_mb() helper

**TextToImage Module** (`text_to_image.rs`):
- [ ] Define 1 Request struct with AsyncStream response channel
- [ ] Define TextToImageWorkerHandle with 1 channel + shutdown
- [ ] Implement text_to_image_worker loop with 1 select! arm + shutdown
- [ ] Create TEXT_TO_IMAGE_WORKERS DashMap storage
- [ ] Create TEXT_TO_IMAGE_POOL Lazy static
- [ ] Implement text_to_image_pool() accessor
- [ ] Implement spawn_text_to_image_worker() (follow text_to_text spawn pattern)
- [ ] Implement generate_image() API method (Pattern B: spawn_stream, blocking iteration)
- [ ] Add query_system_memory_mb() helper

**Module Exports** (`mod.rs`):
- [ ] Add `pub mod image_embedding;`
- [ ] Add `pub mod vision;`
- [ ] Add `pub mod text_to_image;`
- [ ] Add `pub use image_embedding::image_embedding_pool;`
- [ ] Add `pub use vision::vision_pool;`
- [ ] Add `pub use text_to_image::text_to_image_pool;`

**Compilation**:
- [ ] `cargo check` passes without errors
- [ ] No unused import warnings
- [ ] All trait bounds satisfied

## DEPENDENCIES

**Requires**: 
- MPOOL_2A (Pool<T> core implementation)
- MPOOL_2B (WorkerHandle, PoolConfig, PoolError types)

**Blocks**: 
- MPOOL_5 (registry integration with all 5 capability pools)

**Parallel with**: 
- MPOOL_3A (TextEmbedding module) - COMPLETE
- MPOOL_3B (TextToText module) - COMPLETE

## CRITICAL IMPLEMENTATION NOTES

### Memory Management
- Check 80% system memory limit before spawning workers
- Track memory with atomic counters in Pool<T>
- Use sysinfo crate to query total system memory

### Worker Selection
- Find least-busy worker: `min_by_key(|w| w.core.pending_requests.load(Ordering::Acquire))`
- Increment pending_requests before sending request
- Decrement pending_requests after receiving response
- Touch worker (update last_used timestamp) on each request

### Channel Types
- **Pattern A** (ImageEmbedding): Use `bounded(0)` for synchronous rendezvous
- **Pattern B** (Vision, TextToImage): Use `unbounded()` for AsyncStream responses
- **Shutdown**: Always use `unbounded()` channel

### AsyncStream Forwarding (Pattern B)
```rust
// CORRECT pattern (from text_to_text.rs:247-252)
for chunk in worker_stream {
    ystream::emit!(sender, chunk);
}

// ❌ WRONG - AsyncStream is !Send, cannot use runtime.spawn
runtime.spawn(async move {
    while let Some(chunk) = worker_stream.next().await {
        ystream::emit!(sender, chunk);
    }
});
```

### Error Handling
- Convert trait errors to PoolError::ModelError
- Use `map_err(|e| PoolError::ModelError(e.to_string()))`
- Check shutdown flag before processing requests
- Return PoolError::ShuttingDown if pool is shutting down
- Return PoolError::NoWorkers if no workers available
- Return PoolError::Timeout on recv_timeout failure

## CODE GENERATION GUIDELINES

### Zero Allocation Patterns
- Use `Arc::clone()` instead of creating new Arcs
- Reuse DashMap entries with `entry().or_insert_with(Vec::new)`
- Pre-allocate Vec capacity where known
- Use `as_str()` to avoid string clones in trait calls

### Performance Optimizations
- Inline worker selection logic (no function calls in hot path)
- Use Ordering::Acquire for reads, Ordering::Release for writes
- Minimize lock contention (DashMap for concurrent access)
- Touch worker atomically (no mutex locks)

### Correctness
- Never use `unwrap()` or `expect()` in src/
- Handle all Result and Option types explicitly
- Use `map_err()` for error conversion
- Check shutdown flag before long operations
- Validate worker availability before sending requests

### Code Quality
- Follow existing patterns from text_embedding.rs and text_to_text.rs exactly
- Copy helper functions (query_system_memory_mb) to each file
- Use descriptive variable names (embed_image_tx, not tx1)
- Add doc comments for public types and methods
- Use consistent formatting (rustfmt)

## DEFINITION OF DONE

- [ ] All 3 pool modules created (image_embedding.rs, vision.rs, text_to_image.rs)
- [ ] All Request types defined with proper generics
- [ ] All WorkerHandle types defined with channel fields
- [ ] All worker loop functions implemented with select! macro
- [ ] All global storage (DashMap) created
- [ ] All global pool instances (Lazy<Pool<T>>) created
- [ ] All accessor functions (image_embedding_pool, vision_pool, text_to_image_pool) implemented
- [ ] All spawn methods implemented following reference patterns
- [ ] All API methods implemented (11 total: 4 ImageEmbedding + 2 Vision + 1 TextToImage)
- [ ] Module exports wired up in mod.rs
- [ ] `cargo check` passes without errors or warnings
- [ ] Memory checks implemented (80% limit)
- [ ] Worker selection implemented (least-busy)
- [ ] Error handling complete (no unwrap/expect)
- [ ] AsyncStream forwarding uses correct blocking pattern
- [ ] All atomic operations use proper Ordering

**Success Criteria**: Code compiles, follows existing patterns exactly, handles all edge cases, uses zero unsafe code, optimized for performance.

## REFERENCE FILE LOCATIONS

- **Pattern A Reference**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/text_embedding.rs`
- **Pattern B Reference**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/text_to_text.rs`
- **Trait Definitions**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/traits.rs`
- **Pool Core**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/core/pool.rs`
- **Pool Types**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/core/types.rs`
- **Existing Models**:
  - ImageEmbedding: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/image_embedding/clip_vision_embedding.rs`
  - Vision: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/vision/llava.rs`
  - TextToImage: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/text_to_image/flux_schnell.rs`
