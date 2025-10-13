# Remove block_on from agent_role.rs:1586 (HIGH)

**Location:** `src/builders/agent_role.rs:1586`

**Priority:** HIGH - Inside ystream::spawn_stream, part of same refactoring as 1514

## Current Code

```rust
// Combine builder tools with auto-generated tools
if let Some(ref router) = tool_router {
    let mut all_tools: Vec<ToolInfo> = tools.clone().into();

    // Try to get auto-generated tools if runtime is available
    if let Some(runtime) = crate::runtime::shared_runtime() {
        let auto_generated_tools =
            runtime.block_on(router.get_available_tools());
        all_tools.extend(auto_generated_tools);
    }

    if !all_tools.is_empty() {
        // Pass merged tools to completion system for function calling
        params.tools = Some(ZeroOneOrMany::from(all_tools));
    }
}
```

Context: Inside ystream::spawn_stream at line 1438, after router initialization and memory search.

## Problem: Eager Blocking to Fetch Tools

The code **eagerly blocks** to fetch available tools from the router. This:
1. Forces synchronous execution of async tool fetching
2. Uses shared_runtime().block_on() risking nested runtime errors
3. Inside ystream sync closure instead of AsyncStream async block

## Solution: Use .await Inside AsyncStream

When refactoring to use `AsyncStream::with_channel(|sender| async move { ... })`:

```rust
// Inside AsyncStream::with_channel(|sender| async move { ... })

// Combine builder tools with auto-generated tools
if let Some(ref router) = tool_router {
    let mut all_tools: Vec<ToolInfo> = tools.clone().into();

    // Use .await instead of block_on
    let auto_generated_tools = router.get_available_tools().await;
    all_tools.extend(auto_generated_tools);

    if !all_tools.is_empty() {
        // Pass merged tools to completion system for function calling
        params.tools = Some(ZeroOneOrMany::from(all_tools));
    }
}
```

**Pattern Explanation:**
- **ANTIPATTERN (current):** `let tools = runtime.block_on(router.get_tools())`
- **CORRECT (fix):** `let tools = router.get_available_tools().await` (inside async block)

## Implementation Notes

1. **Part of same refactoring** as BLOCK_ON_AGENT_ROLE_1514.md and BLOCK_ON_AGENT_ROLE_1535.md
2. Remove `if let Some(runtime)` wrapper - no longer needed
3. Use direct `.await` on `get_available_tools()`
4. Consider error handling if the method returns Result
5. This is identical pattern to BLOCK_ON_AGENT_ROLE_145.md but in different code path

## Dependencies

- Must be fixed with BLOCK_ON_AGENT_ROLE_1514.md
- Part of ystream → AsyncStream refactoring at line 1438
- Related to BLOCK_ON_AGENT_ROLE_1535.md and BLOCK_ON_AGENT_ROLE_1673.md
## Reference Example

See `src/agent/builder.rs:345` for working example of `AsyncStream::with_channel` with `async move` block.
