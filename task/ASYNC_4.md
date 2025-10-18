# ASYNC_4: Fix Sync Locks in capability/ Directory

## OBJECTIVE

Fix all sync lock calls without `.await` in async contexts within the `src/capability/` directory. Convert sync locks to async where needed, or document legitimate exceptions.

**Core Issue**: Using `std::sync` locks (Mutex/RwLock) in async contexts blocks the tokio runtime thread, preventing other tasks from executing. This violates async best practices and can cause performance degradation or deadlocks.

**Solution**: Replace `std::sync` with `tokio::sync` in async contexts, or use `spawn_blocking` for CPU-bound work with sync locks.

---

## RESEARCH FINDINGS

### Investigation Summary

After examining all files in `src/capability/`, I found:

**✅ CORRECT (No Changes Needed)**:
1. `capability/text_embedding/stella.rs` - Uses `std::sync::Mutex` **inside** `tokio::task::spawn_blocking` (lines 658-659, 790-791)
2. `capability/text_embedding/nvembed.rs` - Uses `std::sync::Mutex` **inside** `tokio::task::spawn_blocking` (lines 560-561, 677-678)

**❌ NEEDS FIX**:
3. `capability/registry.rs` - Uses `std::sync::RwLock` in **async functions** (6 violations on lines 1457, 1477, 1500, 1511, 1527, 1544)
4. `capability/text_to_image/flux_schnell.rs` - Uses `std::sync::Mutex` in **async function** `ensure_thread_spawned()` (4 violations on lines 89, 94, 158, 165)

### Why stella.rs and nvembed.rs Are Correct

Both files use the **correct pattern** for CPU-bound work with !Send types:

```rust
// stella.rs:658-659, nvembed.rs:560-561
let embeddings = tokio::task::spawn_blocking(move || {
    // ✅ CORRECT: std::sync::Mutex locked inside blocking context
    let mut model_guard = model.lock()
        .map_err(|_| "Failed to lock model mutex".to_string())?;
    
    // CPU-intensive model inference here
    model_guard.forward_norm(&input_ids, &attention_mask)
        .map_err(|e| format!("Forward pass failed: {}", e))
}).await
```

**Why this is correct**:
- `spawn_blocking` moves work to a dedicated blocking thread pool
- Blocking the blocking thread is fine - that's what it's for
- Model inference is CPU-bound, not I/O bound
- `std::sync::Mutex` is cheaper than `tokio::sync::Mutex` when no `.await` inside critical section

**Reference**: See [../packages/candle/src/capability/text_embedding/stella.rs:658-791](../packages/candle/src/capability/text_embedding/stella.rs) and [nvembed.rs:560-678](../packages/candle/src/capability/text_embedding/nvembed.rs)

---

## ISSUE 1: registry.rs - Runtime Registries Use Sync Locks

### Problem

File: [`src/capability/registry.rs`](../packages/candle/src/capability/registry.rs)

Lines with sync locks in async contexts:
- **1457**: `runtime.write()` in `register_image_embedding()`
- **1477**: `runtime.write()` in `register_text_to_image()`
- **1500**: `runtime.write()` in `register_text_to_text()`
- **1511**: `runtime.read()` in `get_image_embedding_runtime()`
- **1527**: `runtime.read()` in `get_text_to_image_runtime()`
- **1544**: `runtime.read()` in `get_text_to_text_runtime()`

### Current Code

```rust
// Line 1275: Declaration
static IMAGE_EMBEDDING_RUNTIME: OnceLock<RwLock<HashMap<String, ImageEmbeddingModel>>> =
    OnceLock::new();

// Line 1457: register_image_embedding()
pub fn register_image_embedding(key: impl Into<String>, model: ImageEmbeddingModel) {
    let runtime = IMAGE_EMBEDDING_RUNTIME.get_or_init(|| RwLock::new(HashMap::new()));
    if let Ok(mut map) = runtime.write() {  // ❌ BLOCKS async runtime
        map.insert(key.into(), model);
    }
}

// Line 1511: get_image_embedding_runtime()
pub fn get_image_embedding_runtime(key: &str) -> Option<ImageEmbeddingModel> {
    if let Some(runtime) = IMAGE_EMBEDDING_RUNTIME.get()
        && let Ok(map) = runtime.read()  // ❌ BLOCKS async runtime
        && let Some(model) = map.get(key)
    {
        return Some(model.clone());
    }
    IMAGE_EMBEDDING_REGISTRY.get(key).cloned()
}
```

### Root Cause

