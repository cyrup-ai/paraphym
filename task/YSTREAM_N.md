# YSTREAM_N: CLI & Builder Async Stream Migration

## OBJECTIVE
Convert AsyncStream usage to tokio Stream in CLI runner and agent builder modules, completing the async stream migration for the application's user-facing components.

## FILES TO CONVERT

### 1. `packages/candle/src/cli/runner.rs`
**Location:** Lines 223-241  
**Current Pattern:** Uses `ystream::{AsyncStream, emit, spawn_task}`  
**Usage Context:** Resolves user input and forwards chat stream chunks in the CLI conversation loop

### 2. `packages/candle/src/builders/agent_role.rs`
**Location:** Multiple occurrences starting at line 48  
**Current Pattern:** `AsyncStream::with_channel` throughout agent chat methods  
**Usage Context:** Returns streaming responses for agent chat operations, tool calls, and conversation turns

## TECHNICAL CONTEXT

### Current AsyncStream Helper (Reference)
See [`packages/candle/src/async_stream.rs`](../packages/candle/src/async_stream.rs) for the tokio-based helper:
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
- ✅ `domain/chat/config.rs` - YSTREAM_M
- ✅ `domain/chat/conversation/mod.rs` - YSTREAM_M
- ✅ `domain/chat/formatting.rs` - YSTREAM_M
- ✅ `domain/chat/macros.rs` - YSTREAM_M
- ✅ `domain/chat/message/message_processing.rs` - YSTREAM_M

## CONVERSION PATTERNS

### Pattern 1: Simple Stream Creation

**BEFORE:**
```rust
use ystream::{AsyncStream, emit, spawn_task};

AsyncStream::with_channel(move |sender| {
    spawn_task(async move || {
        let resolved = resolve_input(&message).await.unwrap_or(message.clone());
        let chat_stream = agent_clone.chat(CandleChatLoop::UserPrompt(resolved));
        
        while let Some(chunk) = chat_stream.try_next() {
            emit!(sender, chunk);
        }
    });
})
```

**AFTER:**
```rust
use tokio_stream::StreamExt;
use std::pin::Pin;

Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
    let resolved = resolve_input(&message).await.unwrap_or(message.clone());
    let chat_stream = agent_clone.chat(CandleChatLoop::UserPrompt(resolved));
    
    while let Some(chunk) = chat_stream.next().await {
        let _ = sender.send(chunk);
    }
}))
```

**Key Changes:**
1. Remove `spawn_task` wrapper - `spawn_stream` already handles spawning
2. Replace `emit!(sender, chunk)` with `let _ = sender.send(chunk)`
3. Replace `.try_next()` with `.next().await`
4. Wrap in `Box::pin()` for return type
5. Update imports: remove `ystream`, add `tokio_stream::StreamExt` and `std::pin::Pin`

### Pattern 2: Return Type Signatures

**BEFORE:**
```rust
pub fn chat(&self, chat_loop: CandleChatLoop) -> AsyncStream<CandleMessageChunk> {
    AsyncStream::with_channel(|sender| {
        // ...
    })
}
```

**AFTER:**
```rust
use std::pin::Pin;
use tokio_stream::Stream;

pub fn chat(&self, chat_loop: CandleChatLoop) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>> {
    Box::pin(crate::async_stream::spawn_stream(|sender| async move {
        // ...
    }))
}
```

### Pattern 3: Sender Usage

**BEFORE:**
```rust
let _ = sender.send(final_chunk);  // Synchronous send in AsyncStream
emit!(sender, chunk);               // Macro for sending
```

**AFTER:**
```rust
let _ = sender.send(final_chunk);   // Already correct - UnboundedSender::send() is sync
```

**Note:** The `mpsc::UnboundedSender::send()` method is synchronous and non-blocking, so `let _ = sender.send(value)` remains correct.

## SPECIFIC CHANGES REQUIRED

### File: `cli/runner.rs`

**Location: Lines 223-241**

1. Update imports at top of file:
```rust
// Remove
use ystream::{AsyncStream, emit, spawn_task};

// Add
use tokio_stream::StreamExt;
use std::pin::Pin;
```

2. Convert the `InputHandlerResult::Chat` arm:
```rust
// BEFORE (lines 223-241)
InputHandlerResult::Chat(message) => {
    use ystream::{AsyncStream, emit, spawn_task};

    let agent_clone = agent.clone();

    AsyncStream::with_channel(move |sender| {
        spawn_task(async move || {
            let resolved = resolve_input(&message).await.unwrap_or(message.clone());
            let chat_stream = agent_clone.chat(CandleChatLoop::UserPrompt(resolved));
            
            while let Some(chunk) = chat_stream.try_next() {
                emit!(sender, chunk);
            }
        });
    })
}

// AFTER
InputHandlerResult::Chat(message) => {
    let agent_clone = agent.clone();

    Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
        let resolved = resolve_input(&message).await.unwrap_or(message.clone());
        let chat_stream = agent_clone.chat(CandleChatLoop::UserPrompt(resolved));
        
        while let Some(chunk) = chat_stream.next().await {
            let _ = sender.send(chunk);
        }
    }))
}
```

### File: `builders/agent_role.rs`

**Multiple locations - search for all `AsyncStream::with_channel` calls**

