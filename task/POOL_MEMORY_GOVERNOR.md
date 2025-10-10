# POOL_MEMORY_GOVERNOR

**Priority**: HIGH
**Component**: pool/core
**Estimated Effort**: 2 days
**Risk**: Medium
**Dependencies**: POOL_LIFECYCLE_STATE_MACHINE

---

## ⚠️ IMPLEMENTATION STATUS

### EXISTING CODE DISCOVERED

The MemoryGovernor **already exists** at:
- **File**: [`packages/candle/src/pool/core/memory_governor.rs`](../packages/candle/src/pool/core/memory_governor.rs) (526 lines)
- **Status**: ✅ Implemented but **INCORRECT - uses async/tokio**
- **Problem**: Uses `async fn try_allocate()` and `tokio::spawn()` for pressure monitoring

### CRITICAL GAPS TO ADDRESS

| Feature | Required | Current Status | Action Needed |
|---------|----------|----------------|---------------|
| Synchronous API | ✅ REQUIRED | ❌ Uses async/await | **REWRITE to sync** |
| RAII AllocationGuard | ✅ REQUIRED | ❌ Missing | **ADD new struct** |
| Pool Integration | ✅ REQUIRED | ❌ Not used | **INTEGRATE with spawn.rs** |
| Pressure Monitoring | ✅ REQUIRED | ⚠️ Uses tokio::spawn | **REWRITE with std::thread** |
| Atomic Allocation | ✅ REQUIRED | ⚠️ Needs CAS loop | **ADD compare_exchange** |
| Emergency Eviction | ✅ REQUIRED | ⚠️ Logs only | **ADD actual eviction** |

---

## 🔍 CODE ANALYSIS

### What Already Works ✅

The existing implementation at [`memory_governor.rs:1-526`](../packages/candle/src/pool/core/memory_governor.rs) has:

```rust
// ✅ Correct data structures (lines 18-64)
pub enum MemoryPressure { Low, Normal, High, Critical }
pub struct MemoryGovernor {
    total_system_mb: AtomicU64,
    allocated_mb: AtomicU64,
    limit_mb: AtomicU64,
    pressure: Arc<RwLock<MemoryPressure>>,
    allocations: Arc<RwLock<BTreeMap<String, ModelMemory>>>,
    // ... etc
}

// ✅ Correct dependencies
use parking_lot::RwLock;
use sysinfo::{System, SystemExt, ProcessExt};

// ✅ Huge pages support (lines 443-469)
#[cfg(target_os = "linux")]
fn enable_huge_pages_for_allocation(&self, size_mb: usize)

// ✅ NUMA awareness (lines 471-485)
fn get_numa_node(&self) -> Option<usize>
```

### What's Broken ❌

```rust
// ❌ ASYNC API - needs to be synchronous (line 180)
pub async fn try_allocate(&self, size_mb: usize) -> bool {
    let _permit = match self.allocation_sem.try_acquire() {
        // ... uses tokio Semaphore
    }
}

// ❌ TOKIO monitoring thread (line 337)
fn start_pressure_monitor(&self) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(interval);
        // ... needs std::thread::spawn instead
    })
}

// ❌ No RAII guard - manual cleanup required
pub fn release(&self, size_mb: usize) {
    // Caller must remember to call this!
}

// ❌ Not integrated with spawn logic
```

---

## 🎯 IMPLEMENTATION PLAN

### Step 1: Convert to Synchronous API

**File**: [`packages/candle/src/pool/core/memory_governor.rs`](../packages/candle/src/pool/core/memory_governor.rs)

**Changes Required**:

#### Remove Tokio Dependencies (lines 10, 62)

```rust
// ❌ DELETE these imports
use tokio::sync::Semaphore;

// ❌ DELETE this field from MemoryGovernor struct (line 62)
allocation_sem: Arc<Semaphore>,
```

#### Replace Async try_allocate with Sync + CAS Loop (lines 180-235)

