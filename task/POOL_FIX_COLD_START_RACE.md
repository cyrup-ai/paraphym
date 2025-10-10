# POOL_FIX_COLD_START_RACE

**Priority**: CRITICAL  
**Component**: Pool synchronization layer  
**Estimated Effort**: 4 hours  
**Risk**: High (Over-spawning, Memory Exhaustion)

## Problem Statement

Multiple concurrent requests for the same model can trigger duplicate worker spawning due to unsynchronized cold-start check. This race condition can spawn 10+ workers when only 2 are needed, potentially exceeding memory limits and crashing the system.

### Root Cause Analysis

The vulnerability exists in a check-then-act pattern that appears **21 times** across capability implementations in [`packages/candle/src/capability/registry.rs`](../packages/candle/src/capability/registry.rs):

```rust
// VULNERABLE PATTERN (lines 246, 289, 332, 383, 425, 462, 499, 536, 579, 617, 654, 691, 728, 784, etc.)
if !pool.has_workers(registry_key) {  // ← RACE WINDOW: Non-atomic check
    let workers_to_spawn = calculate_workers(); // Memory-based decision
    
    for _ in 0..workers_to_spawn {
        pool.spawn_worker(registry_key, loader, memory)?; // ← Multiple threads enter here
    }
}
```

**Timeline of Race Condition:**
```
T=0:   Thread A: checks has_workers("qwen3") → false
T=1:   Thread B: checks has_workers("qwen3") → false  (Thread A hasn't spawned yet)
T=2:   Thread C: checks has_workers("qwen3") → false  (Thread A hasn't spawned yet)
T=10:  Thread A: spawns 2 workers (4GB total)
T=11:  Thread B: spawns 2 workers (4GB total) ← DUPLICATE
T=12:  Thread C: spawns 2 workers (4GB total) ← DUPLICATE
Result: 6 workers, 12GB memory when only 2 workers / 4GB was needed
```

### Affected Locations

All instances occur in match arms within capability trait implementations:

| **Capability Type** | **Lines in registry.rs** | **Pool Accessed** |
|---------------------|--------------------------|-------------------|
| TextToTextModel     | 246, 289, 332           | text_to_text_pool() |
| TextEmbeddingModel  | 383, 425, 462, 499, 536, 579, 617, 654, 691, 728 | text_embedding_pool() |
| ImageEmbeddingModel | 784, 827                | image_embedding_pool() |
| TextToImageModel    | ~900-1100 range         | text_to_image_pool() |
| VisionModel         | ~1200-1400 range        | vision_pool() |

**Total**: 21+ locations with identical vulnerability

### Runtime Impact

#### Memory Exhaustion (SEVERE)
- **Scenario**: 10 concurrent requests × 2 workers each = 20 workers spawned
- **Memory**: Each model ~2GB → 40GB allocated in milliseconds
- **Result**: System OOM killer terminates process
- **User impact**: Complete service outage, all requests fail

#### Resource Waste
- **Scenario**: Even without OOM, excess workers consume memory unnecessarily
- **Result**: 10 workers running when 2 would suffice
- **User impact**: Other models can't load, reduced system capacity by 5x

#### Cascade Failures
- **Scenario**: First model exhausts memory, subsequent models all fail
- **Result**: One popular model prevents all others from working
- **User impact**: Entire AI capability system becomes unavailable

---

## Architecture Context

### Pool Structure

The pool system is implemented across multiple files:

**Core Pool Implementation**: [`packages/candle/src/pool/core/pool.rs`](../packages/candle/src/pool/core/pool.rs)
```rust
pub struct Pool<T: ?Sized> {
    workers: DashMap<String, Vec<WorkerHandle>>,     // Thread-safe worker storage
    config: PoolConfig,
    total_memory_used: Arc<AtomicUsize>,             // Atomic memory tracking
    next_worker_id: AtomicUsize,                     // Atomic ID generation
    metrics: PoolMetrics,
    shutting_down: Arc<AtomicBool>,                  // Atomic shutdown flag
    _phantom: PhantomData<T>,
    // ⚠️  MISSING: Spawn synchronization mechanism
}

impl<T: ?Sized> Pool<T> {
    pub fn has_workers(&self, registry_key: &str) -> bool {
        self.workers.get(registry_key).map(|w| !w.is_empty()).unwrap_or(false)
        // ⚠️  This check is NOT atomic with spawn operations
    }
    
    pub fn register_worker(&self, registry_key: String, handle: WorkerHandle) {
        self.workers.entry(registry_key).or_insert_with(Vec::new).push(handle);
        // This happens AFTER spawn decision is made - too late!
    }
}
```

