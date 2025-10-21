# BUILDER_1: Remove Agent Builder Stub Comments

## OBJECTIVE

Clean up "for now" stub comments in agent builder by implementing proper extensibility patterns for stop sequences.

## BACKGROUND

The agent builder has empty stop sequences with a "for now" comment suggesting future extension. This needs proper API design rather than a stub comment.

## SUBTASK 1: Design Stop Sequences API

**Location:** `packages/candle/src/builders/agent_role/agent_builder.rs:263`

**Current State:**
```rust
// Stop sequences (empty for now, could be extended)
stop_sequences: Vec::new(),
```

**Required Changes:**
- Remove "for now" comment
- Add builder method `stop_sequences(sequences: Vec<String>)` to API
- Add builder method `add_stop_sequence(sequence: String)` for single additions
- Document stop sequence behavior in builder docs
- Set sensible defaults (empty vec is valid default)

**Why:** Stop sequences are a valid configuration option that needs proper API support.

## SUBTASK 2: Implement Stop Sequence Configuration

**Location:** `packages/candle/src/builders/agent_role/agent_builder.rs`

**Required Changes:**
- Add stop sequence builder methods to `CandleAgentBuilder`
- Chain builder methods properly (return Self)
- Validate stop sequences (non-empty strings)
- Pass stop sequences through to completion parameters
- Document common stop sequences in examples

**Why:** Users need ability to configure custom stop sequences for their use cases.

## SUBTASK 3: Wire Stop Sequences to Completion

**Location:** Integration with completion providers

**Required Changes:**
- Ensure stop sequences reach completion provider config
- Verify providers respect stop sequences
- Add stop sequences to completion request parameters
- Document provider-specific stop sequence handling

**Why:** Stop sequences must actually affect model behavior.

## DEFINITION OF DONE

- [ ] No "for now" or "could be extended" comments in code
- [ ] Builder API includes `stop_sequences()` and `add_stop_sequence()` methods
- [ ] Stop sequences properly passed to completion providers
- [ ] Documentation explains stop sequence usage with examples
- [ ] Empty default remains valid (no breaking changes)
- [ ] NO test code written (separate team responsibility)
- [ ] NO benchmark code written (separate team responsibility)

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

- DO NOT write unit tests (separate team handles testing)
- DO NOT write benchmarks (separate team handles benchmarks)
- Maintain fluent builder pattern consistency
- Keep backward compatibility (empty vec is valid)
