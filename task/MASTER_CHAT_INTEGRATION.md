# MASTER: Chat System Integration - RESEARCH & ANALYSIS

**Objective**: Integrate builders with domain/chat (24,390 lines) by moving orchestration logic to the right architectural layer.

---

## Core Problem Statement

**Current State**:
- `builders/agent_role/chat/chat_orchestration.rs` (340 lines) contains full chat session orchestration
- `domain/chat/` (24,390 lines) has types, helpers, and systems but NO session executor
- Builders reimplement orchestration instead of delegating to domain layer

**Goal**: 
- MOVE orchestration logic from builders to domain/chat
- Builders become thin delegation layer
- Domain/chat becomes the reusable orchestration core

**NOT DUPLICATION**: We're MOVING existing code to the right place, not creating copies.

---

## What domain/chat ALREADY Provides (Verified)

###  1. Core Types (`/Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/`)

**Conversation Management** (`conversation/mod.rs` - 507 lines):
```rust
pub struct CandleStreamingConversation {
    messages: Vec<CandleImmutableMessage>,
    sequence_counter: AtomicUsize,
    // Atomic counters for stats
}

// Already implemented:
- add_user_message()
- add_assistant_message() 
- add_system_message()
- messages()
- latest_user_message()
```

**Configuration** (`config/` - 7 files):
```rust
// config/model.rs
pub struct CandleModelConfig {
    pub provider: String,
    pub registry_key: String,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
    pub system_prompt: Option<String>,
    pub enable_functions: bool,
    pub timeout_ms: u64,
    // + retry_config, performance config
}

// config/types.rs
pub struct CandleChatConfig {
    pub max_message_length: usize,
    pub enable_history: bool,
    pub enable_streaming: bool,
    pub personality: CandlePersonalityConfig,
}
```

**Message Types** (`message/mod.rs`):
```rust
pub enum CandleMessageRole { User, Assistant, System }
pub enum CandleMessageChunk {
    Text(String),
    Complete { text, finish_reason, usage, ... },
    ToolCallStart { id, name },
    ToolCall { id, name, partial_input },
    ToolCallComplete { id, name, input },
    Error(String),
}
```

### 2. Helper Systems

**Orchestration Helpers** (`orchestration.rs` - 254 lines):
```rust
// Already implemented:
pub fn format_tools_for_selection(tools: &[ToolInfo]) -> String
pub fn format_tools_openai(tools: &[ToolInfo]) -> Result<String>
pub fn render_stage1_prompt(user_input: &str, tools: &[ToolInfo]) -> Result<String>
pub fn render_stage2_prompt(user_input: &str, tools: &[ToolInfo]) -> Result<String>
pub fn render_stage4_prompt(user_message: &str, tool_calls, results) -> Result<String>
pub fn parse_tool_selection_response(json_str: &str) -> Result<ToolSelectionResponse>
pub fn parse_function_call_response(json_str: &str) -> Result<OpenAIFunctionCallResponse>
```

**Templates** (`templates/` - ~1,500 lines):
```rust
pub fn render_template(name: &str, variables: &HashMap<String, String>) -> TemplateResult<String>
pub fn store_template(template: ChatTemplate) -> TemplateResult<()>
pub fn get_template(name: &str) -> Option<ChatTemplate>
```

**Commands** (`commands/` - slash command system):
- Command parsing and execution
- Not relevant to chat sessions

**Search** (`search/` - ~2,000 lines):
- History search and indexing
- Not used in current orchestration

### 3. What's MISSING

**No Session Executor**: domain/chat has NO function that:
1. Takes user input + memory + tools + config
2. Searches memory for context
3. Builds full prompt
4. Calls provider with streaming
5. Executes tools
6. Stores conversation to memory
7. Handles recursion via on_conversation_turn

**This orchestration exists in**: `builders/agent_role/chat/chat_orchestration.rs`

---

## Current Architecture (BEFORE)

```
User → CLI
       ↓
    Registry (provides TextToTextModel with CandleModelInfo)
       ↓
    Builders (agent_role/agent_builder.rs)
       ↓
    Builders.chat() calls chat_orchestration.rs (340 lines)
       ├─ Initializes memory (memory_ops.rs)
       ├─ Loads context into memory
       ├─ Searches memory
       ├─ Builds prompt
       ├─ Calls provider
       ├─ Streams chunks
       ├─ Executes tools
       ├─ Stores to memory
       └─ Handles recursion
```

**Problem**: All orchestration logic is in builders layer, can't be reused.

---

## Target Architecture (AFTER)

