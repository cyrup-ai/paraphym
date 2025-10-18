# ASYNC_2: Convert Memory Tool Sync Mutex to Async

## OBJECTIVE

Convert `src/domain/memory/tool.rs` from using `std::sync::Mutex` to `tokio::sync::Mutex` to prevent blocking tokio worker threads when accessing the global result queue in async contexts.

---

## BACKGROUND

**File**: `src/domain/memory/tool.rs`

The file currently uses `std::sync::Mutex` for wrapping the receiver in a static global queue. This is problematic because:

1. **Line 15**: Imports `std::sync::Mutex`
2. **Line 31**: Type definition uses sync Mutex
3. **Line 47**: Static initialization creates sync Mutex
4. **Line 202**: `dequeue_result()` calls `.lock()` without `.await`

When `.lock()` is called on a `std::sync::Mutex` in an async context, it **blocks the entire tokio worker thread** if the lock is contended, starving other async tasks.

---

## SUBTASKS

### SUBTASK 1: Change Mutex import

**Location**: Line 15

**Change**:
```rust
// ❌ BEFORE
use std::sync::Mutex;

// ✅ AFTER
use tokio::sync::Mutex;
```

**Why**: Use async-aware Mutex that yields instead of blocking

---

### SUBTASK 2: Update type definition

**Location**: Line 29-32

**Current**:
```rust
type MemoryNodeQueue = (
    mpsc::UnboundedSender<MemoryNode>,
    Arc<Mutex<mpsc::UnboundedReceiver<MemoryNode>>>,
);
```

**No change needed** - just verify it now uses `tokio::sync::Mutex` from the import

**Why**: Type definition references `Mutex` by name, which now resolves to tokio's version

---

### SUBTASK 3: Update static initialization

**Location**: Lines 45-48

**Current**:
```rust
static RESULT_QUEUE: LazyLock<MemoryNodeQueue> = LazyLock::new(|| {
    let (sender, receiver) = mpsc::unbounded_channel();
    (sender, Arc::new(Mutex::new(receiver)))
});
```

**No change needed** - just verify it now uses `tokio::sync::Mutex` from the import

**Why**: `Mutex::new()` now resolves to tokio's version

---

### SUBTASK 4: Make `dequeue_result()` async

**Location**: Lines 197-207

**Current**:
```rust
#[inline]
#[must_use]
pub fn dequeue_result() -> Option<MemoryNode> {
    let (_, receiver) = &*RESULT_QUEUE;
    if let Ok(mut rx) = receiver.lock() {
        rx.try_recv().ok()
    } else {
        None
    }
}
```

**Changes Required**:
```rust
#[inline]
#[must_use]
pub async fn dequeue_result() -> Option<MemoryNode> {
    let (_, receiver) = &*RESULT_QUEUE;
    let mut rx = receiver.lock().await;  // ✅ Add .await
    rx.try_recv().ok()
}
```

**Why**: 
- `tokio::sync::Mutex::lock()` returns a Future that must be awaited
- No `Ok()/Err()` handling needed - tokio Mutex doesn't poison

---

### SUBTASK 5: Find and update all callers of `dequeue_result()`

**Command to find callers**:
```bash
cd /Volumes/samsung_t9/paraphym/packages/candle
grep -rn "dequeue_result()" src/ --include="*.rs"
```

**For each caller**:
1. Ensure containing function is async (or make it async)
2. Change `dequeue_result()` to `dequeue_result().await`

---

### SUBTASK 6: Verify no sync Mutex remains

**Verification commands**:
```bash
cd /Volumes/samsung_t9/paraphym/packages/candle

# Should only show tokio::sync::Mutex
grep -n "use.*Mutex" src/domain/memory/tool.rs

# Should show no std::sync::Mutex
grep -n "std::sync::Mutex" src/domain/memory/tool.rs
```

**Expected**: 
- First command shows `use tokio::sync::Mutex;`
- Second command shows no matches

---

## DEFINITION OF DONE

- ✅ Line 15 imports `tokio::sync::Mutex`
- ✅ `dequeue_result()` is `pub async fn`
- ✅ Line 202 contains `receiver.lock().await`
- ✅ All callers updated to `.await` the method
- ✅ No `std::sync::Mutex` in the file
- ✅ `cargo check -p paraphym_candle` passes without errors

---

## CONSTRAINTS

- ❌ **NO TESTS**: Do not write unit tests, integration tests, or test code
- ❌ **NO BENCHMARKS**: Do not write benchmark code
- ✅ **Focus on src/ only**: Only modify source files

---

## RESEARCH NOTES

### tokio::sync::Mutex vs std::sync::Mutex

**std::sync::Mutex**:
- Blocks OS thread when lock is contended
- Returns `Result<MutexGuard, PoisonError>` (can poison on panic)
- OK for pure sync code, NOT OK in async contexts

**tokio::sync::Mutex**:
- Yields to tokio scheduler when lock is contended
- Returns `MutexGuard` directly (no Result, no poison)
- Designed for async contexts
- Slightly slower than std::sync but prevents thread starvation

### LazyLock with tokio::sync::Mutex

This is safe because:
- LazyLock initializes once at first access
- After initialization, only the Mutex is used (which is now async-aware)
- The initialization itself happens in whatever context first accesses it

---

## FILE LOCATIONS

**Primary file**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/tool.rs`

**Related documentation**:
- [tokio::sync::Mutex docs](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html)
- [Tokio shared state tutorial](https://tokio.rs/tokio/tutorial/shared-state)

---

## ESTIMATED TIME

**1 hour**:
- 10 min: Change import and verify type/static
- 10 min: Make `dequeue_result()` async
- 20 min: Find and update all callers
- 20 min: Verification and testing
