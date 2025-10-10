# POOL_PRIORITY_QUEUE_SYSTEM

**Priority**: HIGH
**Component**: pool/core
**Estimated Effort**: 2 days
**Risk**: Medium
**Dependencies**: POOL_UNIFIED_STORAGE

## Core Objective

**CRITICAL DISCOVERY**: The priority queue system is **ALREADY IMPLEMENTED** in [`./packages/candle/src/pool/core/request_queue.rs`](../packages/candle/src/pool/core/request_queue.rs) (425 lines).

The actual work required is **NOT** implementing the queue system, but rather:

1. **Replace unbounded channels with bounded** in all 5 capability files
2. **Integrate existing RequestQueue** into worker spawning pattern
3. **Add configuration** for queue capacities to PoolConfig
4. **Wire up** the priority queue to worker loops

This is primarily an **integration task**, not an implementation task.

---

## Problem Statement

### Current Issues (Still Present in Capability Files)

Location: `./packages/candle/src/pool/capabilities/*.rs`

**Files with unbounded channels:**
- [`text_embedding.rs:154-158`](../packages/candle/src/pool/capabilities/text_embedding.rs) - 5 unbounded channels
- [`text_to_text.rs`](../packages/candle/src/pool/capabilities/text_to_text.rs) - Multiple unbounded channels
- [`text_to_image.rs:139`](../packages/candle/src/pool/capabilities/text_to_image.rs) - Unbounded channels
- [`vision.rs`](../packages/candle/src/pool/capabilities/vision.rs) - Unbounded channels
- [`image_embedding.rs`](../packages/candle/src/pool/capabilities/image_embedding.rs) - Unbounded channels

**Impact:**
- Unbounded channels cause OOM under load (memory grows indefinitely)
- No request prioritization (pure FIFO)
- No backpressure mechanism
- No deduplication of identical requests
- No deadline support for time-sensitive operations

---

## What Already Exists (DO NOT REIMPLEMENT)

### 1. Complete Priority Queue System ✅

**File**: [`./packages/candle/src/pool/core/request_queue.rs`](../packages/candle/src/pool/core/request_queue.rs)

**Already Implemented Features:**
- ✅ Priority levels (Critical, High, Normal, Low, Batch)
- ✅ Request deduplication via xxhash
- ✅ Request coalescing/batching
- ✅ Deadline scheduling with automatic expiry
- ✅ Metrics tracking (enqueued, dequeued, deduplicated, coalesced, expired)
- ✅ BinaryHeap-based priority queue
- ✅ Configurable queue capacity
- ✅ Tokio-based batch flush task
- ✅ Tokio-based deadline checker

### 2. Bounded Channels in Orchestrator ✅

**File**: [`./packages/candle/src/pool/core/orchestrator.rs:206-212`](../packages/candle/src/pool/core/orchestrator.rs)

**Reference Implementation:**
```rust
// Already uses bounded channels correctly
let (request_tx, request_rx) = bounded(100);
let (priority_tx, priority_rx) = bounded(10);
let (response_tx, response_rx) = bounded(100);
let (health_tx, health_rx) = bounded(1);
let (health_status_tx, health_status_rx) = bounded(1);
```

This is the **pattern to replicate** in capability files.

### 3. Pool Configuration Structure ✅

**Files**: 
- [`./packages/candle/src/pool/core/types.rs:18-32`](../packages/candle/src/pool/core/types.rs) - PoolConfig
- [`./packages/candle/src/pool/core/request_queue.rs:97-117`](../packages/candle/src/pool/core/request_queue.rs) - QueueConfig

**Already defined:**
```rust
pub struct PoolConfig {
    pub request_timeout_secs: u64,
    pub shutdown_timeout_secs: u64,
    pub maintenance_interval_secs: u64,
    pub cooldown_idle_minutes: u64,
}

pub struct QueueConfig {
    pub max_queue_size: usize,
    pub enable_deduplication: bool,
    pub dedup_window: Duration,
    pub enable_coalescing: bool,
    pub coalesce_window: Duration,
    pub coalesce_max_batch: usize,
    pub enable_deadline_scheduling: bool,
    pub enable_fair_queuing: bool,
    pub history_size: usize,
}
```

