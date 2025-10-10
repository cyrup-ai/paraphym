# POOL_LIFECYCLE_STATE_MACHINE

**Priority**: CRITICAL  
**Component**: pool/core, pool/capabilities
**Estimated Effort**: 2 days (50% complete)
**Risk**: Medium
**Dependencies**: POOL_UNIFIED_STORAGE (can proceed independently with temporary dual storage)

## Implementation Progress

### ✅ COMPLETED (50% of task)

1. **WorkerHandle State Field Added** - `pool/core/types.rs`
   - Added `state: Arc<AtomicU32>` field to WorkerHandle struct
   - Added Clone derive to WorkerHandle
   - Implemented helper methods: `get_state()`, `set_state()`, `can_accept_requests()`, `is_evictable()`

2. **Text Embedding Worker Updated** - `pool/capabilities/text_embedding.rs`
   - Added Clone derive to TextEmbeddingWorkerHandle
   - Added state transitions in spawn method:
     - Spawning → Loading → Ready (success path)
     - Spawning → Loading → Failed (failure path with memory cleanup)
   - Updated worker loop with state transitions:
     - Ready/Idle → Processing → Ready (request handling)
     - Ready → Idle (after 5 minutes inactive)
     - Any → Evicting → Dead (shutdown)
   - Passes registry_key to worker function for state lookups

3. **Text-to-Text Worker Updated** - `pool/capabilities/text_to_text.rs`  
   - Applied same pattern as text_embedding
   - Clone derive, spawn transitions, loop transitions
   - Memory cleanup on model load failure

### ⚠️ IN PROGRESS (partially started)

4. **Image Embedding Worker** - `pool/capabilities/image_embedding.rs`
   - Clone derive added to struct
   - Still needs: spawn transitions, worker loop transitions

### ❌ NOT STARTED (50% remaining)

5. **Vision Worker** - `pool/capabilities/vision.rs`
6. **Text-to-Image Worker** - `pool/capabilities/text_to_image.rs`
7. **Maintenance Eviction Logic** - `pool/maintenance.rs`
8. **Pool Validation Logic** - `pool/core/pool.rs`
9. **Export WorkerState** - `pool/core/mod.rs` and `pool/mod.rs`

---

## Existing Code Analysis

### ✅ Already Implemented

The state machine infrastructure **already exists** in the codebase:

1. **[`pool/core/worker_state.rs`](../packages/candle/src/pool/core/worker_state.rs)** (lines 12-23)
   - Complete `WorkerState` enum: Spawning, Loading, Ready, Processing, Idle, Evicting, Dead, Failed
   - `From<u32>` conversion for atomic storage
   - `UnifiedWorkerHandle<Req, Resp>` with full state tracking

2. **[`pool/core/orchestrator.rs`](../packages/candle/src/pool/core/orchestrator.rs)** (lines 252-276)
   - State transition patterns during model loading
   - Failure cleanup with state tracking
   - Example implementation of lifecycle callbacks

3. **[`pool/ULTIMATE_SOLUTION.md`](../packages/candle/src/pool/ULTIMATE_SOLUTION.md)** (lines 15-31)
   - Design vision for complete lifecycle management
   - Valid state transition diagram
   - Production-grade features roadmap

### ❌ Not Yet Integrated

The **working pool code** does NOT use state machines:

1. **[`pool/capabilities/text_embedding.rs`](../packages/candle/src/pool/capabilities/text_embedding.rs)**
   - Uses simple `WorkerHandle` from `types.rs` (no state field)
   - Worker loop has no state transitions (lines 69-119)
   - No failure cleanup or lifecycle tracking

2. **[`pool/capabilities/text_to_text.rs`](../packages/candle/src/pool/capabilities/text_to_text.rs)**
   - Same simple worker pattern
   - No state tracking in spawn or loop (lines 67-95, 238-318)

3. **[`pool/core/types.rs`](../packages/candle/src/pool/core/types.rs)** (lines 54-119)
   - `WorkerHandle` has no `state: Arc<AtomicU32>` field
   - `is_alive()` uses health ping, not state checking

4. **[`pool/maintenance.rs`](../packages/candle/src/pool/maintenance.rs)** (lines 94-108)
   - Eviction logic doesn't check worker state
   - No awareness of Loading/Failed states

