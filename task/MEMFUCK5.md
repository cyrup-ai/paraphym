# MEMFUCK5: Hardcoded Memory Retrieval Timeout

## Problem
Memory retrieval timeout is hardcoded to 1000ms (1 second), which is too short for production systems with large memory stores or under load. This can cause memory context to be silently dropped when the system is slow.

## Location
- **File**: `/packages/candle/src/domain/agent/chat.rs`
- **Line 397**: `std::time::Duration::from_millis(1000)`

## Current Broken Code
```rust
// Receive results with timeout
let retrieval_results = retrieval_rx.recv_timeout(
    std::time::Duration::from_millis(1000)  // HARDCODED! Too short for production!
).unwrap_or_else(|_| Vec::new());
```

## What Should Happen
Make it configurable with sensible defaults:
```rust
// Add to agent configuration
pub struct CandleAgentRoleImpl {
    // ... existing fields ...
    memory_timeout_ms: Option<u64>,  // New field
}

// In inject_memory_context:
let timeout_ms = self.memory_timeout_ms.unwrap_or(5000);  // Default 5 seconds
let retrieval_results = retrieval_rx.recv_timeout(
    std::time::Duration::from_millis(timeout_ms)
).unwrap_or_else(|_| {
    warn!("Memory retrieval timed out after {}ms", timeout_ms);
    Vec::new()
});
```

## Impact
- Memories silently dropped under load
- Poor user experience when system is busy
- No way to tune for different deployment scenarios
- Production systems with large memory stores will fail
- Silent failures - user doesn't know memories were skipped

## Production Scenarios That Will Break
1. Large memory database (>100k memories)
2. High system load
3. Network latency to database
4. Complex vector searches
5. Concurrent users causing DB contention

## Recommended Values
- Development: 1000ms (current)
- Production default: 5000ms
- High-load production: 10000ms
- Configurable via environment variable

## Fix Priority
**HIGH** - Will cause production failures under load