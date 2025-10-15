# FIX: Complete YStream and Crossbeam Migration

## OBJECTIVE

Fix all remaining compilation errors by completing the migration from ystream to tokio_stream and from crossbeam to tokio channels. This task addresses 31+ compilation errors across 8 files that were not covered by previous YSTREAM tasks (M, N, O, P).

## CURRENT STATE

The codebase has a working tokio-based async_stream helper at [`packages/candle/src/async_stream.rs`](../packages/candle/src/async_stream.rs) but several files still reference non-existent types (`AsyncTask`, `spawn_task`, `AsyncStreamSender`) from the old ystream library, and some files use crossbeam channels instead of tokio channels.

**Compilation Status:** ❌ 103 errors (31+ from these 8 files)

## FILES TO FIX

### Category 1: Missing Type Imports (AsyncTask, spawn_task)
1. [`packages/candle/src/core/mod.rs`](../packages/candle/src/core/mod.rs) - Line 11: imports `crate::AsyncTask`
2. [`packages/candle/src/core/engine.rs`](../packages/candle/src/core/engine.rs) - Line 21: imports `crate::{AsyncTask, spawn_task}`
3. [`packages/candle/src/domain/core/mod.rs`](../packages/candle/src/domain/core/mod.rs) - Line 11: imports `crate::AsyncTask`
4. [`packages/candle/src/domain/context/loader.rs`](../packages/candle/src/domain/context/loader.rs) - Line 15: imports `crate::AsyncTask`
5. [`packages/candle/src/domain/mod.rs`](../packages/candle/src/domain/mod.rs) - Line 41: re-exports `ystream::{AsyncTask, spawn_task}`

### Category 2: AsyncStreamSender Usage
6. [`packages/candle/src/domain/chat/commands/types/events.rs`](../packages/candle/src/domain/chat/commands/types/events.rs) - Line 13: uses `crate::AsyncStreamSender`

### Category 3: Missing StreamExt Trait
7. [`packages/candle/src/workflow/ops.rs`](../packages/candle/src/workflow/ops.rs) - Uses `.try_next()` without StreamExt import
8. [`packages/candle/src/workflow/parallel.rs`](../packages/candle/src/workflow/parallel.rs) - Uses `.try_next()` without StreamExt import

### Category 4: Lifetime Issues
9. [`packages/candle/src/memory/core/systems/episodic.rs`](../packages/candle/src/memory/core/systems/episodic.rs) - Line 364: borrowed data escapes closure

### Category 5: Crossbeam Channel Usage
10. [`packages/candle/src/domain/core/mod.rs`](../packages/candle/src/domain/core/mod.rs) - Lines 7-8, 102-114: uses `crossbeam_channel`

## TECHNICAL CONTEXT

### Async Stream Helper (Already Available)

The tokio-based helper is at [`packages/candle/src/async_stream.rs`](../packages/candle/src/async_stream.rs):

```rust
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

pub use tokio_stream::{Stream, StreamExt};
pub use tokio_stream::wrappers::ReceiverStream;

/// Create a stream from a spawned async task
pub fn spawn_stream<T, F, Fut>(f: F) -> impl Stream<Item = T>
where
    T: Send + 'static,
    F: FnOnce(mpsc::UnboundedSender<T>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    let (tx, rx) = mpsc::unbounded_channel();
    tokio::spawn(f(tx));
    UnboundedReceiverStream::new(rx)
}
```

### Type Replacements

| Old (ystream/crossbeam) | New (tokio) | Location |
|-------------------------|-------------|----------|
| `ystream::AsyncTask<T>` | Remove - not needed | N/A |
| `ystream::spawn_task` | `tokio::spawn` | Direct usage |
| `crate::AsyncStreamSender<T>` | `tokio::sync::mpsc::UnboundedSender<T>` | events.rs |
| `crossbeam_channel::Sender<T>` | `tokio::sync::mpsc::Sender<T>` | core/mod.rs |
| `crossbeam_channel::Receiver<T>` | `tokio::sync::mpsc::Receiver<T>` | core/mod.rs |
| `crossbeam_channel::bounded(1)` | `tokio::sync::mpsc::channel(1)` | core/mod.rs |

## SPECIFIC CHANGES REQUIRED

### File 1: `domain/mod.rs`

**Line 41:** Remove ystream re-export

```rust
// BEFORE
pub use ystream::{AsyncTask, spawn_task};
pub use tokio_stream::Stream;

// AFTER  
pub use tokio_stream::Stream;
```

**Why:** `AsyncTask` and `spawn_task` don't exist in the codebase. Remove the invalid re-export.

---

### File 2: `domain/chat/commands/types/events.rs`