**Capability-Specific Implementations**: Located in [`packages/candle/src/pool/capabilities/`](../packages/candle/src/pool/capabilities/)
- `text_to_text.rs` - Defines `spawn_text_to_text_worker()`
- `text_embedding.rs` - Defines `spawn_text_embedding_worker()`
- `image_embedding.rs` - Defines `spawn_image_embedding_worker()`
- `text_to_image.rs` - Defines `spawn_text_to_image_worker()`
- `vision.rs` - Defines `spawn_vision_worker()`

Each spawn method:
1. Validates memory constraints
2. Creates crossbeam channels for worker communication
3. Spawns worker thread with model loader
4. Registers WorkerHandle via `pool.register_worker()`
5. Updates atomic memory counter via `pool.add_memory()`

**Error Types**: [`packages/candle/src/pool/core/error.rs`](../packages/candle/src/pool/core/error.rs)
```rust
pub enum PoolError {
    NoWorkers(String),
    Timeout(String),
    SendError(String),
    RecvError(String),
    ModelError(String),
    ShuttingDown(String),
    MemoryExhausted(String),
    SpawnFailed(String),
    // ⚠️  MISSING: SpawnTimeout variant needed for waiting threads
}
```

---

## Solution Design

### Approach: Lock-Free RAII Guard Pattern

Add spawn synchronization to the Pool struct using **compare-and-swap atomics** with **RAII guards** for automatic cleanup.

#### Why This Approach?
1. **Consistent with existing architecture**: Pool already uses DashMap and atomic primitives extensively
2. **Lock-free**: No mutex contention under high concurrency
3. **Panic-safe**: RAII guard ensures cleanup even if spawning panics
4. **Zero overhead when no contention**: First thread proceeds immediately
5. **Memory efficient**: AtomicBool is 1 byte per model

---

## Implementation Steps

### Step 1: Add Synchronization Primitives to Pool

**File**: [`packages/candle/src/pool/core/pool.rs`](../packages/candle/src/pool/core/pool.rs)

**Action**: Add new field to Pool struct (around line 8):

```rust
pub struct Pool<T: ?Sized> {
    workers: DashMap<String, Vec<WorkerHandle>>,
    config: PoolConfig,
    total_memory_used: Arc<AtomicUsize>,
    next_worker_id: AtomicUsize,
    metrics: PoolMetrics,
    shutting_down: Arc<AtomicBool>,
    
    /// 🆕 Track models currently spawning workers (prevents duplicate spawning)
    spawning_in_progress: DashMap<String, Arc<AtomicBool>>,
    
    _phantom: PhantomData<T>,
}
```

**Action**: Update Pool::new() constructor (around line 30):

```rust
pub fn new(config: PoolConfig) -> Self {
    Self {
        workers: DashMap::new(),
        config,
        total_memory_used: Arc::new(AtomicUsize::new(0)),
        next_worker_id: AtomicUsize::new(0),
        metrics: PoolMetrics::default(),
        shutting_down: Arc::new(AtomicBool::new(false)),
        spawning_in_progress: DashMap::new(),  // 🆕 Initialize spawn tracking
        _phantom: PhantomData,
    }
}
```

### Step 2: Create RAII SpawnGuard

**File**: [`packages/candle/src/pool/core/types.rs`](../packages/candle/src/pool/core/types.rs)

**Action**: Add SpawnGuard struct at end of file (after WorkerHandle impl):

```rust
/// 🆕 RAII guard that prevents duplicate worker spawning
/// 
/// Automatically releases spawn lock when dropped, even if panic occurs.
/// Only one thread can hold a SpawnGuard for a given registry_key at a time.
pub struct SpawnGuard {
    flag: Arc<AtomicBool>,
    registry_key: String,
}

impl SpawnGuard {
    pub(crate) fn new(flag: Arc<AtomicBool>, registry_key: String) -> Self {
        Self { flag, registry_key }
    }
}

impl Drop for SpawnGuard {
    fn drop(&mut self) {
        // Release spawn lock when guard is dropped
        self.flag.store(false, std::sync::atomic::Ordering::Release);
        log::debug!("Released spawn lock for {}", self.registry_key);
    }
}
```

