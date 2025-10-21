# STUB_8: Expose Circuit Breaker Configuration via Getter Methods

## Problem Statement

The `ErrorCircuitBreaker` in [`packages/candle/src/domain/error/breaker.rs`](../packages/candle/src/domain/error/breaker.rs) has two fields marked with `#[allow(dead_code)]`:
- `failure_threshold: usize` 
- `recovery_timeout: Duration`

**Important Discovery**: The circuit breaker functionality is ALREADY FULLY IMPLEMENTED and WORKING. The underlying `CircuitBreaker` in [`circuit_breaker.rs`](../packages/candle/src/domain/error/circuit_breaker.rs) correctly uses these configuration values for state transitions.

The fields are marked as dead code because they're stored redundantly - they're passed to `CircuitBreaker::new()` during construction but never accessed afterward. This is a code quality issue, not a functional defect.

## Current Architecture

### How Circuit Breakers Work (Already Implemented)

The circuit breaker system has two layers:

**1. Generic CircuitBreaker** ([`circuit_breaker.rs`](../packages/candle/src/domain/error/circuit_breaker.rs)):
```rust
pub struct CircuitBreaker {
    state: AtomicU64,           // 0=Closed, 1=Open, 2=HalfOpen
    failure_count: AtomicU64,
    last_failure_time: AtomicU64,
    failure_threshold: u64,      // USED CORRECTLY
    recovery_timeout_ms: u64,    // USED CORRECTLY
}
```

State transitions (ALREADY WORKING):
- **Closed → Open**: When `failure_count >= failure_threshold` (line 92-98)
- **Open → HalfOpen**: When `now - last_failure_time > recovery_timeout_ms` (line 60-66)
- **HalfOpen → Closed**: On successful operation (line 86-88)
- **HalfOpen → Open**: On failed operation (line 92-98)

**2. ErrorCircuitBreaker Wrapper** ([`breaker.rs`](../packages/candle/src/domain/error/breaker.rs)):
```rust
pub struct ErrorCircuitBreaker {
    breaker: CircuitBreaker,              // Does all the work
    counter: ErrorCounter,
    failure_threshold: usize,             // #[allow(dead_code)] - REDUNDANT
    recovery_timeout: Duration,           // #[allow(dead_code)] - REDUNDANT  
    half_open_requests: AtomicU64,
}
```

The constructor properly passes config to the underlying breaker (lines 150-163):
```rust
pub fn new(failure_threshold: usize, recovery_timeout: Duration) -> Self {
    let breaker = CircuitBreaker::new(
        failure_threshold as u64,
        duration_to_millis_u64(recovery_timeout),
    );
    
    Self {
        breaker,  // Config is here, working correctly
        counter: ErrorCounter::new(),
        failure_threshold,      // Stored but never read
        recovery_timeout,       // Stored but never read
        half_open_requests: AtomicU64::new(0),
    }
}
```

### Global Integration

Circuit breakers are created per error category in [`stats.rs`](../packages/candle/src/domain/error/stats.rs) (lines 49-60):
```rust
breakers: [
    ErrorCircuitBreaker::new(10, Duration::from_secs(30)),  // All categories
    ErrorCircuitBreaker::new(10, Duration::from_secs(30)),  // use same config
    // ... 8 more with identical config
]
```

Accessible via: `error_breaker(category: ErrorCategory) -> &'static ErrorCircuitBreaker`

## Goal

Remove the `#[allow(dead_code)]` attributes by making the configuration fields useful. Add public getter methods to expose circuit breaker configuration for:
- Debugging and logging
- Metrics and monitoring  
- Configuration introspection
- Error statistics reporting

## What Needs to Change

### File: `packages/candle/src/domain/error/breaker.rs`

**Current state** (lines 137-142):
```rust
/// Circuit breaker for error recovery
#[derive(Debug)]
pub struct ErrorCircuitBreaker {
    breaker: CircuitBreaker,
    counter: ErrorCounter,
    /// Failure threshold
    #[allow(dead_code)] // TODO: Implement in circuit breaker system
    failure_threshold: usize,
    /// Recovery timeout
    #[allow(dead_code)] // TODO: Implement in circuit breaker system
    recovery_timeout: Duration,
    half_open_requests: AtomicU64,
}
```

