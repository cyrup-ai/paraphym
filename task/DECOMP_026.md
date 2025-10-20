# DECOMP_026: Decompose `actions.rs`

**File:** `packages/candle/src/domain/chat/commands/types/actions.rs`  
**Current Size:** 812 lines  
**Module Area:** domain / chat / commands / types

## CORE OBJECTIVE

Transform the monolithic 812-line `actions.rs` file into a modular directory structure with 5 focused modules, each under 300 lines. The file contains 11 independent action enums that follow a consistent pattern and can be cleanly separated by domain.

**Why this matters:** The file is a simple aggregation of similar enums with no cross-dependencies. Decomposition will improve maintainability, navigation, and adherence to single-responsibility principle.

## CURRENT STATE ANALYSIS

### File Structure
[Source: `./src/domain/chat/commands/types/actions.rs`](../../packages/candle/src/domain/chat/commands/types/actions.rs)

The file contains 11 action enumerations:

1. **TemplateAction** (lines 8-68) - Template management operations
2. **MacroAction** (lines 70-158) - Macro operations and execution control
3. **BranchAction** (lines 160-230) - Conversation branch management
4. **SessionAction** (lines 232-309) - Session lifecycle and management
5. **ToolAction** (lines 311-400) - Tool installation and execution
6. **SearchScope** (lines 402-454) - Search filtering and scoping
7. **StatsType** (lines 456-510) - Statistics and metrics types
8. **ThemeAction** (lines 512-580) - Theme customization operations
9. **DebugAction** (lines 582-642) - Debug and diagnostics operations
10. **HistoryAction** (lines 644-710) - History management operations
11. **ImportType** (lines 712-812) - Data import type definitions

### Common Pattern

Each enum follows this structure:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SomeAction {
    /// Variant documentation
    VariantName,
    // ... more variants
}

impl SomeAction {
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self { /* ... */ }
    }
    
    // Domain-specific helper methods (1-3 methods)
    // e.g., requires_name(), is_mutating(), etc.
}
```

### Dependencies

**Imports:** Only `serde::{Deserialize, Serialize}` - no internal dependencies between enums.

**Used by:** [`commands.rs`](../../packages/candle/src/domain/chat/commands/types/commands.rs) imports all action types (lines 10-13):
```rust
use super::actions::{
    BranchAction, DebugAction, HistoryAction, ImportType, MacroAction, SearchScope, SessionAction,
    StatsType, TemplateAction, ThemeAction, ToolAction,
};
```

**Re-exported by:** [`mod.rs`](../../packages/candle/src/domain/chat/commands/types/mod.rs) (line 9):
```rust
pub use self::actions::*;
```

**Key insight:** No cross-dependencies between action enums = clean separation possible.

## DECOMPOSITION PLAN

### Target Structure

```
types/
├── actions/                    # NEW directory (replaces actions.rs)
│   ├── mod.rs                  # ~30 lines - module aggregator
│   ├── content_actions.rs      # ~250 lines - content management
│   ├── state_actions.rs        # ~200 lines - state management  
│   ├── system_actions.rs       # ~200 lines - system operations
│   ├── ui_actions.rs           # ~100 lines - UI configuration
│   └── data_actions.rs         # ~150 lines - data operations
├── commands.rs
├── mod.rs
└── ... (other files)
```

**Existing Pattern:** See [`code_execution/`](../../packages/candle/src/domain/chat/commands/types/code_execution/) directory for working example of this exact pattern in the same module.

### Module Groupings (Domain-Driven)

#### 1. `content_actions.rs` (~250 lines)
**Domain:** Content and workflow management

- `TemplateAction` - Template CRUD and operations
- `MacroAction` - Macro recording, execution, management
- `BranchAction` - Conversation branching and merging

**Rationale:** These manage user-created content and workflows.

#### 2. `state_actions.rs` (~200 lines)  
**Domain:** State persistence and lifecycle

- `SessionAction` - Session creation, switching, archival
- `HistoryAction` - History search, backup, analysis

**Rationale:** Both manage persistent state across time.

#### 3. `system_actions.rs` (~200 lines)
**Domain:** System-level operations

- `ToolAction` - Tool installation, execution, configuration
- `DebugAction` - Diagnostics, logging, profiling

**Rationale:** System administration and debugging functionality.

#### 4. `ui_actions.rs` (~100 lines)
**Domain:** User interface configuration

- `ThemeAction` - Theme customization and management

**Rationale:** Isolated UI/appearance concerns.

#### 5. `data_actions.rs` (~150 lines)
**Domain:** Data query and operations

- `SearchScope` - Search filtering parameters
- `StatsType` - Statistics and metrics categories  
- `ImportType` - Data import type specifications

**Rationale:** All deal with data retrieval and categorization.

### Module Re-export Pattern

**`actions/mod.rs`** will be created as:

```rust
//! Action type definitions for command variants with zero allocation patterns
//!
//! Provides blazing-fast action enumeration dispatch with owned strings for
//! maximum performance. No Arc usage, no locking, pure enum-based dispatch.

