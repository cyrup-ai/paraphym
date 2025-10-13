# Remove block_on from agent_role.rs:145 (HIGH)

**Location:** `src/builders/agent_role.rs:145`

**Priority:** HIGH - Inside ystream::spawn_stream, part of same refactoring as line 117

## Current Code

```rust
// Add tools
if let Some(ref router) = tool_router {
    let mut all_tools: Vec<sweet_mcp_type::ToolInfo> = state.tools.clone().into();
    if let Some(runtime) = crate::runtime::shared_runtime() {
        let auto_generated_tools = runtime.block_on(router.get_available_tools());
        all_tools.extend(auto_generated_tools);
    }
    if !all_tools.is_empty() {
        params.tools = Some(ZeroOneOrMany::from(all_tools));
    }
}
```

Context: This is inside the same `ystream::spawn_stream` closure from line 76, after router initialization.

## Problem: Eager Blocking to Fetch Tools

The code **eagerly blocks** to fetch tools from the router before setting up completion parameters. This:
1. Forces synchronous execution of async tool fetching
2. Uses shared_runtime().block_on() creating nested runtime risk
3. Happens during stream setup instead of lazy stream execution

## Solution: Move Inside AsyncStream Async Block

When refactoring line 117 to use `AsyncStream::with_channel(|sender| async move { ... })`, this line should use `.await`:

```rust
// Add tools
if let Some(ref router) = tool_router {
    let mut all_tools: Vec<sweet_mcp_type::ToolInfo> = state.tools.clone().into();
    
    // Use .await instead of block_on
    let auto_generated_tools = router.get_available_tools().await;
    all_tools.extend(auto_generated_tools);
    
    if !all_tools.is_empty() {
        params.tools = Some(ZeroOneOrMany::from(all_tools));
    }
}
```

**Pattern Explanation:**
- **ANTIPATTERN (current):** `let tools = runtime.block_on(router.get_tools())`
- **CORRECT (fix):** `let tools = router.get_tools().await` (inside async block)

## Implementation Notes

1. This fix is **part of the same refactoring** as BLOCK_ON_AGENT_ROLE_117.md
2. When converting `run_inference_cycle` to use `AsyncStream::with_channel(|sender| async move { ... })`
3. Replace the `runtime.block_on()` call with direct `.await`
4. Remove the `if let Some(runtime)` wrapper - no longer needed
5. Consider error handling if `get_available_tools()` returns Result

## Dependencies

- Must be fixed together with BLOCK_ON_AGENT_ROLE_117.md
- Part of larger refactoring to remove ystream::spawn_stream wrapper
## Reference Example

See `src/agent/builder.rs:345` for working example of `AsyncStream::with_channel` with `async move` block.
