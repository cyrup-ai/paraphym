# DECOMP_020: Decompose `parsing.rs`

**File:** `packages/candle/src/domain/chat/commands/parsing.rs`  
**Current Size:** 894 lines  
**Module Area:** domain / chat / commands

## OBJECTIVE

Decompose the monolithic `parsing.rs` (894 lines) into 4 focused, maintainable modules while preserving all existing functionality.

## CONSTRAINTS

- **NO TESTS:** Do not write any unit tests, integration tests, or test code.
- **NO BENCHMARKS:** Do not write any benchmark code.
- **NO DOCUMENTATION:** Do not add extensive documentation beyond existing module-level comments.
- **MAINTAIN FUNCTIONALITY:** All existing functionality must be preserved exactly as-is.
- **SINGLE SESSION:** This task must be completable in one focused session.

## RESEARCH FINDINGS

### Current File Structure Analysis

The 894-line `parsing.rs` contains these logical sections:

1. **Error Types** (lines 15-63, ~60 lines)
   - `ParseError` enum with 5 variants
   - `ParseResult<T>` type alias
   - Self-contained error handling

2. **Parser Struct** (lines 64-90, ~25 lines)
   - `CommandParser` struct with 3 fields
   - HashMap for commands, aliases
   - Vector for command history

3. **Main Parsing Logic** (lines 91-237, ~145 lines)
   - `parse_command()` - inline parsing with match statements
   - Returns `Result<ImmutableChatCommand, CandleCommandError>`
   - Currently used by [`execution.rs`](../execution.rs)

4. **Command Registration** (lines 238-567, ~330 lines)
   - `register_builtin_commands()` method
   - Verbose `CommandInfo` definitions for 5 commands: help, clear, export, config, search
   - Data-heavy but logically separate

5. **Alternative Parsing API** (lines 548-790, ~240 lines)
   - `parse()` method (alternative to `parse_command()`)
   - Individual `parse_*` helper functions for each command
   - Returns `Result<ImmutableChatCommand, ParseError>`
   - Cleaner error handling but currently unused

6. **Utility Methods** (lines 869-894, ~120 lines)
   - `validate_command()`
   - `get_suggestions()`, `get_command_info()`
   - `get_history()`, `add_to_history()`

### Dependencies

**Imports Used:**
- `std::collections::HashMap`
- `thiserror::Error`
- `super::types::{CandleCommandError, CommandInfo, ImmutableChatCommand, ParameterInfo, ParameterType, SearchScope, StabilityLevel}`

