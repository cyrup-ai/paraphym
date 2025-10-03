# Graceful Shutdown: Comprehensive System Disconnection

## Status
**DISCONNECTED** - Full graceful shutdown system exists but core flow bypassed

## Problem
Complete graceful shutdown infrastructure implemented (signal handling, connection draining, mDNS goodbye, peer notification, state preservation) but never activated. Coordinator instantiated but critical setup methods never called.

## Disconnected Components (18 items)

### 1. Signal Handler Setup (Never Called)
**File**: `shutdown.rs:183-206`
- `listen_for_shutdown()` never called
  - SIGTERM handler never registered
  - SIGINT handler never registered
  - Graceful shutdown never triggered by signals

### 2. State Preservation (Never Used)
**File**: `shutdown.rs`
- `update_state()` never called (line 132)
- `load_state()` never called (line 144)
- `save_state()` never called (line 171)
- State file never written/read
- Fast recovery after restart impossible

**ServerState fields never populated**:
- `peers: Vec<String>` - never saved
- `shutdown_at: Option<u64>` - never recorded
- `circuit_breakers: Vec<CircuitBreakerState>` - never preserved

### 3. Discovery Deregistration (Disconnected)
**File**: `shutdown.rs:239-279`
- `set_peer_registry()` never called (line 98)
- `set_local_port()` never called (line 94)
- Without these, deregistration cannot work:
  - `send_mdns_goodbye_packets()` will use default port 8443
  - `notify_peers_of_shutdown()` returns early (no registry)

### 4. mDNS Goodbye Packets (Cannot Execute)
**File**: `shutdown.rs:281-322`
- `send_mdns_goodbye_packets()` exists but port not set
- RFC 6762 compliant goodbye (TTL=0) never sent
- Peers don't know service is shutting down

### 5. Peer Shutdown Notification (Cannot Execute)
**File**: `shutdown.rs:324-379`
- `notify_peers_of_shutdown()` cannot run (registry not set)
- UDP shutdown messages never sent
- Peers continue routing to dead service

### 6. Shuttle Deregistration (Incomplete)
**File**: `shutdown.rs:394-427`
- `deregister_from_shuttle()` only removes local state file
- No actual Shuttle API deregistration
- Service registry not updated

### 7. Request Tracking (Partially Connected)
**File**: `shutdown.rs:113-121, 429-450`
- `request_start()` exists and returns `RequestGuard`
- BUT: Never called from request handler
- Active request count always 0
- Connection draining ineffective

### 8. ShutdownAware Wrapper (Never Used)
**File**: `shutdown.rs:452-478`
- `ShutdownAware<S>` wrapper exists
- Never constructed
- `track_request()` method never called
- Service wrapping pattern unused

## Current vs Intended Flow

### Current (Partial Shutdown)
```rust
// EdgeService::new() - line 125
let shutdown_coordinator = Arc::new(ShutdownCoordinator::new(
    std::env::temp_dir().join("sweetmcp")
));

// Later in EdgeService::shutdown()
self.shutdown_coordinator.initiate_shutdown().await;

// But signal handlers NEVER REGISTERED
// State preservation NEVER USED
// Discovery deregistration CANNOT RUN (registry not set)
```

### Intended (Full Graceful Shutdown)
```rust
// 1. Setup coordinator fully
let mut shutdown_coordinator = ShutdownCoordinator::new(data_dir);
shutdown_coordinator.set_local_port(local_port);
shutdown_coordinator.set_peer_registry(peer_registry.clone());

// 2. Register signal handlers
let coordinator_clone = Arc::new(shutdown_coordinator);
coordinator_clone.clone().listen_for_shutdown().await;

// 3. Track requests
impl ProxyHttp for EdgeService {
    async fn request_filter(&self, session: &mut Session) {
        let _guard = self.shutdown_coordinator.request_start();
        // ... request handling
    }
}

// 4. Preserve state during operation
self.shutdown_coordinator.update_state(|state| {
    state.peers = peer_registry.get_all_peers();
    state.circuit_breakers = circuit_breaker_manager.export_states();
}).await;

// 5. Restore state on startup
if let Some(state) = shutdown_coordinator.load_state().await? {
    for peer in state.peers {
        peer_registry.add_peer(peer.parse()?);
    }
}

// 6. On SIGTERM:
// - Stop accepting requests
// - Send mDNS goodbye packets (with correct port)
// - Notify all peers via UDP
// - Drain active connections
// - Save state to disk
// - Exit cleanly
```

## Reconnection Steps

