# Remove block_on from agent_role.rs:1514 (HIGH)

**Location:** `src/builders/agent_role.rs:1514`

**Priority:** HIGH - Inside ystream::spawn_stream, duplicate of line 117 pattern

## Current Code

```rust
// Inside ystream::spawn_stream at line 1438
let _background_stream = ystream::spawn_stream(move |stream_sender| {
    // ...
    let mut router = SweetMcpRouter::with_configs(plugin_configs, None);

    match runtime.block_on(router.initialize()) {
        Ok(()) => Some(router),
        Err(e) => {
            let error_chunk = CandleMessageChunk::Error(format!(
                "Tool initialization failed: {}",
                e
            ));
            ystream::emit!(stream_sender, error_chunk);
            return;
        }
    }
});
```

Context: Inside ystream::spawn_stream closure at line 1438, initializing tool router.

## Problem: Eager Blocking Inside Sync Closure

The code **eagerly blocks** to initialize the router inside ystream::spawn_stream. This:
1. Forces synchronous execution in a sync closure
2. Uses shared_runtime().block_on() risking nested runtime errors
3. Violates lazy async pattern - should use AsyncStream with async blocks

## Solution: Replace ystream with AsyncStream

The parent stream (starting around line 1420) should use `AsyncStream::with_channel` instead of the nested ystream pattern:

```rust
// In chat() method, instead of:
// let _background_stream = ystream::spawn_stream(move |stream_sender| { ... });

// Use AsyncStream directly:
AsyncStream::with_channel(move |sender| async move {
    // Build router config
    let reasoner_schema = crate::domain::agent::role::convert_serde_to_sweet_json(
        serde_json::json!({ /* ... */ }),
    );

    let default_plugin_config = PluginConfig {
        tool_name: "mcp-reasoner".to_string(),
        wasm_path: "packages/sweetmcp/plugins/reasoner/target/wasm32-unknown-unknown/release/sweetmcp_plugin_reasoner.wasm".to_string(),
        description: "Advanced reasoning tool with Beam Search and MCTS strategies".to_string(),
        input_schema: reasoner_schema,
    };

    let plugin_configs = vec![default_plugin_config];
    let mut router = SweetMcpRouter::with_configs(plugin_configs, None);

    // Use .await instead of block_on
    if let Err(e) = router.initialize().await {
        let error_chunk = CandleMessageChunk::Error(format!(
            "Tool initialization failed: {}",
            e
        ));
        let _ = sender.send(error_chunk);
        return;
    }

    // Continue with rest of inference logic...
})
```

**Pattern Explanation:**
- **ANTIPATTERN (current):** `ystream::spawn_stream(|sender| { runtime.block_on(async_op()) })`
- **CORRECT (fix):** `AsyncStream::with_channel(|sender| async move { async_op().await })`

## Implementation Notes

1. This is **similar to BLOCK_ON_AGENT_ROLE_117** but in a different code path
2. Line 1438's ystream::spawn_stream should be replaced with AsyncStream::with_channel
3. Remove all shared_runtime().block_on() calls (lines 1514, 1535, 1586, 1673)
4. Replace with direct `.await` calls
5. Change `ystream::emit!` to `sender.send()`
6. This is a substantial refactoring of the second inference path

## Related Tasks

- BLOCK_ON_AGENT_ROLE_1535.md - Memory search (same ystream)
- BLOCK_ON_AGENT_ROLE_1586.md - Tool fetching (same ystream)
- BLOCK_ON_AGENT_ROLE_1673.md - Tool execution (same ystream)

All four must be fixed together as part of one refactoring.
## Reference Example

See `src/agent/builder.rs:345` for working example of `AsyncStream::with_channel` with `async move` block.