---

## Core Objective

**Integrate the existing `WorkerState` machine into the working pool implementation** to:

1. Track worker lifecycle (Spawning → Loading → Ready → Processing → Dead/Failed)
2. Clean up memory correctly when model loading fails
3. Prevent eviction of workers in Loading or Processing states
4. Provide visibility into worker health via state

**This is NOT about**:
- Implementing the full `WorkerOrchestrator` from orchestrator.rs (incomplete/experimental)
- Adding advanced features (circuit breakers, request queues, memory governor)
- Replacing the working pool system

**This IS about**:
- Adding a single `state: Arc<AtomicU32>` field to `WorkerHandle`
- Updating worker spawn/loop code to set states
- Using state for smarter eviction and health checks

---

## Implementation Plan

### Step 1: Update WorkerHandle in `pool/core/types.rs`

**File**: `packages/candle/src/pool/core/types.rs`

**Add state field** to `WorkerHandle` struct (after line 54):

```rust
// Line 54-66 (modify)
pub struct WorkerHandle {
    pub pending_requests: Arc<AtomicUsize>,
    pub last_used: Arc<AtomicU64>,
    pub worker_id: usize,
    pub shutdown_tx: Sender<()>,
    pub per_worker_mb: usize,
    pub health_tx: Sender<HealthPing>,
    pub health_rx: Receiver<HealthPong>,
    
    // NEW: Add state tracking
    pub state: Arc<AtomicU32>,  // WorkerState as u32
}
```

**Update constructor** (after line 67):

```rust
impl WorkerHandle {
    pub fn new(
        worker_id: usize,
        shutdown_tx: Sender<()>,
        per_worker_mb: usize,
        health_tx: Sender<HealthPing>,
        health_rx: Receiver<HealthPong>,
    ) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            pending_requests: Arc::new(AtomicUsize::new(0)),
            last_used: Arc::new(AtomicU64::new(now)),
            worker_id,
            shutdown_tx,
            per_worker_mb,
            health_tx,
            health_rx,
            state: Arc::new(AtomicU32::new(0)), // Start in Spawning state
        }
    }
}
```

**Add state helper methods** (after line 119):

```rust
impl WorkerHandle {
    /// Get current worker state
    pub fn get_state(&self) -> WorkerState {
        use crate::pool::core::worker_state::WorkerState;
        let state_val = self.state.load(std::sync::atomic::Ordering::Acquire);
        WorkerState::from(state_val)
    }
    
    /// Set worker state (atomic)
    pub fn set_state(&self, new_state: WorkerState) {
        self.state.store(new_state as u32, std::sync::atomic::Ordering::Release);
    }
    
    /// Check if worker can accept requests
    pub fn can_accept_requests(&self) -> bool {
        use crate::pool::core::worker_state::WorkerState;
        matches!(
            self.get_state(),
            WorkerState::Ready | WorkerState::Processing | WorkerState::Idle
        )
    }
    
    /// Check if worker should be evicted
    pub fn is_evictable(&self) -> bool {
        use crate::pool::core::worker_state::WorkerState;
        matches!(
            self.get_state(),
            WorkerState::Ready | WorkerState::Idle
        )
    }
}
```

### Step 2: Update Text Embedding Worker Spawning

**File**: `packages/candle/src/pool/capabilities/text_embedding.rs`

**Modify `spawn_text_embedding_worker`** (lines 136-226):

Add state transitions in the spawned thread:

