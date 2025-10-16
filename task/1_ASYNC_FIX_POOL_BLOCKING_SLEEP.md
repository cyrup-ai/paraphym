# Task: Convert Pool::wait_for_workers Blocking Sleep to Async

## Location
`packages/candle/src/pool/core/pool.rs` lines 283-315

## Problem
**CATASTROPHIC BLOCKING OPERATION IN ASYNC CONTEXT**

```rust
pub fn wait_for_workers(&self, registry_key: &str, timeout: Duration) -> Result<(), PoolError> {
    let start = Instant::now();

    loop {
        // Check if workers are ready
        if self.has_workers(registry_key) {
            debug!("Workers ready for {}", registry_key);
            return Ok(());
        }

        // ... checks omitted ...

        // ❌❌❌ BLOCKING SLEEP IN TOKIO RUNTIME ❌❌❌
        std::thread::sleep(Duration::from_millis(50));
        //  ^^^^^^^^^^^^ BLOCKS ENTIRE TOKIO THREAD FOR 50ms
    }
}
```

**Impact:**
- Blocks tokio worker thread every 50ms during worker spawn wait
- Can block for up to 30 seconds (timeout) with repeated 50ms sleeps
- Multiple concurrent requests = multiple blocked threads
- **Runtime thread starvation under load**

## Solution

Convert to async with tokio::time::sleep:

```rust
pub async fn wait_for_workers(&self, registry_key: &str, timeout: Duration) -> Result<(), PoolError> {
    let start = Instant::now();

    loop {
        // Check if workers are ready
        if self.has_workers(registry_key) {
            debug!("Workers ready for {}", registry_key);
            return Ok(());
        }

        // Check if spawning thread released lock (spawn completed or failed)
        if let Some(flag) = self.spawning_in_progress.get(registry_key)
            && !flag.load(Ordering::Acquire)
        {
            // Spawning finished but no workers available = spawn failed
            return Err(PoolError::SpawnFailed(format!(
                "Worker spawning completed for {} but no workers available. \
                     Check logs for model loading errors.",
                registry_key
            )));
        }

        // Check timeout
        if start.elapsed() > timeout {
            return Err(PoolError::SpawnTimeout(format!(
                "Timed out after {:?} waiting for {} workers to spawn",
                timeout, registry_key
            )));
        }

        // ✅ ASYNC SLEEP - yields to tokio scheduler
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
```

## Call Sites to Update

From previous search, called in:
- `pool/core/spawn.rs:108` - `pool.wait_for_workers(registry_key, Duration::from_secs(30))`
- `pool/core/spawn.rs:185` - `pool.wait_for_workers(registry_key, Duration::from_secs(30))`
- `pool/core/spawn.rs:267` - Trait implementation wrapper

All must add `.await`.

## Additional Changes Required

1. **SpawnLock trait** (`spawn.rs:239`):
```rust
async fn wait_for_workers(&self, registry_key: &str, timeout: Duration) -> Result<(), PoolError>;
```

2. **Trait implementation** (`spawn.rs:266-268`):
```rust
async fn wait_for_workers(&self, registry_key: &str, timeout: Duration) -> Result<(), PoolError> {
    Pool::wait_for_workers(self, registry_key, timeout).await
}
```

3. **All call sites** in spawn functions must be in async context and use `.await`

## Steps
1. Change `Pool::wait_for_workers` to `async fn`
2. Replace `std::thread::sleep` with `tokio::time::sleep().await`
3. Update `SpawnLock` trait to have `async fn wait_for_workers`
4. Update trait implementation to be async and add `.await`
5. Find all call sites and add `.await`
6. Verify calling functions are async or spawn async tasks
7. Test pool spawning under load

## Priority
🔥 **CRITICAL** - Blocking sleep in hot path during concurrent model loading

## Status
⏳ TODO
