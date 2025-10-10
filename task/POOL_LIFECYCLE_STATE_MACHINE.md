# POOL_LIFECYCLE_STATE_MACHINE

**Priority**: CRITICAL  
**Component**: pool/core
**Estimated Effort**: 2 days
**Risk**: Medium
**Dependencies**: POOL_UNIFIED_STORAGE

## Problem Statement

Current implementation has no worker lifecycle tracking:
- Model load failures leave zombie threads with corrupt memory tracking
- No way to know if worker is loading, ready, processing, or dead
- No cleanup on failure paths
- No visibility into worker health

## Solution Design

### Worker State Machine

```rust
// pool/core/state.rs
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerState {
    Spawning = 0,    // Thread created, not yet loading
    Loading = 1,     // Model weights loading
    Ready = 2,       // Ready to accept requests
    Processing = 3,  // Currently processing request
    Idle = 4,        // No requests for idle_threshold
    Evicting = 5,    // Shutdown signal sent
    Dead = 6,        // Thread terminated
    Failed = 7,      // Model load or runtime failure
}

pub struct WorkerStateMachine {
    state: Arc<AtomicU32>,
    spawn_time: Instant,
    transitions: Arc<RwLock<Vec<StateTransition>>>,
}

pub struct StateTransition {
    from: WorkerState,
    to: WorkerState,
    timestamp: Instant,
    reason: Option<String>,
}
```

### Valid State Transitions

```
Spawning -> Loading    (model load started)
Loading -> Ready       (model loaded successfully)
Loading -> Failed      (model load error)
Ready -> Processing    (request started)
Processing -> Ready    (request completed)
Processing -> Failed   (request error)
Ready -> Idle         (no activity timeout)
Idle -> Ready         (new request arrived)
* -> Evicting         (shutdown requested)
Evicting -> Dead      (thread exited)
Failed -> Dead        (cleanup completed)
```

### Integration with Worker Spawning

```rust
impl Pool<dyn TextEmbeddingCapable> {
    pub fn spawn_text_embedding_worker<T, F>(
        &self,
        registry_key: &str,
        model_loader: F,
        per_worker_mb: usize,
    ) -> Result<WorkerHandle, PoolError>
    where
        F: FnOnce() -> Result<T, PoolError> + Send + 'static,
        T: TextEmbeddingCapable + Send + 'static,
    {
        let worker_id = self.next_worker_id();
        let state_machine = WorkerStateMachine::new();
        
        // Create cleanup channel for failure cases
        let (cleanup_tx, cleanup_rx) = bounded(1);
        
        // Clone for thread
        let state_clone = state_machine.state.clone();
        let cleanup_tx_clone = cleanup_tx.clone();
        let memory_tracker = self.total_memory_used.clone();
        
        std::thread::spawn(move || {
            // Transition: Spawning -> Loading
            state_clone.store(WorkerState::Loading as u32, Ordering::Release);
            
            let model = match model_loader() {
                Ok(m) => {
                    // Transition: Loading -> Ready
                    state_clone.store(WorkerState::Ready as u32, Ordering::Release);
                    info!("Worker {} ready", worker_id);
                    m
                }
                Err(e) => {
                    // Transition: Loading -> Failed
                    state_clone.store(WorkerState::Failed as u32, Ordering::Release);
                    error!("Worker {} failed: {}", worker_id, e);
                    
                    // Signal cleanup needed
                    let _ = cleanup_tx_clone.send(CleanupRequest {
                        worker_id,
                        memory_mb: per_worker_mb,
                        error: e.to_string(),
                    });
                    
                    return; // Exit thread
                }
            };
            
            // Run worker loop with state tracking
            worker_loop_with_state(
                model,
                state_clone,
                request_rx,
                response_tx,
                shutdown_rx,
            );
            
            // Transition: * -> Dead
            state_clone.store(WorkerState::Dead as u32, Ordering::Release);
        });
        
        // Start cleanup monitor
        self.monitor_cleanup(cleanup_rx, per_worker_mb);
        
        Ok(WorkerHandle {
            worker_id,
            state: state_machine,
            // ... other fields
        })
    }
}
```

### Worker Loop with State Tracking

