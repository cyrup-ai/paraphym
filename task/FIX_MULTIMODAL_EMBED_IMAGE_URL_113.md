# Fix Runtime Creation in multimodal_service.rs:113 (embed_image_url)

**Location:** `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/vector/multimodal_service.rs:113`  
**Priority:** HIGH  
**Issue Type:** Runtime Creation Antipattern

## Current Code (WRONG)

```rust
pub fn embed_image_url(&self, url: String) -> PendingEmbedding {
    let vision_model = self.vision_model.clone();
    let (tx, rx) = tokio::sync::oneshot::channel();

    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to create runtime for image URL embedding");
            
        rt.block_on(async move {
            let result = vision_model
                .embed_image_url(&url).await
                .map_err(|e| crate::memory::utils::error::Error::Other(format!("Failed to embed image from URL: {}", e)));
            let _ = tx.send(result);
        });
    });

    PendingEmbedding::new(rx)
}
```

## Problem

This code creates a **new tokio runtime** for each image URL embedding operation:
- `Runtime::new_current_thread()` is called per-operation (wasteful)
- Creates unnecessary runtime overhead
- Violates single-runtime architecture
- Pattern doesn't scale (each embedding = new runtime)

## Correct Solution

Use the shared runtime instead:

```rust
pub fn embed_image_url(&self, url: String) -> PendingEmbedding {
    let vision_model = self.vision_model.clone();
    let (tx, rx) = tokio::sync::oneshot::channel();

    std::thread::spawn(move || {
        let Some(rt) = crate::runtime::shared_runtime() else {
            log::error!("Shared runtime unavailable for image URL embedding");
            let _ = tx.send(Err(crate::memory::utils::error::Error::Other(
                "Runtime unavailable".to_string()
            )));
            return;
        };
        
        rt.block_on(async move {
            let result = vision_model
                .embed_image_url(&url).await
                .map_err(|e| crate::memory::utils::error::Error::Other(format!("Failed to embed image from URL: {}", e)));
            let _ = tx.send(result);
        });
    });

    PendingEmbedding::new(rx)
}
```

## Why This Pattern is Correct

1. **Single Runtime Architecture**: Uses the ONE shared runtime created at application startup
2. **Error Handling**: Gracefully handles runtime unavailability with early return
3. **Resource Efficiency**: No per-operation runtime creation overhead
4. **Thread Safety**: `std::thread::spawn` correctly handles `!Sync` types (async work doesn't need to be Sync)
5. **Consistent Pattern**: Matches the fix applied to cognitive_worker.rs

## Pattern Learned from cognitive_worker.rs Fix

Reference: `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/core/cognitive_worker.rs:272-328`

The cognitive_worker.rs fix showed that:
- `std::thread::spawn` does NOT require captured types to be Sync
- `shared_runtime().block_on()` is the ONLY correct way to run async work from sync context
- Early return with error message when runtime unavailable
- Never create dedicated runtimes

## Implementation Steps

1. **Read current code** to confirm exact line numbers haven't shifted
2. **Replace lines 113-124** with the corrected version above
3. **Verify imports** - ensure `log` crate is available if not already imported
4. **Test compilation**: `cargo check -p paraphym_candle`
5. **Verify runtime behavior** with image URL embedding test

## Related Issues

- Same antipattern exists at:
  - `multimodal_service.rs:89` (embed_image) - [FIX_MULTIMODAL_EMBED_IMAGE_89.md]
  - `multimodal_service.rs:137` (embed_image_base64) - next to fix
  - `multimodal_service.rs:161` (batch_embed_images) - next to fix

All four must be fixed with the same shared_runtime() pattern.
