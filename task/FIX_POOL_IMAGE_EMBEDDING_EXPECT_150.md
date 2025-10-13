# Fix Runtime .expect() in pool/image_embedding.rs:150 (embed_image_base64 handler)

**Location:** `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/image_embedding.rs:148-150`  
**Priority:** HIGH  
**Issue Type:** Panic-on-Error with .expect() instead of graceful handling

## Current Code (WRONG)

```rust
recv(embed_image_base64_rx) -> req => {
    if let Ok(req) = req {
        // Transition: Ready/Idle → Processing
        state.store(WorkerState::Processing as u32, std::sync::atomic::Ordering::Release);

        let rt = crate::runtime::shared_runtime()
            .expect("Shared runtime required for async operations");
        let result = rt.block_on(model.embed_image_base64(&req.base64_data))
            .map_err(|e| PoolError::ModelError(e.to_string()));
        let _ = req.response.send(result);

        // Transition: Processing → Ready
        state.store(WorkerState::Ready as u32, std::sync::atomic::Ordering::Release);
        last_activity = SystemTime::now();
    }
}
```

## Problem

This code uses `.expect()` on `shared_runtime()`:
- **Will panic** if runtime is unavailable, crashing the worker thread
- No graceful error handling - violates pool resilience
- Should send error response to caller instead of panicking
- Doesn't transition state back to Ready on failure
- **Critical for API usage**: base64 embedding is commonly used in HTTP APIs

## Correct Solution

Use graceful error handling with early continue:

```rust
recv(embed_image_base64_rx) -> req => {
    if let Ok(req) = req {
        // Transition: Ready/Idle → Processing
        state.store(WorkerState::Processing as u32, std::sync::atomic::Ordering::Release);

        let Some(rt) = crate::runtime::shared_runtime() else {
            log::error!("Shared runtime unavailable for base64 image embedding");
            let _ = req.response.send(Err(PoolError::RuntimeUnavailable));
            state.store(WorkerState::Ready as u32, std::sync::atomic::Ordering::Release);
            last_activity = SystemTime::now();
            continue;
        };

        let result = rt.block_on(model.embed_image_base64(&req.base64_data))
            .map_err(|e| PoolError::ModelError(e.to_string()));
        let _ = req.response.send(result);

        // Transition: Processing → Ready
        state.store(WorkerState::Ready as u32, std::sync::atomic::Ordering::Release);
        last_activity = SystemTime::now();
    }
}
```

## Additional Change Required

Add `RuntimeUnavailable` variant to `PoolError` enum if it doesn't exist (same as line 118 fix).

## Why This Pattern is Correct

1. **No Panic**: Uses `let Some(...) else` pattern instead of `.expect()`
2. **API Resilience**: Critical for HTTP APIs that send base64 images
3. **Graceful Error**: Sends error response to caller via channel
4. **State Management**: Transitions back to Ready even on error
5. **Activity Tracking**: Updates last_activity to keep worker alive
6. **Worker Resilience**: Worker continues running and can handle next request
7. **Logging**: Error is logged for observability

## Pattern Learned from cognitive_worker.rs Fix

Reference: 
- `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/core/cognitive_worker.rs:272-328`
- `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/agent/chat.rs:443-454` (already correct)

Key insight:
- Never use `.expect()` or `.unwrap()` on `shared_runtime()`
- Always handle `None` case gracefully
- Send error to caller instead of panicking

## Implementation Steps

1. **Ensure RuntimeUnavailable variant exists** (from line 118 fix)
2. **Replace lines 148-150** with graceful error handling pattern
3. **Verify continue** keyword is correct for crossbeam select! macro
4. **Add log import** if not already present: `use log;`
5. **Test compilation**: `cargo check -p paraphym_candle`
6. **Test pool behavior**: Verify worker handles base64 embedding errors gracefully

## Related Issues

- Line 118: Same .expect() in embed_image handler - [FIX_POOL_IMAGE_EMBEDDING_EXPECT_118.md]
- Line 134: Same .expect() in embed_image_url handler - [FIX_POOL_IMAGE_EMBEDDING_EXPECT_134.md]
- Line 167: Same .expect() in batch_embed_images handler - [FIX_POOL_IMAGE_EMBEDDING_EXPECT_167.md]

All four must be fixed with the same graceful error handling pattern.
