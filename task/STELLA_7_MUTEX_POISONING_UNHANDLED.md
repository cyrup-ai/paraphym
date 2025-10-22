# Issue: Mutex Poisoning Not Properly Handled

## Severity: MEDIUM
**Impact**: Worker can become permanently stuck after panic

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/stella/loaded.rs:249-251, 338-340`

## Problem Description

When locking the model mutex, poisoning is converted to a simple error string:

```rust
let mut model_guard = model
    .lock()
    .map_err(|_| "Failed to lock model mutex".to_string())?;
```

## What is Mutex Poisoning?

A mutex becomes "poisoned" when a thread panics while holding the lock. Rust's `Mutex::lock()` returns `Result<MutexGuard, PoisonError>` to detect this.

## Current Behavior

If a panic occurs during model inference:
1. Mutex becomes poisoned
2. Next request gets `"Failed to lock model mutex"` error
3. Worker reports error but **remains alive**
4. All subsequent requests fail with the same error
5. Worker never recovers

## Impact

**Permanent worker failure** after any panic:
- Worker appears healthy (not Dead state)
- Pool keeps routing requests to it
- All requests fail
- No automatic recovery

## Example Scenario

```rust
// Request 1: Panic during forward pass
let embeddings = model_guard.forward_norm(...)?;  // ← Panics (OOM, invalid tensor, etc.)
// Mutex is now poisoned

// Request 2+: All fail
let mut model_guard = model.lock().map_err(|_| "Failed to lock model mutex")?;
// ← Returns Err, worker keeps running but is useless
```

## Proper Handling

### Option 1: Recover from Poisoning (Recommended)

```rust
let mut model_guard = model
    .lock()
    .unwrap_or_else(|poisoned| {
        log::warn!("Model mutex was poisoned, recovering...");
        poisoned.into_inner()  // Extract the guard despite poisoning
    });
```

**Pros**:
- Worker can recover
- Mutex state is cleared
- Subsequent requests work

**Cons**:
- Model might be in inconsistent state
- Could propagate corruption

### Option 2: Mark Worker as Dead

```rust
let mut model_guard = model
    .lock()
    .map_err(|_| {
        log::error!("Model mutex poisoned - worker is dead");
        // Set worker state to Dead so pool stops using it
        // Return error that triggers worker shutdown
        "FATAL: Model mutex poisoned"
    })?;
```

**Pros**:
- Clear failure mode
- Pool spawns replacement worker
- No risk of corruption

**Cons**:
- Worker needs restart
- More complex error handling

### Option 3: Panic on Poisoning

```rust
let mut model_guard = model.lock().expect("Model mutex poisoned - worker panic");
```

**Pros**:
- Simple
- Worker task exits cleanly
- Pool detects dead worker

**Cons**:
- Abrupt termination
- No graceful cleanup

## Recommendation

**Use Option 1 (Recovery)** for production resilience:

```rust
let mut model_guard = model
    .lock()
    .unwrap_or_else(|poisoned| {
        log::error!("Model mutex was poisoned, attempting recovery");
        // Log the panic info
        log::error!("Poison error: {:?}", poisoned);
        poisoned.into_inner()
    });
```

Add monitoring to track poison recovery events - if they happen frequently, there's a deeper bug.

## Related Code

Same issue in `batch_embed()` at lines 338-340.
