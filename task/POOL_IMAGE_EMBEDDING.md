# POOL_IMAGE_EMBEDDING: Integrate ImageEmbedding Pool into Registry

## OBJECTIVE

Integrate the ImageEmbedding pool into `capability/registry.rs` so that ClipVision routes through the pool with automatic worker spawning and memory management for all 4 image embedding methods.

## SCOPE

**1 Model:**
- ClipVision

**4 Methods:**
- `embed_image()` - embed from file path
- `embed_image_url()` - embed from URL
- `embed_image_base64()` - embed from base64 string
- `batch_embed_images()` - batch embed multiple images

## CURRENT STATE (Direct Call)

**File**: `packages/candle/src/capability/registry.rs` lines ~485-522

```rust
impl ImageEmbeddingCapable for ImageEmbeddingModel {
    fn embed_image(&self, image_path: &str)
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::ClipVision(m) => m.embed_image(image_path)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        }
    }

    fn embed_image_url(&self, url: &str)
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::ClipVision(m) => m.embed_image_url(url)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        }
    }

    fn embed_image_base64(&self, base64_data: &str)
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::ClipVision(m) => m.embed_image_base64(base64_data)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        }
    }

    fn batch_embed_images(&self, image_paths: Vec<&str>)
        -> std::result::Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::ClipVision(m) => m.batch_embed_images(image_paths)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        }
    }

    fn embedding_dimension(&self) -> usize {
        match self {
            Self::ClipVision(m) => m.embedding_dimension(),
        }
    }
}
```

## REQUIRED STATE (Pool Integration)

Add cold start logic for ALL 4 embedding methods:

```rust
impl ImageEmbeddingCapable for ImageEmbeddingModel {
    fn embed_image(&self, image_path: &str)
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::ClipVision(m) => {
                let registry_key = m.info().registry_key;
                let pool = image_embedding_pool();

                // Cold start: spawn workers if needed
                if !pool.has_workers(registry_key) {
                    let per_worker_mb = m.info().est_memory_allocation_mb;
                    let current_mb = pool.total_memory_mb();
                    let total_system_mb = query_system_memory_mb();
                    let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;

                    let workers_to_spawn = if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
                        2
                    } else if current_mb + per_worker_mb <= memory_limit_mb {
                        1
                    } else {
                        return Err(Box::new(PoolError::MemoryExhausted(format!(
                            "Cannot spawn workers for {}. Need {} MB, only {} MB available",
                            registry_key, per_worker_mb, memory_limit_mb.saturating_sub(current_mb)
                        ))) as Box<dyn std::error::Error + Send + Sync>);
                    };

                    for _ in 0..workers_to_spawn {
                        let m_clone = m.clone();
                        pool.spawn_image_embedding_worker(
                            registry_key,
                            move || {
                                LoadedClipVisionModel::load(&m_clone)
                                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                            },
                            per_worker_mb,
                        ).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                    }
                }

                // Route through pool
                pool.embed_image(registry_key, image_path)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
        }
    }

    fn embed_image_url(&self, url: &str)
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::ClipVision(m) => {
                let registry_key = m.info().registry_key;
                let pool = image_embedding_pool();

                // Same cold start logic as embed_image()
                if !pool.has_workers(registry_key) {
                    // ... (copy cold start block from embed_image)
                }

                // Route through pool
                pool.embed_image_url(registry_key, url)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
        }
    }

    fn embed_image_base64(&self, base64_data: &str)
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::ClipVision(m) => {
                let registry_key = m.info().registry_key;
                let pool = image_embedding_pool();

                // Same cold start logic
                if !pool.has_workers(registry_key) {
                    // ... (copy cold start block)
                }

                // Route through pool
                pool.embed_image_base64(registry_key, base64_data)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
        }
    }

    fn batch_embed_images(&self, image_paths: Vec<&str>)
        -> std::result::Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::ClipVision(m) => {
                let registry_key = m.info().registry_key;
                let pool = image_embedding_pool();

                // Same cold start logic
                if !pool.has_workers(registry_key) {
                    // ... (copy cold start block)
                }

                // Route through pool
                pool.batch_embed_images(registry_key, image_paths)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
        }
    }

    fn embedding_dimension(&self) -> usize {
        // No pool routing needed - this is just metadata
        match self {
            Self::ClipVision(m) => m.embedding_dimension(),
        }
    }
}
```

## REQUIRED IMPORTS

Add to top of registry.rs:

```rust
use crate::pool::capabilities::image_embedding::image_embedding_pool;
```

LoadedModel type (verify exists):
- LoadedClipVisionModel

## IMPLEMENTATION STEPS

### 1. Add Pool Import
Add `image_embedding_pool` to imports at top of registry.rs

### 2. Extract Cold Start Helper (Optional)
Since the cold start logic is identical across all 4 methods, you could extract it to a helper function within the impl block

### 3. Update embed_image()
Add cold start logic and pool routing

### 4. Update embed_image_url()
Copy cold start logic from embed_image(), route through pool

### 5. Update embed_image_base64()
Copy cold start logic, route through pool

### 6. Update batch_embed_images()
Copy cold start logic, route through pool

### 7. Leave embedding_dimension() Unchanged
This is metadata-only, no pool routing needed

### 8. Verify LoadedClipVisionModel Exists
Search for LoadedClipVisionModel definition

### 9. Add LoadedModel Import
Import LoadedClipVisionModel if it exists

## POOL INFRASTRUCTURE (Already Complete)

✅ `image_embedding_pool()` - Global pool accessor
✅ `pool.has_workers(registry_key)` - Check if workers exist
✅ `pool.spawn_image_embedding_worker()` - Spawn worker with model loader
✅ `pool.embed_image()` - Route request to worker
✅ `pool.embed_image_url()` - Route URL request
✅ `pool.embed_image_base64()` - Route base64 request
✅ `pool.batch_embed_images()` - Route batch request
✅ `pool.total_memory_mb()` - Current memory usage
✅ Worker loop in `pool/capabilities/image_embedding.rs`
✅ Maintenance thread coordinates this pool

## VERIFICATION

### Compile Check
```bash
cargo check -p paraphym_candle
```

### Test All 4 Methods
```rust
let model = registry::get_image_embedding_runtime("openai/clip-vit-base-patch32")?;

// Test single image
let emb1 = model.embed_image("path/to/image.jpg")?;  // Should spawn 2 workers

// Test URL (workers already exist)
let emb2 = model.embed_image_url("https://example.com/image.jpg")?;

// Test base64
let emb3 = model.embed_image_base64("iVBORw0KG...")?;

// Test batch
let embs = model.batch_embed_images(vec!["img1.jpg", "img2.jpg"])?;
```

## DEFINITION OF DONE

- [ ] `image_embedding_pool` imported in registry.rs
- [ ] `embed_image()` has cold start logic and routes through pool
- [ ] `embed_image_url()` has cold start logic and routes through pool
- [ ] `embed_image_base64()` has cold start logic and routes through pool
- [ ] `batch_embed_images()` has cold start logic and routes through pool
- [ ] `embedding_dimension()` unchanged (metadata only)
- [ ] LoadedClipVisionModel imported
- [ ] `cargo check -p paraphym_candle` passes
- [ ] No unwrap() or expect() in implementation

## ESTIMATED TIME

15 minutes (4 methods, but repetitive pattern)
