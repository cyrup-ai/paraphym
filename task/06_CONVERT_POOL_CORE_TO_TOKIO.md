# Convert Pool Core Infrastructure from Crossbeam to Tokio

## Locations
- `packages/candle/src/pool/core/types.rs`
- `packages/candle/src/pool/core/worker_state.rs`
- `packages/candle/src/pool/core/orchestrator.rs`
- `packages/candle/src/pool/core/request_queue.rs`

## Current State
- Uses crossbeam channels for health checks and coordination
- Worker state management with crossbeam channels
- Orchestrator uses crossbeam for worker coordination
- Request queue uses crossbeam for request distribution

## Target State
- All channels converted to tokio mpsc
- All select! macros converted to tokio::select!
- All blocking operations converted to async/await

## Tasks by File

### 1. pool/core/types.rs
**Current**:
```rust
use crossbeam::channel::{Receiver, Sender};
```

**Changes**:
- Replace with `tokio::sync::mpsc`
- Update `HealthPing` and `HealthPong` channel types
- Update `PoolWorkerHandle` trait to use tokio channels

### 2. pool/core/worker_state.rs
**Current**:
```rust
use crossbeam::channel::{Receiver, Sender};
```

**Changes**:
- Replace with `tokio::sync::mpsc`
- Convert any blocking channel operations to async
- Update state transition signaling to use tokio channels

### 3. pool/core/orchestrator.rs
**Current**:
```rust
use crossbeam::channel::{bounded, unbounded, Sender, Receiver};
```

**Changes**:
- Replace with `tokio::sync::mpsc`
- Convert orchestrator loop to async
- Replace `crossbeam::select!` with `tokio::select!`
- Convert any blocking operations to async/await

### 4. pool/core/request_queue.rs
**Current**:
```rust
use crossbeam::channel::{Sender, Receiver, bounded, unbounded};
```

**Changes**:
- Replace with `tokio::sync::mpsc`
- Convert queue management to async
- Update request distribution to use tokio channels
- Replace any blocking operations with async/await

## Dependencies
These conversions must happen AFTER or alongside the individual pool worker conversions since they coordinate with them.

## Success Criteria
- ✅ Zero crossbeam::channel usage
- ✅ All channel operations use tokio mpsc
- ✅ All blocking operations converted to async
- ✅ Compiles and passes tests
- ✅ Pool coordination still works correctly
