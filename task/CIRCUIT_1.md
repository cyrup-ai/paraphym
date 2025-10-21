# CIRCUIT_1: Implement Circuit Breaker System

## OBJECTIVE

Implement the circuit breaker monitoring system to provide proper error tracking for failure thresholds and recovery timeouts.

## BACKGROUND

The circuit breaker infrastructure exists but critical monitoring fields are marked as dead code with TODO comments. This prevents proper error tracking and recovery in production systems.

## SUBTASK 1: Implement Circuit Breaker Statistics Tracking

**Location:** `packages/candle/src/domain/error/breaker.rs:137-143`

**Current State:**
```rust
#[allow(dead_code)] // TODO: Implement in circuit breaker system
failure_threshold: usize,
/// Recovery timeout
#[allow(dead_code)] // TODO: Implement in circuit breaker system
recovery_timeout: Duration,
```

**Required Changes:**
- Remove `#[allow(dead_code)]` attributes
- Add state management fields: `state: AtomicU64`, `failure_count: AtomicU64`, `last_failure_time: AtomicU64`
- Add `recovery_timeout_ms()` helper method to convert Duration to milliseconds
- Implement `record_failure()` method that increments failure count and opens circuit when threshold reached
- Implement `record_success()` method that resets failure count and closes circuit
- Implement `should_allow_request()` method with timeout-based recovery logic
- Modify `execute()` method to use new state management instead of delegating to generic CircuitBreaker
- Update `is_open()`, `is_half_open()`, and `reset()` methods to use local state fields

**Why:** The circuit breaker pattern requires these fields to function. Dead code indicates incomplete implementation that must be activated for proper monitoring.

## SUBTASK 2: Create Circuit State Management

**Location:** Same file, enhance `ErrorCircuitBreaker` struct

**Required Changes:**
- Import `CircuitBreakerState` from `super::circuit_breaker` for type consistency
- Add atomic state field using 0=Closed, 1=Open, 2=HalfOpen encoding
- Implement state transition logic based on `failure_threshold` and `recovery_timeout`
- Implement timeout-based recovery using `recovery_timeout` for Open → HalfOpen transitions
- Ensure thread-safe state management using atomic operations with Relaxed ordering
- Add methods: `record_failure()`, `record_success()`, `should_allow_request()`
- Integrate state management with existing `ErrorCounter` for comprehensive statistics

**Why:** Circuit breakers need state management to prevent cascading failures in distributed systems.

## SUBTASK 3: Integrate Circuit Breaker with Error Handling

**Location:** `packages/candle/src/domain/error/breaker.rs`

**Required Changes:**
- Modify `execute()` method to check `should_allow_request()` before operation execution
- Call `record_success()` on successful operations, `record_failure()` on failures
- Ensure circuit state changes are reflected in error responses
- Maintain existing integration with `ErrorCounter` for statistics tracking
- Document circuit breaker behavior in module-level docs

**Why:** The implementation must be wired into actual error paths to be effective in production.

## DEFINITION OF DONE

- [ ] No `#[allow(dead_code)]` attributes remain in breaker.rs
- [ ] Circuit breaker actively uses `failure_threshold` and `recovery_timeout`
- [ ] State transitions work correctly (closed → open → half-open → closed)
- [ ] Thread-safe implementation using atomic operations
- [ ] Module documentation explains circuit breaker usage
- [ ] `record_failure()`, `record_success()`, `should_allow_request()` methods implemented
- [ ] `execute()` method uses new state management logic

## RESEARCH NOTES

### Circuit Breaker Pattern Analysis

The circuit breaker pattern prevents cascading failures in distributed systems by monitoring operation success/failure rates and temporarily halting requests when failure thresholds are exceeded. Key states:

- **Closed**: Normal operation, requests allowed, failures counted
- **Open**: Circuit "tripped", requests blocked, timeout-based recovery attempted  
- **Half-Open**: Testing phase, limited requests allowed to check service recovery

### Implementation Approach

Analysis of existing code shows:
- Generic `CircuitBreaker` in `circuit_breaker.rs` provides reference implementation
- `ErrorCircuitBreaker` needs domain-specific state management using stored fields
- Thread-safety achieved via `AtomicU64` with `Ordering::Relaxed` (sufficient for this use case)
- Integration with `ErrorCounter` provides comprehensive failure statistics

### Core Patterns Demonstrated

**State Management:**
```rust
// Atomic state encoding
const CLOSED: u64 = 0;
const OPEN: u64 = 1; 
const HALF_OPEN: u64 = 2;

// Thread-safe state check
pub fn should_allow_request(&self) -> bool {
    match self.state.load(Ordering::Relaxed) {
        CLOSED => true,
        OPEN => {
            // Timeout-based recovery logic
            let now = duration_to_millis_u64(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default());
            let last_failure = self.last_failure_time.load(Ordering::Relaxed);
            if now - last_failure > self.recovery_timeout_ms() {
                self.state.store(HALF_OPEN, Ordering::Relaxed);
                true
            } else {
                false
            }
        }
        HALF_OPEN => true,
        _ => true,
    }
}
```

**Failure Tracking:**
```rust
pub fn record_failure(&self) {
    let failures = self.failure_count.fetch_add(1, Ordering::Relaxed) + 1;
    if failures >= self.failure_threshold as u64 {
        self.state.store(OPEN, Ordering::Relaxed);
        let now = duration_to_millis_u64(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default());
        self.last_failure_time.store(now, Ordering::Relaxed);
    }
}
```

**Success Recovery:**
```rust
pub fn record_success(&self) {
    self.failure_count.store(0, Ordering::Relaxed);
    self.state.store(CLOSED, Ordering::Relaxed);
}
```

### Integration Points

- `execute()` method orchestrates state checks and recording
- `ErrorCounter` provides detailed failure categorization
- Atomic operations ensure thread-safety without locks
- Duration utilities from `crate::domain::util` handle time conversions

## SOURCE CITATIONS

- [Circuit Breaker Reference Implementation](./src/domain/error/circuit_breaker.rs) - Generic circuit breaker with state management patterns
- [Error Counter Implementation](./src/domain/error/breaker.rs) - Statistics tracking integration
- [Duration Utilities](./src/domain/util.rs) - Time conversion functions used for timeout calculations

## CONSTRAINTS

- Maintain compatibility with existing error handling
- Use existing patterns from codebase (atomic operations, zero-allocation)
- Follow thread-safety patterns established in `ErrorCounter` and other components