```rust
// Around line 178 (inside std::thread::spawn)
std::thread::spawn(move || {
    use crate::pool::core::worker_state::WorkerState;
    
    // Transition: Spawning -> Loading
    if let Some(workers) = TEXT_EMBEDDING_WORKERS.get(&registry_key_clone) {
        if let Some(worker) = workers.iter().find(|w| w.core.worker_id == worker_id) {
            worker.core.set_state(WorkerState::Loading);
        }
    }
    
    // Load model
    let model = match model_loader() {
        Ok(m) => {
            // Transition: Loading -> Ready
            if let Some(workers) = TEXT_EMBEDDING_WORKERS.get(&registry_key_clone) {
                if let Some(worker) = workers.iter().find(|w| w.core.worker_id == worker_id) {
                    worker.core.set_state(WorkerState::Ready);
                    log::info!("TextEmbedding worker {} ready", worker_id);
                }
            }
            m
        }
        Err(e) => {
            // Transition: Loading -> Failed
            log::error!("TextEmbedding worker {} failed: {}", worker_id, e);
            
            if let Some(workers) = TEXT_EMBEDDING_WORKERS.get(&registry_key_clone) {
                if let Some(worker) = workers.iter().find(|w| w.core.worker_id == worker_id) {
                    worker.core.set_state(WorkerState::Failed);
                    
                    // Clean up memory tracking (CRITICAL FIX for issue #11 from review)
                    // This prevents memory leak when model loading fails
                    text_embedding_pool().remove_memory(per_worker_mb);
                }
            }
            
            return; // Exit thread without running worker loop
        }
    };

    text_embedding_worker(
        model,
        embed_rx,
        batch_embed_rx,
        shutdown_rx,
        health_rx_worker_clone,
        health_tx_main_clone,
        worker_id,
    );
    
    // Transition: * -> Dead (when worker loop exits)
    if let Some(workers) = TEXT_EMBEDDING_WORKERS.get(&registry_key_clone) {
        if let Some(worker) = workers.iter().find(|w| w.core.worker_id == worker_id) {
            worker.core.set_state(WorkerState::Dead);
        }
    }
});
```

### Step 3: Update Text Embedding Worker Loop

**File**: `packages/candle/src/pool/capabilities/text_embedding.rs`

**Modify `text_embedding_worker`** function (lines 69-119):

Add state transitions during request processing:

```rust
pub fn text_embedding_worker<T: TextEmbeddingCapable>(
    model: T,
    embed_rx: Receiver<EmbedRequest>,
    batch_embed_rx: Receiver<BatchEmbedRequest>,
    shutdown_rx: Receiver<()>,
    health_rx: Receiver<HealthPing>,
    health_tx: Sender<HealthPong>,
    worker_id: usize,
) {
    use std::time::{SystemTime, UNIX_EPOCH, Duration};
    use crate::pool::core::worker_state::WorkerState;
    
    // Track last activity for idle detection
    let mut last_activity = SystemTime::now();
    let idle_threshold = Duration::from_secs(300); // 5 minutes
    
    // Helper to get worker handle for state updates
    let get_worker = || {
        TEXT_EMBEDDING_WORKERS.get("current_registry_key")
            .and_then(|workers| workers.iter().find(|w| w.core.worker_id == worker_id))
    };
    
    loop {
        select! {
            recv(embed_rx) -> req => {
                if let Ok(req) = req {
                    // Transition: Ready/Idle -> Processing
                    if let Some(worker) = get_worker() {
                        worker.core.set_state(WorkerState::Processing);
                    }
                    
                    let result = model.embed(&req.text, req.task)
                        .map_err(|e| PoolError::ModelError(e.to_string()));
                    let _ = req.response.send(result);
                    
                    // Transition: Processing -> Ready
                    if let Some(worker) = get_worker() {
                        worker.core.set_state(WorkerState::Ready);
                    }
                    
                    last_activity = SystemTime::now();
                }
            }
            recv(batch_embed_rx) -> req => {
                if let Ok(req) = req {
                    // Transition: Ready/Idle -> Processing
                    if let Some(worker) = get_worker() {
                        worker.core.set_state(WorkerState::Processing);
                    }
                    
                    let result = model.batch_embed(&req.texts, req.task)
                        .map_err(|e| PoolError::ModelError(e.to_string()));
                    let _ = req.response.send(result);
                    
                    // Transition: Processing -> Ready
                    if let Some(worker) = get_worker() {
                        worker.core.set_state(WorkerState::Ready);
                    }
                    
                    last_activity = SystemTime::now();
                }
            }
            recv(health_rx) -> ping => {
                if ping.is_ok() {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(0);
                    
                    let pong = HealthPong {
                        worker_id,
                        timestamp: now,
                        queue_depth: embed_rx.len() + batch_embed_rx.len(),
                    };
                    
                    let _ = health_tx.send(pong);
                }
            }
            recv(shutdown_rx) -> _ => {
                log::info!("TextEmbedding worker {} shutting down", worker_id);
                
                // Transition: * -> Evicting
                if let Some(worker) = get_worker() {
                    worker.core.set_state(WorkerState::Evicting);
                }
                
                break;
            }
            default(Duration::from_secs(1)) => {
                // Check for idle timeout
                if last_activity.elapsed().unwrap_or(Duration::ZERO) > idle_threshold {
                    if let Some(worker) = get_worker() {
                        // Transition: Ready -> Idle
                        if matches!(worker.core.get_state(), WorkerState::Ready) {
                            worker.core.set_state(WorkerState::Idle);
                        }
                    }
                }
            }
        }
    }
}
```

