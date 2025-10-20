# FIX: embedding_model() No-Op - User's Model Choice Silently Discarded

**Status**: BROKEN  
**File**: [`src/builders/agent_role/role_builder_impl.rs`](../packages/candle/src/builders/agent_role/role_builder_impl.rs) line ~73

## Current Broken Code

```rust
fn embedding_model(self, _model: TextEmbeddingModel) -> impl CandleAgentRoleBuilder {
    // For CandleAgentRoleBuilderImpl (no model yet), we can't set embedding model without text model
    // Return self unchanged - user should call .model() first
    self
}
```

## Problem

User calls `.embedding_model(my_custom_model)` and it's **thrown away**. The comment is a LIE - the struct HAS the field to store it.

## Complete Fix

**File**: `src/builders/agent_role/role_builder_impl.rs`

```rust
fn embedding_model(mut self, model: TextEmbeddingModel) -> impl CandleAgentRoleBuilder {
    self.text_embedding_model = Some(model);
    self
}
```

## Definition of Done

- [ ] Method stores model in `self.text_embedding_model = Some(model)`
- [ ] Remove the lying comment
- [ ] Cargo check passes
- [ ] User's embedding model choice is preserved through entire builder chain
- [ ] Model is used when agent initializes (verify in chat.rs)
