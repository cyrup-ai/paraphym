# TASK: Replace Unbounded Channels with Bounded Channels for Memory Safety

## EXECUTIVE SUMMARY

**Critical performance issue**: All worker communication channels use `mpsc::unbounded_channel()` which allows unbounded memory growth. Under heavy load, this can cause OOM crashes.

**Solution**: Replace unbounded channels with bounded channels using capacities already defined in `PoolConfig`, enabling backpressure and preventing memory exhaustion.

---

## LOCATION & SCOPE

### Primary File
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs`

### Related Files
- [../core/types.rs](../packages/candle/src/capability/registry/pool/core/types.rs) - PoolConfig with channel capacities
- [text_embedding.rs](../packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs) - Implementation file

---

## PROBLEM ANALYSIS

### Current Implementation (VULNERABLE)

**Channel Creation** (lines 219-223):
```rust
let (embed_tx, embed_rx) = mpsc::unbounded_channel();
let (batch_embed_tx, batch_embed_rx) = mpsc::unbounded_channel();
let (shutdown_tx, shutdown_rx) = mpsc::unbounded_channel();
let (health_tx_main, health_rx_worker) = mpsc::unbounded_channel();
let (health_tx_worker, health_rx_main) = mpsc::unbounded_channel();
```

**Send Operations** (lines 373, 442):
```rust
worker.embed_tx.send(EmbedRequest { ... }).map_err(...)?;
worker.batch_embed_tx.send(BatchEmbedRequest { ... }).map_err(...)?;
```

**Queue Depth Reporting** (line 172):
```rust
queue_depth: 0, // Note: tokio mpsc doesn't expose len()
```

### The Vulnerability

**Scenario: Slow Worker + Fast Producer**

1. Client sends **10,000 embed requests/sec**
2. Worker processes **100 requests/sec** (slow model, large embeddings)
3. Queue grows by **9,900 requests/sec**
4. After 10 seconds: **99,000 requests queued**
5. Memory usage: `99,000 × (avg_request_size + overhead)`

**Memory Growth:**
- Average text: 1KB per request
- 99,000 requests = **~100MB** in queue
- After 1 minute: **~600MB**
- After 10 minutes: **~6GB**
- Result: **System OOM crash**

**Real-World Triggers:**
- Slow model (large embedding dimension, e.g., 4096-dim)
- Burst traffic (sudden spike in API requests)
- Worker stuck (processing very large batch)
- Worker deadlocked (bug in model code)
- Memory pressure (model in swap, very slow inference)

---

## EXISTING INFRASTRUCTURE (ALREADY WRITTEN)

### PoolConfig Defines Bounded Capacities

From [../core/types.rs](../packages/candle/src/capability/registry/pool/core/types.rs#L30-L35):

```rust
pub struct PoolConfig {
    // ... other fields ...
    
