# ASYNC_THREADS: Convert std::thread::spawn to tokio::spawn

## OBJECTIVE
Replace all std::thread::spawn with tokio::spawn for 100% tokio async runtime.
No OS thread spawning - everything should use tokio's async task scheduler.

## CONVERSION PATTERN

```rust
// BEFORE (synchronous thread spawning):
std::thread::spawn(move || {
    // blocking work
    some_sync_function();
});

// AFTER (tokio async task):
tokio::spawn(async move {
    // async work
    some_async_function().await;
});
```

## FILES TO CONVERT (9 files)

### 1. src/runtime/mod.rs - Line 27
**Context:** Runtime initialization
```rust
// CURRENT:
std::thread::spawn(|| Runtime::new().ok())

// REQUIRED:
tokio::spawn(async { Runtime::new_async().await.ok() })
```

### 2. src/agent/prompt.rs - Line 128
**Context:** Prompt processing
```rust
// CURRENT:
std::thread::spawn(move || {
    // prompt processing logic
});

// REQUIRED:
tokio::spawn(async move {
    // async prompt processing
});
```

### 3. src/pool/core/memory_governor.rs - Line 431
**Context:** Memory governor background task
```rust
// CURRENT:
std::thread::spawn(move || {
    // memory monitoring loop
});

// REQUIRED:
tokio::spawn(async move {
    // async memory monitoring
});
```

### 4. src/cli/runner.rs - Line 74
**Context:** CLI command runner
```rust
// CURRENT:
std::thread::spawn(move || {
    // command execution
});

// REQUIRED:
tokio::spawn(async move {
    // async command execution
});
```

### 5. src/domain/agent/core.rs - Line 327
**Context:** Agent background processing
```rust
// CURRENT:
std::thread::spawn(move || {
    // agent work
});

// REQUIRED:
tokio::spawn(async move {
    // async agent work
});
```

### 6. src/domain/chat/config.rs - Line 853
**Context:** Chat configuration updates
```rust
// CURRENT:
std::thread::spawn(move || {
    // config update logic
});

// REQUIRED:
tokio::spawn(async move {
    // async config update
});
```

### 7. src/domain/memory/cache.rs - Line 58
**Context:** Cache maintenance
```rust
// CURRENT:
std::thread::spawn(|| {
    // cache cleanup
});

// REQUIRED:
tokio::spawn(async {
    // async cache cleanup
});
```

### 8. src/domain/chat/realtime/connection.rs - Line 301
**Context:** Realtime connection handler
```rust
// CURRENT:
std::thread::spawn(move || {
    // connection handling
});

// REQUIRED:
tokio::spawn(async move {
    // async connection handling
});
```

### 9. src/domain/chat/realtime/typing.rs - Line 315
**Context:** Typing indicator updates
```rust
// CURRENT:
std::thread::spawn(move || {
    // typing updates
});

// REQUIRED:
tokio::spawn(async move {
    // async typing updates
});
```

## NOTES

- All functions called within tokio::spawn must be async or wrapped in async blocks
- If blocking I/O is necessary, use tokio::fs or tokio::io instead
- Ensure proper .await calls on async operations
- Update function signatures to return impl Future or use async fn

## VERIFICATION

```bash
# Should return 0
cd packages/candle && rg "std::thread::spawn" --type rust src/ | wc -l

# Should compile
cargo check --package paraphym_candle
```

## DEFINITION OF DONE
- Zero std::thread::spawn in codebase
- All background tasks use tokio::spawn
- All async functions properly use .await
- Code compiles successfully
- 100% tokio async runtime
