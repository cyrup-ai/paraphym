# YSTREAM_O: Builder Files AsyncStream Migration

## OBJECTIVE
Convert AsyncStream usage to tokio Stream in builder module files, completing the async stream migration for all builder patterns in the codebase.

## SCOPE CLARIFICATION

**Note:** `builders/agent_role.rs` is listed in both YSTREAM_N and YSTREAM_O. To avoid duplication:
- ✅ **YSTREAM_N covers:** `builders/agent_role.rs` and `cli/runner.rs`
- ✅ **YSTREAM_O covers:** The remaining 4 builder files (agent_builder, audio, completion_response_builder, image)

If YSTREAM_N is complete, agent_role.rs should already be converted. This task focuses on the 4 remaining builder files.

## FILES TO CONVERT

### 1. `packages/candle/src/builders/agent_builder.rs`
**AsyncStream Usage:** 1 occurrence  
**Location:** Line 113 - `build()` method returns AsyncStream  
**Pattern:** AsyncStream::with_channel in synchronous builder method  
**Crossbeam:** Line 7 has `crossbeam_utils::CachePadded` - **DO NOT CHANGE** (only removing crossbeam_queue, not crossbeam_utils)

### 2. `packages/candle/src/builders/audio.rs`
**AsyncStream Usage:** 4 occurrences  
**Locations:**  
- Line 41: `decode()` trait method signature
- Line 45: `stream()` trait method signature  
- Lines 165, 180: `AsyncStream::new(rx)` - **Already using tokio mpsc!**
**Pattern:** AsyncStream wrapper around existing tokio::sync::mpsc channels  
**Note:** This file already uses `tokio::sync::mpsc::unbounded_channel()` - just needs to use `UnboundedReceiverStream` instead of `AsyncStream::new()`

### 3. `packages/candle/src/builders/completion/completion_response_builder.rs`
**AsyncStream Usage:** 3 occurrences  
**Location:** Line 152 - `build()` method returns AsyncStream  
**Pattern:** AsyncStream::with_channel in synchronous builder method

### 4. `packages/candle/src/builders/image.rs`
**AsyncStream Usage:** 4 occurrences  
**Locations:**  
- Line 137: `load()` trait method signature
- Line 140: `process()` trait method signature
- Line 651: `load()` implementation using AsyncStream::with_channel
- Line 664: `process()` implementation using AsyncStream::with_channel  
**Pattern:** AsyncStream::with_channel with synchronous closure

## TECHNICAL CONTEXT

### Current AsyncStream Helper (Reference)
See [`packages/candle/src/async_stream.rs`](../packages/candle/src/async_stream.rs):
```rust
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

### Related Conversions (Already Complete)
- ✅ YSTREAM_M: Chat core modules (config, conversation, formatting, macros, message_processing)
- ✅ YSTREAM_N: CLI runner and agent role builder

## CONVERSION PATTERNS

### Pattern 1: Simple Build Method (agent_builder.rs, completion_response_builder.rs)

These methods are **synchronous** but return an async stream. The closure in AsyncStream::with_channel is also synchronous.

**BEFORE:**
```rust
use ystream::AsyncStream;

pub fn build(self) -> AsyncStream<AgentResult<Agent>> {
    AsyncStream::with_channel(|stream_sender| {
        let result = /* compute result synchronously */;
        let _ = stream_sender.send(result);
    })
}
```

**AFTER:**
```rust
use std::pin::Pin;
use tokio_stream::Stream;

pub fn build(self) -> Pin<Box<dyn Stream<Item = AgentResult<Agent>> + Send>> {
    Box::pin(crate::async_stream::spawn_stream(|sender| async move {
        let result = /* compute result synchronously */;
        let _ = sender.send(result);
    }))
}
```

**Key Changes:**
1. Return type: `AsyncStream<T>` → `Pin<Box<dyn Stream<Item = T> + Send>>`
2. Replace `AsyncStream::with_channel` with `Box::pin(crate::async_stream::spawn_stream(...))` 
3. Add `async move` to closure (even though body is sync)
4. Update imports

### Pattern 2: Tokio Channel Wrapper (audio.rs)

This file **already uses tokio::sync::mpsc** but wraps it in AsyncStream. Just use UnboundedReceiverStream directly.

**BEFORE:**
```rust
use ystream::AsyncStream;

fn decode(self) -> impl AsyncStream<Item = TranscriptionChunk> {
    let chunk = TranscriptionChunk { /* ... */ };
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let _ = tx.send(chunk);
    AsyncStream::new(rx)  // ← Unnecessary wrapper!
}
```

**AFTER:**
```rust
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::Stream;

