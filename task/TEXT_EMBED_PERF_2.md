# Performance Issue: Unbounded Channel Memory Growth

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs`

Lines 201-205

## Severity
**HIGH** - Potential OOM (Out of Memory) in production

## Issue Description

All worker channels are unbounded:

```rust
// Lines 201-205
let (embed_tx, embed_rx) = mpsc::unbounded_channel();
let (batch_embed_tx, batch_embed_rx) = mpsc::unbounded_channel();
let (shutdown_tx, shutdown_rx) = mpsc::unbounded_channel();
let (health_tx_main, health_rx_worker) = mpsc::unbounded_channel();
let (health_tx_worker, health_rx_main) = mpsc::unbounded_channel();
```

## Impact

### Scenario: Slow Worker + Fast Producer

1. Client sends 10,000 embed requests/sec
2. Worker processes 100 requests/sec
3. Queue grows by 9,900 requests/sec
4. After 10 seconds: 99,000 requests queued
5. Memory usage: 99,000 * (avg_text_size + overhead)

If average text is 1KB:
- 99,000 requests = ~100MB in queue
- After 1 minute: ~600MB
- After 10 minutes: ~6GB
- **System OOM crash**

### Real-World Trigger

- Model is slow (large embedding model)
- Burst traffic (sudden spike in requests)
- Worker stuck (processing very large batch)
- Worker deadlocked (bug in model code)

## Current State

The `PoolConfig` defines bounded capacities:

```rust
// From types.rs
pub struct PoolConfig {
    pub embed_queue_capacity: usize,       // Default: 100
    pub batch_queue_capacity: usize,       // Default: 50
    // ...
}
```

**BUT THESE ARE NOT USED** in text_embedding.rs!

## Fix Required

Use bounded channels with backpressure:

```rust
let config = self.config();

// Use configured capacities
let (embed_tx, embed_rx) = mpsc::channel(config.embed_queue_capacity);
let (batch_embed_tx, batch_embed_rx) = mpsc::channel(config.batch_queue_capacity);

// Shutdown can stay unbounded (only 1 message)
let (shutdown_tx, shutdown_rx) = mpsc::unbounded_channel();

// Health checks can be bounded (only need latest)
let (health_tx_main, health_rx_worker) = mpsc::channel(1);
let (health_tx_worker, health_rx_main) = mpsc::channel(1);
```

## Backpressure Handling

With bounded channels, `send()` becomes async and can fail:

```rust
// Current (unbounded)
worker.embed_tx.send(req).map_err(|e| PoolError::SendError(e.to_string()))?;

// Fixed (bounded)
worker.embed_tx.send(req).await.map_err(|e| PoolError::SendError(e.to_string()))?;
```

This provides natural backpressure:
- If queue is full, sender waits
- Prevents unbounded memory growth
- Gives visibility into overload (can return error or shed load)

## Alternative: Load Shedding

If queue is full, reject request immediately:

```rust
match worker.embed_tx.try_send(req) {
    Ok(_) => { /* success */ }
    Err(mpsc::error::TrySendError::Full(_)) => {
        return Err(PoolError::Overloaded("Worker queue full".to_string()));
    }
    Err(mpsc::error::TrySendError::Closed(_)) => {
        return Err(PoolError::SendError("Worker dead".to_string()));
    }
}
```

## Monitoring

After fix, add metrics:
- Queue depth per worker (already in HealthPong but always 0)
- Queue full rejections
- Backpressure wait time

## Related Issue

Line 162 in worker loop:
```rust
queue_depth: 0, // Note: tokio mpsc doesn't expose len()
```

This is incorrect - bounded channels DO expose `len()`:
```rust
// For bounded channel
let depth = embed_rx.len();
```

This should be fixed to provide accurate queue depth metrics.
