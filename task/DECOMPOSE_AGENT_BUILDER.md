# TASK: Decompose agent_role.rs into Modular Structure

**Status**: PENDING  
**Priority**: HIGH  
**File**: [`packages/candle/src/builders/agent_role.rs`](../packages/candle/src/builders/agent_role.rs)  
**Current Size**: 2064 lines  
**Target**: 7 modules, each < 500 lines

---

## ⚠️ CRITICAL DIRECTIVE

**ANY ROLLBACK OR RESTORATION OF THE ORIGINAL MONOLITHIC FILE WILL RESULT IN IMMEDIATE TERMINATION.**

This decomposition MUST be completed successfully. No exceptions. Fix forward only.

---

## Core Objective

**Pure refactoring**: Move code from single 2064-line file into 7 logical modules without changing ANY behavior.

**Success = Code compiles + Examples run identically**

NOT required:
- ❌ Unit tests
- ❌ Integration tests  
- ❌ Benchmarks
- ❌ Documentation beyond module headers
- ❌ Performance validation
- ❌ New functionality

---

## Context: Existing Builder Architecture

The codebase already has three separate builder systems:

1. **`src/builders/agent_builder.rs`** (195 lines)  
   Zero-allocation builder using const generics - LOW-LEVEL pattern  
   *No conflict - different purpose*

2. **`src/builders/chat/`** directory (397 lines total)  
   Chat conversation and history builders - already well-decomposed  
   *No conflict - different domain*

3. **`src/builders/agent_role.rs`** (2064 lines) ← **TARGET FOR DECOMPOSITION**  
   High-level fluent API with `CandleAgentRoleBuilder` trait  
   Used in [`examples/fluent_builder.rs`](../packages/candle/examples/fluent_builder.rs)

The new module will be: **`src/builders/agent_role/`** (directory)

---

## File Structure Analysis

### Current Monolith: agent_role.rs (2064 lines)

```
Lines     | Content                                      | Target Module
----------|----------------------------------------------|------------------
1-43      | Imports, type aliases, doc comments          | mod.rs
44-287    | CandleAgentRoleAgent helper struct + impl    | helpers.rs
288-395   | CandleAgentRoleBuilder trait definition      | traits.rs
396-406   | CandleMcpServerBuilder trait definition      | traits.rs  
407-506   | CandleAgentBuilder trait definition          | traits.rs
507-531   | MCP stub implementations                     | stubs.rs
532-533   | McpServerConfig struct                       | stubs.rs
534-656   | CandleAgentRoleBuilderImpl struct definition | role_builder.rs
659-899   | impl CandleAgentRoleBuilder for ...Impl     | stubs.rs
900-907   | AgentDebugInfo struct                        | agent_builder.rs
908-927   | CandleAgentBuilderImpl struct definition     | agent_builder.rs
928-951   | Debug impl for CandleAgentBuilderImpl        | agent_builder.rs
952-1124  | impl CandleAgentRoleBuilder for ...Impl     | agent_builder.rs
1125-1155 | format_memory_context helper function        | helpers.rs
1156-1968 | impl CandleAgentBuilder for ...Impl (BIG)   | chat.rs
1969-2046 | ConversationHistoryArgs trait + impls        | helpers.rs
2047-2064 | CandleFluentAi entry point struct            | helpers.rs
```

---

## Target Module Structure

```
src/builders/agent_role/
├── mod.rs              (~65 lines)
├── traits.rs           (~255 lines)
├── helpers.rs          (~395 lines)
├── stubs.rs            (~275 lines)
├── role_builder.rs     (~125 lines)
├── agent_builder.rs    (~220 lines)
└── chat.rs             (~820 lines)
```

### mod.rs (~65 lines)

**Purpose**: Module organization, shared imports, type aliases

**Contents**:
- Module declarations (`mod helpers;`, `mod traits;`, etc.)
- Public re-exports for external API
- Shared type aliases (OnChunkHandler, OnToolResultHandler, OnConversationTurnHandler)
- Shared imports wrapped in `pub(crate) use`