// Module declarations
pub mod content_actions;
pub mod state_actions;
pub mod system_actions;
pub mod ui_actions;
pub mod data_actions;

// Re-export all public types to maintain existing API
pub use content_actions::{BranchAction, MacroAction, TemplateAction};
pub use state_actions::{HistoryAction, SessionAction};
pub use system_actions::{DebugAction, ToolAction};
pub use ui_actions::ThemeAction;
pub use data_actions::{ImportType, SearchScope, StatsType};
```

**Critical:** This preserves the existing `pub use self::actions::*;` pattern in parent `mod.rs`.

## EXECUTION STEPS

### Step 1: Create Module Directory
```bash
cd /Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/commands/types/
mkdir actions
```

### Step 2: Create `content_actions.rs`

Extract from `actions.rs`:
- Lines 1-7 (imports and module doc - adapt for this module)
- Lines 8-68 (TemplateAction)
- Lines 70-158 (MacroAction)  
- Lines 160-230 (BranchAction)

Add module documentation:
```rust
//! Content and workflow management action types
//!
//! Provides action enums for templates, macros, and conversation branches.

use serde::{Deserialize, Serialize};

// ... paste enums and impls
```

### Step 3: Create `state_actions.rs`

Extract from `actions.rs`:
- Lines 232-309 (SessionAction)
- Lines 644-710 (HistoryAction)

Add module documentation:
```rust
//! State persistence and lifecycle action types
//!
//! Provides action enums for session and history management.

use serde::{Deserialize, Serialize};

// ... paste enums and impls
```

### Step 4: Create `system_actions.rs`

Extract from `actions.rs`:
- Lines 311-400 (ToolAction)
- Lines 582-642 (DebugAction)

Add module documentation:
```rust
//! System-level operation action types
//!
//! Provides action enums for tool management and debugging.

use serde::{Deserialize, Serialize};

// ... paste enums and impls
```

### Step 5: Create `ui_actions.rs`

Extract from `actions.rs`:
- Lines 512-580 (ThemeAction)

Add module documentation:
```rust
//! User interface configuration action types
//!
//! Provides action enums for theme customization.

use serde::{Deserialize, Serialize};

// ... paste enum and impl
```

### Step 6: Create `data_actions.rs`

Extract from `actions.rs`:
- Lines 402-454 (SearchScope)
- Lines 456-510 (StatsType)
- Lines 712-812 (ImportType)

Add module documentation:
```rust
//! Data query and operation action types
//!
//! Provides action enums for search, statistics, and import operations.

use serde::{Deserialize, Serialize};

