# DECOMP_029: Decompose `events.rs`

**File:** `packages/candle/src/domain/chat/commands/types/events.rs`  
**Current Size:** 762 lines  
**Module Area:** domain / chat / commands / types

## OBJECTIVE

Decompose the monolithic `events.rs` (762 lines) into smaller, focused, maintainable modules using the proven subdirectory pattern already established in this codebase. All existing functionality must be preserved exactly as-is.

## CONSTRAINTS

- **NO TESTS:** Do not write any unit tests, integration tests, or test code.
- **NO BENCHMARKS:** Do not write any benchmark code.
- **NO DOCUMENTATION:** Do not create separate documentation files. Module-level doc comments are sufficient.
- **MAINTAIN FUNCTIONALITY:** All existing functionality must be preserved exactly as-is.
- **SINGLE SESSION:** This task must be completable in one focused Claude session.

## CURRENT STRUCTURE ANALYSIS

The `events.rs` file (763 lines total) contains five distinct logical components:

### Component 1: CommandExecutionContext (Lines ~10-155, ~145 lines)
```rust
pub struct CommandExecutionContext {
    pub execution_id: u64,
    pub command_name: String,
    pub start_time: u64,
    pub resource_usage: ResourceUsage,
    // ... other fields
    execution_counter: AtomicU64,
    event_counter: AtomicUsize,
}
```

**Responsibilities:**
- Execution context tracking with owned strings
- Builder pattern methods (`with_user_id`, `with_session_id`, etc.)
- Timing methods (`elapsed_time_us`, `is_timed_out`, `remaining_time_us`)
- Atomic counter management (`next_execution_id`, `next_event_id`)

### Component 2: CommandEvent Enum (Lines ~156-420, ~265 lines)
```rust
pub enum CommandEvent {
    Started { command, execution_id, timestamp_us },
    Progress { execution_id, progress, message, timestamp },
    Output { execution_id, content, output_type, timestamp_us },
    Completed { execution_id, result, duration_us, resource_usage, timestamp_us },
    Failed { execution_id, error, error_code, duration_us, resource_usage, timestamp_us },
    Cancelled { execution_id, reason, duration_us, resource_usage, timestamp_us },
    Warning { execution_id, message, severity, timestamp_us },
    ResourceAlert { execution_id, resource_type, current_value, threshold_value, timestamp_us },
}
```

**Responsibilities:**
- 8 event type variants for streaming command execution
- Zero-allocation constructor methods for each variant
- Accessor methods (`execution_id`, `timestamp_us`, `is_terminal`, `is_success`, etc.)
- Event classification methods (`severity`)

### Component 3: StreamingCommandExecutor (Lines ~421-650, ~230 lines)
```rust
pub struct StreamingCommandExecutor {
    execution_counter: AtomicU64,
    active_executions: AtomicUsize,
    total_executions: AtomicU64,
    successful_executions: AtomicU64,
    failed_executions: AtomicU64,
    event_sender: Option<tokio::sync::mpsc::UnboundedSender<CommandEvent>>,
}
```

**Responsibilities:**
- Atomic execution state tracking
- Execution lifecycle management (`start_execution`, `complete_execution`, `fail_execution`, `cancel_execution`)
- Event streaming (`send_progress`, `send_output`, `send_warning`)
- Statistics access (`stats`, `peek_next_execution_id`)

### Component 4: CommandExecutorStats (Lines ~651-710, ~60 lines)
```rust
pub struct CommandExecutorStats {
    pub active_executions: u64,
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
}
```

**Responsibilities:**
- Execution statistics with zero-allocation accessors
- Rate calculations (`success_rate`, `failure_rate`, `completion_rate`)
- State checks (`has_active_executions`, `is_idle`)

### Component 5: Trait Implementations (Lines ~711-762, ~50 lines)
```rust
impl Debug for StreamingCommandExecutor { ... }
impl MessageChunk for CommandEvent { ... }
impl Default for CommandEvent { ... }
```