fn decode(self) -> impl Stream<Item = TranscriptionChunk> {
    let chunk = TranscriptionChunk { /* ... */ };
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let _ = tx.send(chunk);
    UnboundedReceiverStream::new(rx)  // ← Direct stream creation!
}
```

**Key Changes:**
1. Return type: `impl AsyncStream<Item = T>` → `impl Stream<Item = T>`
2. Remove `AsyncStream::new(rx)` wrapper
3. Use `UnboundedReceiverStream::new(rx)` directly
4. Update imports

### Pattern 3: Synchronous Channel with Sync Closure (image.rs)

Uses AsyncStream::with_channel but the closure is completely synchronous (no async operations).

**BEFORE:**
```rust
fn load(self) -> AsyncStream<ImageChunk> {
    ystream::AsyncStream::with_channel(move |sender| {
        let chunk = ImageChunk { /* ... */ };
        let _ = sender.send(chunk);
    })
}
```

**AFTER:**
```rust
use std::pin::Pin;
use tokio_stream::Stream;

fn load(self) -> Pin<Box<dyn Stream<Item = ImageChunk> + Send>> {
    Box::pin(crate::async_stream::spawn_stream(|sender| async move {
        let chunk = ImageChunk { /* ... */ };
        let _ = sender.send(chunk);
    }))
}
```

**Key Changes:**
1. Return type: `AsyncStream<T>` → `Pin<Box<dyn Stream<Item = T> + Send>>`
2. Replace `ystream::AsyncStream::with_channel` with `Box::pin(crate::async_stream::spawn_stream(...))`
3. Add `async move` to closure
4. Update imports

### Pattern 4: Stream with Collected Items (image.rs process method)

Uses AsyncStream::with_channel and collects from another stream synchronously (blocking).

**BEFORE:**
```rust
fn process<F>(self, f: F) -> AsyncStream<ImageChunk>
where
    F: FnOnce(ImageChunk) -> ImageChunk + Send + 'static,
{
    let load_stream = self.load();
    
    ystream::AsyncStream::with_channel(move |sender| {
        let chunks = load_stream.collect();  // ← Blocking collect!
        if let Some(chunk) = chunks.into_iter().next() {
            let processed = f(chunk);
            let _ = sender.send(processed);
        }
    })
}
```

**AFTER:**
```rust
use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};

fn process<F>(self, f: F) -> Pin<Box<dyn Stream<Item = ImageChunk> + Send>>
where
    F: FnOnce(ImageChunk) -> ImageChunk + Send + 'static,
{
    let load_stream = self.load();
    
    Box::pin(crate::async_stream::spawn_stream(|sender| async move {
        tokio::pin!(load_stream);
        if let Some(chunk) = load_stream.next().await {
            let processed = f(chunk);
            let _ = sender.send(processed);
        }
    }))
}
```

**Key Changes:**
1. Return type updated
2. Use `async move` with `.next().await` instead of blocking `.collect()`
3. Pin the stream with `tokio::pin!` before calling `.next().await`
4. Update imports to include `StreamExt`

## SPECIFIC CHANGES REQUIRED

### File: `builders/agent_builder.rs`

**Line 8:** Remove ystream import
```rust
// REMOVE
use ystream::AsyncStream;

// ADD
use std::pin::Pin;
use tokio_stream::Stream;
```

**Lines 113-175:** Convert build() method
```rust
// BEFORE (line 113)
pub fn build(self) -> AsyncStream<AgentResult<Agent>> {
    AsyncStream::with_channel(|stream_sender| {
        // ... 60 lines of synchronous logic ...
        let _ = stream_sender.send(result);
    })
}

// AFTER
pub fn build(self) -> Pin<Box<dyn Stream<Item = AgentResult<Agent>> + Send>> {
    Box::pin(crate::async_stream::spawn_stream(|sender| async move {
        let result = (|| {
            // ... same 60 lines of synchronous logic ...
            // (keep all the validation, memory init, tool conversion logic unchanged)
        })();
        let _ = sender.send(result);
    }))
}
```

**Note:** Keep the `crossbeam_utils::CachePadded` import on line 7 - we only remove `crossbeam_queue`, not `crossbeam_utils`.

### File: `builders/audio.rs`

**Line 6:** Update import
```rust
// REMOVE
use ystream::AsyncStream;

// ADD
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::Stream;
```

**Lines 41, 45:** Update trait method signatures
```rust
// BEFORE
fn decode(self) -> impl AsyncStream<Item = TranscriptionChunk>;
fn stream(self) -> impl AsyncStream<Item = SpeechChunk>;

