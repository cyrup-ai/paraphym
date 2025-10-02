# PANIC_2: Fix Runtime Initialization Panic in Chat

## OBJECTIVE
Replace panic-inducing `expect()` with proper error handling in the agent chat memory operation initialization.

## PRIORITY
🔴 CRITICAL - Must fix before production deployment

## BACKGROUND
The chat agent initialization code expects the Tokio runtime to be available and panics if it's not initialized. This creates a hard crash point instead of allowing graceful error recovery.

## SUBTASK 1: Replace expect() at Line 362
**File:** `packages/candle/src/domain/agent/chat.rs`  
**Line:** 362

**Current code:**
```rust
let runtime = shared_runtime()
    .expect("Tokio runtime not initialized - required for memory operations");
```

**Required replacement:**
```rust
let runtime = shared_runtime()
    .ok_or_else(|| anyhow::anyhow!("Tokio runtime not initialized - required for memory operations"))?;
```

## SUBTASK 2: Verify Error Propagation
**Action:** Ensure the containing function returns `Result` type and can propagate errors

**Requirements:**
- Function must return `Result<T, E>` where E is compatible with `anyhow::Error`
- Add function signature update if needed
- Verify `?` operator can be used (function returns Result)

## SUBTASK 3: Update Call Sites if Needed
**Action:** If the function signature changes, update all call sites to handle the new error type

**Verification:**
- Search for all calls to the modified function
- Ensure errors are properly handled or propagated
- No silently ignored errors

## DEFINITION OF DONE
- [ ] `expect()` call removed
- [ ] Replaced with `ok_or_else()` + `?` operator pattern
- [ ] Function returns Result type (if not already)
- [ ] All call sites handle errors appropriately
- [ ] Code compiles without warnings
- [ ] No panic paths in runtime initialization

## CONSTRAINTS
- ❌ DO NOT write unit tests
- ❌ DO NOT write integration tests
- ❌ DO NOT write benchmarks
- ✅ Focus solely on ./src modifications

## TECHNICAL NOTES
- Runtime initialization failure is rare but possible
- Proper error handling allows caller to decide recovery strategy
- Error message should remain descriptive for debugging
- Consider logging the error before propagating
