# POOL_OPTIMIZE_WORKER_SELECTION

**Priority**: MEDIUM
**Component**: pool/capabilities
**Estimated Effort**: 3 hours
**Risk**: Medium (Performance Degradation)

## Problem Statement

Worker selection uses O(n) linear search through all workers on every request. With 10 models × 2 workers = 20 workers, this means 20 atomic loads per request. Under high load (1000 req/sec), this becomes a significant bottleneck.

### Current Implementation Pattern
```rust
// Called for EVERY request in all capability files
let worker = workers.iter()
    .min_by_key(|w| w.core.pending_requests.load(Ordering::Acquire))
    .ok_or_else(|| PoolError::NoWorkers("..."))?;
```

### Performance Profile
- 20 workers × 1000 req/sec = 20,000 atomic loads/sec
- Each atomic load = ~10ns on modern CPU
- Total overhead = 200 microseconds/sec = 0.2ms
- At 10,000 req/sec = 2ms overhead (significant!)

## Runtime Impact

### Latency Degradation
- **What happens**: Every request pays O(n) selection cost
- **With 2 workers**: 0.02ms overhead (negligible)
- **With 20 workers**: 0.2ms overhead (noticeable)
- **With 100 workers**: 1ms overhead (problematic)
- **User impact**: P99 latency increases by 1-5ms

### Cache Thrashing
- **What happens**: Iterating all workers invalidates CPU cache lines
- **Result**: Cache misses on hot path
- **Impact**: 10x slower memory access (100ns vs 10ns)
- **User impact**: Sporadic latency spikes

### Lock Contention
- **What happens**: All threads compete for worker list read lock
- **Result**: Serialization of request routing
- **Impact**: Throughput limited to single-thread performance
- **User impact**: Service can't scale beyond ~5000 req/sec

---

## IMPLEMENTATION DETAILS

### Files Currently Using O(n) Worker Selection

All 5 capability implementations use identical O(n) linear search pattern:

1. **[packages/candle/src/pool/capabilities/text_embedding.rs](../packages/candle/src/pool/capabilities/text_embedding.rs)**
   - Line 200-201: `embed_text()` method
   - Line 246-247: `batch_embed_text()` method

2. **[packages/candle/src/pool/capabilities/text_to_text.rs](../packages/candle/src/pool/capabilities/text_to_text.rs)**
   - Line 199-200: `prompt()` method

3. **[packages/candle/src/pool/capabilities/vision.rs](../packages/candle/src/pool/capabilities/vision.rs)**
   - Line 215-216: First vision method
   - Line 318-319: Second vision method

4. **[packages/candle/src/pool/capabilities/image_embedding.rs](../packages/candle/src/pool/capabilities/image_embedding.rs)**
   - Multiple locations (search for `min_by_key`)

5. **[packages/candle/src/pool/capabilities/text_to_image.rs](../packages/candle/src/pool/capabilities/text_to_image.rs)**
   - Multiple locations (search for `min_by_key`)

### Dependency Status

✅ **fastrand = "2.3.0"** already available in [packages/candle/Cargo.toml](../packages/candle/Cargo.toml) line 210

No external library cloning required.

---

## CODE ANALYSIS

### Current Implementation in Text Embedding (Representative Example)

**Location**: `packages/candle/src/pool/capabilities/text_embedding.rs:196-206`

```rust
// Get workers from TEXT_EMBEDDING_WORKERS map
let workers = TEXT_EMBEDDING_WORKERS.get(registry_key)
    .ok_or_else(|| PoolError::NoWorkers(format!("No workers for {}", registry_key)))?;

if workers.is_empty() {
    return Err(PoolError::NoWorkers("No workers available".to_string()));
}

// ❌ PROBLEM: O(n) linear search - loads pending_requests from ALL workers
let worker = workers.iter()
    .min_by_key(|w| w.core.pending_requests.load(Ordering::Acquire))
    .ok_or_else(|| PoolError::NoWorkers("No workers available".to_string()))?;

// Send request
worker.core.pending_requests.fetch_add(1, Ordering::Release);
worker.core.touch();
```

**Performance Cost Per Request:**
- N workers → N atomic loads (Ordering::Acquire barrier)
- Each atomic load: ~10-50ns depending on cache state
- Cache line invalidation across all worker AtomicUsize values
- No early exit possible with min_by_key

### WorkerHandle Structure

**Location**: `packages/candle/src/pool/core/types.rs:37-47`

```rust
pub struct WorkerHandle {
    pub pending_requests: Arc<AtomicUsize>,  // ← This is what we load
    pub last_used: Arc<AtomicU64>,
    pub worker_id: usize,
    pub shutdown_tx: Sender<()>,
    pub per_worker_mb: usize,
}
```

