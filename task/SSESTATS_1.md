# SSESTATS_1: Implement Real Connection Statistics

## OBJECTIVE
Replace placeholder connection statistics with real metrics tracking using atomic counters.

## LOCATION
`packages/sweetmcp/packages/daemon/src/service/sse/bridge/core.rs:123`

## SUBTASK 1: Create ConnectionStatsTracker struct
```rust
struct ConnectionStatsTracker {
    active: AtomicUsize,
    total_requests: AtomicU64,
    failed_requests: AtomicU64,
    start_time: Instant,
}
```

## SUBTASK 2: Implement get_connection_stats
```rust
pub fn get_connection_stats(&self) -> ConnectionStats {
    ConnectionStats {
        active_connections: self.stats_tracker.active.load(Ordering::Relaxed),
        idle_connections: 0, // Not available from reqwest
        total_requests: self.stats_tracker.total_requests.load(Ordering::Relaxed),
        failed_requests: self.stats_tracker.failed_requests.load(Ordering::Relaxed),
    }
}
```

## SUBTASK 3: Create send_request_tracked wrapper
```rust
async fn send_request_tracked(&self, req: Value) -> Result<Value> {
    self.stats_tracker.total_requests.fetch_add(1, Ordering::Relaxed);
    self.stats_tracker.active.fetch_add(1, Ordering::Relaxed);
    
    let result = self.send_request(req).await;
    
    self.stats_tracker.active.fetch_sub(1, Ordering::Relaxed);
    if result.is_err() {
        self.stats_tracker.failed_requests.fetch_add(1, Ordering::Relaxed);
    }
    
    result
}
```

## SUBTASK 4: Replace placeholder zeros
- Remove comment about reqwest not exposing stats
- Replace all placeholder zero returns
- Update all request paths to use tracked version

## DEFINITION OF DONE
- Real metrics for active connections, total requests, failed requests
- No placeholder zeros remain
- Atomic operations ensure thread safety
- Code compiles without warnings

## RESEARCH NOTES
- std::sync::atomic documentation
- Ordering semantics: Relaxed vs Acquire/Release
- reqwest limitations

## CONSTRAINTS
- Do NOT write unit tests
- Do NOT write integration tests
- Do NOT write benchmarks
- Focus solely on src modification
