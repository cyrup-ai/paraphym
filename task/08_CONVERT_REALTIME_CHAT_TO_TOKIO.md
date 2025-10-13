# Convert Realtime Chat Connection from Crossbeam to Tokio

## Location
`packages/candle/src/domain/chat/realtime/connection.rs`

## Current State
```rust
use crossbeam_channel::{Receiver, Sender, unbounded};
```

## Target State
- Tokio mpsc channels
- Async message passing for realtime chat
- Integrated with tokio runtime

## Tasks

### 1. Replace Channel Types
**Current**:
```rust
use crossbeam_channel::{Receiver, Sender, unbounded};
```

**Changes**:
```rust
use tokio::sync::mpsc;
```

### 2. Update Connection Types
- Find all struct fields using crossbeam Sender/Receiver
- Replace with tokio mpsc equivalents
- Update connection initialization

### 3. Convert Message Passing
- Make all `send()` operations async
- Make all `recv()` operations async
- Update connection handlers to be async

### 4. Update Connection Management
- Convert any blocking operations to async/await
- Ensure proper shutdown handling with tokio channels

## Success Criteria
- ✅ Zero crossbeam_channel usage
- ✅ All channel operations use tokio mpsc
- ✅ Message passing is async
- ✅ Realtime chat still works correctly
- ✅ Compiles and passes tests
