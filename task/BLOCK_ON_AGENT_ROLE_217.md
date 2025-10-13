# Remove block_on from agent_role.rs:217 (HIGH)

**Location:** `src/builders/agent_role.rs:217`

**Priority:** HIGH - Inside ystream::spawn_stream, tool execution context

## Current Code

```rust
match crate::runtime::shared_runtime() {
    Some(runtime) => {
        match runtime.block_on(router.call_tool(&name, sweet_args)) {
            Ok(response) => {
                // Call tool result handler if configured
                if let Some(ref handler) = on_tool_result_handler {
                    let results = vec![format!("{:?}", response)];
                    handler(&results);
                }
                
                CandleMessageChunk::Text(format!(
                    "Tool '{}' executed: {:?}",
                    name, response
                ))
            }
            Err(e) => CandleMessageChunk::Error(format!(
                "Tool '{}' failed: {}",
                name, e
            )),
        }
    }
    None => CandleMessageChunk::Error(
        "Runtime unavailable".to_string(),
    ),
}
```

Context: Inside ystream::spawn_stream closure, handling tool calls from the model.

## Problem: Blocking Tool Execution

The code **eagerly blocks** to execute tools synchronously. This:
1. Blocks the thread during potentially long-running tool execution
2. Uses shared_runtime().block_on() risking nested runtime errors
3. Forces sync execution of async tool calls

## Solution: Use .await Inside AsyncStream

When refactoring the entire method to use `AsyncStream::with_channel(|sender| async move { ... })`:

```rust
// Use .await instead of block_on
match router.call_tool(&name, sweet_args).await {
    Ok(response) => {
        // Call tool result handler if configured
        if let Some(ref handler) = on_tool_result_handler {
            let results = vec![format!("{:?}", response)];
            handler(&results);
        }
        
        CandleMessageChunk::Text(format!(
            "Tool '{}' executed: {:?}",
            name, response
        ))
    }
    Err(e) => CandleMessageChunk::Error(format!(
        "Tool '{}' failed: {}",
        name, e
    )),
}
```

**Pattern Explanation:**
- **ANTIPATTERN (current):** `runtime.block_on(router.call_tool())` inside sync closure
- **CORRECT (fix):** `router.call_tool().await` inside async block

## Implementation Notes

1. Part of the same refactoring as BLOCK_ON_AGENT_ROLE_117.md
2. Remove `crate::runtime::shared_runtime()` wrapper entirely
3. Use `.await` directly on `router.call_tool()`
4. No longer need to handle "Runtime unavailable" case
5. Tool execution becomes properly async

## Dependencies

- Must be fixed with BLOCK_ON_AGENT_ROLE_117.md
- Part of larger ystream::spawn_stream → AsyncStream refactoring
## Reference Example

See `src/agent/builder.rs:345` for working example of `AsyncStream::with_channel` with `async move` block.