**Responsibilities:**
- Debug formatting for StreamingCommandExecutor
- MessageChunk trait implementation for CommandEvent
- Default trait implementation for CommandEvent

## PUBLIC API USAGE ANALYSIS

Based on codebase search, the following types are used externally:

### Used in `action_executor_impls.rs`
```rust
use super::{
    CommandExecutionContext,
    // ... other imports
};
```

### Used in `commands/mod.rs`
```rust
pub fn execute_candle_command_streaming(command: ImmutableChatCommand) -> impl Stream<Item = CommandEvent> {
    // Uses CommandEvent in return type
}

pub async fn initialize_candle_command_executor(context: CommandExecutionContext) {
    // Uses CommandExecutionContext as parameter
}
```

### Current Public Exports (from `types/mod.rs`)
```rust
pub use self::{
    events::*,
    // ... other re-exports
};
```

All items from `events` module are re-exported at the parent level, meaning external code can access them as:
- `types::CommandEvent`
- `types::CommandExecutionContext`
- `types::StreamingCommandExecutor`
- `types::CommandExecutorStats`

## PROVEN DECOMPOSITION PATTERN

The codebase already uses a proven pattern for decomposing large files. See [./packages/candle/src/domain/chat/commands/types/code_execution/](../packages/candle/src/domain/chat/commands/types/code_execution/):

**Structure:**
```
code_execution/
├── mod.rs           # Module declarations and re-exports
├── errors.rs        # ValidationError types
├── language.rs      # CodeLanguage enum
├── limits.rs        # ResourceLimits struct
├── request.rs       # CodeExecutionRequest struct
├── response.rs      # CodeExecutionResult, ExecutionError, ExecutionStatus
├── tool.rs          # CodeExecutionTool struct
└── validation.rs    # ValidationConfig struct
```

**mod.rs Pattern:**
```rust
// Module declarations
pub mod errors;
pub mod language;
pub mod limits;
pub mod request;
pub mod response;
pub mod tool;
pub mod validation;

// Re-export all public types for backward compatibility
pub use errors::ValidationError;
pub use language::CodeLanguage;
pub use limits::ResourceLimits;
pub use request::CodeExecutionRequest;
pub use response::{CodeExecutionResult, ExecutionError, ExecutionStatus, OutputFormat};
pub use tool::CodeExecutionTool;
pub use validation::ValidationConfig;
```

This pattern:
1. ✅ Maintains backward compatibility via re-exports
2. ✅ Creates focused, single-responsibility modules
3. ✅ Keeps each module under 300 lines
4. ✅ Preserves the public API contract
5. ✅ Provides clear module boundaries

## RECOMMENDED DECOMPOSITION PLAN

Create an `events/` subdirectory following the proven pattern:

```
events/
├── mod.rs           # Module declarations and re-exports (~30 lines)
├── context.rs       # CommandExecutionContext struct (~150 lines)
├── event_types.rs   # CommandEvent enum and methods (~270 lines)
├── executor.rs      # StreamingCommandExecutor struct (~230 lines)
├── stats.rs         # CommandExecutorStats struct (~60 lines)
└── impls.rs         # Trait implementations (~50 lines)
```

### Module: `events/context.rs` (~150 lines)
**Contents:**
- `CommandExecutionContext` struct definition
- All builder pattern methods (`with_user_id`, `with_session_id`, `with_priority`, etc.)
- Timing methods (`elapsed_time_us`, `is_timed_out`, `remaining_time_us`)
- Atomic counter methods (`next_execution_id`, `next_event_id`)

**Imports needed:**
```rust
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use super::super::metadata::ResourceUsage;
use crate::domain::util::unix_timestamp_micros;
```

