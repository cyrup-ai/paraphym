# Race Condition: pending_requests Counter Leak on Channel Error

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs`

Lines 369-396 (embed_text) and 453-480 (batch_embed_text)

## Severity
**HIGH** - Memory leak and incorrect load balancing

## Issue Description

When the response channel is closed (worker died), the counter is never decremented. The error path at line 394 returns early:

```rust
// Line 369
worker.core.pending_requests.fetch_add(1, Ordering::Release);

// Line 394 - Early return on channel error
.map_err(|_| PoolError::RecvError("Response channel closed".to_string()))?;

// Line 396 - NEVER REACHED
worker.core.pending_requests.fetch_sub(1, Ordering::Release);
```

## Impact

Same as TEXT_EMBED_RACE_1:
1. Counter permanently incremented
2. Load balancing broken
3. Worker appears busy forever
4. Dead workers still counted as having pending work

## Additional Concern

This is particularly problematic because:
- It happens when workers die unexpectedly
- Dead workers will accumulate phantom pending requests
- The pool's `validate_workers()` may not clean this up properly

## Fix Required

Same RAII guard solution as TEXT_EMBED_RACE_1, or explicit decrement in both error paths.

## Related Files

Check if other capability modules have the same issue:
- `text_to_text.rs`
- `image_embedding.rs`
- `vision.rs`
- `text_to_image.rs`