1. Update imports at top of file:
```rust
// Remove
use ystream::AsyncStream;

// Add (if not already present)
use std::pin::Pin;
use tokio_stream::Stream;
```

2. Convert all method return types:
```rust
// BEFORE
pub fn chat(&self, chat_loop: CandleChatLoop) -> AsyncStream<CandleMessageChunk>

// AFTER
pub fn chat(&self, chat_loop: CandleChatLoop) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>
```

3. Convert all stream creation sites:
```rust
// BEFORE
AsyncStream::with_channel(|sender| {
    let final_chunk = CandleMessageChunk::Complete { /* ... */ };
    let _ = sender.send(final_chunk);
})

// AFTER
Box::pin(crate::async_stream::spawn_stream(|sender| async move {
    let final_chunk = CandleMessageChunk::Complete { /* ... */ };
    let _ = sender.send(final_chunk);
}))
```

4. For methods with spawned tasks, remove the runtime spawn wrapper:
```rust
// BEFORE
AsyncStream::with_channel(move |stream_sender| {
    if let Some(runtime) = crate::runtime::shared_runtime() {
        runtime.spawn(async move {
            // async logic here
        });
    }
})

// AFTER
Box::pin(crate::async_stream::spawn_stream(move |stream_sender| async move {
    // async logic here directly - spawn_stream handles the spawning
}))
```

## IMPLEMENTATION CHECKLIST

- [ ] Update imports in `cli/runner.rs`
  - Remove `ystream::{AsyncStream, emit, spawn_task}`
  - Add `tokio_stream::StreamExt` and `std::pin::Pin`
  
- [ ] Convert `cli/runner.rs` line 223-241
  - Remove `spawn_task` wrapper
  - Replace `emit!` with `sender.send()`
  - Replace `.try_next()` with `.next().await`
  - Wrap in `Box::pin()`

- [ ] Update imports in `builders/agent_role.rs`
  - Remove `use ystream::AsyncStream`
  - Add `use std::pin::Pin` and `use tokio_stream::Stream`

- [ ] Convert all return types in `builders/agent_role.rs`
  - Change `AsyncStream<T>` to `Pin<Box<dyn Stream<Item = T> + Send>>`

- [ ] Convert all `AsyncStream::with_channel` calls in `builders/agent_role.rs`
  - Replace with `Box::pin(crate::async_stream::spawn_stream(...))`
  - Remove nested `runtime.spawn()` wrappers
  - Ensure all closures are `async move`

- [ ] Run `cargo check` to verify compilation
- [ ] Verify no `AsyncStream` references remain in either file
- [ ] Verify no `ystream` imports remain

## IMPORTANT NOTES

### What NOT to Change

**std::sync::mpsc in runner.rs (lines 59-60):**
```rust
use std::sync::mpsc::channel;
let (shutdown_tx, shutdown_rx) = channel();
```
This is **CORRECT** - used for Ctrl+C signal handling in a blocking context. Do NOT convert to tokio::sync::mpsc.

### Stream Consumption Pattern

When consuming streams, use:
```rust
while let Some(item) = stream.next().await {
    // process item
}
```

NOT:
```rust
while let Some(item) = stream.try_next() {  // WRONG - no .await
    // ...
}
```

### Error Handling

Maintain existing error handling patterns. The `let _ = sender.send(...)` pattern is correct for unbounded channels (send never fails unless receiver is dropped).

## DEFINITION OF DONE

- ✅ All `AsyncStream` usage removed from `cli/runner.rs`
- ✅ All `AsyncStream` usage removed from `builders/agent_role.rs`
- ✅ All `ystream` imports removed from both files
- ✅ All return types updated to `Pin<Box<dyn Stream<Item = T> + Send>>`
- ✅ All stream consumption uses `.next().await` pattern
- ✅ All stream emission uses `sender.send(item)` pattern
- ✅ Project compiles with `cargo check` (0 errors)
- ✅ No stubs or placeholder code introduced
- ✅ std::sync::mpsc for Ctrl+C handling remains unchanged

## VERIFICATION COMMANDS

```bash
# Verify no AsyncStream references remain
rg "AsyncStream" packages/candle/src/cli/runner.rs
rg "AsyncStream" packages/candle/src/builders/agent_role.rs

# Verify no ystream imports remain
rg "use ystream" packages/candle/src/cli/runner.rs
rg "use ystream" packages/candle/src/builders/agent_role.rs

# Verify compilation
cd packages/candle && cargo check

# Check for proper patterns
rg "Box::pin\(crate::async_stream::spawn_stream" packages/candle/src/cli/runner.rs
rg "Box::pin\(crate::async_stream::spawn_stream" packages/candle/src/builders/agent_role.rs
```

## REFERENCES

- **async_stream helper:** [`packages/candle/src/async_stream.rs`](../packages/candle/src/async_stream.rs)
- **Previous conversion examples:** YSTREAM_M (chat core modules)
  - [`packages/candle/src/domain/chat/config.rs`](../packages/candle/src/domain/chat/config.rs)
  - [`packages/candle/src/domain/chat/macros.rs`](../packages/candle/src/domain/chat/macros.rs)
  - [`packages/candle/src/domain/chat/formatting.rs`](../packages/candle/src/domain/chat/formatting.rs)
