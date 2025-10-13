# Fix Incorrect block_on Usage in image_embedding.rs Line 191

## Location
`packages/candle/src/pool/capabilities/image_embedding.rs:191`

## Problem
Worker thread using `block_on` for batch embedding.

```rust
let paths: Vec<&str> = req.image_paths.iter().map(|s| s.as_str()).collect();
let result = rt.block_on(model.batch_embed_images(paths))
    .map_err(|e| PoolError::ModelError(e.to_string()));
let _ = req.response.send(result);
```

## Why It's Wrong
- Same anti-pattern as other worker methods
- **Especially critical for batch operations** which process multiple images
- Blocking thread during batch processing defeats concurrency
- Batch operations are prime candidates for async parallelism

## Correct Approach
Make worker async and use await:

```rust
let paths: Vec<&str> = req.image_paths.iter().map(|s| s.as_str()).collect();
let result = model.batch_embed_images(paths).await
    .map_err(|e| PoolError::ModelError(e.to_string()));
let _ = req.response.send(result);
```

## Impact
- **Critical for performance**: Batch operations process many images
- Proper async allows streaming results or concurrent processing
- Part of the worker loop async refactor
- Could enable parallel batch processing if refactored properly