// AFTER  
fn decode(self) -> impl Stream<Item = TranscriptionChunk>;
fn stream(self) -> impl Stream<Item = SpeechChunk>;
```

**Line 166:** Convert decode() implementation
```rust
// BEFORE (line 165-167)
let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
let _ = tx.send(chunk);
AsyncStream::new(rx)

// AFTER
let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
let _ = tx.send(chunk);
UnboundedReceiverStream::new(rx)
```

**Line 181:** Convert stream() implementation
```rust
// BEFORE (line 180-182)
let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
let _ = tx.send(chunk);
AsyncStream::new(rx)

// AFTER
let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
let _ = tx.send(chunk);
UnboundedReceiverStream::new(rx)
```

### File: `builders/completion/completion_response_builder.rs`

**Line 4:** Update import
```rust
// REMOVE
use ystream::AsyncStream;

// ADD
use std::pin::Pin;
use tokio_stream::Stream;
```

**Lines 152-176:** Convert build() method
```rust
// BEFORE (line 152)
pub fn build(self) -> AsyncStream<CompactCompletionResponse> {
    AsyncStream::with_channel(move |sender| {
        let response = CompactCompletionResponse { /* ... */ };
        let _ = sender.send(response);
    })
}

// AFTER
pub fn build(self) -> Pin<Box<dyn Stream<Item = CompactCompletionResponse> + Send>> {
    Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
        let response = CompactCompletionResponse {
            // ... same field initialization ...
        };
        let _ = sender.send(response);
    }))
}
```

### File: `builders/image.rs`

**Line 10:** Update import
```rust
// REMOVE
use ystream::AsyncStream;

// ADD
use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};
```

**Lines 137, 140:** Update trait method signatures
```rust
// BEFORE
fn load(self) -> AsyncStream<ImageChunk>;
fn process<F>(self, f: F) -> AsyncStream<ImageChunk>
    where F: FnOnce(ImageChunk) -> ImageChunk + Send + 'static;

// AFTER
fn load(self) -> Pin<Box<dyn Stream<Item = ImageChunk> + Send>>;
fn process<F>(self, f: F) -> Pin<Box<dyn Stream<Item = ImageChunk> + Send>>
    where F: FnOnce(ImageChunk) -> ImageChunk + Send + 'static;
```

**Line 651:** Convert load() implementation
```rust
// BEFORE (line 643-655)
fn load(self) -> AsyncStream<ImageChunk> {
    let chunk = ImageChunk { /* ... */ };
    ystream::AsyncStream::with_channel(move |sender| {
        let _ = sender.send(chunk);
    })
}

// AFTER
fn load(self) -> Pin<Box<dyn Stream<Item = ImageChunk> + Send>> {
    let chunk = ImageChunk { /* ... */ };
    Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
        let _ = sender.send(chunk);
    }))
}
```

**Line 664:** Convert process() implementation
```rust
// BEFORE (line 658-672)
fn process<F>(self, f: F) -> AsyncStream<ImageChunk>
where
    F: FnOnce(ImageChunk) -> ImageChunk + Send + 'static,
{
    let load_stream = self.load();
    ystream::AsyncStream::with_channel(move |sender| {
        let chunks = load_stream.collect();  // Blocking!
        if let Some(chunk) = chunks.into_iter().next() {
            let processed = f(chunk);
            let _ = sender.send(processed);
        }
    })
}

