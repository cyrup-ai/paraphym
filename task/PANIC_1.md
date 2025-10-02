# PANIC_1: Fix Thread Spawn Panics in Coordinator

## OBJECTIVE
Replace panic-inducing `expect()` calls with proper error handling in the MemoryCoordinator thread spawning logic.

## PRIORITY
🔴 CRITICAL - Must fix before production deployment

## BACKGROUND
The MemoryCoordinator spawns cognitive worker threads during initialization. Currently uses `expect()` which causes the entire application to panic if thread creation fails (e.g., OS thread limits, resource exhaustion). This violates production error handling standards.

## SUBTASK 1: Update Constructor Signature
**File:** `packages/candle/src/memory/core/manager/coordinator.rs`  
**Change:** Modify the `new()` constructor to return `Result<Self, Error>` instead of `Self`

**Current behavior:**
- Constructor panics on thread spawn failure
- No recovery possible

**Required change:**
- Return `Result<Self, Error>` 
- Propagate thread spawn errors to caller
- Allow graceful degradation

## SUBTASK 2: Replace expect() at Line 126
**File:** `packages/candle/src/memory/core/manager/coordinator.rs`  
**Line:** 126

**Current code:**
```rust
.expect("Failed to spawn cognitive worker thread");
```

**Required replacement:**
```rust
.map_err(|e| Error::Internal(format!("Failed to spawn cognitive worker {}: {}", worker_id, e)))?;
```

## SUBTASK 3: Replace expect() at Line 267
**File:** `packages/candle/src/memory/core/manager/coordinator.rs`  
**Line:** 267

**Current code:**
```rust
.expect("Failed to spawn cognitive worker thread");
```

**Required replacement:**
```rust
.map_err(|e| Error::Internal(format!("Failed to spawn cognitive worker {}: {}", worker_id, e)))?;
```

## SUBTASK 4: Update All Call Sites
**Action:** Update all locations that call `MemoryCoordinator::new()` to handle the new `Result` return type

**Files to check:**
- Search for all `MemoryCoordinator::new(` calls
- Add proper error handling (`.map_err()`, `?`, or `match`)

## DEFINITION OF DONE
- [ ] Constructor returns `Result<Self, Error>`
- [ ] Both `expect()` calls replaced with `?` operator
- [ ] All call sites updated to handle `Result`
- [ ] Code compiles without warnings
- [ ] No panic paths in thread spawning logic
- [ ] Error messages include worker_id for debugging

## CONSTRAINTS
- ❌ DO NOT write unit tests
- ❌ DO NOT write integration tests  
- ❌ DO NOT write benchmarks
- ✅ Focus solely on ./src modifications

## TECHNICAL NOTES
- Thread spawn failures are rare but critical
- Proper error handling allows graceful shutdown
- Error messages should include worker ID for diagnostics
- Consider using `std::thread::Builder` for better error context