// ... paste enums and impls
```

### Step 7: Create `actions/mod.rs`

Use the re-export pattern shown above in "Module Re-export Pattern" section.

### Step 8: Delete Original File

```bash
rm actions.rs
```

### Step 9: Verify Compilation

```bash
cd /Volumes/samsung_t9/cyrup/packages/candle
cargo check
```

**Expected:** No errors. All imports in `commands.rs` and parent `mod.rs` continue to work unchanged.

## PUBLIC API PRESERVATION

### Before (Current)
```rust
// In commands.rs
use super::actions::{TemplateAction, MacroAction, /* ... */};
```

### After (Post-Decomposition)
```rust
// In commands.rs - NO CHANGE REQUIRED
use super::actions::{TemplateAction, MacroAction, /* ... */};
```

The `actions/mod.rs` re-exports make all types available as if they were in a single file.

## COMMON PATTERNS TO REUSE

Every action enum shares this structure:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionName {
    Variant1,
    Variant2,
}

impl ActionName {
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Variant1 => "variant1",
            Self::Variant2 => "variant2",
        }
    }
    
    // Optional: 1-3 domain helper methods
    pub const fn some_property(&self) -> bool {
        matches!(self, Self::Variant1 | Self::Variant2)
    }
}
```

**Copy-paste safe:** Each enum is self-contained with no external references except serde.

## DEFINITION OF DONE

- [ ] Directory `actions/` created at `/Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/commands/types/actions/`
- [ ] File `actions.rs` deleted
- [ ] Five new module files created:
  - [ ] `content_actions.rs` (TemplateAction, MacroAction, BranchAction)
  - [ ] `state_actions.rs` (SessionAction, HistoryAction)
  - [ ] `system_actions.rs` (ToolAction, DebugAction)
  - [ ] `ui_actions.rs` (ThemeAction)
  - [ ] `data_actions.rs` (SearchScope, StatsType, ImportType)
- [ ] File `actions/mod.rs` created with proper re-exports
- [ ] Each new file is < 300 lines
- [ ] All 11 action enums preserved with identical functionality
- [ ] `cargo check` passes without errors
- [ ] Imports in `commands.rs` unchanged and working
- [ ] Re-export in parent `mod.rs` unchanged and working

## CONSTRAINTS

- **NO TESTS:** Do not write any unit tests, integration tests, or test code
- **NO BENCHMARKS:** Do not write any benchmark code or performance tests
- **NO EXTENSIVE DOCUMENTATION:** Module-level doc comments only, no additional README or design docs
- **MAINTAIN FUNCTIONALITY:** All existing functionality must be preserved exactly as-is
- **PRESERVE PUBLIC API:** The public interface must remain completely unchanged
- **SINGLE SESSION:** This task must be completable in one focused work session

## RISK ANALYSIS

**Low Risk Factors:**
- No cross-dependencies between enums
- Simple pattern repetition
- Proven pattern exists in codebase (`code_execution/` directory)
- Only serde dependency (external)

**Zero Risk of:**
- Breaking changes (re-export pattern maintains API)
- Circular dependencies (enums are independent)
- Complex refactoring (pure file organization)

## REFERENCES

### Source Files
- Current file: [`actions.rs`](../../packages/candle/src/domain/chat/commands/types/actions.rs)
- Consumer: [`commands.rs`](../../packages/candle/src/domain/chat/commands/types/commands.rs)  
- Parent module: [`mod.rs`](../../packages/candle/src/domain/chat/commands/types/mod.rs)
- Reference pattern: [`code_execution/`](../../packages/candle/src/domain/chat/commands/types/code_execution/)

### Key Insights from Code Review
1. All action enums are `Copy` types - no ownership issues
2. All use `const fn` - compile-time evaluation preserved
3. All methods are `#[inline]` - performance preserved
4. Zero internal dependencies - perfect for decomposition
5. Existing `code_execution/` proves pattern works in this module

## SUCCESS CRITERIA

This task is successful when:
1. The original 812-line file is replaced with a modular directory structure
2. Five focused modules exist, each handling a clear domain  
3. Each module is under 300 lines
4. All functionality is preserved without behavior changes
5. The public API remains unchanged (imports work identically)
6. `cargo check` compiles without errors
7. Code organization improves maintainability and navigation
