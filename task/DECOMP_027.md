# DECOMP_027: Decompose `chat.rs`

**File:** `packages/candle/src/builders/agent_role/chat.rs`  
**Current Size:** 807 lines (verified: 846 lines total)  
**Module Area:** builders / agent_role

## CORE OBJECTIVE

Decompose the monolithic `chat.rs` file containing a single massive trait implementation (`impl CandleAgentBuilder for CandleAgentBuilderImpl`) into smaller, focused modules organized under a new `chat/` subdirectory. The file currently contains one giant trait implementation with repetitive builder methods and a 600+ line `chat()` method that handles memory, context loading, tool calling, and streaming.

## CONSTRAINTS

- **MAINTAIN FUNCTIONALITY:** All existing functionality must be preserved exactly as-is
- **SINGLE SESSION:** This task must be completable in one focused Claude session
- **NO BREAKING CHANGES:** Public API must remain unchanged
- **NO TESTS:** Do not write unit tests, integration tests, or test code
- **NO BENCHMARKS:** Do not write benchmark code
- **NO EXTENSIVE DOCUMENTATION:** Only add essential module-level comments

## CURRENT FILE STRUCTURE ANALYSIS

The `chat.rs` file contains a single `impl CandleAgentBuilder for CandleAgentBuilderImpl` block with:

### Simple Builder Methods (~100 lines)
- `model()`, `embedding_model()`, `temperature()`, `max_tokens()`
- `memory_read_timeout()`, `system_prompt()`
- `additional_params()`, `metadata()`, `context()`
- `tools()`, `mcp_server()`, `add_mcp_server_config()`
- `conversation_history()`

**Pattern:**
```rust
fn method_name(mut self, param: Type) -> Self {
    self.field = param;
    self
}
```

### Handler Registration Methods (~50 lines)
- `on_chunk()`, `on_tool_result()`, `on_conversation_turn()`

**Pattern:**
```rust
fn on_handler<F, Fut>(mut self, handler: F) -> Self {
    let wrapped = move |args| Box::pin(handler(args)) as Pin<Box<...>>;
    self.handler_field = Some(Arc::new(wrapped));
    self
}
```

### Main Chat Orchestration (~600 lines in chat() method)
- Async closure handling (`CandleChatLoop` processing)
- Memory manager initialization (SurrealDB setup)
- Database directory creation and connection
- Context loading (files, directories, github repos)
- Memory search with timeout handling
- Tool router initialization
- Prompt building with memory augmentation
- Request/response streaming
- Tool execution loop
- Conversation storage in memory
- Recursive agent support via `on_conversation_turn`

### Simple Chat Method (~60 lines)
- `chat_with_message()` - Simplified streaming without full orchestration

## RELATED FILES (DO NOT MODIFY)

### Existing Module Structure
- `mod.rs` - Defines `AgentBuilderState` and type aliases (OnChunkHandler, etc.)
- `traits.rs` - Defines `CandleAgentBuilder` trait interface
- `helpers.rs` - Contains `CandleAgentRoleAgent` with `run_inference_cycle()`
- `agent_builder.rs` - Alternative builder implementation (keep in sync)
- `role_builder.rs` - Main builder entry point
- `role_builder_impl.rs` - MCP server configuration

### Related Directories
- `builders/chat/` - Separate chat builders (conversation, history, templates) - NO CONFLICT
- `domain/agent/` - Agent domain types
- `domain/chat/` - Chat domain types and message handling
- `memory/` - Memory system implementation

Reference: [../../packages/candle/src/builders/agent_role/](../../packages/candle/src/builders/agent_role/)

## DECOMPOSITION STRATEGY

### Create New Directory Structure
```
packages/candle/src/builders/agent_role/
├── chat.rs                 (transform to ~80 lines - module aggregator)
└── chat/
    ├── mod.rs              (~20 lines - module declarations)
    ├── builder_methods.rs  (~100 lines - simple setters)
    ├── handler_registration.rs (~50 lines - handler wrapping)
    ├── memory_ops.rs       (~200 lines - memory operations)
    ├── chat_orchestration.rs (~400 lines - main chat() logic)
    └── simple_chat.rs      (~60 lines - chat_with_message())
```

