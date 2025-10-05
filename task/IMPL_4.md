# IMPL_4: Forwarding Statistics Tracking

You must be an expert in [Model Context Protocol](https://modelcontextprotocol.io/docs/getting-started/intro) (MCP) to work on this task.

## OBJECTIVE
Replace placeholder zeros in get_forwarding_stats() with actual atomic counters for monitoring and observability.

## CONTEXT
**File:** `packages/sweetmcp/packages/daemon/src/service/sse/bridge/forwarding.rs`  
**Lines:** 224-227  
**Current State:** Returns hardcoded zeros  
**Severity:** MEDIUM-HIGH - Monitoring/Observability Broken

## REQUIREMENTS
- **NO unit tests** - Testing team handles all test code
- **NO benchmarks** - Benchmarking team handles performance testing
- Focus solely on `./src` modifications

## SUBTASK1: Add Atomic Counter Fields to McpBridge

**What:** Add Arc<AtomicU64> fields for statistics  
**Where:** McpBridge struct definition  
**Why:** Thread-safe counters for concurrent access

Add fields:
```rust
use std::sync::atomic::{AtomicU64, Ordering};

pub struct McpBridge {
    // ... existing fields ...
    
    // Statistics counters
    stats_total_requests: Arc<AtomicU64>,
    stats_successful_requests: Arc<AtomicU64>,
    stats_failed_requests: Arc<AtomicU64>,
}
```

## SUBTASK2: Initialize Counters in Constructor

**What:** Set counters to zero in McpBridge::new()  
**Where:** Constructor implementation  
**Why:** Initialize statistics on creation

Update constructor:
```rust
impl McpBridge {
    pub fn new(base_url: String) -> Result<Self> {
        // ... existing initialization ...
        
        Ok(Self {
            // ... existing fields ...
            stats_total_requests: Arc::new(AtomicU64::new(0)),
            stats_successful_requests: Arc::new(AtomicU64::new(0)),
            stats_failed_requests: Arc::new(AtomicU64::new(0)),
        })
    }
}
```

## SUBTASK3: Track Request Statistics in send_request()

**What:** Increment counters in send_request() method  
**Where:** send_request() implementation  
**Why:** Capture actual request metrics

Modify send_request():
```rust
pub async fn send_request(&self, request: JsonRpcRequest) -> Result<JsonRpcResponse> {
    self.stats_total_requests.fetch_add(1, Ordering::Relaxed);
    
    match self.send_request_internal(request).await {
        Ok(response) => {
            self.stats_successful_requests.fetch_add(1, Ordering::Relaxed);
            Ok(response)
        }
        Err(e) => {
            self.stats_failed_requests.fetch_add(1, Ordering::Relaxed);
            Err(e)
        }
    }
}
```

## SUBTASK4: Rename Current Implementation to send_request_internal()

**What:** Move current send_request() logic to send_request_internal()  
**Where:** Same file  
**Why:** Separate instrumentation from core logic

Rename method:
```rust
async fn send_request_internal(&self, request: JsonRpcRequest) -> Result<JsonRpcResponse> {
    // Current send_request() implementation goes here
}
```

## SUBTASK5: Update get_forwarding_stats() Implementation

**What:** Replace placeholder zeros with actual counter values  
**Where:** Lines 224-227  
**Why:** Return real statistics

Replace:
```rust
pub fn get_forwarding_stats(&self) -> ForwardingStats {
    ForwardingStats {
        total_requests: 0,
        successful_requests: 0,
        // ... more zeros
    }
}
```

With:
```rust
pub fn get_forwarding_stats(&self) -> ForwardingStats {
    ForwardingStats {
        total_requests: self.stats_total_requests.load(Ordering::Relaxed),
        successful_requests: self.stats_successful_requests.load(Ordering::Relaxed),
        failed_requests: self.stats_failed_requests.load(Ordering::Relaxed),
    }
}
```

## SUBTASK6: Update ForwardingStats Struct (if needed)

**What:** Ensure ForwardingStats has failed_requests field  
**Where:** Struct definition (likely in same file or types module)  
**Why:** Track failures separately

Add if missing:
```rust
pub struct ForwardingStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
}
```

## DEFINITION OF DONE
- [ ] Atomic counter fields added to McpBridge
- [ ] Counters initialized in constructor
- [ ] send_request_internal() created with original logic
- [ ] send_request() wraps internal with stats tracking
- [ ] get_forwarding_stats() returns actual values
- [ ] ForwardingStats includes all counter fields
- [ ] Code compiles without errors

## RESEARCH NOTES
### Atomic Operations
- AtomicU64::fetch_add() for increment
- Ordering::Relaxed sufficient for counters
- Arc needed for shared ownership across threads

### Why Relaxed Ordering
- Statistics don't require strict ordering
- Relaxed has best performance
- Eventual consistency acceptable for metrics

## DEPENDENCIES
- std::sync::atomic (standard library)
- std::sync::Arc (standard library)
