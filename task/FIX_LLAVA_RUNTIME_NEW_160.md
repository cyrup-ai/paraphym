# Fix Runtime Creation in llava.rs:160 (worker thread initialization)

**Location:** `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/vision/llava.rs:160`  
**Priority:** CRITICAL  
**Issue Type:** Runtime Creation Antipattern

## Current Code (WRONG)

```rust
// Step 6: Spawn dedicated thread with async runtime
thread::spawn(move || {
    // Create tokio runtime for async image operations
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            let _ = init_tx.send(Err(format!("Runtime creation failed: {}", e)));
            return;
        }
    };

    // ... model loading code ...

    // Signal successful initialization
    let _ = init_tx.send(Ok(()));

    // Start worker loop
    Self::worker_loop(
        model,
        tokenizer,
        request_rx,
        llava_config,
        device,
        image_size,
        image_mean,
        image_std,
        temperature,
        max_new_tokens,
        use_kv_cache,
        rt,  // <-- passes dedicated runtime
    );
});
```

## Problem

This code creates a **dedicated tokio runtime** for the LLaVA worker thread:
- `Runtime::new()` creates a separate runtime instead of using shared_runtime()
- Violates single-runtime architecture
- The runtime is passed to `worker_loop` which uses `rt.block_on()` at lines 249 and 268
- Creates unnecessary runtime proliferation

## Correct Solution

Use the shared runtime instead:

```rust
// Step 6: Spawn dedicated thread with shared async runtime
thread::spawn(move || {
    // Get shared runtime reference
    let Some(rt) = crate::runtime::shared_runtime() else {
        log::error!("Shared runtime unavailable for LLaVA worker initialization");
        let _ = init_tx.send(Err("Runtime unavailable".to_string()));
        return;
    };

    // ... model loading code (unchanged) ...

    // Signal successful initialization
    let _ = init_tx.send(Ok(()));

    // Start worker loop with shared runtime
    Self::worker_loop(
        model,
        tokenizer,
        request_rx,
        llava_config,
        device,
        image_size,
        image_mean,
        image_std,
        temperature,
        max_new_tokens,
        use_kv_cache,
        rt,  // <-- now uses shared runtime reference
    );
});
```

## Worker Loop Signature Change Required

The `worker_loop` function signature must also change:

**Before:**
```rust
fn worker_loop(
    // ... other params ...
    rt: tokio::runtime::Runtime,  // WRONG - owned runtime
)
```

**After:**
```rust
fn worker_loop(
    // ... other params ...
    rt: &'static tokio::runtime::Runtime,  // CORRECT - shared runtime reference
)
```

This affects lines 249 and 268 where `rt.block_on()` is called - the calls remain the same, but now use a reference to the shared runtime.

## Why This Pattern is Correct

1. **Single Runtime Architecture**: Uses the ONE shared runtime for all async operations
2. **Worker Thread Pattern**: Worker threads can hold a reference to the shared runtime
3. **Resource Efficiency**: No dedicated runtime overhead per LLaVA instance
4. **Error Handling**: Gracefully handles runtime unavailability during initialization
5. **Long-Lived Reference**: `&'static` lifetime is safe because shared_runtime lives for program lifetime

## Pattern Learned from cognitive_worker.rs Fix

Reference: `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/core/cognitive_worker.rs:272-328`

Key insight:
- Worker threads that need repeated async operations can hold a `&'static Runtime` reference
- Use `shared_runtime()` once at worker initialization
- Pass the reference through to worker loop
- `rt.block_on()` works the same way with a reference

## Implementation Steps

1. **Read llava.rs** to confirm current structure
2. **Replace lines 160-166** with shared_runtime() initialization
3. **Update worker_loop signature** (around line 230) to accept `&'static tokio::runtime::Runtime`
4. **Verify lines 249 and 268** still compile correctly (should be automatic)
5. **Add log import** if not already present: `use log;`
6. **Test compilation**: `cargo check -p paraphym_candle`
7. **Verify runtime behavior** with LLaVA vision tests

## Related Issues

- Line 249: `rt.block_on()` in Ask handler - will use shared runtime after this fix
- Line 268: `rt.block_on()` in AskUrl handler - will use shared runtime after this fix

Both block_on calls are correct once the runtime parameter is changed to `&'static Runtime`.
