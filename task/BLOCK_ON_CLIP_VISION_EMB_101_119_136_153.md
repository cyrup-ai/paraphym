# Remove block_on from clip_vision_embedding.rs:101, 119, 136, 153 (HIGH)

**Locations:** 
- `src/capability/image_embedding/clip_vision_embedding.rs:101` - embed_image
- `src/capability/image_embedding/clip_vision_embedding.rs:119` - embed_image_url
- `src/capability/image_embedding/clip_vision_embedding.rs:136` - embed_image_batch
- `src/capability/image_embedding/clip_vision_embedding.rs:153` - embed_url_batch

**Priority:** HIGH - Sync methods wrapping async, called from tokio::spawn

## Current Code Pattern

```rust
pub fn embed_image(&self, image_path: &str) -> Result<Vec<f32>> {
    let runtime = crate::runtime::shared_runtime()
        .ok_or_else(|| MemoryError::ModelError("Shared runtime unavailable".to_string()))?;

    let provider = self.provider.clone();
    let tensor = runtime
        .block_on(provider.encode_image(image_path))
        .map_err(|e| MemoryError::ModelError(format!("Image encoding failed: {}", e)))?;

    // ... convert tensor ...
}
```

## Problem: Sync Methods Called From tokio::spawn

These sync methods with block_on are called from `src/memory/vector/multimodal_service.rs:90, 107, 124, 142`:

```rust
// multimodal_service.rs:88-95
tokio::spawn(async move {
    let result = vision_model
        .embed_image(&image_path)  // ← Calls sync method with block_on!
        .map_err(|e| crate::memory::utils::error::Error::Other(format!("Failed to embed image: {}", e)));
    let _ = tx.send(result);
});
```

This is wrong: spawning async task to call sync method that blocks on async!

## Solution: Make Methods Async

### Step 1: Change methods to async

In `clip_vision_embedding.rs`:
```rust
pub async fn embed_image(&self, image_path: &str) -> Result<Vec<f32>> {
    let tensor = self.provider.encode_image(image_path).await
        .map_err(|e| MemoryError::ModelError(format!("Image encoding failed: {}", e)))?;

    tensor
        .flatten_all()
        .and_then(|t| t.to_vec1::<f32>())
        .map_err(|e| MemoryError::ModelError(format!("Failed to convert tensor to vector: {}", e)))
}

pub async fn embed_image_url(&self, url: &str) -> Result<Vec<f32>> {
    // Same pattern
}

pub async fn embed_image_batch(&self, images: &[String]) -> Result<Vec<Vec<f32>>> {
    // Same pattern
}

pub async fn embed_url_batch(&self, urls: &[String]) -> Result<Vec<Vec<f32>>> {
    // Same pattern  
}
```

### Step 2: Update call sites in multimodal_service.rs

Change lines 90, 107, 124, 142 to use `.await`:

```rust
// Line 88-95
tokio::spawn(async move {
    let result = vision_model
        .embed_image(&image_path).await  // ← Add .await
        .map_err(|e| crate::memory::utils::error::Error::Other(format!("Failed to embed image: {}", e)));
    let _ = tx.send(result);
});

// Line 105-109
tokio::spawn(async move {
    let result = vision_model
        .embed_image_url(&url).await  // ← Add .await
        .map_err(|e| crate::memory::utils::error::Error::Other(format!("Failed to embed image from URL: {}", e)));
    let _ = tx.send(result);
});

// Similar for batch methods
```

**Pattern Explanation:**
- **ANTIPATTERN (current):** `tokio::spawn(async { sync_method_with_block_on() })`
- **CORRECT (fix):** `tokio::spawn(async { async_method().await })`

## Implementation Notes

1. Change 4 methods in clip_vision_embedding.rs from `fn` to `async fn`
2. Remove all `runtime.block_on()` wrappers
3. Use `.await` directly on provider methods
4. Update multimodal_service.rs lines 90, 107, 124, 142 to add `.await`
5. Also check lines 159, 182, 216 which may already be async

## Files to Modify

- `src/capability/image_embedding/clip_vision_embedding.rs` - Make 4 methods async
- `src/memory/vector/multimodal_service.rs` - Add .await at lines 90, 107, 124, 142
