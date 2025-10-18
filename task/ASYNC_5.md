# ASYNC_5: Fix Sync Locks in domain/ Directory

## OBJECTIVE

Convert all `std::sync` locks to `tokio::sync` locks in async contexts within the `src/domain/` directory. This task eliminates blocking operations that can starve the tokio runtime's thread pool.

---

## RESEARCH FINDINGS

**Search command used:**
```bash
cd /Volumes/samsung_t9/paraphym/packages/candle
find src/domain -name "*.rs" -exec grep -Hn "\.lock()\|\.read()\|\.write()" {} \; | grep -v "await" | grep -v "//"
```

**Results - 4 sync lock calls found:**
1. ✅ `src/domain/memory/pool.rs:48` - `receiver.lock()` in `async fn acquire()`
2. ✅ `src/domain/chat/commands/mod.rs:37` - `CANDLE_COMMAND_EXECUTOR.write()` in sync fn called from async
3. ✅ `src/domain/chat/commands/mod.rs:45` - `CANDLE_COMMAND_EXECUTOR.read()` in sync fn called from async  
4. ✅ `src/domain/embedding/service.rs:75` - `receiver.lock()` in sync fn potentially called from async

---

## FILE 1: domain/memory/pool.rs

### Current Code (Lines 1-48)

**Imports (Line 2):**
```rust
use std::sync::{Arc, Mutex};  // ❌ Using std::sync::Mutex
```

**Struct (Line 9):**
```rust
receiver: Arc<Mutex<mpsc::UnboundedReceiver<MemoryNode>>>,  // ❌ std::sync::Mutex
```

**Usage (Line 48 in async fn):**
```rust
pub async fn acquire(&self) -> Result<PooledMemoryNode<'_>, super::primitives::MemoryError> {
    let mut node = if let Ok(mut receiver) = self.receiver.lock() {  // ❌ Blocks tokio thread
        receiver.try_recv().unwrap_or_else(|_| {
            // ...
        })
    } else {
        // ...
    };
```

### Required Changes

**1. Change imports (Line 2):**
```rust
use std::sync::Arc;
use tokio::sync::Mutex;  // ✅ Use tokio::sync::Mutex
```

**2. Change struct field (Line 9):**
```rust
receiver: Arc<Mutex<mpsc::UnboundedReceiver<MemoryNode>>>,  // ✅ Now tokio::sync::Mutex
```

**3. Update usage (Line 48):**
```rust
pub async fn acquire(&self) -> Result<PooledMemoryNode<'_>, super::primitives::MemoryError> {
    let mut receiver = self.receiver.lock().await;  // ✅ Added .await
    let mut node = receiver.try_recv().unwrap_or_else(|_| {
        // ... same fallback logic
    });
```

**Note:** `tokio::sync::Mutex` doesn't return `Result`, so no `if let Ok` needed.

**4. Check release() method:**

Look for similar pattern in `release()` or `return_to_pool()` methods - likely around line 70-90. Apply same conversion.

---

## FILE 2: domain/chat/commands/mod.rs

### Current Code (Lines 14-48)

**Imports (Line 14):**
```rust
use std::sync::{Arc, RwLock};  // ❌ Using std::sync::RwLock
```

**Static (Line 30):**
```rust
static CANDLE_COMMAND_EXECUTOR: std::sync::LazyLock<Arc<RwLock<Option<CommandExecutor>>>> =
    std::sync::LazyLock::new(|| Arc::new(RwLock::new(None)));  // ❌ std::sync::RwLock
```

**Initialize function (Line 37):**
```rust
pub fn initialize_candle_command_executor(context: &CommandExecutionContext) {
    let executor = CommandExecutor::with_context(context);
    if let Ok(mut writer) = CANDLE_COMMAND_EXECUTOR.write() {  // ❌ Blocks if called from async
        *writer = Some(executor);
    }
}
```

**Get function (Line 44-48):**
```rust
pub fn get_candle_command_executor() -> Option<CommandExecutor> {
    CANDLE_COMMAND_EXECUTOR
        .read()  // ❌ Line 45 - Blocks if called from async
        .ok()
        .and_then(|guard| guard.clone())
}
```

### Required Changes

**1. Change imports (Line 14):**
```rust
use std::sync::Arc;
use tokio::sync::RwLock;  // ✅ Use tokio::sync::RwLock
```

**2. Change static (Line 30):**
```rust
static CANDLE_COMMAND_EXECUTOR: std::sync::LazyLock<Arc<RwLock<Option<CommandExecutor>>>> =
    std::sync::LazyLock::new(|| Arc::new(RwLock::new(None)));  // ✅ Now tokio::sync::RwLock
```

**3. Make initialize_candle_command_executor async (Line 36):**
```rust
pub async fn initialize_candle_command_executor(context: &CommandExecutionContext) {
    let executor = CommandExecutor::with_context(context);
    let mut writer = CANDLE_COMMAND_EXECUTOR.write().await;  // ✅ Added .await
    *writer = Some(executor);
}
```

**4. Make get_candle_command_executor async (Line 44):**
```rust
pub async fn get_candle_command_executor() -> Option<CommandExecutor> {
    let guard = CANDLE_COMMAND_EXECUTOR.read().await;  // ✅ Added .await
    guard.clone()
}
```