**Line 13:** Replace AsyncStreamSender

```rust
// BEFORE
use crate::AsyncStreamSender;

// AFTER
// Remove line - AsyncStreamSender doesn't exist
```

**Lines 439, 442, etc.:** Update field type

```rust
// BEFORE
pub struct StreamingCommandExecutor {
    event_sender: Option<AsyncStreamSender<CommandEvent>>,
}

// AFTER
pub struct StreamingCommandExecutor {
    event_sender: Option<tokio::sync::mpsc::UnboundedSender<CommandEvent>>,
}
```

**Update constructor:**

```rust
// BEFORE
pub fn with_event_sender(event_sender: AsyncStreamSender<CommandEvent>) -> Self {

// AFTER
pub fn with_event_sender(event_sender: tokio::sync::mpsc::UnboundedSender<CommandEvent>) -> Self {
```

**Why:** `AsyncStreamSender` was a ystream type alias. Replace with tokio's unbounded sender directly.

---

### File 3: `core/mod.rs`

**Line 11:** Remove invalid import

```rust
// BEFORE (line 11)
use crate::AsyncTask;

// AFTER
// Remove line completely
```

**Lines 102-114:** Replace crossbeam channel with tokio channel

```rust
// BEFORE
use crossbeam_channel;

pub struct ChannelSender<T> {
    sender: crossbeam_channel::Sender<std::result::Result<T, ChannelError>>,
}

impl<T: Send + 'static> ChannelSender<T> {
    pub fn finish(self, value: T) {
        let _ = self.sender.send(Ok(value));
    }
}

pub fn channel<T: Send + 'static>() -> (
    ChannelSender<T>,
    AsyncTask<std::result::Result<T, ChannelError>>,
) {
    let (tx, rx) = crossbeam_channel::bounded(1);
    (ChannelSender { sender: tx }, AsyncTask::new(rx))
}

// AFTER
pub struct ChannelSender<T> {
    sender: tokio::sync::mpsc::Sender<std::result::Result<T, ChannelError>>,
}

impl<T: Send + 'static> ChannelSender<T> {
    pub async fn finish(self, value: T) {
        let _ = self.sender.send(Ok(value)).await;
    }
    
    pub async fn finish_with_error(self, error: ChannelError) {
        let _ = self.sender.send(Err(error)).await;
    }
}

pub fn channel<T: Send + 'static>() -> (
    ChannelSender<T>,
    tokio::sync::mpsc::Receiver<std::result::Result<T, ChannelError>>,
) {
    let (tx, rx) = tokio::sync::mpsc::channel(1);
    (ChannelSender { sender: tx }, rx)
}
```

**Why:** Replace synchronous crossbeam channels with tokio async channels. Note that `finish()` becomes async since tokio's `send()` is async.

---

### File 4: `core/engine.rs`

**Line 21:** Remove invalid imports

```rust
// BEFORE
use crate::{AsyncTask, spawn_task};

// AFTER
// Remove line completely - not needed
```

**Why:** These types don't exist. If spawn_task functionality is needed, use `tokio::spawn` directly.

---

### File 5: `domain/core/mod.rs`

**Lines 7-8:** Remove crossbeam imports

```rust
// BEFORE
use crossbeam_channel;
use crossbeam_utils::CachePadded;

// AFTER
use crossbeam_utils::CachePadded;  // Keep CachePadded - only removing crossbeam_channel
```

**Line 11:** Remove invalid import

```rust
// BEFORE
use crate::AsyncTask;

// AFTER
// Remove line completely
```

**Lines 102-114:** Same crossbeam → tokio channel changes as File 3

**Why:** Remove crossbeam_channel (but keep crossbeam_utils for CachePadded). Remove invalid AsyncTask import.

---

### File 6: `domain/context/loader.rs`

**Line 15:** Remove invalid import

```rust
// BEFORE
use crate::AsyncTask;

// AFTER
// Remove line completely
```

**Why:** AsyncTask doesn't exist in the codebase.

---

### File 7: `workflow/ops.rs`

**Line 201:** Add StreamExt import and fix .try_next() usage

```rust
// At top of file, add import
use tokio_stream::StreamExt;

// BEFORE (line 201)
while let Some(mid_value) = first_stream.try_next() {

// AFTER
while let Some(mid_value) = first_stream.next().await {
```

**Line 206:** Fix second .try_next()

```rust
// BEFORE
while let Some(output) = second_stream.try_next() {

// AFTER
while let Some(output) = second_stream.next().await {
```

**Line 254:** Fix third .try_next()

```rust
// BEFORE
while let Some(output) = stream.try_next() {

// AFTER  
while let Some(output) = stream.next().await {
```

