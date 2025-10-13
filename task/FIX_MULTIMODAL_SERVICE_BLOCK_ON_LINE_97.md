# Fix Incorrect block_on Usage in multimodal_service.rs Line 97

## Location
`packages/candle/src/memory/vector/multimodal_service.rs:97`

## Problem
Spawning `std::thread` just to call `block_on` on an async method. This is an anti-pattern.

```rust
std::thread::spawn(move || {
    let Some(rt) = crate::runtime::shared_runtime() else {
        // error handling
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

## Why It's Wrong
- Spawns OS thread unnecessarily
- Uses `block_on` to bridge sync/async instead of using tokio::spawn
- Defeats the purpose of async/await
- Creates thread overhead for every embedding request

## Correct Approach
Use `tokio::spawn` directly since we're already in an async context:

```rust
tokio::spawn(async move {
    let result = vision_model
        .embed_image(&image_path).await
        .map_err(|e| crate::memory::utils::error::Error::Other(format!("Failed to embed image: {}", e)));
    let _ = tx.send(result);
});
```

## Impact
- Performance: Reduces thread creation overhead
- Correctness: Properly uses async runtime
- Maintainability: Clearer async code flow
