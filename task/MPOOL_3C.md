# MPOOL_3C: Implement ImageEmbedding, Vision, TextToImage Capability Modules

**PREFIX**: MPOOL (Model Pool)

## OBJECTIVE

Implement remaining 3 capability modules: `image_embedding.rs`, `vision.rs`, and `text_to_image.rs`. These follow same patterns as text_embedding and text_to_text but are combined into one task as they have fewer models using them.

## CONTEXT

- **ImageEmbedding**: 4 methods (embed_image, embed_image_url, embed_image_base64, batch_embed_images) - immediate results
- **Vision**: 2 methods (describe_image, describe_url) - streaming results
- **TextToImage**: 1 method (generate_image) - streaming results

## SUBTASK 1: Implement ImageEmbedding Module

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/image_embedding.rs`

**Implementation**:
```rust
use crossbeam::channel::{Sender, Receiver};
use crate::pool::core::PoolError;
use crate::capability::traits::ImageEmbeddingCapable;
use crate::pool::core::{Pool, PoolConfig, WorkerHandle};
use once_cell::sync::Lazy;
use crossbeam::select;

/// Request structs (4 operations)
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

/// Worker handle with 4 channels
pub struct ImageEmbeddingWorkerHandle {
    pub core: WorkerHandle,
    pub embed_image_tx: Sender<EmbedImageRequest>,
    pub embed_image_url_tx: Sender<EmbedImageUrlRequest>,
    pub embed_image_base64_tx: Sender<EmbedImageBase64Request>,
    pub batch_embed_images_tx: Sender<BatchEmbedImagesRequest>,
}

/// Worker loop for ImageEmbedding models (4 channels)
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
                    let result = model.batch_embed_images(
                        req.image_paths.iter().map(|s| s.as_str()).collect()
                    ).map_err(|e| PoolError::ModelError(e.to_string()));
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

/// Global ImageEmbedding pool instance
static IMAGE_EMBEDDING_POOL: Lazy<Pool<dyn ImageEmbeddingCapable>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

pub fn image_embedding_pool() -> &'static Pool<dyn ImageEmbeddingCapable> {
    &IMAGE_EMBEDDING_POOL
}

impl Pool<dyn ImageEmbeddingCapable> {
    /// Spawn worker for ImageEmbedding model
    /// (Implementation follows text_embedding pattern with 4 channels)
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
        // Similar to spawn_text_embedding_worker but with 4 channels
        todo!("Follow text_embedding spawn pattern")
    }

    /// Embed image using pooled worker
    /// (Implementation follows embed_text pattern)
    pub fn embed_image(
        &self,
        registry_key: &str,
        image_path: &str,
    ) -> Result<Vec<f32>, PoolError> {
        todo!("Follow embed_text pattern")
    }

    // Add: embed_image_url, embed_image_base64, batch_embed_images methods
}
```

**Why**: ImageEmbedding has 4 operations, all immediate results (Pattern A).

## SUBTASK 2: Implement Vision Module

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/vision.rs`

**Implementation**:
```rust
use crossbeam::channel::{Sender, Receiver};
use crate::pool::core::PoolError;
use crate::capability::traits::VisionCapable;
use crate::pool::core::{Pool, PoolConfig, WorkerHandle};
use once_cell::sync::Lazy;
use crossbeam::select;
use ystream::AsyncStream;
use crate::domain::chat::message::CandleStringChunk;

/// Request structs (2 operations, both streaming)
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

/// Worker handle with 2 channels
pub struct VisionWorkerHandle {
    pub core: WorkerHandle,
    pub describe_image_tx: Sender<DescribeImageRequest>,
    pub describe_url_tx: Sender<DescribeUrlRequest>,
}

/// Worker loop for Vision models (2 channels, streaming)
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

/// Global Vision pool instance
static VISION_POOL: Lazy<Pool<dyn VisionCapable>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

pub fn vision_pool() -> &'static Pool<dyn VisionCapable> {
    &VISION_POOL
}

impl Pool<dyn VisionCapable> {
    /// Spawn worker for Vision model (follow text_to_text pattern)
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
        todo!("Follow text_to_text spawn pattern")
    }

    /// Describe image using pooled worker (returns AsyncStream)
    pub fn describe_image(
        &self,
        registry_key: &str,
        image_path: &str,
        query: &str,
    ) -> AsyncStream<CandleStringChunk> {
        todo!("Follow text_to_text prompt pattern")
    }

    // Add: describe_url method
}
```

