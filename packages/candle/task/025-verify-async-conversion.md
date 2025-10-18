# Task 025: Verify Complete Async Conversion

## Status: READY FOR EXECUTION

## Objective

Comprehensive verification that all sync/async bridges have been removed and the codebase is fully async. This is a **VERIFICATION TASK**, not an implementation task. The goal is to identify remaining sync primitives and confirm that async conversions from Task 023 and Task 024 have been completed correctly.

## Prerequisites

- **Task 023** ([023-replace-sync-mutexes.md](./023-replace-sync-mutexes.md)) - Replace sync primitives with tokio::sync
- **Task 024** ([024-image-spawn-blocking.md](./024-image-spawn-blocking.md)) - Image spawn_blocking implementation

---

## Research Findings (2025-10-17)

### Codebase Structure

```
src/
├── capability/           # Model capabilities (vision, text-to-text, embeddings)
│   ├── vision/          # LLaVA vision model (uses !Send types correctly)
│   ├── text_to_text/    # Text generation models
│   └── text_to_image/   # FLUX, SD3.5 (image generation)
├── pool/                # Worker pool orchestration
│   └── core/            # Request queue, orchestrator (CRITICAL PATH)
├── domain/              # Core domain logic
│   ├── chat/           # Chat configuration and templates
│   ├── memory/         # Memory systems (cognitive, primitives)
│   └── embedding/      # Embedding services
├── memory/              # Memory infrastructure
│   └── monitoring/     # Performance monitoring
└── cli/                 # CLI runner (may have sync context)
```

### Key Architecture Patterns Found

From examining [../src/capability/text_to_image/flux_schnell.rs](../src/capability/text_to_image/flux_schnell.rs) and [../src/capability/vision/llava.rs](../src/capability/vision/llava.rs):

**Worker Thread + LocalSet Pattern (CORRECT for !Send types):**
```rust
// Pattern used in flux_schnell.rs:114-152
std::thread::spawn(move || {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .expect("Failed to create worker runtime");

    let local = tokio::task::LocalSet::new();
    rt.block_on(local.run_until(async move {
        // Models with !Send types stay in this thread
        let models = Self::load_models(&device).await?;
        while let Some(req) = request_rx.recv().await {
            // Process requests with direct model access
            Self::process_generation(&mut models, ...);
        }
    }))
});
```

**Why This Pattern is Everywhere:**
- Candle ML models contain raw GPU pointers (CUDA/Metal contexts)
- Raw pointers don't implement `Send`
- Models must stay on their origin thread
- Worker thread + LocalSet ensures thread isolation
- `spawn_blocking` bridges from multi-threaded runtime to worker thread

---

## Current State Analysis

### Sync Primitives Inventory

Based on comprehensive searches of `/Volumes/samsung_t9/paraphym/packages/candle/src/`:

#### std::sync::Mutex - 9 occurrences