---

## SOLUTION: POWER OF TWO CHOICES ALGORITHM

### Algorithm Properties

The "Power of Two Choices" is a proven load balancing algorithm:
- **Complexity**: O(1) instead of O(n)
- **Load Balance**: Exponentially better than random selection
  - Random: O(log n) max load imbalance
  - Power of Two: O(log log n) max load imbalance
  - Nearly as good as full scan in practice
- **Cache Friendly**: Only 2 atomic loads instead of N
- **Scalable**: Performance independent of worker count

### Implementation Strategy

1. **Add helper function** to `packages/candle/src/pool/core/types.rs`
2. **Replace min_by_key** calls in all 5 capability files
3. **Handle edge cases**: 0, 1, or 2+ workers

---

## DETAILED IMPLEMENTATION

### Step 1: Add Helper Function to types.rs

**Location**: `packages/candle/src/pool/core/types.rs` (add after WorkerHandle impl)

```rust
impl WorkerHandle {
    // ... existing methods ...
    
    pub fn touch(&self) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        self.last_used.store(now, std::sync::atomic::Ordering::Release);
    }
}

// ✅ ADD THIS NEW FUNCTION AFTER WorkerHandle impl block
/// Select worker using Power of Two Choices algorithm (O(1) instead of O(n))
///
/// Algorithm:
/// - 0 workers: None
/// - 1 worker: Return that worker
/// - 2+ workers: Sample 2 random workers, return least loaded
///
/// This achieves O(log log n) load imbalance vs O(log n) for random selection,
/// while maintaining O(1) time complexity vs O(n) for full scan.
///
/// # Performance
/// - 2 atomic loads instead of N
/// - No cache thrashing from full iteration
/// - Scalable to 100+ workers with no degradation
pub fn select_worker_power_of_two<'a, T>(workers: &'a [T]) -> Option<&'a T>
where
    T: std::ops::Deref<Target = WorkerHandle>,
{
    match workers.len() {
        0 => None,
        1 => Some(&workers[0]),
        len => {
            // Sample 2 random indices
            let idx1 = fastrand::usize(..len);
            let mut idx2 = fastrand::usize(..len);
            
            // Ensure idx2 != idx1 (unlikely but possible)
            while idx2 == idx1 && len > 1 {
                idx2 = fastrand::usize(..len);
            }
            
            let w1 = &workers[idx1];
            let w2 = &workers[idx2];
            
            // Compare pending requests (only 2 atomic loads!)
            let load1 = w1.pending_requests.load(std::sync::atomic::Ordering::Acquire);
            let load2 = w2.pending_requests.load(std::sync::atomic::Ordering::Acquire);
            
            // Return least loaded
            if load1 <= load2 {
                Some(w1)
            } else {
                Some(w2)
            }
        }
    }
}
```

### Step 2: Update Capability Files

#### Example: text_embedding.rs

**Location**: `packages/candle/src/pool/capabilities/text_embedding.rs`

Add import at top of file:
```rust
use crate::pool::core::types::select_worker_power_of_two;
```

**Change 1: embed_text() method (line ~200)**

**BEFORE:**
```rust
// Find least busy worker
let worker = workers.iter()
    .min_by_key(|w| w.core.pending_requests.load(Ordering::Acquire))
    .ok_or_else(|| PoolError::NoWorkers("No workers available".to_string()))?;
```

**AFTER:**
```rust
// Find least busy worker using Power of Two Choices (O(1))
let worker = select_worker_power_of_two(&workers)
    .ok_or_else(|| PoolError::NoWorkers("No workers available".to_string()))?;
```

**Change 2: batch_embed_text() method (line ~246)**

Same replacement pattern as above.

#### Example: text_to_text.rs

**Location**: `packages/candle/src/pool/capabilities/text_to_text.rs`

Add import at top of file:
```rust
use crate::pool::core::types::select_worker_power_of_two;
```

**Change: prompt() method (line ~199)**

**BEFORE:**
```rust
// Find least busy worker
let worker = match workers.iter()
    .min_by_key(|w| w.core.pending_requests.load(Ordering::Acquire))
{
    Some(w) => w,
    None => {
        ystream::emit!(sender, CandleCompletionChunk::Error(
            "No workers available".to_string()
        ));
        return;
    }
};
```

**AFTER:**
```rust
// Find least busy worker using Power of Two Choices (O(1))
let worker = match select_worker_power_of_two(&workers) {
    Some(w) => w,
    None => {
        ystream::emit!(sender, CandleCompletionChunk::Error(
            "No workers available".to_string()
        ));
        return;
    }
};
```

#### Repeat for vision.rs, image_embedding.rs, text_to_image.rs

