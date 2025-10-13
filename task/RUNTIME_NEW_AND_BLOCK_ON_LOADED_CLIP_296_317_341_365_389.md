# Remove Runtime::new() and block_on from LoadedClipVisionModel (HIGH)

**Locations:**
- `src/capability/image_embedding/clip_vision_embedding.rs:296` - Runtime::new()
- `src/capability/image_embedding/clip_vision_embedding.rs:317` - self.runtime.block_on()
- `src/capability/image_embedding/clip_vision_embedding.rs:341` - self.runtime.block_on()
- `src/capability/image_embedding/clip_vision_embedding.rs:365` - self.runtime.block_on()
- `src/capability/image_embedding/clip_vision_embedding.rs:389` - self.runtime.block_on()

**Priority:** HIGH - Creates Runtime per worker, uses it for blocking

## Current Code

```rust
pub struct LoadedClipVisionModel {
    model: ClipVisionEmbeddingModel,
    runtime: tokio::runtime::Runtime,  // ← Owns a Runtime
}

impl LoadedClipVisionModel {
    pub fn load(base_model: &ClipVisionEmbeddingModel)
        -> std::result::Result<Self, Box<dyn std::error::Error + Send + Sync>>
    {
        // Create persistent runtime for this worker
        let runtime = tokio::runtime::Runtime::new()  // ← Line 296
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        let model = ClipVisionEmbeddingModel {
            provider: Arc::clone(&base_model.provider),
            dimension: base_model.dimension,
        };
        
        Ok(Self { model, runtime })
    }
}

impl ImageEmbeddingCapable for LoadedClipVisionModel {
    fn embed_image(&self, image_path: &str) -> Result<...> {
        let result = self.runtime.block_on(async {  // ← Line 317
            provider.encode_image(image_path).await
        });
        // ...
    }
    // Lines 341, 365, 389 follow same pattern
}
```

**Used in:** `src/capability/registry.rs:643, 676, 709, 742` - spawned in worker threads via closures.

## Problem: Each Worker Creates Its Own Runtime

LoadedClipVisionModel is designed for worker threads in the pool. Each worker:
1. Creates its own tokio::runtime::Runtime (line 296)
2. Stores it in the struct
3. Uses self.runtime.block_on() for all operations (lines 317, 341, 365, 389)

This is wasteful - multiple runtimes when one shared runtime would suffice.

## Solution: Remove LoadedClipVisionModel Entirely

LoadedClipVisionModel exists only to own a Runtime for blocking. This is unnecessary.

### Step 1: Remove LoadedClipVisionModel struct

Delete the entire LoadedClipVisionModel struct and its impl blocks from `clip_vision_embedding.rs`.

### Step 2: Update worker spawning in registry.rs

Change from:
```rust
// registry.rs:643
move || LoadedClipVisionModel::load(&m_clone)
    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
```

To:
```rust
move || Ok(m_clone.clone())
```

The worker just needs a clone of ClipVisionEmbeddingModel, not a wrapped version with runtime.

### Step 3: Update ImageEmbeddingCapable trait to async

The reason LoadedClipVisionModel exists is because ImageEmbeddingCapable trait is sync. Fix the trait:

```rust
// In src/capability/traits.rs
#[async_trait::async_trait]
pub trait ImageEmbeddingCapable: Send + Sync {
    async fn embed_image(&self, image_path: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>>;
    
    async fn embed_image_url(&self, url: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>>;
    
    async fn embed_image_base64(&self, base64_data: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>>;
    
    async fn batch_embed_images(&self, image_paths: Vec<&str>) 
        -> std::result::Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>>;
    
    fn embedding_dimension(&self) -> usize;
    fn supported_dimensions(&self) -> Vec<usize>;
}
```

### Step 4: Update pool worker to handle async trait

Workers in the pool need to properly handle async trait methods. Check `src/pool/capabilities/image_embedding.rs` and ensure workers can call async methods.

**Pattern Explanation:**
- **ANTIPATTERN (current):** Wrapper struct that owns Runtime for blocking
- **CORRECT (fix):** Remove wrapper, make trait async, workers use async methods

## Implementation Notes

1. Delete LoadedClipVisionModel entirely from clip_vision_embedding.rs
2. Make ImageEmbeddingCapable trait async with #[async_trait]
3. Update ClipVisionEmbeddingModel to implement async trait (remove block_on from lines 101, 119, 136, 153)
4. Update ClipVisionModel to implement async trait (remove block_on from lines 403, 423, 443, 463)
5. Update registry.rs worker spawning at lines 643, 676, 709, 742
6. Update pool worker implementation to call async trait methods

## Dependencies

- Related to BLOCK_ON_CLIP_VISION_EMB_101_119_136_153.md
- Related to BLOCK_ON_CLIP_VISION_403_423_443_463.md
- Requires making ImageEmbeddingCapable trait async

## Files to Modify

- `src/capability/image_embedding/clip_vision_embedding.rs` - Delete LoadedClipVisionModel
- `src/capability/image_embedding/clip_vision.rs` - Make methods async
- `src/capability/traits.rs` - Make ImageEmbeddingCapable async
- `src/capability/registry.rs` - Update worker spawning (4 locations)
- `src/pool/capabilities/image_embedding.rs` - Update worker to handle async
