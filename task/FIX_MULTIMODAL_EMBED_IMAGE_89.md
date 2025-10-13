# Fix Runtime Creation in multimodal_service.rs:89 (embed_image)

**Location:** `src/memory/vector/multimodal_service.rs:89`  
**Priority:** HIGH - Creating unnecessary runtime per call

## Current Code (WRONG)

```rust
std::thread::spawn(move || {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to create runtime for image embedding");
        
    rt.block_on(async move {
        let result = vision_model
            .embed_image(&image_path).await
            .map_err(|e| crate::memory::utils::error::Error::Other(format!("Failed to embed image: {}", e)));
        let _ = tx.send(result);
    });
});
```

## Problem

Creates a NEW runtime for every single image embedding call. This is wasteful and violates the principle of using ONE shared runtime.

The method already uses `std::thread::spawn` to handle !Sync AsyncStream from vision_model.embed_image(), which is correct. The ONLY issue is creating a new runtime instead of using the shared one.

## Correct Solution

Use `crate::runtime::shared_runtime()` instead of creating a new runtime:

```rust
std::thread::spawn(move || {
    let Some(rt) = crate::runtime::shared_runtime() else {
        log::error!("Shared runtime unavailable for image embedding");
        let _ = tx.send(Err(crate::memory::utils::error::Error::Other(
            "Runtime unavailable".to_string()
        )));
        return;
    };
        
    rt.block_on(async move {
        let result = vision_model
            .embed_image(&image_path).await
            .map_err(|e| crate::memory::utils::error::Error::Other(format!("Failed to embed image: {}", e)));
        let _ = tx.send(result);
    });
});
```

## Why This Pattern is Correct

1. **std::thread::spawn** - Required because vision_model.embed_image() returns AsyncStream which is !Sync
2. **shared_runtime()** - Uses the ONE shared runtime instance, not creating a new one
3. **Error handling** - Properly handles the case where shared_runtime is unavailable
4. **Returns PendingEmbedding** - The sync method returns a future wrapper, caller controls when to await

## Pattern Learned from cognitive_worker.rs Fix

This is the EXACT same pattern used to fix cognitive_worker.rs:
- Thread spawn for !Sync types ✓
- Shared runtime, not new runtime ✓
- Proper error handling ✓
- No nested runtime issues ✓

## Implementation Steps

1. Replace lines 89-93 with the corrected code
2. Verify the method signature remains unchanged (sync method returning PendingEmbedding)
3. Test compilation: `cargo check`
4. Verify no runtime is created per call
