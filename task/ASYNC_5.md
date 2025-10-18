# ASYNC_5: Fix Sync Locks in domain/ Directory

## OBJECTIVE

Investigate and fix all sync lock calls without `.await` in the `src/domain/` directory. Convert sync locks to async where needed, or document legitimate exceptions.

---

## BACKGROUND

During async verification, the following files in `src/domain/` were found to have `.lock()`, `.read()`, or `.write()` calls without `.await`:

1. `domain/chat/commands/mod.rs` - sync locks (`.read()` and `.write()`)
2. `domain/embedding/service.rs` - sync locks (`.lock()`)

These are critical domain modules that likely have async APIs, so sync locks would block async operations.

---

## SUBTASKS

### SUBTASK 1: Fix `domain/chat/commands/mod.rs`

**Find lock calls**:
```bash
grep -n "\.read()\|\.write()" src/domain/chat/commands/mod.rs
```

**Investigation required**:
- Found `.read()` and `.write()` calls without `.await`
- Likely a global command executor using RwLock
- Check if it's `std::sync::RwLock` or `tokio::sync::RwLock`

**Pattern to look for**:
```rust
// Common pattern
static CANDLE_COMMAND_EXECUTOR: LazyLock<RwLock<...>> = ...;

// Usage
if let Ok(mut writer) = CANDLE_COMMAND_EXECUTOR.write() {
    // ❌ Missing .await if RwLock is tokio::sync
}
```

**Fix if in async context**:
```rust
// Change import
use std::sync::RwLock;  // ❌
use tokio::sync::RwLock;  // ✅

// Change usage
if let Ok(writer) = lock.write() {  // ❌ Old pattern
let mut writer = lock.write().await;  // ✅ New pattern
```

**Note**: `tokio::sync::RwLock` doesn't return `Result`, so no `if let Ok` needed

**Verification**:
```bash
grep -n "\.read()\|\.write()" src/domain/chat/commands/mod.rs -A 1 | grep -v "await"
```

---

### SUBTASK 2: Fix `domain/embedding/service.rs`

**Find lock calls**:
```bash
grep -n "\.lock()" src/domain/embedding/service.rs
```

**Investigation required**:
- Found `.lock()` call without `.await`
- Embedding service likely has cache protected by lock
- Check if lock is on async or sync Mutex

**Pattern to look for**:
```rust
// Cache pattern
struct EmbeddingCache {
    cache: Arc<Mutex<HashMap<String, Vec<f32>>>>,
}

// Usage
if let Ok(mut receiver) = self.receiver.lock() {
    // ❌ Missing .await if Mutex is tokio::sync
}
```

**Fix if in async context**:
```rust
// Change import
use std::sync::Mutex;  // ❌
use tokio::sync::Mutex;  // ✅

// Change usage
if let Ok(cache) = self.cache.lock() {  // ❌ Old pattern
let cache = self.cache.lock().await;  // ✅ New pattern
```

**Verification**:
```bash
grep -n "\.lock()" src/domain/embedding/service.rs -A 1 | grep -v "await"
```

---

### SUBTASK 3: Check for cascading async propagation

After fixing the locks, check if:
1. Functions containing the locks are now async
2. All callers of those functions are updated
3. Async propagates correctly up the call chain

**Commands**:
```bash
# Find functions in commands/mod.rs
grep -n "^pub fn\|^pub async fn" src/domain/chat/commands/mod.rs

# Find functions in embedding/service.rs
grep -n "^pub fn\|^pub async fn" src/domain/embedding/service.rs
```

If you made a function async, find its callers:
```bash
grep -rn "function_name(" src/ --include="*.rs"
```

---

### SUBTASK 4: Verify no sync locks remain in domain/

**Verification command**:
```bash
cd /Volumes/samsung_t9/paraphym/packages/candle

# Find all lock calls in domain/ directory (excluding domain/memory/pool.rs - fixed in ASYNC_1)
find src/domain -name "*.rs" ! -path "*/memory/pool.rs" -exec grep -Hn "\.lock()\|\.read()\|\.write()" {} \; | grep -v "await" | grep -v "//"
```

**Expected**: No matches or only documented exceptions

---

## DEFINITION OF DONE

- ✅ Both files investigated and fixed
- ✅ All async context locks use `tokio::sync` with `.await`
- ✅ Async propagation complete (all callers updated)
- ✅ No unawaited locks in async contexts in `domain/` directory
- ✅ `cargo check -p paraphym_candle` passes without errors

---

## CONSTRAINTS

- ❌ **NO TESTS**: Do not write unit tests, integration tests, or test code
- ❌ **NO BENCHMARKS**: Do not write benchmark code
- ✅ **Focus on src/ only**: Only modify source files
- ✅ **Document exceptions**: If keeping `std::sync`, add clear comment

---

## RESEARCH NOTES

### Command Executor Pattern

Command executors often use:
```rust
static EXECUTOR: LazyLock<RwLock<Executor>> = LazyLock::new(|| {
    RwLock::new(Executor::new())
});

pub fn execute_command(cmd: Command) -> Result<()> {
    let mut executor = EXECUTOR.write()?;  // ❌ Sync lock in async context
    executor.execute(cmd)
}
```

**Fix**: Use `tokio::sync::RwLock` and make `execute_command` async

### Embedding Cache Pattern

Embedding services cache results to avoid recomputation:
```rust
struct EmbeddingService {
    cache: Arc<Mutex<HashMap<String, Embedding>>>,
}

impl EmbeddingService {
    pub fn get_cached(&self, text: &str) -> Option<Embedding> {
        let cache = self.cache.lock()?;  // ❌ Sync lock in async context
        cache.get(text).cloned()
    }
}
```

**Fix**: Use `tokio::sync::Mutex` and make `get_cached` async

### LazyLock with tokio::sync

Using `tokio::sync` locks in `LazyLock` is safe:
- Initialization happens once
- After init, only the async lock is used
- No blocking during initialization

---

## FILE LOCATIONS

**Files to investigate**:
- `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/chat/commands/mod.rs`
- `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/embedding/service.rs`

**Related**:
- `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/pool.rs` (fixed in ASYNC_1)

---

## ESTIMATED TIME

**1 hour**:
- 25 min: Fix commands/mod.rs
- 25 min: Fix embedding/service.rs
- 10 min: Check async propagation
- 10 min: Final verification