### Technical Approach

**Challenge:** Rust trait implementations cannot be split across multiple files directly.

**Solution:** Extract complex logic into helper functions in separate modules, then have the trait implementation delegate to those helpers.

### Module Breakdown

#### 1. `chat/mod.rs` (~20 lines)
```rust
//! Chat implementation modules for CandleAgentBuilder

mod builder_methods;
mod handler_registration;
mod memory_ops;
mod chat_orchestration;
mod simple_chat;

pub(super) use memory_ops::*;
pub(super) use chat_orchestration::*;
pub(super) use simple_chat::*;
```

#### 2. `chat/builder_methods.rs` (~100 lines)
**Purpose:** Simple builder setters that mutate fields and return Self

**Contains:**
- All simple builder methods extracted as inherent impl methods or helper functions
- Methods: model(), embedding_model(), temperature(), max_tokens(), memory_read_timeout(), system_prompt(), additional_params(), metadata(), context(), tools(), mcp_server(), add_mcp_server_config(), conversation_history()

**Pattern to extract:**
```rust
pub(super) fn set_model(mut builder: CandleAgentBuilderImpl, model: TextToTextModel) -> CandleAgentBuilderImpl {
    builder.text_to_text_model = model;
    builder
}
```

Reference: [Lines 4-95 in chat.rs](../../packages/candle/src/builders/agent_role/chat.rs)

#### 3. `chat/handler_registration.rs` (~50 lines)
**Purpose:** Handler registration logic with Arc wrapping

**Contains:**
- on_chunk handler wrapping
- on_tool_result handler wrapping  
- on_conversation_turn handler wrapping

**Pattern to extract:**
```rust
pub(super) fn wrap_chunk_handler<F, Fut>(handler: F) -> OnChunkHandler 
where
    F: Fn(CandleMessageChunk) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = CandleMessageChunk> + Send + 'static,
{
    let wrapped = move |chunk: CandleMessageChunk| 
        Box::pin(handler(chunk)) as Pin<Box<dyn std::future::Future<Output = CandleMessageChunk> + Send>>;
    Arc::new(wrapped)
}
```

Reference: [Lines 96-129 in chat.rs](../../packages/candle/src/builders/agent_role/chat.rs)

#### 4. `chat/memory_ops.rs` (~200 lines)
**Purpose:** All memory-related operations

**Contains:**
- `initialize_memory_manager()` - Database setup and connection
- `load_context_into_memory()` - Load files/directories into memory
- `search_memory_with_timeout()` - Memory retrieval with timeout
- `store_conversation_memory()` - Save conversation to memory
- `create_memory_node()` - Memory node construction

**Key functions to create:**
```rust
pub(super) async fn initialize_memory_manager(
    emb_model: TextEmbeddingModel,
) -> Result<Arc<dyn MemoryManager>, String> {
    // Database path setup
    // SurrealDB connection
    // Manager initialization
    // Schema initialization
}

pub(super) async fn load_context_into_memory(
    memory: &Arc<dyn MemoryManager>,
    context_file: Option<CandleContext<CandleFile>>,
    context_files: Option<CandleContext<CandleFiles>>,
    context_directory: Option<CandleContext<CandleDirectory>>,
    context_github: Option<CandleContext<CandleGithub>>,
) -> Result<(), String> {
    // Document loading
    // Memory ingestion
}

pub(super) async fn search_memory_with_timeout(
    memory: &Arc<dyn MemoryManager>,
    query: &str,
    timeout_ms: u64,
) -> Option<Vec<MemoryNode>> {
    // Timeout handling
    // Memory search execution
}
```

**Extract from:** Lines 200-400 and 750-790 in chat() method

Reference: [chat.rs lines with memory operations](../../packages/candle/src/builders/agent_role/chat.rs)

#### 5. `chat/chat_orchestration.rs` (~400 lines)
**Purpose:** Main chat() method orchestration logic

