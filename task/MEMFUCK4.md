# MEMFUCK4: Replace eprintln! with Proper Tracing Framework

## Status: NEEDS IMPLEMENTATION

Error handling currently uses `eprintln!` to write directly to stderr instead of using the existing `tracing` framework. This breaks production observability.

## Problem

Using `eprintln!` for error logging is not production-grade because:
- No log levels (debug, info, warn, error)
- No structured logging with metadata
- No log aggregation support
- No runtime log filtering
- Goes directly to stderr regardless of environment
- Cannot be controlled or routed by log configuration

## Current State

The project **already has** `tracing = "0.1.41"` as a dependency ([`/packages/candle/Cargo.toml:121`](../packages/candle/Cargo.toml#L121)) and it's **already being used** in the codebase (see [`/packages/candle/src/workflow/parallel.rs:241-244`](../packages/candle/src/workflow/parallel.rs#L241-L244) for example usage).

## Locations to Fix

### 1. [`/packages/candle/src/domain/agent/chat.rs:519`](../packages/candle/src/domain/agent/chat.rs#L519)
**Current broken code:**
```rust
if let Err(e) = user_pending.await {
    eprintln!("Failed to store user memory: {e:?}");
}
```

**Required fix:**
```rust
if let Err(e) = user_pending.await {
    tracing::error!(
        error = ?e,
        memory_type = "user",
        "Failed to store memory to database"
    );
}
```

### 2. [`/packages/candle/src/domain/agent/chat.rs:536`](../packages/candle/src/domain/agent/chat.rs#L536)
**Current broken code:**
```rust
if let Err(e) = assistant_pending.await {
    eprintln!("Failed to store assistant memory: {e:?}");
}
```

**Required fix:**
```rust
if let Err(e) = assistant_pending.await {
    tracing::error!(
        error = ?e,
        memory_type = "assistant",
        "Failed to store memory to database"
    );
}
```

### 3. [`/packages/candle/src/domain/agent/chat.rs:555`](../packages/candle/src/domain/agent/chat.rs#L555)
**Current broken code:**
```rust
if let Err(e) = context_pending.await {
    eprintln!("Failed to store context memory: {e:?}");
}
```

**Required fix:**
```rust
if let Err(e) = context_pending.await {
    tracing::error!(
        error = ?e,
        memory_type = "context",
        "Failed to store memory to database"
    );
}
```

### 4. [`/packages/candle/src/builders/agent_role.rs:1009`](../packages/candle/src/builders/agent_role.rs#L1009)
**Current broken code:**
```rust
if let Err(e) = user_pending.await {
    eprintln!("Failed to store user memory: {:?}", e);
}
```

**Required fix:**
```rust
if let Err(e) = user_pending.await {
    tracing::error!(
        error = ?e,
        memory_type = "user",
        "Failed to store memory to database"
    );
}
```

### 5. [`/packages/candle/src/builders/agent_role.rs:1014`](../packages/candle/src/builders/agent_role.rs#L1014)
**Current broken code:**
```rust
if let Err(e) = assistant_pending.await {
    eprintln!("Failed to store assistant memory: {:?}", e);
}
```

**Required fix:**
```rust
if let Err(e) = assistant_pending.await {
    tracing::error!(
        error = ?e,
        memory_type = "assistant",
        "Failed to store memory to database"
    );
}
```

## Existing Tracing Usage Pattern

The codebase already uses tracing in [`/packages/candle/src/workflow/parallel.rs:241-244`](../packages/candle/src/workflow/parallel.rs#L241-L244):

```rust
tracing::debug!(
    "Parallel operation {} receiver dropped - terminating", 
    op_index
);
```

Follow this pattern but use `tracing::error!` for error cases with structured fields.

## Implementation Pattern

### Tracing Macro Usage (No Import Needed)

The codebase uses `tracing::` prefix directly without importing:

```rust
// Use tracing::error! directly - no import needed
tracing::error!(
    error = ?e,              // Structured field: error value with Debug formatting
    memory_type = "user",    // Structured field: context metadata
    "Message string"         // Message (always last parameter)
);
```

### Structured Logging Benefits

Using structured fields provides:
- **Searchable metadata**: Filter logs by `memory_type = "user"`
- **Error correlation**: Track errors by error type/value
- **Runtime filtering**: Control what gets logged
- **Log aggregation**: Export to monitoring systems
- **Better debugging**: Rich context without parsing strings

## Dependencies

✅ **Already Available**: `tracing = "0.1.41"` is already in [`Cargo.toml`](../packages/candle/Cargo.toml#L121)

No additional dependencies needed.

## Definition of Done

- ✅ Replace `eprintln!` at `/packages/candle/src/domain/agent/chat.rs:519` with `tracing::error!`
- ✅ Replace `eprintln!` at `/packages/candle/src/domain/agent/chat.rs:536` with `tracing::error!`
- ✅ Replace `eprintln!` at `/packages/candle/src/domain/agent/chat.rs:555` with `tracing::error!`
- ✅ Replace `eprintln!` at `/packages/candle/src/builders/agent_role.rs:1009` with `tracing::error!`
- ✅ Replace `eprintln!` at `/packages/candle/src/builders/agent_role.rs:1014` with `tracing::error!`
- ✅ All error logging uses structured fields (`error = ?e`, `memory_type = "..."`)
- ✅ No `eprintln!` statements remain in memory storage error handling

## Impact

✅ **Enables**: Production-grade logging with levels and structure
✅ **Enables**: Runtime log filtering and control
✅ **Enables**: Integration with log aggregation and monitoring systems
✅ **Enables**: Searchable, structured error metadata
✅ **Enables**: Better debugging with contextual information
✅ **Fixes**: Errors no longer bypass logging infrastructure