**Required changes**:

1. Remove both `#[allow(dead_code)]` attributes
2. Remove the misleading TODO comments  
3. Add getter methods in the `impl ErrorCircuitBreaker` block (after line 223):

```rust
impl ErrorCircuitBreaker {
    // ... existing methods ...

    /// Get configured failure threshold
    #[inline]
    #[must_use]
    pub fn failure_threshold(&self) -> usize {
        self.failure_threshold
    }

    /// Get configured recovery timeout
    #[inline]
    #[must_use]
    pub fn recovery_timeout(&self) -> Duration {
        self.recovery_timeout
    }
}
```

### Optional Enhancement: Configuration Introspection

If you want to make circuit breaker configuration visible in error statistics or logging, you could add a method to format the current configuration:

```rust
impl ErrorCircuitBreaker {
    /// Get circuit breaker configuration summary
    #[must_use]
    pub fn config_summary(&self) -> String {
        format!(
            "threshold={}, timeout={}ms, state={:?}",
            self.failure_threshold,
            self.recovery_timeout.as_millis(),
            self.breaker.get_state()
        )
    }
}
```

But this is OPTIONAL - the core requirement is just adding the two getter methods.

## Why These Fields Exist

The fields serve a legitimate purpose even though they duplicate data stored in `CircuitBreaker`:

1. **Type Preservation**: `CircuitBreaker` stores `recovery_timeout_ms` as `u64`, but users may want the `Duration` type
2. **API Consistency**: Provides a stable public API independent of internal implementation
3. **Future-Proofing**: Allows changing internal representation without breaking the public interface
4. **Debugging**: Makes configuration directly inspectable without reaching into private internals

The `#[allow(dead_code)]` was added because Rust sees the fields are written but never read. Adding getters makes them "used" and removes the warning.

## Implementation Steps

1. Open `packages/candle/src/domain/error/breaker.rs`
2. Locate the `ErrorCircuitBreaker` struct definition (around line 137)
3. Remove `#[allow(dead_code)]` from both `failure_threshold` and `recovery_timeout` fields
4. Remove the TODO comments (they're incorrect - the system IS implemented)
5. Scroll to the `impl ErrorCircuitBreaker` block
6. Add the two getter methods at the end of the impl block (before the closing brace)
7. Ensure they're marked `#[inline]` and `#[must_use]` for performance and API safety

## Definition of Done

- `#[allow(dead_code)]` removed from `failure_threshold` field
- `#[allow(dead_code)]` removed from `recovery_timeout` field  
- Misleading TODO comments removed
- `pub fn failure_threshold(&self) -> usize` method added
- `pub fn recovery_timeout(&self) -> Duration` method added
- Code compiles without warnings: `cargo check -p paraphym_candle`
- No functional changes - circuit breaker continues working as before

## What NOT to Do

- Do NOT modify `CircuitBreaker` in `circuit_breaker.rs` - it's already correct
- Do NOT change circuit breaker logic - state transitions are working
- Do NOT add complex configuration loading systems
- Do NOT refactor the wrapper pattern - it's appropriate as-is
- Do NOT remove the fields - they provide useful API surface

## Reference Implementation in Codebase

The circuit breaker state machine is fully implemented in [`circuit_breaker.rs`](../packages/candle/src/domain/error/circuit_breaker.rs):

- **State transitions**: Lines 50-76 in `call()` method
- **Failure tracking**: Lines 78-103 in `execute_operation()` method
- **Timeout checking**: Lines 60-66 (Open → HalfOpen transition)
- **Threshold checking**: Lines 92-98 (increment and trip)

Error statistics integration in [`stats.rs`](../packages/candle/src/domain/error/stats.rs):
- **Global breakers**: Lines 16 (field), 49-60 (initialization)
- **Public API**: Line 155 `error_breaker(category)`

## Notes

This task was originally titled "Restore Circuit Breaker Threshold and Timeout Configuration" which implied broken functionality. Research revealed the circuit breaker is fully functional - this is purely a code quality improvement to expose configuration for introspection.

The hardcoded configuration in `stats.rs` (10 failures, 30 seconds for all categories) could be made configurable in a future task, but that's outside the scope of this issue.
