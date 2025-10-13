# Convert Domain Concurrency Module from Crossbeam to Tokio

## Locations
- `packages/candle/src/domain/concurrency/mod.rs`
- `packages/candle/src/concurrency/mod.rs`
- `packages/candle/src/domain/context/realtime.rs`

## Current State
- Uses `crossbeam_channel::{bounded, unbounded}` for message passing
- Wraps channels in custom abstractions
- Used by various domain components

## Target State
- All channels converted to tokio mpsc
- Async-first API
- Integrated with tokio runtime

## Tasks

### 1. domain/concurrency/mod.rs
**Current**:
```rust
use crossbeam_channel::{bounded, unbounded};
```

**Changes**:
- Replace with `tokio::sync::mpsc`
- Convert `Channel` type to use tokio channels
- Update `send()` and `recv()` methods to be async
- Update all consumers of this module

### 2. concurrency/mod.rs
**Current**:
```rust
use crossbeam_channel::{bounded, unbounded};
```

**Changes**:
- Replace with `tokio::sync::mpsc`
- Convert channel wrappers to use tokio
- Make all operations async

### 3. domain/context/realtime.rs
**Current**:
```rust
use crossbeam_channel::{Receiver, Sender, bounded};
```

**Changes**:
- Replace with `tokio::sync::mpsc`
- Convert file watcher to use tokio channels
- Make event handling async

## Success Criteria
- ✅ Zero crossbeam_channel usage
- ✅ All channel operations use tokio mpsc
- ✅ All blocking operations converted to async
- ✅ Compiles and passes tests
- ✅ Domain components work with new async API