The `RwLock` type is `std::sync::RwLock`, not `tokio::sync::RwLock`:
- `std::sync::RwLock::read()` returns `Result<RwLockReadGuard>`
- `std::sync::RwLock::write()` returns `Result<RwLockWriteGuard>`
- Neither returns a Future, so no `.await` is possible
- These functions **block the current thread** while waiting for the lock

When called from async functions (e.g., pool registration code), this blocks the tokio worker thread.

### Fix Pattern

Use `tokio::sync::RwLock` instead. See correct usage in [`src/domain/tool/router.rs:25-27`](../packages/candle/src/domain/tool/router.rs):

```rust
// router.rs:25-27 - CORRECT PATTERN ✅
pub struct SweetMcpRouter {
    available_tools: Arc<tokio::sync::RwLock<Vec<ToolInfo>>>,
    tool_routes: Arc<tokio::sync::RwLock<HashMap<String, ToolRoute>>>,
}

// router.rs:114-122 - CORRECT USAGE ✅
pub async fn initialize(&mut self) -> Result<(), RouterError> {
    // ... discovery code ...
    
    {
        let mut available_tools = self.available_tools.write().await;  // ✅ .await
        *available_tools = tools;
    }
    {
        let mut tool_routes = self.tool_routes.write().await;  // ✅ .await
        *tool_routes = routes;
    }
    Ok(())
}
```

### Required Changes in registry.rs

**Step 1**: Change type declarations (lines ~1275, 1281, 1290):

```rust
// ❌ BEFORE
static IMAGE_EMBEDDING_RUNTIME: OnceLock<RwLock<HashMap<String, ImageEmbeddingModel>>> =
    OnceLock::new();

// ✅ AFTER
static IMAGE_EMBEDDING_RUNTIME: OnceLock<tokio::sync::RwLock<HashMap<String, ImageEmbeddingModel>>> =
    OnceLock::new();
```

**Step 2**: Add imports at top of file:

```rust
use tokio::sync::RwLock as TokioRwLock;
```

**Step 3**: Make registration functions async and use `.await`:

```rust
// ❌ BEFORE
pub fn register_image_embedding(key: impl Into<String>, model: ImageEmbeddingModel) {
    let runtime = IMAGE_EMBEDDING_RUNTIME.get_or_init(|| RwLock::new(HashMap::new()));
    if let Ok(mut map) = runtime.write() {
        map.insert(key.into(), model);
    }
}

// ✅ AFTER
pub async fn register_image_embedding(key: impl Into<String>, model: ImageEmbeddingModel) {
    let runtime = IMAGE_EMBEDDING_RUNTIME.get_or_init(|| TokioRwLock::new(HashMap::new()));
    let mut map = runtime.write().await;  // No Result, no if-let
    map.insert(key.into(), model);
}
```

**Step 4**: Make getter functions async and use `.await`:

```rust
// ❌ BEFORE
pub fn get_image_embedding_runtime(key: &str) -> Option<ImageEmbeddingModel> {
    if let Some(runtime) = IMAGE_EMBEDDING_RUNTIME.get()
        && let Ok(map) = runtime.read()
        && let Some(model) = map.get(key)
    {
        return Some(model.clone());
    }
    IMAGE_EMBEDDING_REGISTRY.get(key).cloned()
}

// ✅ AFTER
pub async fn get_image_embedding_runtime(key: &str) -> Option<ImageEmbeddingModel> {
    if let Some(runtime) = IMAGE_EMBEDDING_RUNTIME.get() {
        let map = runtime.read().await;  // No Result, no if-let
        if let Some(model) = map.get(key) {
            return Some(model.clone());
        }
    }
    IMAGE_EMBEDDING_REGISTRY.get(key).cloned()
}
```

**Note**: `tokio::sync::RwLock` doesn't return `Result`, so no `if let Ok` needed.

### Files That Call These Functions

After making these functions async, you must update all call sites to add `.await`:

```bash
# Find all call sites
grep -rn "register_image_embedding\|register_text_to_image\|register_text_to_text" packages/candle/src/
grep -rn "get_image_embedding_runtime\|get_text_to_image_runtime\|get_text_to_text_runtime" packages/candle/src/
```

Common call sites will be:
- Pool initialization code
- Builder methods that register models
- Tests (if any exist)

**Repeat this fix for all 3 runtime registries**:
1. `IMAGE_EMBEDDING_RUNTIME` (lines 1457, 1511)
2. `TEXT_TO_IMAGE_RUNTIME` (lines 1477, 1527)
3. `TEXT_TO_TEXT_RUNTIME` (lines 1500, 1544)

---

## ISSUE 2: flux_schnell.rs - Lazy Initialization Uses Sync Locks

### Problem

