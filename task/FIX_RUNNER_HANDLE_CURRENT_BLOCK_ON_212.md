# Fix Handle::current().block_on in runner.rs:212

**Location:** `/Volumes/samsung_t9/paraphym/packages/candle/src/cli/runner.rs:212`  
**Priority:** HIGH  
**Issue Type:** Handle::current().block_on() Antipattern

## Current Code (WRONG)

```rust
InputHandlerResult::Chat(message) => {
    // Resolve with smart input detection
    let resolved = tokio::runtime::Handle::current().block_on(async {
        resolve_smart_input(&message).await.unwrap_or(message.clone())
    });
    agent.chat(CandleChatLoop::UserPrompt(resolved))
}
```

## Problem

This code uses `Handle::current().block_on()` inside a closure that may be called from an async context:
- `Handle::current()` assumes there's a current tokio runtime context
- Will panic if no runtime context exists
- Can cause nested runtime issues if called from within async context
- The closure is passed to `on_conversation_turn` which may be invoked from various contexts

## Correct Solution

Use the shared runtime instead:

```rust
InputHandlerResult::Chat(message) => {
    // Resolve with smart input detection using shared runtime
    let resolved = match crate::runtime::shared_runtime() {
        Some(rt) => rt.block_on(async {
            resolve_smart_input(&message).await.unwrap_or(message.clone())
        }),
        None => {
            log::warn!("Shared runtime unavailable, using message as-is: {}", message);
            message.clone()
        }
    };
    agent.chat(CandleChatLoop::UserPrompt(resolved))
}
```

## Alternative: Make the Closure Async

If `on_conversation_turn` supports async closures, this could be refactored to avoid `block_on` entirely:

```rust
// Check if on_conversation_turn accepts async closures
// If yes, this would be even better:
InputHandlerResult::Chat(message) => {
    // Await directly without blocking
    let resolved = resolve_smart_input(&message).await.unwrap_or(message.clone());
    agent.chat(CandleChatLoop::UserPrompt(resolved))
}
```

**However**, this requires checking if the `on_conversation_turn` API supports async closures. If not, use the shared_runtime() pattern above.

## Why This Pattern is Correct

1. **Explicit Runtime Access**: `shared_runtime()` explicitly gets the runtime instead of assuming context
2. **Error Handling**: Gracefully handles runtime unavailability with fallback
3. **No Panic Risk**: Won't panic if called outside runtime context
4. **No Nested Runtime Issues**: Uses the shared runtime consistently
5. **Logging**: Warns when runtime unavailable so issues are visible

## Pattern Learned from cognitive_worker.rs Fix

Reference: `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/core/cognitive_worker.rs:272-328`

Key insights:
- Always use `shared_runtime()` instead of `Handle::current()`
- Provide graceful fallback when runtime unavailable
- Use log::warn or log::error for visibility

## Implementation Steps

1. **Read runner.rs** to understand the `on_conversation_turn` closure context
2. **Check if async closures are supported**: Look at `CandleAgentBuilder::on_conversation_turn` signature
3. **If async is supported**: Refactor to await directly (preferred)
4. **If async is NOT supported**: Replace with `shared_runtime().block_on()` pattern above
5. **Add log import** if not already present: `use log;`
6. **Test compilation**: `cargo check -p paraphym_candle`
7. **Test CLI interaction**: Run `cargo run --bin candle-chat` and verify smart input resolution works

## Related Issues

None directly related - this is a standalone `Handle::current().block_on()` issue.

## Notes

The current code comment says "Resolve with smart input detection" - the shared_runtime() approach maintains this functionality while being more robust.