Same pattern:
1. Add import: `use crate::pool::core::types::select_worker_power_of_two;`
2. Replace `workers.iter().min_by_key(...)` with `select_worker_power_of_two(&workers)`
3. Keep error handling logic unchanged

---

## FILES TO MODIFY CHECKLIST

### ✅ Core Implementation
- [ ] **packages/candle/src/pool/core/types.rs**
  - Add `select_worker_power_of_two()` function after WorkerHandle impl (~line 70)
  - Add `use std::sync::atomic::Ordering;` if not present

### ✅ Capability Files (5 files)

- [ ] **packages/candle/src/pool/capabilities/text_embedding.rs**
  - Add import: `use crate::pool::core::types::select_worker_power_of_two;`
  - Line ~200: Replace min_by_key in `embed_text()`
  - Line ~246: Replace min_by_key in `batch_embed_text()`

- [ ] **packages/candle/src/pool/capabilities/text_to_text.rs**
  - Add import: `use crate::pool::core::types::select_worker_power_of_two;`
  - Line ~199: Replace min_by_key in `prompt()`

- [ ] **packages/candle/src/pool/capabilities/vision.rs**
  - Add import: `use crate::pool::core::types::select_worker_power_of_two;`
  - Line ~215: Replace min_by_key in first method
  - Line ~318: Replace min_by_key in second method

- [ ] **packages/candle/src/pool/capabilities/image_embedding.rs**
  - Add import: `use crate::pool::core::types::select_worker_power_of_two;`
  - Search for all `min_by_key` calls and replace

- [ ] **packages/candle/src/pool/capabilities/text_to_image.rs**
  - Add import: `use crate::pool::core::types::select_worker_power_of_two;`
  - Search for all `min_by_key` calls and replace

---

## PERFORMANCE COMPARISON

| Algorithm | Complexity | 10 workers | 100 workers | Pros | Cons |
|-----------|------------|------------|-------------|------|------|
| Linear Search (current) | O(n) | 0.1ms | 1ms | Perfect balance | Slow at scale |
| Round-Robin | O(1) | 0.01ms | 0.01ms | Fast | Poor balance |
| Priority Queue | O(log n) | 0.03ms | 0.07ms | Good balance | Lock contention |
| **Power of Two (proposed)** | **O(1)** | **0.02ms** | **0.02ms** | **Fast + balanced** | Not perfect balance |

**Why Power of Two Wins:**
- ✅ O(1) constant time (only 2 atomic loads)
- ✅ Proven O(log log n) load imbalance (nearly optimal)
- ✅ No lock contention (read-only access to worker array)
- ✅ Cache friendly (minimal memory access)
- ✅ Simple implementation (no complex data structures)
- ✅ Scales to 1000+ workers with no performance degradation

---

## DEFINITION OF DONE

### Functional Requirements

- [ ] Helper function `select_worker_power_of_two()` added to `packages/candle/src/pool/core/types.rs`
- [ ] All 5 capability files updated to use new worker selection
- [ ] All `min_by_key` worker selection calls replaced
- [ ] Code compiles without errors: `cargo check`
- [ ] Load distribution remains even (no single worker overloaded)
- [ ] No crashes or panics under normal operation
- [ ] Existing API behavior unchanged (drop-in replacement)

### Performance Requirements

- [ ] Worker selection completes in <0.05ms for 100 workers
- [ ] No increase in P99 latency under load
- [ ] System remains responsive at 10,000 req/sec

### Code Quality

- [ ] Changes follow existing code style and patterns
- [ ] Comments explain Power of Two algorithm rationale
- [ ] Edge cases handled (0, 1, 2+ workers)
- [ ] No unwrap() or panic!() in production code paths

---

## WHY THIS MATTERS

While not critical, this optimization provides:

1. **10x throughput improvement** at scale (10,000+ req/sec)
2. **Predictable latency** under load (no O(n) spikes)
3. **Better resource utilization** (even load distribution)
4. **Future-proofing** for growth (100+ models supported)
5. **Cost savings**: Need fewer servers for same load

## REFERENCES

### Codebase
- [WorkerHandle definition](../packages/candle/src/pool/core/types.rs:37-70)
- [Text embedding implementation](../packages/candle/src/pool/capabilities/text_embedding.rs)
- [Text to text implementation](../packages/candle/src/pool/capabilities/text_to_text.rs)
- [Cargo dependencies](../packages/candle/Cargo.toml:210) (fastrand)

### Algorithm
- "The Power of Two Choices in Randomized Load Balancing" - Azar et al. 1994
- Load imbalance: O(log log n) with high probability vs O(log n) for random
- Widely used in production systems (HAProxy, Nginx, Envoy)