**External Usage:**
- [`mod.rs`](./mod.rs) re-exports: `pub use parsing::{CommandParser, ParseError, ParseResult}`
- [`execution.rs`](./execution.rs) imports and uses `CommandParser` as a struct field
- [`execution.rs`](./execution.rs#L628) calls `parser.parse_command(input)` method

**Public API Surface:**
- `CommandParser` struct (Debug, Clone)
- `ParseError` enum (Error, Debug, Clone)
- `ParseResult<T>` type alias
- All public methods on `CommandParser`

## DECOMPOSITION DESIGN

### Target Module Structure

Transform single file into directory structure:

```
packages/candle/src/domain/chat/commands/
├── parsing/                    # NEW DIRECTORY
│   ├── mod.rs                 # Main parser (~200 lines)
│   ├── errors.rs              # Error types (~60 lines)
│   ├── command_parsers.rs     # Alternative parsing API (~290 lines)
│   └── builtin_commands.rs    # Command metadata (~340 lines)
└── parsing.rs                 # DELETE THIS FILE
```

### Module Responsibilities

#### 1. `errors.rs` (~60 lines)

**Purpose:** Command parsing error types

**Contains:**
- `ParseError` enum with all 5 variants
- `ParseResult<T>` type alias
- Doc comments from original

**Imports:**
```rust
use thiserror::Error;
```

**Exports:**
- All public (will be re-exported by mod.rs)

**Code Pattern:**
```rust
//! Command parsing errors

use thiserror::Error;

/// Command parsing errors with owned strings
#[derive(Error, Debug, Clone)]
pub enum ParseError {
    // ... all variants from original file lines 15-58
}

pub type ParseResult<T> = Result<T, ParseError>;
```

#### 2. `command_parsers.rs` (~290 lines)

**Purpose:** Alternative parsing API with individual command parsers

**Contains:**
- `parse()` method (alternative to `parse_command()`)
- `parse_help_command()`
- `parse_clear_command()`
- `parse_export_command()`
- `parse_config_command()`
- `parse_search_args()`

**Imports:**
```rust
use super::errors::{ParseError, ParseResult};
use super::super::types::{ImmutableChatCommand, SearchScope};
```

**Visibility:**
- Functions marked `pub(super)` to be used only by mod.rs
- Not part of public API but available to parent module

**Code Pattern:**
```rust
//! Individual command parsing functions (alternative API)

use super::errors::{ParseError, ParseResult};
use super::super::types::{ImmutableChatCommand, SearchScope};

/// Parse command using modular approach (alternative to parse_command)
pub(super) fn parse(input: &str, aliases: &HashMap<String, String>) -> ParseResult<ImmutableChatCommand> {
    // Lines 548-590 from original
}

/// Parse help command
pub(super) fn parse_help_command(args: &[&str]) -> ParseResult<ImmutableChatCommand> {
    // Lines 592-615 from original
}

// ... other parse_* functions
```

#### 3. `builtin_commands.rs` (~340 lines)

**Purpose:** Built-in command definitions and registration

**Contains:**
- `register_builtin_commands()` as standalone function
- All `CommandInfo` definitions for built-in commands

**Imports:**
```rust
use std::collections::HashMap;
use super::super::types::{CommandInfo, ParameterInfo, ParameterType, StabilityLevel};
```

**Visibility:**
- `pub(super)` - internal to parsing module

**Refactoring Note:**
Original is a method on `impl CommandParser`. Extract the method body into a standalone function:

```rust
//! Built-in command definitions and registration

use std::collections::HashMap;
use super::super::types::{CommandInfo, ParameterInfo, ParameterType, StabilityLevel};

/// Register all built-in commands
pub(super) fn register_builtin_commands(commands: &mut HashMap<String, CommandInfo>) {
    // Help command
    commands.insert("help".to_string(), CommandInfo {
        // Lines 242-318 from original
    });
    
    // Clear command
    commands.insert("clear".to_string(), CommandInfo {
        // Lines 320-381 from original
    });
    
    // ... other commands
}
```

#### 4. `mod.rs` (~200 lines)

**Purpose:** Main parser orchestration and public API

**Contains:**
- Module declarations (`mod errors`, `mod command_parsers`, `mod builtin_commands`)
- Public re-exports
- `CommandParser` struct definition
- `impl Default for CommandParser`
- `impl CommandParser` with:
  - `new()`
  - `parse_command()` (main parsing method used by execution.rs)
  - `register_command()`
  - `validate_command()`
  - `get_suggestions()`, `get_command_info()`
  - `get_history()`, `add_to_history()`

**Code Pattern:**
```rust
//! Command parsing and validation logic
//!
//! Provides zero-allocation command parsing with comprehensive validation and error handling.
//! Uses blazing-fast parsing algorithms with ergonomic APIs and production-ready error messages.

mod errors;
mod command_parsers;
mod builtin_commands;

// Public re-exports
pub use errors::{ParseError, ParseResult};

use std::collections::HashMap;
use super::types::{
    CandleCommandError, CommandInfo, ImmutableChatCommand, ParameterInfo, ParameterType,
    SearchScope, StabilityLevel,
};

/// Zero-allocation command parser with owned strings
#[derive(Debug, Clone)]
pub struct CommandParser {
    /// Registered commands
    commands: HashMap<String, CommandInfo>,
    /// Command aliases
    aliases: HashMap<String, String>,
    /// Command history for auto-completion
    history: Vec<String>,
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandParser {
    /// Create a new command parser
    #[must_use]
    pub fn new() -> Self {
        let mut parser = Self {
            commands: HashMap::new(),
            aliases: HashMap::new(),
            history: Vec::new(),
        };
        builtin_commands::register_builtin_commands(&mut parser.commands);
        parser
    }

    /// Parse command from input string (zero-allocation)
    pub fn parse_command(&self, input: &str) -> Result<ImmutableChatCommand, CandleCommandError> {
        // Lines 98-236 from original - inline parsing logic
    }

    // ... rest of impl methods
}
```

## IMPLEMENTATION STEPS

### STEP 1: Create Directory Structure

```bash
mkdir -p /Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/commands/parsing
```

### STEP 2: Create `errors.rs`

Extract lines 15-63 from original [`parsing.rs`](../../../packages/candle/src/domain/chat/commands/parsing.rs):
- File header comment
- `ParseError` enum with all variants
- `ParseResult<T>` type alias

### STEP 3: Create `command_parsers.rs`

Extract lines 548-790 from original:
- Module doc comment
- `parse()` method
- All `parse_*` helper functions
- Update function signatures from `Self::` to standalone
- Mark all as `pub(super)`

### STEP 4: Create `builtin_commands.rs`

Extract lines 238-567 from original:
- Refactor `register_builtin_commands()` from impl method to standalone function
- Function signature: `pub(super) fn register_builtin_commands(commands: &mut HashMap<String, CommandInfo>)`
- Extract method body, replacing `self.register_command(&CommandInfo {...})` with direct HashMap inserts

### STEP 5: Create `mod.rs`

Combine remaining sections:
- Module declarations and re-exports
- Struct definition (lines 64-71)
- impl Default (lines 73-77)
- impl CommandParser::new() - modified to call `builtin_commands::register_builtin_commands(&mut commands)`
- parse_command() method (lines 98-236)
- register_command() (lines 569-590)
- Utility methods (lines 869-894)

### STEP 6: Update `new()` Method

Original calls `self.register_builtin_commands()`. Change to:

```rust
pub fn new() -> Self {
    let mut parser = Self {
        commands: HashMap::new(),
        aliases: HashMap::new(),
        history: Vec::new(),
    };
    builtin_commands::register_builtin_commands(&mut parser.commands);
    parser
}
```

### STEP 7: Delete Original File

```bash
rm /Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/commands/parsing.rs
```

### STEP 8: Verify Compilation

```bash
cd /Volumes/samsung_t9/cyrup/packages/candle
cargo check
```

Expected: No errors. The module structure change is transparent to external code due to re-exports.

## FILE CHANGES REQUIRED

### Files to Create

1. `/Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/commands/parsing/errors.rs` (~60 lines)
2. `/Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/commands/parsing/command_parsers.rs` (~290 lines)
3. `/Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/commands/parsing/builtin_commands.rs` (~340 lines)
4. `/Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/commands/parsing/mod.rs` (~200 lines)

### Files to Delete

1. `/Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/commands/parsing.rs`

### Files to Verify Unchanged

1. [`/Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/commands/mod.rs`](../../../packages/candle/src/domain/chat/commands/mod.rs) - Should not need changes (uses `pub use parsing::*`)
2. [`/Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/commands/execution.rs`](../../../packages/candle/src/domain/chat/commands/execution.rs) - Should not need changes (uses re-exported `CommandParser`)

## CRITICAL IMPLEMENTATION NOTES

### Preserve Both Parsing Methods

The file contains TWO distinct parsing approaches:

1. **`parse_command()`** - Currently used by execution.rs, inline matching
2. **`parse()` + `parse_*` functions** - Alternative API, cleaner error handling

Both must be preserved even though they duplicate functionality. This is existing technical debt, not introduced by this refactoring.

### Handle Aliases Properly

The `parse()` method in command_parsers.rs needs access to the aliases HashMap. Pass it as a parameter:

```rust
pub(super) fn parse(input: &str, aliases: &HashMap<String, String>) -> ParseResult<ImmutableChatCommand>
```

### CommandInfo Registration Pattern

Original pattern:
```rust
self.register_command(&CommandInfo { ... });
```

New pattern in builtin_commands.rs:
```rust
commands.insert("help".to_string(), CommandInfo { ... });
```

Also extract alias registration from CommandInfo.aliases field into the commands HashMap.

### Imports and Visibility

- errors.rs: All public (re-exported by mod.rs)
- command_parsers.rs: All `pub(super)` (internal to parsing module)
- builtin_commands.rs: All `pub(super)` (internal to parsing module)
- mod.rs: Re-exports public items, contains public CommandParser impl

## DEFINITION OF DONE

- [x] Research complete: File structure analyzed
- [x] Research complete: Dependencies identified
- [x] Research complete: Public API mapped
- [ ] Directory `parsing/` created
- [ ] File `errors.rs` created (~60 lines)
- [ ] File `command_parsers.rs` created (~290 lines)
- [ ] File `builtin_commands.rs` created (~340 lines)
- [ ] File `mod.rs` created (~200 lines)
- [ ] Original `parsing.rs` deleted
- [ ] `cargo check` passes without errors
- [ ] Public API unchanged (CommandParser, ParseError, ParseResult)
- [ ] No tests written (per constraints)
- [ ] No benchmarks written (per constraints)
- [ ] No extensive documentation added (per constraints)

## VERIFICATION COMMANDS

```bash
# Check file sizes
wc -l /Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/commands/parsing/*.rs

# Verify compilation
cd /Volumes/samsung_t9/cyrup/packages/candle && cargo check

# Verify public API (should show CommandParser, ParseError, ParseResult)
grep "pub use parsing" /Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/commands/mod.rs
```

## SUCCESS CRITERIA

This task is successful when:
1. The original 894-line file is decomposed into 4 focused modules
2. Each module is under 350 lines (target <300, acceptable if data-heavy)
3. All functionality is preserved without behavior changes
4. The codebase compiles without errors (`cargo check`)
5. The public API remains unchanged
6. Code is more maintainable with clear module boundaries
7. No tests, benchmarks, or extensive documentation added