**Code Pattern**:
```rust
//! Agent role builder - Fluent API for configuring AI agents

// Submodules
mod helpers;
mod traits;
mod stubs;
mod role_builder;
mod agent_builder;
mod chat;

// Public API
pub use helpers::{CandleAgentRoleAgent, CandleFluentAi};
pub use traits::{CandleAgentRoleBuilder, CandleMcpServerBuilder, CandleAgentBuilder};
pub use stubs::McpServerConfig;
pub use role_builder::CandleAgentRoleBuilderImpl;
pub use agent_builder::{CandleAgentBuilderImpl, AgentDebugInfo};
pub use helpers::ConversationHistoryArgs;

// Shared imports (available to all submodules via `use super::*`)
pub(crate) use std::num::NonZeroU64;
pub(crate) use std::sync::Arc;
pub(crate) use std::pin::Pin;
pub(crate) use cyrup_sugars::ZeroOneOrMany;
pub(crate) use tokio_stream::{Stream, StreamExt};
// ... (all imports from lines 1-28 of original)

// Type aliases
pub(crate) type OnChunkHandler = Arc<dyn Fn(CandleMessageChunk) -> ...>;
pub(crate) type OnToolResultHandler = Arc<dyn Fn(&[String]) -> ...>;
pub(crate) type OnConversationTurnHandler = Arc<dyn Fn(...) -> ...>;

// Shared state struct (extracted from nested usage in CandleAgentRoleAgent)
pub(crate) struct AgentBuilderState {
    pub text_to_text_model: TextToTextModel,
    pub temperature: f64,
    pub max_tokens: u64,
    pub system_prompt: String,
    pub tools: ZeroOneOrMany<ToolInfo>,
    pub on_chunk_handler: Option<OnChunkHandler>,
    pub on_tool_result_handler: Option<OnToolResultHandler>,
}
```

**Extraction**:
```bash
# Lines 1-43 from original
sed -n '1,43p' src/builders/agent_role.rs
```

---

### traits.rs (~255 lines)

**Purpose**: Public trait definitions

**Contents**:
- `CandleAgentRoleBuilder` trait (lines 288-395)
- `CandleMcpServerBuilder` trait (lines 396-406)
- `CandleAgentBuilder` trait (lines 407-506)

**Code Pattern**:
```rust
//! Trait definitions for agent role builders

use super::*;

/// Builder trait for agent roles - fluent configuration API
pub trait CandleAgentRoleBuilder: Sized + Send {
    // Method signatures from lines 288-395
}

/// MCP server configuration builder
pub trait CandleMcpServerBuilder: Sized + Send {
    // Method signatures from lines 396-406
}

/// Agent builder with model attached
pub trait CandleAgentBuilder: Sized + Send + Sync {
    // Method signatures from lines 407-506
}
```

**Extraction**:
```bash
sed -n '288,506p' src/builders/agent_role.rs > src/builders/agent_role/traits_raw.txt
```

**Visibility**: All traits are `pub` (public API)

---

### helpers.rs (~395 lines)

**Purpose**: Helper types and utility functions

**Contents**:
- `CandleAgentRoleAgent` struct + impl (lines 44-287)
- `format_memory_context` function (lines 1125-1155)
- `ConversationHistoryArgs` trait + impls (lines 1969-2046)
- `CandleFluentAi` entry point (lines 2047-2064)

**Code Pattern**:
```rust
//! Helper types for agent builders

use super::*;

/// Agent helper for recursive inference in on_conversation_turn callbacks
#[derive(Clone)]
pub struct CandleAgentRoleAgent {
    state: Arc<AgentBuilderState>,
}

impl CandleAgentRoleAgent {
    pub fn chat(&self, chat_loop: CandleChatLoop) -> Pin<Box<dyn Stream<...>>> {
        // Lines 49-97
    }
    
    fn run_inference_cycle(&self, user_message: String) -> Pin<Box<dyn Stream<...>>> {
        // Lines 99-287
    }
}

/// Format memory nodes into prompt context
fn format_memory_context(
    memories: &[crate::memory::primitives::node::MemoryNode],
    max_tokens: usize,
) -> String {
    // Lines 1125-1155
}

/// Trait for conversation history syntax sugar
pub trait ConversationHistoryArgs {
    // Lines 1969-1973
}

// Trait impls for various tuple types (lines 1974-2046)

/// Fluent API entry point
pub struct CandleFluentAi;

impl CandleFluentAi {
    pub fn agent_role(name: impl Into<String>) -> impl CandleAgentRoleBuilder {
        // Lines 2050-2052
    }
}
```

