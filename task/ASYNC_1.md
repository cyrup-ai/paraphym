# ASYNC_1: Fix Memory Pool Unawaited Reset Calls

## OBJECTIVE

Fix critical data leakage bug in memory pool where `MemoryNode::reset()` async method is called without `.await`, causing the reset operation to never execute and metadata to persist between pool reuses.

---

## BACKGROUND

`MemoryNode::reset()` is an async method (returns `Future<Result<()>>`) located at:
- **File**: `src/domain/memory/primitives/node.rs:533`
- **Signature**: `pub async fn reset(&mut self, memory_type: MemoryTypeEnum) -> MemoryResult<()>`

This method uses `tokio::sync::RwLock` internally which requires `.await`.

In `src/domain/memory/pool.rs`, this method is called **without `.await`** in two locations:
- Line 68: `acquire()` method
- Line 110: `initialize()` method

**Result**: The Future is created, immediately dropped, and reset never executes. Metadata persists between pool reuses causing data leakage.

---

## SUBTASKS

### SUBTASK 1: Make `MemoryPool::acquire()` async

**Location**: `src/domain/memory/pool.rs:48-75`

**Current Code**:
```rust
pub fn acquire(&self) -> PooledMemoryNode<'_> {
    let mut node = if let Ok(mut receiver) = self.receiver.lock() {
        // ... node acquisition logic ...
    } else {
        // ... fallback logic ...
    };

    // ❌ WRONG: reset() not awaited
    let _ = node.reset(MemoryType::Working);
    
    PooledMemoryNode {
        node: std::mem::ManuallyDrop::new(node),
        pool: self,
        taken: false,
    }
}
```

**Changes Required**:
1. Change signature to: `pub async fn acquire(&self) -> Result<PooledMemoryNode<'_>, MemoryError>`
2. Change line 68 to: `node.reset(MemoryType::Working).await?;`
3. Wrap return in: `Ok(PooledMemoryNode { ... })`

**Why**: The method must be async to await the reset operation. Returning Result allows proper error propagation.

---

### SUBTASK 2: Make `PooledMemoryNode::initialize()` async

**Location**: `src/domain/memory/pool.rs:106-127`

**Current Code**:
```rust
pub fn initialize(&mut self, content: String, memory_type: MemoryType) {
    if !self.taken {
        // ❌ WRONG: reset() not awaited
        let _ = self.node.reset(memory_type);

        // ... content initialization ...
        
        self.taken = true;
    }
}
```

**Changes Required**:
1. Change signature to: `pub async fn initialize(&mut self, content: String, memory_type: MemoryType) -> Result<(), MemoryError>`
2. Change line 110 to: `self.node.reset(memory_type).await?;`
3. Add `Ok(())` at end of function

**Why**: Same reason - must be async to await reset.

---

### SUBTASK 3: Find and update all callers of `acquire()`

**Command to find callers**:
```bash
cd /Volumes/samsung_t9/paraphym/packages/candle
grep -rn "\.acquire()" src/ --include="*.rs" | grep -v "^Binary"
```

**For each caller**:
1. Ensure containing function is async (or make it async)
2. Change `pool.acquire()` to `pool.acquire().await?`
3. Handle the Result type

---

### SUBTASK 4: Find and update all callers of `initialize()`

**Command to find callers**:
```bash
cd /Volumes/samsung_t9/paraphym/packages/candle
grep -rn "\.initialize(" src/ --include="*.rs" | grep -v "^Binary"
```

**For each caller**:
1. Ensure containing function is async (or make it async)
2. Change `node.initialize(...)` to `node.initialize(...).await?`
3. Handle the Result type

---

### SUBTASK 5: Verify no more unawaited reset calls

**Verification command**:
```bash
cd /Volumes/samsung_t9/paraphym/packages/candle
grep -rn "\.reset(" src/domain/memory/pool.rs | grep -v "await"
```

**Expected**: No matches (or only comments)

---

## DEFINITION OF DONE

- ✅ `acquire()` is `pub async fn` returning `Result<PooledMemoryNode, MemoryError>`
- ✅ `acquire()` contains `node.reset(MemoryType::Working).await?`
- ✅ `initialize()` is `pub async fn` returning `Result<(), MemoryError>`
- ✅ `initialize()` contains `self.node.reset(memory_type).await?`
- ✅ All callers updated to `.await` the methods
- ✅ No unawaited `.reset()` calls in `pool.rs`
- ✅ `cargo check -p paraphym_candle` passes without errors

---

## CONSTRAINTS

- ❌ **NO TESTS**: Do not write unit tests, integration tests, or test code
- ❌ **NO BENCHMARKS**: Do not write benchmark code
- ✅ **Focus on src/ only**: Only modify source files

---

## RESEARCH NOTES

### Error Type
Use existing `MemoryError` from `src/domain/memory/primitives/types.rs`

### Async Propagation
When making a function async, all its callers must also be updated. This may cascade up the call chain. Continue propagating async until you reach:
- Another async function (just add `.await`)
- A tokio runtime boundary (main, spawn, block_on)

### Poison Errors
`tokio::sync` locks don't poison. No need for `.unwrap()` or poison error handling.

---

## FILE LOCATIONS

**Primary file**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/pool.rs`

**Related files**:
- `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/primitives/node.rs` (reset implementation)
- `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/primitives/types.rs` (MemoryError)

---

## ESTIMATED TIME

**1.5-2 hours**:
- 15 min: Update `acquire()` and `initialize()` signatures
- 45 min: Find and update all callers
- 30 min: Handle cascading async propagation
- 15 min: Verification
