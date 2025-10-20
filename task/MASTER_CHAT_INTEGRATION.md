# MASTER: Chat System Integration & Deduplication

**Objective**: Builders are configuration collectors (~100 lines). domain/chat (24,390 lines) is the real chat system. Remove all duplication and integrate memory/history properly INTO domain/chat.

---

## Current State Analysis

### Builders (src/builders/agent_role/)

**`chat.rs`**: 845 lines total
- Lines 1-142: Configuration methods (KEEP - this is the builder API)
- Lines 143-844: Chat implementation (DELETE/MOVE - 700+ lines)

**Breakdown of chat() method** (lines 143-844):
- Lines 200-270: Memory initialization with SurrealDB (~70 lines) → **Move to domain/chat/memory**
- Lines 280-380: Context ingestion from files/directories (~100 lines) → **Move to domain/chat/memory**  
- Lines 400-480: Memory retrieval with search_by_content (~80 lines) → **Move to domain/chat/memory**
- Lines 480-550: Prompt construction with history formatting (~70 lines) → **Use domain/chat/templates**
- Lines 550-650: Provider calling and streaming (~100 lines) → **Move to domain/chat/session**
- Lines 650-720: Tool orchestration (~70 lines) → **Use domain/chat/orchestration** (already exists)
- Lines 720-780: Memory storage after conversation (~60 lines) → **Move to domain/chat/memory**
- Lines 780-844: on_conversation_turn recursive handler (~60 lines) → **Use domain/chat/loop**

**Total to delete from builders**: ~700 lines  
**Total to move into domain/chat**: ~300 lines (rest is duplication of existing functionality)

---

### Domain Chat (src/domain/chat/)

**What EXISTS** (24,390 lines, 58 files):

1. **Conversation** (`conversation/mod.rs`)
   - `CandleStreamingConversation` - Immutable messages with atomic sequences
   - `add_user_message()`, `add_assistant_message()`, `add_system_message()`
   - Event streaming for real-time updates

2. **Commands** (`commands/mod.rs`)
   - `parse_candle_command()` - Slash command parsing
   - `execute_candle_command()` - Streaming execution

3. **Orchestration** (`orchestration.rs`) - 254 lines
   - Tool selection, function calling, result interpretation
   - Already handles tool orchestration (lines 650-720 in chat.rs duplicate this)

4. **Templates** (`templates/`) - ~1,500 lines
   - Template rendering with variables

5. **Search** (`search/`) - ~2,000 lines
   - History search and indexing

6. **Config** (`config.rs`) - 1,470 lines
   - `CandleModelConfig` (provider, temperature, max_tokens, system_prompt)
   - `CandleChatConfig` (personality, behavior, UI, history settings)

**What's MISSING**:
- Memory integration entry point
- Chat session coordinator
- Loop execution (CandleChatLoop handling)

---

## Configuration Structure

### CandleModelConfig

```rust
pub struct CandleModelConfig {
    pub provider: String,
    pub registry_key: String,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
    pub system_prompt: Option<String>,
    pub enable_functions: bool,
    pub timeout_ms: u64,
}
```

### CandleChatConfig

```rust
pub struct CandleChatConfig {
    pub max_message_length: usize,
    pub enable_history: bool,
    pub enable_streaming: bool,
    pub personality: CandlePersonalityConfig,
}
```

---

## Task Breakdown (12 Tasks)

### Phase 1: Analysis

#### TASK-01: Audit Builder Chat Logic
**Objective**: Create complete inventory of chat.rs with line numbers and categorization.

#### TASK-02: Map Memory Integration Architecture
**Objective**: Design how memory integrates with domain/chat subsystems.

### Phase 2: Add to domain/chat

#### TASK-03: Create domain/chat/memory.rs Module
**Objective**: Move memory logic FROM builders TO domain/chat (~280 lines).

#### TASK-04: Create domain/chat/session.rs Module
**Objective**: Create main chat session coordinator (~200 lines).

#### TASK-05: Enhance domain/chat/loop.rs
**Objective**: Add handler execution support (~50 lines added to existing 52).

### Phase 3: Configuration

#### TASK-06: Builder to CandleModelConfig Mapping
**Objective**: Add `build_model_config()` method (~20 lines).

#### TASK-07: Builder to CandleChatConfig Mapping
**Objective**: Add `build_chat_config()` method (~30 lines).

### Phase 4: Delete Duplication

#### TASK-08: Delete Builder Chat Implementation
**Objective**: Delete lines 143-844 from chat.rs.

#### TASK-09: Delete Stub Conversation Type
**Objective**: Delete `CandleAgentConversation`, use `CandleStreamingConversation`.

### Phase 5: Wire Together

#### TASK-10: Create Chat Entry Point
**Objective**: Add public entry point to domain/chat/mod.rs.

#### TASK-11: Implement New Builder chat()
**Objective**: Replace 700 lines with 50 lines that call domain/chat.

#### TASK-12: Integration & Cleanup
**Objective**: Ensure everything compiles and works.

---

## Target Implementation

### New chat() method (50 lines)

```rust
fn chat<F, Fut>(self, handler: F) -> Result<...> {
    let model_config = self.build_model_config();
    let chat_config = self.build_chat_config();
    
    let memory = if let Some(emb) = self.text_embedding_model {
        Some(CandleChatMemoryManager::new(emb, db_path).await?)
    } else { None };
    
    let initial_conv = /* build from history */;
    
    let session = domain::chat::start_chat_session(
        model_config,
        chat_config,
        initial_conv,
        memory,
    );
    
    Ok(session.chat_with_loop(handler))
}
```

---

## Success Criteria

### Before
- Builders: 1,123 lines reimplementing chat
- domain/chat: 24,390 lines unused
- Duplication: Massive

### After
- Builders: ~150 lines (config only)
- domain/chat: ~25,000 lines (with memory/session modules)
- Duplication: ZERO

**Net change**: -973 lines in builders, +545 lines in domain/chat, -428 lines total

---

## Key Principles

1. **Builders = Configuration** - NO business logic
2. **domain/chat = All Logic** - Memory, sessions, loops, templates, commands
3. **Memory Integration** - Part of chat session, not separate
4. **Zero Duplication** - One conversation type, one chat system

---

## Research Citations

- [`src/builders/agent_role/chat.rs`](../packages/candle/src/builders/agent_role/chat.rs) - 845 lines
- [`src/domain/chat/config.rs`](../packages/candle/src/domain/chat/config.rs) - 1,470 lines
- [`src/domain/chat/orchestration.rs`](../packages/candle/src/domain/chat/orchestration.rs) - 254 lines
- [`src/domain/chat/conversation/mod.rs`](../packages/candle/src/domain/chat/conversation/mod.rs)
- Codemap: Candle Chat Module - Core Functionality Flows

---

## Next Steps

Start with TASK-01 to create detailed audit of chat.rs showing exactly what moves where.
