# SECRTY_2: TCP Health Checks Implementation

## OBJECTIVE

Implement TCP health checks for backend services to prevent routing traffic to failed/unreachable backends. Currently, the system assumes all backends are healthy if they exist in metrics targets, causing request failures when backends are down. This implementation adds active health monitoring with configurable thresholds and automatic backend exclusion.

## SUBTASK 1: Add Health Status Data Structures

**Location:** `packages/sweetmcp/packages/pingora/src/edge/core/operations.rs`

**What needs to change:**
- Add `health_status` field to `EdgeServiceData` struct
- Add `health_check_config` field to `EdgeServiceData` struct
- Create `HealthCheckConfig` struct with health check parameters

**Why:**
- Track health status of each backend independently
- Configure health check behavior (intervals, thresholds, timeouts)
- Enable atomic updates from background health check task

**Required Code:**
```rust
use dashmap::DashMap;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct EdgeServiceData {
    // ... existing fields ...
    
    /// Backend health status: backend_addr -> is_healthy
    health_status: Arc<DashMap<String, Arc<AtomicBool>>>,
    
    /// Health check configuration
    health_check_config: HealthCheckConfig,
}

#[derive(Clone)]
pub struct HealthCheckConfig {
    /// Interval between health checks (milliseconds)
    pub interval_ms: u64,
    /// TCP connection timeout (milliseconds)
    pub timeout_ms: u64,
    /// Consecutive failures before marking unhealthy
    pub failure_threshold: u32,
    /// Consecutive successes before marking healthy
    pub success_threshold: u32,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            interval_ms: 5000,      // 5 seconds
            timeout_ms: 3000,       // 3 second timeout
            failure_threshold: 3,   // 3 failures -> unhealthy
            success_threshold: 2,   // 2 successes -> healthy
        }
    }
}
```

## SUBTASK 2: Implement TCP Health Check Function

**Location:** `packages/sweetmcp/packages/pingora/src/edge/core/operations.rs`

**What needs to change:**
- Create `perform_tcp_health_check()` async function
- Implement TCP connection attempt with timeout
- Return boolean health status
- Add debug logging for failures/timeouts

**Why:**
- Core health check logic to verify backend reachability
- Timeout prevents hanging on unresponsive backends
- Logging aids troubleshooting

**Required Code:**
```rust
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use std::net::SocketAddr;

async fn perform_tcp_health_check(addr: &SocketAddr, timeout_ms: u64) -> bool {
    match timeout(
        Duration::from_millis(timeout_ms),
        TcpStream::connect(addr)
    ).await {
        Ok(Ok(_stream)) => {
            debug!("Health check succeeded for {}", addr);
            true
        }
        Ok(Err(e)) => {
            debug!("Health check failed for {}: {}", addr, e);
            false
        }
        Err(_) => {
            debug!("Health check timeout for {}", addr);
            false
        }
    }
}
```

## SUBTASK 3: Implement Health Check Background Task

**Location:** `packages/sweetmcp/packages/pingora/src/edge/core/operations.rs`

**What needs to change:**
- Create `run_health_checks()` async function
- Implement interval-based health checking loop
- Track consecutive failures/successes per backend
- Update health status atomically based on thresholds
- Add logging for health transitions (healthy ↔ unhealthy)

**Why:**
- Continuous monitoring of backend health
- Threshold-based state transitions prevent flapping
- Atomic updates ensure thread-safe health status

**Required Code:**
```rust
use tokio::time::interval;
use std::collections::HashMap;

async fn run_health_checks(
    backends: Vec<(String, SocketAddr)>,
    health_status: Arc<DashMap<String, Arc<AtomicBool>>>,
    config: HealthCheckConfig,
) {
    let mut check_interval = interval(Duration::from_millis(config.interval_ms));
    let mut failure_counts: HashMap<String, u32> = HashMap::new();
    let mut success_counts: HashMap<String, u32> = HashMap::new();
    
    loop {
        check_interval.tick().await;
        
        for (backend_id, addr) in &backends {
            let is_healthy = perform_tcp_health_check(addr, config.timeout_ms).await;
            
            if is_healthy {
                success_counts.entry(backend_id.clone())
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                failure_counts.insert(backend_id.clone(), 0);
                
                if success_counts[backend_id] >= config.success_threshold {
                    if let Some(status) = health_status.get(backend_id) {
                        let was_unhealthy = !status.load(Ordering::Relaxed);
                        status.store(true, Ordering::Relaxed);
                        if was_unhealthy {
                            info!("Backend {} marked healthy", backend_id);
                        }
                    }
                }
            } else {
                failure_counts.entry(backend_id.clone())
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                success_counts.insert(backend_id.clone(), 0);
                
                if failure_counts[backend_id] >= config.failure_threshold {
                    if let Some(status) = health_status.get(backend_id) {
                        let was_healthy = status.load(Ordering::Relaxed);
                        status.store(false, Ordering::Relaxed);
                        if was_healthy {
                            warn!("Backend {} marked unhealthy after {} failures", 
                                  backend_id, failure_counts[backend_id]);
                        }
                    }
                }
            }
        }
    }
}
```

