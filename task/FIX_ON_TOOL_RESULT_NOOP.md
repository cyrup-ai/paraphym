# FIX: on_tool_result() No-Op - Tool Callbacks Never Fire

**Status**: BROKEN  
**File**: [`src/builders/agent_role/role_builder_impl.rs`](../packages/candle/src/builders/agent_role/role_builder_impl.rs) line ~187

## Current Broken Code

```rust
fn on_tool_result<F, Fut>(self, _handler: F) -> impl CandleAgentRoleBuilder
where
    F: Fn(&[String]) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    // Cannot set handler without a model - return self unchanged
    self
}
```

## Problem

User registers a callback for tool execution results and it's **thrown away**. The struct HAS the field to store it.

## Complete Fix

**File**: `src/builders/agent_role/role_builder_impl.rs`

```rust
fn on_tool_result<F, Fut>(mut self, handler: F) -> impl CandleAgentRoleBuilder
where
    F: Fn(&[String]) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    let wrapped = move |results: &[String]| Box::pin(handler(results)) as Pin<Box<dyn std::future::Future<Output = ()> + Send>>;
    self.on_tool_result_handler = Some(Arc::new(wrapped));
    self
}
```

**Reference**: Same pattern as `on_chunk` on line ~177 which WORKS.

## Definition of Done

- [ ] Method wraps handler in Arc and stores in `self.on_tool_result_handler`
- [ ] Cargo check passes
- [ ] Handler is called when tools execute in chat.rs
- [ ] Verify handler fires in the tool execution path around chat.rs line ~550-580