**Pattern Reference**: See existing CAS pattern at [`request_queue.rs:373-388`](../packages/candle/src/pool/core/request_queue.rs#L373-L388)

```rust
// REPLACE async fn with synchronous compare-and-swap loop
pub fn try_allocate(&self, size_mb: usize) -> Result<AllocationGuard, MemoryError> {
    let mut current = self.allocated_mb.load(Ordering::Acquire);
    let limit = self.limit_mb.load(Ordering::Acquire);
    let reserved = self.reserved_mb.load(Ordering::Acquire);
    
    loop {
        // Check if allocation would exceed limit
        if current + size_mb as u64 > limit - reserved {
            // Try to find evictable memory
            if let Some(evictable) = self.find_evictable_memory(size_mb) {
                return Err(MemoryError::RequiresEviction(evictable));
            }
            return Err(MemoryError::Exhausted {
                requested: size_mb,
                available: (limit - reserved - current) as usize,
            });
        }
        
        // Try atomic compare-and-swap to reserve memory
        match self.allocated_mb.compare_exchange_weak(
            current,
            current + size_mb as u64,
            Ordering::Release,  // Success: publish reservation
            Ordering::Acquire,  // Failure: get updated value
        ) {
            Ok(_) => {
                // Successfully reserved - return RAII guard
                info!("Allocated {} MB (total: {} MB)", size_mb, current + size_mb as u64);
                self.update_pressure();
                
                return Ok(AllocationGuard {
                    governor: self.clone(),
                    size_mb,
                });
            }
            Err(actual) => {
                // Another thread modified allocated_mb - retry with new value
                current = actual;
            }
        }
    }
}
```

**Why CAS Loop?**
- Prevents race conditions between multiple spawning threads
- Lock-free coordination (no mutex contention)
- Matches existing pattern in [`request_queue.rs:376`](../packages/candle/src/pool/core/request_queue.rs#L376)

#### Replace Tokio Monitoring with std::thread (lines 337-372)

**Pattern Reference**: See existing thread spawn at [`text_embedding.rs:169`](../packages/candle/src/pool/capabilities/text_embedding.rs#L169)

```rust
fn start_pressure_monitor(&self) {
    let governor = self.clone();
    let interval = self.config.pressure_check_interval;
    
    // Use std::thread instead of tokio::spawn
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(interval);
            
            // Refresh system memory info
            {
                let mut sys = governor.system.write();
                sys.refresh_memory();
                let used = sys.used_memory() / 1024 / 1024;
                let total = sys.total_memory() / 1024 / 1024;
                governor.total_system_mb.store(total, Ordering::Release);
            }
            
            // Update pressure
            governor.update_pressure();
            
            // Handle critical pressure
            if governor.get_pressure() == MemoryPressure::Critical {
                governor.handle_critical_pressure();
            }
        }
    });
}
```

**Why std::thread?**
- No tokio dependency required
- Simpler blocking sleep vs tokio intervals
- Matches pool worker pattern

---

### Step 2: Add RAII AllocationGuard

**File**: [`packages/candle/src/pool/core/memory_governor.rs`](../packages/candle/src/pool/core/memory_governor.rs)

**Pattern Reference**: See existing guard at [`types.rs:180-197`](../packages/candle/src/pool/core/types.rs#L180-L197)

**Add after MemoryGovernor struct definition** (~line 65):

```rust
/// RAII guard that automatically releases memory allocation on drop
/// 
/// Prevents memory leaks when worker spawning fails or panics.
/// Follows the same pattern as SpawnGuard in types.rs.
pub struct AllocationGuard {
    governor: MemoryGovernor,
    size_mb: usize,
}

impl Drop for AllocationGuard {
    fn drop(&mut self) {
        // Atomic release - can't fail
        let previous = self.governor.allocated_mb.fetch_sub(
            self.size_mb as u64,
            Ordering::Release
        );
        
        info!(
            "Released {} MB via AllocationGuard (total: {} MB)",
            self.size_mb,
            previous - self.size_mb as u64
        );
        
        // Update pressure after release
        self.governor.update_pressure();
        
        // Return memory to pool if enabled
        if self.governor.config.enable_memory_pools {
            self.governor.return_to_pool(self.size_mb);
        }
    }
}
```

**Why RAII?**
- Automatic cleanup on panic/error (Rust safety)
- Prevents memory tracking leaks
- Matches existing SpawnGuard pattern
- Used by all capability workers (see [`text_embedding.rs:43`](../packages/candle/src/pool/capabilities/text_embedding.rs#L43))

---

### Step 3: Add Error Types

**File**: [`packages/candle/src/pool/core/memory_governor.rs`](../packages/candle/src/pool/core/memory_governor.rs)

**Add after imports** (~line 12):

```rust
#[derive(Debug, Clone)]
pub struct EvictionCandidate {
    pub registry_key: String,
    pub worker_id: u64,
    pub size_mb: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("Memory exhausted: requested {requested} MB, only {available} MB available")]
    Exhausted { requested: usize, available: usize },
    
    #[error("Memory allocation requires eviction")]
    RequiresEviction(Vec<EvictionCandidate>),
}
```

---

### Step 4: Implement Emergency Eviction

**File**: [`packages/candle/src/pool/core/memory_governor.rs`](../packages/candle/src/pool/core/memory_governor.rs)

**Replace log-only version** (~line 356) **with actual eviction**:

```rust
fn handle_critical_pressure(&self) {
    warn!("CRITICAL memory pressure - initiating emergency eviction");
    
    // Target: free 10% of allocated memory
    let target_mb = (self.allocated_mb.load(Ordering::Acquire) / 10) as usize;
    
    if let Some(candidates) = self.find_evictable_memory(target_mb) {
        for candidate in &candidates {
            warn!(
                "Emergency evicting worker {} from {} ({} MB)",
                candidate.worker_id,
                candidate.registry_key,
                candidate.size_mb
            );
        }
        
        // Store eviction candidates for pool to handle
        // (Pool maintenance thread will pick these up)
        let mut eviction_queue = self.eviction_queue.write();
        eviction_queue.extend(candidates);
    } else {
        error!("No evictable workers found despite critical pressure!");
    }
}
```

**Add to MemoryGovernor struct**:

```rust
/// Queue of workers marked for emergency eviction
eviction_queue: Arc<RwLock<Vec<EvictionCandidate>>>,
```

---

### Step 5: Integrate with Pool Spawning

**File**: [`packages/candle/src/pool/core/spawn.rs`](../packages/candle/src/pool/core/spawn.rs)

**Current code** (lines 46-73) does **manual memory calculation**:

```rust
// ❌ CURRENT: Manual calculation without governor
let current_mb = pool.total_memory_mb();
let total_system_mb = query_system_memory_mb();
let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;

if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
    2  // spawn 2 workers
} else if current_mb + per_worker_mb <= memory_limit_mb {
    1  // spawn 1 worker  
} else {
    return Err(PoolError::MemoryExhausted(...));
}
```

**REPLACE with governor integration**:

```rust
// ✅ NEW: Use MemoryGovernor for allocation
let workers_to_spawn = if let Ok(_guard1) = pool.memory_governor.try_allocate(per_worker_mb) {
    // First worker fits
    if let Ok(_guard2) = pool.memory_governor.try_allocate(per_worker_mb) {
        // Second worker also fits - release both guards, will re-allocate in spawn
        drop(_guard1);
        drop(_guard2);
        2
    } else {
        // Only first fits
        drop(_guard1);
        1
    }
} else {
    return Err(PoolError::MemoryExhausted(format!(
        "Memory governor rejected allocation for {}",
        registry_key
    )));
};

// Spawn workers with allocation guards
for worker_idx in 0..workers_to_spawn {
    // Allocate with guard - will auto-release on panic/error
    let _allocation_guard = pool.memory_governor
        .try_allocate(per_worker_mb)
        .map_err(|e| PoolError::MemoryExhausted(e.to_string()))?;
    
    spawn_fn(worker_idx, _allocation_guard)?;
    
    // Guard transferred to worker thread, will drop when worker exits
}
```

**Add to Pool struct** ([`pool.rs:33`](../packages/candle/src/pool/core/pool.rs#L33)):

```rust
/// Memory governor for system-wide coordination
memory_governor: Arc<MemoryGovernor>,
```

---

### Step 6: Worker Thread Integration

**Pattern**: All capability workers must **hold AllocationGuard** until exit

**Example for** [`text_embedding.rs:169`](../packages/candle/src/pool/capabilities/text_embedding.rs#L169):

```rust
pub fn spawn_text_embedding_worker<T, F>(
    &self,
    registry_key: &str,
    model_loader: F,
    per_worker_mb: usize,
    allocation_guard: AllocationGuard,  // ← NEW PARAMETER
) -> Result<(), PoolError>
where
    F: FnOnce() -> Result<T, PoolError> + Send + 'static,
    T: TextEmbeddingCapable + Send + 'static,
{
    // ... existing setup ...
    
    std::thread::spawn(move || {
        // Guard held by worker thread - will drop on exit
        let _memory_guard = allocation_guard;
        
        let model = match model_loader() {
            Ok(m) => m,
            Err(e) => {
                error!("Model load failed: {}", e);
                // _memory_guard drops here, releasing memory
                return;
            }
        };
        
        // ... run worker loop ...
        
        // _memory_guard drops here when worker exits
    });
}
```

**Repeat for all capability modules**:
- [`text_to_text.rs`](../packages/candle/src/pool/capabilities/text_to_text.rs)
- [`vision.rs`](../packages/candle/src/pool/capabilities/vision.rs)
- [`text_to_image.rs`](../packages/candle/src/pool/capabilities/text_to_image.rs)
- [`image_embedding.rs`](../packages/candle/src/pool/capabilities/image_embedding.rs)

---

## 📝 IMPLEMENTATION CHECKLIST

### Core Changes

- [ ] Remove tokio dependencies from `memory_governor.rs`
- [ ] Convert `try_allocate()` to synchronous with CAS loop
- [ ] Convert pressure monitor to `std::thread::spawn`
- [ ] Add `AllocationGuard` struct with `Drop` impl
- [ ] Add `MemoryError` enum
- [ ] Add `eviction_queue` to `MemoryGovernor`
- [ ] Implement actual emergency eviction (not just logging)

### Integration Changes

- [ ] Add `memory_governor: Arc<MemoryGovernor>` to `Pool` struct
- [ ] Update `spawn.rs::ensure_workers_spawned()` to use governor
- [ ] Update all capability `spawn_*_worker()` to accept `AllocationGuard`
- [ ] Update capability worker threads to hold guard until exit

### Cleanup

- [ ] Remove manual memory calculation from `spawn.rs`
- [ ] Remove `Semaphore` from dependencies (no longer needed)
- [ ] Update `MemoryConfig::default()` to remove async-specific settings

---

## 🔗 CODE REFERENCES

### Existing Patterns to Follow

| Pattern | Reference File | Lines | Use For |
|---------|---------------|-------|---------|
| RAII Guard | [`types.rs`](../packages/candle/src/pool/core/types.rs) | 180-197 | AllocationGuard pattern |
| CAS Loop | [`request_queue.rs`](../packages/candle/src/pool/core/request_queue.rs) | 373-388 | Atomic allocation |
| Thread Spawn | [`text_embedding.rs`](../packages/candle/src/pool/capabilities/text_embedding.rs) | 169 | Pressure monitor |
| Drop Cleanup | [`text_embedding.rs`](../packages/candle/src/pool/capabilities/text_embedding.rs) | 43-50 | Worker cleanup |

### Files to Modify

1. **Primary Implementation**:
   - [`packages/candle/src/pool/core/memory_governor.rs`](../packages/candle/src/pool/core/memory_governor.rs) - Rewrite async → sync

2. **Integration Points**:
   - [`packages/candle/src/pool/core/pool.rs`](../packages/candle/src/pool/core/pool.rs) - Add governor field
   - [`packages/candle/src/pool/core/spawn.rs`](../packages/candle/src/pool/core/spawn.rs) - Use governor for allocation

3. **Worker Spawning** (add guard parameter to each):
   - [`packages/candle/src/pool/capabilities/text_embedding.rs`](../packages/candle/src/pool/capabilities/text_embedding.rs)
   - [`packages/candle/src/pool/capabilities/text_to_text.rs`](../packages/candle/src/pool/capabilities/text_to_text.rs)
   - [`packages/candle/src/pool/capabilities/vision.rs`](../packages/candle/src/pool/capabilities/vision.rs)
   - [`packages/candle/src/pool/capabilities/text_to_image.rs`](../packages/candle/src/pool/capabilities/text_to_image.rs)
   - [`packages/candle/src/pool/capabilities/image_embedding.rs`](../packages/candle/src/pool/capabilities/image_embedding.rs)

---

## 🎯 DEFINITION OF DONE

### Functional Requirements

- ✅ Memory allocation is **synchronous** (no async/await)
- ✅ Failed spawns **automatically release** memory via RAII
- ✅ Pressure monitoring runs on **std::thread** (no tokio)
- ✅ Critical pressure triggers **actual eviction** (not just logs)
- ✅ AllocationGuard prevents leaks on **panic/error paths**
- ✅ CAS loop prevents **race conditions** during allocation
- ✅ Pool integration uses governor for **all spawns**

### Code Quality

- ✅ No tokio dependencies in `memory_governor.rs`
- ✅ All capability workers hold `AllocationGuard`
- ✅ Follows existing RAII patterns (`SpawnGuard`, worker `Drop`)
- ✅ Matches atomic patterns (`request_queue.rs`)
- ✅ Clean error handling with `MemoryError` enum

---

## 🚫 OUT OF SCOPE

Per project requirements, the following are **explicitly excluded**:

- ❌ Unit tests / functional tests
- ❌ Benchmarks / performance tests  
- ❌ Documentation generation
- ❌ Examples / demos
- ❌ Integration tests

**Focus**: Implementation only. Verification through manual testing and cargo check.

---

## 📚 DEPENDENCY NOTES

### Task Dependency: POOL_LIFECYCLE_STATE_MACHINE

The task file lists `POOL_LIFECYCLE_STATE_MACHINE` as a dependency. However, based on code analysis:

**Current State**: Worker lifecycle tracking exists in basic form
- Workers have health checks ([`types.rs:96`](../packages/candle/src/pool/core/types.rs#L96))
- Failed spawns are detected
- Memory cleanup happens via Drop

**Recommendation**: Memory governor can be implemented **independently** of state machine. The RAII pattern provides sufficient cleanup guarantees.

### Cargo.toml Dependencies

**Already available** in [`packages/candle/Cargo.toml`](../packages/candle/Cargo.toml):

```toml
sysinfo = "0.30"          # Line 232 - system memory queries
parking_lot = "0.12.5"     # Line 251 - RwLock
crossbeam = "0.8.4"        # Line 237 - atomic utilities
```

**To remove** (tokio only needed for other parts of candle):
- No changes needed - tokio can remain for other modules

---

## 💡 KEY INSIGHTS FROM CODE ANALYSIS

### Why Synchronous is Better

1. **Pool workers use std::thread** - no async runtime needed
2. **Memory operations are CPU-bound** - no I/O to await
3. **Simpler error handling** - no async fn propagation
4. **Lower latency** - no tokio scheduler overhead

### Why RAII is Critical

From existing code patterns:
- **SpawnGuard** prevents duplicate spawning ([`types.rs:180`](../packages/candle/src/pool/core/types.rs#L180))
- **Worker Drop** cleans up registries ([`text_embedding.rs:43`](../packages/candle/src/pool/capabilities/text_embedding.rs#L43))
- **AllocationGuard** must follow same pattern for memory

### Integration Strategy

The pool already has excellent patterns:
- Atomic operations for coordination
- RAII guards for cleanup  
- Thread-based workers

**Memory governor should match these patterns**, not introduce new paradigms.

---

**Last Updated**: 2025-10-10 (Automated analysis by Claude Code)
**Implementation File**: [`packages/candle/src/pool/core/memory_governor.rs`](../packages/candle/src/pool/core/memory_governor.rs)