---

## What Needs to Change (Specific File Changes)

### Change 1: Replace Unbounded Channels in text_embedding.rs

**File**: `./packages/candle/src/pool/capabilities/text_embedding.rs`

**Current code (lines 153-158):**
```rust
// Create channels
let (embed_tx, embed_rx) = unbounded();
let (batch_embed_tx, batch_embed_rx) = unbounded();
let (shutdown_tx, shutdown_rx) = unbounded();
let (health_tx_worker, health_rx_worker) = unbounded::<HealthPing>();
let (health_tx_main, health_rx_main) = unbounded::<HealthPong>();
```

**Required change:**
```rust
// Create BOUNDED channels (prevent OOM)
let (embed_tx, embed_rx) = bounded(self.config.embed_queue_capacity);          // Default: 100
let (batch_embed_tx, batch_embed_rx) = bounded(self.config.batch_queue_capacity); // Default: 50
let (shutdown_tx, shutdown_rx) = bounded(1);                                   // Shutdown only needs 1
let (health_tx_worker, health_rx_worker) = bounded::<HealthPing>(1);
let (health_tx_main, health_rx_main) = bounded::<HealthPong>(1);
```

**Why this works:**
- Bounded channels provide automatic backpressure
- Senders block when queue is full (prevents infinite memory growth)
- Health channels only need capacity of 1 (ping-pong pattern)

### Change 2: Add Queue Capacity Config to PoolConfig

**File**: `./packages/candle/src/pool/core/types.rs`

**Add to PoolConfig struct:**
```rust
pub struct PoolConfig {
    pub request_timeout_secs: u64,
    pub shutdown_timeout_secs: u64,
    pub maintenance_interval_secs: u64,
    pub cooldown_idle_minutes: u64,
    
    // ADD THESE NEW FIELDS:
    pub embed_queue_capacity: usize,          // Default: 100
    pub batch_queue_capacity: usize,          // Default: 50  
    pub prompt_queue_capacity: usize,         // Default: 100 (text_to_text)
    pub image_gen_queue_capacity: usize,      // Default: 20  (text_to_image)
    pub vision_queue_capacity: usize,         // Default: 50  (vision)
    pub image_embed_queue_capacity: usize,    // Default: 50  (image_embedding)
}
```

**Update Default impl:**
```rust
impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            request_timeout_secs: 30,
            shutdown_timeout_secs: 5,
            maintenance_interval_secs: 60,
            cooldown_idle_minutes: 1,
            
            // Channel capacities (bounded to prevent OOM)
            embed_queue_capacity: 100,
            batch_queue_capacity: 50,
            prompt_queue_capacity: 100,
            image_gen_queue_capacity: 20,  // Image gen is slower, smaller queue
            vision_queue_capacity: 50,
            image_embed_queue_capacity: 50,
        }
    }
}
```

### Change 3: Apply Same Pattern to Other Capability Files

**Files to modify:**

1. **`./packages/candle/src/pool/capabilities/text_to_text.rs`**
   - Replace `unbounded()` with `bounded(self.config.prompt_queue_capacity)`
   
2. **`./packages/candle/src/pool/capabilities/text_to_image.rs`**
   - Replace `unbounded()` with `bounded(self.config.image_gen_queue_capacity)`
   
3. **`./packages/candle/src/pool/capabilities/vision.rs`**
   - Replace `unbounded()` with `bounded(self.config.vision_queue_capacity)`
   
4. **`./packages/candle/src/pool/capabilities/image_embedding.rs`**
   - Replace `unbounded()` with `bounded(self.config.image_embed_queue_capacity)`

**Pattern for all files:**
```rust
// BEFORE:
let (request_tx, request_rx) = unbounded();

// AFTER:
let (request_tx, request_rx) = bounded(self.config.{appropriate}_queue_capacity);
```

