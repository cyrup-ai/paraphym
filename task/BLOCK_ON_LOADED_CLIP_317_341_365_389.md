# Remove block_on from LoadedClipVisionModel:317, 341, 365, 389 (HIGH)

**Locations:**
- `src/capability/image_embedding/clip_vision_embedding.rs:317` - embed_image
- `src/capability/image_embedding/clip_vision_embedding.rs:341` - embed_image_url
- `src/capability/image_embedding/clip_vision_embedding.rs:365` - embed_image_base64
- `src/capability/image_embedding/clip_vision_embedding.rs:389` - batch_embed_images

**Priority:** HIGH - Using self.runtime.block_on() with owned Runtime

## Current Code Pattern

All four methods in `LoadedClipVisionModel` follow this pattern:

```rust
impl crate::capability::traits::ImageEmbeddingCapable for LoadedClipVisionModel {
    fn embed_image(&self, image_path: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> 
    {
        let provider = Arc::clone(&self.model.provider);
        let result = self.runtime.block_on(async {
            provider.encode_image(image_path).await
        });
        
        match result {
            Ok(tensor) => {
                tensor.flatten_all()
                    .and_then(|t| t.to_vec1::<f32>())
                    .map_err(|e| Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to convert tensor: {}", e)
                    )) as Box<dyn std::error::Error + Send + Sync>)
            }
            Err(e) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                e
            )) as Box<dyn std::error::Error + Send + Sync>)
        }
    }
    
    // Similar for embed_image_url, embed_image_base64, batch_embed_images
}
```

Context: LoadedClipVisionModel owns a tokio::runtime::Runtime (created at line 296) and uses it for blocking.

## Problem: Using Owned Runtime for Blocking

The code owns a Runtime and blocks on it. This:
1. Each LoadedClipVisionModel has its own runtime (resource waste)
2. Using `self.runtime.block_on()` on owned runtime can still cause issues
3. Runtime::new() at line 296 can fail with nested runtime error
4. Should use shared_runtime pattern or make trait async

## Solution: Use shared_runtime Instead

### Step 1: Fix RUNTIME_NEW_CLIP_VISION_EMB_296.md First

Don't create owned Runtime - use shared_runtime:

```rust
pub struct LoadedClipVisionModel {
    model: ClipVisionEmbeddingModel,
    // Remove runtime field
}

impl LoadedClipVisionModel {
    pub fn load(base_model: &ClipVisionEmbeddingModel) -> Self {
        Self {
            model: ClipVisionEmbeddingModel {
                provider: Arc::clone(&base_model.provider),
                dimension: base_model.dimension,
            },
        }
    }
}
```

### Step 2: Use shared_runtime().block_on() in Trait Methods

```rust
impl crate::capability::traits::ImageEmbeddingCapable for LoadedClipVisionModel {
    fn embed_image(&self, image_path: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> 
    {
        // Use shared runtime instead of owned
        let runtime = crate::runtime::shared_runtime()
            .ok_or_else(|| Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Shared runtime unavailable"
            )) as Box<dyn std::error::Error + Send + Sync>)?;

        let provider = Arc::clone(&self.model.provider);
        let result = runtime.block_on(async {
            provider.encode_image(image_path).await
        });
        
        match result {
            Ok(tensor) => {
                tensor.flatten_all()
                    .and_then(|t| t.to_vec1::<f32>())
                    .map_err(|e| Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to convert tensor: {}", e)
                    )) as Box<dyn std::error::Error + Send + Sync>)
            }
            Err(e) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                e
            )) as Box<dyn std::error::Error + Send + Sync>)
        }
    }
    
    // Similar pattern for other 3 methods
}
```

**Pattern Explanation:**
- **ANTIPATTERN (current):** Own Runtime per model: `self.runtime.block_on(async_op())`
- **CORRECT (fix):** Use shared runtime: `shared_runtime().block_on(async_op())`

## Implementation Notes

1. **Must fix RUNTIME_NEW_CLIP_VISION_EMB_296.md first** - remove Runtime field
2. Update all 4 trait methods to use shared_runtime()
3. Keep block_on() since ImageEmbeddingCapable trait requires sync methods
4. This reduces resource usage (one shared runtime vs many owned runtimes)
5. Still has nested runtime risk if called from async, but better than current

## Long-term Solution

Consider if ImageEmbeddingCapable trait should be async:

```rust
#[async_trait::async_trait]
pub trait ImageEmbeddingCapable {
    async fn embed_image(&self, image_path: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>>;
    // ...
}
```

But this requires changing the trait definition and all implementations.

## Dependencies

- Must fix RUNTIME_NEW_CLIP_VISION_EMB_296.md first
- All 4 methods updated together
