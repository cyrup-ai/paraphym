# POOL_CIRCUIT_BREAKER

**Priority**: MEDIUM  
**Component**: pool/core  
**Estimated Effort**: 1-2 hours (most code exists, needs integration)  
**Risk**: Low  
**Dependencies**: POOL_LIFECYCLE_STATE_MACHINE (already implemented)

## Executive Summary

**CRITICAL DISCOVERY**: The circuit breaker is ~90% implemented but NOT INTEGRATED into the actual request routing path.

### What Already Exists ✅

1. **Full CircuitBreaker Implementation**: [worker_state.rs:116-179](../../packages/candle/src/pool/core/worker_state.rs)
   - State tracking (Closed/Open/HalfOpen via atomic u32)
   - `record_success()` and `record_failure()` methods
   - `can_request()` with timeout-based state transitions
   - Configuration with thresholds and timeouts

2. **WorkerOrchestrator Integration**: [orchestrator.rs:24,309-318](../../packages/candle/src/pool/core/orchestrator.rs)
   - `circuit_breakers: Arc<DashMap<String, Arc<CircuitBreaker>>>` field
   - `get_or_create_circuit_breaker()` method
   - Each `UnifiedWorkerHandle` includes circuit_breaker reference

3. **Worker-Level Integration**: [worker_state.rs:67,237](../../packages/candle/src/pool/core/worker_state.rs)
   - `UnifiedWorkerHandle.circuit_breaker` field
   - `can_accept_requests()` method checks circuit state

### What's Missing ❌

**The circuit breaker is NEVER CALLED in actual request routing!**

The old Pool system in `capabilities/` bypasses WorkerOrchestrator entirely:
- [text_to_text.rs:219-325](../../packages/candle/src/pool/capabilities/text_to_text.rs) - No circuit breaker checks
- [text_embedding.rs](../../packages/candle/src/pool/capabilities/text_embedding.rs) - Same issue
- [image_embedding.rs](../../packages/candle/src/pool/capabilities/image_embedding.rs) - Same issue
- Other capabilities follow same pattern

## Problem Statement

Current request flow in `Pool::prompt()`:
```rust
// text_to_text.rs:260-275 - NO CIRCUIT BREAKER CHECKING
let alive_workers: Vec<_> = workers
    .iter()
    .filter(|w| w.core.is_alive())  // ← Only checks health, not circuit state!
    .collect();

let worker = select_worker_power_of_two(&alive_workers, |w| &w.core)?;
worker.prompt_tx.send(request)?;  // ← Sends directly without circuit check
```

**Result**: Single model failures CAN cascade because there's no circuit breaker protection in the actual request path.

## Solution Design

### Option A: Integrate into Existing Pool (RECOMMENDED - Minimal Changes)

Add circuit breaker to each capability's request routing in `pool/capabilities/*.rs`.

#### Step 1: Add Circuit Breaker to Pool Struct

**File**: `packages/candle/src/pool/core/pool.rs`

```rust
use super::worker_state::{CircuitBreaker, CircuitBreakerConfig};

pub struct Pool<T: ?Sized> {
    workers: DashMap<String, Vec<WorkerHandle>>,
    config: PoolConfig,
    total_memory_used: Arc<AtomicUsize>,
    next_worker_id: AtomicUsize,
    metrics: PoolMetrics,
    shutting_down: Arc<AtomicBool>,
    spawning_in_progress: DashMap<String, Arc<AtomicBool>>,
    
    /// Circuit breakers per model - ADD THIS
    circuit_breakers: DashMap<String, Arc<CircuitBreaker>>,  // ← NEW
    
    _phantom: PhantomData<T>,
}

impl<T: ?Sized> Pool<T> {
    pub fn new(config: PoolConfig) -> Self {
        Self {
            workers: DashMap::new(),
            config,
            total_memory_used: Arc::new(AtomicUsize::new(0)),
            next_worker_id: AtomicUsize::new(0),
            metrics: PoolMetrics::default(),
            shutting_down: Arc::new(AtomicBool::new(false)),
            spawning_in_progress: DashMap::new(),
            circuit_breakers: DashMap::new(),  // ← NEW
            _phantom: PhantomData,
        }
    }
    
    /// Get or create circuit breaker for model - ADD THIS METHOD
    pub fn get_circuit_breaker(&self, registry_key: &str) -> Arc<CircuitBreaker> {
        self.circuit_breakers
            .entry(registry_key.to_string())
            .or_insert_with(|| {
                Arc::new(CircuitBreaker::new(CircuitBreakerConfig {
                    failure_threshold: 5,
                    success_threshold: 3,
                    timeout: Duration::from_secs(60),
                    half_open_requests: 3,
                }))
            })
            .clone()
    }
}
```

**Location**: Add field at [pool.rs:~30](../../packages/candle/src/pool/core/pool.rs) after `spawning_in_progress`.  
**Add method**: After existing Pool impl methods around line 100.

