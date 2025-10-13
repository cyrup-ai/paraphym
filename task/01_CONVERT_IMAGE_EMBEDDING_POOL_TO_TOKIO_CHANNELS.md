# Convert Image Embedding Pool from Crossbeam to Tokio Channels

## Location
`packages/candle/src/pool/capabilities/image_embedding.rs`

## Current Architecture

**Sync Worker Pattern**:
- Uses crossbeam bounded channels
- Sync worker function with `crossbeam::select!` macro
- Spawns workers with `std::thread::spawn`
- Uses `rt.block_on()` to bridge sync → async model methods
- 4 block_on call sites (lines 149, 171, 193, 216)

## Target Architecture

**Async Worker Pattern** (same as cognitive workers):
- Use tokio::sync::mpsc unbounded/bounded channels
- Async worker function with `tokio::select!` macro
- Spawn workers with `tokio::spawn`
- Direct `.await` on model methods (no block_on)

## Work Breakdown

### Task 1: Convert Channel Types

**File**: `image_embedding.rs`

**Request Structs** (lines 14-35):
- Change `Sender<Result<...>>` from crossbeam to tokio::sync::oneshot
- Pattern: `tokio::sync::oneshot::Sender<Result<Vec<f32>, PoolError>>`

**Worker Handle** (lines 38-69):
- Change all channel types from crossbeam to tokio::sync::mpsc
- Pattern: `tokio::sync::mpsc::UnboundedSender<EmbedImageRequest>`

**Worker Channels** (lines 72-80):
- Change all receiver types to tokio::sync::mpsc
- Pattern: `tokio::sync::mpsc::UnboundedReceiver<EmbedImageRequest>`

### Task 2: Convert Worker Function to Async

**Function Signature** (line 97):
```rust
// FROM:
pub fn image_embedding_worker<T: ImageEmbeddingCapable>(
    model: T,
    channels: ImageEmbeddingWorkerChannels,
    context: ImageEmbeddingWorkerContext,
)

// TO:
pub async fn image_embedding_worker<T: ImageEmbeddingCapable>(
    model: T,
    channels: ImageEmbeddingWorkerChannels,
    context: ImageEmbeddingWorkerContext,
)
```

### Task 3: Convert select! Macro

**Worker Loop** (lines 135-248):

```rust
// FROM:
select! {
    recv(embed_image_rx) -> req => {
        if let Ok(req) = req {
            // handler
        }
    }
}

// TO:
tokio::select! {
    Some(req) = embed_image_rx.recv() => {
        // handler (req already unwrapped)
    }
}
```

### Task 4: Replace block_on with .await

**4 Locations** (lines 141-150, 163-172, 185-194, 207-217):

```rust
// FROM:
let Some(rt) = crate::runtime::shared_runtime() else {
    log::error!("Shared runtime unavailable");
    let _ = req.response.send(Err(PoolError::RuntimeUnavailable));
    state.store(WorkerState::Ready as u32, Ordering::Release);
    last_activity = SystemTime::now();
    continue;
};

let result = rt.block_on(model.embed_image(&req.image_path))
    .map_err(|e| PoolError::ModelError(e.to_string()));

// TO:
let result = model.embed_image(&req.image_path)
    .await
    .map_err(|e| PoolError::ModelError(e.to_string()));
```

### Task 5: Convert Worker Spawning

**Spawn Function** (lines 262-360):

```rust
// FROM (line 300):
std::thread::spawn(move || {
    // worker setup
    image_embedding_worker(model, channels, context);
});

// TO:
tokio::spawn(async move {
    // worker setup
    image_embedding_worker(model, channels, context).await;
});
```

### Task 6: Update Channel Creation

**Channel Creation** (lines 274-283):

```rust
// FROM:
let (embed_image_tx, embed_image_rx) = bounded(self.config().image_embed_queue_capacity);

// TO:
let (embed_image_tx, embed_image_rx) = tokio::sync::mpsc::unbounded_channel();
// OR for bounded:
let (embed_image_tx, embed_image_rx) = tokio::sync::mpsc::channel(self.config().image_embed_queue_capacity);
```

### Task 7: Update Response Pattern

**All Request Handlers**:

```rust
// FROM:
let _ = req.response.send(result);

// TO (same, oneshot sender works the same way):
let _ = req.response.send(result);
```

## Testing Requirements

After conversion:
1. ✅ Verify no block_on anywhere in file
2. ✅ Verify no crossbeam channels anywhere in file  
3. ✅ Verify worker spawns with tokio::spawn
4. ✅ Verify all model methods use .await
5. ✅ Verify cargo check passes
6. ✅ Test image embedding operations work

## Success Criteria

- ✅ Zero `block_on` calls
- ✅ Zero `crossbeam::channel` usage
- ✅ Zero `std::thread::spawn` usage
- ✅ All async/await with tokio runtime
- ✅ Compiles successfully
- ✅ Functional equivalence maintained

## Dependencies Already Available

✅ tokio = { version = "1.47", features = ["full"] }
✅ Model methods return Send futures (already fixed)

## Estimated Impact

- **Lines changed**: ~150 lines
- **Complexity**: Medium (similar to cognitive worker conversion)
- **Risk**: Low (same pattern we successfully used before)
