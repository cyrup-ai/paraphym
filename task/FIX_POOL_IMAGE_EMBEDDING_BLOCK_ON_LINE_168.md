# Fix Incorrect block_on Usage in image_embedding.rs Line 168

## Location
`packages/candle/src/pool/capabilities/image_embedding.rs:168`

## Problem
Worker thread using `block_on` for base64 embedding.

```rust
let result = rt.block_on(model.embed_image_base64(&req.base64_data))
    .map_err(|e| PoolError::ModelError(e.to_string()));
let _ = req.response.send(result);
```

## Why It's Wrong
- Same anti-pattern as lines 124 and 146
- Worker should be async-native
- Base64 decoding + model inference should be async
- Blocks worker thread unnecessarily

## Correct Approach
Make worker async and use await:

```rust
let result = model.embed_image_base64(&req.base64_data).await
    .map_err(|e| PoolError::ModelError(e.to_string()));
let _ = req.response.send(result);
```

## Impact
- Part of comprehensive worker loop async refactor
- Allows proper concurrency for base64 decoding and embedding
- Consistent with other embedding methods