**Real uses:**
- [../src/cli/runner.rs:138](../src/cli/runner.rs#L138) - `Arc<Mutex<Handler>>` in CLI callback
- [../src/domain/memory/tool.rs:15](../src/domain/memory/tool.rs#L15) - Commented line

**Documentation/comments only (OK):**
- ../src/capability/text_embedding/nvembed.rs:471-478 (3 occurrences in docs)
- ../src/capability/text_embedding/stella.rs:530-535 (2 occurrences in docs)

**Remaining occurrences:** All in comments/docs

#### std::sync::RwLock - 16 occurrences (PROBLEMATIC)

**Critical conversions needed:**
1. [../src/domain/embedding/service.rs:102](../src/domain/embedding/service.rs#L102) - `InMemoryEmbeddingCache.cache`
2. [../src/domain/chat/macros.rs:9](../src/domain/chat/macros.rs#L9) - Import statement
3. [../src/domain/chat/templates/cache/store.rs:49](../src/domain/chat/templates/cache/store.rs#L49) - `MemoryStore.templates`
4. [../src/domain/chat/search/index.rs:8](../src/domain/chat/search/index.rs#L8) - Search index structures
5. [../src/domain/chat/config.rs:9](../src/domain/chat/config.rs#L9) - Configuration manager
6. [../src/domain/memory/cognitive/types.rs:3](../src/domain/memory/cognitive/types.rs#L3) - `QuantumSignature.entanglement_bonds`
7. [../src/memory/monitoring/performance.rs:47](../src/memory/monitoring/performance.rs#L47) - `PerformanceMonitor.response_times`

**Why RwLock is worse than Mutex in async:**
- `std::sync::RwLock` can cause starvation when writers wait for readers
- Blocking read() can hold up the entire tokio thread pool
- tokio::sync::RwLock yields between contention, allowing other tasks to run

#### parking_lot - 17 occurrences (PROBLEMATIC)

**Critical conversions needed:**
1. [../src/pool/core/request_queue.rs:8](../src/pool/core/request_queue.rs#L8) - `use parking_lot::RwLock`
2. [../src/pool/core/orchestrator.rs:8](../src/pool/core/orchestrator.rs#L8) - `use parking_lot::RwLock`
3. [../src/domain/core/mod.rs:94](../src/domain/core/mod.rs#L94) - `CircuitBreaker.last_failure`
4. [../src/domain/memory/primitives/types.rs:364](../src/domain/memory/primitives/types.rs#L364) - `BaseMemory.metadata`
5. [../src/domain/chat/config.rs:597](../src/domain/chat/config.rs#L597) - Configuration locks
6. [../src/domain/chat/macros.rs:742-746](../src/domain/chat/macros.rs#L742-L746) - `ExecutionStats` fields (3 occurrences)

**Why parking_lot must go:**
- Even though parking_lot is faster than std::sync, it still blocks OS threads
- In tokio runtime, blocking a worker thread starves other tasks
- tokio::sync primitives are async-aware and yield properly

### Correct Async Patterns Found (DO NOT CHANGE)

#### spawn_blocking - 118 occurrences ✅

**Correct usage patterns identified:**

**Tokenizer loading** (CPU-intensive):
```rust
// From src/capability/text_to_text/kimi_k2.rs:436-442
tokio::task::spawn_blocking(move || {
    let tokenizer = Tokenizer::from_file(&path)
        .map_err(|e| format!("Failed to load tokenizer: {}", e))?;
    Ok(tokenizer)
})
.await??
```

**File I/O** (blocking operations):
```rust
// From src/domain/context/loader.rs:133
tokio::task::spawn_blocking(move || {
    std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))
})
.await??
```

**Image processing** (from src/builders/image.rs:116-448):
```rust
tokio::task::spawn_blocking(move || {
    // Heavy image decoding/processing
    image::load_from_memory(&bytes)
})
.await??
```

**Vector search** (CPU-intensive, from src/memory/vector/vector_search.rs:591, 764):
```rust
tokio::task::spawn_blocking(move || {
    // HNSW graph traversal, distance calculations
    search_nearest_neighbors(&index, &query)
})
.await??
```

**Health checks** (from src/memory/monitoring/health.rs:368, 381):
```rust
tokio::task::spawn_blocking(move || {
    // System health metrics collection
    collect_metrics()
})
.await??
```

#### block_on inside spawn_blocking - 5 occurrences ✅

**All in [../src/capability/vision/llava.rs](../src/capability/vision/llava.rs) - CORRECT PATTERN**

**Line 220:** Inside worker thread runtime for !Send model initialization
```rust
std::thread::spawn(move || {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .expect("Failed to create worker runtime");

    let local = tokio::task::LocalSet::new();
    rt.block_on(local.run_until(async move {
        // ✅ CORRECT: Worker thread setup for !Send LLaVA model
        let vision_model = LlavaVision::load(&device).await?;
    }))
});
```

**Line 402:** Inside spawn_blocking for Image::to_tensor async call
```rust
tokio::task::spawn_blocking(move || {
    let runtime = tokio::runtime::Handle::current();
    runtime.block_on(async move {
        // ✅ CORRECT: Bridge async image processing into blocking context
        Image::from_path(&path).to_tensor(&device).await
    })
})
.await??
```

**Line 541:** Inside spawn_blocking for Image::to_tensor async call
```rust
tokio::task::spawn_blocking(move || {
    let runtime = tokio::runtime::Handle::current();
    runtime.block_on(async move {
        // ✅ CORRECT: Same pattern, different code path
        image_data.to_tensor(&device).await
    })
})
.await??
```

**Why this is CORRECT:**

From Tokio documentation ([Bridging with sync code](https://tokio.rs/tokio/topics/bridging)):

> When you have a !Send type (like Candle models with raw GPU pointers), you cannot move it across await points in a multi-threaded runtime. The solution is to:
> 1. Create a dedicated thread for the !Send type
> 2. Run a LocalSet in that thread (allows !Send futures)
> 3. Use channels to communicate between the main runtime and the dedicated thread
> 4. If you need to call async functions FROM the !Send context, use `block_on` inside `spawn_blocking`

**This is the TEXTBOOK pattern** for !Send types in async Rust.

#### tokio::sync usage - 177+ occurrences ✅

**Heavy async adoption found in:**

- [../src/async_stream.rs](../src/async_stream.rs) - Pure tokio streaming utilities
- [../src/pool/core/orchestrator.rs](../src/pool/core/orchestrator.rs) - Worker pool (still has parking_lot imports)
- [../src/domain/tool/router.rs](../src/domain/tool/router.rs) - Tool routing
- [../src/domain/embedding/service.rs](../src/domain/embedding/service.rs) - Embedding cache (mixed with std::sync::RwLock)
- Multiple domain modules showing active async conversion

**Good sign:** 177 occurrences means async conversion is well underway, just needs completion.

---

## What "Fully Async" Means

### Definition

A codebase is "fully async" when:

1. **No sync primitives in async contexts**
   - All `std::sync::Mutex`, `std::sync::RwLock`, `parking_lot::Mutex`, `parking_lot::RwLock` in async functions → `tokio::sync` equivalents
   
2. **All blocking I/O isolated**
   - File operations wrapped in `spawn_blocking`
   - Network I/O uses async APIs (tokio::net, reqwest, hyper)
   - CPU-intensive work (>1ms) wrapped in `spawn_blocking`
   
3. **No blocking across await points**
   - Never hold a sync lock while calling `.await`
   - Example of WRONG pattern:
     ```rust
     async fn bad_example() {
         let guard = std_mutex.lock().unwrap(); // ❌ Blocks thread
         some_async_call().await; // ❌ Still holding lock, blocking thread
         drop(guard);
     }
     ```
   
4. **Proper async propagation**
   - Methods that acquire locks are marked `async fn`
   - Lock calls followed by `.await`:
     ```rust
     async fn good_example() {
         let guard = tokio_mutex.lock().await; // ✅ Yields if contended
         // Do work with guard
         drop(guard); // ✅ Released before next await
         some_async_call().await;
     }
     ```

### What IS Allowed

These are **OK to use** in async code:

- **std::sync::Arc** - Shared ownership (not a lock, just atomic ref counting)
  ```rust
  let shared = Arc::new(data); // ✅ OK
  ```

- **std::sync::atomic::\*** - Lock-free atomic operations
  ```rust
  let counter = Arc::new(AtomicU64::new(0)); // ✅ OK
  counter.fetch_add(1, Ordering::SeqCst); // ✅ Lock-free
  ```

- **std::sync::LazyLock** - Static initialization (happens once at startup)
  ```rust
  static CONFIG: LazyLock<Config> = LazyLock::new(|| load_config()); // ✅ OK
  ```

- **block_on inside spawn_blocking** - Bridge pattern for async in blocking contexts
  ```rust
  tokio::task::spawn_blocking(move || {
      let rt = tokio::runtime::Handle::current();
      rt.block_on(async move {
          // ✅ OK - async operation in blocking context
          async_operation().await
      })
  })
  .await??
  ```

- **Sync primitives in pure-sync code** - If module is 100% sync with no async callers
  ```rust
  // If this module has NO async fn and is NEVER called from async contexts:
  fn pure_sync_function() {
      let mutex = std::sync::Mutex::new(data); // ✅ OK if truly sync-only
  }
  ```

### What is NOT Allowed

These cause **thread pool blocking** and must be converted:

- **std::sync::Mutex in async fn**
  ```rust
  async fn bad() {
      let guard = std_mutex.lock().unwrap(); // ❌ Blocks tokio worker thread
      // If lock is contended, entire thread is blocked
  }
  ```

- **parking_lot::Mutex in async fn**
  ```rust
  async fn bad() {
      let guard = parking_lot_mutex.lock(); // ❌ Same issue, blocks threads
  }
  ```

- **std::sync::RwLock in async fn**
  ```rust
  async fn bad() {
      let guard = std_rwlock.read().unwrap(); // ❌ Can cause starvation
  }
  ```

- **.lock() without .await in async fn**
  ```rust
  async fn bad() {
      let guard = mutex.lock(); // ❌ No .await → sync primitive
      // Should be: mutex.lock().await
  }
  ```

- **Poison error handling**
  ```rust
  // ❌ Sign of std::sync usage (tokio::sync doesn't poison)
  let guard = mutex.lock().map_err(|e| format!("Poisoned: {}", e))?;
  ```

---

## Files Requiring Conversion

> **NOTE:** Task 023 ([023-replace-sync-mutexes.md](./023-replace-sync-mutexes.md)) contains the complete conversion specification with patterns and examples. This section cross-references those files with current verification status.

### Status Legend
- ❌ **NOT CONVERTED** - Still uses std::sync or parking_lot
- ⚠️ **PARTIAL** - Some conversions done, some remain
- ✅ **CONVERTED** - All tokio::sync
- 🔍 **NEEDS REVIEW** - Unclear if legitimate sync context

### Pool Infrastructure (Critical Path)

**Why critical:** Worker pool orchestration is the hot path for all model requests. Blocking here affects all concurrent requests.

| File | Status | Sync Primitives Found | Task 023 Reference |
|------|--------|----------------------|-------------------|
| [../src/pool/core/orchestrator.rs](../src/pool/core/orchestrator.rs) | ❌ NOT CONVERTED | `parking_lot::RwLock` (line 8) | Category 1, File #1 |
| [../src/pool/core/request_queue.rs](../src/pool/core/request_queue.rs) | ❌ NOT CONVERTED | `parking_lot::RwLock` (line 8) | Category 1, File #2 |
| ../src/pool/core/memory_governor.rs | 🔍 NEEDS SEARCH | Unknown | Category 1, File #3 |

**Impact:** High - affects all model inference requests

### Vision Capabilities

| File | Status | Sync Primitives Found | Task 023 Reference |
|------|--------|----------------------|-------------------|
| [../src/capability/vision/llava.rs](../src/capability/vision/llava.rs) | ✅ USES CORRECT PATTERN | `block_on` inside `spawn_blocking` (CORRECT) | Category 2, File #4 |

**Note:** LLaVA uses the worker thread + LocalSet pattern correctly. Do NOT change this file.

### Domain Core

| File | Status | Sync Primitives Found | Task 023 Reference |
|------|--------|----------------------|-------------------|
| [../src/domain/core/mod.rs](../src/domain/core/mod.rs) | ❌ NOT CONVERTED | `parking_lot::Mutex` (line 94, `CircuitBreaker.last_failure`) | Category 3, File #5 |

**Impact:** Medium - circuit breaker affects error handling

### Memory System

| File | Status | Sync Primitives Found | Task 023 Reference |
|------|--------|----------------------|-------------------|
| [../src/domain/memory/cognitive/types.rs](../src/domain/memory/cognitive/types.rs) | ⚠️ PARTIAL | `std::sync::RwLock` (line 3) + `tokio::sync::Mutex` | Category 4, File #6 |
| [../src/domain/memory/primitives/types.rs](../src/domain/memory/primitives/types.rs) | ❌ NOT CONVERTED | `parking_lot::RwLock` (line 364, `BaseMemory.metadata`) | Category 4, File #7 |

**Impact:** Medium - memory operations are frequent but not always on hot path

### Configuration Management

| File | Status | Sync Primitives Found | Task 023 Reference |
|------|--------|----------------------|-------------------|
| [../src/domain/chat/config.rs](../src/domain/chat/config.rs) | ⚠️ PARTIAL | Both `std::sync::RwLock` (line 9) and `parking_lot::RwLock` (line 597) | Category 5, File #8 |
| [../src/domain/chat/macros.rs](../src/domain/chat/macros.rs) | ⚠️ PARTIAL | Both `std::sync::RwLock` (line 9) and `parking_lot::Mutex` (lines 742-746) | Category 5, File #9 |

**Impact:** Low - configuration is typically read-heavy and infrequently updated

### Services

| File | Status | Sync Primitives Found | Task 023 Reference |
|------|--------|----------------------|-------------------|
| [../src/domain/embedding/service.rs](../src/domain/embedding/service.rs) | ⚠️ PARTIAL | `std::sync::RwLock` (line 102, cache) + some `tokio::sync` | Category 6, File #10 |
| [../src/domain/chat/search/index.rs](../src/domain/chat/search/index.rs) | ❌ NOT CONVERTED | `std::sync::RwLock` (line 8) | Category 6, File #11 |
| [../src/domain/chat/templates/cache/store.rs](../src/domain/chat/templates/cache/store.rs) | ❌ NOT CONVERTED | `std::sync::RwLock` (line 49, `MemoryStore.templates`) | Category 6, File #12 |

**Impact:** Medium - embedding cache is frequently accessed

### Other Files (Not in Task 023)

| File | Status | Sync Primitives Found | Action Required |
|------|--------|----------------------|-----------------|
| [../src/cli/runner.rs](../src/cli/runner.rs) | 🔍 NEEDS REVIEW | `std::sync::Mutex` (line 138, `Arc<Mutex<Handler>>`) | Determine if CLI context allows sync |
| [../src/memory/monitoring/performance.rs](../src/memory/monitoring/performance.rs) | ❌ NOT CONVERTED | `std::sync::RwLock` (line 47, `PerformanceMonitor.response_times`) | Add to conversion list |

---

## Verification Steps

Run these commands in `/Volumes/samsung_t9/paraphym/packages/candle/`:

### Step 1: Search for std::sync::Mutex

```bash
# Find all std::sync::Mutex usage (excluding Arc, LazyLock, atomic)
rg "std::sync::Mutex" src/ --type rust

# Expected: Only comments or legitimate sync contexts
# Current: 9 matches (mostly comments/docs)
```

**Review checklist for each match:**
1. Is it in a comment/doc string? → ✅ OK
2. Is it in pure-sync code with no async callers? → ✅ OK (but rare)
3. Is it in CLI or build scripts? → 🔍 Review case-by-case
4. Is it in async code? → ❌ MUST CONVERT

**Files found:**
- [../src/cli/runner.rs:138](../src/cli/runner.rs#L138) - 🔍 NEEDS REVIEW
- [../src/domain/memory/tool.rs:15](../src/domain/memory/tool.rs#L15) - ✅ Commented out
- Documentation files - ✅ OK (just examples/comments)

### Step 2: Search for std::sync::RwLock

```bash
# Find all RwLock usage (most problematic in async code)
rg "std::sync::RwLock" src/ --type rust -C 2

# Expected: Zero matches (RwLock is almost always in async contexts)
# Current: 16 matches - ALL need conversion
```

**Why RwLock is worse than Mutex:**
- Writers block until ALL readers finish
- Readers block when a writer is waiting
- In async context, this creates cascading blocking across tasks
- `tokio::sync::RwLock` yields instead of blocking

**Files found (all need conversion):**
- [../src/domain/embedding/service.rs:102](../src/domain/embedding/service.rs#L102)
- [../src/domain/chat/macros.rs:9](../src/domain/chat/macros.rs#L9)
- [../src/domain/chat/templates/cache/store.rs:49](../src/domain/chat/templates/cache/store.rs#L49)
- [../src/domain/chat/search/index.rs:8](../src/domain/chat/search/index.rs#L8)
- [../src/domain/chat/config.rs:9](../src/domain/chat/config.rs#L9)
- [../src/domain/memory/cognitive/types.rs:3](../src/domain/memory/cognitive/types.rs#L3)
- [../src/memory/monitoring/performance.rs:47](../src/memory/monitoring/performance.rs#L47)

### Step 3: Search for parking_lot Usage

```bash
# Find parking_lot imports and usage
rg "use parking_lot" src/ --type rust
rg "parking_lot::" src/ --type rust -C 2

# Expected: Zero matches
# Current: 17 matches - all need conversion
```

**Why parking_lot must be removed:**
- Even though it's faster than std::sync, it still blocks OS threads
- In tokio runtime, blocking a worker thread starves ALL tasks on that thread
- tokio has 8-16 worker threads by default, so blocking even one reduces concurrency significantly
- `tokio::sync` primitives are async-aware and yield to the scheduler

**Files found (all need conversion):**
- [../src/pool/core/request_queue.rs:8](../src/pool/core/request_queue.rs#L8) - ❌ CRITICAL (hot path)
- [../src/pool/core/orchestrator.rs:8](../src/pool/core/orchestrator.rs#L8) - ❌ CRITICAL (hot path)
- [../src/domain/core/mod.rs:94](../src/domain/core/mod.rs#L94)
- [../src/domain/memory/primitives/types.rs:364](../src/domain/memory/primitives/types.rs#L364)
- [../src/domain/chat/config.rs:597](../src/domain/chat/config.rs#L597)
- [../src/domain/chat/macros.rs:742-746](../src/domain/chat/macros.rs#L742-L746)

### Step 4: Verify spawn_blocking Usage

```bash
# Find all spawn_blocking calls with context
rg "spawn_blocking" src/ --type rust -A 10 -B 2

# Expected: 118+ occurrences wrapping CPU/I/O work
# Current: Correct usage patterns found
```

**Correct patterns to verify:**

✅ **CPU-intensive operations:**
```rust
tokio::task::spawn_blocking(move || {
    // Tokenization, hashing, compression, etc.
    expensive_computation()
})
.await??
```

✅ **File I/O:**
```rust
tokio::task::spawn_blocking(move || {
    std::fs::read_to_string(path)
})
.await.map_err(|e| format!("Spawn error: {}", e))??
```

✅ **Image processing:**
```rust
tokio::task::spawn_blocking(move || {
    image::load_from_memory(&bytes)
})
.await??
```

✅ **!Send types with async operations:**
```rust
tokio::task::spawn_blocking(move || {
    let rt = tokio::runtime::Handle::current();
    rt.block_on(async move {
        // Async call from blocking context
        async_operation().await
    })
})
.await??
```

❌ **INCORRECT - wrapping simple async operations:**
```rust
// ❌ DON'T DO THIS
tokio::task::spawn_blocking(move || {
    // This is already async, no need for spawn_blocking
    async_operation()
})
.await??
```

**Locations verified:**
- Tokenizer loading: ../src/capability/text_to_text/kimi_k2.rs:436-442 ✅
- File I/O: ../src/domain/context/loader.rs:133 ✅
- Image processing: ../src/builders/image.rs:116-448 ✅
- Vector search: ../src/memory/vector/vector_search.rs:591, 764 ✅
- Health checks: ../src/memory/monitoring/health.rs:368, 381 ✅

### Step 5: Verify block_on Usage

```bash
# Find Runtime::block_on calls (should ONLY be inside spawn_blocking)
rg "block_on" src/ --type rust -B 10 -A 5

# Expected: ONLY inside spawn_blocking contexts for !Send types
# Current: 5 matches in llava.rs - all correct
```

**Correct pattern (from [../src/capability/vision/llava.rs:402](../src/capability/vision/llava.rs#L402)):**
```rust
tokio::task::spawn_blocking(move || {
    // ✅ CORRECT: Inside spawn_blocking
    let runtime = tokio::runtime::Handle::current();
    runtime.block_on(async move {
        // Async image processing for !Send model
        Image::from_path(&path).to_tensor(&device).await
    })
})
.await??
```

**Incorrect pattern (would be problematic):**
```rust
async fn my_async_function() {
    // ❌ WRONG: block_on in async context blocks thread
    let result = tokio::runtime::Handle::current().block_on(async {
        some_async_operation().await
    });
}
```

**All block_on locations verified:**
- Line 220: Worker thread runtime initialization ✅
- Line 402: Image::to_tensor async bridge ✅
- Line 541: Image::to_tensor async bridge ✅

**Verdict:** All usage is correct. These are the textbook pattern for !Send types.

### Step 6: Check for Poison Error Handling

```bash
# Find poison error patterns (sign of std::sync::Mutex)
rg "Lock poisoned" src/ --type rust
rg "PoisonError" src/ --type rust
rg "\.lock\(\)\.map_err" src/ --type rust
rg "\.lock\(\)\.unwrap\(\)" src/ --type rust

# Expected: Zero matches (tokio::sync doesn't poison)
# If found: Indicates std::sync::Mutex still in use
```

**Why poison errors indicate std::sync:**

`std::sync::Mutex` and `std::sync::RwLock` can become "poisoned" if a thread panics while holding the lock. The Result returned from `.lock()` contains `PoisonError`.

**Example of std::sync pattern:**
```rust
// std::sync::Mutex pattern:
let guard = mutex.lock()
    .map_err(|e| format!("Lock poisoned: {}", e))?; // ← Sign of std::sync
```

**tokio::sync doesn't poison:**
```rust
// tokio::sync::Mutex pattern:
let guard = mutex.lock().await; // ← No Result, no poison errors
```

**If you find:**
- `.lock().unwrap()` → Probably std::sync (tokio::sync is not Result)
- `.lock().map_err(...)` → Definitely std::sync (handling poison)
- `PoisonError` imports → Definitely std::sync usage

### Step 7: Verify tokio::sync Adoption

```bash
# Find tokio::sync imports and usage (should be widespread)
rg "tokio::sync" src/ --type rust --stats

# Expected: Many matches (177+ found in current codebase)
# This is GOOD - indicates async adoption is happening
```

**Good patterns to see:**
```rust
use tokio::sync::{Mutex, RwLock, mpsc, oneshot};

async fn example() {
    let guard = mutex.lock().await; // ✅ Correct async lock
    let reader = rwlock.read().await; // ✅ Correct async read
    let writer = rwlock.write().await; // ✅ Correct async write
}
```

**High usage indicates:**
- Async conversion is active and widespread
- Team understands async patterns
- Just needs final cleanup of remaining sync primitives

### Step 8: Find Locks Without await

```bash
# Find .lock() calls without .await (potential sync primitive usage)
rg "\.lock\(\)" src/ --type rust -A 1 | rg -v "await"

# Find .read()/.write() without .await
rg "\.read\(\)" src/ --type rust -A 1 | rg -v "await"
rg "\.write\(\)" src/ --type rust -A 1 | rg -v "await"

# Review each match:
# - Is containing function async? Should have .await
# - Is it pure sync code? OK
# - Is it a different .lock() like HashMap? OK
```

**Why this matters:**

In async code, ALL lock acquisitions should be awaited:
```rust
async fn correct() {
    let guard = mutex.lock().await; // ✅
    // ...
}

async fn incorrect() {
    let guard = mutex.lock(); // ❌ Forgot .await OR using std::sync
    // ...
}
```

**False positives to ignore:**
- `HashMap.entry().or_insert()` - Different .lock(), OK
- Pure sync functions - If function is NOT `async fn`, OK
- Comments/doc strings - OK

### Step 9: Compiler Verification

```bash
# Check for async-related compiler errors/warnings
cd /Volumes/samsung_t9/paraphym/packages/candle
cargo check 2>&1 | grep -i "future"
cargo check 2>&1 | grep -i "await"
cargo check 2>&1 | grep -i "send"

# Expected: No errors (warnings OK if documented)
```

**Common async warnings to look for:**

❌ **"unused Result"** - Missing ? or .await:
```rust
async fn bad() {
    mutex.lock().await; // ❌ Warning: unused Result
    // Should be: let _guard = mutex.lock().await;
}
```

❌ **"Future must be .await-ed"** - Missing .await on async call:
```rust
async fn bad() {
    async_function(); // ❌ Warning: future not awaited
    // Should be: async_function().await;
}
```

❌ **"cannot be sent between threads safely"** - !Send type across await:
```rust
async fn bad(model: &ModelWithRawPointers) {
    tokio::spawn(async move {
        model.forward(); // ❌ Error: `*const T` not Send
    });
}
```

**If these appear:** The codebase is NOT fully async yet.

### Step 10: CLI Context Review

Special case: [../src/cli/runner.rs:138](../src/cli/runner.rs#L138)

**Current code:**
```rust
use std::sync::Mutex;
let handler = Arc::new(Mutex::new(self.handler.clone()));
```

**Questions to answer:**

1. **Is the containing function async?**
   - Read runner.rs to check function signature
   - If `async fn` → Must convert to tokio::sync::Mutex
   - If regular `fn` → May be OK

2. **Are there await points nearby?**
   - Look for `.await` calls in the same function
   - If yes → Must convert (async context)
   - If no → May be OK

3. **Is the closure/callback executed in async context?**
   - Check if callback returns `impl Future` or `Pin<Box<dyn Stream>>`
   - Check if callback is called with `.await`
   - If yes → Must convert

4. **Is this a blocking callback (like ctrlc handler)?**
   - Some system callbacks (signals, FFI) are inherently blocking
   - If truly blocking → std::sync may be OK
   - Document with comment explaining why

**Verification script:**
```bash
# Read the context around line 138
rg "Arc<Mutex<Handler>>" ../src/cli/runner.rs -B 20 -A 20

# Check if runner.rs has async functions
rg "async fn" ../src/cli/runner.rs

# Check if there are awaits near the Mutex usage
rg "\.await" ../src/cli/runner.rs -B 5 -A 5 | grep -C 10 "Mutex"
```

**Decision tree:**
- If in async context → ❌ Convert to tokio::sync::Mutex
- If in pure sync context → ✅ OK, but add comment
- If unclear → 🔍 Err on side of caution, convert to tokio::sync

---

## Success Criteria

The async conversion is complete when ALL of the following are true:

### Code Patterns ✅

- [ ] **Zero `std::sync::Mutex`** in async contexts (excluding Arc, atomics, LazyLock)
- [ ] **Zero `std::sync::RwLock`** anywhere (this is an async-heavy codebase)
- [ ] **Zero `parking_lot::Mutex`** or `parking_lot::RwLock`**
- [ ] **All `.lock()`, `.read()`, `.write()` followed by `.await`** in async functions
- [ ] **Zero poison error handling** (no `.map_err()` on locks, no `PoisonError` imports)
- [ ] **`block_on` only inside `spawn_blocking`** contexts (for !Send types)

### Compiler Checks ✅

- [ ] **`cargo check` passes** without errors
- [ ] **No "Future must be .await-ed" warnings**
- [ ] **No "unused Result" warnings on lock calls**
- [ ] **No "cannot be sent between threads safely" errors**

### Architectural Patterns ✅

- [ ] **All file I/O wrapped in `spawn_blocking`**
- [ ] **All CPU-intensive operations (>1ms) wrapped in `spawn_blocking`**
- [ ] **Tokenizer operations wrapped in `spawn_blocking`**
- [ ] **Image processing wrapped in `spawn_blocking`**
- [ ] **!Send model operations use LocalSet + dedicated thread pattern**
- [ ] **Worker pool uses tokio::sync for coordination**

---

## Definition of Done

Task 025 is complete when:

1. **All verification steps executed** and results documented in this file
2. **All files from Task 023's list verified** (either converted or marked as legitimate exceptions)
3. **All new sync primitive usage reviewed** and categorized (convert vs. allowed)
4. **Remaining sync primitives documented** with justification if kept (e.g., pure sync context)
5. **Compiler checks pass** without async-related warnings
6. **Success criteria checklist filled out** with ✅ or documented exceptions

**NOT required:**
- ❌ Unit tests for async behavior
- ❌ Performance benchmarks
- ❌ Documentation updates
- ❌ Integration tests

**Deliverable:**
- Updated this file with verification results
- List of files needing conversion (if Task 023 is incomplete)
- List of legitimate exceptions (if any)
- Compiler output showing clean async checks

---

## Estimated Effort

**1-2 hours** for complete verification:

- **30 min:** Run all search commands and catalog results
  - Execute Steps 1-8 with output saved to files
  - Build spreadsheet/table of findings
  
- **30 min:** Review each sync primitive usage
  - Check if in async context (must convert)
  - Check if in sync context (may be OK)
  - Check for false positives (comments, different .lock())
  
- **15 min:** Run compiler checks (Step 9)
  - `cargo check` full output analysis
  - Categorize warnings/errors
  
- **15 min:** Document findings
  - Update file status tables in this document
  - Create follow-up tasks if needed
  - Fill out success criteria checklist

---

## Next Steps After Verification

Based on verification results, take ONE of these paths:

### Path A: Task 023 Incomplete (sync primitives remain)

1. **Update Task 023 status to IN_PROGRESS**
2. **Execute conversions** as specified in [Task 023](./023-replace-sync-mutexes.md)
3. **Re-run this verification** to confirm completion

### Path B: New Sync Primitives Found (not in Task 023)

1. **Create new conversion task** (Task 026+)
2. **Document why they exist** (when were they added, what feature)
3. **Plan conversion approach** following Task 023 patterns
4. **Add to tracking list**

### Path C: Legitimate Sync Usage Found

For files that legitimately can use std::sync (rare):

1. **Document in code** with comment explaining why:
   ```rust
   // SYNC CONTEXT: This function is called from ctrlc signal handler
   // which is a blocking context. std::sync::Mutex is appropriate here.
   let guard = std_mutex.lock().unwrap();
   ```

2. **Add to "allowed exceptions" list** in this document

3. **Ensure no async callers exist:**
   ```bash
   # Find all call sites
   rg "function_name" src/ --type rust
   
   # Verify none are in async functions
   ```

4. **Consider refactoring** to avoid mixing sync and async

### Path D: Verification Complete (all checks pass) ✅

1. **Mark Task 025 as COMPLETE**
2. **Update success criteria** with all ✅
3. **Mark Task 023 as COMPLETE** (if conversions done)
4. **Proceed to next tasks** (likely [Task 021](./021-async-flux-schnell.md), [Task 022](./022-async-stable-diffusion-35.md))

---

## References

### Related Tasks

- [Task 021: Async FLUX.1-schnell](./021-async-flux-schnell.md) - Text-to-image async conversion
- [Task 022: Async Stable Diffusion 3.5](./022-async-stable-diffusion-35.md) - SD3.5 worker thread pattern
- [Task 023: Replace Sync Mutexes](./023-replace-sync-mutexes.md) - Comprehensive conversion specification
- [Task 024: Image spawn_blocking](./024-image-spawn-blocking.md) - Image processing async patterns

### File Locations

All paths relative to `/Volumes/samsung_t9/paraphym/packages/candle/`:

- **Source root:** [src/](../src/)
- **Pool infrastructure:** [src/pool/core/](../src/pool/core/)
- **Vision capabilities:** [src/capability/vision/llava.rs](../src/capability/vision/llava.rs)
- **Text-to-image:** [src/capability/text_to_image/](../src/capability/text_to_image/)
  - [FLUX.1-schnell](../src/capability/text_to_image/flux_schnell.rs) - Reference implementation
  - [SD3.5 Large Turbo](../src/capability/text_to_image/stable_diffusion_35_turbo/mod.rs) - Needs conversion
- **Domain modules:** [src/domain/](../src/domain/)
- **Memory system:** [src/memory/](../src/memory/)
- **CLI:** [src/cli/runner.rs](../src/cli/runner.rs)

### Tokio Documentation

- [tokio::sync::Mutex](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html)
  - Async-aware mutex that yields instead of blocking
  - No poison errors (panics don't poison)
  
- [tokio::sync::RwLock](https://docs.rs/tokio/latest/tokio/sync/struct.RwLock.html)
  - Async-aware reader-writer lock
  - Fair scheduling (no starvation)
  
- [Tokio: Shared State](https://tokio.rs/tokio/tutorial/shared-state)
  - Tutorial on using tokio::sync in async contexts
  - Explains when to use Mutex vs RwLock vs channels
  
- [Tokio: Bridging with sync code](https://tokio.rs/tokio/topics/bridging)
  - **Critical resource for !Send types**
  - Explains worker thread + LocalSet pattern
  - Shows correct block_on usage in spawn_blocking

### Rust Documentation

- [Send trait](https://doc.rust-lang.org/std/marker/trait.Send.html)
  - Marker trait for types safe to transfer across threads
  - Raw pointers (`*const T`, `*mut T`) are !Send
  
- [std::sync vs tokio::sync](https://tokio.rs/tokio/tutorial/shared-state#holding-a-mutexguard-across-an-await)
  - Why std::sync blocks async runtime
  - Performance implications

### Architecture

- [ARCHITECTURE.md](../ARCHITECTURE.md) - Fluent API patterns, builder chains
- [CLAUDE.md](../CLAUDE.md) - Build commands, testing, workspace structure

---

## Appendix: Quick Reference

### Search Commands Summary

```bash
# Run from /Volumes/samsung_t9/paraphym/packages/candle/

# Step 1: std::sync::Mutex
rg "std::sync::Mutex" src/ --type rust

# Step 2: std::sync::RwLock (highest priority)
rg "std::sync::RwLock" src/ --type rust -C 2

# Step 3: parking_lot (critical for hot paths)
rg "use parking_lot" src/ --type rust
rg "parking_lot::" src/ --type rust -C 2

# Step 4: spawn_blocking verification
rg "spawn_blocking" src/ --type rust -A 10 -B 2

# Step 5: block_on verification
rg "block_on" src/ --type rust -B 10 -A 5

# Step 6: Poison error handling
rg "Lock poisoned" src/ --type rust
rg "PoisonError" src/ --type rust
rg "\.lock\(\)\.map_err" src/ --type rust
rg "\.lock\(\)\.unwrap\(\)" src/ --type rust

# Step 7: tokio::sync adoption (should be high)
rg "tokio::sync" src/ --type rust --stats

# Step 8: Locks without await
rg "\.lock\(\)" src/ --type rust -A 1 | rg -v "await"
rg "\.read\(\)" src/ --type rust -A 1 | rg -v "await"
rg "\.write\(\)" src/ --type rust -A 1 | rg -v "await"

# Step 9: Compiler checks
cargo check 2>&1 | grep -i "future"
cargo check 2>&1 | grep -i "await"
cargo check 2>&1 | grep -i "send"

# Step 10: CLI context review
rg "Arc<Mutex<Handler>>" src/cli/runner.rs -B 20 -A 20
```

### Conversion Quick Reference

| From | To | Async? |
|------|-----|--------|
| `std::sync::Mutex::new()` | `tokio::sync::Mutex::new()` | No |
| `mutex.lock().unwrap()` | `mutex.lock().await` | Yes |
| `std::sync::RwLock::new()` | `tokio::sync::RwLock::new()` | No |
| `rwlock.read().unwrap()` | `rwlock.read().await` | Yes |
| `rwlock.write().unwrap()` | `rwlock.write().await` | Yes |
| `parking_lot::Mutex` | `tokio::sync::Mutex` | - |
| `parking_lot::RwLock` | `tokio::sync::RwLock` | - |

### Pattern Quick Reference

✅ **CORRECT:**
```rust
// Worker thread for !Send types
std::thread::spawn(move || {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1).enable_all().build().unwrap();
    let local = tokio::task::LocalSet::new();
    rt.block_on(local.run_until(async move {
        // !Send types stay here
    }))
});

// Async operations in blocking context
tokio::task::spawn_blocking(move || {
    let rt = tokio::runtime::Handle::current();
    rt.block_on(async move {
        async_operation().await
    })
}).await??;
```

❌ **INCORRECT:**
```rust
// Sync lock in async context
async fn bad() {
    let guard = std_mutex.lock().unwrap(); // ❌ Blocks thread
    async_call().await; // ❌ Still holding lock
}

// block_on in async context
async fn bad() {
    let rt = tokio::runtime::Handle::current();
    rt.block_on(async { /* ... */ }); // ❌ Blocks thread
}
```
