# CIRCUIT_1: Implement Circuit Breaker System

## OBJECTIVE

Implement the circuit breaker monitoring system to eliminate dead code and provide proper error tracking for failure thresholds and recovery timeouts.

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
- Implement methods that utilize `failure_threshold` to track consecutive failures
- Implement methods that utilize `recovery_timeout` to manage half-open state transitions
- Add circuit state transitions (closed → open → half-open → closed)
- Integrate with existing `ErrorCounter` to track failures against threshold

**Why:** The circuit breaker pattern requires these fields to function. Dead code indicates incomplete implementation.

## SUBTASK 2: Create Circuit State Management

**Location:** Same file, enhance `CircuitBreaker` struct

**Required Changes:**
- Add circuit state enum (Closed, Open, HalfOpen)
- Implement state transition logic based on `failure_threshold`
- Implement timeout-based recovery using `recovery_timeout`
- Add methods: `record_failure()`, `record_success()`, `should_allow_request()`
- Ensure thread-safe state management using atomic operations

**Why:** Circuit breakers need state management to prevent cascading failures.

## SUBTASK 3: Integrate Circuit Breaker with Error Handling

**Location:** `packages/candle/src/domain/error/breaker.rs`

**Required Changes:**
- Connect circuit breaker to existing error handling paths
- Add circuit breaker check before operations
- Emit circuit state change events for monitoring
- Document usage patterns in module-level docs

**Why:** The implementation must be wired into actual error paths to be effective.

## DEFINITION OF DONE

- [ ] No `#[allow(dead_code)]` attributes remain in breaker.rs
- [ ] Circuit breaker actively uses `failure_threshold` and `recovery_timeout`
- [ ] State transitions work correctly (closed → open → half-open → closed)
- [ ] Thread-safe implementation using atomic operations
- [ ] Module documentation explains circuit breaker usage
- [ ] NO test code written (separate team responsibility)
- [ ] NO benchmark code written (separate team responsibility)

## RESEARCH NOTES

### Circuit Breaker Pattern References
- Standard pattern: track consecutive failures, open circuit after threshold
- Half-open state: allow limited requests to test recovery
- Timeout-based recovery: transition to half-open after recovery_timeout

### Integration Points
- ErrorCounter already exists for tracking failures
- Need to integrate with completion providers and tool execution
- Consider using `AtomicUsize` for state management (existing pattern in codebase)

## CONSTRAINTS

- DO NOT write unit tests (separate team handles testing)
- DO NOT write benchmarks (separate team handles benchmarks)
- Maintain compatibility with existing error handling
- Use existing patterns from codebase (atomic operations, zero-allocation)
