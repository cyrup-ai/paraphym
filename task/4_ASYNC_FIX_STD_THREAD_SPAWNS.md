# Task: Replace std::thread::spawn with tokio::spawn

## Problem
**11 locations using `std::thread::spawn` instead of `tokio::spawn`**

These create OS threads outside tokio's control, causing:
- Thread proliferation
- No cancellation support
- Can't `.await` in spawned context
- Resource waste (OS threads are expensive)
- No integration with tokio runtime

## Locations Found

### 1. 🔥 LLaVA Vision Model Thread
**File**: `src/capability/vision/llava.rs:215`
```rust
thread::spawn(move || {
    Self::model_thread_with_config(model, tokenizer, config, context);
});
```
**Issue**: Dedicated thread using `rt.block_on()` for async work
**Fix**: Already has dedicated task file: `ASYNC_FIX_LLAVA_BLOCKING_THREAD.md`
**Priority**: 🔥 CRITICAL

### 2. 🔴 Memory Vector Search
**File**: `src/memory/vector/vector_search.rs:592`
```rust
let handle = thread::spawn(move || {
    // Vector search computation
    let result = search.search_by_embedding(&embedding, options);
    let _ = sender.send((index, result));
});
```
**Issue**: Creating threads for parallel search instead of using async memory system
**Fix**: Should use async memory system which already has proper concurrency
**Action**: Integrate with async MemoryManager, use `tokio::spawn` for parallel work
**Priority**: 🔴 HIGH - Bypasses memory system architecture

### 3. Agent Prompt Driver
**File**: `src/agent/prompt.rs:128`
```rust
std::thread::spawn(move || {
    // Drive completion streams
    let mut completion_stream = self.agent.completion(...).send();
    // ...
});
```
**Issue**: Should be tokio task driving async streams
**Priority**: 🔴 HIGH

### 4. CLI Runner
**File**: `src/cli/runner.rs:74`
```rust
std::thread::spawn(move || {
    // CLI execution
});
```
**Issue**: CLI logic should be async task
**Priority**: 🟡 MEDIUM

### 5. Memory Cache
**File**: `src/domain/memory/cache.rs:58`
```rust
std::thread::spawn(|| {
    loop {
        std::thread::sleep(Duration::from_millis(100));  // ❌ Blocking sleep
        // Cache cleanup
    }
});
```
**Issue**: Blocking sleep in cleanup loop
**Priority**: 🟡 MEDIUM

### 6. Chat Config
**File**: `src/domain/chat/config.rs:853`
```rust
std::thread::spawn(move || {
    // Config watching/updates
});
```
**Issue**: Should be async task with tokio::fs::watch
**Priority**: 🟡 MEDIUM

### 7. 🔴 Realtime Typing Cleanup
**File**: `src/domain/chat/realtime/typing.rs:315`
```rust
std::thread::spawn(move || {
    loop {
        std::thread::sleep(cleanup_interval);  // ❌ Blocking sleep
        // Expire typing indicators
    }
});
```
**Issue**: Blocking sleep in hot cleanup loop
**Priority**: 🔴 HIGH

### 8. 🔴 Realtime Connection Monitor
**File**: `src/domain/chat/realtime/connection.rs:301`
```rust
std::thread::spawn(move || {
    loop {
        std::thread::sleep(Duration::from_secs(1));  // ❌ Blocking sleep
        // Monitor connections
    }
});
```
**Issue**: Blocking sleep monitoring connections
**Priority**: 🔴 HIGH

### 9. Agent Core
**File**: `src/domain/agent/core.rs:327`
```rust
std::thread::spawn(move || {
    // Agent execution
});
```
**Issue**: Should be tokio task
**Priority**: 🟡 MEDIUM

### 10. 🔥 Pool Memory Governor
**File**: `src/pool/core/memory_governor.rs:431`
```rust
std::thread::spawn(move || {
    loop {
        std::thread::sleep(interval);  // ❌ Blocking sleep
        // Monitor memory pressure
        // Handle critical pressure
    }
});
```
**Issue**: Critical memory monitoring with blocking sleep
**Priority**: 🔥 CRITICAL - This is pool infrastructure

### 11. 🔥 Runtime Bootstrap (WRONG PATTERN)
**File**: `src/runtime/mod.rs:27`
```rust
// Comment says: "Create runtime in separate thread to avoid nested runtime issues"
std::thread::spawn(|| Runtime::new().ok())
    .join()
    .ok()
    .flatten()
```
**Issue**: Creating tokio Runtime in std::thread to "avoid nested runtime issues"
**Why This is Wrong**:
- If you're already in tokio context, use `Handle::current()` 
- If you need global runtime, create it directly with `Runtime::new()`
- Spawning a thread to create a runtime is an anti-pattern
- The "nested runtime" error means the code calling this is already in a runtime

**Fix**: 
- If this is app initialization: Create runtime directly
- If called from async context: Use `Handle::current()` instead
- Remove the thread spawn entirely

**Priority**: 🔥 CRITICAL - Fundamentally wrong pattern

## Solution Pattern

**Before**:
```rust
std::thread::spawn(move || {
    loop {
        std::thread::sleep(Duration::from_secs(1));
        // Do work
    }
});
```

**After**:
```rust
tokio::spawn(async move {
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        // Do work
    }
});
```

## When std::thread::spawn is ACCEPTABLE

**NEVER.** In a tokio async system, `std::thread::spawn` is NEVER acceptable.

**ALL cases must use `tokio::spawn`:**
- ✅ I/O operations: Use `tokio::spawn` with async I/O
- ✅ CPU-intensive work: Use `tokio::task::spawn_blocking` 
- ✅ External sync libraries: Wrap in `tokio::task::spawn_blocking`

**NO EXCEPTIONS.** Every `std::thread::spawn` is a bug that bypasses tokio's scheduler.

## Steps

For each location (except vector search and runtime):
1. Change `std::thread::spawn` → `tokio::spawn`
2. Change closure to `async move`
3. Replace `std::thread::sleep` with `tokio::time::sleep().await`
4. If joining threads, use `JoinHandle::await` instead of `.join()`
5. Test functionality

## Priority Summary

- 🔥 **CRITICAL** (3): LLaVA thread, Memory governor, Runtime bootstrap
- 🔴 **HIGH** (4): Agent prompt, Typing cleanup, Connection monitor, Vector search
- 🟡 **MEDIUM** (4): CLI runner, Cache, Config, Agent core

## Status
⏳ TODO - **ALL 11 locations must be fixed**