**Extraction**:
```bash
# Three separate sections
sed -n '44,287p' src/builders/agent_role.rs    # CandleAgentRoleAgent
sed -n '1125,1155p' src/builders/agent_role.rs  # format_memory_context
sed -n '1969,2064p' src/builders/agent_role.rs  # ConversationHistoryArgs + CandleFluentAi
```

---

### stubs.rs (~275 lines)

**Purpose**: MCP stub implementations and early-stage builder impl

**Contents**:
- `CandleMcpServerBuilderImpl` struct + impl (lines 507-531)
- `McpServerConfig` struct (lines 532-533)
- `CandleAgentRoleBuilderImpl` impl `CandleAgentRoleBuilder` (lines 659-899)

**Why these together**: These represent the "no model yet" stage of the builder, before into_agent() is called

**Code Pattern**:
```rust
//! MCP stubs and pre-model builder implementations

use super::*;

/// MCP server builder stub
pub struct CandleMcpServerBuilderImpl<T> {
    parent_builder: CandleAgentRoleBuilderImpl,
    binary_path: Option<String>,
}

impl<T> CandleMcpServerBuilder for CandleMcpServerBuilderImpl<T> {
    // Lines 512-531
}

/// MCP server configuration
pub struct McpServerConfig {
    pub binary_path: String,
    pub init_command: String,
}

// impl CandleAgentRoleBuilder for CandleAgentRoleBuilderImpl (lines 659-899)
// This is the "stub" implementation before model is attached
```

**Extraction**:
```bash
sed -n '507,533p' src/builders/agent_role.rs   # MCP structs
sed -n '659,899p' src/builders/agent_role.rs   # Early impl
```

---

### role_builder.rs (~125 lines)

**Purpose**: CandleAgentRoleBuilderImpl struct definition

**Contents**:
- `CandleAgentRoleBuilderImpl` struct (lines 534-556)
- Debug impl (lines 558-576)
- `new()` constructor (lines 577-656)

**Code Pattern**:
```rust
//! CandleAgentRoleBuilderImpl - builder before model attachment

use super::*;

/// First-stage builder without model
struct CandleAgentRoleBuilderImpl {
    name: String,
    text_to_text_model: Option<TextToTextModel>,
    text_embedding_model: Option<TextEmbeddingModel>,
    temperature: f64,
    max_tokens: Option<u64>,
    memory_read_timeout: u64,
    system_prompt: String,
    tools: ZeroOneOrMany<ToolInfo>,
    context_file: Option<CandleContext<CandleFile>>,
    context_files: Option<CandleContext<CandleFiles>>,
    context_directory: Option<CandleContext<CandleDirectory>>,
    context_github: Option<CandleContext<CandleGithub>>,
    additional_params: std::collections::HashMap<String, String>,
    metadata: std::collections::HashMap<String, String>,
    on_chunk_handler: Option<OnChunkHandler>,
    on_tool_result_handler: Option<OnToolResultHandler>,
    on_conversation_turn_handler: Option<OnConversationTurnHandler>,
}

impl std::fmt::Debug for CandleAgentRoleBuilderImpl {
    // Lines 558-576
}

impl CandleAgentRoleBuilderImpl {
    pub fn new(name: impl Into<String>) -> Self {
        // Lines 577-656 - creates default thinking/reasoner tools
    }
}
```

**Extraction**:
```bash
sed -n '534,656p' src/builders/agent_role.rs
```

---

### agent_builder.rs (~220 lines)

**Purpose**: CandleAgentBuilderImpl struct and first trait impl

**Contents**:
- `AgentDebugInfo` struct (lines 900-907)
- `CandleAgentBuilderImpl` struct (lines 908-927)
- Debug impl (lines 928-951)
- impl `CandleAgentRoleBuilder` for `CandleAgentBuilderImpl` (lines 952-1124)

**Why this grouping**: This is the "model attached" builder with setter methods