### Step 4: Update Text-to-Text Worker (Same Pattern)

**File**: `packages/candle/src/pool/capabilities/text_to_text.rs`

Apply the same pattern as text_embedding:

1. Add state transitions in spawn thread (lines 147-178)
2. Add state transitions in worker loop (lines 67-95)
3. Add cleanup on model load failure
4. Track idle state with 5-minute timeout

**Reference pattern**: See text_embedding.rs changes above - apply identically.

### Step 5: Update Remaining Capabilities

**Files to update** (same pattern):
- `pool/capabilities/image_embedding.rs`
- `pool/capabilities/vision.rs`
- `pool/capabilities/text_to_image.rs`

**Pattern**: Copy the state transition logic from text_embedding.rs, adapting channel names.

### Step 6: Update Maintenance Eviction Logic

**File**: `packages/candle/src/pool/maintenance.rs`

**Modify eviction check** (around line 15):

```rust
// BEFORE (line 15-29):
fn all_workers_idle(workers: &[WorkerHandle], idle_threshold_secs: u64) -> bool {
    if workers.is_empty() {
        return false;
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    workers.iter().all(|w| {
        let pending = w.pending_requests.load(Ordering::Acquire);
        let last_used = w.last_used.load(Ordering::Acquire);
        let idle_duration = now.saturating_sub(last_used);

        pending == 0 && idle_duration >= idle_threshold_secs
    })
}

// AFTER:
fn all_workers_idle(workers: &[WorkerHandle], idle_threshold_secs: u64) -> bool {
    use crate::pool::core::worker_state::WorkerState;
    
    if workers.is_empty() {
        return false;
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    workers.iter().all(|w| {
        // CRITICAL: Don't evict workers that are Loading or Processing!
        let state = w.get_state();
        if matches!(state, WorkerState::Loading | WorkerState::Processing | WorkerState::Spawning) {
            return false; // Not evictable
        }
        
        let pending = w.pending_requests.load(Ordering::Acquire);
        let last_used = w.last_used.load(Ordering::Acquire);
        let idle_duration = now.saturating_sub(last_used);

        pending == 0 && idle_duration >= idle_threshold_secs
    })
}
```

**Add dead worker cleanup** (new function after line 50):

```rust
/// Remove dead and failed workers from pool
fn cleanup_dead_workers<T: ?Sized>(pool: &Pool<T>) {
    use crate::pool::core::worker_state::WorkerState;
    
    for entry in pool.workers().iter() {
        let registry_key = entry.key();
        let mut removed_count = 0;
        
        if let Some(mut workers) = pool.workers().get_mut(registry_key) {
            workers.retain(|worker| {
                let state = worker.get_state();
                
                if matches!(state, WorkerState::Dead | WorkerState::Failed) {
                    log::warn!(
                        "Removing {} worker {} for {}",
                        format!("{:?}", state).to_lowercase(),
                        worker.worker_id,
                        registry_key
                    );
                    
                    // Clean up memory
                    pool.remove_memory(worker.per_worker_mb);
                    
                    removed_count += 1;
                    false // Remove from vector
                } else {
                    true // Keep in vector
                }
            });
        }
        
        if removed_count > 0 {
            pool.metrics().workers_evicted.fetch_add(removed_count, Ordering::Release);
        }
    }
}
```

**Call cleanup in maintenance loop** (around line 241):

