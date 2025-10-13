# Remove block_on from agent_role.rs:1673 (HIGH)

**Location:** `src/builders/agent_role.rs:1673`

**Priority:** HIGH - Inside ystream::spawn_stream, tool execution context

## Current Code

```rust
// Execute the tool if runtime is available
match crate::runtime::shared_runtime() {
    Some(runtime) => {
        match runtime.block_on(
            router.call_tool(&name, sweet_args),
        ) {
            Ok(response) => {
                // Call tool result handler if configured (zero allocation for None)
                if let Some(ref handler) = on_tool_result_handler {
                    let results = vec![format!("{:?}", response)];
                    handler(&results);
                }
                
                // Convert response to text result
                let result_text = format!(
                    "Tool '{}' executed successfully: {:?}",
                    name, response
                );
                CandleMessageChunk::Text(result_text)
            }
            Err(e) => {
                // Return error as text
                CandleMessageChunk::Error(format!(
                    "Tool '{}' execution failed: {}",
                    name, e
                ))
            }
        }
    }
    None => CandleMessageChunk::Error(
        "Runtime unavailable for tool execution".to_string(),
    ),
}
```

Context: Inside ystream::spawn_stream at line 1438, handling tool calls from completion.

## Problem: Blocking Tool Execution

The code **eagerly blocks** to execute tools synchronously. This:
1. Blocks the thread during potentially long-running tool execution
2. Uses shared_runtime().block_on() risking nested runtime errors
3. Forces sync execution of async tool calls
4. Inside ystream sync closure instead of AsyncStream async block

## Solution: Use .await Inside AsyncStream

When refactoring to use `AsyncStream::with_channel(|sender| async move { ... })`:

```rust
// Inside AsyncStream::with_channel(|sender| async move { ... })

// Use .await instead of block_on
match router.call_tool(&name, sweet_args).await {
    Ok(response) => {
        // Call tool result handler if configured
        if let Some(ref handler) = on_tool_result_handler {
            let results = vec![format!("{:?}", response)];
            handler(&results);
        }
        
        // Convert response to text result
        let result_text = format!(
            "Tool '{}' executed successfully: {:?}",
            name, response
        );
        CandleMessageChunk::Text(result_text)
    }
    Err(e) => {
        // Return error as text
        CandleMessageChunk::Error(format!(
            "Tool '{}' execution failed: {}",
            name, e
        ))
    }
}
```

**Pattern Explanation:**
- **ANTIPATTERN (current):** `runtime.block_on(router.call_tool())` inside sync closure
- **CORRECT (fix):** `router.call_tool().await` inside async block

## Implementation Notes

1. **Part of same refactoring** as BLOCK_ON_AGENT_ROLE_1514.md
2. Remove `crate::runtime::shared_runtime()` wrapper entirely
3. Use `.await` directly on `router.call_tool()`
4. No longer need to handle "Runtime unavailable" case
5. Tool execution becomes properly async
6. This is identical pattern to BLOCK_ON_AGENT_ROLE_217.md but in different code path

## Dependencies

- Must be fixed with BLOCK_ON_AGENT_ROLE_1514.md
- Part of ystream → AsyncStream refactoring at line 1438
- Related to BLOCK_ON_AGENT_ROLE_1535.md and BLOCK_ON_AGENT_ROLE_1586.md

All four block_on calls (lines 1514, 1535, 1586, 1673) must be eliminated together.
## Reference Example

See `src/agent/builder.rs:345` for working example of `AsyncStream::with_channel` with `async move` block.
