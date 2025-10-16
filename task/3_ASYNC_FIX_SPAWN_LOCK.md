# Task: Convert SpawnLock::wait_for_workers to Async

## Location
`packages/candle/src/pool/core/spawn.rs` lines 237-240

## Problem
```rust
pub trait SpawnLock {
    fn try_acquire_spawn_lock(&self, registry_key: &str) -> Option<SpawnGuard>;
    fn wait_for_workers(&self, registry_key: &str, timeout: Duration) -> Result<(), PoolError>;
    //  ^^^^^^^^^^^^^^^^ ❌ BLOCKING WAIT
}
```

**Issue**: Method is called "wait_for_workers" with a timeout - this implies blocking/sleeping which will block tokio runtime!

## Solution
Convert to async:

```rust
pub trait SpawnLock {
    fn try_acquire_spawn_lock(&self, registry_key: &str) -> Option<SpawnGuard>;
    async fn wait_for_workers(&self, registry_key: &str, timeout: Duration) -> Result<(), PoolError>;
    //   ^^^^^ ✅ ASYNC
}
```

## Implementation Update Required
Also update the implementation in the same file (lines 266-268):

```rust
impl<W: super::types::PoolWorkerHandle> SpawnLock for Pool<W> {
    fn try_acquire_spawn_lock(&self, registry_key: &str) -> Option<SpawnGuard> {
        Pool::try_acquire_spawn_lock(self, registry_key)
    }

    async fn wait_for_workers(&self, registry_key: &str, timeout: Duration) -> Result<(), PoolError> {
    //   ^^^^^ Add async
        Pool::wait_for_workers(self, registry_key, timeout).await
        //                                                   ^^^^^^ Add await
    }
}
```

## Steps
1. Add `async` to trait method signature
2. Add `async` to impl method signature  
3. Add `.await` to the `Pool::wait_for_workers` call
4. Check the actual `Pool::wait_for_workers` implementation and make it async if needed
5. Find all call sites and update to await the call
6. Replace any blocking sleep/wait with `tokio::time::sleep` or similar

## Priority
🔴 **HIGH** - Any "wait" operation should be async to avoid blocking the runtime

## Status
⏳ TODO
