# COMMAND_1: Implement Command Execution Duration Tracking

## OBJECTIVE

Replace the stubbed duration tracking (currently hardcoded to 0) with actual timing measurements for command execution.

## BACKGROUND

Command execution results include a `duration_us` field that's hardcoded to 0 with a TODO comment. This prevents accurate performance monitoring and debugging of slow commands.

## SUBTASK 1: Add Timing Infrastructure

**Location:** `packages/candle/src/domain/chat/commands/execution.rs:819`

**Current State:**
```rust
duration_us: 0, // TODO: Calculate actual duration
```

**Required Changes:**
- Import timing utilities (use `std::time::Instant`)
- Create timing helper for microsecond precision
- Add timing fields to execution context
- Use existing `current_timestamp_us()` pattern from same file

**Why:** Accurate timing is essential for performance monitoring and SLA tracking.

## SUBTASK 2: Implement Execution Timing

**Location:** `packages/candle/src/domain/chat/commands/execution.rs`

**Required Changes:**
- Start timer at command execution begin
- Stop timer at command completion
- Calculate duration in microseconds
- Store duration in `ExecutionResult`
- Handle timing for both success and error cases

**Why:** Both successful and failed executions need timing data.

## SUBTASK 3: Add Timing to All Execution Paths

**Location:** Command execution paths in same file

**Required Changes:**
- Add timing to synchronous command execution
- Add timing to async command execution  
- Add timing to streaming command execution
- Ensure timing captures full execution lifecycle
- Include any pre/post processing time

**Why:** All execution paths must be timed for complete monitoring.

## SUBTASK 4: Expose Duration Metrics

**Location:** `packages/candle/src/domain/chat/commands/execution.rs`

**Required Changes:**
- Ensure `duration_us` is properly serialized in results
- Add duration to execution logs
- Include duration in error reports
- Document duration field in `ExecutionResult` docs

**Why:** Duration data must be accessible for monitoring and debugging.

## DEFINITION OF DONE

- [ ] No hardcoded `duration_us: 0` in code
- [ ] Actual execution time measured in microseconds
- [ ] Timing includes full execution lifecycle
- [ ] Timing works for sync, async, and streaming commands
- [ ] Duration exposed in results and logs
- [ ] Documentation updated to explain timing
- [ ] NO test code written (separate team responsibility)
- [ ] NO benchmark code written (separate team responsibility)

## RESEARCH NOTES

### Timing Pattern in Codebase
- Function `current_timestamp_us()` exists in same file
- Use `std::time::Instant` for high-precision timing
- Calculate microseconds: `instant.elapsed().as_micros() as u64`

### Execution Paths to Time
- Synchronous tool execution
- Async tool execution with futures
- Streaming tool execution with chunks
- Error paths (partial execution)

### Integration Points
- `ExecutionResult` struct contains duration field
- `ResourceUsage` also tracked (existing pattern)
- Logging and monitoring systems consume duration

## CONSTRAINTS

- DO NOT write unit tests (separate team handles testing)
- DO NOT write benchmarks (separate team handles benchmarks)
- Maintain microsecond precision (don't downgrade to milliseconds)
- Ensure minimal overhead from timing code itself