**5. Update all callers (Lines 69, 82, 105, 128, 151):**

Search for calls to these functions:
```bash
grep -n "get_candle_command_executor()" src/domain/chat/commands/mod.rs
```

Each call like:
```rust
if let Some(executor) = get_candle_command_executor() {  // ❌ Old
```

Becomes:
```rust
if let Some(executor) = get_candle_command_executor().await {  // ✅ New
```

**Expected matches around lines:** 69, 82, 105, 128, 151

---

## FILE 3: domain/embedding/service.rs

### Current Code (Lines 7, 50, 75)

**Imports (Line 7):**
```rust
use std::sync::{Arc, Mutex};  // ❌ Using std::sync::Mutex
```

**Struct (Line 50):**
```rust
pub struct EmbeddingPool {
    sender: mpsc::UnboundedSender<Vec<f32>>,
    receiver: Arc<Mutex<mpsc::UnboundedReceiver<Vec<f32>>>>,  // ❌ std::sync::Mutex
    dimension: usize,
    max_capacity: usize,
}
```

**Usage (Line 75):**
```rust
pub fn acquire(&self) -> Vec<f32> {  // ❌ Should be async
    if let Ok(mut receiver) = self.receiver.lock() {  // ❌ Blocks
        receiver.try_recv().unwrap_or_else(|_| vec![0.0; self.dimension])
    } else {
        vec![0.0; self.dimension]
    }
}
```

### Required Changes

**1. Change imports (Line 7):**
```rust
use std::sync::Arc;
use tokio::sync::Mutex;  // ✅ Use tokio::sync::Mutex
```

**2. Change struct field (Line 50):**
```rust
receiver: Arc<Mutex<mpsc::UnboundedReceiver<Vec<f32>>>>,  // ✅ Now tokio::sync::Mutex
```

**3. Make acquire() async (Line 75):**
```rust
pub async fn acquire(&self) -> Vec<f32> {  // ✅ Made async
    let mut receiver = self.receiver.lock().await;  // ✅ Added .await
    receiver.try_recv().unwrap_or_else(|_| vec![0.0; self.dimension])
}
```

**4. Update callers:**

The `generate_deterministic()` method at line ~138 calls `self.pool.acquire()`:
```rust
pub fn generate_deterministic(&self, content: &str) -> Vec<f32> {  // ❌ Should be async
    let mut embedding = self.pool.acquire();  // ❌ Needs .await
    // ...
}
```

Change to:
```rust
pub async fn generate_deterministic(&self, content: &str) -> Vec<f32> {  // ✅ Made async
    let mut embedding = self.pool.acquire().await;  // ✅ Added .await
    // ...
}
```

**5. Check stats() method:**

If `stats()` method also uses `self.receiver.lock()`, apply same pattern.

---

## VERIFICATION COMMANDS

After making changes, verify no sync locks remain in domain/:

```bash
cd /Volumes/samsung_t9/paraphym/packages/candle

# Find all lock calls without .await (excluding comments)
find src/domain -name "*.rs" -exec grep -Hn "\.lock()\|\.read()\|\.write()" {} \; \
  | grep -v "await" \
  | grep -v "//" \
  | grep -v "blocking"

# Expected: No results (or only documented exceptions)
```

Verify code compiles:
```bash
cargo check -p paraphym_candle
```

---

## DEFINITION OF DONE

- ✅ `domain/memory/pool.rs` - Converted to `tokio::sync::Mutex`, `acquire()` uses `.await`
- ✅ `domain/chat/commands/mod.rs` - Converted to `tokio::sync::RwLock`, made init/get async, updated all 5+ callers
- ✅ `domain/embedding/service.rs` - Converted to `tokio::sync::Mutex`, made `acquire()` and `generate_deterministic()` async
- ✅ All callers updated with `.await`
- ✅ `cargo check -p paraphym_candle` passes
- ✅ Verification command shows no remaining sync locks in domain/

---

## SUMMARY OF CHANGES

| File | Lines | Change |
|------|-------|--------|
| `domain/memory/pool.rs` | 2, 9, 48 | `std::sync::Mutex` → `tokio::sync::Mutex` + `.await` |
| `domain/chat/commands/mod.rs` | 14, 30, 37, 45, 69, 82, 105, 128, 151 | `std::sync::RwLock` → `tokio::sync::RwLock` + make async + `.await` |
| `domain/embedding/service.rs` | 7, 50, 75, ~138 | `std::sync::Mutex` → `tokio::sync::Mutex` + make async + `.await` |

**Total:** 3 files, ~15 lines to modify

---

## WHY THIS MATTERS

**Before (blocking):**
```rust
let receiver = self.receiver.lock();  // Blocks entire tokio thread
// Other async tasks stalled
```

**After (non-blocking):**
```rust
let receiver = self.receiver.lock().await;  // Yields to tokio scheduler
// Other async tasks continue
```

Using `std::sync` locks in async contexts can cause:
- Thread pool starvation (all tokio threads blocked waiting for locks)
- Deadlocks (async task holds lock and can't progress)
- Increased latency (tasks wait for thread availability)

`tokio::sync` locks are designed to:
- Yield to tokio scheduler when lock unavailable
- Allow other tasks to run on same thread
- Prevent thread pool exhaustion