    // Channel capacities (bounded to prevent OOM)
    pub embed_queue_capacity: usize,       // Default: 100
    pub batch_queue_capacity: usize,       // Default: 50
    pub prompt_queue_capacity: usize,      // Default: 100 (text_to_text)
    pub image_gen_queue_capacity: usize,   // Default: 20  (text_to_image)
    pub vision_queue_capacity: usize,      // Default: 50  (vision)
    pub image_embed_queue_capacity: usize, // Default: 50  (image_embedding)
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            // ... other defaults ...
            embed_queue_capacity: 100,
            batch_queue_capacity: 50,
            // ... rest ...
        }
    }
}
```

**KEY INSIGHT**: These capacities are **ALREADY DEFINED** but **NOT BEING USED**!

### Pool Methods to Access Config

From [../core/pool.rs](../packages/candle/src/capability/registry/pool/core/pool.rs):
```rust
impl<W: PoolWorkerHandle> Pool<W> {
    pub fn config(&self) -> &PoolConfig {
        &self.config
    }
}
```

---

## THE FIX: Bounded Channels with Backpressure

### Change 1: Update Channel Type Definitions

**File**: `text_embedding.rs` **Lines**: 63-68

**BEFORE (Unbounded)**:
```rust
pub struct TextEmbeddingWorkerChannels {
    pub embed_rx: mpsc::UnboundedReceiver<EmbedRequest>,
    pub batch_embed_rx: mpsc::UnboundedReceiver<BatchEmbedRequest>,
    pub shutdown_rx: mpsc::UnboundedReceiver<()>,
    pub health_rx: mpsc::UnboundedReceiver<HealthPing>,
    pub health_tx: mpsc::UnboundedSender<HealthPong>,
}
```

**AFTER (Bounded)**:
```rust
pub struct TextEmbeddingWorkerChannels {
    pub embed_rx: mpsc::Receiver<EmbedRequest>,        // CHANGED: bounded
    pub batch_embed_rx: mpsc::Receiver<BatchEmbedRequest>, // CHANGED: bounded
    pub shutdown_rx: mpsc::UnboundedReceiver<()>,       // UNCHANGED: only 1 message
    pub health_rx: mpsc::Receiver<HealthPing>,          // CHANGED: bounded(1)
    pub health_tx: mpsc::Sender<HealthPong>,            // CHANGED: bounded(1)
}
```

**Rationale**:
- **embed_rx, batch_embed_rx**: Bounded to prevent OOM
- **shutdown_rx**: Unbounded OK (only 1 shutdown message ever sent)
- **health_rx/tx**: Bounded to 1 (only need latest ping/pong)

---

### Change 2: Create Bounded Channels

**File**: `text_embedding.rs` **Lines**: 219-223

**BEFORE (Unbounded)**:
```rust
// Create unbounded channels for worker communication
let (embed_tx, embed_rx) = mpsc::unbounded_channel();
let (batch_embed_tx, batch_embed_rx) = mpsc::unbounded_channel();
let (shutdown_tx, shutdown_rx) = mpsc::unbounded_channel();
let (health_tx_main, health_rx_worker) = mpsc::unbounded_channel();
let (health_tx_worker, health_rx_main) = mpsc::unbounded_channel();
```

**AFTER (Bounded)**:
```rust
// Access config for channel capacities
let config = self.config();

// Create bounded channels with configured capacities
let (embed_tx, embed_rx) = mpsc::channel(config.embed_queue_capacity);
let (batch_embed_tx, batch_embed_rx) = mpsc::channel(config.batch_queue_capacity);

// Shutdown stays unbounded (only 1 message ever sent)
let (shutdown_tx, shutdown_rx) = mpsc::unbounded_channel();

// Health channels bounded to 1 (only need latest ping/pong)
let (health_tx_main, health_rx_worker) = mpsc::channel(1);
let (health_tx_worker, health_rx_main) = mpsc::channel(1);
```

---

### Change 3: Update Send Operations to Async

**File**: `text_embedding.rs` **Lines**: 373, 442

**Bounded channels require `.await` on send:**

**BEFORE (Unbounded - Synchronous)**:
```rust
// Line 373 (embed_text method)
worker
    .embed_tx
    .send(EmbedRequest {
        text: Arc::from(text),
        task,
        response: response_tx,
    })
    .map_err(|e| PoolError::SendError(e.to_string()))?;

// Line 442 (batch_embed_text method)
worker
    .batch_embed_tx
    .send(BatchEmbedRequest {
        texts: Arc::from(texts),
        task,
        response: response_tx,
    })
    .map_err(|e| PoolError::SendError(e.to_string()))?;
```

**AFTER (Bounded - Async)**:
```rust
// Line 373 (embed_text method)
worker
    .embed_tx
    .send(EmbedRequest {
        text: Arc::from(text),
        task,
        response: response_tx,
    })
    .await // ADDED: bounded send is async
    .map_err(|e| PoolError::SendError(e.to_string()))?;

// Line 442 (batch_embed_text method)
worker
    .batch_embed_tx
    .send(BatchEmbedRequest {
        texts: Arc::from(texts),
        task,
        response: response_tx,
    })
    .await // ADDED: bounded send is async
    .map_err(|e| PoolError::SendError(e.to_string()))?;
