# COMMAND_1: Implement Command Execution Duration Tracking

## OBJECTIVE

Replace the stubbed duration tracking (currently hardcoded to 0) with actual timing measurements for command execution.

## BACKGROUND

Command execution results include a `duration_us` field that's hardcoded to 0 with a TODO comment. This prevents accurate performance monitoring and debugging of slow commands.

## SUBTASK 1: Fix parse_and_execute Method

**Location:** `packages/candle/src/domain/chat/commands/execution.rs:809`

**Current State:**
```rust
pub fn parse_and_execute(&self, input: &str) -> Pin<Box<dyn Stream<Item = CommandEvent> + Send>> {
    let execution_id = self.execution_counter.fetch_add(1, Ordering::AcqRel);
    let command_result = self.parser.parse_command(input);

    Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
        match command_result {
            Ok(command) => {
                // Emit Started event
                let _ = sender.send(CommandEvent::Started {
                    command: command.clone(),
                    execution_id,
                    timestamp_us: current_timestamp_us(),
                });

                // Emit successful Output event
                let _ = sender.send(CommandEvent::Output {
                    execution_id,
                    content: "Command executed successfully".to_string(),
                    output_type: OutputType::Text,
                    timestamp_us: current_timestamp_us(),
                });

                // Emit Completed event
                let _ = sender.send(CommandEvent::Completed {
                    execution_id,
                    result: CommandExecutionResult::Success(
                        "Command completed".to_string()
                    ),
                    duration_us: 0, // TODO: Calculate actual duration
                    resource_usage: ResourceUsage::default(),
                    timestamp_us: current_timestamp_us(),
                });
            }
            Err(e) => {
                // Emit Failed event for parse errors
                let _ = sender.send(CommandEvent::Failed {
                    execution_id,
                    error: format!("Parse error: {e}"),
                    error_code: 1001, // Parse error code
                    duration_us: 0,
                    resource_usage: ResourceUsage::default(),
                    timestamp_us: current_timestamp_us(),
                });
            }
        }
    }))
}
```

**Required Changes:**
- Add timing start: `let start_time = Instant::now();` after execution_id generation
- For successful parsing: delegate to `self.execute_streaming(execution_id, command)` instead of emitting stub events
- For parse errors: calculate `duration_us = start_time.elapsed().as_micros() as u64` and use in Failed event

**Why:** The `execute_streaming` method already implements comprehensive timing for all command execution paths using the established `Instant` pattern.

## SUBTASK 2: Verify Timing Integration

**Location:** Existing `execute_streaming` method in same file

**Current State:** Already implemented with proper timing:
```rust
let start_time = Instant::now();
// ... command execution logic ...
let duration_us = start_time.elapsed().as_micros().min(u128::from(u64::MAX)) as u64;
let _ = sender.send(CommandEvent::completed(execution_id, result, duration_us, resource_usage));
```

**Required Changes:** None - timing is already correct and comprehensive.

**Why:** All execution paths (sync, async, streaming) are already timed in `execute_streaming`.

## SUBTASK 3: Update Error Timing

**Location:** Parse error handling in `parse_and_execute`

**Required Changes:**
- Replace `duration_us: 0` with calculated elapsed time for parse errors
- Use same `Instant` pattern as successful execution

**Why:** Parse errors should also report timing for debugging slow parsing issues.

## SUBTASK 4: Ensure Duration Exposure

**Location:** `CommandEvent` serialization and logging

**Required Changes:** None - duration is already exposed in `Completed` and `Failed` events.

**Why:** Existing event structure properly serializes duration_us field.

## DEFINITION OF DONE

- [ ] No hardcoded `duration_us: 0` in `parse_and_execute` method
- [ ] Successful command execution uses `execute_streaming` timing
- [ ] Parse errors include actual timing measurements
- [ ] Timing includes full execution lifecycle (handled by `execute_streaming`)
- [ ] Timing works for sync, async, and streaming commands (handled by `execute_streaming`)
- [ ] Duration exposed in results and logs (existing `CommandEvent` structure)

## RESEARCH NOTES

### Timing Pattern in Codebase
- Function `current_timestamp_us()` exists in same file: `packages/candle/src/domain/chat/commands/execution.rs:24`
- Use `std::time::Instant` for high-precision timing: `packages/candle/src/domain/chat/commands/execution.rs:8`
- Calculate microseconds: `instant.elapsed().as_micros() as u64`
- Pattern established in `execute_streaming`: `packages/candle/src/domain/chat/commands/execution.rs:108`

### Execution Paths to Time
- Synchronous command execution: handled by `execute_streaming` match statement
- Async tool execution: handled by `execute_streaming` with futures
- Streaming tool execution: handled by `execute_streaming` with chunks
- Error paths: both parse errors (new timing) and execution errors (existing timing)
- Parse timing: added to `parse_and_execute` for errors

### Integration Points
- `CommandEvent::Completed` and `CommandEvent::Failed` contain duration field: `packages/candle/src/domain/chat/commands/types/events/event_types.rs:51`
- `ResourceUsage` also tracked with duration methods: `packages/candle/src/domain/chat/commands/types/metadata.rs:298`
- Logging and monitoring systems consume duration from events

## CONSTRAINTS

- Maintain microsecond precision (don't downgrade to milliseconds)
- Ensure minimal overhead from timing code itself
- Use existing `Instant` infrastructure already imported

## IMPLEMENTATION PATTERN

```rust
// In parse_and_execute
let start_time = Instant::now();
match self.parser.parse_command(input) {
    Ok(command) => {
        // Delegate to existing timed execution
        self.execute_streaming(execution_id, command)
    }
    Err(e) => {
        // Calculate timing for parse errors
        let duration_us = start_time.elapsed().as_micros() as u64;
        // Emit Failed event with actual duration_us
    }
}
```