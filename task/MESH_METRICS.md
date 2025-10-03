# MESH_METRICS: HTTP Metrics Integration - Context Population Fix

**Status**: 🔴 CRITICAL - Metrics infrastructure exists but context not populated  
**Priority**: HIGH - Breaks production observability  
**Complexity**: LOW - 5 specific code additions needed  

---

## Architecture Overview

### Pingora Request Lifecycle Hooks

The EdgeService implements the ProxyHttp trait with these lifecycle hooks:

```
1. new_ctx()              → Create per-request context
2. request_filter()       → Auth, rate limiting, local API handling
   ↓ (if Ok(false))
3. upstream_peer()        → Select backend
4. request_body_filter()  → Protocol conversion (request)
5. upstream_response_filter() → Capture status code
6. response_body_filter() → Protocol conversion (response)
7. logging()             → Record final metrics
```

**Current Implementation Status:**
- ✅ EdgeContext struct has metrics fields ([src/edge/core/proxy_impl.rs:22-37](../../../packages/sweetmcp/packages/pingora/src/edge/core/proxy_impl.rs))
- ✅ new_ctx() initializes fields with defaults ([proxy_impl.rs:46-58](../../../packages/sweetmcp/packages/pingora/src/edge/core/proxy_impl.rs))
- ✅ response_size tracked in response_body_filter ([proxy_impl.rs:404,418,429,435,440](../../../packages/sweetmcp/packages/pingora/src/edge/core/proxy_impl.rs))
- ✅ status_code tracked in upstream_response_filter ([proxy_impl.rs:499](../../../packages/sweetmcp/packages/pingora/src/edge/core/proxy_impl.rs))
- ✅ logging() calls record_http_request() and decrement_active_requests() ([proxy_impl.rs:463-473](../../../packages/sweetmcp/packages/pingora/src/edge/core/proxy_impl.rs))

### HTTP Metrics Functions Available

From [src/metrics.rs:201-252](../../../packages/sweetmcp/packages/pingora/src/metrics.rs):

```rust
/// Record comprehensive HTTP metrics
pub fn record_http_request(
    method: &str,
    endpoint: &str,
    status_code: u16,
    duration_secs: f64,
    request_size_bytes: usize,
    response_size_bytes: usize,
)

/// Increment active requests (call at request start)
pub fn increment_active_requests(method: &str, endpoint: &str)

/// Decrement active requests (call at request end)
pub fn decrement_active_requests(method: &str, endpoint: &str)
```

---

## Core Problem: Context Never Populated

### Current State in request_filter()

**File**: [src/edge/core/proxy_impl.rs:148-239](../../../packages/sweetmcp/packages/pingora/src/edge/core/proxy_impl.rs)

Lines 151-152 extract local variables:
```rust
let path = req_header.uri.path().to_string(); // Clone path to avoid borrow conflict
let method = req_header.method.clone();
```

**BUT these are NEVER copied to _ctx!**

### Result: Broken Metrics

When logging() executes (line 463):
```rust
crate::metrics::record_http_request(
    &_ctx.method,      // ← Empty string "" from new_ctx()
    &_ctx.endpoint,    // ← Empty string "" from new_ctx()
    _ctx.status_code,  // ← Could be 200 (default) or actual value
    duration_secs,
    _ctx.request_size, // ← Always 0 from new_ctx()
    _ctx.response_size,
);
```

**Impact:**
- All metrics have method="" endpoint="" (unlabeled, unusable for queries)
- HTTP_REQUESTS_ACTIVE gauge goes negative (only decrements, never increments)
- HTTP_REQUESTS_CONCURRENT gauge goes negative
- Error responses (401, 429, 500) have no metrics at all
- Prometheus queries return garbage data

---

## Implementation: 5 Required Fixes

### Fix 1: Populate Context at Request Start

**Location**: [src/edge/core/proxy_impl.rs:148-152](../../../packages/sweetmcp/packages/pingora/src/edge/core/proxy_impl.rs)

**Current Code** (lines 148-152):
```rust
Box::pin(async move {
    // Track active connection (request started)
    self.metrics.active_connections.fetch_add(1, Ordering::Relaxed);

    let req_header = session.req_header();
    let path = req_header.uri.path().to_string();
    let method = req_header.method.clone();
```

**Required Addition** (add after line 152):
```rust
    // Populate context with request metadata for metrics
    _ctx.request_start = std::time::Instant::now();
    _ctx.method = method.as_str().to_string();
    _ctx.endpoint = path.clone();
    
    // Extract request size from Content-Length header
    if let Some(content_length) = req_header.headers.get("content-length") {
        _ctx.request_size = content_length
            .to_str()
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
    }
    
    // Increment active requests metric
    crate::metrics::increment_active_requests(&_ctx.method, &_ctx.endpoint);
```