```rust
// In process_pool_maintenance function
fn process_pool_maintenance<T: ?Sized>(
    pool: &'static Pool<T>,
    idle_threshold_secs: u64,
    pool_name: &str,
) {
    // FIRST: Clean up dead/failed workers
    cleanup_dead_workers(pool);
    
    // THEN: Check for idle eviction (existing code)
    // ... rest of function ...
}
```

### Step 7: Update Pool Validation Logic

**File**: `packages/candle/src/pool/core/pool.rs`

**Modify `validate_workers`** to use state (around line 113):

```rust
pub fn validate_workers(&self, registry_key: &str) -> usize {
    use crate::pool::core::worker_state::WorkerState;
    
    let mut removed_count = 0;
    
    if let Some(mut workers_guard) = self.workers.get_mut(registry_key) {
        workers_guard.retain(|worker| {
            let state = worker.get_state();
            
            // Remove dead/failed workers immediately
            if matches!(state, WorkerState::Dead | WorkerState::Failed) {
                log::warn!(
                    "Removing {} worker {} for {}",
                    format!("{:?}", state).to_lowercase(),
                    worker.worker_id,
                    registry_key
                );
                
                self.remove_memory(worker.per_worker_mb);
                let _ = worker.shutdown_tx.send(());
                removed_count += 1;
                
                false // Remove
            }
            // Also check health for workers that should be alive
            else if matches!(state, WorkerState::Ready | WorkerState::Processing | WorkerState::Idle) {
                // Only do health check for workers claiming to be active
                if !worker.is_alive() {
                    log::warn!(
                        "Removing unresponsive worker {} for {} (state: {:?})",
                        worker.worker_id,
                        registry_key,
                        state
                    );
                    
                    worker.set_state(WorkerState::Dead);
                    self.remove_memory(worker.per_worker_mb);
                    let _ = worker.shutdown_tx.send(());
                    removed_count += 1;
                    
                    false // Remove
                } else {
                    true // Keep
                }
            }
            else {
                // Keep workers in Spawning/Loading states
                true
            }
        });
        
        if removed_count > 0 {
            log::warn!("Removed {} workers for {}", removed_count, registry_key);
            self.metrics.workers_evicted.fetch_add(removed_count, Ordering::Release);
        }
    }
    
    removed_count
}
```

### Step 8: Export WorkerState from pool module

**File**: `packages/candle/src/pool/core/mod.rs`