**Contains:**
- CandleChatLoop processing (Break, UserPrompt, Reprompt)
- Tool router initialization
- Prompt building with memory context
- Streaming coordination
- Tool execution loop
- Chunk handler invocation
- on_conversation_turn handler invocation

**Key function signature:**
```rust
pub(super) fn execute_chat<F, Fut>(
    builder: CandleAgentBuilderImpl,
    handler: F,
) -> Result<Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>, AgentError>
where
    F: FnOnce(&CandleAgentConversation) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = CandleChatLoop> + Send + 'static,
{
    // Extract builder state
    // Create async stream
    // Call handler to get CandleChatLoop
    // Process based on variant
    // Initialize memory if needed
    // Execute tool-enabled inference
}
```

**Extract from:** Lines 130-790 in chat() method

Reference: [chat.rs chat() method](../../packages/candle/src/builders/agent_role/chat.rs)

#### 6. `chat/simple_chat.rs` (~60 lines)
**Purpose:** Simple message-based chat without full orchestration

**Contains:**
- `chat_with_message()` implementation
- Simple prompt building
- Streaming without tools or memory
- Performance metrics tracking

**Key function signature:**
```rust
pub(super) fn execute_simple_chat(
    builder: CandleAgentBuilderImpl,
    message: String,
) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>> {
    // Build simple prompt
    // Create completion stream
    // Track metrics
    // Apply chunk handler
}
```

**Extract from:** Lines 791-845

Reference: [chat.rs chat_with_message()](../../packages/candle/src/builders/agent_role/chat.rs)

### Transformed `chat.rs` (~80 lines)

The main file becomes a coordination layer:

```rust
//! Chat implementation for CandleAgentBuilder
//!
//! This module coordinates the chat functionality across several focused submodules:
//! - builder_methods: Simple builder pattern setters
//! - handler_registration: Handler wrapping and Arc management
//! - memory_ops: Memory initialization, context loading, and storage
//! - chat_orchestration: Main chat loop with tools and memory
//! - simple_chat: Lightweight chat without full orchestration

mod chat;

use super::*;
use chat::*;

impl CandleAgentBuilder for CandleAgentBuilderImpl {
    fn model(mut self, model: TextToTextModel) -> Self {
        self.text_to_text_model = model;
        self
    }

    // ... other simple delegations ...

    fn chat<F, Fut>(self, handler: F) -> Result<Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>, AgentError>
    where
        F: FnOnce(&CandleAgentConversation) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = CandleChatLoop> + Send + 'static,
    {
        chat::execute_chat(self, handler)
    }

    fn chat_with_message(self, message: impl Into<String>) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>> {
        chat::execute_simple_chat(self, message.into())
    }
}
```

## EXECUTION PLAN

### STEP 1: Create Directory Structure
```bash
mkdir -p /Volumes/samsung_t9/cyrup/packages/candle/src/builders/agent_role/chat
```

### STEP 2: Extract Modules (in order)

1. **Create `chat/mod.rs`**
   - Add module declarations
   - Add re-exports

2. **Create `chat/builder_methods.rs`**
   - Extract lines 4-95 from chat.rs
   - Convert to helper functions or keep as partial impl

3. **Create `chat/handler_registration.rs`**
   - Extract lines 96-129 from chat.rs
   - Create handler wrapping functions

4. **Create `chat/memory_ops.rs`**
   - Extract memory initialization code (from chat() method)
   - Extract context loading code
   - Extract memory search code
   - Extract memory storage code
   - Create helper functions

5. **Create `chat/chat_orchestration.rs`**
   - Extract main chat() implementation
   - Use memory_ops helpers
   - Preserve all logic

6. **Create `chat/simple_chat.rs`**
   - Extract chat_with_message() implementation (lines 791-845)

### STEP 3: Transform Main File

Update `chat.rs` to:
- Add `mod chat;` declaration
- Add `use chat::*;`
- Keep minimal trait impl that delegates to submodules
- Add module documentation

### STEP 4: Verify Compilation

```bash
cd /Volumes/samsung_t9/cyrup/packages/candle
cargo check
```

