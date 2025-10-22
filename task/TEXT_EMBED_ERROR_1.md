# Hidden Error: Silent Failure on Response Send

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs`

Lines 134, 147, 165

## Severity
**MEDIUM** - Silent failures in production

## Issue Description

Worker loop silently ignores send failures:

```rust
// Line 134
let _ = req.response.send(result);

// Line 147
let _ = req.response.send(result);

// Line 165
let _ = health_tx.send(pong);
```

The `let _ =` pattern discards errors, hiding critical failures.

## Impact

### Scenario 1: Client Timeout

1. Client sends request
2. Client times out and drops response channel
3. Worker processes request successfully
4. Worker tries to send response
5. **Send fails silently** (channel closed)
6. No logging, no metrics, no visibility

### Scenario 2: Channel Corruption

If the channel is broken due to a bug:
- Worker appears healthy
- Requests are processed
- Responses are lost
- No error indication

## Production Consequences

1. **Silent Data Loss**: Successful work is discarded
2. **No Observability**: Can't detect the issue
3. **Wasted Resources**: CPU/GPU cycles for nothing
4. **Misleading Metrics**: Success rate appears normal

## Fix Required

Log and track send failures:

```rust
// For request responses
if let Err(e) = req.response.send(result) {
    log::warn!(
        "Worker {}: Failed to send response (client likely timed out): {:?}",
        worker_id,
        e
    );
    // Could add metric here
}

// For health checks
if let Err(e) = health_tx.send(pong) {
    log::error!(
        "Worker {}: Health channel broken: {:?}",
        worker_id,
        e
    );
    // This is more serious - health system is broken
    // Consider breaking worker loop
}
```

## Alternative: Metrics

Add counter for send failures:

```rust
static RESPONSE_SEND_FAILURES: AtomicUsize = AtomicUsize::new(0);

if req.response.send(result).is_err() {
    RESPONSE_SEND_FAILURES.fetch_add(1, Ordering::Relaxed);
    log::debug!("Worker {}: Client disconnected before response", worker_id);
}
```

## Health Check Failure

The health check failure (line 165) is particularly concerning:
- If health channel breaks, worker can't report status
- Pool may think worker is dead
- But worker keeps processing requests
- Creates zombie worker state

## Recommendation

1. Log response send failures at DEBUG level (expected on client timeout)
2. Log health send failures at ERROR level (unexpected)
3. Add metrics for tracking frequency
4. Consider breaking worker loop if health channel fails