### Module: `events/event_types.rs` (~270 lines)
**Contents:**
- `CommandEvent` enum with all 8 variants
- Constructor methods (`started`, `progress`, `output`, `completed`, `failed`, `cancelled`, `warning`, `resource_alert`)
- Accessor methods (`execution_id`, `timestamp_us`, `is_terminal`, `is_success`, `is_failure`, `is_cancelled`, `severity`)
- Helper method (`current_timestamp_us`)

**Imports needed:**
```rust
use serde::{Deserialize, Serialize};
use super::super::commands::{CommandExecutionResult, ImmutableChatCommand, OutputType};
use super::super::metadata::ResourceUsage;
use crate::domain::util::unix_timestamp_micros;
```

### Module: `events/executor.rs` (~230 lines)
**Contents:**
- `StreamingCommandExecutor` struct definition
- Constructor methods (`new`, `with_event_sender`)
- Execution lifecycle methods (`start_execution`, `complete_execution`, `fail_execution`, `cancel_execution`)
- Event streaming methods (`send_progress`, `send_output`, `send_warning`)
- Statistics methods (`stats`, `peek_next_execution_id`)
- `Default` trait implementation

**Imports needed:**
```rust
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use super::event_types::CommandEvent;
use super::stats::CommandExecutorStats;
use super::super::commands::{CommandExecutionResult, ImmutableChatCommand, OutputType};
use super::super::metadata::ResourceUsage;
```

### Module: `events/stats.rs` (~60 lines)
**Contents:**
- `CommandExecutorStats` struct definition
- Calculation methods (`success_rate`, `failure_rate`, `completion_rate`)
- State check methods (`has_active_executions`, `is_idle`)
- Derive macros

**Imports needed:**
```rust
use serde::{Deserialize, Serialize};
```

### Module: `events/impls.rs` (~50 lines)
**Contents:**
- `Debug` implementation for `StreamingCommandExecutor`
- `MessageChunk` implementation for `CommandEvent`
- `Default` implementation for `CommandEvent`

**Imports needed:**
```rust
use std::sync::atomic::Ordering;
use cyrup_sugars::prelude::MessageChunk;
use super::event_types::CommandEvent;
use super::executor::StreamingCommandExecutor;
use super::super::commands::ImmutableChatCommand;
use super::super::metadata::ResourceUsage;
use crate::domain::util::unix_timestamp_micros;
```

### Module: `events/mod.rs` (~30 lines)
**Template:**
```rust
//! Command execution events and context tracking with zero allocation patterns
//!
//! Provides blazing-fast event streaming and execution context management
//! with owned strings allocated once for maximum performance. No Arc usage, no locking.

// Module declarations
pub mod context;
pub mod event_types;
pub mod executor;
pub mod stats;
pub mod impls;

// Re-export all public types for backward compatibility
pub use context::CommandExecutionContext;
pub use event_types::CommandEvent;
pub use executor::StreamingCommandExecutor;
pub use stats::CommandExecutorStats;
```

## STEP-BY-STEP IMPLEMENTATION INSTRUCTIONS

### Step 1: Create the events/ subdirectory
```bash
mkdir -p packages/candle/src/domain/chat/commands/types/events
```

### Step 2: Create context.rs
**Location:** `packages/candle/src/domain/chat/commands/types/events/context.rs`

**Extract from events.rs:**
- Lines 1-6: Module doc comment (adapt for context module)
- Lines 7-9: Import statements (adjust paths with `super::super::`)
- Lines 10-155: CommandExecutionContext struct and all its implementations

**Key changes:**
- Change `use super::metadata::ResourceUsage` to `use super::super::metadata::ResourceUsage`
- Change `use super::commands::` to `use super::super::commands::`

### Step 3: Create event_types.rs
**Location:** `packages/candle/src/domain/chat/commands/types/events/event_types.rs`

**Extract from events.rs:**
- Lines 156-420: CommandEvent enum and all its methods
- Adjust import paths (use `super::super::` pattern)

