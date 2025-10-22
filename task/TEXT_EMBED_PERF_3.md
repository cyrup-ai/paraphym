# Performance Issue: Idle Detection Runs on Every Loop Iteration

## Location
**File:** [`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs`](../packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs)

**Lines:** 113-124

## Severity
**LOW** - Minor CPU waste, but unnecessary overhead in hot path

---

## Issue Description

The idle detection check runs on **every iteration** of the worker loop, creating unnecessary CPU overhead:

```rust
loop {
    // ❌ PROBLEMATIC: Runs on EVERY loop iteration
    if let Ok(elapsed) = last_activity.elapsed()
        && elapsed > idle_threshold
    {
        let current_state = WorkerState::from(state.load(std::sync::atomic::Ordering::Acquire));
        if matches!(current_state, WorkerState::Ready) {
            state.store(
                WorkerState::Idle as u32,
                std::sync::atomic::Ordering::Release,
            );
        }
    }

    tokio::select! {
        // ... handle requests
    }
}
```

### Why This Is Inefficient

On every loop iteration (potentially thousands per second):
1. **Syscall overhead**: `last_activity.elapsed()` calls the OS to get current time
2. **Atomic load**: `state.load()` with Acquire ordering
3. **Pattern matching**: State comparison
4. **Potential atomic store**: If transitioning to Idle

For a worker processing 1000 requests/second:
- 1000 unnecessary time syscalls/sec
- 1000 unnecessary atomic loads/sec
- 1000 unnecessary comparisons/sec

While each operation is cheap (microseconds), this is **wasteful polling** that can be eliminated.

---

## Recommended Solution: Timeout-Based Approach

Use **`tokio::time::sleep` with reset** for accurate idle detection:

### Why This Approach?

1. **Accurate timing**: Idle transition happens exactly after 5 minutes of inactivity
2. **Zero polling**: No syscalls or checks on every iteration
3. **Event-driven**: Timer is just another branch in the select!
4. **Idiomatic Rust**: Standard tokio pattern for timeouts

### Complete Implementation

```rust
use tokio::time::{sleep, Instant};  // Add to imports at top of file

pub async fn text_embedding_worker<T: TextEmbeddingCapable>(
    model: T,
    channels: TextEmbeddingWorkerChannels,
    context: TextEmbeddingWorkerContext,
) {
    // ... existing setup code ...

    let idle_threshold = Duration::from_secs(300); // 5 minutes
    
    // Create timeout OUTSIDE loop
    let timeout = sleep(idle_threshold);
    tokio::pin!(timeout);  // Pin to stack for reuse in loop

    loop {
        tokio::select! {
            // ✅ NEW: Idle timeout branch
            _ = &mut timeout => {
                let current_state = WorkerState::from(state.load(Ordering::Acquire));
                if matches!(current_state, WorkerState::Ready) {
                    state.store(WorkerState::Idle as u32, Ordering::Release);
                }
                // Reset timeout for next check
                timeout.as_mut().reset(Instant::now() + idle_threshold);
            }
            
            Some(req) = embed_rx.recv() => {
                state.store(WorkerState::Processing as u32, Ordering::Release);
                
                let result = model.embed(&req.text, req.task)
                    .await
                    .map_err(|e| PoolError::ModelError(e.to_string()));
                let _ = req.response.send(result);
                
                state.store(WorkerState::Ready as u32, Ordering::Release);
                
                // ✅ Reset timeout on activity
                timeout.as_mut().reset(Instant::now() + idle_threshold);
            }
            
            Some(req) = batch_embed_rx.recv() => {
                state.store(WorkerState::Processing as u32, Ordering::Release);
                
                let result = model.batch_embed(&req.texts, req.task)
                    .await
                    .map_err(|e| PoolError::ModelError(e.to_string()));
                let _ = req.response.send(result);
                
                state.store(WorkerState::Ready as u32, Ordering::Release);
                
                // ✅ Reset timeout on activity
                timeout.as_mut().reset(Instant::now() + idle_threshold);
            }
            
            Some(_ping) = health_rx.recv() => {
                // ... existing health check code ...
            }
            
            Some(_) = shutdown_rx.recv() => {
                log::info!("TextEmbedding worker {} shutting down", worker_id);
                state.store(WorkerState::Evicting as u32, Ordering::Release);
                break;
            }
        }
    }
}
```

---

## Technical Details

### Why `tokio::pin!`?

The `Sleep` future returned by `tokio::time::sleep()` must be **pinned** to be reused in a loop:
- `&mut timeout` requires the future to not move in memory
- `tokio::pin!` pins it to the stack
- `timeout.as_mut().reset()` allows reusing the same future with a new deadline

### Reference: tokio::pin! Usage in Codebase