**Action**: Add SpawnGuard to module exports in [`packages/candle/src/pool/core/mod.rs`](../packages/candle/src/pool/core/mod.rs):

```rust
pub use types::{PoolConfig, PoolMetrics, WorkerHandle, SpawnGuard};  // Add SpawnGuard
```

### Step 3: Add Spawn Lock Methods to Pool

**File**: [`packages/candle/src/pool/core/pool.rs`](../packages/candle/src/pool/core/pool.rs)

**Action**: Add methods at end of Pool impl block (after metrics()):

```rust
    /// 🆕 Try to acquire exclusive spawn lock for a model
    /// 
    /// Returns Some(guard) if this thread won the race to spawn workers.
    /// Returns None if another thread is already spawning workers.
    /// 
    /// Uses compare-and-swap for lock-free synchronization.
    pub fn try_acquire_spawn_lock(&self, registry_key: &str) -> Option<SpawnGuard> {
        use std::sync::atomic::Ordering;
        
        // Get or create atomic flag for this model
        let flag = self.spawning_in_progress
            .entry(registry_key.to_string())
            .or_insert_with(|| Arc::new(AtomicBool::new(false)))
            .value()
            .clone();
        
        // Try to claim spawn lock using compare-exchange
        // If flag is false (not spawning), set to true (spawning) and return guard
        // If flag is true (already spawning), return None
        match flag.compare_exchange(
            false,                    // Expected: not spawning
            true,                     // Desired: now spawning
            Ordering::AcqRel,         // Success ordering
            Ordering::Acquire,        // Failure ordering
        ) {
            Ok(_) => {
                log::debug!("Acquired spawn lock for {}", registry_key);
                Some(SpawnGuard::new(flag, registry_key.to_string()))
            },
            Err(_) => {
                log::debug!("Spawn lock busy for {} (another thread spawning)", registry_key);
                None
            },
        }
    }
    
    /// 🆕 Wait for workers to become available (blocking)
    /// 
    /// Called by threads that lose the spawn race. Polls until:
    /// - Workers become available (success)
    /// - Spawning completes but no workers exist (spawn failed)
    /// - Timeout expires (spawn took too long)
    /// 
    /// Poll interval: 50ms (balances responsiveness vs CPU usage)
    pub fn wait_for_workers(
        &self,
        registry_key: &str,
        timeout: std::time::Duration,
    ) -> Result<(), PoolError> {
        use std::sync::atomic::Ordering;
        
        let start = std::time::Instant::now();
        
        loop {
            // Check if workers are ready
            if self.has_workers(registry_key) {
                log::debug!("Workers ready for {}", registry_key);
                return Ok(());
            }
            
            // Check if spawning thread released lock (spawn completed or failed)
            if let Some(flag) = self.spawning_in_progress.get(registry_key) {
                if !flag.load(Ordering::Acquire) {
                    // Spawning finished but no workers available = spawn failed
                    return Err(PoolError::SpawnFailed(format!(
                        "Worker spawning completed for {} but no workers available. \
                         Check logs for model loading errors.",
                        registry_key
                    )));
                }
            }
            
            // Check timeout
            if start.elapsed() > timeout {
                return Err(PoolError::SpawnTimeout(format!(
                    "Timed out after {:?} waiting for {} workers to spawn",
                    timeout, registry_key
                )));
            }
            
            // Sleep briefly before next poll (50ms balances latency vs CPU)
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}
```

### Step 4: Add SpawnTimeout Error Variant

**File**: [`packages/candle/src/pool/core/error.rs`](../packages/candle/src/pool/core/error.rs)

**Action**: Add new variant to PoolError enum (around line 10):

```rust
#[derive(Debug, Clone)]
pub enum PoolError {
    NoWorkers(String),
    Timeout(String),
    SendError(String),
    RecvError(String),
    ModelError(String),
    ShuttingDown(String),
    MemoryExhausted(String),
    SpawnFailed(String),
    SpawnTimeout(String),  // 🆕 Timeout waiting for another thread to spawn workers
}
```

