# Convert LLaVA Vision Pool from Crossbeam to Tokio Channels

## Location
`packages/candle/src/pool/capabilities/vision.rs`

## Current State
- Uses crossbeam bounded channels
- Sync worker with `crossbeam::select!` macro
- `std::thread::spawn` for workers
- Uses `rt.block_on()` for async operations (lines 314, 338)

## Target State
- Tokio mpsc channels
- Async worker with `tokio::select!` macro
- `tokio::spawn` for workers
- Direct `.await` on async methods (NO block_on)

## Special Considerations
- LLaVA model may have non-Send constraints
- If model is !Send, use `tokio::task::LocalSet` pattern
- Same pattern we used for cognitive workers

## Tasks

### 1. Replace Channel Imports
```rust
// FROM:
use crossbeam::channel::{Receiver, Sender, bounded};
use crossbeam::select;

// TO:
use tokio::sync::mpsc;
use tokio::sync::oneshot;
```

### 2. Convert Worker Function to Async
- Change `fn llava_worker` to `async fn`
- Replace `crossbeam::select!` with `tokio::select!`
- Replace `rt.block_on()` with direct `.await`

### 3. Handle Non-Send Model (if needed)
If model is !Send:
```rust
// Use LocalSet pattern
let local = tokio::task::LocalSet::new();
tokio::task::spawn_local(async move {
    llava_worker(model, channels, context).await;
});
```

### 4. Update Worker Spawning
- Change `std::thread::spawn` to `tokio::spawn` (or `spawn_local`)
- Await the async worker function

### 5. Convert Channel Creation
- Replace `bounded()` with `mpsc::channel()` or `mpsc::unbounded_channel()`
- Use `oneshot::channel()` for responses

## Success Criteria
- ✅ Zero crossbeam usage
- ✅ Zero block_on calls
- ✅ Zero std::thread::spawn
- ✅ All async/await with tokio
- ✅ Handles !Send model correctly (LocalSet if needed)
- ✅ Compiles and passes tests
