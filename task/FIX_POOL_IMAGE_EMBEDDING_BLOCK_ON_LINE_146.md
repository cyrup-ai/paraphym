# Fix Incorrect block_on Usage in image_embedding.rs Line 146

## Location
`packages/candle/src/pool/capabilities/image_embedding.rs:146`

## Problem
Worker thread using `block_on` for URL embedding.

```rust
let result = rt.block_on(model.embed_image_url(&req.url))
    .map_err(|e| PoolError::ModelError(e.to_string()));
let _ = req.response.send(result);
```

## Why It's Wrong
- Same anti-pattern as line 124
- Worker loop should be async, not sync with block_on
- URL operations are especially suited for async (I/O bound)
- Blocking thread while waiting for network I/O

## Correct Approach
Make worker async and use await:

```rust
let result = model.embed_image_url(&req.url).await
    .map_err(|e| PoolError::ModelError(e.to_string()));
let _ = req.response.send(result);
```

## Impact
- URL fetching is I/O bound - blocking defeats async benefits
- Should allow concurrent URL fetches across multiple requests
- Part of the same worker refactor as line 124
