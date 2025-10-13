# Fix Incorrect block_on Usage in image_embedding.rs Line 124

## Location
`packages/candle/src/pool/capabilities/image_embedding.rs:124`

## Problem
Worker thread using `block_on` to call async model methods.

```rust
let result = rt.block_on(model.embed_image(&req.image_path))
    .map_err(|e| PoolError::ModelError(e.to_string()));
let _ = req.response.send(result);
```

## Why It's Wrong
- Worker thread blocks on async operation instead of being async-native
- Creates sync/async impedance mismatch
- The entire worker loop should be async, not sync with block_on calls
- Defeats the purpose of having async trait methods

## Correct Approach

### Option 1: Make worker loop async
```rust
async fn image_embedding_worker<T: ImageEmbeddingCapable>(
    model: T,
    embed_image_rx: Receiver<EmbedImageRequest>,
    // ... other params
) {
    loop {
        select! {
            recv(embed_image_rx) -> req => {
                if let Ok(req) = req {
                    let result = model.embed_image(&req.image_path).await
                        .map_err(|e| PoolError::ModelError(e.to_string()));
                    let _ = req.response.send(result);
                }
            }
            // ... other channels
        }
    }
}
```

### Option 2: Spawn async tasks for processing
```rust
recv(embed_image_rx) -> req => {
    if let Ok(req) = req {
        let model_clone = model.clone();
        tokio::spawn(async move {
            let result = model_clone.embed_image(&req.image_path).await
                .map_err(|e| PoolError::ModelError(e.to_string()));
            let _ = req.response.send(result);
        });
    }
}
```

## Impact
- Critical: This is in a hot path for all image embedding operations
- Performance: Proper async will allow better concurrency
- Architecture: Pool workers should be fully async-native
