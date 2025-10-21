# BUILDER_1: Remove Agent Builder Stub Comments

## OBJECTIVE

Clean up "for now" stub comments in agent builder by implementing proper extensibility patterns for stop sequences.

## BACKGROUND

The agent builder has empty stop sequences with a "for now" comment suggesting future extension. This needs proper API design rather than a stub comment.

## SUBTASK 1: Add Stop Sequences Field to Builder Structs

**Location:** `packages/candle/src/builders/agent_role/agent_builder.rs:25-43` and `packages/candle/src/builders/agent_role/role_builder.rs:6-25`

**Current State:**
```rust
pub struct CandleAgentBuilderImpl {
    // ... existing fields
    pub(super) conversation_history: ZeroOneOrMany<(CandleMessageRole, String)>,
}

pub struct CandleAgentRoleBuilderImpl {
    // ... existing fields  
    pub(super) conversation_history: ZeroOneOrMany<(CandleMessageRole, String)>,
}
```

**Required Changes:**
- Add `pub(super) stop_sequences: Vec<String>` field to both structs
- Initialize with `Vec::new()` in constructors for backward compatibility
- Field allows storing user-configured stop sequences for model configuration

## SUBTASK 2: Add Stop Sequences Methods to Traits

**Location:** `packages/candle/src/builders/agent_role/traits.rs:21-31`

**Current State:**
```rust
/// Set max tokens - EXACT syntax: .max_tokens(8000)
#[must_use]
fn max_tokens(self, max: u64) -> impl CandleAgentRoleBuilder;

/// Set memory read timeout in milliseconds - EXACT syntax: .memory_read_timeout(5000)
#[must_use]
fn memory_read_timeout(self, timeout_ms: u64) -> impl CandleAgentRoleBuilder;
```

**Required Changes:**
- Add `stop_sequences(self, sequences: Vec<String>) -> impl CandleAgentRoleBuilder` method
- Add `add_stop_sequence(self, sequence: impl Into<String>) -> impl CandleAgentRoleBuilder` method
- Both methods marked with `#[must_use]` for fluent chaining
- Methods provide clean API for configuring stop sequences without stub comments

## SUBTASK 3: Implement Stop Sequences Methods

**Location:** `packages/candle/src/builders/agent_role/agent_builder.rs:95-105` and `packages/candle/src/builders/agent_role/role_builder_impl.rs:95-105`

**Required Changes:**
- Implement `stop_sequences` method that replaces entire stop sequences vector
- Implement `add_stop_sequence` method that appends single sequence to existing list
- Both implementations follow fluent builder pattern returning `self`
- Methods mutate the `stop_sequences` field directly

**Core Pattern:**
```rust
/// Set stop sequences - EXACT syntax: .stop_sequences(vec!["\n\n".to_string(), "###".to_string()])
fn stop_sequences(mut self, sequences: Vec<String>) -> impl CandleAgentRoleBuilder {
    self.stop_sequences = sequences;
    self
}

/// Add single stop sequence - EXACT syntax: .add_stop_sequence("\n\n")
fn add_stop_sequence(mut self, sequence: impl Into<String>) -> impl CandleAgentRoleBuilder {
    self.stop_sequences.push(sequence.into());
    self
}
```

## SUBTASK 4: Wire Stop Sequences to Model Configuration

**Location:** `packages/candle/src/builders/agent_role/agent_builder.rs:276`

**Current State:**
```rust
// Stop sequences (empty for now, could be extended)
stop_sequences: Vec::new(),
```

**Required Changes:**
- Remove stub comment entirely
- Replace `Vec::new()` with `self.stop_sequences.clone()`
- Stop sequences now properly flow from builder configuration to model config
- Maintains backward compatibility (empty vec is valid default)

## DEFINITION OF DONE

- [ ] No "for now" or "could be extended" comments in code
- [ ] Builder API includes `stop_sequences()` and `add_stop_sequence()` methods  
- [ ] Stop sequences properly passed to completion providers
- [ ] Documentation explains stop sequence usage with examples
- [ ] Empty default remains valid (no breaking changes)

## RESEARCH NOTES

### Stop Sequences Purpose
- Tell model when to stop generating text
- Common examples: "\n\n", "Human:", "###", custom delimiters
- Provider-specific behavior may vary

### Integration Points
- `CompletionParameters` struct likely contains stop sequences field
- Providers (Qwen3Coder, KimiK2) need to use stop sequences
- Builder pattern already established in file

### API Design Pattern
```rust
.stop_sequences(vec!["Human:".to_string(), "###".to_string()])
// or
.add_stop_sequence("Human:".to_string())
.add_stop_sequence("###".to_string())
```

## CONSTRAINTS

- Maintain fluent builder pattern consistency
- Keep backward compatibility (empty vec is valid)
- Remove all stub comments completely