**Code Pattern**:
```rust
//! CandleAgentBuilderImpl - builder with model attached

use super::*;

/// Debug info for agent configuration
#[derive(Debug, Clone)]
pub struct AgentDebugInfo {
    pub name: String,
    pub temperature: Option<f64>,
    pub max_tokens: Option<u64>,
    pub has_system_prompt: bool,
}

/// Agent builder with model
pub struct CandleAgentBuilderImpl {
    name: String,
    text_to_text_model: TextToTextModel,
    text_embedding_model: Option<TextEmbeddingModel>,
    temperature: f64,
    max_tokens: u64,
    memory_read_timeout: u64,
    system_prompt: String,
    tools: ZeroOneOrMany<ToolInfo>,
    context_file: Option<CandleContext<CandleFile>>,
    context_files: Option<CandleContext<CandleFiles>>,
    context_directory: Option<CandleContext<CandleDirectory>>,
    context_github: Option<CandleContext<CandleGithub>>,
    additional_params: std::collections::HashMap<String, String>,
    metadata: std::collections::HashMap<String, String>,
    on_chunk_handler: Option<OnChunkHandler>,
    on_tool_result_handler: Option<OnToolResultHandler>,
    on_conversation_turn_handler: Option<OnConversationTurnHandler>,
}

impl std::fmt::Debug for CandleAgentBuilderImpl {
    // Lines 928-951
}

// First trait impl - configuration methods
impl CandleAgentRoleBuilder for CandleAgentBuilderImpl {
    // Lines 952-1124 - model(), temperature(), max_tokens(), etc.
}
```

**Extraction**:
```bash
sed -n '900,1124p' src/builders/agent_role.rs
```

---

### chat.rs (~820 lines)

**Purpose**: Main chat() implementation - the largest method

**Contents**:
- impl `CandleAgentBuilder` for `CandleAgentBuilderImpl` (lines 1156-1968)
  - Includes the massive `chat()` method (566 lines)
  - `chat_with_message()` helper (88 lines)
  - Setter methods (model, embedding_model, temperature, etc.)

**Why 820 lines**: This is the core inference logic - cannot be split further without refactoring the actual logic

**Code Pattern**:
```rust
//! Chat implementation for CandleAgentBuilder

use super::*;

impl CandleAgentBuilder for CandleAgentBuilderImpl {
    fn model(mut self, model: TextToTextModel) -> Self {
        self.text_to_text_model = model;
        self
    }

    fn embedding_model(mut self, model: TextEmbeddingModel) -> Self {
        self.text_embedding_model = Some(model);
        self
    }

    // More setters... (lines 1156-1284)

    fn chat<F, Fut>(self, handler: F) -> Result<Pin<Box<dyn Stream<...>>>, AgentError>
    where
        F: Fn(&CandleAgentConversation) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = CandleChatLoop> + Send + 'static,
    {
        // Lines 1285-1850 - THE BIG METHOD
        // - Memory initialization
        // - Tool router setup
        // - Prompt formatting
        // - Stream generation loop
        // - Chunk forwarding
        // - Memory storage
    }

    fn chat_with_message(self, message: impl Into<String>) -> Pin<Box<dyn Stream<...>>> {
        // Lines 1891-1968 - convenience wrapper
    }
}
```

**Extraction**:
```bash
sed -n '1156,1968p' src/builders/agent_role.rs
```

**Note**: This file will be ~820 lines, exceeding the 500-line limit. However:
- The `chat()` method (566 lines) is a single coherent async logic block
- Breaking it would require actual refactoring (extracting helper functions)
- Task scope is "decompose into modules", not "refactor internal methods"
- **Acceptable exception** given method cohesion

---

## Step-by-Step Execution

### Phase 1: Prepare (5 min)

```bash
cd /Volumes/samsung_t9/paraphym/packages/candle

# Verify original file exists and size
wc -l src/builders/agent_role.rs
# Expected: 2064 src/builders/agent_role.rs

# Create module directory
mkdir -p src/builders/agent_role
```

---

### Phase 2: Create mod.rs (10 min)

Create the module root with all shared infrastructure:

