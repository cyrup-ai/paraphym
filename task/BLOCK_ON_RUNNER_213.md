# Remove block_on from runner.rs:213 (CRITICAL)

**Location:** `src/cli/runner.rs:213`

**Priority:** CRITICAL - This is already inside an async context (line 228 has `.await`)

## Current Code

```rust
InputHandlerResult::Chat(message) => {
    // Resolve with smart input detection
    let resolved = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            resolve_smart_input(&message).await.unwrap_or(message)
        })
    });
    agent.chat(CandleChatLoop::UserPrompt(resolved))
}
```

Context: Line 228 shows `while let Some(chunk) = stream.next().await {` proving this entire function is already async.

## Problem: Nested Runtime Antipattern

This code is **eagerly blocking** inside an async context to call another async function. This causes the nested runtime error:
```
Cannot start a runtime from within a runtime
```

The code uses `Handle::current().block_on()` which tries to block on the current runtime from within itself - a classic nested runtime error.

## Solution: Just Use .await

Since we're already in an async context (proven by line 228), we should simply `.await` the async operation directly:

```rust
InputHandlerResult::Chat(message) => {
    // Resolve with smart input detection
    let resolved = resolve_smart_input(&message).await.unwrap_or(message);
    agent.chat(CandleChatLoop::UserPrompt(resolved))
}
```

**Pattern Explanation:**
- **ANTIPATTERN (current):** `let x = block_on(async_fn()); /* inside async context */`
- **CORRECT (fix):** `let x = async_fn().await; /* inside async context */`

This is the simplest fix - no stream refactoring needed, just remove the unnecessary blocking wrapper.

## Implementation Notes

1. Remove the entire `tokio::task::block_in_place` wrapper
2. Remove the `Handle::current().block_on` wrapper
3. Simply `.await` the `resolve_smart_input` call directly
4. This function is clearly async-capable since it already uses `.await` at line 228

## Testing

After fix, run the phi4_reasoning_example to verify the nested runtime error is resolved.