Add export (verify it's not already there):

```rust
pub use worker_state::{WorkerState, UnifiedWorkerHandle, CircuitBreaker, HealthCheck, HealthStatus};
```

**File**: `packages/candle/src/pool/mod.rs`

Re-export for convenience:

```rust
pub use core::WorkerState;
```

---

## Code References

### Existing Patterns to Follow

1. **State Transitions**: [`pool/core/orchestrator.rs:252-276`](../packages/candle/src/pool/core/orchestrator.rs#L252-L276)
   - Shows Loading → Ready transition on success
   - Shows Loading → Failed transition on error
   - Shows memory cleanup on failure

2. **State Machine**: [`pool/core/worker_state.rs:12-23`](../packages/candle/src/pool/core/worker_state.rs#L12-L23)
   - Complete WorkerState enum definition
   - u32 representation for atomic storage

3. **Valid Transitions**: [`pool/core/worker_state.rs:192-216`](../packages/candle/src/pool/core/worker_state.rs#L192-L216)
   - State transition validation logic
   - Can adapt for simpler use case (optional enforcement)

4. **Current Worker Loop**: [`pool/capabilities/text_embedding.rs:69-119`](../packages/candle/src/pool/capabilities/text_embedding.rs#L69-L119)
   - Shows crossbeam::select! pattern
   - Shows where to add state transitions

### Issues This Fixes

From the code review performed:

- **Issue #11**: Memory leak on spawn failure - Fixed by cleanup in Failed state
- **Issue #1**: Race condition in validate_workers - Improved by state-based filtering
- **Issue #15**: Eviction of Loading workers - Prevented by state check in maintenance

---

## Detailed Remaining Work

### 5. Vision Worker - `pool/capabilities/vision.rs`
**Changes needed:**
- Add `#[derive(Clone)]` to VisionWorkerHandle struct
- Update worker function signature to accept `registry_key: String`
- In spawn method, add state transitions identical to text_embedding:
  ```rust
  // After thread spawn
  worker.core.set_state(WorkerState::Loading);
  // After successful model load
  worker.core.set_state(WorkerState::Ready);
  // On failure
  worker.core.set_state(WorkerState::Failed);
  vision_pool().remove_memory(per_worker_mb_clone);
  // After worker loop exits
  worker.core.set_state(WorkerState::Dead);
  ```
- Update worker loop with processing transitions and idle detection

### 6. Text-to-Image Worker - `pool/capabilities/text_to_image.rs`
**Changes needed:** Same pattern as Vision worker above

### 7. Image Embedding Worker - `pool/capabilities/image_embedding.rs`
**Changes needed:** 
- Complete the spawn and loop transitions (Clone already added)
- Same pattern but handles 3 channels: embed_image, embed_image_url, embed_image_base64

### 8. Maintenance Eviction Logic - `pool/maintenance.rs`
**Line 17-33: Update all_workers_idle()**
```rust
fn all_workers_idle(workers: &[WorkerHandle], idle_threshold_secs: u64) -> bool {
    use crate::pool::core::worker_state::WorkerState;
    
    if workers.is_empty() {
        return false;
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    workers.iter().all(|w| {
        // CRITICAL: Don't evict workers that are Loading or Processing!
        let state = w.get_state();
        if matches!(state, WorkerState::Loading | WorkerState::Processing | WorkerState::Spawning) {
            return false; // Not evictable
        }
        
        let pending = w.pending_requests.load(Ordering::Acquire);
        let last_used = w.last_used.load(Ordering::Acquire);
        let idle_duration = now.saturating_sub(last_used);

        pending == 0 && idle_duration >= idle_threshold_secs
    })
}
```

**After line 50: Add cleanup_dead_workers()**
```rust
/// Remove dead and failed workers from pool
fn cleanup_dead_workers<T: ?Sized>(pool: &Pool<T>) {
    use crate::pool::core::worker_state::WorkerState;
    
    for entry in pool.workers().iter() {
        let registry_key = entry.key();
        let mut removed_count = 0;
        
        if let Some(mut workers) = pool.workers().get_mut(registry_key) {
            workers.retain(|worker| {
                let state = worker.get_state();
                
                if matches!(state, WorkerState::Dead | WorkerState::Failed) {
                    log::warn!(
                        "Removing {} worker {} for {}",
                        format!("{:?}", state).to_lowercase(),
                        worker.worker_id,
                        registry_key
                    );
                    
                    // Clean up memory
                    pool.remove_memory(worker.per_worker_mb);
                    
                    removed_count += 1;
                    false // Remove from vector
                } else {
                    true // Keep in vector
                }
            });
        }
        
        if removed_count > 0 {
            pool.metrics().workers_evicted.fetch_add(removed_count, Ordering::Release);
        }
    }
}
```

**Line 241: Update process_pool_maintenance()** - Add call to cleanup_dead_workers() before idle eviction

### 9. Pool Validation Logic - `pool/core/pool.rs`
**Line 113-165: Replace validate_workers() method:**
- Use worker.get_state() to check state
- Remove Dead/Failed workers immediately
- Only health-check Ready/Processing/Idle workers
- Set unresponsive workers to Dead before removal
- Keep workers in Spawning/Loading states

### 10. Export WorkerState - Module files
**`pool/core/mod.rs`:** Add line:
```rust
pub use worker_state::{WorkerState, UnifiedWorkerHandle, CircuitBreaker, HealthCheck, HealthStatus};
```

**`pool/mod.rs`:** Add line:
```rust
pub use core::WorkerState;
```

## Definition of Done

### Runtime Behavior Verification

1. **Cold Start with Failure**:
   - Force model load error
   - Verify worker transitions: Spawning → Loading → Failed → Dead
   - Verify memory is released (check `total_memory_mb()` returns to baseline)
   - Verify no zombie threads remain

2. **Cold Start with Success**:
   - Spawn worker normally
   - Verify worker transitions: Spawning → Loading → Ready
   - Send request, verify Ready → Processing → Ready
   - Wait 5+ minutes with no activity, verify Ready → Idle

3. **Eviction Safety**:
   - Start worker with slow model load (simulate with sleep)
   - Trigger maintenance eviction during Loading state
   - Verify worker is NOT evicted (state check prevents it)
   - Verify worker completes loading and transitions to Ready

4. **Request Processing**:
   - Worker in Idle state
   - Send request, verify Idle → Processing → Ready
   - Verify request completes successfully

5. **Shutdown Sequence**:
   - Worker in Ready state processing requests
   - Send shutdown signal
   - Verify transition: * → Evicting → Dead
   - Verify memory released

### Observable States

After implementation, these should be observable:

```rust
// Example: Check worker state via debug logging or metrics
let workers = TEXT_EMBEDDING_WORKERS.get("model-key").unwrap();
for worker in workers.iter() {
    println!("Worker {}: {:?}", worker.core.worker_id, worker.core.get_state());
}
```

Expected output during lifecycle:
```
Worker 1: Spawning    // Just created
Worker 1: Loading     // Model loading
Worker 1: Ready       // Ready for requests
Worker 1: Processing  // Handling request
Worker 1: Ready       // Request done
Worker 1: Idle        // No activity for 5min
Worker 1: Evicting    // Shutdown initiated
Worker 1: Dead        // Thread exited
```

### Memory Correctness

Run scenario:
1. Initial: `pool.total_memory_mb() == 0`
2. Spawn worker (8GB model): `== 8192`
3. Force load failure: `== 0` (cleanup happened)
4. Spawn worker successfully: `== 8192`
5. Evict worker: `== 0`

All transitions must maintain correct memory accounting.

---

## Integration Notes

### With POOL_UNIFIED_STORAGE

When unified storage is implemented:
- Replace `TEXT_EMBEDDING_WORKERS.get()` with `pool.registry.get_worker()`
- State transitions remain the same
- Cleanup becomes simpler (single storage location)

### With Current Dual Storage

Until unified storage:
- Must update state in BOTH locations:
  - `pool.workers` (contains WorkerHandle with state)
  - `TEXT_EMBEDDING_WORKERS` (contains WorkerHandle reference)
- They share the same `Arc<AtomicU32>` so updates propagate

### Backward Compatibility

This change is **backward compatible** at the API level:
- `pool.embed_text()` signature unchanged
- `pool.spawn_text_embedding_worker()` signature unchanged
- State is internal implementation detail

---

## Success Criteria

✅ **DONE** - WorkerHandle has `state: Arc<AtomicU32>` field  
🔄 **PARTIAL** - All worker spawn methods set state: Spawning → Loading → Ready/Failed (2/5 done)
🔄 **PARTIAL** - All worker loops transition: Ready → Processing → Ready/Idle (2/5 done)  
🔄 **PARTIAL** - Failed model loads transition to Failed state and clean up memory (2/5 done)
❌ **TODO** - Maintenance doesn't evict Loading/Processing workers  
❌ **TODO** - Dead/Failed workers are cleaned up from pool  
🔄 **PARTIAL** - Shutdown transitions workers through Evicting → Dead (2/5 done)
🔄 **PARTIAL** - Memory accounting correct across all state transitions (2/5 done)  

---

## Files Modified Summary

1. `packages/candle/src/pool/core/types.rs` - Add state field to WorkerHandle
2. `packages/candle/src/pool/core/pool.rs` - Use state in validate_workers
3. `packages/candle/src/pool/capabilities/text_embedding.rs` - Add state transitions
4. `packages/candle/src/pool/capabilities/text_to_text.rs` - Add state transitions
5. `packages/candle/src/pool/capabilities/image_embedding.rs` - Add state transitions
6. `packages/candle/src/pool/capabilities/vision.rs` - Add state transitions
7. `packages/candle/src/pool/capabilities/text_to_image.rs` - Add state transitions
8. `packages/candle/src/pool/maintenance.rs` - Use state in eviction logic
9. `packages/candle/src/pool/core/mod.rs` - Export WorkerState
10. `packages/candle/src/pool/mod.rs` - Re-export WorkerState

**Total**: 10 files modified, ~400 lines of code changes