**Pattern Reference**: Similar to [src/rate_limit/distributed.rs:167](../../../packages/sweetmcp/packages/pingora/src/rate_limit/distributed.rs):
```rust
if !allowed {
    crate::metrics::record_rate_limit_rejection(endpoint);
    warn!("Rate limit exceeded for endpoint {}", endpoint);
}
```

---

### Fix 2: Metrics Before 500 Error

**Location**: [src/edge/core/proxy_impl.rs:170-172](../../../packages/sweetmcp/packages/pingora/src/edge/core/proxy_impl.rs)

**Current Code**:
```rust
Err(e) => {
    warn!("Failed to serialize peers response: {}", e);
    session.respond_error(500).await?;
    return Ok(true);
}
```

**Required Fix** (replace with):
```rust
Err(e) => {
    warn!("Failed to serialize peers response: {}", e);
    
    // Record metrics before returning
    let duration_secs = _ctx.request_start.elapsed().as_secs_f64();
    _ctx.status_code = 500;
    crate::metrics::record_http_request(
        &_ctx.method,
        &_ctx.endpoint,
        500,
        duration_secs,
        _ctx.request_size,
        0, // No response body
    );
    crate::metrics::decrement_active_requests(&_ctx.method, &_ctx.endpoint);
    
    session.respond_error(500).await?;
    return Ok(true);
}
```

---

### Fix 3: Metrics Before First 401 Error

**Location**: [src/edge/core/proxy_impl.rs:196-203](../../../packages/sweetmcp/packages/pingora/src/edge/core/proxy_impl.rs)

**Current Code**:
```rust
Err(e) => {
    // Token verification failed - log and return 401
    warn!("Peer discovery token verification failed from {}: {}", 
        session.client_addr().map(|a| a.to_string()).unwrap_or_else(|| "unknown".to_string()),
        e
    );
    session.respond_error(401).await?;
    return Ok(true);
}
```

**Required Fix** (add before session.respond_error):
```rust
Err(e) => {
    warn!("Peer discovery token verification failed from {}: {}", 
        session.client_addr().map(|a| a.to_string()).unwrap_or_else(|| "unknown".to_string()),
        e
    );
    
    // Record metrics before returning
    let duration_secs = _ctx.request_start.elapsed().as_secs_f64();
    _ctx.status_code = 401;
    crate::metrics::record_http_request(
        &_ctx.method,
        &_ctx.endpoint,
        401,
        duration_secs,
        _ctx.request_size,
        0,
    );
    crate::metrics::decrement_active_requests(&_ctx.method, &_ctx.endpoint);
    
    session.respond_error(401).await?;
    return Ok(true);
}
```

---

### Fix 4: Metrics Before Second 401 Error

**Location**: [src/edge/core/proxy_impl.rs:213-217](../../../packages/sweetmcp/packages/pingora/src/edge/core/proxy_impl.rs)

**Current Code**:
```rust
Ok(_) | Err(_) => {
    // Authentication failed - send 401 and stop processing
    warn!("Authentication required - no valid credentials provided");
    session.respond_error(401).await?;
    return Ok(true);
}
```

**Required Fix**:
```rust
Ok(_) | Err(_) => {
    warn!("Authentication required - no valid credentials provided");
    
    // Record metrics before returning
    let duration_secs = _ctx.request_start.elapsed().as_secs_f64();
    _ctx.status_code = 401;
    crate::metrics::record_http_request(
        &_ctx.method,
        &_ctx.endpoint,
        401,
        duration_secs,
        _ctx.request_size,
        0,
    );
    crate::metrics::decrement_active_requests(&_ctx.method, &_ctx.endpoint);
    
    session.respond_error(401).await?;
    return Ok(true);
}
```

---

### Fix 5: Metrics Before 429 Error

**Location**: [src/edge/core/proxy_impl.rs:229-233](../../../packages/sweetmcp/packages/pingora/src/edge/core/proxy_impl.rs)

**Current Code**:
```rust
// Check rate limit
if !self.rate_limit_manager.check_request(&path, Some(&client_id), 1) {
    warn!("Rate limit exceeded for client: {} on endpoint: {}", client_id, path);
    session.respond_error(429).await?;
    return Ok(true);
}
```