#### Step 2: Integrate Circuit Breaker Checks in text_to_text.rs

**File**: `packages/candle/src/pool/capabilities/text_to_text.rs`

**Current code** (lines 260-275):
```rust
let alive_workers: Vec<_> = workers
    .iter()
    .filter(|w| w.core.is_alive())
    .collect();
```

**Replace with**:
```rust
// Get circuit breaker for this model
let pool = text_to_text_pool();
let circuit = pool.get_circuit_breaker(&registry_key);

// Check circuit state BEFORE routing
if !circuit.can_request() {
    ystream::emit!(sender, CandleCompletionChunk::Error(
        format!("Circuit breaker open for {}", registry_key)
    ));
    // Update metrics
    pool.metrics().total_errors.fetch_add(1, Ordering::Relaxed);
    return;
}

// Filter for alive workers
let alive_workers: Vec<_> = workers
    .iter()
    .filter(|w| w.core.is_alive())
    .collect();
```

**Location**: [text_to_text.rs:260](../../packages/candle/src/pool/capabilities/text_to_text.rs)

#### Step 3: Record Success/Failure in Request Handling

**Current error handling** (lines 290-310):
```rust
let worker_stream = match response_rx.recv_timeout(timeout) {
    Ok(Ok(stream)) => stream,
    Ok(Err(e)) => {
        ystream::emit!(sender, CandleCompletionChunk::Error(
            format!("Worker error: {}", e)
        ));
        worker.core.pending_requests.fetch_sub(1, Ordering::Release);
        return;
    }
    Err(e) => {
        ystream::emit!(sender, CandleCompletionChunk::Error(
            format!("Request timeout: {}", e)
        ));
        worker.core.pending_requests.fetch_sub(1, Ordering::Release);
        return;
    }
};
```

**Replace with**:
```rust
let worker_stream = match response_rx.recv_timeout(timeout) {
    Ok(Ok(stream)) => {
        // Record success on circuit breaker
        circuit.record_success();
        stream
    }
    Ok(Err(e)) => {
        // Record failure on circuit breaker
        circuit.record_failure();
        pool.metrics().total_errors.fetch_add(1, Ordering::Relaxed);
        
        ystream::emit!(sender, CandleCompletionChunk::Error(
            format!("Worker error: {}", e)
        ));
        worker.core.pending_requests.fetch_sub(1, Ordering::Release);
        return;
    }
    Err(e) => {
        // Record timeout as failure
        circuit.record_failure();
        pool.metrics().total_timeouts.fetch_add(1, Ordering::Relaxed);
        
        ystream::emit!(sender, CandleCompletionChunk::Error(
            format!("Request timeout: {}", e)
        ));
        worker.core.pending_requests.fetch_sub(1, Ordering::Release);
        return;
    }
};

// Also record success after successful stream completion
// (after the chunk forwarding loop completes)
```

**Location**: [text_to_text.rs:290-310](../../packages/candle/src/pool/capabilities/text_to_text.rs)

#### Step 4: Apply Same Pattern to Other Capabilities

**Files to modify** (same pattern as text_to_text.rs):
1. `packages/candle/src/pool/capabilities/text_embedding.rs`
2. `packages/candle/src/pool/capabilities/image_embedding.rs`
3. `packages/candle/src/pool/capabilities/vision.rs`
4. `packages/candle/src/pool/capabilities/text_to_image.rs`

Each file has a similar request routing method. Apply the 3-step pattern:
1. Get circuit breaker and check `can_request()` before routing
2. Record success when request succeeds
3. Record failure when request fails or times out

#### Step 5: Add Circuit Metrics to PoolError

**File**: `packages/candle/src/pool/core/error.rs`

**Current** (line 7):
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
    SpawnTimeout(String),
}
```

**Add**:
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
    SpawnTimeout(String),
    CircuitOpen(String),  // ← ADD THIS
}
```

**Add Display impl** (line 16):
```rust
Self::CircuitOpen(msg) => write!(f, "Circuit breaker open: {}", msg),
```

**Location**: [error.rs:7,16](../../packages/candle/src/pool/core/error.rs)

#### Step 6: Update PoolMetrics

**File**: `packages/candle/src/pool/core/types.rs`

**Current** (line 36):
```rust
#[derive(Debug, Default)]
pub struct PoolMetrics {
    pub total_requests: AtomicUsize,
    pub total_timeouts: AtomicUsize,
    pub total_errors: AtomicUsize,
    pub workers_spawned: AtomicUsize,
    pub workers_evicted: AtomicUsize,
}
```

**Add circuit metrics**:
```rust
#[derive(Debug, Default)]
pub struct PoolMetrics {
    pub total_requests: AtomicUsize,
    pub total_timeouts: AtomicUsize,
    pub total_errors: AtomicUsize,
    pub workers_spawned: AtomicUsize,
    pub workers_evicted: AtomicUsize,
    pub circuit_rejections: AtomicUsize,  // ← ADD THIS
}
```

