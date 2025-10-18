# ASYNC_3: Fix Sync Locks in memory/ Directory

## OBJECTIVE

Investigate and fix all sync lock calls without `.await` in the `src/memory/` directory. Convert sync locks to async where needed, or document legitimate exceptions.

---

## BACKGROUND

During async verification, the following files in `src/memory/` were found to have `.lock()`, `.read()`, or `.write()` calls without `.await`:

1. `memory/core/cognitive_queue.rs` - sync locks
2. `memory/monitoring/performance.rs` - sync locks  
3. `memory/transaction/transaction_manager.rs` - sync locks
4. `memory/core/systems/episodic.rs` - sync locks

Each needs manual investigation to determine:
- Is it in an async context? → Must convert to `tokio::sync` with `.await`
- Is it in a sync-only context? → `std::sync` is acceptable (document why)
- Is the lock already async but missing `.await`? → Add `.await`

---

## SUBTASKS

### SUBTASK 1: Fix `memory/core/cognitive_queue.rs`

**Find lock calls**:
```bash
grep -n "\.lock()\|\.read()\|\.write()" src/memory/core/cognitive_queue.rs
```

**For each match**:
1. Read surrounding code to determine context
2. Check if containing function is `async fn`
3. If async:
   - Check if lock is `tokio::sync` or `std::sync`
   - If `std::sync` → Change import to `tokio::sync`
   - Add `.await` to lock call
   - Make sure function signature is async
4. If sync:
   - Add comment: `// SYNC CONTEXT: [explain why]`
   - Keep `std::sync`

**Verification**:
```bash
grep -n "\.lock()\|\.read()\|\.write()" src/memory/core/cognitive_queue.rs -A 1 | grep -v "await"
```

---

### SUBTASK 2: Fix `memory/monitoring/performance.rs`

**Find lock calls**:
```bash
grep -n "\.write()" src/memory/monitoring/performance.rs
```

**Investigation required**:
- Multiple `.write()` calls found
- Check if these are on `tokio::sync::RwLock` or `std::sync::RwLock`
- Check context of each call

**Fix pattern**:
```rust
// ❌ BEFORE
let mut guard = self.something.write().unwrap();

// ✅ AFTER (if in async context)
let mut guard = self.something.write().await;
```

**Verification**:
```bash
grep -n "\.write()" src/memory/monitoring/performance.rs -A 1 | grep -v "await"
```

---

### SUBTASK 3: Fix `memory/transaction/transaction_manager.rs`

**Find lock calls**:
```bash
grep -n "\.write()" src/memory/transaction/transaction_manager.rs
```

**Investigation required**:
- Check what type is being locked
- Determine async vs sync context
- Convert if needed

**Verification**:
```bash
grep -n "\.write()" src/memory/transaction/transaction_manager.rs -A 1 | grep -v "await"
```

---

### SUBTASK 4: Fix `memory/core/systems/episodic.rs`

**Find lock calls**:
```bash
grep -n "\.write()" src/memory/core/systems/episodic.rs
```

**Investigation required**:
- Check lock type and context
- Fix if in async context

**Verification**:
```bash
grep -n "\.write()" src/memory/core/systems/episodic.rs -A 1 | grep -v "await"
```

---

### SUBTASK 5: Check `memory/query/query_monitor.rs`

**Find lock calls**:
```bash
grep -n "\.read()" src/memory/query/query_monitor.rs
```

**Investigation required**:
- Was found during verification
- Check if needs fixing

**Verification**:
```bash
grep -n "\.read()" src/memory/query/query_monitor.rs -A 1 | grep -v "await"
```

---

### SUBTASK 6: Final verification across memory/ directory

**Verification command**:
```bash
cd /Volumes/samsung_t9/paraphym/packages/candle

# Find all lock calls in memory/ directory
find src/memory -name "*.rs" -exec grep -Hn "\.lock()\|\.read()\|\.write()" {} \; | grep -v "await" | grep -v "//" | head -20
```

**Expected**: Only documented exceptions or sync-only contexts

---

## DEFINITION OF DONE

- ✅ All 5 files investigated
- ✅ All async context locks use `tokio::sync` with `.await`
- ✅ All sync-only contexts documented with comments
- ✅ No unawaited locks in async contexts
- ✅ `cargo check -p paraphym_candle` passes without errors

---

## CONSTRAINTS

- ❌ **NO TESTS**: Do not write unit tests, integration tests, or test code
- ❌ **NO BENCHMARKS**: Do not write benchmark code
- ✅ **Focus on src/ only**: Only modify source files
- ✅ **Document exceptions**: If keeping `std::sync`, add clear comment

---

## RESEARCH NOTES

### How to Identify Async Context

**Async context**:
```rust
async fn my_function() {  // ← async keyword
    let guard = lock.lock();  // ❌ Missing .await
}
```

**Sync context**:
```rust
fn my_function() {  // ← No async keyword
    let guard = lock.lock();  // ✅ OK if using std::sync
}
```

### Legitimate std::sync Usage

Acceptable when:
1. Function is NOT async
2. Function is NEVER called from async context
3. Inside `spawn_blocking` or worker thread
4. Static initialization in `LazyLock`

**Always add comment explaining why!**

### Common Fix Pattern

```rust
// Import change
use std::sync::RwLock;  // ❌
use tokio::sync::RwLock;  // ✅

// Usage change
let guard = rwlock.write().unwrap();  // ❌
let guard = rwlock.write().await;  // ✅
```

---

## FILE LOCATIONS

**Files to investigate**:
- `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/core/cognitive_queue.rs`
- `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/monitoring/performance.rs`
- `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/transaction/transaction_manager.rs`
- `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/core/systems/episodic.rs`
- `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/query/query_monitor.rs`

---

## ESTIMATED TIME

**1.5-2 hours**:
- 20 min per file × 5 files = 100 min
- 20 min: Final verification