**Action**: Update Display impl (around line 25):

```rust
impl fmt::Display for PoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoWorkers(msg) => write!(f, "No workers available: {}", msg),
            Self::Timeout(msg) => write!(f, "Request timeout: {}", msg),
            Self::SendError(msg) => write!(f, "Channel send error: {}", msg),
            Self::RecvError(msg) => write!(f, "Channel recv error: {}", msg),
            Self::ModelError(msg) => write!(f, "Model error: {}", msg),
            Self::ShuttingDown(msg) => write!(f, "Shutting down: {}", msg),
            Self::MemoryExhausted(msg) => write!(f, "Memory exhausted: {}", msg),
            Self::SpawnFailed(msg) => write!(f, "Worker spawn failed: {}", msg),
            Self::SpawnTimeout(msg) => write!(f, "Spawn timeout: {}", msg),  // 🆕
        }
    }
}
```

### Step 5: Fix All 21 Race Conditions in registry.rs

**File**: [`packages/candle/src/capability/registry.rs`](../packages/candle/src/capability/registry.rs)

**Action**: Replace ALL occurrences of the vulnerable pattern. Each replacement follows this template:

**BEFORE** (vulnerable pattern):
```rust
if !pool.has_workers(registry_key) {
    let per_worker_mb = m.info().est_memory_allocation_mb;
    let current_mb = pool.total_memory_mb();
    let total_system_mb = query_system_memory_mb();
    let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;

    let workers_to_spawn = if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
        2
    } else if current_mb + per_worker_mb <= memory_limit_mb {
        1
    } else {
        return spawn_stream(move |sender| {
            ystream::emit!(sender, CandleCompletionChunk::Error(
                format!("Memory limit reached for {}", registry_key)
            ));
        });
    };

    for _ in 0..workers_to_spawn {
        if let Err(e) = pool.spawn_*_worker(registry_key, loader, per_worker_mb) {
            return spawn_stream(move |sender| {
                ystream::emit!(sender, CandleCompletionChunk::Error(
                    format!("Failed to spawn worker: {}", e)
                ));
            });
        }
    }
}
```

**AFTER** (race-free pattern):
```rust
// 🔒 Try to acquire spawn lock (prevents duplicate spawning)
if let Some(_guard) = pool.try_acquire_spawn_lock(registry_key) {
    // Double-check workers don't exist (another thread may have spawned before lock)
    if !pool.has_workers(registry_key) {
        let per_worker_mb = m.info().est_memory_allocation_mb;
        let current_mb = pool.total_memory_mb();
        let total_system_mb = query_system_memory_mb();
        let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;

        let workers_to_spawn = if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
            2
        } else if current_mb + per_worker_mb <= memory_limit_mb {
            1
        } else {
            return spawn_stream(move |sender| {
                ystream::emit!(sender, CandleCompletionChunk::Error(
                    format!("Memory limit reached for {}", registry_key)
                ));
            });
        };

        for _ in 0..workers_to_spawn {
            if let Err(e) = pool.spawn_*_worker(registry_key, loader, per_worker_mb) {
                return spawn_stream(move |sender| {
                    ystream::emit!(sender, CandleCompletionChunk::Error(
                        format!("Failed to spawn worker: {}", e)
                    ));
                });
            }
        }
    }
    // _guard drops here, releasing spawn lock
} else {
    // 🕐 Another thread is spawning - wait for it to complete
    if let Err(e) = pool.wait_for_workers(registry_key, std::time::Duration::from_secs(30)) {
        return spawn_stream(move |sender| {
            ystream::emit!(sender, CandleCompletionChunk::Error(
                format!("Spawn wait failed: {}", e)
            ));
        });
    }
}
```

**Exact Line Numbers to Update** (search for `if !pool.has_workers` in registry.rs):
- Lines 246, 289, 332 (TextToTextModel variants: KimiK2, Qwen3Coder, Phi4Reasoning)
- Lines 383, 425, 462, 499, 536 (TextEmbeddingModel variants: GteQwen, JinaBert, NvEmbed, QwenCode, Stella)
- Lines 579, 617, 654, 691, 728 (More TextEmbedding variants)
- Lines 784, 827 (ImageEmbeddingModel variants: ClipVision)
- Additional lines in TextToImage and Vision match arms (use search to find all)