## SUBTASK 4: Initialize Health Checks on Service Start

**Location:** `packages/sweetmcp/packages/pingora/src/edge/core/operations.rs` (EdgeServiceData::new())

**What needs to change:**
- Initialize `health_status` DashMap with all backends set to healthy
- Create default `HealthCheckConfig`
- Extract backend list from `backend_metrics_targets`
- Spawn background health check task with tokio::spawn

**Why:**
- Start with optimistic assumption (backends healthy)
- Background task runs independently of request handling
- Ensures health checks begin immediately on service start

**Required Code:**
```rust
impl EdgeServiceData {
    pub fn new(/* existing parameters */) -> Self {
        let health_status = Arc::new(DashMap::new());
        
        // Initialize all backends as healthy
        for (backend_id, _) in &backend_metrics_targets {
            health_status.insert(
                backend_id.clone(), 
                Arc::new(AtomicBool::new(true))
            );
        }
        
        // Extract backends for health check task
        let backends: Vec<_> = backend_metrics_targets.iter()
            .map(|(id, addr)| (id.clone(), *addr))
            .collect();
        
        let health_status_clone = health_status.clone();
        let config = HealthCheckConfig::default();
        
        // Spawn health check background task
        tokio::spawn(async move {
            run_health_checks(backends, health_status_clone, config.clone()).await;
        });
        
        Self {
            // ... existing fields ...
            health_status,
            health_check_config: config,
        }
    }
}
```

## SUBTASK 5: Update get_healthy_backend_count()

**Location:** `packages/sweetmcp/packages/pingora/src/edge/core/operations.rs` (line 65-67)

**What needs to change:**
- Replace TODO comment and `backend_count` return
- Filter `health_status` map to count only healthy backends
- Return count of backends with `true` health status

**Why:**
- Remove assumption that all backends are healthy
- Provide accurate count for load balancing decisions
- Prevent routing to failed backends

**Current Code:**
```rust
// For now, assume all backends are healthy if we have metrics targets
// TODO: Implement actual TCP health checks
backend_count
```

**Required Code:**
```rust
// Count only healthy backends
let healthy_count = self.health_status
    .iter()
    .filter(|entry| entry.value().load(Ordering::Relaxed))
    .count();

healthy_count
```

## DEFINITION OF DONE

- [ ] Health status data structures added to EdgeServiceData
- [ ] HealthCheckConfig struct implemented with Default trait
- [ ] perform_tcp_health_check() function implemented
- [ ] run_health_checks() background task implemented
- [ ] Health checks initialized in EdgeServiceData::new()
- [ ] get_healthy_backend_count() updated to use actual health status
- [ ] TODO comment removed from get_healthy_backend_count()
- [ ] Code compiles without errors
- [ ] Appropriate debug/info/warn logging added
- [ ] **NO TESTS WRITTEN** (separate team responsibility)
- [ ] **NO BENCHMARKS WRITTEN** (separate team responsibility)

## RESEARCH NOTES

### Health Check Best Practices
- **Threshold-based state transitions:** Prevent flapping from transient failures
- **TCP connection checks:** Simplest health check that verifies network reachability
- **Fail-healthy on startup:** Assume backends healthy until proven otherwise
- **Atomic status updates:** Use AtomicBool for lock-free concurrent access

### Tokio Patterns
- `interval()`: Periodic task execution without drift accumulation
- `timeout()`: Wrap async operations with time limit
- `spawn()`: Launch background task that runs independently

### Architecture Integration Points
- `backend_metrics_targets`: Source of backend addresses in EdgeServiceData
- `DashMap`: Concurrent hashmap for thread-safe health status storage
- Health status read by get_healthy_backend_count() for routing decisions

## IMPORTANT CONSTRAINTS

- **NO UNIT TESTS**: Test team handles all test code
- **NO INTEGRATION TESTS**: Test team handles all test code  
- **NO BENCHMARKS**: Benchmark team handles performance testing
- Focus solely on ./src modifications and functionality
- Metrics implementation is optional/future work (focus on core health checking)