**Why**: Vision has 2 streaming operations (Pattern B).

## SUBTASK 3: Implement TextToImage Module

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/text_to_image.rs`

**Implementation**:
```rust
use crossbeam::channel::{Sender, Receiver};
use crate::pool::core::PoolError;
use crate::capability::traits::TextToImageCapable;
use crate::pool::core::{Pool, PoolConfig, WorkerHandle};
use once_cell::sync::Lazy;
use crossbeam::select;
use ystream::AsyncStream;
use crate::domain::image::ImageGenerationChunk;
use crate::domain::image::ImageGenerationConfig;
use candle_core::Device;

/// Request struct (1 operation, streaming)
pub struct GenerateImageRequest {
    pub prompt: String,
    pub config: ImageGenerationConfig,
    pub device: Device,
    pub response: Sender<Result<AsyncStream<ImageGenerationChunk>, PoolError>>,
}

/// Worker handle with 1 channel
pub struct TextToImageWorkerHandle {
    pub core: WorkerHandle,
    pub generate_image_tx: Sender<GenerateImageRequest>,
}

/// Worker loop for TextToImage models (1 channel, streaming)
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

/// Global TextToImage pool instance
static TEXT_TO_IMAGE_POOL: Lazy<Pool<dyn TextToImageCapable>> = Lazy::new(|| {
    Pool::new(PoolConfig::default())
});

pub fn text_to_image_pool() -> &'static Pool<dyn TextToImageCapable> {
    &TEXT_TO_IMAGE_POOL
}

impl Pool<dyn TextToImageCapable> {
    /// Spawn worker for TextToImage model (follow text_to_text pattern)
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
        todo!("Follow text_to_text spawn pattern")
    }

    /// Generate image using pooled worker (returns AsyncStream)
    pub fn generate_image(
        &self,
        registry_key: &str,
        prompt: &str,
        config: &ImageGenerationConfig,
        device: &Device,
    ) -> AsyncStream<ImageGenerationChunk> {
        todo!("Follow text_to_text prompt pattern")
    }
}
```

**Why**: TextToImage has 1 streaming operation (Pattern B).

## SUBTASK 4: Wire Up Module Exports

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/mod.rs`

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

## DEFINITION OF DONE

- [ ] `image_embedding.rs` file created with 4 Request types and worker loop
- [ ] `vision.rs` file created with 2 Request types and streaming worker loop
- [ ] `text_to_image.rs` file created with 1 Request type and streaming worker loop
- [ ] All 3 global pool instances created
- [ ] All 3 accessor functions implemented
- [ ] Spawn methods implemented for all 3 pools
- [ ] API methods implemented (embed_image, describe_image, generate_image, etc.)
- [ ] Module exports configured
- [ ] Code compiles with `cargo check`

## DEPENDENCIES

**Requires**: MPOOL_2A (Pool<T>), MPOOL_2B (worker helpers)

**Blocks**: MPOOL_5 (registry integration)

**Parallel with**: MPOOL_3A (TextEmbedding), MPOOL_3B (TextToText)

## RESEARCH NOTES

**Channel Counts per Capability** (from MODEL_POOL.md Scenario 3):
- TextEmbedding: 2 channels (embed, batch_embed)
- TextToText: 1 channel (prompt)
- ImageEmbedding: 4 channels (embed_image, embed_image_url, embed_image_base64, batch_embed_images)
- Vision: 2 channels (describe_image, describe_url)
- TextToImage: 1 channel (generate_image)

**Response Patterns**:
- ImageEmbedding: Immediate results (Pattern A)
- Vision: Streaming results (Pattern B)
- TextToImage: Streaming results (Pattern B)

## CONSTRAINTS

- **NO TESTS**: Do not write any test code. Tests are handled by separate team.
- **NO BENCHMARKS**: Do not write any benchmark code. Benchmarks are handled by separate team.
- **GENERIC TRAITS**: Worker loops work for ANY model implementing the traits.
- **FOLLOW PATTERNS**: ImageEmbedding follows text_embedding pattern, Vision/TextToImage follow text_to_text pattern.
