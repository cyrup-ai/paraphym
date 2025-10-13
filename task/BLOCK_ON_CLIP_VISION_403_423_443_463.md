# Remove block_on from clip_vision.rs:403, 423, 443, 463 (MEDIUM)

**Locations:**
- `src/capability/image_embedding/clip_vision.rs:403` - embed_image
- `src/capability/image_embedding/clip_vision.rs:423` - embed_image_url  
- `src/capability/image_embedding/clip_vision.rs:443` - embed_image_base64
- `src/capability/image_embedding/clip_vision.rs:463` - batch_embed_images

**Priority:** MEDIUM - Sync trait methods using shared_runtime(), architectural question

## Current Code Pattern

All four methods in ClipVisionModel's ImageEmbeddingCapable implementation follow this pattern:

```rust
impl crate::capability::traits::ImageEmbeddingCapable for ClipVisionModel {
    fn embed_image(&self, image_path: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> 
    {
        // Use shared runtime to avoid nested runtime issues
        let rt = crate::runtime::shared_runtime()
            .ok_or_else(|| Box::new(std::io::Error::other("Shared runtime unavailable")) as Box<dyn std::error::Error + Send + Sync>)?;
        
        // Encode image to tensor (1, embed_dim)
        let tensor = rt.block_on(self.encode_image(image_path))
            .map_err(|e| Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error + Send + Sync>)?;
        
        // Extract first batch element and convert to Vec<f32>
        let embedding = tensor.get(0)
            .map_err(|e| format!("Failed to extract embedding: {}", e))?
            .to_vec1::<f32>()
            .map_err(|e| format!("Failed to convert embedding to vec: {}", e))?;
        
        Ok(embedding)
    }
    
    // Similar for embed_image_url, embed_image_base64, batch_embed_images
}
```

Context: Implementing sync trait ImageEmbeddingCapable for async operations.

## Problem: Sync Trait Requiring Blocking

The `ImageEmbeddingCapable` trait requires sync methods, but the underlying operations are async. This forces use of `shared_runtime().block_on()` which:
1. Can cause nested runtime errors if called from async contexts
2. Blocks threads unnecessarily
3. Defeats the purpose of async operations

This is an **architectural issue** - the trait design doesn't match the async nature of the operations.

## Solution Options

### Option 1: Make ImageEmbeddingCapable Async (RECOMMENDED)

Change the trait to be async using async_trait:

```rust
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

// Implementation becomes clean
#[async_trait::async_trait]
impl ImageEmbeddingCapable for ClipVisionModel {
    async fn embed_image(&self, image_path: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> 
    {
        // No blocking - just await
        let tensor = self.encode_image(image_path).await
            .map_err(|e| Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error + Send + Sync>)?;
        
        let embedding = tensor.get(0)
            .map_err(|e| format!("Failed to extract embedding: {}", e))?
            .to_vec1::<f32>()
            .map_err(|e| format!("Failed to convert embedding to vec: {}", e))?;
        
        Ok(embedding)
    }
    
    // Similar clean async implementations for other methods
}
```

Then update all call sites to use `.await`.

### Option 2: Keep Sync Trait + Document (if trait can't change)

If the trait must remain sync (external constraints), keep current approach but document:

```rust
impl crate::capability::traits::ImageEmbeddingCapable for ClipVisionModel {
    /// # Warning
    /// This method blocks on async operations using shared_runtime().
    /// Do not call from async contexts to avoid nested runtime errors.
    /// Consider using `self.encode_image().await` directly if in async context.
    fn embed_image(&self, image_path: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> 
    {
        let rt = crate::runtime::shared_runtime()
            .ok_or_else(|| Box::new(std::io::Error::other(
                "Shared runtime unavailable. Cannot call from async context."
            )) as Box<dyn std::error::Error + Send + Sync>)?;
        
        // ... rest stays same ...
    }
}
```

**Pattern Explanation:**
- **ANTIPATTERN (current):** Sync trait forces blocking on async operations
- **RECOMMENDED (fix):** Make trait async with #[async_trait]
- **ALTERNATIVE:** Keep sync + document clearly

## Implementation Notes

1. Check if ImageEmbeddingCapable trait can be changed to async
2. Search all implementations and usages of the trait
3. If trait can be async, add async_trait dependency and convert
4. Update all call sites to use `.await`
5. If trait must stay sync, add comprehensive documentation

## Impact Analysis Needed

1. Where is ImageEmbeddingCapable trait defined?
2. What code depends on it being sync?
3. How many implementations exist?
4. How many call sites would need updating?

## Related Tasks

- BLOCK_ON_CLIP_VISION_EMB_101_119_136_153.md - Same architectural issue in ClipVisionEmbeddingModel
- BLOCK_ON_LOADED_CLIP_317_341_365_389.md - Same architectural issue in LoadedClipVisionModel

All these are symptoms of the same root cause: sync trait for async operations.
