# Queue Depth Reporting - RESOLVED ✅

## Status
**COMPLETE** - This issue has been fully resolved in the current codebase.

## Location
[`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs`](../packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs)

**Relevant Lines:**
- Line 162: Queue depth calculation in health check handler
- Lines 220-225: Bounded channel creation with configured capacities

## Original Issue Description

Queue depth was previously hardcoded to 0 with an incorrect comment stating "tokio mpsc doesn't expose len()". This prevented visibility into queue backlog and made it impossible to detect overloaded workers.

## Current Implementation (RESOLVED)

### Bounded Channels Enable Queue Depth Tracking

The implementation uses **bounded channels** created via `tokio::sync::mpsc::channel(capacity)`, which DO expose the `.len()` method for queue depth inspection.

#### Channel Creation (Lines 220-225)

```rust
// Create bounded channels with configured capacities
let (embed_tx, embed_rx) = mpsc::channel(config.embed_queue_capacity);
let (batch_embed_tx, batch_embed_rx) = mpsc::channel(config.batch_queue_capacity);
```

**Configuration Values** (from [`core/types.rs`](../packages/candle/src/capability/registry/pool/core/types.rs)):
- `embed_queue_capacity`: 100 (default)
- `batch_queue_capacity`: 50 (default)

#### Queue Depth Calculation (Line 162)

```rust
Some(_ping) = health_rx.recv() => {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let pong = HealthPong {
        worker_id,
        timestamp: now,
        queue_depth: embed_rx.len() + batch_embed_rx.len(),  // ✅ CORRECTLY IMPLEMENTED
    };

    if let Err(e) = health_tx.send(pong) {
        log::error!(
            "Worker {}: Health channel broken: {:?}",
            worker_id,
            e
        );
    }
}
```

## Technical Details

### Why Bounded Channels Support `.len()`

**Unbounded channels** (`mpsc::unbounded_channel()`):
- No capacity limit
- Cannot report queue depth (no `.len()` method)
- Risk of unbounded memory growth

**Bounded channels** (`mpsc::channel(capacity)`):
- Fixed capacity limit
- Expose `.len()` method returning current queue size
- Provide backpressure when full
- Enable observability and load detection

### Queue Depth Semantics

The reported `queue_depth` represents:
```
queue_depth = embed_rx.len() + batch_embed_rx.len()
```

Where:
- `embed_rx.len()`: Number of pending single-text embedding requests
- `batch_embed_rx.len()`: Number of pending batch embedding requests
- Total: Combined backlog across both operation types

### Health Check Flow

1. **Maintenance worker** sends `HealthPing` via `health_tx` (capacity: 1)
2. **Worker loop** receives ping in `health_rx.recv()` branch
3. **Worker** calculates current queue depth using `.len()` on both channels
4. **Worker** sends `HealthPong` with `queue_depth` back to maintenance worker
5. **Maintenance worker** uses queue depth for load balancing and scaling decisions

## Architecture Context

### Worker Pool Structure

```
Pool<TextEmbeddingWorkerHandle>
├── workers: DashMap<String, Vec<TextEmbeddingWorkerHandle>>
├── config: PoolConfig (defines channel capacities)
└── maintenance worker (polls health, scales workers)

TextEmbeddingWorkerHandle
├── core: WorkerHandle (shared worker metadata)
├── embed_tx: Sender<EmbedRequest>
├── batch_embed_tx: Sender<BatchEmbedRequest>
└── shutdown_tx: Sender<()>

Worker Task (async loop)
├── embed_rx: Receiver<EmbedRequest> (bounded, capacity: 100)
├── batch_embed_rx: Receiver<BatchEmbedRequest> (bounded, capacity: 50)
├── health_rx: Receiver<HealthPing> (bounded, capacity: 1)
└── shutdown_rx: Receiver<()> (unbounded, single message)
```

### Related Files

- [`core/types.rs`](../packages/candle/src/capability/registry/pool/core/types.rs) - `PoolConfig`, `HealthPong`, channel capacity defaults
- [`core/pool.rs`](../packages/candle/src/capability/registry/pool/core/pool.rs) - Generic pool implementation
- [`maintenance.rs`](../packages/candle/src/capability/registry/pool/maintenance.rs) - Health check orchestration
- [`text_embedding.rs`](../packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs) - Worker implementation

## Benefits of Current Implementation

### Observability
- ✅ Real-time queue depth visibility
- ✅ Per-worker load metrics
- ✅ Backlog detection for scaling decisions

### Resource Protection
- ✅ Bounded queues prevent OOM from request floods
- ✅ Backpressure when queues full (`.send().await` blocks)
- ✅ Configurable capacity per operation type

### Load Balancing
- ✅ Power-of-Two worker selection uses queue depth
- ✅ Routes requests to least-loaded workers
- ✅ Avoids hot-spotting on single worker

## Configuration

Queue capacities can be tuned via `PoolConfig`:

```rust
PoolConfig {
    embed_queue_capacity: 100,      // Single-text embedding queue
    batch_queue_capacity: 50,       // Batch embedding queue (larger payloads)
    // ... other config
}
```

**Tuning Guidance:**
- **Increase capacity**: If requests frequently timeout due to full queues
- **Decrease capacity**: If memory pressure is high and backpressure is desired
- **Batch < Embed**: Batch operations are larger, so smaller queue prevents memory spikes

## Definition of Done

✅ **COMPLETE** - Queue depth is correctly reported as the sum of both channel lengths.

The implementation:
- Uses bounded channels with `.len()` support
- Calculates total queue depth in health check handler
- Provides real-time observability into worker load
- Enables intelligent load balancing and scaling

No further changes required.