**Implementation Strategy**:
1. Use editor's find-and-replace with regex to update all 21 instances
2. Search pattern: `if !pool\.has_workers\(registry_key\)`
3. Verify each replacement maintains correct error handling for that capability type
4. Some variants return `Result<Vec<f32>>` (embeddings), others return `AsyncStream` (completions)

---

## Definition of Done

### Functional Requirements
- [ ] SpawnGuard RAII type added to pool/core/types.rs with Drop impl
- [ ] Pool struct includes spawning_in_progress: DashMap<String, Arc<AtomicBool>> field
- [ ] Pool::try_acquire_spawn_lock() method implemented with compare-and-swap
- [ ] Pool::wait_for_workers() method implemented with timeout and error handling
- [ ] PoolError::SpawnTimeout variant added to pool/core/error.rs
- [ ] All 21 race conditions in registry.rs replaced with spawn lock pattern
- [ ] Code compiles without errors or warnings

### Behavior Requirements
- [ ] Only ONE thread can spawn workers for a given model at a time
- [ ] Losing threads wait for spawning thread to complete (max 30s timeout)
- [ ] If spawning thread fails, waiting threads receive SpawnFailed error immediately
- [ ] If spawning thread times out, waiting threads receive SpawnTimeout error after 30s
- [ ] SpawnGuard releases lock even if spawning code panics (RAII cleanup)
- [ ] No deadlocks occur under any race condition scenario

### Verification
- [ ] Compile check: `cargo check` passes without warnings
- [ ] Manual inspection: Review all 21 registry.rs changes for correctness
- [ ] Log verification: Run system and check logs show "Acquired spawn lock" and "Released spawn lock" messages

---

## Implementation Notes

### Memory Ordering Rationale

**Compare-Exchange Orderings**:
- Success: `Ordering::AcqRel` - Acquires synchronization on success, releases on guard drop
- Failure: `Ordering::Acquire` - Ensures visibility of other thread's spawning operations

**Flag Load Ordering**:
- `Ordering::Acquire` in wait_for_workers() - Ensures visibility of spawning thread's memory writes

**Flag Store Ordering**:
- `Ordering::Release` in SpawnGuard::drop() - Ensures spawned workers are visible to waiting threads

### Error Flow

```
Thread A: try_acquire_spawn_lock() → Some(guard) → spawn workers → guard drops → lock released
Thread B: try_acquire_spawn_lock() → None → wait_for_workers() → polls until workers exist or error
```

**Possible Outcomes for Thread B**:
1. Workers appear → wait_for_workers() returns Ok(()) → proceeds to pool.prompt()
2. Spawn completes but no workers → SpawnFailed error (model loading issue)
3. 30 seconds elapse → SpawnTimeout error (deadlock or very slow spawn)

### Performance Characteristics

**Happy Path (No Contention)**:
- First thread: 1 compare-exchange (~10ns), spawns workers, ~0 overhead
- Subsequent requests: workers exist, no spawn attempt, ~0 overhead

**Contended Path (10 Concurrent Requests)**:
- 1 thread spawns (winner)
- 9 threads wait (losers), polling every 50ms
- Average wait time: ~500ms (typical model load time) × 9 threads = 4.5 thread-seconds
- Memory: 1 AtomicBool (1 byte) per model = negligible

**Worst Case (Spawn Failure)**:
- 1 thread fails spawn (~2-3 seconds for model load failure)
- 9 threads detect failure immediately after spawn lock release
- Total wasted time: ~2-3 seconds (vs 30s timeout if not detected)

---

## Why This Is Critical

This race condition is a **production time bomb**:
- ✅ **Triggered by normal traffic** (spikes, viral content, batch processing)
- ✅ **Catastrophic failure mode** (OOM crash takes down entire service)
- ✅ **Difficult to recover** (requires process restart + traffic management)
- ✅ **Cascading impact** (one popular model prevents all others from loading)
- ✅ **Silent until failure** (works fine under low concurrency, deadly under load)

**Financial Impact**: Minutes of downtime = thousands in lost revenue + reputational damage