**Location**: [types.rs:36](../../packages/candle/src/pool/core/types.rs)

### Option B: Migrate to WorkerOrchestrator (Future Work)

The `WorkerOrchestrator` in `orchestrator.rs` already has circuit breakers fully integrated. Long-term, migrate all capabilities to use WorkerOrchestrator instead of the simpler Pool. This is a larger refactor and should be a separate task.

## Implementation Checklist

- [ ] Add `circuit_breakers` field to Pool struct ([pool.rs:~30](../../packages/candle/src/pool/core/pool.rs))
- [ ] Add `get_circuit_breaker()` method to Pool impl ([pool.rs:~100](../../packages/candle/src/pool/core/pool.rs))
- [ ] Add `CircuitOpen` variant to PoolError ([error.rs:7](../../packages/candle/src/pool/core/error.rs))
- [ ] Add `circuit_rejections` to PoolMetrics ([types.rs:36](../../packages/candle/src/pool/core/types.rs))
- [ ] Integrate circuit breaker in text_to_text.rs ([text_to_text.rs:260,290](../../packages/candle/src/pool/capabilities/text_to_text.rs))
- [ ] Integrate circuit breaker in text_embedding.rs
- [ ] Integrate circuit breaker in image_embedding.rs
- [ ] Integrate circuit breaker in vision.rs
- [ ] Integrate circuit breaker in text_to_image.rs

## Definition of Done

- [ ] Circuit breaker checks prevent requests when circuit is open
- [ ] Successful requests call `circuit.record_success()`
- [ ] Failed requests call `circuit.record_failure()`
- [ ] Circuit automatically opens after 5 consecutive failures
- [ ] Circuit transitions to half-open after 60s timeout
- [ ] Circuit closes after 3 successful requests in half-open state
- [ ] `pool.metrics().circuit_rejections` increments when circuit rejects requests
- [ ] All 5 capability files integrate circuit breaker consistently

## Reference Implementation

The circuit breaker implementation already exists and is complete:

**Core Implementation**: [worker_state.rs:116-179](../../packages/candle/src/pool/core/worker_state.rs)

```rust
pub struct CircuitBreaker {
    state: Arc<AtomicU32>,              // 0=closed, 1=open, 2=half-open
    failure_count: Arc<AtomicU64>,
    success_count: Arc<AtomicU64>,
    last_failure: Arc<AtomicU64>,
    config: CircuitBreakerConfig,
}

impl CircuitBreaker {
    pub fn record_success(&self) { /* ... */ }
    pub fn record_failure(&self) { /* ... */ }
    pub fn can_request(&self) -> bool { /* ... */ }
}
```

**Configuration**: [orchestrator.rs:313-318](../../packages/candle/src/pool/core/orchestrator.rs)

```rust
CircuitBreakerConfig {
    failure_threshold: 5,      // Open after 5 failures
    success_threshold: 3,      // Close after 3 successes in half-open
    timeout: Duration::from_secs(60),  // Try half-open after 60s
    half_open_requests: 3,     // Allow 3 requests in half-open state
}
```

## Success Metrics

- Circuit operations add < 50ns latency (2 atomic loads in hot path)
- Zero false positives (spurious opens) under normal load
- Recovery time matches configured timeout (60s default)
- 99% reduction in cascade failures when model fails

## Related Files

- [worker_state.rs](../../packages/candle/src/pool/core/worker_state.rs) - CircuitBreaker implementation
- [orchestrator.rs](../../packages/candle/src/pool/core/orchestrator.rs) - WorkerOrchestrator with circuit breakers
- [pool.rs](../../packages/candle/src/pool/core/pool.rs) - Pool struct (needs circuit_breakers field)
- [error.rs](../../packages/candle/src/pool/core/error.rs) - PoolError enum (needs CircuitOpen variant)
- [types.rs](../../packages/candle/src/pool/core/types.rs) - PoolMetrics (needs circuit_rejections)
- [text_to_text.rs](../../packages/candle/src/pool/capabilities/text_to_text.rs) - Main integration point
- [text_embedding.rs](../../packages/candle/src/pool/capabilities/text_embedding.rs) - Apply same pattern
- [image_embedding.rs](../../packages/candle/src/pool/capabilities/image_embedding.rs) - Apply same pattern
- [vision.rs](../../packages/candle/src/pool/capabilities/vision.rs) - Apply same pattern
- [text_to_image.rs](../../packages/candle/src/pool/capabilities/text_to_image.rs) - Apply same pattern

## Notes

- CircuitBreaker uses lock-free atomic operations for zero-allocation
- State transitions validated in worker_state.rs with proper atomics ordering
- WorkerOrchestrator already has full integration (future migration path)
- This task bridges the gap between existing infrastructure and active use
