# Fix Incorrect block_on Usage in multimodal_service.rs Line 178

## Location
`packages/candle/src/memory/vector/multimodal_service.rs:178`

## Problem
Spawning `std::thread` just to call `block_on` on an async method for batch embedding.

```rust
std::thread::spawn(move || {
    let Some(rt) = crate::runtime::shared_runtime() else {
        // error handling
        return;
    };
    
    rt.block_on(async move {
        let paths: Vec<&str> = image_paths.iter().map(|s| s.as_str()).collect();
        let result = vision_model
            .batch_embed_images(paths).await
            .map_err(|e| crate::memory::utils::error::Error::Other(format!("Failed to batch embed images: {}", e)));
        let _ = tx.send(result);
    });
});
```

## Why It's Wrong
- Same anti-pattern repeated for batch operations
- Thread overhead for async work
- Defeats async runtime efficiency

## Correct Approach
Use `tokio::spawn`:

```rust
tokio::spawn(async move {
    let paths: Vec<&str> = image_paths.iter().map(|s| s.as_str()).collect();
    let result = vision_model
        .batch_embed_images(paths).await
        .map_err(|e| crate::memory::utils::error::Error::Other(format!("Failed to batch embed images: {}", e)));
    let _ = tx.send(result);
});
```

## Impact
- Especially important for batch operations which may process many images
- Better scalability with tokio tasks
- Reduced OS thread pressure