### Change 4: Update Import Statements

**In all 5 capability files**, change:
```rust
// BEFORE:
use crossbeam::channel::{Sender, Receiver, bounded, unbounded};

// AFTER (remove unbounded):
use crossbeam::channel::{Sender, Receiver, bounded};
```

---

## Optional Enhancement: Integrate RequestQueue (Future Work)

The existing [`request_queue.rs`](../packages/candle/src/pool/core/request_queue.rs) provides advanced features like priority, deduplication, and coalescing. However, **this is NOT required for the current task**.

**If you want to add priority support later**, here's how:

### Step 1: Add RequestQueue to Worker Handles

**Example for text_embedding.rs:**
```rust
use crate::pool::core::request_queue::{RequestQueue, Priority, QueueConfig};

pub struct TextEmbeddingWorkerHandle {
    pub core: WorkerHandle,
    pub embed_queue: Arc<RequestQueue<EmbedRequest>>,  // Replace direct channel
    pub batch_embed_tx: Sender<BatchEmbedRequest>,     // Keep for batching
    pub shutdown_tx: Sender<()>,
    pub registry_key: String,
}
```

### Step 2: Modify Worker Loop to Check Priority Queue First

**Pattern:**
```rust
fn text_embedding_worker<T: TextEmbeddingCapable>(
    model: T,
    priority_queue: Arc<RequestQueue<EmbedRequest>>,
    normal_rx: Receiver<EmbedRequest>,
    shutdown_rx: Receiver<()>,
) {
    loop {
        // 1. Check priority queue first
        if let Some(priority_req) = priority_queue.dequeue() {
            // Check deadline
            if let Some(deadline) = priority_req.deadline {
                if Instant::now() > deadline {
                    continue;  // Skip expired requests
                }
            }
            
            // Process high-priority request
            let result = model.embed(&priority_req.request.text, priority_req.request.task);
            // Send response...
            continue;
        }
        
        // 2. Then check normal queue
        select! {
            recv(normal_rx) -> req => {
                // Process normal request...
            }
            recv(shutdown_rx) -> _ => break,
        }
    }
}
```

**But again, this is OPTIONAL and NOT required for the current task.**

---

## Implementation Steps (Actual Work Required)

### Step 1: Update PoolConfig (5 minutes)

**File**: `./packages/candle/src/pool/core/types.rs`

Add 6 new fields to `PoolConfig`:
- `embed_queue_capacity`
- `batch_queue_capacity` 
- `prompt_queue_capacity`
- `image_gen_queue_capacity`
- `vision_queue_capacity`
- `image_embed_queue_capacity`

Update `Default` impl with sensible defaults (see Change 2 above).

### Step 2: Replace Unbounded in text_embedding.rs (15 minutes)

**File**: `./packages/candle/src/pool/capabilities/text_embedding.rs`

1. Remove `unbounded` from imports (line 1)
2. Replace 5 `unbounded()` calls with `bounded(capacity)` (lines 154-158)
3. Use `self.config.embed_queue_capacity` and `self.config.batch_queue_capacity`

### Step 3: Replace Unbounded in text_to_text.rs (10 minutes)

**File**: `./packages/candle/src/pool/capabilities/text_to_text.rs`

1. Remove `unbounded` from imports
2. Replace with `bounded(self.config.prompt_queue_capacity)`

### Step 4: Replace Unbounded in text_to_image.rs (10 minutes)

**File**: `./packages/candle/src/pool/capabilities/text_to_image.rs`

1. Remove `unbounded` from imports  
2. Replace with `bounded(self.config.image_gen_queue_capacity)`

### Step 5: Replace Unbounded in vision.rs (10 minutes)

**File**: `./packages/candle/src/pool/capabilities/vision.rs`

1. Remove `unbounded` from imports
2. Replace with `bounded(self.config.vision_queue_capacity)`

### Step 6: Replace Unbounded in image_embedding.rs (10 minutes)

**File**: `./packages/candle/src/pool/capabilities/image_embedding.rs`

