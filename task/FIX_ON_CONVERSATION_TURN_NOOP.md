# FIX: on_conversation_turn() No-Op - Conversation Control Lost

**Status**: BROKEN  
**File**: [`src/builders/agent_role/role_builder_impl.rs`](../packages/candle/src/builders/agent_role/role_builder_impl.rs) line ~197

## Current Broken Code

```rust
fn on_conversation_turn<F, Fut>(self, _handler: F) -> impl CandleAgentRoleBuilder
where
    F: Fn(&CandleAgentConversation, &CandleAgentRoleAgent) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>> + Send + 'static,
{
    // Cannot set handler without a model - return self unchanged
    self
}
```

## Problem

User registers a callback for conversation flow control and it's **thrown away**. The struct HAS the field to store it.

## Complete Fix

**File**: `src/builders/agent_role/role_builder_impl.rs`

```rust
fn on_conversation_turn<F, Fut>(mut self, handler: F) -> impl CandleAgentRoleBuilder
where
    F: Fn(&CandleAgentConversation, &CandleAgentRoleAgent) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>> + Send + 'static,
{
    let wrapped_handler = move |conv: &CandleAgentConversation, agent: &CandleAgentRoleAgent| {
        Box::pin(handler(conv, agent)) as Pin<Box<dyn std::future::Future<Output = Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>> + Send>>
    };
    self.on_conversation_turn_handler = Some(Arc::new(wrapped_handler));
    self
}
```

**Reference**: Working implementation in `agent_builder.rs` line ~178.

## Definition of Done

- [ ] Method wraps handler in Arc and stores in `self.on_conversation_turn_handler`
- [ ] Cargo check passes
- [ ] Handler is preserved through builder chain
- [ ] Verify handler can be invoked in conversation flow (if implemented)
