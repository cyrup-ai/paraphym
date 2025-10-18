# ASYNC_4: Fix Sync Locks in capability/ Directory

## OBJECTIVE

Investigate and fix all sync lock calls without `.await` in the `src/capability/` directory. Convert sync locks to async where needed, or document legitimate exceptions.

---

## BACKGROUND

During async verification, the following files in `src/capability/` were found to have `.lock()`, `.read()`, or `.write()` calls without `.await`:

1. `capability/text_embedding/stella.rs` - sync locks
2. `capability/text_embedding/nvembed.rs` - sync locks
3. `capability/registry.rs` - sync locks (`.read()` and `.write()`)
4. `capability/text_to_image/flux_schnell.rs` - sync locks

**Note on flux_schnell.rs**: This file uses worker thread + LocalSet pattern for !Send types. Sync locks inside the worker thread context may be acceptable if they never cross async boundaries.

---

## SUBTASKS

### SUBTASK 1: Fix `capability/text_embedding/stella.rs`

**Find lock calls**:
```bash
grep -n "\.lock()" src/capability/text_embedding/stella.rs
```

**Investigation required**:
- Multiple `.lock()` calls found
- Check if these are in async embed functions or worker thread contexts
- Determine lock type (`std::sync` vs `tokio::sync`)

**Fix pattern if in async context**:
```rust
// ❌ BEFORE
let guard = something.lock().unwrap();

// ✅ AFTER
let guard = something.lock().await;
```

**Verification**:
```bash
grep -n "\.lock()" src/capability/text_embedding/stella.rs -A 1 | grep -v "await"
```

---

### SUBTASK 2: Fix `capability/text_embedding/nvembed.rs`

**Find lock calls**:
```bash
grep -n "\.lock()" src/capability/text_embedding/nvembed.rs
```

**Investigation required**:
- Similar to stella.rs
- Check async vs sync context
- Convert if needed

**Verification**:
```bash
grep -n "\.lock()" src/capability/text_embedding/nvembed.rs -A 1 | grep -v "await"
```

---

### SUBTASK 3: Fix `capability/registry.rs`

**Find lock calls**:
```bash
grep -n "\.read()\|\.write()" src/capability/registry.rs
```

**Investigation required**:
- Multiple `.read()` and `.write()` calls found
- Registry pattern - likely has a static RwLock
- Check if lock is `std::sync::RwLock` or `tokio::sync::RwLock`
- Check if called from async functions

**Common pattern to fix**:
```rust
// ❌ BEFORE
if let Ok(map) = runtime.read() {
    // ...
}

if let Ok(mut map) = runtime.write() {
    // ...
}

// ✅ AFTER (if in async context)
let map = runtime.read().await;
// ...

let mut map = runtime.write().await;
// ...
```

**Note**: `tokio::sync::RwLock` doesn't return `Result`, so no `if let Ok` needed

**Verification**:
```bash
grep -n "\.read()\|\.write()" src/capability/registry.rs -A 1 | grep -v "await"
```

---

### SUBTASK 4: Investigate `capability/text_to_image/flux_schnell.rs`

**Find lock calls**:
```bash
grep -n "\.lock()" src/capability/text_to_image/flux_schnell.rs
```

**Special consideration**:
This file uses the worker thread + LocalSet pattern for !Send FLUX models:
```rust
std::thread::spawn(move || {
    let rt = tokio::runtime::Builder::new_multi_thread()...build()?;
    let local = tokio::task::LocalSet::new();
    rt.block_on(local.run_until(async move {
        // Models stay here - never sent across threads
    }))
});
```

**Investigation required**:
1. Are the `.lock()` calls inside the LocalSet async block? → Need `.await`
2. Are they inside `spawn_blocking` calls? → `std::sync` is OK
3. Are they in sync helper functions called from async? → Need conversion

**Read around lines with locks to determine context**

**Verification**:
```bash
grep -n "\.lock()" src/capability/text_to_image/flux_schnell.rs -B 5 -A 5
```

---

### SUBTASK 5: Final verification across capability/ directory

**Verification command**:
```bash
cd /Volumes/samsung_t9/paraphym/packages/candle

# Find all lock calls in capability/ directory
find src/capability -name "*.rs" -exec grep -Hn "\.lock()\|\.read()\|\.write()" {} \; | grep -v "await" | grep -v "//" | head -30
```

**Expected**: Only documented exceptions or sync-only contexts

---

## DEFINITION OF DONE

- ✅ All 4 files investigated
- ✅ All async context locks use `tokio::sync` with `.await`
- ✅ Worker thread contexts documented if using `std::sync`
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

### Worker Thread Pattern (flux_schnell.rs)

The file uses this pattern for !Send types:
```rust
std::thread::spawn(move || {
    // Dedicated thread for models with raw GPU pointers
    let rt = Runtime::new()?;
    let local = LocalSet::new();
    rt.block_on(local.run_until(async move {
        // ← ASYNC CONTEXT INSIDE
        // Locks here need .await
    }))
})
```

**Inside the LocalSet async block**: Must use `tokio::sync` with `.await`

**Outside in spawn_blocking**: Can use `std::sync` (document why)

### Registry Pattern

Registries typically use:
```rust
static REGISTRY: LazyLock<RwLock<HashMap<...>>> = ...;

pub fn get<T>(key: &str) -> Option<T> {
    let map = REGISTRY.read()?;  // ← Need to check if this is std or tokio
    map.get(key).cloned()
}
```

If functions are called from async contexts → Must use `tokio::sync::RwLock`

### Embedding Models

Text embedding models may use locks to protect model state. Check if:
- Embed functions are async
- Locks are held across `.await` points
- Using correct lock type

---

## FILE LOCATIONS

**Files to investigate**:
- `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/text_embedding/stella.rs`
- `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/text_embedding/nvembed.rs`
- `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs`
- `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/text_to_image/flux_schnell.rs`

**Reference implementation**:
- `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/vision/llava.rs` (correct worker thread pattern)

---

## ESTIMATED TIME

**1.5-2 hours**:
- 25 min per file × 4 files = 100 min
- 20 min: Final verification
