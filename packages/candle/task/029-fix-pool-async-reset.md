# Task 029: Fix Async Reset Not Awaited in Memory Pool

## Status: NOT STARTED

## Priority: HIGH

## Objective

Fix critical async-not-awaited bug in [`src/domain/memory/pool.rs`](../src/domain/memory/pool.rs) where the async `reset()` method is called without `.await`, causing metadata clearing to never execute and leading to data leakage between pooled memory node reuses.

---

## Problem Analysis

### Root Cause

The `MemoryNode::reset()` method was made async in Task 027 because it contains:

```rust
// From src/domain/memory/primitives/node.rs:565
let mut meta = self.base_memory.metadata.write().await;  // tokio::sync::RwLock
meta.clear();
```

The `metadata` field uses `tokio::sync::RwLock` which requires `.await`. However, callers in `pool.rs` were not updated:

**Line 68:**
```rust
let _ = node.reset(MemoryType::Working);  // ❌ Future created but never awaited
```

**Line 110:**
```rust
let _ = self.node.reset(memory_type);  // ❌ Future created but never awaited
```

### Why Compiler Didn't Catch It

1. Calling async fn from sync context just returns a Future - no error
2. `let _ =` discards the Future without triggering "unused must_use" warning
3. Code compiles successfully but reset never executes

### Impact

**Data Leakage:** Pooled memory nodes retain metadata HashMap from previous usage, causing information disclosure between operations.

**Good News:** Research shows the memory pool is **NOT currently used** anywhere in the codebase (no callers found), so making these methods async has **zero cascading impact**.

---

## Implementation

### Change 1: Make `MemoryNodePool::acquire()` Async

**File:** [`src/domain/memory/pool.rs`](../src/domain/memory/pool.rs)  
**Line:** 43-76

**Current (BROKEN):**
```rust
#[inline]
#[must_use]
pub fn acquire(&self) -> PooledMemoryNode<'_> {
    let mut node = if let Ok(mut receiver) = self.receiver.lock() {
        receiver.try_recv().unwrap_or_else(|_| {
            let content = MemoryContent::text(String::with_capacity(1024));
            let mut node = MemoryNode::new(MemoryType::Working, content);
            if self.embedding_dimension > 0 {
                let _ = node.set_embedding(vec![0.0; self.embedding_dimension]);
            }
            node
        })
    } else {
        let content = MemoryContent::text(String::with_capacity(1024));
        let mut node = MemoryNode::new(MemoryType::Working, content);
        if self.embedding_dimension > 0 {
            let _ = node.set_embedding(vec![0.0; self.embedding_dimension]);
        }
        node
    };

    // ❌ WRONG: reset() returns Future but not awaited
    let _ = node.reset(MemoryType::Working);

    PooledMemoryNode {
        node: std::mem::ManuallyDrop::new(node),
        pool: self,
        taken: false,
    }
}
```

**Fixed (CORRECT):**
```rust
#[inline]
pub async fn acquire(&self) -> Result<PooledMemoryNode<'_>, super::primitives::MemoryError> {
    let mut node = if let Ok(mut receiver) = self.receiver.lock() {
        receiver.try_recv().unwrap_or_else(|_| {
            let content = MemoryContent::text(String::with_capacity(1024));
            let mut node = MemoryNode::new(MemoryType::Working, content);
            if self.embedding_dimension > 0 {
                let _ = node.set_embedding(vec![0.0; self.embedding_dimension]);
            }
            node
        })
    } else {
        let content = MemoryContent::text(String::with_capacity(1024));
        let mut node = MemoryNode::new(MemoryType::Working, content);
        if self.embedding_dimension > 0 {
            let _ = node.set_embedding(vec![0.0; self.embedding_dimension]);
        }
        node
    };

    // ✅ CORRECT: await the reset and propagate error
    node.reset(MemoryType::Working).await?;

    Ok(PooledMemoryNode {
        node: std::mem::ManuallyDrop::new(node),
        pool: self,
        taken: false,
    })
}
```

**Changes:**
1. `pub fn` → `pub async fn`
2. Return type: `PooledMemoryNode<'_>` → `Result<PooledMemoryNode<'_>, super::primitives::MemoryError>`
3. `let _ = node.reset(...)` → `node.reset(...).await?`
4. `PooledMemoryNode { ... }` → `Ok(PooledMemoryNode { ... })`

---

### Change 2: Make `PooledMemoryNode::initialize()` Async

**File:** [`src/domain/memory/pool.rs`](../src/domain/memory/pool.rs)  
**Line:** 106-127

**Current (BROKEN):**
```rust
#[inline]
pub fn initialize(&mut self, content: String, memory_type: MemoryType) {
    if !self.taken {
        // ❌ WRONG: reset() returns Future but not awaited
        let _ = self.node.reset(memory_type);

        match &mut self.node.base_memory.content {
            MemoryContent::Text(s) => {
                s.clear();
                s.push_str(&content);
            }
            _ => {
                self.node.base_memory.content = MemoryContent::text(content);
            }
        }

        let importance = memory_type.base_importance();
        let _ = self.node.set_importance(importance);
    }
}
```