**Required Fix**:
```rust
// Check rate limit
if !self.rate_limit_manager.check_request(&path, Some(&client_id), 1) {
    warn!("Rate limit exceeded for client: {} on endpoint: {}", client_id, path);
    
    // Record metrics before returning
    let duration_secs = _ctx.request_start.elapsed().as_secs_f64();
    _ctx.status_code = 429;
    crate::metrics::record_http_request(
        &_ctx.method,
        &_ctx.endpoint,
        429,
        duration_secs,
        _ctx.request_size,
        0,
    );
    crate::metrics::decrement_active_requests(&_ctx.method, &_ctx.endpoint);
    
    session.respond_error(429).await?;
    return Ok(true);
}
```

---

## Expected Prometheus Output

After fixes, `/metrics` endpoint will expose:

### Request Latency Histogram
```prometheus
sweetmcp_http_request_duration_seconds_bucket{method="POST",endpoint="/v1/messages",status_code="200",le="0.001"} 150
sweetmcp_http_request_duration_seconds_bucket{method="POST",endpoint="/v1/messages",status_code="200",le="0.01"} 450
sweetmcp_http_request_duration_seconds_sum{method="POST",endpoint="/v1/messages",status_code="200"} 5.234
sweetmcp_http_request_duration_seconds_count{method="POST",endpoint="/v1/messages",status_code="200"} 500
```

### Response Status Codes
```prometheus
sweetmcp_http_responses_total{method="POST",endpoint="/v1/messages",status_code="200"} 450
sweetmcp_http_responses_total{method="GET",endpoint="/api/peers",status_code="401"} 12
sweetmcp_http_responses_total{method="POST",endpoint="/v1/messages",status_code="429"} 50
```

### Active Requests (Non-negative!)
```prometheus
sweetmcp_http_requests_active{method="POST",endpoint="/v1/messages"} 3
sweetmcp_http_requests_concurrent_total 8
```

---

## Definition of Done

### Implementation Complete When:

1. ✅ Context population added after line 152 in request_filter()
   - _ctx.method, _ctx.endpoint, _ctx.request_size set from request
   - crate::metrics::increment_active_requests() called

2. ✅ Metrics added before 500 error (line ~171)
   - record_http_request() and decrement_active_requests() called

3. ✅ Metrics added before first 401 error (line ~201)
   - record_http_request() and decrement_active_requests() called

4. ✅ Metrics added before second 401 error (line ~215)
   - record_http_request() and decrement_active_requests() called

5. ✅ Metrics added before 429 error (line ~231)
   - record_http_request() and decrement_active_requests() called

### Verification (No Tests Required):

```bash
# 1. Compilation check
cargo check -p sweetmcp-pingora
# Expected: No warnings about unused increment_active_requests

# 2. Start server
cargo run -p sweetmcp-pingora --release

# 3. Check metrics endpoint
curl http://localhost:9090/metrics | grep sweetmcp_http_requests_active
# Expected: Non-negative gauge values with method/endpoint labels

# 4. Trigger 401 error
curl -X POST http://localhost:8443/v1/messages -H "Authorization: Bearer invalid"
# Expected: Returns 401

# 5. Verify 401 metrics recorded
curl http://localhost:9090/metrics | grep 'sweetmcp_http_responses_total.*401'
# Expected: Counter > 0

# 6. Check gauge values
curl http://localhost:9090/metrics | grep sweetmcp_http_requests_concurrent
# Expected: Value >= 0 (not negative)
```

---

## Files to Modify

**ONLY modify**: [src/edge/core/proxy_impl.rs](../../../packages/sweetmcp/packages/pingora/src/edge/core/proxy_impl.rs)

**Lines requiring changes**:
- Line ~152: Add context population + increment_active_requests()
- Line ~171: Add metrics before 500 error
- Line ~201: Add metrics before first 401 error  
- Line ~215: Add metrics before second 401 error
- Line ~231: Add metrics before 429 error

**Do NOT modify** (already correct):
- EdgeContext struct (lines 22-37)
- new_ctx() method (lines 46-58)
- response_body_filter (lines 404-442)
- upstream_response_filter (line 499)
- logging() method (lines 463-473)

---

## Reference Files

- [src/metrics.rs](../../../packages/sweetmcp/packages/pingora/src/metrics.rs) - Metric function definitions
- [src/rate_limit/distributed.rs](../../../packages/sweetmcp/packages/pingora/src/rate_limit/distributed.rs) - Example metrics pattern
- [src/circuit_breaker.rs](../../../packages/sweetmcp/packages/pingora/src/circuit_breaker.rs) - Another metrics pattern example

---

## Notes

- Metric recording is lock-free and non-blocking
- Failed metric recording does not fail requests
- Label cardinality is bounded (methods ~10, endpoints ~50, status codes ~10)
- Expected overhead: <100μs per request
- Prometheus registry is already configured in [src/main.rs:150](../../../packages/sweetmcp/packages/pingora/src/main.rs)
