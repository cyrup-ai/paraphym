# Performance Issue: Idle Detection Runs on Every Loop Iteration

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs`

Lines 113-124

## Severity
**LOW** - Minor CPU waste

## Issue Description

The idle detection check runs on every iteration of the worker loop:

```rust
loop {
    // Check for idle timeout (Ready → Idle after 5 minutes of inactivity)
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

## Impact

### CPU Overhead

On every loop iteration (potentially thousands per second):
1. Call `last_activity.elapsed()` (syscall to get current time)
2. Compare elapsed time
3. Load atomic state
4. Pattern match state
5. Potentially store new state

If worker processes 1000 requests/sec:
- 1000 time syscalls/sec
- 1000 atomic loads/sec
- 1000 comparisons/sec

### Actual Impact

The overhead is small (microseconds per check), but it's unnecessary work.

## Fix: Use tokio::time::interval

```rust
let mut idle_check_interval = tokio::time::interval(Duration::from_secs(60));
idle_check_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

loop {
    tokio::select! {
        _ = idle_check_interval.tick() => {
            // Check for idle timeout
            if let Ok(elapsed) = last_activity.elapsed()
                && elapsed > idle_threshold
            {
                let current_state = WorkerState::from(state.load(Ordering::Acquire));
                if matches!(current_state, WorkerState::Ready) {
                    state.store(WorkerState::Idle as u32, Ordering::Release);
                }
            }
        }
        Some(req) = embed_rx.recv() => {
            // ... handle request
        }
        // ... other branches
    }
}
```

## Benefits

1. **Reduced CPU**: Check runs every 60 seconds instead of every loop iteration
2. **Better precision**: Timer-based instead of polling
3. **Cleaner code**: Idle check is just another select branch

## Alternative: Timeout on Select

```rust
loop {
    let timeout = tokio::time::sleep(idle_threshold);
    tokio::pin!(timeout);

    tokio::select! {
        _ = &mut timeout => {
            // Idle timeout reached
            let current_state = WorkerState::from(state.load(Ordering::Acquire));
            if matches!(current_state, WorkerState::Ready) {
                state.store(WorkerState::Idle as u32, Ordering::Release);
            }
        }
        Some(req) = embed_rx.recv() => {
            // Reset timeout on activity
            timeout.as_mut().reset(tokio::time::Instant::now() + idle_threshold);
            // ... handle request
        }
        // ... other branches
    }
}
```

This approach resets the timeout on each request, which is more accurate.

## Recommendation

Use the interval-based approach for simplicity, or the timeout-based approach for accuracy.
