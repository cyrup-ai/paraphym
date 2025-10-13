# Remove block_on from agent_role.rs:117 (HIGH)

**Location:** `src/builders/agent_role.rs:117`

**Priority:** HIGH - Inside ystream::spawn_stream which should use AsyncStream with async blocks

## Current Code

```rust
AsyncStream::with_channel(move |_sender| {
    let _background_stream = ystream::spawn_stream(move |stream_sender| {
        // ...
        let mut router = crate::domain::tool::router::SweetMcpRouter::with_configs(
            plugin_configs,
            None,
        );

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
});
```

Context: This is inside `ystream::spawn_stream(move |stream_sender| { ... })` at line 76, which is a sync callback pattern that forces blocking.

## Problem: Eager Blocking Before Stream Execution

The code **eagerly blocks** to initialize the router inside the spawn_stream closure. This creates several issues:
1. Blocks the thread during stream setup
2. Uses shared_runtime().block_on() which can cause nested runtime errors if called from async context
3. Violates the lazy evaluation principle of streams

## Solution: Use AsyncStream with Async Block

Move the async initialization inside `AsyncStream::with_channel` which accepts async blocks natively:

```rust
AsyncStream::with_channel(move |sender| async move {
    // Build router config
    let reasoner_schema = crate::domain::agent::role::convert_serde_to_sweet_json(
        serde_json::json!({ /* ... */ }),
    );

    let default_plugin_config = crate::domain::tool::router::PluginConfig {
        tool_name: "mcp-reasoner".to_string(),
        wasm_path: "packages/sweetmcp/plugins/reasoner/target/wasm32-unknown-unknown/release/sweetmcp_plugin_reasoner.wasm".to_string(),
        description: "Advanced reasoning tool".to_string(),
        input_schema: reasoner_schema,
    };

    let plugin_configs = vec![default_plugin_config];
    let mut router = crate::domain::tool::router::SweetMcpRouter::with_configs(
        plugin_configs,
        None,
    );

    // Use .await instead of block_on
    if let Err(e) = router.initialize().await {
        let error_chunk = CandleMessageChunk::Error(format!(
            "Tool initialization failed: {}",
            e
        ));
        let _ = sender.send(error_chunk);
        return;
    }

    // Continue with rest of stream logic...
})
```

**Pattern Explanation:**
- **ANTIPATTERN (current):** Create sync ystream with `runtime.block_on(async_op())` inside
- **CORRECT (fix):** Use `AsyncStream::with_channel(|sender| async move { async_op().await })` directly

## Implementation Notes

1. Replace `ystream::spawn_stream` wrapper with direct `AsyncStream::with_channel`
2. Change closure to `async move` block
3. Replace all `runtime.block_on()` calls with `.await`
4. Remove shared_runtime() dependency - no longer needed
5. Update error handling to use `sender.send()` instead of `ystream::emit!` macro
6. This will require refactoring the entire `run_inference_cycle` method

## Impact

This change will eliminate:
- The nested ystream pattern (AsyncStream wrapping ystream::spawn_stream)
- All shared_runtime().block_on() calls in this inference cycle
- Potential for nested runtime errors

## Reference Example

See `src/agent/builder.rs:345` for working example of `AsyncStream::with_channel` with `async move`:

```rust
pub fn build(self) -> AsyncStream<super::agent::Agent<M>> {
    AsyncStream::with_channel(move |sender| async move {
        // Initialize tool router - uses .await, no block_on!
        if let Some(router) = self.tool_router.as_ref() {
            let mut mutable_router = SweetMcpRouter::new();
            if let Err(e) = mutable_router.initialize().await {
                log::warn!("Failed to initialize tool router during build: {}", e);
            }
        }
        // ... rest of async initialization ...
    })
}
```
