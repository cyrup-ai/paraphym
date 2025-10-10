# POOL_ADAPTIVE_SCALING

**Priority**: LOW
**Component**: pool/core
**Estimated Effort**: 4 hours
**Risk**: Low
**Dependencies**: None

## Problem Statement

Missing requirement: When all workers are busy, spawn one additional worker (up to configured max).

Current behavior:
- ✅ 0 workers until first request
- ✅ Cold start spawns 2 workers (if memory allows)
- ❌ Never spawns more workers even when all are busy

## Solution Design

### Simple Load-Based Scaling

Modify `ensure_workers_spawned` to handle the "all busy" case:

```rust
// pool/core/spawn.rs - Enhanced version
pub fn ensure_workers_spawned_adaptive<P, F>(
    pool: &P,
    registry_key: &str,
    per_worker_mb: usize,
    max_workers: usize,  // New parameter (default: 4)
    spawn_fn: F,
) -> Result<(), PoolError>
where
    P: HasWorkers + TotalMemory + SpawnLock + WorkerMetrics,
    F: Fn(usize) -> Result<(), PoolError>,
{
    // Check if we have workers
    let worker_count = pool.worker_count(registry_key);

    if worker_count == 0 {
        // Cold start: spawn 1-2 workers as before
        if let Some(_guard) = pool.try_acquire_spawn_lock(registry_key) {
            let current_mb = pool.total_memory_mb();
            let total_system_mb = query_system_memory_mb();
            let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;

            let workers_to_spawn = if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
                2
            } else if current_mb + per_worker_mb <= memory_limit_mb {
                1
            } else {
                return Err(PoolError::MemoryExhausted(format!(
                    "Cannot spawn workers for {}. Need {} MB, only {} MB available",
                    registry_key, per_worker_mb, memory_limit_mb.saturating_sub(current_mb)
                )));
            };

            for idx in 0..workers_to_spawn {
                spawn_fn(idx)?;
            }

            return Ok(());
        } else {
            return pool.wait_for_workers(registry_key, Duration::from_secs(30));
        }
    }

    // NEW: Check if all workers are busy
    if worker_count < max_workers {
        let busy_count = pool.busy_worker_count(registry_key);

        // If all workers are busy, try to spawn one more
        if busy_count >= worker_count {
            if let Some(_guard) = pool.try_acquire_spawn_lock(registry_key) {
                // Double-check after acquiring lock
                let current_count = pool.worker_count(registry_key);
                let current_busy = pool.busy_worker_count(registry_key);

                if current_busy >= current_count && current_count < max_workers {
                    // Check memory
                    let current_mb = pool.total_memory_mb();
                    let total_system_mb = query_system_memory_mb();
                    let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;

                    if current_mb + per_worker_mb <= memory_limit_mb {
                        log::info!(
                            "All {} workers busy for {}, spawning 1 more (max: {})",
                            current_count, registry_key, max_workers
                        );
                        spawn_fn(current_count)?;
                    }
                }
            }
        }
    }

    Ok(())
}

// Add new trait for worker metrics
pub trait WorkerMetrics {
    fn worker_count(&self, registry_key: &str) -> usize;
    fn busy_worker_count(&self, registry_key: &str) -> usize;
}

impl<T: ?Sized> WorkerMetrics for Pool<T> {
    fn worker_count(&self, registry_key: &str) -> usize {
        self.workers
            .get(registry_key)
            .map(|workers| workers.len())
            .unwrap_or(0)
    }

    fn busy_worker_count(&self, registry_key: &str) -> usize {
        self.workers
            .get(registry_key)
            .map(|workers| {
                workers.iter()
                    .filter(|w| w.pending_requests.load(Ordering::Acquire) > 0)
                    .count()
            })
            .unwrap_or(0)
    }
}
```

## Implementation Steps

1. **Enhance spawn.rs** with the adaptive function above
2. **Add WorkerMetrics trait** to pool.rs
3. **Update registry.rs** to call `ensure_workers_spawned_adaptive` with max_workers=4
4. **Test with concurrent requests**

## Configuration

```rust
// Simple configuration - add to PoolConfig
pub struct PoolConfig {
    // ... existing fields ...
    pub max_workers_per_model: usize,  // Default: 4
}
```

## Acceptance Criteria

- [ ] Cold start spawns 2 workers (unchanged)
- [ ] When all workers busy, spawns 1 additional (up to max)
- [ ] Respects memory constraints
- [ ] No complex background threads
- [ ] Simple, deterministic behavior

## Testing Strategy

1. **Cold Start Test**: First request spawns 2 workers
2. **Load Test**: 3+ concurrent requests trigger 3rd worker spawn
3. **Max Test**: Verify stops at max_workers limit
4. **Memory Test**: High memory prevents additional spawns

## Success Metrics

- Zero added complexity
- < 50 lines of new code
- No performance overhead when not scaling