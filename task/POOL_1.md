# POOL_1: Remove Worker Status Stub Logic

## OBJECTIVE

Replace the "assume alive for now" worker status check with proper worker health detection in the capability pool system.

## BACKGROUND

Worker health checking assumes workers are alive when channel checks fail. This can cause requests to be routed to dead workers, causing timeouts and failures.

## SUBTASK 1: Implement Robust Health Checking

**Location:** `packages/candle/src/capability/registry/pool/core/types.rs:351`

**Current State:**
```rust
Err(_) => {
    // Channel empty or closed - assume alive for now
    // (worker may not have responded yet)
    true
}
```

**Required Changes:**
- Remove "assume alive for now" logic
- Implement proper health probe with timeout
- Send health check request and await response
- Mark worker as unhealthy if no response within timeout
- Use non-blocking health check to avoid delays

**Why:** Routing to dead workers causes user-facing failures.

## SUBTASK 2: Add Worker Health State

**Location:** `packages/candle/src/capability/registry/pool/core/types.rs`

**Required Changes:**
- Add health state field to worker struct (Healthy, Unhealthy, Unknown)
- Track last successful health check timestamp
- Implement health state transitions
- Add health check interval configuration
- Use atomic state updates for thread safety

**Why:** Workers need explicit health state rather than assumptions.

## SUBTASK 3: Implement Health Recovery

**Location:** Same file, pool management

**Required Changes:**
- Periodically retry health checks on unhealthy workers
- Transition back to healthy when worker responds
- Remove workers that stay unhealthy beyond threshold
- Spawn replacement workers when workers are removed
- Log health state transitions for debugging

**Why:** Temporary failures shouldn't permanently remove workers.

## SUBTASK 4: Integrate Health Checks with Pool Selection

**Location:** Pool worker selection logic

**Required Changes:**
- Filter out unhealthy workers during selection
- Fall back to spawning new worker if all unhealthy
- Implement health-aware load balancing
- Add metrics for worker health transitions

**Why:** Pool must only route to healthy workers.

## DEFINITION OF DONE

- [ ] No "assume alive for now" comments or logic
- [ ] Explicit health checking with configurable timeout
- [ ] Workers marked unhealthy when unresponsive
- [ ] Unhealthy workers excluded from request routing
- [ ] Health recovery mechanism for temporary failures
- [ ] Logging for health state transitions
- [ ] NO test code written (separate team responsibility)
- [ ] NO benchmark code written (separate team responsibility)

## RESEARCH NOTES

### Health Check Patterns
- Use dedicated health check channel or message type
- Timeout should be short (100-500ms) to avoid delays
- Exponential backoff for retry attempts
- Remove workers after N consecutive failures

### Integration Points
- Worker spawning in pool management
- Request routing in pool selection
- Existing channel communication infrastructure
- Pool metrics and monitoring

### Thread Safety
- Use atomic operations for health state (existing pattern)
- Non-blocking health checks (don't block request path)
- Consider using `tokio::time::timeout` for health probes

## CONSTRAINTS

- DO NOT write unit tests (separate team handles testing)
- DO NOT write benchmarks (separate team handles benchmarks)
- Health checks must not block request routing
- Maintain backward compatibility with existing pool API
