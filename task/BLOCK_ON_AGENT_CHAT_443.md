# Remove block_on from agent/chat.rs:443 (CRITICAL)

**Location:** `src/domain/agent/chat.rs:443`

**Priority:** CRITICAL - Uses Handle::current().block_on() pattern

## Current Code

```rust
// Receive results with configurable timeout
let timeout_ms = Self::get_memory_timeout_ms(timeout_ms);

let retrieval_results = tokio::task::block_in_place(|| {
    tokio::runtime::Handle::current().block_on(async {
        let timeout_duration = std::time::Duration::from_millis(timeout_ms);
        
        match tokio::time::timeout(timeout_duration, retrieval_rx.recv()).await {
            Ok(Some(Ok(results))) => results,
            Ok(Some(Err(e))) => {
                log::error!("Memory retrieval failed: {e}");
                Vec::new()
            }
            Ok(None) => {
                log::error!("Memory retrieval channel closed unexpectedly");
                Vec::new()
            }
            Err(_) => {
                log::warn!(
                    "Memory retrieval timed out - context may be incomplete (timeout_ms: {timeout_ms}, message: {message:?})"
                );
                Vec::new()
            }
        }
    })
});

let memory_nodes_used = retrieval_results.len();
```

Context: Inside memory retrieval logic, receiving from channel with timeout.

## Problem: Nested Runtime Error

This code uses **`Handle::current().block_on()`** which tries to block on the current runtime from within itself. This:
1. Will cause "Cannot start a runtime from within a runtime" error if called from async context
2. Uses block_in_place + Handle::current().block_on() antipattern
3. Unnecessarily complex for receiving from a channel with timeout

## Solution: Check Context and Use Appropriate Pattern

Need to determine if this method is sync or async:

### If this is inside a sync function:

Use shared_runtime instead of Handle::current():

```rust
let timeout_ms = Self::get_memory_timeout_ms(timeout_ms);
let timeout_duration = std::time::Duration::from_millis(timeout_ms);

let runtime = crate::runtime::shared_runtime()
    .ok_or_else(|| /* error */)?;

let retrieval_results = runtime.block_on(async {
    match tokio::time::timeout(timeout_duration, retrieval_rx.recv()).await {
        Ok(Some(Ok(results))) => results,
        Ok(Some(Err(e))) => {
            log::error!("Memory retrieval failed: {e}");
            Vec::new()
        }
        Ok(None) => {
            log::error!("Memory retrieval channel closed unexpectedly");
            Vec::new()
        }
        Err(_) => {
            log::warn!(
                "Memory retrieval timed out - context may be incomplete (timeout_ms: {timeout_ms})"
            );
            Vec::new()
        }
    }
});
```

### If this is inside an async function (BETTER):

Just use .await directly:

```rust
let timeout_ms = Self::get_memory_timeout_ms(timeout_ms);
let timeout_duration = std::time::Duration::from_millis(timeout_ms);

let retrieval_results = match tokio::time::timeout(timeout_duration, retrieval_rx.recv()).await {
    Ok(Some(Ok(results))) => results,
    Ok(Some(Err(e))) => {
        log::error!("Memory retrieval failed: {e}");
        Vec::new()
    }
    Ok(None) => {
        log::error!("Memory retrieval channel closed unexpectedly");
        Vec::new()
    }
    Err(_) => {
        log::warn!(
            "Memory retrieval timed out - context may be incomplete (timeout_ms: {timeout_ms})"
        );
        Vec::new()
    }
};
```

**Pattern Explanation:**
- **ANTIPATTERN (current):** `block_in_place(|| Handle::current().block_on(async { recv().await }))`
- **IF SYNC (fix):** `shared_runtime().block_on(async { recv().await })`
- **IF ASYNC (fix):** `recv().await` (no blocking at all)

## Implementation Notes

1. **First, check if the containing function is async or sync**
2. Read the function signature around line 443
3. If async, remove ALL blocking wrappers and use .await
4. If sync, replace Handle::current() with shared_runtime()
5. Simplify the code - no need for block_in_place either way

## Investigation Needed

Need to see the function signature to determine proper fix. Read around line 430-450 to see the containing function.