Fix any import or visibility issues.

## CRITICAL IMPLEMENTATION NOTES

### Import Management
Each submodule needs:
```rust
use super::super::*;  // Access parent module exports
```

### Visibility Rules
Helper functions should be:
- `pub(super)` - Accessible to parent module (chat.rs)
- `pub(crate)` - If needed across the crate

### Preserve All Logic
- Do not refactor or "improve" logic
- Keep exact same async/await patterns
- Maintain error handling as-is
- Preserve comment blocks explaining complex logic

### Common Imports Needed
```rust
use std::sync::Arc;
use std::pin::Pin;
use std::num::NonZeroU64;
use tokio_stream::{Stream, StreamExt};
use surrealdb::engine::any::connect;
use crate::memory::core::manager::surreal::SurrealDBMemoryManager;
use crate::memory::primitives::node::MemoryNode;
use crate::memory::primitives::types::{MemoryContent, MemoryTypeEnum};
use crate::domain::completion::CandleCompletionChunk;
use crate::domain::chat::message::CandleMessageChunk;
use crate::domain::tool::SweetMcpRouter;
```

## DEFINITION OF DONE

✅ **File Structure:**
- [ ] `chat/` directory created under `agent_role/`
- [ ] Six new files created (mod.rs, builder_methods.rs, handler_registration.rs, memory_ops.rs, chat_orchestration.rs, simple_chat.rs)
- [ ] Main `chat.rs` transformed to ~80 lines

✅ **Module Sizes:**
- [ ] `chat/mod.rs` < 30 lines
- [ ] `chat/builder_methods.rs` < 150 lines
- [ ] `chat/handler_registration.rs` < 80 lines
- [ ] `chat/memory_ops.rs` < 250 lines
- [ ] `chat/chat_orchestration.rs` < 450 lines
- [ ] `chat/simple_chat.rs` < 80 lines
- [ ] Main `chat.rs` < 100 lines

✅ **Functionality:**
- [ ] All builder methods work identically
- [ ] Handler registration works identically
- [ ] Memory initialization works identically
- [ ] Context loading works identically
- [ ] Tool calling works identically
- [ ] Streaming works identically
- [ ] Public API unchanged

✅ **Compilation:**
- [ ] `cargo check` passes without errors
- [ ] No new warnings introduced
- [ ] All imports resolved correctly

✅ **Code Quality:**
- [ ] Each module has clear purpose
- [ ] Module-level documentation added
- [ ] Related code grouped together
- [ ] No code duplication
- [ ] Visibility (pub/pub(super)/private) correct

## SUCCESS CRITERIA

This task succeeds when:

1. **Original file decomposed**: The 807-line monolith is split into 6 focused modules
2. **Each module manageable**: No module exceeds 300 lines
3. **Public API preserved**: External code using this builder sees no changes
4. **Compilation succeeds**: `cargo check` passes without errors
5. **Code more maintainable**: Each module has single responsibility
6. **Zero behavior changes**: All functionality works exactly as before

## WHAT NOT TO DO

❌ **Do not** write unit tests or integration tests  
❌ **Do not** write benchmarks or performance tests  
❌ **Do not** add extensive documentation beyond module-level comments  
❌ **Do not** refactor or improve the logic - preserve as-is  
❌ **Do not** change the public API  
❌ **Do not** add new features or capabilities  
❌ **Do not** modify related files (helpers.rs, traits.rs, etc.)  
❌ **Do not** reorganize imports beyond what's necessary  

## REFERENCES

- Current file: [packages/candle/src/builders/agent_role/chat.rs](../../packages/candle/src/builders/agent_role/chat.rs)
- Trait definition: [packages/candle/src/builders/agent_role/traits.rs](../../packages/candle/src/builders/agent_role/traits.rs)
- Helper types: [packages/candle/src/builders/agent_role/helpers.rs](../../packages/candle/src/builders/agent_role/helpers.rs)
- Module exports: [packages/candle/src/builders/agent_role/mod.rs](../../packages/candle/src/builders/agent_role/mod.rs)