// AFTER
fn process<F>(self, f: F) -> Pin<Box<dyn Stream<Item = ImageChunk> + Send>>
where
    F: FnOnce(ImageChunk) -> ImageChunk + Send + 'static,
{
    let load_stream = self.load();
    Box::pin(crate::async_stream::spawn_stream(|sender| async move {
        tokio::pin!(load_stream);
        if let Some(chunk) = load_stream.next().await {
            let processed = f(chunk);
            let _ = sender.send(processed);
        }
    }))
}
```

## IMPLEMENTATION CHECKLIST

### agent_builder.rs
- [ ] Update imports (remove ystream, add Pin and Stream)
- [ ] Convert build() return type
- [ ] Replace AsyncStream::with_channel with spawn_stream
- [ ] Add async move to closure
- [ ] Keep crossbeam_utils::CachePadded import unchanged

### audio.rs  
- [ ] Update imports (remove ystream, add UnboundedReceiverStream and Stream)
- [ ] Update decode() trait signature
- [ ] Update stream() trait signature
- [ ] Replace AsyncStream::new(rx) with UnboundedReceiverStream::new(rx) in decode()
- [ ] Replace AsyncStream::new(rx) with UnboundedReceiverStream::new(rx) in stream()

### completion_response_builder.rs
- [ ] Update imports (remove ystream, add Pin and Stream)
- [ ] Convert build() return type
- [ ] Replace AsyncStream::with_channel with spawn_stream
- [ ] Add async move to closure

### image.rs
- [ ] Update imports (remove ystream, add Pin, Stream, and StreamExt)
- [ ] Update load() trait signature
- [ ] Update process() trait signature
- [ ] Convert load() implementation
- [ ] Convert process() implementation with tokio::pin! and .next().await

### Global Verification
- [ ] Run `cargo check` to verify compilation
- [ ] Verify no `AsyncStream` references remain
- [ ] Verify no `ystream` imports remain
- [ ] Confirm agent_role.rs was already handled in YSTREAM_N

## IMPORTANT NOTES

### What NOT to Change

1. **crossbeam_utils in agent_builder.rs:** 
   ```rust
   use crossbeam_utils::CachePadded;  // ← KEEP THIS
   ```
   We only remove `crossbeam_queue`, not `crossbeam_utils`.

2. **tokio::sync::mpsc in audio.rs:**
   ```rust
   let (tx, rx) = tokio::sync::mpsc::unbounded_channel();  // ← KEEP THIS
   ```
   The tokio mpsc usage is already correct - just change the wrapper.

3. **Synchronous logic in builders:**
   All the builder logic (validation, field construction) remains synchronous. We just wrap it in `async move` for the spawn_stream closure.

### Key Differences from Previous Tasks

**audio.rs is special:** Unlike other files that used pure `AsyncStream::with_channel`, audio.rs already uses `tokio::sync::mpsc::unbounded_channel` internally but wraps the receiver in `AsyncStream::new()`. The fix is simpler: just use `UnboundedReceiverStream::new()` directly instead of `AsyncStream::new()`.

**image.rs process() is special:** It consumes another stream. Must use `tokio::pin!` and `.next().await` instead of the blocking `.collect()` pattern.

## DEFINITION OF DONE

- ✅ All AsyncStream usage removed from agent_builder.rs (1 occurrence)
- ✅ All AsyncStream usage removed from audio.rs (4 occurrences)  
- ✅ All AsyncStream usage removed from completion_response_builder.rs (3 occurrences)
- ✅ All AsyncStream usage removed from image.rs (4 occurrences)
- ✅ All ystream imports removed from all 4 files
- ✅ All return types updated to Pin<Box<dyn Stream>> or impl Stream
- ✅ Project compiles with `cargo check` (0 errors)
- ✅ No stubs or placeholder code introduced
- ✅ crossbeam_utils::CachePadded remains in agent_builder.rs

## VERIFICATION COMMANDS

```bash
# Verify no AsyncStream references remain in target files
rg "AsyncStream" packages/candle/src/builders/agent_builder.rs
rg "AsyncStream" packages/candle/src/builders/audio.rs
rg "AsyncStream" packages/candle/src/builders/completion/completion_response_builder.rs
rg "AsyncStream" packages/candle/src/builders/image.rs

# Verify no ystream imports remain
rg "use ystream" packages/candle/src/builders/agent_builder.rs
rg "use ystream" packages/candle/src/builders/audio.rs
rg "use ystream" packages/candle/src/builders/completion/completion_response_builder.rs
rg "use ystream" packages/candle/src/builders/image.rs

# Verify compilation
cd packages/candle && cargo check

# Verify proper patterns
rg "Box::pin\(crate::async_stream::spawn_stream" packages/candle/src/builders/
rg "UnboundedReceiverStream::new" packages/candle/src/builders/audio.rs

# Check that crossbeam_utils is still present (should find it)
rg "crossbeam_utils" packages/candle/src/builders/agent_builder.rs

# Verify agent_role.rs was already converted (should find no AsyncStream)
rg "AsyncStream" packages/candle/src/builders/agent_role.rs
```

## REFERENCES

- **async_stream helper:** [`packages/candle/src/async_stream.rs`](../packages/candle/src/async_stream.rs)
- **Previous conversion examples:**
  - YSTREAM_M: [`packages/candle/src/domain/chat/`](../packages/candle/src/domain/chat/) (config, macros, formatting)
  - YSTREAM_N: [`packages/candle/src/cli/runner.rs`](../packages/candle/src/cli/runner.rs), [`packages/candle/src/builders/agent_role.rs`](../packages/candle/src/builders/agent_role.rs)

## TASK SCOPE SUMMARY

**Converting 4 builder files (12 total AsyncStream occurrences):**
- agent_builder.rs: 1 occurrence
- audio.rs: 4 occurrences (special case - already uses tokio mpsc)
- completion_response_builder.rs: 3 occurrences  
- image.rs: 4 occurrences (including special stream consumption pattern)

**NOT converting:** agent_role.rs (handled in YSTREAM_N)
