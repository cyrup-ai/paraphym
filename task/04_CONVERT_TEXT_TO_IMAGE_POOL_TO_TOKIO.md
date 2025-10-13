# Convert Text-to-Image Pool from Crossbeam to Tokio Channels

## Location
`packages/candle/src/pool/capabilities/text_to_image.rs`

## Current State
- Uses crossbeam bounded channels
- Sync worker with `crossbeam::select!` macro
- `std::thread::spawn` for workers
- Uses `block_on` for async model operations

## Target State
- Tokio mpsc channels
- Async worker with `tokio::select!` macro
- `tokio::spawn` for workers
- Direct `.await` on model methods (NO block_on)

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
- Change `fn text_to_image_worker` to `async fn`
- Replace `crossbeam::select!` with `tokio::select!`
- Replace all `block_on` calls with direct `.await`

### 3. Update Worker Spawning
- Change `std::thread::spawn` to `tokio::spawn`
- Await the async worker function

### 4. Convert Channel Creation
- Replace `bounded()` with `mpsc::channel()` or `mpsc::unbounded_channel()`
- Use `oneshot::channel()` for responses

## Success Criteria
- ✅ Zero crossbeam usage
- ✅ Zero block_on calls
- ✅ Zero std::thread::spawn
- ✅ All async/await with tokio
- ✅ Compiles and passes tests