```
User → CLI
       ↓
    Registry (provides TextToTextModel with CandleModelInfo)
       ↓
    Builders (agent_role/agent_builder.rs)
       ├─ build_model_config() - merges model.info() + overrides
       ├─ build_chat_config() 
       └─ chat() - delegates to domain::chat (thin wrapper ~70 lines)
       ↓
    domain::chat::session::execute_chat_session() (moved from builders)
       ├─ Uses CandleStreamingConversation
       ├─ Uses memory operations (import from builders/memory_ops.rs)
       ├─ Uses orchestration helpers
       ├─ Uses templates
       └─ Returns stream
```

**Result**: Orchestration is reusable, builders are thin delegation layer.

---

## Implementation Plan (NO DUPLICATION)

### Step 1: Move Orchestration to domain/chat

**Action**: Take `builders/agent_role/chat/chat_orchestration.rs` and refactor it into `domain/chat/session.rs`

**Changes needed**:
1. Instead of taking `CandleAgentBuilderImpl`, take individual parameters:
   - `CandleModelConfig` (built from model.info() + overrides)
   - `CandleChatConfig`
   - `provider: TextToTextModel`
   - `memory: Option<Arc<dyn MemoryManager>>`
   - `tools: Vec<ToolInfo>`
   - `handler: F` (the user's chat loop handler)
   - `on_chunk_handler`, `on_tool_result_handler`, `on_conversation_turn_handler`

2. Import memory operations from `builders/agent_role/chat/memory_ops.rs`:
   - `search_memory_with_timeout()`
   - `format_memory_context()`
   - `create_user_memory()` / `create_assistant_memory()`

3. Use domain/chat types:
   - `CandleMessageChunk`
   - `CandleMessageRole`
   - `CandleChatLoop`
   - `CandlePrompt`
   - `CandleCompletionChunk`, `CandleCompletionParams`

**Result**: `domain/chat/session.rs` with ~350 lines (slight reduction from removing builder extraction code)

### Step 2: Add Config Builders to agent_builder.rs

**File**: `builders/agent_role/agent_builder.rs`

**Add methods** (~50 lines total):

```rust
impl CandleAgentBuilderImpl {
    /// Build CandleModelConfig from model.info() + builder overrides
    pub(crate) fn build_model_config(&self) -> CandleModelConfig {
        let model_info = self.text_to_text_model.info();
        
        CandleModelConfig {
            provider: model_info.provider_str().to_string(),
            registry_key: model_info.registry_key.to_string(),
            temperature: self.temperature as f32,
            max_tokens: Some(self.max_tokens as u32),
            system_prompt: Some(self.system_prompt.clone()),
            enable_functions: !self.tools.is_empty(),
            timeout_ms: self.memory_read_timeout,
            retry_config: CandleModelRetryConfig::default(),
            performance: CandleModelPerformanceConfig {
                enable_streaming: model_info.supports_streaming,
                ..Default::default()
            },
            ..Default::default()
        }
    }
    
    /// Build CandleChatConfig
    pub(crate) fn build_chat_config(&self) -> CandleChatConfig {
        CandleChatConfig {
            max_message_length: 100_000,
            enable_history: !self.conversation_history.is_empty(),
            enable_streaming: true,
            personality: CandlePersonalityConfig::default(),
        }
    }
}
```

### Step 3: Rewrite builders/agent_role/chat/mod.rs

**Action**: Replace current `chat()` method with thin delegation (~70 lines)

```rust
fn chat<F, Fut>(self, handler: F) -> Result<Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>, AgentError>
where
    F: FnOnce(&CandleAgentConversation) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = CandleChatLoop> + Send + 'static,
{
    // Build configs
    let model_config = self.build_model_config();
    let chat_config = self.build_chat_config();
    
    // Extract state
    let provider = self.text_to_text_model;
    let embedding_model = self.text_embedding_model;
    let context_file = self.context_file;
    let context_files = self.context_files;
    let context_directory = self.context_directory;
    let context_github = self.context_github;
    let tools = self.tools;
    let metadata = self.metadata;
    let on_chunk_handler = self.on_chunk_handler;
    let on_tool_result_handler = self.on_tool_result_handler;
    let on_conversation_turn_handler = self.on_conversation_turn_handler;
    
    Ok(Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
        // Initialize memory if embedding model available
        let memory = if let Some(ref emb) = embedding_model {
            match memory_ops::initialize_memory_manager(emb).await {
                Ok(mem) => {
                    // Load context
                    if let Err(e) = memory_ops::load_context_into_memory(
                        &mem,
                        context_file,
                        context_files,
                        context_directory,
                        context_github,
                    ).await {
                        let _ = sender.send(CandleMessageChunk::Error(e));
                        return;
                    }
                    Some(mem)
                }
                Err(e) => {
                    let _ = sender.send(CandleMessageChunk::Error(e));
                    return;
                }
            }
        } else {
            None
        };
        
        // DELEGATE to domain::chat
        let session_stream = crate::domain::chat::session::execute_chat_session(
            model_config,
            chat_config,
            provider,
            memory,
            tools.into(),
            metadata,
            handler,
            on_chunk_handler,
            on_tool_result_handler,
            on_conversation_turn_handler,
        );
        
        tokio::pin!(session_stream);
        while let Some(chunk) = session_stream.next().await {
            let _ = sender.send(chunk);
        }
    })))
}
```

### Step 4: Update domain/chat/mod.rs

**Add**:
```rust
pub mod session;

pub use session::execute_chat_session;
```

### Step 5: Delete chat_orchestration.rs

**After** session.rs is working and chat/mod.rs delegates to it:
- Delete `builders/agent_role/chat/chat_orchestration.rs`
- Remove from `builders/agent_role/chat/mod.rs`: `mod chat_orchestration;`

---

## Definition of Done

### Code Changes

- [ ] `domain/chat/session.rs` created (moved/refactored from chat_orchestration.rs)
  - `execute_chat_session()` function with proper signature
  - Imports memory_ops functions from builders
  - Uses domain/chat types throughout
  - ~350 lines, production quality
  
- [ ] `builders/agent_role/agent_builder.rs` modified
  - `build_model_config()` method added
  - `build_chat_config()` method added
  - ~50 lines added
  
- [ ] `builders/agent_role/chat/mod.rs` modified
  - `chat()` method rewritten to delegate
  - ~70 lines (down from current implementation)
  - Calls `domain::chat::session::execute_chat_session()`
  
- [ ] `domain/chat/mod.rs` modified
  - `pub mod session;` added
  - `pub use session::execute_chat_session;` added
  
- [ ] `builders/agent_role/chat/chat_orchestration.rs` deleted
  - After verifying session.rs works
  - Remove mod declaration

### Quality Standards

- [ ] No `unwrap()` or `expect()` - proper error handling throughout
- [ ] No stubs - all code fully implemented
- [ ] No duplication - orchestration exists once in domain/chat/session.rs
- [ ] Compiles without errors: `cargo check` passes
- [ ] Existing tests pass (if any)

### Functional Verification

- [ ] CLI still works end-to-end
- [ ] Model configs merge correctly (model.info() + builder overrides)
- [ ] Memory integration works (if embedding model provided)
- [ ] Tools execute
- [ ] Handlers fire correctly
- [ ] Streaming works

---

## File Paths

### Files to READ:
- Current orchestration: [`/Volumes/samsung_t9/cyrup/packages/candle/src/builders/agent_role/chat/chat_orchestration.rs`](../packages/candle/src/builders/agent_role/chat/chat_orchestration.rs)
- Memory ops: [`/Volumes/samsung_t9/cyrup/packages/candle/src/builders/agent_role/chat/memory_ops.rs`](../packages/candle/src/builders/agent_role/chat/memory_ops.rs)
- Builder: [`/Volumes/samsung_t9/cyrup/packages/candle/src/builders/agent_role/agent_builder.rs`](../packages/candle/src/builders/agent_role/agent_builder.rs)

### Files to CREATE:
- Session executor: `/Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/session.rs` (NEW - moved from chat_orchestration.rs)

### Files to MODIFY:
- Builder methods: `/Volumes/samsung_t9/cyrup/packages/candle/src/builders/agent_role/agent_builder.rs`
- Builder chat: `/Volumes/samsung_t9/cyrup/packages/candle/src/builders/agent_role/chat/mod.rs`
- Domain exports: `/Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/mod.rs`

### Files to DELETE:
- Orchestration: `/Volumes/samsung_t9/cyrup/packages/candle/src/builders/agent_role/chat/chat_orchestration.rs` (AFTER session.rs works)

---

## Key Principles

1. **MOVE, DON'T DUPLICATE**: chat_orchestration.rs moves to domain/chat/session.rs
2. **USE EXISTING CODE**: domain/chat types, memory_ops functions, orchestration helpers
3. **THIN BUILDERS**: Builders just build configs and delegate
4. **REUSABLE CORE**: domain/chat becomes the orchestration layer
5. **PRODUCTION QUALITY**: No unwrap(), no expect(), no stubs