# Race Condition: pending_requests Counter Leak on Timeout

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs`

Lines 369-396 (embed_text) and 453-480 (batch_embed_text)

## Severity
**HIGH** - Memory leak and incorrect load balancing

## Issue Description

When a request times out, the `pending_requests` counter is decremented at line 396 (and 480), but the timeout occurs at line 384-393 (and 468-477) which returns early. This creates a race condition where:

1. Counter is incremented (line 369/453)
2. Request is sent (line 373-380/457-464)
3. Timeout occurs and function returns early (line 384-393/468-477)
4. Counter is decremented (line 396/480) - **BUT THIS LINE IS NEVER REACHED ON TIMEOUT**

## Current Code Flow

```rust
// Line 369
worker.core.pending_requests.fetch_add(1, Ordering::Release);

// Lines 384-393 - Early return on timeout
let result = tokio::time::timeout(timeout, response_rx)
    .await
    .map_err(|_| {
        circuit.record_failure();
        self.metrics()
            .total_timeouts
            .fetch_add(1, Ordering::Relaxed);
        PoolError::Timeout("Request timed out".to_string())
    })?  // <-- EARLY RETURN, line 396 never executes
    .map_err(|_| PoolError::RecvError("Response channel closed".to_string()))?;

// Line 396 - NEVER REACHED ON TIMEOUT
worker.core.pending_requests.fetch_sub(1, Ordering::Release);
```

## Impact

1. **Counter Leak**: Each timeout permanently increments the counter by 1
2. **Load Balancing Failure**: Workers appear busier than they are, causing poor load distribution
3. **Worker Starvation**: After enough timeouts, workers may never receive new requests
4. **Memory Pressure**: Incorrect load metrics may prevent proper worker scaling

## Reproduction

1. Send requests that will timeout (e.g., very large batch with short timeout)
2. Observe `pending_requests` counter never decrements
3. Worker appears permanently busy even when idle

## Fix Required

Move the decrement into a guard or ensure it executes on all paths:

```rust
// Option 1: RAII guard
struct PendingGuard<'a> {
    counter: &'a Arc<AtomicUsize>,
}

impl<'a> Drop for PendingGuard<'a> {
    fn drop(&mut self) {
        self.counter.fetch_sub(1, Ordering::Release);
    }
}

// Usage
worker.core.pending_requests.fetch_add(1, Ordering::Release);
let _guard = PendingGuard { counter: &worker.core.pending_requests };

// Option 2: Explicit decrement in error path
let result = tokio::time::timeout(timeout, response_rx)
    .await
    .map_err(|_| {
        worker.core.pending_requests.fetch_sub(1, Ordering::Release); // ADD THIS
        circuit.record_failure();
        self.metrics().total_timeouts.fetch_add(1, Ordering::Relaxed);
        PoolError::Timeout("Request timed out".to_string())
    })?;
```

## Testing

After fix:
1. Send 100 requests that timeout
2. Verify `pending_requests` returns to 0
3. Verify subsequent requests are still routed to the worker
4. Monitor load balancing remains fair across workers
