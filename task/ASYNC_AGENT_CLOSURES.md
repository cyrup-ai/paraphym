# Task: Convert Agent Builder Closures to Async

## Status: NOT STARTED
## Priority: CRITICAL - Blocks CLI text output functionality
## Created: 2025-10-19

---

## Problem Statement

The `chat()` methods in the decomposed agent builder structure use **synchronous closures** but are called **inside async tokio tasks**, causing blocking I/O operations to block the async runtime.

Additionally, the CLI text output is not displaying because the CLI runner configures `.on_chunk()` handlers that expect async closures, but the underlying implementations still use sync signatures.

### Current State (BROKEN)

**Trait Definitions:**
- `src/builders/agent_role/traits.rs:104-107` - `CandleAgentRoleBuilder::chat()` - SYNC
- `src/builders/agent_role/traits.rs:207-210` - `CandleAgentBuilder::chat()` - SYNC

**Implementations:**
- `src/builders/agent_role/chat.rs` - `CandleAgentBuilder::chat()` impl - Uses SYNC handler
- `src/builders/agent_role/role_builder.rs` - `CandleAgentRoleBuilder::chat()` impl - Uses SYNC handler
- `src/builders/agent_role/stubs.rs` - Stub implementations - Use SYNC handlers

**Call Sites:**
- `src/cli/runner.rs:133-179` - Uses async closure pattern (won't compile with sync trait)
- `examples/fluent_builder.rs:69, 103, 131` - Uses async closures
- `examples/interactive_chat.rs:82` - Uses async closure

---

## The Solution: Full Async Conversion

Convert ALL `chat()` method signatures and implementations from sync to async closures.

### Required Changes

#### 1. Trait Signatures (2 traits to update)

**File:** `src/builders/agent_role/traits.rs`

Both traits need async signatures to support the role/agent override pattern.

**Change 1 - CandleAgentRoleBuilder::chat() (lines 103-107) - ROLE LEVEL**

```rust
// FROM (SYNC):
fn chat<F>(self, handler: F) -> Result<Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>, AgentError>
where
    F: FnOnce(&CandleAgentConversation) -> CandleChatLoop + Send + 'static;

// TO (ASYNC):
fn chat<F, Fut>(self, handler: F) -> Result<Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>, AgentError>
where
    F: Fn(&CandleAgentConversation) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = CandleChatLoop> + Send + 'static;
```

**Change 2 - CandleAgentBuilder::chat() (lines 207-210) - AGENT INSTANCE LEVEL**

```rust
// FROM (SYNC):
fn chat<F>(self, handler: F) -> Result<Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>, AgentError>
where
    F: FnOnce(&CandleAgentConversation) -> CandleChatLoop + Send + 'static;

// TO (ASYNC):
fn chat<F, Fut>(self, handler: F) -> Result<Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>, AgentError>
where
    F: Fn(&CandleAgentConversation) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = CandleChatLoop> + Send + 'static;
```

**Note:** Agent instance closures override role closures when both are configured.

#### 2. Implementation Files (3 files to update)

**Important:** The implementation must respect the override pattern:
- Role-level closures are stored in the builder state
- Agent-level closures override role-level when provided
- The runtime checks agent-level first, falls back to role-level

**File:** `src/builders/agent_role/chat.rs` - AGENT INSTANCE IMPLEMENTATION

Search for:
```rust
impl CandleAgentBuilder for CandleAgentBuilderImpl {
    fn chat<F>(self, handler: F) -> Result<...>
```

Update signature to:
```rust
fn chat<F, Fut>(self, handler: F) -> Result<...>
where
    F: Fn(&CandleAgentConversation) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = CandleChatLoop> + Send + 'static,
```

Find the handler invocation (should be around line 30-40 in the spawned task):
```rust
// FROM:
let chat_loop_result = handler(&conversation);

// TO:
let chat_loop_result = handler(&conversation).await;
```

**File:** `src/builders/agent_role/role_builder.rs` - ROLE TEMPLATE IMPLEMENTATION

Search for:
```rust
impl CandleAgentRoleBuilder for CandleAgentRoleBuilderImpl {
    fn chat<F>(self, handler: F) -> Result<...>
```

Update signature and add `.await` to handler invocation (same pattern as above).

**Note:** This provides the default/template chat behavior for the role. Agent instances can override it.

**File:** `src/builders/agent_role/stubs.rs`

Search for stub implementations of `chat()` methods. Update their signatures to match the async pattern.

Note: Stubs may just return error streams, so they might not call the handler, but the signature MUST match the trait.

#### 3. Helper Functions (if any)

**File:** `src/builders/agent_role/helpers.rs`

Check for any helper functions that create or wrap chat handlers. Update their signatures if needed.

---

## Implementation Checklist

### Phase 1: Trait Definitions
- [ ] Update `CandleAgentRoleBuilder::chat()` signature in `traits.rs` (lines 103-107)
- [ ] Update `CandleAgentBuilder::chat()` signature in `traits.rs` (lines 207-210)
- [ ] Update doc comments to reflect async closures (`|conversation| async { ... }`)

### Phase 2: Implementations
- [ ] Update `chat()` impl in `chat.rs` for `CandleAgentBuilder`
  - [ ] Update function signature
  - [ ] Add `.await` to `handler(&conversation)` call
- [ ] Update `chat()` impl in `role_builder.rs` for `CandleAgentRoleBuilder`
  - [ ] Update function signature  
  - [ ] Add `.await` to `handler(&conversation)` call
- [ ] Update stub implementations in `stubs.rs`
  - [ ] Update all `chat()` stub signatures

### Phase 3: Call Sites (Already Updated)
- [x] `src/cli/runner.rs` - Already uses async closures
- [x] `examples/fluent_builder.rs` - Already uses async closures
- [x] `examples/interactive_chat.rs` - Already uses async closures

### Phase 4: Verification
- [ ] Run `cargo check` - should compile without errors
- [ ] Run `cargo build --release` - should succeed
- [ ] Run `cargo run --release` - CLI should display text output
- [ ] Run `cargo run --example fluent_builder -- --query "test"` - should work
- [ ] Run `cargo run --example interactive_chat` - should work

---

## Definition of Done

### Core Implementation
- [ ] All 2 trait `chat()` signatures use `<F, Fut>` generic with `Fut: Future<Output = CandleChatLoop>`
- [ ] All implementation `chat()` methods have matching async signatures
- [ ] All handler invocations use `.await` (not sync calls)
- [ ] Doc comments updated to show `|conversation| async { ... }` syntax

### Verification
- [ ] `cargo check` passes
- [ ] `cargo build --release` succeeds
- [ ] CLI text output displays when running `cargo run --release`
- [ ] Fluent builder example works
- [ ] Interactive chat example works

### Testing
Run these verification commands:

```bash
cd /Volumes/samsung_t9/paraphym/packages/candle

# Compile check
cargo check

# Build release
cargo build --release

# Test CLI - should display text
cargo run --release
# Type: what's 7*7
# Expected: See text response streaming

# Test examples
cargo run --example fluent_builder -- --query "what is 2+2"
cargo run --example interactive_chat
```

---

## Technical Notes

### Architecture: Role vs Agent Closures

The builder pattern supports TWO levels of closure configuration:

1. **Agent Role Closures** - Template/default closures set at the role level
   - Configured via `CandleAgentRoleBuilder::chat()`, `.on_chunk()`, etc.
   - Provide defaults for all agents created from this role
   - Example: Default chat handler for all "helpful-assistant" agents

2. **Agent Instance Closures** - Overrides set on specific agent instances
   - Configured via `CandleAgentBuilder::chat()`, `.on_chunk()`, etc.
   - Override role-level defaults for this specific agent
   - Example: Custom chat handler for one specific agent instance

**Override Behavior:**
```rust
CandleFluentAi::agent_role("helpful")
    .on_chunk(|chunk| async { /* ROLE DEFAULT */ })  // Role-level default
    .into_agent()
    .on_chunk(|chunk| async { /* INSTANCE OVERRIDE */ })  // ← This wins!
    .chat(...)
```

**If agent instance closure is configured, it OVERRIDES the agent role closure.**

### Why Async Closures?

The `chat()` method creates a stream using `tokio::spawn`, which runs the handler inside an async task. The handler needs to:

1. Read from stdin using async I/O (`tokio::io::stdin()`)
2. Yield control to the runtime while waiting for input
3. Allow the stream consumer to receive chunks concurrently

Synchronous closures BLOCK the async task, preventing chunks from flowing to consumers.

### Pattern Explanation

```rust
// Async closure pattern:
.chat(|conversation| async move {
    // Can use .await here!
    let input = reader.read_line(&mut buf).await?;
    CandleChatLoop::UserPrompt(input)
})
```

The closure returns a `Future<Output = CandleChatLoop>`, which the implementation `.await`s.

### Usage Examples

**Example 1: Role-level default only**
```rust
CandleFluentAi::agent_role("helpful")
    .on_chunk(|chunk| async move {
        // This runs for ALL agents from this role
        println!("Role handler: {:?}", chunk);
        chunk
    })
    .into_agent()
    .chat(|_| async move { CandleChatLoop::UserPrompt("test".to_string()) })
```

**Example 2: Agent instance override**
```rust
CandleFluentAi::agent_role("helpful")
    .on_chunk(|chunk| async move {
        // Role default - will be OVERRIDDEN below
        println!("Role handler");
        chunk
    })
    .into_agent()
    .on_chunk(|chunk| async move {
        // Agent override - THIS ONE RUNS instead
        println!("Agent handler");
        chunk
    })
    .chat(|_| async move { CandleChatLoop::UserPrompt("test".to_string()) })
```

**Example 3: Multiple agents from same role, different overrides**
```rust
let role = CandleFluentAi::agent_role("helpful")
    .on_chunk(|chunk| async move {
        println!("Default chunk handler");
        chunk
    });

// Agent 1: Uses role default
let agent1 = role.clone().into_agent()
    .chat(...)

// Agent 2: Overrides with custom handler
let agent2 = role.clone().into_agent()
    .on_chunk(|chunk| async move {
        println!("Custom handler for agent 2");
        chunk
    })
    .chat(...)
```

### Critical Fix Context

This async conversion ALSO enables the CLI text output fix that was already applied to `src/cli/runner.rs` (the `.on_chunk()` handlers on lines 106-125). Those handlers won't work until the async signatures are in place.

---

## Related Context

- **Root Cause**: CLI not displaying text because chunks aren't flowing properly
- **Fix Applied**: Added `.on_chunk()` handlers to CLI runner (lines 106-125)
- **Blocker**: Async signatures needed for the on_chunk fix to compile and work
- **Previous Work**: This async conversion was done before but accidentally reverted when resolving a file conflict

---

## Priority Justification

**CRITICAL** because:

1. CLI text output is completely broken (no text displayed)
2. All interactive examples are broken
3. Core functionality is unusable
4. Blocks user testing and development
5. The fix is already partially applied (on_chunk handlers) but won't work until this is done

This must be completed for the CLI to function at all.