**Why:** The `.try_next()` method requires `StreamExt` trait import and `.await`. tokio_stream uses `.next().await` pattern.

---

### File 8: `workflow/parallel.rs`

**Line 237:** Add StreamExt import and fix .try_next() usage

```rust
// At top of file, add import
use tokio_stream::StreamExt;

// BEFORE (line 237)
while let Some(result) = op_stream.try_next() {

// AFTER
tokio::pin!(op_stream);
while let Some(result) = op_stream.next().await {
```

**Why:** Same as File 7. Also need to pin the stream before calling `.next().await`.

---

### File 9: `memory/core/systems/episodic.rs`

**Lines 364-368:** Fix lifetime issue by converting &str to String before closure

```rust
// BEFORE (line 361-368)
pub fn create_episodic_memory(
    memory_repo: Arc<RwLock<dyn MemoryRepository + Send + Sync>>,
    id: &str,
    name: &str,
    description: &str,
) -> Pin<Box<dyn Stream<Item = EpisodicMemoryChunk> + Send>> {
    Box::pin(crate::async_stream::spawn_stream(move |tx| async move {
        let id_string = id.to_string();
        let name_string = name.to_string();
        let description_string = description.to_string();

// AFTER
pub fn create_episodic_memory(
    memory_repo: Arc<RwLock<dyn MemoryRepository + Send + Sync>>,
    id: &str,
    name: &str,
    description: &str,
) -> Pin<Box<dyn Stream<Item = EpisodicMemoryChunk> + Send>> {
    // Convert to owned strings BEFORE moving into closure
    let id_owned = id.to_string();
    let name_owned = name.to_string();
    let description_owned = description.to_string();
    
    Box::pin(crate::async_stream::spawn_stream(move |tx| async move {
        let id_string = id_owned;
        let name_string = name_owned;
        let description_string = description_owned;
```

**Why:** Borrowed &str parameters can't escape the function scope. Convert to owned String before the closure so the closure owns the data.

---

## IMPLEMENTATION PATTERNS

### Pattern 1: Remove Invalid Imports

**Files:** core/mod.rs, core/engine.rs, domain/core/mod.rs, domain/context/loader.rs, domain/mod.rs

```rust
// REMOVE these lines completely
use crate::AsyncTask;
use crate::spawn_task;
use crate::{AsyncTask, spawn_task};
pub use ystream::{AsyncTask, spawn_task};
```

### Pattern 2: Replace AsyncStreamSender

**File:** events.rs

```rust
// BEFORE
use crate::AsyncStreamSender;
event_sender: Option<AsyncStreamSender<CommandEvent>>

// AFTER  
// Remove import
event_sender: Option<tokio::sync::mpsc::UnboundedSender<CommandEvent>>
```

### Pattern 3: Replace Crossbeam Channels

**Files:** core/mod.rs, domain/core/mod.rs

```rust
// BEFORE
use crossbeam_channel;
let (tx, rx) = crossbeam_channel::bounded(1);
sender: crossbeam_channel::Sender<T>
let _ = sender.send(value);  // Synchronous

// AFTER
let (tx, rx) = tokio::sync::mpsc::channel(1);
sender: tokio::sync::mpsc::Sender<T>
let _ = sender.send(value).await;  // Async - add .await
```

### Pattern 4: Add StreamExt and Fix .try_next()

**Files:** workflow/ops.rs, workflow/parallel.rs

```rust
// Add at top of file
use tokio_stream::StreamExt;

// BEFORE
while let Some(item) = stream.try_next() {

// AFTER
while let Some(item) = stream.next().await {

// If stream is a local variable (not pinned), add:
tokio::pin!(stream);
while let Some(item) = stream.next().await {
```

### Pattern 5: Fix Borrowed Data in Closures

**File:** episodic.rs

```rust
// BEFORE
fn create(id: &str, name: &str) -> impl Stream {
    spawn_stream(move |tx| async move {
        let id = id.to_string();  // ❌ Borrowed id escapes

// AFTER
fn create(id: &str, name: &str) -> impl Stream {
    let id = id.to_string();  // ✅ Convert before closure
    let name = name.to_string();
    spawn_stream(move |tx| async move {
        // Use owned values
```

## CROSSBEAM → TOKIO CHANNEL MIGRATION

### Key Differences

| Aspect | Crossbeam | Tokio |
|--------|-----------|-------|
| Send | `sender.send(val)` - sync | `sender.send(val).await` - async |
| Receive | `receiver.recv()` - blocking | `receiver.recv().await` - async |
| Bounded | `bounded(n)` | `mpsc::channel(n)` |
| Unbounded | `unbounded()` | `mpsc::unbounded_channel()` |