**Key changes:**
- Add `use super::super::commands::{CommandExecutionResult, ImmutableChatCommand, OutputType}`
- Add `use super::super::metadata::ResourceUsage`

### Step 4: Create executor.rs
**Location:** `packages/candle/src/domain/chat/commands/types/events/executor.rs`

**Extract from events.rs:**
- Lines 421-650: StreamingCommandExecutor struct and implementations
- Lines 651-663: Default trait implementation (move to this file for proximity)

**Key changes:**
- Add `use super::event_types::CommandEvent`
- Add `use super::stats::CommandExecutorStats`
- Add `use super::super::commands::{CommandExecutionResult, ImmutableChatCommand, OutputType}`

### Step 5: Create stats.rs
**Location:** `packages/candle/src/domain/chat/commands/types/events/stats.rs`

**Extract from events.rs:**
- Lines 651-710: CommandExecutorStats struct and all its methods

**Key changes:**
- Minimal imports needed (only serde)

### Step 6: Create impls.rs
**Location:** `packages/candle/src/domain/chat/commands/types/events/impls.rs`

**Extract from events.rs:**
- Lines 711-740: Debug implementation for StreamingCommandExecutor
- Lines 742-757: MessageChunk implementation for CommandEvent
- Lines 759-762: Default implementation for CommandEvent

**Key changes:**
- Add `use super::event_types::CommandEvent`
- Add `use super::executor::StreamingCommandExecutor`

### Step 7: Create mod.rs
**Location:** `packages/candle/src/domain/chat/commands/types/events/mod.rs`

Use the template provided above in the "Module: events/mod.rs" section.

### Step 8: Update parent mod.rs
**Location:** `packages/candle/src/domain/chat/commands/types/mod.rs`

**No changes needed!** The line `pub use self::events::*;` will automatically re-export everything from the new `events/mod.rs`, which in turn re-exports from the submodules.

### Step 9: Delete the old file
```bash
rm packages/candle/src/domain/chat/commands/types/events.rs
```

### Step 10: Verify compilation
```bash
cargo check
```

Fix any import path issues that arise.

## DEFINITION OF DONE

- [x] `events/` subdirectory created
- [x] Five focused modules created (context.rs, event_types.rs, executor.rs, stats.rs, impls.rs)
- [x] Each module is < 300 lines
- [x] `events/mod.rs` declares modules and re-exports all public types
- [x] Old `events.rs` file deleted
- [x] Public API remains unchanged (all types accessible via `types::` module)
- [x] `cargo check` passes without errors
- [x] All functionality preserved exactly as-is

## CRITICAL SUCCESS FACTORS

1. **Import Path Adjustments**: When moving code into subdirectory, all `use super::` references must become `use super::super::`
   
2. **Trait Implementations**: Keep trait implementations separate in `impls.rs` for clean organization

3. **Re-exports**: The `events/mod.rs` must re-export ALL public types to maintain backward compatibility

4. **Atomic Operations**: Ensure all atomic counter operations remain thread-safe and use correct `Ordering`

5. **Zero Dependencies Between Submodules**: Minimize cross-dependencies:
   - `context.rs` is independent
   - `event_types.rs` is independent  
   - `executor.rs` depends on `event_types` and `stats`
   - `stats.rs` is independent
   - `impls.rs` depends on `event_types` and `executor`

## REFERENCES

- Current file: [./packages/candle/src/domain/chat/commands/types/events.rs](../packages/candle/src/domain/chat/commands/types/events.rs)
- Proven pattern: [./packages/candle/src/domain/chat/commands/types/code_execution/](../packages/candle/src/domain/chat/commands/types/code_execution/)
- Parent module: [./packages/candle/src/domain/chat/commands/types/mod.rs](../packages/candle/src/domain/chat/commands/types/mod.rs)
- Usage example: [./packages/candle/src/domain/chat/commands/types/action_executor_impls.rs](../packages/candle/src/domain/chat/commands/types/action_executor_impls.rs)