File: [`src/capability/text_to_image/flux_schnell.rs`](../packages/candle/src/capability/text_to_image/flux_schnell.rs)

Lines with sync locks in async context:
- **89**: `self.request_tx.lock()` in `ensure_thread_spawned()`
- **94**: `self.device.lock()` in `ensure_thread_spawned()`
- **158**: `self.request_tx.lock()` (mutable) in `ensure_thread_spawned()`
- **165**: `self.device.lock()` (mutable) in `ensure_thread_spawned()`

### Current Code

```rust
// Line 68: Struct fields
#[derive(Clone, Debug)]
pub struct FluxSchnell {
    request_tx: Arc<Mutex<Option<mpsc::UnboundedSender<FluxRequest>>>>,
    device: Arc<Mutex<Option<Device>>>,
}

// Line 85-107: ensure_thread_spawned() - async function
fn ensure_thread_spawned(
    &self,
    device: &Device,
) -> Result<mpsc::UnboundedSender<FluxRequest>, String> {
    // Check if thread already spawned
    {
        let tx_guard = self.request_tx.lock()  // ❌ Line 89: BLOCKS async runtime
            .map_err(|e| format!("Lock poisoned: {}", e))?;
        
        if let Some(sender) = tx_guard.as_ref() {
            let device_guard = self.device.lock()  // ❌ Line 94: BLOCKS async runtime
                .map_err(|e| format!("Lock poisoned: {}", e))?;
            
            if let Some(worker_device) = device_guard.as_ref() {
                if !devices_match(worker_device, device) {
                    return Err(format!("Worker already initialized with {:?}", worker_device));
                }
            }
            return Ok(sender.clone());
        }
    }

    // ... spawn thread ...

    // Cache sender and device
    {
        let mut tx_guard = self.request_tx.lock()  // ❌ Line 158: BLOCKS async runtime
            .map_err(|e| format!("Lock poisoned: {}", e))?;
        *tx_guard = Some(request_tx.clone());
    }
    {
        let mut device_guard = self.device.lock()  // ❌ Line 165: BLOCKS async runtime
            .map_err(|e| format!("Lock poisoned: {}", e))?;
        *device_guard = Some(device.clone());
    }

    Ok(request_tx)
}
```

### Root Cause

The `Mutex` type is `std::sync::Mutex`, not `tokio::sync::Mutex`:
- Called from `generate()` method which returns `Pin<Box<dyn Stream>>`
- The stream is created in async context
- Blocks tokio worker thread during lazy initialization

### Fix Pattern

Replace `std::sync::Mutex` with `tokio::sync::Mutex`:

```rust
// ❌ BEFORE
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct FluxSchnell {
    request_tx: Arc<Mutex<Option<mpsc::UnboundedSender<FluxRequest>>>>,
    device: Arc<Mutex<Option<Device>>>,
}

// ✅ AFTER
use tokio::sync::Mutex as TokioMutex;

#[derive(Clone, Debug)]
pub struct FluxSchnell {
    request_tx: Arc<TokioMutex<Option<mpsc::UnboundedSender<FluxRequest>>>>,
    device: Arc<TokioMutex<Option<Device>>>,
}
```

### Required Changes in flux_schnell.rs

**Step 1**: Change imports (line ~18):

```rust
// ❌ BEFORE
use std::sync::{Arc, Mutex};

// ✅ AFTER
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
```

**Step 2**: Update struct fields (line ~68):

```rust
// ❌ BEFORE
pub struct FluxSchnell {
    request_tx: Arc<Mutex<Option<mpsc::UnboundedSender<FluxRequest>>>>,
    device: Arc<Mutex<Option<Device>>>,
}

// ✅ AFTER
pub struct FluxSchnell {
    request_tx: Arc<TokioMutex<Option<mpsc::UnboundedSender<FluxRequest>>>>,
    device: Arc<TokioMutex<Option<Device>>>,
}
```

**Step 3**: Update `new()` method (line ~80):

```rust
// ❌ BEFORE
Self {
    request_tx: Arc::new(Mutex::new(None)),
    device: Arc::new(Mutex::new(None)),
}

// ✅ AFTER
Self {
    request_tx: Arc::new(TokioMutex::new(None)),
    device: Arc::new(TokioMutex::new(None)),
}
```

**Step 4**: Make `ensure_thread_spawned()` async and add `.await` (lines 85-177):

