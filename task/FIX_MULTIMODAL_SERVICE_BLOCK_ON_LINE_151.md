# Fix Incorrect block_on Usage in multimodal_service.rs Line 151

## Location
`packages/candle/src/memory/vector/multimodal_service.rs:151`

## Problem
Spawning `std::thread` just to call `block_on` on an async method for base64 embedding.

```rust
std::thread::spawn(move || {
    let Some(rt) = crate::runtime::shared_runtime() else {
        // error handling
        return;
    };
    
    rt.block_on(async move {
        let result = vision_model
            .embed_image_base64(&base64_data).await
            .map_err(|e| crate::memory::utils::error::Error::Other(format!("Failed to embed image from base64: {}", e)));
        let _ = tx.send(result);
    });
});
```

## Why It's Wrong
- Same anti-pattern as other embed methods
- Thread spawn + block_on = inefficient async bridge
- Should use tokio task instead

## Correct Approach
Use `tokio::spawn`:

```rust
tokio::spawn(async move {
    let result = vision_model
        .embed_image_base64(&base64_data).await
        .map_err(|e| crate::memory::utils::error::Error::Other(format!("Failed to embed image from base64: {}", e)));
    let _ = tx.send(result);
});
```

## Impact
- Removes unnecessary thread overhead
- Proper async pattern
- Consistent with async best practices
