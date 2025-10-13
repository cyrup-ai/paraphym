# Fix Incorrect block_on Usage in multimodal_service.rs Line 124

## Location
`packages/candle/src/memory/vector/multimodal_service.rs:124`

## Problem
Spawning `std::thread` just to call `block_on` on an async method for URL embedding.

```rust
std::thread::spawn(move || {
    let Some(rt) = crate::runtime::shared_runtime() else {
        // error handling
        return;
    };
    
    rt.block_on(async move {
        let result = vision_model
            .embed_image_url(&url).await
            .map_err(|e| crate::memory::utils::error::Error::Other(format!("Failed to embed image from URL: {}", e)));
        let _ = tx.send(result);
    });
});
```

## Why It's Wrong
- Same anti-pattern as embed_image (line 97)
- Unnecessary thread spawning for async work
- `block_on` used as workaround instead of proper async

## Correct Approach
Use `tokio::spawn`:

```rust
tokio::spawn(async move {
    let result = vision_model
        .embed_image_url(&url).await
        .map_err(|e| crate::memory::utils::error::Error::Other(format!("Failed to embed image from URL: {}", e)));
    let _ = tx.send(result);
});
```

## Impact
- Eliminates unnecessary OS thread creation
- Proper async/await usage
- Better resource utilization