```rust
// ❌ BEFORE
fn ensure_thread_spawned(
    &self,
    device: &Device,
) -> Result<mpsc::UnboundedSender<FluxRequest>, String> {
    {
        let tx_guard = self.request_tx.lock()
            .map_err(|e| format!("Lock poisoned: {}", e))?;
        // ...
    }
}

// ✅ AFTER
async fn ensure_thread_spawned(
    &self,
    device: &Device,
) -> Result<mpsc::UnboundedSender<FluxRequest>, String> {
    {
        let tx_guard = self.request_tx.lock().await;  // No Result, no map_err
        // ...
    }
}
```

**Note**: `tokio::sync::Mutex` doesn't return `Result`, so remove `.map_err()` calls.

**Step 5**: Update all 4 lock call sites:

```rust
// Line 89 ❌ BEFORE
let tx_guard = self.request_tx.lock()
    .map_err(|e| format!("Lock poisoned: {}", e))?;

// Line 89 ✅ AFTER
let tx_guard = self.request_tx.lock().await;

// Line 94 ❌ BEFORE
let device_guard = self.device.lock()
    .map_err(|e| format!("Lock poisoned: {}", e))?;

// Line 94 ✅ AFTER
let device_guard = self.device.lock().await;

// Lines 158, 165: Same pattern - replace .lock().map_err() with .lock().await
```

**Step 6**: Update caller to add `.await`:

The caller is the `generate()` method in `ImageGenerationModel` impl:

```rust
// ❌ BEFORE (inside stream creation)
let request_tx = match self.ensure_thread_spawned(device) {
    Ok(tx) => tx,
    Err(e) => { /* ... */ }
};

// ✅ AFTER
let request_tx = match self.ensure_thread_spawned(device).await {
    Ok(tx) => tx,
    Err(e) => { /* ... */ }
};
```

---

## VERIFICATION COMMANDS

After making changes, verify no unawaited locks remain:

```bash
# Check registry.rs
grep -n "\.read()\|\.write()" packages/candle/src/capability/registry.rs | grep -v "await" | grep -v "//"

# Check flux_schnell.rs  
grep -n "\.lock()" packages/candle/src/capability/text_to_image/flux_schnell.rs | grep -v "await" | grep -v "//"

# Check all capability files
find packages/candle/src/capability -name "*.rs" -exec grep -Hn "\.lock()\|\.read()\|\.write()" {} \; | grep -v "await" | grep -v "//"

# Verify cargo check passes
cargo check -p paraphym_candle
```

**Expected**: Only stella.rs and nvembed.rs should show locks (inside spawn_blocking contexts).

---

## DEFINITION OF DONE

- ✅ `registry.rs`: All 6 lock calls converted to tokio::sync with `.await`
  - 3 runtime registry types changed to `tokio::sync::RwLock`
  - 3 registration functions made async
  - 3 getter functions made async
  - All call sites updated to add `.await`

- ✅ `flux_schnell.rs`: All 4 lock calls converted to tokio::sync with `.await`
  - Struct fields changed to `tokio::sync::Mutex`
  - `ensure_thread_spawned()` made async
  - Caller updated to add `.await`

- ✅ No unawaited locks in async contexts remain in `capability/` directory

- ✅ `cargo check -p paraphym_candle` passes without errors

---

## CONSTRAINTS

- ❌ **NO TESTS**: Do not write unit tests, integration tests, or test code
- ❌ **NO BENCHMARKS**: Do not write benchmark code or performance measurements
- ❌ **NO DOCUMENTATION**: Do not write README updates, CHANGELOG entries, or doc comments beyond what already exists
- ✅ **Focus on src/ only**: Only modify source files to fix the async lock issues
- ✅ **Preserve behavior**: Changes should be drop-in replacements with no behavior changes

---

## REFERENCE FILES

### Correct Patterns (Do Not Modify)
- [`src/capability/text_embedding/stella.rs`](../packages/candle/src/capability/text_embedding/stella.rs) - Correct spawn_blocking usage
- [`src/capability/text_embedding/nvembed.rs`](../packages/candle/src/capability/text_embedding/nvembed.rs) - Correct spawn_blocking usage
- [`src/domain/tool/router.rs`](../packages/candle/src/domain/tool/router.rs) - Correct tokio::sync::RwLock usage pattern

### Files to Modify
- [`src/capability/registry.rs`](../packages/candle/src/capability/registry.rs) - Lines 1457, 1477, 1500, 1511, 1527, 1544
- [`src/capability/text_to_image/flux_schnell.rs`](../packages/candle/src/capability/text_to_image/flux_schnell.rs) - Lines 89, 94, 158, 165

---

## ESTIMATED TIME

**2-3 hours**:
- 60 min: registry.rs changes (6 lock sites + type changes + call site updates)
- 45 min: flux_schnell.rs changes (4 lock sites + async propagation)
- 30 min: Verification and cargo check
- 15 min: Find and update call sites