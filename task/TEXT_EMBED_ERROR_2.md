# Hidden Error: Inconsistent Error Handling in Timeout Path

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs`

Lines 384-407 (embed_text) and 468-491 (batch_embed_text)

## Severity
**MEDIUM** - Inconsistent error handling

## Issue Description

The timeout error path records metrics but the channel error path does not:

```rust
// Lines 384-393 - Timeout path
let result = tokio::time::timeout(timeout, response_rx)
    .await
    .map_err(|_| {
        // Records failure
        circuit.record_failure();
        self.metrics()
            .total_timeouts
            .fetch_add(1, Ordering::Relaxed);
        PoolError::Timeout("Request timed out".to_string())
    })?
    // Lines 394 - Channel error path
    .map_err(|_| PoolError::RecvError("Response channel closed".to_string()))?;
    // ^^^ NO circuit.record_failure() or metrics update!

// Lines 398-405 - Success/failure path
match &result {
    Ok(_) => circuit.record_success(),
    Err(_) => {
        circuit.record_failure();
        self.metrics().total_errors.fetch_add(1, Ordering::Relaxed);
    }
}
```

## Impact

### Channel Closure Scenario

When response channel closes (worker died):
1. Timeout succeeds (worker is alive)
2. Channel recv fails
3. **No circuit breaker update**
4. **No error metrics**
5. Returns `PoolError::RecvError`

But the outer match (lines 398-405) is never reached because of the `?` operator.

## Inconsistency

- Timeout: Updates circuit breaker + metrics
- Channel error: No updates
- Result error: Updates circuit breaker + metrics (but unreachable)

## Fix Required

Update metrics in channel error path:

```rust
let result = tokio::time::timeout(timeout, response_rx)
    .await
    .map_err(|_| {
        circuit.record_failure();
        self.metrics()
            .total_timeouts
            .fetch_add(1, Ordering::Relaxed);
        PoolError::Timeout("Request timed out".to_string())
    })?
    .map_err(|_| {
        // ADD THESE:
        circuit.record_failure();
        self.metrics().total_errors.fetch_add(1, Ordering::Relaxed);
        PoolError::RecvError("Response channel closed".to_string())
    })?;

// This block becomes redundant for error cases
match &result {
    Ok(_) => circuit.record_success(),
    Err(_) => {
        // This is now unreachable due to ? operators above
        // Can be removed or kept for defensive programming
        circuit.record_failure();
        self.metrics().total_errors.fetch_add(1, Ordering::Relaxed);
    }
}
```

## Alternative: Unified Error Handling

```rust
let result = match tokio::time::timeout(timeout, response_rx).await {
    Err(_) => {
        circuit.record_failure();
        self.metrics().total_timeouts.fetch_add(1, Ordering::Relaxed);
        Err(PoolError::Timeout("Request timed out".to_string()))
    }
    Ok(Err(_)) => {
        circuit.record_failure();
        self.metrics().total_errors.fetch_add(1, Ordering::Relaxed);
        Err(PoolError::RecvError("Response channel closed".to_string()))
    }
    Ok(Ok(res)) => res,
};

match &result {
    Ok(_) => circuit.record_success(),
    Err(_) => {
        // Already recorded above, but defensive
    }
}
```

## Recommendation

Use the alternative unified approach for clarity and consistency.