### 1. Setup Coordinator Fully in Main
**File**: `main.rs` (after line 95 where coordinator is created)
```rust
// Currently coordinator is created in EdgeService::new()
// Move to main and configure properly:

let mut shutdown_coordinator = ShutdownCoordinator::new(
    get_data_dir().join("state")
);
shutdown_coordinator.set_local_port(local_port);

// After peer_registry created:
shutdown_coordinator.set_peer_registry(peer_registry.clone());

let shutdown_coordinator = Arc::new(shutdown_coordinator);

// Register signal handlers
let coordinator_for_signals = shutdown_coordinator.clone();
tokio::spawn(async move {
    coordinator_for_signals.listen_for_shutdown().await;
});
```

### 2. Pass Configured Coordinator to EdgeService
**File**: `edge/core/service.rs:74-165`
```rust
// Change EdgeService::new() signature
pub fn new(
    cfg: Arc<Config>,
    bridge_tx: Sender<BridgeMsg>,
    peer_registry: PeerRegistry,
    circuit_breaker_manager: Arc<CircuitBreakerManager>,
    shutdown_coordinator: Arc<ShutdownCoordinator>,  // ADD THIS
) -> Self {
    // Remove local coordinator creation
    // Use passed coordinator instead
    Self {
        // ...
        shutdown_coordinator,
        // ...
    }
}
```

### 3. Track Requests in Proxy Handler
**File**: `edge/core/proxy_impl.rs` (request_filter)
```rust
async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<()> {
    // Track request for graceful shutdown
    let _request_guard = self.shutdown_coordinator.request_start();
    
    // Rest of request handling...
}
```

### 4. Preserve State Periodically
**File**: Add new background service in `main.rs`
```rust
struct StatePersistenceService {
    shutdown_coordinator: Arc<ShutdownCoordinator>,
    peer_registry: PeerRegistry,
    circuit_breaker_manager: Arc<CircuitBreakerManager>,
}

impl BackgroundService for StatePersistenceService {
    fn start(&self, mut shutdown: ShutdownWatch) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let coordinator = self.shutdown_coordinator.clone();
        let peers = self.peer_registry.clone();
        let breakers = self.circuit_breaker_manager.clone();
        
        Box::pin(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        coordinator.update_state(|state| {
                            state.peers = peers.get_all_peers()
                                .into_iter()
                                .map(|a| a.to_string())
                                .collect();
                            // Export circuit breaker states
                        }).await;
                    }
                    _ = shutdown.changed() => break,
                }
            }
        })
    }
}
```

### 5. Restore State on Startup
**File**: `main.rs` (after peer_registry created)
```rust
// Load preserved state
if let Ok(Some(state)) = shutdown_coordinator.load_state().await {
    info!("Restoring state from previous run (shutdown at: {:?})", state.shutdown_at);
    
    // Restore peers
    for peer_str in state.peers {
        if let Ok(addr) = peer_str.parse() {
            peer_registry.add_peer(addr);
        }
    }
    
    // Restore circuit breaker states
    for cb_state in state.circuit_breakers {
        // Restore circuit breaker state
    }
}
```

### 6. Use ShutdownAware Wrapper (Optional)
**File**: Service construction
```rust
let aware_service = ShutdownAware::new(edge_service, shutdown_coordinator.clone());

// Check before processing:
if aware_service.is_shutting_down() {
    return Err(Error::ServiceShuttingDown);
}

if let Some(guard) = aware_service.track_request() {
    // Process request
}
```

## Investigation Required

### Find Request Handler Entry Point
```bash
grep -r "async fn request_filter" packages/sweetmcp/packages/pingora/src/edge/
grep -r "ProxyHttp" packages/sweetmcp/packages/pingora/src/edge/
```

### Check Current Signal Handling
```bash
grep -r "signal::" packages/sweetmcp/packages/pingora/src/
grep -r "SIGTERM\\|SIGINT" packages/sweetmcp/packages/pingora/src/
```

## Files to Modify
- `main.rs` - Setup coordinator fully, register signal handlers, add state persistence service
- `edge/core/service.rs` - Accept coordinator as parameter instead of creating
- `edge/core/proxy_impl.rs` - Track requests with RequestGuard
- `edge/core/builder.rs` - Accept coordinator in builder

## Testing After Reconnection
1. ✅ Send SIGTERM → graceful shutdown triggered
2. ✅ Active requests complete before shutdown
3. ✅ mDNS goodbye packets sent (verify with tcpdump)
4. ✅ Peer notification UDP packets sent
5. ✅ State saved to disk before exit
6. ✅ State restored on next startup (peers, circuit breakers)
7. ✅ New requests rejected during shutdown
8. ✅ Timeout enforced (30s max drain)

## Benefits Unlocked
1. **Zero-downtime deployments** - Requests complete before shutdown
2. **Fast recovery** - State preserved and restored
3. **Mesh coherence** - Peers notified of departure
4. **Clean mDNS** - Goodbye packets prevent stale entries
5. **Audit trail** - Shutdown timestamps preserved
6. **Graceful degradation** - Circuit breaker states maintained