### Migration Checklist for Crossbeam Files

- [ ] Replace `crossbeam_channel::Sender` with `tokio::sync::mpsc::Sender`
- [ ] Replace `crossbeam_channel::Receiver` with `tokio::sync::mpsc::Receiver`
- [ ] Replace `.send(val)` with `.send(val).await`
- [ ] Replace `.recv()` with `.recv().await`
- [ ] Make functions using channels `async`
- [ ] Keep `crossbeam_utils::CachePadded` (not removing crossbeam_utils)

## IMPLEMENTATION CHECKLIST

### Category 1: Invalid Imports
- [ ] Remove `use crate::AsyncTask` from core/mod.rs
- [ ] Remove `use crate::{AsyncTask, spawn_task}` from core/engine.rs
- [ ] Remove `use crate::AsyncTask` from domain/core/mod.rs
- [ ] Remove `use crate::AsyncTask` from domain/context/loader.rs
- [ ] Remove `pub use ystream::{AsyncTask, spawn_task}` from domain/mod.rs

### Category 2: AsyncStreamSender
- [ ] Remove `use crate::AsyncStreamSender` from events.rs
- [ ] Replace `AsyncStreamSender<T>` with `tokio::sync::mpsc::UnboundedSender<T>` in struct
- [ ] Update constructor parameter type

### Category 3: Crossbeam Channels
- [ ] Remove `use crossbeam_channel` from core/mod.rs (keep crossbeam_utils)
- [ ] Replace crossbeam channel types with tokio types
- [ ] Add `.await` to all `.send()` calls
- [ ] Add `.await` to all `.recv()` calls
- [ ] Make affected functions `async`
- [ ] Repeat for domain/core/mod.rs

### Category 4: StreamExt
- [ ] Add `use tokio_stream::StreamExt` to workflow/ops.rs
- [ ] Replace `.try_next()` with `.next().await` (3 occurrences)
- [ ] Add `use tokio_stream::StreamExt` to workflow/parallel.rs
- [ ] Add `tokio::pin!(stream)` before `.next().await`
- [ ] Replace `.try_next()` with `.next().await`

### Category 5: Lifetime Issues
- [ ] Convert &str to String before spawn_stream closure in episodic.rs
- [ ] Verify all parameters are owned before moving into closure

### Verification
- [ ] Run `cargo check` - should show 0 errors
- [ ] Verify no `ystream` references remain
- [ ] Verify no `AsyncTask` references remain
- [ ] Verify no `spawn_task` references remain
- [ ] Verify no `AsyncStreamSender` references remain
- [ ] Verify `crossbeam_channel` removed (crossbeam_utils kept)

## DEFINITION OF DONE

✅ All invalid imports removed (AsyncTask, spawn_task, AsyncStreamSender)
✅ AsyncStreamSender replaced with tokio::sync::mpsc::UnboundedSender
✅ crossbeam_channel replaced with tokio::sync::mpsc (keep crossbeam_utils)
✅ StreamExt trait imported in workflow files
✅ .try_next() replaced with .next().await
✅ Lifetime issues fixed in episodic.rs
✅ Compiles with `cargo check` (0 errors)
✅ No ystream references remain
✅ No stubs or placeholder code

## VERIFICATION COMMANDS

```bash
# Check for removed types
rg "AsyncTask" packages/candle/src --type rust
rg "spawn_task" packages/candle/src --type rust  
rg "AsyncStreamSender" packages/candle/src --type rust
rg "use ystream" packages/candle/src --type rust

# Check for crossbeam_channel (should be 0)
rg "crossbeam_channel" packages/candle/src --type rust

# Check for crossbeam_utils (should still exist - we keep CachePadded)
rg "crossbeam_utils" packages/candle/src --type rust

# Verify compilation
cd packages/candle && cargo check
```

## REFERENCES

- **Async stream helper:** [`packages/candle/src/async_stream.rs`](../packages/candle/src/async_stream.rs)
- **Tokio mpsc docs:** https://docs.rs/tokio/latest/tokio/sync/mpsc/
- **tokio_stream docs:** https://docs.rs/tokio-stream/latest/tokio_stream/

## NOTES

**Why AsyncTask/spawn_task Don't Exist:**
These were ystream types that were removed. The codebase now uses tokio::spawn directly and tokio_stream::Stream for streaming.

**Why Keep crossbeam_utils:**
`crossbeam_utils::CachePadded` is still used for cache-line padding optimization. We only remove `crossbeam_channel` and replace with tokio channels.

**Async Changes:**
Converting from crossbeam (sync) to tokio (async) means functions using channels must become `async` and add `.await` to send/recv calls.