```bash
cat > src/builders/agent_role/mod.rs << 'EOF'
//! Agent role builder - Fluent API for AI agent configuration

mod helpers;
mod traits;
mod stubs;
mod role_builder;
mod agent_builder;
mod chat;

pub use helpers::{CandleAgentRoleAgent, CandleFluentAi, ConversationHistoryArgs};
pub use traits::{CandleAgentRoleBuilder, CandleMcpServerBuilder, CandleAgentBuilder};
pub use stubs::McpServerConfig;
pub use role_builder::CandleAgentRoleBuilderImpl;
pub use agent_builder::{CandleAgentBuilderImpl, AgentDebugInfo};

pub(crate) use std::num::NonZeroU64;
pub(crate) use std::sync::Arc;
pub(crate) use std::pin::Pin;
pub(crate) use cyrup_sugars::ZeroOneOrMany;
pub(crate) use tokio_stream::{Stream, StreamExt};
pub(crate) use crate::capability::registry::{TextEmbeddingModel, TextToTextModel};
pub(crate) use crate::capability::traits::TextToTextCapable;
pub(crate) use crate::domain::agent::core::AgentError;
pub(crate) use crate::domain::chat::CandleChatLoop;
pub(crate) use crate::domain::chat::message::{CandleMessageChunk, CandleMessageRole};
pub(crate) use crate::domain::completion::{CandleCompletionChunk, types::CandleCompletionParams};
pub(crate) use crate::domain::context::provider::{CandleContext, CandleDirectory, CandleFile, CandleFiles, CandleGithub};
pub(crate) use crate::domain::prompt::CandlePrompt;
pub(crate) use crate::domain::tool::SweetMcpRouter;
pub(crate) use serde_json;
pub(crate) use sweet_mcp_type::ToolInfo;
pub(crate) use crate::domain::agent::role::CandleAgentConversation;
pub(crate) use crate::memory::core::manager::surreal::Result as MemoryResult;

pub(crate) type OnChunkHandler = Arc<dyn Fn(CandleMessageChunk) -> Pin<Box<dyn std::future::Future<Output = CandleMessageChunk> + Send>> + Send + Sync>;
pub(crate) type OnToolResultHandler = Arc<dyn Fn(&[String]) -> Pin<Box<dyn std::future::Future<Output = ()> + Send>> + Send + Sync>;
pub(crate) type OnConversationTurnHandler = Arc<dyn Fn(&CandleAgentConversation, &CandleAgentRoleAgent) -> Pin<Box<dyn std::future::Future<Output = Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>> + Send>> + Send + Sync>;

pub(crate) struct AgentBuilderState {
    pub text_to_text_model: TextToTextModel,
    pub temperature: f64,
    pub max_tokens: u64,
    pub system_prompt: String,
    pub tools: ZeroOneOrMany<ToolInfo>,
    pub on_chunk_handler: Option<OnChunkHandler>,
    pub on_tool_result_handler: Option<OnToolResultHandler>,
}
EOF
```

---

### Phase 3: Extract traits.rs (10 min)

```bash
{
  echo "//! Trait definitions for agent role builders"
  echo ""
  echo "use super::*;"
  echo ""
  sed -n '288,506p' src/builders/agent_role.rs
} > src/builders/agent_role/traits.rs
```

---

### Phase 4: Extract helpers.rs (15 min)

```bash
{
  echo "//! Helper types and utility functions"
  echo ""
  echo "use super::*;"
  echo ""
  sed -n '44,287p' src/builders/agent_role.rs
  echo ""
  sed -n '1125,1155p' src/builders/agent_role.rs
  echo ""
  sed -n '1969,2064p' src/builders/agent_role.rs
} > src/builders/agent_role/helpers.rs
```

---

### Phase 5: Extract stubs.rs (10 min)

```bash
{
  echo "//! MCP stubs and pre-model builder implementations"
  echo ""
  echo "use super::*;"
  echo ""
  sed -n '507,533p' src/builders/agent_role.rs
  echo ""
  sed -n '659,899p' src/builders/agent_role.rs
} > src/builders/agent_role/stubs.rs
```

---

### Phase 6: Extract role_builder.rs (10 min)

```bash
{
  echo "//! CandleAgentRoleBuilderImpl - builder before model"
  echo ""
  echo "use super::*;"
  echo ""
  sed -n '534,656p' src/builders/agent_role.rs
} > src/builders/agent_role/role_builder.rs
```

---

### Phase 7: Extract agent_builder.rs (10 min)

```bash
{
  echo "//! CandleAgentBuilderImpl - builder with model"
  echo ""
  echo "use super::*;"
  echo ""
  sed -n '900,1124p' src/builders/agent_role.rs
} > src/builders/agent_role/agent_builder.rs
```

---

### Phase 8: Extract chat.rs (10 min)

```bash
{
  echo "//! Chat implementation for CandleAgentBuilder"
  echo ""
  echo "use super::*;"
  echo ""
  sed -n '1156,1968p' src/builders/agent_role.rs
} > src/builders/agent_role/chat.rs
```

---

### Phase 9: Verify Extraction (5 min)