Similar pattern used in [`workflow/ops.rs:200`](../packages/candle/src/workflow/ops.rs#L200):
```rust
let first_stream = first.call(input);
tokio::pin!(first_stream);
```

---

## Changes Required

### 1. Add Imports (Top of File)

**File:** [`text_embedding.rs:1-5`](../packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs#L1-L5)

```rust
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio::time::{sleep, Instant};  // ✅ ADD THIS LINE
```

### 2. Modify Worker Function

**File:** [`text_embedding.rs:85-188`](../packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs#L85-L188)

#### Remove Old Code (Lines 108-109, 113-124)

Remove these lines:
```rust
// ❌ DELETE: Line 108-109
let mut last_activity = SystemTime::now();
let idle_threshold = Duration::from_secs(300);

// ❌ DELETE: Lines 113-124 (the entire polling check before tokio::select!)
if let Ok(elapsed) = last_activity.elapsed()
    && elapsed > idle_threshold
{
    let current_state = WorkerState::from(state.load(std::sync::atomic::Ordering::Acquire));
    if matches!(current_state, WorkerState::Ready) {
        state.store(
            WorkerState::Idle as u32,
            std::sync::atomic::Ordering::Release,
        );
    }
}
```

#### Add New Code (Before Loop)

```rust
// ✅ ADD: After line 107, before `loop {`
let idle_threshold = Duration::from_secs(300);
let timeout = sleep(idle_threshold);
tokio::pin!(timeout);
```

#### Add Timeout Branch to select!

```rust
// ✅ ADD: As FIRST branch in tokio::select! (after line 125)
tokio::select! {
    _ = &mut timeout => {
        let current_state = WorkerState::from(state.load(Ordering::Acquire));
        if matches!(current_state, WorkerState::Ready) {
            state.store(WorkerState::Idle as u32, Ordering::Release);
        }
        timeout.as_mut().reset(Instant::now() + idle_threshold);
    }
    
    // ... existing branches ...
}
```

#### Add Timeout Reset in Request Branches

In `embed_rx.recv()` branch (after line 139):
```rust
state.store(WorkerState::Ready as u32, Ordering::Release);
timeout.as_mut().reset(Instant::now() + idle_threshold);  // ✅ ADD
```

In `batch_embed_rx.recv()` branch (after line 155):
```rust
state.store(WorkerState::Ready as u32, Ordering::Release);
timeout.as_mut().reset(Instant::now() + idle_threshold);  // ✅ ADD
```

---

## Alternative: Interval-Based Approach (Not Recommended)

```rust
let mut idle_check_interval = tokio::time::interval(Duration::from_secs(60));
idle_check_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

loop {
    tokio::select! {
        _ = idle_check_interval.tick() => {
            if let Ok(elapsed) = last_activity.elapsed()
                && elapsed > idle_threshold
            {
                let current_state = WorkerState::from(state.load(Ordering::Acquire));
                if matches!(current_state, WorkerState::Ready) {
                    state.store(WorkerState::Idle as u32, Ordering::Release);
                }
            }
        }
        // ... other branches ...
    }
}
```

**Why Not Recommended:**
- Still requires `last_activity.elapsed()` syscall every 60 seconds
- Less accurate: Could transition to Idle anywhere from 5:00 to 6:00 after last activity
- More complex: Requires both interval timer AND last_activity tracking

---

## Performance Impact

### Before Fix
- **Syscalls**: 1000/sec (on busy worker)
- **Atomic loads**: 1000/sec
- **CPU overhead**: Continuous polling

### After Fix  
- **Syscalls**: 0 (timer-driven)
- **Atomic loads**: Only on timeout (once per 5 minutes)
- **CPU overhead**: None - event-driven

### Estimated Improvement
- **CPU savings**: ~0.1-1% per worker (depends on request rate)
- **Syscall reduction**: 100% elimination of polling syscalls
- **Code quality**: More idiomatic, cleaner architecture

---

## Definition of Done

The implementation is complete when:

1. ✅ Imports include `use tokio::time::{sleep, Instant};`
2. ✅ `last_activity` variable is removed from worker function
3. ✅ Timeout created before loop with `sleep(idle_threshold)` and pinned
4. ✅ Timeout branch added as first branch in `tokio::select!`
5. ✅ Timeout reset called in both `embed_rx` and `batch_embed_rx` branches after processing
6. ✅ Old polling code (lines 113-124) completely removed
7. ✅ Code compiles without errors or warnings
8. ✅ Worker still correctly transitions Ready → Idle after 5 minutes of inactivity
9. ✅ Worker still correctly transitions Idle/Ready → Processing → Ready on requests

---

## Related Files

Other worker files with same pattern (NOT in scope of this task):
- [`image_embedding.rs:124-136`](../packages/candle/src/capability/registry/pool/capabilities/image_embedding.rs#L124-L136)
- Similar idle detection pattern exists but is separate work

---

## References

- **Tokio Sleep Documentation**: https://docs.rs/tokio/latest/tokio/time/fn.sleep.html
- **Tokio Pin Macro**: https://docs.rs/tokio/latest/tokio/macro.pin.html  
- **Codebase Example**: [`workflow/ops.rs:200`](../packages/candle/src/workflow/ops.rs#L200)