```

**Key Change**: `.send()` → `.send().await`

**Behavior**:
- If queue is full, `.send().await` **waits** (backpressure)
- Prevents unbounded memory growth
- Client naturally slows down when worker is overloaded

---

### Change 4: Fix Queue Depth Reporting

**File**: `text_embedding.rs` **Line**: 172

**BEFORE (Incorrect)**:
```rust
let pong = HealthPong {
    worker_id,
    timestamp: now,
    queue_depth: 0, // Note: tokio mpsc doesn't expose len()
};
```

**AFTER (Correct)**:
```rust
let pong = HealthPong {
    worker_id,
    timestamp: now,
    queue_depth: embed_rx.len() + batch_embed_rx.len(), // Bounded channels expose len()
};
```

**Rationale**:
- The comment `"tokio mpsc doesn't expose len()"` is **INCORRECT**
- **Bounded** `mpsc::Receiver` **DOES** expose `.len()` method
- **Unbounded** `mpsc::UnboundedReceiver` does NOT expose `.len()`
- This fix provides accurate queue depth metrics for monitoring

---

## BACKPRESSURE MECHANICS

### How Bounded Channels Prevent OOM

**With Unbounded Channels (Current - BAD)**:
```
Client → send() → [∞ Queue] → Worker
         instant    grows      slow
                    forever!   
```

**With Bounded Channels (Fixed - GOOD)**:
```
Client → send().await → [100 Queue] → Worker
         waits if full   bounded!    slow

If queue full:
  - send() waits (async)
  - Client slows down
  - No memory growth
```

### Natural Load Balancing

When a worker is overloaded:
1. Queue fills to capacity (100 for embed, 50 for batch)
2. `.send().await` blocks on full queue
3. Caller waits for slot to open
4. Worker processes requests, freeing slots
5. System self-regulates to worker throughput

**Result**: Memory usage bounded to `queue_capacity × avg_request_size`

---

## OPTIONAL ENHANCEMENT: Load Shedding

Instead of waiting when queue is full, can reject immediately with `try_send()`:

```rust
match worker.embed_tx.try_send(req) {
    Ok(_) => {
        // Request accepted
    }
    Err(mpsc::error::TrySendError::Full(_)) => {
        // Queue full - reject immediately
        self.metrics().queue_full_rejections.fetch_add(1, Ordering::Relaxed);
        return Err(PoolError::Overloaded("Worker queue full".to_string()));
    }
    Err(mpsc::error::TrySendError::Closed(_)) => {
        // Worker dead
        return Err(PoolError::SendError("Worker channel closed".to_string()));
    }
}
```

**Trade-off**:
- `send().await`: Better UX (waits instead of failing), natural backpressure
- `try_send()`: Fail-fast, explicit load shedding, better for time-sensitive requests

**Recommendation**: Start with `send().await` (simpler, better UX). Add `try_send()` later if needed.

---

## EXACT FILE MODIFICATIONS

### File: `src/capability/registry/pool/capabilities/text_embedding.rs`

**Modification 1: Update struct definition (lines 63-68)**

REMOVE:
```rust
pub struct TextEmbeddingWorkerChannels {
    pub embed_rx: mpsc::UnboundedReceiver<EmbedRequest>,
    pub batch_embed_rx: mpsc::UnboundedReceiver<BatchEmbedRequest>,
    pub shutdown_rx: mpsc::UnboundedReceiver<()>,
    pub health_rx: mpsc::UnboundedReceiver<HealthPing>,
    pub health_tx: mpsc::UnboundedSender<HealthPong>,
}
```

ADD:
```rust
pub struct TextEmbeddingWorkerChannels {
    pub embed_rx: mpsc::Receiver<EmbedRequest>,
    pub batch_embed_rx: mpsc::Receiver<BatchEmbedRequest>,
    pub shutdown_rx: mpsc::UnboundedReceiver<()>,
    pub health_rx: mpsc::Receiver<HealthPing>,
    pub health_tx: mpsc::Sender<HealthPong>,
}
```

---

**Modification 2: Create bounded channels (lines 219-223)**

REMOVE:
```rust
// Create unbounded channels for worker communication
let (embed_tx, embed_rx) = mpsc::unbounded_channel();
let (batch_embed_tx, batch_embed_rx) = mpsc::unbounded_channel();
let (shutdown_tx, shutdown_rx) = mpsc::unbounded_channel();
let (health_tx_main, health_rx_worker) = mpsc::unbounded_channel();
let (health_tx_worker, health_rx_main) = mpsc::unbounded_channel();
```

ADD:
```rust
// Access config for channel capacities
let config = self.config();

