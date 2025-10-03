# MEMUNWP_1: Remove Unsafe Unwrap in Memory Context

## OBJECTIVE
Replace unsafe unwrap with explicit error handling to prevent production panics.

## LOCATION
`packages/candle/src/domain/agent/chat.rs:420`

## SUBTASK 1: Remove dangerous assumption comment
- Remove comment about "in practice" guarantees
- Remove comment claiming unwrap is safe
- Line 420

## SUBTASK 2: Replace unwrap with error handling
```rust
pub fn inject_memory_context(...) -> Result<ystream::AsyncStream<ContextInjectionResult>> {
    // ... in the retrieval logic:
    let result = if results.len() == 1 {
        results.into_iter().next()
            .ok_or_else(|| AgentError::InternalError(
                "Vector with length 1 contained no elements - impossible state".to_string()
            ))?
    } else {
        // Handle multiple or zero results
    };
}
```

## SUBTASK 3: Add debug assertions
- Add `debug_assert_eq!(results.len(), 1)` before unwrap sites
- Provides early detection in debug builds
- No runtime cost in release builds

## SUBTASK 4: Audit all unwrap() calls
- Search for all `.unwrap()` in inject_memory_context
- Replace with proper error propagation using `?`
- Ensure all error paths return AgentError

## DEFINITION OF DONE
- No `.unwrap()` calls in inject_memory_context
- All errors properly propagated
- Debug assertions added
- Code compiles without warnings

## RESEARCH NOTES
- AgentError enum definition
- Error handling patterns in agent/chat.rs
- Debug assertions: std::debug_assert

## CONSTRAINTS
- Do NOT write unit tests
- Do NOT write integration tests
- Do NOT write benchmarks
- Focus solely on src modification