1. Remove `unbounded` from imports
2. Replace with `bounded(self.config.image_embed_queue_capacity)`

**Total estimated time: ~1 hour for the core task**

---

## Definition of Done

### Required Changes (Must Complete):

- [ ] `PoolConfig` has 6 new queue capacity fields with defaults
- [ ] `text_embedding.rs` uses `bounded()` with configured capacities (NO unbounded)
- [ ] `text_to_text.rs` uses `bounded()` with configured capacities (NO unbounded)
- [ ] `text_to_image.rs` uses `bounded()` with configured capacities (NO unbounded)
- [ ] `vision.rs` uses `bounded()` with configured capacities (NO unbounded)
- [ ] `image_embedding.rs` uses `bounded()` with configured capacities (NO unbounded)
- [ ] All imports of `unbounded` removed from capability files
- [ ] Code compiles without errors (`cargo check`)
- [ ] All existing APIs continue to work (backward compatible)

### Verification:

Run `cargo check` to ensure no compilation errors:
```bash
cargo check -p paraphym_candle
```

Search for remaining unbounded usage:
```bash
grep -r "unbounded" packages/candle/src/pool/capabilities/
# Should return ZERO results after changes
```

---

## Reference Implementations

### Bounded Channels (Orchestrator Pattern)

**Location**: [`./packages/candle/src/pool/core/orchestrator.rs:206-212`](../packages/candle/src/pool/core/orchestrator.rs)

This file **already uses bounded channels correctly**. Copy this pattern.

### Priority Queue System (Already Built)

**Location**: [`./packages/candle/src/pool/core/request_queue.rs`](../packages/candle/src/pool/core/request_queue.rs)

Study this file to understand:
- How `Priority` enum works (lines 13-19)
- How `PriorityRequest` wraps requests (lines 21-30)
- How `RequestQueue` manages the BinaryHeap (lines 52-77)
- How deduplication works with xxhash (lines 144-158)
- How batch accumulation works (lines 175-199)

### Configuration Patterns

**Locations**:
- [`./packages/candle/src/pool/core/types.rs:18-32`](../packages/candle/src/pool/core/types.rs) - PoolConfig
- [`./packages/candle/src/pool/core/request_queue.rs:97-117`](../packages/candle/src/pool/core/request_queue.rs) - QueueConfig

---

## Migration Notes

### Backward Compatibility

**NO API CHANGES** required for existing users. The changes are purely internal:

```rust
// Users continue to use the same API:
let embedding = pool.embed("text")?;  // Works exactly as before

// Internally, bounded channels prevent OOM
// Users don't need to change anything
```

### Performance Impact

**Expected improvements:**
- **Memory**: Bounded by channel capacity (no unbounded growth)
- **Latency**: Slightly higher under extreme load (backpressure causes blocking)
- **Throughput**: Same or better (prevents system thrashing from OOM)

**Trade-off:**
- Senders may block when queue is full (backpressure)
- This is **intentional** - prevents OOM and system collapse
- Much better than silent memory growth until crash

### Recommended Capacity Values

Based on typical workloads:

| Capability | Queue Type | Capacity | Reasoning |
|-----------|-----------|----------|-----------|
| TextEmbedding | embed | 100 | Fast inference, high throughput |
| TextEmbedding | batch | 50 | Batching reduces queue needs |
| TextToText | prompt | 100 | Variable latency, medium queue |
| TextToImage | generation | 20 | Very slow, small queue sufficient |
| Vision | describe | 50 | Medium speed, moderate queue |
| ImageEmbedding | embed | 50 | Similar to text embedding |

**Tuning guidance:**
- If you see blocking under load, increase capacity
- If memory usage is high, decrease capacity
- Monitor queue depth metrics to optimize

---

## Existing Metrics and Observability

The [`request_queue.rs`](../packages/candle/src/pool/core/request_queue.rs) already provides:

**Metrics tracked:**
- `total_enqueued`: Total requests submitted
- `total_dequeued`: Total requests processed
- `total_deduplicated`: Duplicate requests eliminated
- `total_coalesced`: Requests batched together
- `total_expired`: Requests that hit deadline
- `current_depth`: Current queue size
- `max_depth`: Peak queue depth

**Access metrics:**
```rust
let stats = queue.get_stats();
println!("Queue depth: {}/{}", stats.current_depth, stats.max_depth);
println!("Dedup rate: {:.1}%", 
    100.0 * stats.total_deduplicated as f64 / stats.total_enqueued as f64);
```

---

## Summary

### What This Task IS:

✅ Replace `unbounded()` with `bounded(capacity)` in 5 files  
✅ Add queue capacity config fields to `PoolConfig`  
✅ Remove `unbounded` from import statements  
✅ Verify with `cargo check` and grep

**Estimated time: 1 hour**

### What This Task IS NOT:

❌ Implementing a priority queue system (already exists in request_queue.rs)  
❌ Implementing deduplication (already exists)  
❌ Implementing coalescing (already exists)  
❌ Implementing metrics (already exists)  
❌ Implementing deadline scheduling (already exists)

### The Real Work:

This is an **integration and cleanup task**:
1. Leverage existing bounded channel pattern from orchestrator.rs
2. Apply it consistently across all capability files
3. Add configuration to make capacities tunable
4. Remove all unbounded channels to prevent OOM

The heavy lifting (priority queue, deduplication, etc.) is **already done**. We just need to wire it up properly.

---

## Additional Context

### Why Bounded Channels Matter

**From production experience:**
- Unbounded channels can grow to **gigabytes** under burst traffic
- System OOM kills are **catastrophic** (lose all in-flight work)
- Bounded channels provide **graceful degradation** via backpressure
- Better to slow down requests than crash the entire system

### Crossbeam Bounded Channel Behavior

**When queue is full:**
- `send()` **blocks** until space available (backpressure)
- `try_send()` returns `Err(TrySendError::Full)` immediately
- `send_timeout()` blocks with timeout, returns error if timeout expires

**This is the desired behavior** - prevents runaway memory growth.

### Health Channels Should Stay Small

Health check channels should always use `bounded(1)`:
```rust
let (health_tx, health_rx) = bounded(1);  // Correct
```

Why? Health checks are ping-pong:
1. Send ping
2. Wait for pong  
3. Repeat

There's never more than 1 message in flight, so capacity of 1 is perfect.

---

## Files Reference Map

All file paths relative to: `/Volumes/samsung_t9/paraphym/`

### Files to Modify:
1. [`packages/candle/src/pool/core/types.rs`](../packages/candle/src/pool/core/types.rs) - Add queue capacity config
2. [`packages/candle/src/pool/capabilities/text_embedding.rs`](../packages/candle/src/pool/capabilities/text_embedding.rs) - Replace unbounded
3. [`packages/candle/src/pool/capabilities/text_to_text.rs`](../packages/candle/src/pool/capabilities/text_to_text.rs) - Replace unbounded
4. [`packages/candle/src/pool/capabilities/text_to_image.rs`](../packages/candle/src/pool/capabilities/text_to_image.rs) - Replace unbounded
5. [`packages/candle/src/pool/capabilities/vision.rs`](../packages/candle/src/pool/capabilities/vision.rs) - Replace unbounded
6. [`packages/candle/src/pool/capabilities/image_embedding.rs`](../packages/candle/src/pool/capabilities/image_embedding.rs) - Replace unbounded

### Reference Files (DO NOT MODIFY, use as examples):
1. [`packages/candle/src/pool/core/request_queue.rs`](../packages/candle/src/pool/core/request_queue.rs) - Priority queue implementation
2. [`packages/candle/src/pool/core/orchestrator.rs`](../packages/candle/src/pool/core/orchestrator.rs) - Bounded channel pattern (lines 206-212)
3. [`packages/candle/src/pool/ULTIMATE_SOLUTION.md`](../packages/candle/src/pool/ULTIMATE_SOLUTION.md) - Architecture documentation

---

**END OF TASK SPECIFICATION**