```bash
# Check all files created
ls -la src/builders/agent_role/

# Count lines in each file
wc -l src/builders/agent_role/*.rs

# Verify total matches original (approximately)
# Original: 2064 lines
# New total should be ~2100-2200 (overhead from module headers)
```

**Expected output**:
```
  65 src/builders/agent_role/mod.rs
 255 src/builders/agent_role/traits.rs
 395 src/builders/agent_role/helpers.rs
 275 src/builders/agent_role/stubs.rs
 125 src/builders/agent_role/role_builder.rs
 220 src/builders/agent_role/agent_builder.rs
 820 src/builders/agent_role/chat.rs
2155 total
```

---

### Phase 10: Compile Check (10 min)

```bash
cargo check --lib 2>&1 | tee /tmp/decompose_check.log
```

**Common issues and fixes**:

1. **Missing imports**: Add to individual module files
   ```rust
   use super::*;  // Should pull from mod.rs
   ```

2. **Visibility errors**: Check pub vs pub(super) vs pub(crate)
   ```rust
   pub struct X;        // Public API
   pub(super) struct Y; // Module-family only
   pub(crate) struct Z; // Crate-wide
   ```

3. **Incomplete impl blocks**: Ensure method signatures match trait exactly

---

### Phase 11: Delete Original (2 min)

**ONLY after cargo check passes:**

```bash
# Verify compilation first
cargo check --lib || { echo "COMPILATION FAILED - DO NOT DELETE"; exit 1; }

# Delete original monolith
rm src/builders/agent_role.rs

# Verify module is used
cargo check --lib || { echo "MODULE NOT LOADED CORRECTLY"; exit 1; }
```

---

### Phase 12: Clean Up (2 min)

```bash
# Remove any backup files
find src/builders -name "*.bak" -o -name "*.old" -o -name "*.tmp" -delete

# Verify no backups remain
find src/builders -name "*.bak" -o -name "*.old" -o -name "*.tmp"
# Expected: (empty output)
```

---

## Definition of Done

**Compilation**:
- `cargo check --lib` exits 0
- `cargo check --all-targets` exits 0
- Zero warnings

**Examples**:
- `cargo run --example fluent_builder` executes successfully
- Model generates text output (not just logging)

**File Structure**:
- 7 module files in `src/builders/agent_role/`
- 6 files under 500 lines (chat.rs may be ~820)
- Original `agent_role.rs` deleted
- No backup files (*.bak, *.old, *.tmp)

**Code Quality**:
- No unused imports
- No dead code warnings
- Consistent module documentation

---

## Troubleshooting Guide

### Issue: "cannot find type X in this scope"

**Cause**: Missing type alias or import  
**Fix**: Add to mod.rs `pub(crate) use` section

### Issue: "private type X in public interface"

**Cause**: Visibility mismatch  
**Fix**: Change to `pub struct X` in defining module

### Issue: "trait X is not implemented for Y"

**Cause**: Incomplete trait impl extraction  
**Fix**: Verify all methods extracted, check impl block wrapping

### Issue: "multiple applicable items in scope"

**Cause**: Duplicate imports from `use super::*`  
**Fix**: Explicitly import conflicting item in that module

### Issue: "unused import warnings"

**Cause**: Import needed by only some modules  
**Fix**: Move import from mod.rs to specific module file

---

## Critical Requirements Checklist

- [ ] `cargo check --lib` exits 0
- [ ] `cargo run --example fluent_builder` works
- [ ] All 7 module files created
- [ ] Line counts verified (6 under 500, chat.rs exempt)
- [ ] Original agent_role.rs deleted
- [ ] No backup files in src/builders/
- [ ] Module exports match original public API
- [ ] Zero compilation warnings

---

## Source File References

- **Target File**: [`src/builders/agent_role.rs`](../packages/candle/src/builders/agent_role.rs)
- **Example Usage**: [`examples/fluent_builder.rs`](../packages/candle/examples/fluent_builder.rs)
- **Related Builders**: 
  - [`src/builders/agent_builder.rs`](../packages/candle/src/builders/agent_builder.rs) (different pattern)
  - [`src/builders/chat/`](../packages/candle/src/builders/chat/) (already modular)

---

**Task Created**: 2025-10-19  
**Last Updated**: 2025-10-19 16:34  
**Status**: READY FOR EXECUTION