// Create bounded channels with configured capacities
let (embed_tx, embed_rx) = mpsc::channel(config.embed_queue_capacity);
let (batch_embed_tx, batch_embed_rx) = mpsc::channel(config.batch_queue_capacity);

// Shutdown stays unbounded (only 1 message ever sent)
let (shutdown_tx, shutdown_rx) = mpsc::unbounded_channel();

// Health channels bounded to 1 (only need latest ping/pong)
let (health_tx_main, health_rx_worker) = mpsc::channel(1);
let (health_tx_worker, health_rx_main) = mpsc::channel(1);
```

---

**Modification 3: Make send async (line 373)**

CHANGE:
```rust
worker
    .embed_tx
    .send(EmbedRequest {
        text: Arc::from(text),
        task,
        response: response_tx,
    })
    .map_err(|e| PoolError::SendError(e.to_string()))?;
```

TO:
```rust
worker
    .embed_tx
    .send(EmbedRequest {
        text: Arc::from(text),
        task,
        response: response_tx,
    })
    .await
    .map_err(|e| PoolError::SendError(e.to_string()))?;
```

---

**Modification 4: Make send async (line 442)**

CHANGE:
```rust
worker
    .batch_embed_tx
    .send(BatchEmbedRequest {
        texts: Arc::from(texts),
        task,
        response: response_tx,
    })
    .map_err(|e| PoolError::SendError(e.to_string()))?;
```

TO:
```rust
worker
    .batch_embed_tx
    .send(BatchEmbedRequest {
        texts: Arc::from(texts),
        task,
        response: response_tx,
    })
    .await
    .map_err(|e| PoolError::SendError(e.to_string()))?;