**Fixed (CORRECT):**
```rust
#[inline]
pub async fn initialize(
    &mut self,
    content: String,
    memory_type: MemoryType,
) -> Result<(), super::primitives::MemoryError> {
    if !self.taken {
        // ✅ CORRECT: await the reset and propagate error
        self.node.reset(memory_type).await?;

        match &mut self.node.base_memory.content {
            MemoryContent::Text(s) => {
                s.clear();
                s.push_str(&content);
            }
            _ => {
                self.node.base_memory.content = MemoryContent::text(content);
            }
        }

        let importance = memory_type.base_importance();
        let _ = self.node.set_importance(importance);
    }
    Ok(())
}
```

**Changes:**
1. `pub fn` → `pub async fn`
2. Return type: none → `Result<(), super::primitives::MemoryError>`
3. `let _ = self.node.reset(...)` → `self.node.reset(...).await?`
4. Add `Ok(())` at the end

---

### Change 3: Update Global Pool Wrapper

**File:** [`src/domain/memory/pool.rs`](../src/domain/memory/pool.rs)  
**Line:** 216-218

**Current:**
```rust
#[inline]
pub fn acquire_pooled_node() -> Option<PooledMemoryNode<'static>> {
    MEMORY_NODE_POOL.get().map(|pool| pool.acquire())
}
```

**Fixed:**
```rust
#[inline]
pub async fn acquire_pooled_node() -> Option<Result<PooledMemoryNode<'static>, super::primitives::MemoryError>> {
    match MEMORY_NODE_POOL.get() {
        Some(pool) => Some(pool.acquire().await),
        None => None,
    }
}
```

**Changes:**
1. `pub fn` → `pub async fn`
2. Return type: `Option<PooledMemoryNode<'static>>` → `Option<Result<PooledMemoryNode<'static>, ...>>`
3. `.map(|pool| pool.acquire())` → explicit match with `.await`

---

## Step-by-Step Implementation

### Step 1: Update `acquire()` Method

Edit [`src/domain/memory/pool.rs`](../src/domain/memory/pool.rs) lines 43-76:

1. Change line 43: `pub fn acquire` → `pub async fn acquire`
2. Change line 43 return type: add `Result<..., super::primitives::MemoryError>` wrapper
3. Change line 68: `let _ = node.reset(MemoryType::Working);` → `node.reset(MemoryType::Working).await?;`
4. Change line 70: `PooledMemoryNode {` → `Ok(PooledMemoryNode {`

### Step 2: Update `initialize()` Method

Edit [`src/domain/memory/pool.rs`](../src/domain/memory/pool.rs) lines 106-127:

1. Change line 107: `pub fn initialize` → `pub async fn initialize`
2. Add return type: `-> Result<(), super::primitives::MemoryError>`
3. Change line 110: `let _ = self.node.reset(memory_type);` → `self.node.reset(memory_type).await?;`
4. Add `Ok(())` before the closing brace (after line 126)

### Step 3: Update `acquire_pooled_node()` Wrapper

Edit [`src/domain/memory/pool.rs`](../src/domain/memory/pool.rs) lines 216-218:

1. Change line 216: `pub fn` → `pub async fn`
2. Change return type: `Option<PooledMemoryNode<'static>>` → `Option<Result<PooledMemoryNode<'static>, super::primitives::MemoryError>>`
3. Replace body with explicit match and `.await`

---

## Definition of Done

✅ **Line 43:** `acquire()` signature is `pub async fn acquire(&self) -> Result<PooledMemoryNode<'_>, super::primitives::MemoryError>`

✅ **Line 68:** Contains `node.reset(MemoryType::Working).await?;` (with `.await?`)

✅ **Line 70:** Returns `Ok(PooledMemoryNode { ... })`

✅ **Line 107:** `initialize()` signature is `pub async fn initialize(&mut self, content: String, memory_type: MemoryType) -> Result<(), super::primitives::MemoryError>`

✅ **Line 110:** Contains `self.node.reset(memory_type).await?;` (with `.await?`)

✅ **Line 127:** Returns `Ok(())`

✅ **Line 216:** `acquire_pooled_node()` is async with proper return type

✅ **Compilation:** `cargo check -p paraphym_candle` passes with exit code 0

---

## Verification Commands

```bash
# Verify no unawaited reset() calls remain
rg "\.reset\(" src/domain/memory/pool.rs | grep -v "await"
# Expected: No matches (all reset calls should have .await)

# Verify both methods are async
rg "pub async fn (acquire|initialize)" src/domain/memory/pool.rs
# Expected: 2 matches

# Verify compilation
cd /Volumes/samsung_t9/paraphym/packages/candle
cargo check -p paraphym_candle
# Expected: Exit code 0, no errors
```

---

## Why No Cascading Changes

Research shows the memory pool has **zero callers** in the current codebase:
- `acquire_pooled_node()` is defined but never called
- `pool.acquire()` only called from wrapper
- `node.initialize()` never called

This means making these methods async has **zero impact** on existing code.

---

## Related Tasks

- **Caused by:** [Task 027: Fix Async Propagation](./027-fix-compilation-errors.md) - Made `reset()` async
- **Related:** [Task 025: Verify Async Conversion](./025-verify-async-conversion.md) - Async verification

---

## Reference Links

### Source Files
- [`src/domain/memory/pool.rs`](../src/domain/memory/pool.rs) - Memory pool implementation (NEEDS FIX)
- [`src/domain/memory/primitives/node.rs:532`](../src/domain/memory/primitives/node.rs#L532) - `reset()` async method definition

### Key Code Locations
- Line 68: `acquire()` unawaited reset
- Line 110: `initialize()` unawaited reset  
- Line 565 in node.rs: Why reset is async (`metadata.write().await`)