```rust
fn worker_loop_with_state<T: TextEmbeddingCapable>(
    model: T,
    state: Arc<AtomicU32>,
    request_rx: Receiver<EmbedRequest>,
    response_tx: Sender<EmbedResponse>,
    shutdown_rx: Receiver<()>,
) {
    let idle_threshold = Duration::from_secs(300);
    let mut last_activity = Instant::now();
    
    loop {
        select! {
            recv(request_rx) -> req => {
                if let Ok(req) = req {
                    // Transition: Ready/Idle -> Processing
                    let prev_state = state.swap(WorkerState::Processing as u32, Ordering::AcqRel);
                    
                    let result = model.embed(&req.text, req.task);
                    let _ = response_tx.send(result);
                    
                    // Transition: Processing -> Ready
                    state.store(WorkerState::Ready as u32, Ordering::Release);
                    last_activity = Instant::now();
                }
            }
            recv(shutdown_rx) -> _ => {
                // Transition: * -> Evicting
                state.store(WorkerState::Evicting as u32, Ordering::Release);
                info!("Worker shutting down");
                break;
            }
            default(Duration::from_secs(1)) => {
                // Check idle timeout
                if last_activity.elapsed() > idle_threshold {
                    let current = WorkerState::from(state.load(Ordering::Acquire));
                    if current == WorkerState::Ready {
                        // Transition: Ready -> Idle
                        state.store(WorkerState::Idle as u32, Ordering::Release);
                    }
                }
            }
        }
    }
}
```

### Cleanup Monitor

```rust
impl<T> Pool<T> {
    fn monitor_cleanup(&self, cleanup_rx: Receiver<CleanupRequest>, default_memory_mb: usize) {
        let memory_tracker = self.total_memory_used.clone();
        let metrics = self.metrics.clone();
        
        std::thread::spawn(move || {
            while let Ok(cleanup) = cleanup_rx.recv() {
                warn!("Cleaning up failed worker {}: {}", cleanup.worker_id, cleanup.error);
                
                // Release memory that was never actually allocated
                memory_tracker.fetch_sub(cleanup.memory_mb, Ordering::Release);
                
                // Update metrics
                metrics.workers_failed.fetch_add(1, Ordering::Release);
                
                // Could trigger alerts here
            }
        });
    }
}
```

### Health Check Integration

```rust
impl WorkerHandle {
    pub fn get_health(&self) -> WorkerHealth {
        let state = WorkerState::from(self.state.load(Ordering::Acquire));
        
        WorkerHealth {
            worker_id: self.worker_id,
            state,
            is_healthy: matches!(state, WorkerState::Ready | WorkerState::Processing | WorkerState::Idle),
            uptime: self.spawn_time.elapsed(),
            pending_requests: self.pending_requests.load(Ordering::Acquire),
            processed_total: self.processed_requests.load(Ordering::Acquire),
            last_error: self.last_error.read().clone(),
        }
    }
    
    pub fn requires_restart(&self) -> bool {
        matches!(
            WorkerState::from(self.state.load(Ordering::Acquire)),
            WorkerState::Failed | WorkerState::Dead
        )
    }
}
```

## Implementation Steps

1. **Create state.rs** with WorkerState enum and StateMachine
2. **Add state tracking** to WorkerHandle struct
3. **Update spawn methods** to use state transitions
4. **Add cleanup monitor** for failure cases
5. **Update worker loops** to track state changes
6. **Add health check methods** using state
7. **Update eviction logic** to check state
8. **Add state metrics** to Prometheus export

## Acceptance Criteria

- [ ] All workers have state machine tracking
- [ ] Model load failures properly cleanup memory
- [ ] State transitions are atomic and validated
- [ ] Health checks use worker state
- [ ] Failed workers can be identified and restarted
- [ ] Metrics track state transitions
- [ ] No zombie threads or memory corruption

## Testing Strategy

1. **State Transition Test**: Verify only valid transitions allowed
2. **Failure Cleanup Test**: Force model load failures, verify cleanup
3. **Concurrent State Test**: Multiple threads updating state
4. **Health Check Test**: Verify health reflects actual state
5. **Recovery Test**: Failed workers properly restart

## Integration Points

- Maintenance thread uses state to identify dead workers
- Load balancer considers worker state for routing
- Metrics export worker state distribution
- Shutdown waits for Processing -> Ready transition

## Success Metrics

- Zero zombie threads after failures
- 100% memory cleanup on failure paths
- State transitions < 1μs
- Health checks accurate 100% of time