```

---

**Modification 5: Fix queue depth reporting (line 172)**

CHANGE:
```rust
let pong = HealthPong {
    worker_id,
    timestamp: now,
    queue_depth: 0, // Note: tokio mpsc doesn't expose len()
};
```

TO:
```rust
let pong = HealthPong {
    worker_id,
    timestamp: now,
    queue_depth: embed_rx.len() + batch_embed_rx.len(),
};
```

---

## IMPACT ANALYSIS

### Memory Usage (Before vs After)

**Scenario**: 10,000 requests/sec, worker at 100 req/sec

| Time | Unbounded Queue | Bounded Queue (100) |
|------|----------------|---------------------|
| 10s | 99,000 reqs (~100MB) | 100 reqs (~100KB) |
| 1min | 594,000 reqs (~600MB) | 100 reqs (~100KB) |
| 10min | 5,940,000 reqs (~6GB) | 100 reqs (~100KB) |

**Memory savings**: **99.98% reduction** under overload!

### Performance Impact

**Latency**:
- Normal load: No change (queue not full)
- Overload: Requests wait in caller (backpressure)
- **Better**: Prevents cascading OOM failure

**Throughput**:
- Bounded by worker capacity (same as before)
- No degradation under normal load
- Graceful degradation under overload (wait instead of crash)

### Monitoring Improvements

With bounded channels and `.len()`:
- ✓ Real queue depth metrics (not hardcoded 0)
- ✓ Track queue utilization per worker
- ✓ Alert on sustained queue fullness
- ✓ Identify slow workers proactively

---

## DEFINITION OF DONE

### Code Changes Complete When:
- ✓ TextEmbeddingWorkerChannels uses `Receiver` types
- ✓ Channel creation uses `mpsc::channel(capacity)`
- ✓ Config accessed to get queue capacities
- ✓ Send operations use `.await`
- ✓ Queue depth reports actual `.len()`

### Verification Complete When:
- ✓ Code compiles without errors
- ✓ No clippy warnings introduced
- ✓ Under load test: memory usage stays bounded
- ✓ Queue depth metrics show accurate values
- ✓ Backpressure behavior works (sender waits when full)

### Behavioral Verification:
- ✓ Normal load: No latency increase
- ✓ Overload: Requests queue up to capacity, then wait
- ✓ Memory: Bounded to `capacity × avg_request_size`
- ✓ No OOM crashes under sustained load

---

## ARCHITECTURE NOTES

### Channel Types in Tokio

**Unbounded**:
```rust
let (tx, rx) = mpsc::unbounded_channel::<T>();
tx.send(msg); // Synchronous, never blocks, infinite queue
```

**Bounded**:
```rust
let (tx, rx) = mpsc::channel::<T>(capacity);
tx.send(msg).await; // Async, blocks if full, bounded queue
rx.len(); // Returns current queue depth (0 to capacity)
```

### Why Some Channels Stay Unbounded

**shutdown_tx**: Only 1 message ever sent (graceful shutdown signal)
- Unbounded OK: No memory risk
- Simpler: No async needed

**Alternatives Considered**:
- Could make health channels unbounded too (low volume)
- But bounded(1) is better: only latest ping/pong matters

---

## RELATED CODE REFERENCES

### Core Pool Files
- [types.rs](../packages/candle/src/capability/registry/pool/core/types.rs#L30-L35) - PoolConfig with queue capacities
- [pool.rs](../packages/candle/src/capability/registry/pool/core/pool.rs) - Pool implementation

### Capability Implementation
- [text_embedding.rs](../packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs) - Current implementation

### Other Capabilities (Same Issue, Not In Scope)
- [vision.rs](../packages/candle/src/capability/registry/pool/capabilities/vision.rs) - Same unbounded pattern
- [text_to_text.rs](../packages/candle/src/capability/registry/pool/capabilities/text_to_text.rs) - Same unbounded pattern
- [text_to_image.rs](../packages/candle/src/capability/registry/pool/capabilities/text_to_image.rs) - Same unbounded pattern
- [image_embedding.rs](../packages/candle/src/capability/registry/pool/capabilities/image_embedding.rs) - Same unbounded pattern

**Note**: This task focuses ONLY on text_embedding.rs. Other capabilities can be fixed in future tasks using the same pattern.

---

## NOTES FOR IMPLEMENTER

### Why This Fix Is Safe

1. **API unchanged**: External callers see no difference
2. **Behavior improved**: Prevents OOM, adds backpressure
3. **Config reused**: Uses existing PoolConfig capacities
4. **Backward compatible**: Same methods, better internals
5. **Metrics improved**: Queue depth now accurate

### Common Pitfalls to Avoid

1. ❌ Don't forget to add `.await` on send operations
2. ❌ Don't change ALL channels (shutdown should stay unbounded)
3. ❌ Don't hardcode capacities (use `config.embed_queue_capacity`)
4. ❌ Don't forget to update struct type definitions
5. ❌ Don't skip the queue depth fix (important for monitoring)

### Quick Verification

After implementation:
```bash
# Compile check
cargo check --lib

# Look for the changes
rg "mpsc::channel\(config" packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs
rg "\.send\(.*\)\.await" packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs
rg "embed_rx\.len\(\)" packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs
```

---

## FUTURE WORK (Out of Scope)

After text_embedding.rs is fixed, apply same pattern to:
- vision.rs
- text_to_text.rs  
- text_to_image.rs
- image_embedding.rs

Each uses the same unbounded channel pattern and has corresponding `*_queue_capacity` fields in PoolConfig.
