# Task: Migrate Message Tags to Namespaced Format and Add System Message Storage

## Core Objective

**Primary Goal**: Migrate conversation message tags from flat naming to hierarchical namespace format AND implement missing system message storage.

**Why This Matters**:
- System prompts are currently built but NEVER stored in memory
- Exported conversations are incomplete (missing system context)
- Tag naming is inconsistent and doesn't scale for future message types
- Cannot reconstruct the full conversation history including system instructions

---

## Current State Analysis

### Message Flow Architecture

```
User Input → build_prompt_with_context() → Provider → Assistant Response
                        ↓
              [System prompt DISCARDED]
                        ↓
         store_conversation_in_memory()
                        ↓
              ONLY stores user + assistant
```

### Tag Usage Pattern Discovery

**Context Tags** (existing pattern):
- `"context_file"` - loaded from file context
- `"context_directory"` - loaded from directory  
- `"context_github"` - loaded from GitHub
- Pattern: `context_{source}` with underscores

**Message Tags** (current - INCORRECT):
- `"user_message"` - user messages
- `"assistant_response"` - assistant responses
- `"system_message"` - **MISSING** (never stored)
- Pattern: Flat with underscores, inconsistent

**Message Tags** (target - CORRECT):
- `"message_type.user"` - user messages
- `"message_type.assistant"` - assistant responses
- `"message_type.system"` - system prompts
- `"message_type.tool"` - tool responses (future)
- Pattern: `message_type.{role}` with dots for hierarchy

---

## File Locations & References

### Core Files to Modify

1. **[`packages/candle/src/domain/chat/session.rs`](../../packages/candle/src/domain/chat/session.rs)**
   - Lines 206-233: `build_prompt_with_context()` - builds system prompt
   - Lines 399-448: `store_conversation_in_memory()` - storage function
   - Line 411: User message tag
   - Line 421: Assistant message tag  
   - Line 605: Call site for storage

2. **[`packages/candle/src/domain/chat/commands/execution.rs`](../../packages/candle/src/domain/chat/commands/execution.rs)**
   - Lines 622-667: `retrieve_conversation_messages()` - retrieval function
   - Lines 630-632: Tag filter
   - Lines 643-647: Role determination logic

### Supporting Infrastructure (Already Exists)

3. **[`packages/candle/src/domain/chat/message/mod.rs`](../../packages/candle/src/domain/chat/message/mod.rs)**
   - Lines 32-43: `CandleMessageRole` enum with System, User, Assistant, Tool variants
   - Already supports all four message types ✅

4. **[`packages/candle/src/memory/core/primitives/metadata.rs`](../../packages/candle/src/memory/core/primitives/metadata.rs)**
   - Line 23: `pub tags: Vec<String>` - where tags are stored
   - Lines 10-49: Complete MemoryMetadata struct definition

5. **[`packages/candle/src/memory/core/ops/filter.rs`](../../packages/candle/src/memory/core/ops/filter.rs)**
   - Public MemoryFilter API with `with_tags()` builder method
   - Already supports tag-based filtering ✅

---

## Implementation Plan

### Step 1: Extract System Prompt Builder (NEW FUNCTION)

**Location**: `packages/candle/src/domain/chat/session.rs`  
**Insert BEFORE**: Line 206 (before `build_prompt_with_context`)

```rust
/// Build system prompt with personality traits and custom instructions
fn build_system_prompt(
    model_config: &CandleModelConfig,
    chat_config: &CandleChatConfig,
) -> String {
    let mut system_prompt = model_config.system_prompt.clone().unwrap_or_default();

    if let Some(custom) = &chat_config.personality.custom_instructions {
        system_prompt.push_str("\n\n");
        system_prompt.push_str(custom);
    }

    let _ = write!(
        system_prompt,
        "\n\nPersonality: {} (creativity: {:.1}, formality: {:.1}, empathy: {:.1})",
        chat_config.personality.personality_type,
        chat_config.personality.creativity,
        chat_config.personality.formality,
        chat_config.personality.empathy
    );

    system_prompt
}
```

**Purpose**: Extracts system prompt construction into reusable function for both prompt building and storage.

---

### Step 2: Refactor build_prompt_with_context()

**Location**: `packages/candle/src/domain/chat/session.rs:206-233`  
**Action**: REPLACE entire function

**OLD CODE**:
```rust
fn build_prompt_with_context(
    model_config: &CandleModelConfig,
    chat_config: &CandleChatConfig,
    memory_context: &str,
    user_message: &str,
) -> String {
    let mut system_prompt = model_config.system_prompt.clone().unwrap_or_default();

    if let Some(custom) = &chat_config.personality.custom_instructions {
        system_prompt.push_str("\n\n");
        system_prompt.push_str(custom);
    }

    let _ = write!(
        system_prompt,
        "\n\nPersonality: {} (creativity: {:.1}, formality: {:.1}, empathy: {:.1})",
        chat_config.personality.personality_type,
        chat_config.personality.creativity,
        chat_config.personality.formality,
        chat_config.personality.empathy
    );

    if memory_context.is_empty() {
        format!("{system_prompt}\n\nUser: {user_message}")
    } else {
        format!("{system_prompt}\n\n{memory_context}\n\nUser: {user_message}")
    }
}
```

**NEW CODE**:
```rust
fn build_prompt_with_context(
    model_config: &CandleModelConfig,
    chat_config: &CandleChatConfig,
    memory_context: &str,
    user_message: &str,
) -> String {
    let system_prompt = build_system_prompt(model_config, chat_config);

    if memory_context.is_empty() {
        format!("{system_prompt}\n\nUser: {user_message}")
    } else {
        format!("{system_prompt}\n\n{memory_context}\n\nUser: {user_message}")
    }
}
```

**Purpose**: Delegates system prompt construction to dedicated function, simplifies logic.

---

### Step 3: Update store_conversation_in_memory() Signature

**Location**: `packages/candle/src/domain/chat/session.rs:399`  
**Action**: REPLACE function signature and add system_prompt parameter

**OLD SIGNATURE** (line 399):
```rust
fn store_conversation_in_memory<S: std::hash::BuildHasher>(
    user_message: &str,
    assistant_response: &str,
    memory: &Arc<MemoryCoordinator>,
    metadata: &HashMap<String, String, S>,
)
```

**NEW SIGNATURE**:
```rust
fn store_conversation_in_memory<S: std::hash::BuildHasher>(
    system_prompt: &str,
    user_message: &str,
    assistant_response: &str,
    memory: &Arc<MemoryCoordinator>,
    metadata: &HashMap<String, String, S>,
)
```

---

### Step 4: Update store_conversation_in_memory() Implementation

**Location**: `packages/candle/src/domain/chat/session.rs:399-448`  
**Action**: REPLACE entire function body

**NEW IMPLEMENTATION**:
```rust
fn store_conversation_in_memory<S: std::hash::BuildHasher>(
    system_prompt: &str,
    user_message: &str,
    assistant_response: &str,
    memory: &Arc<MemoryCoordinator>,
    metadata: &HashMap<String, String, S>,
) {
    // Base metadata template
    let base_meta = MemoryMetadata {
        user_id: metadata.get("user_id").cloned(),
        agent_id: metadata.get("agent_id").cloned(),
        context: "chat".to_string(),
        importance: 0.8,
        keywords: vec![],
        category: "conversation".to_string(),
        source: Some("chat".to_string()),
        created_at: chrono::Utc::now(),
        last_accessed_at: None,
        embedding: None,
        custom: serde_json::Value::Object(serde_json::Map::new()),
        tags: vec![], // Set per message type below
    };

    // Store SYSTEM message
    if !system_prompt.is_empty() {
        let system_meta = MemoryMetadata {
            tags: vec!["message_type.system".to_string()],
            ..base_meta.clone()
        };

        let memory_clone = memory.clone();
        let system_msg = system_prompt.to_string();
        tokio::spawn(async move {
            if let Err(e) = memory_clone.add_memory(
                system_msg,
                DomainMemoryTypeEnum::Semantic,
                Some(system_meta)
            ).await {
                log::error!("Failed to store system memory: {e:?}");
            }
        });
    }

    // Store USER message
    let user_meta = MemoryMetadata {
        tags: vec!["message_type.user".to_string()],
        ..base_meta.clone()
    };

    let memory_clone = memory.clone();
    let user_msg = user_message.to_string();
    tokio::spawn(async move {
        if let Err(e) = memory_clone.add_memory(
            user_msg,
            DomainMemoryTypeEnum::Episodic,
            Some(user_meta)
        ).await {
            log::error!("Failed to store user memory: {e:?}");
        }
    });

    // Store ASSISTANT message
    let assistant_meta = MemoryMetadata {
        tags: vec!["message_type.assistant".to_string()],
        ..base_meta.clone()
    };

    let memory_clone = memory.clone();
    let assistant_msg = assistant_response.to_string();
    tokio::spawn(async move {
        if let Err(e) = memory_clone.add_memory(
            assistant_msg,
            DomainMemoryTypeEnum::Episodic,
            Some(assistant_meta)
        ).await {
            log::error!("Failed to store assistant memory: {e:?}");
        }
    });
}
```

**Key Changes**:
1. Added `system_prompt` parameter
2. Added system message storage with `"message_type.system"` tag
3. Changed user tag from `"user_message"` to `"message_type.user"`
4. Changed assistant tag from `"assistant_response"` to `"message_type.assistant"`
5. Uses `DomainMemoryTypeEnum::Semantic` for system (conceptual), `::Episodic` for user/assistant (conversational)

---

### Step 5: Update Call Site in handle_user_prompt()

**Location**: `packages/candle/src/domain/chat/session.rs:605`  
**Action**: REPLACE the storage call

**OLD CODE** (line 604-606):
```rust
// Store conversation in memory
if !assistant_response.is_empty() {
    store_conversation_in_memory(&user_message, &assistant_response, memory, metadata);
}
```

**NEW CODE**:
```rust
// Store conversation in memory including system prompt
if !assistant_response.is_empty() {
    let system_prompt = build_system_prompt(model_config, chat_config);
    store_conversation_in_memory(
        &system_prompt,
        &user_message,
        &assistant_response,
        memory,
        metadata
    );
}
```

**Purpose**: Builds system prompt and passes it to storage function.

---

### Step 6: Update retrieve_conversation_messages() Filter

**Location**: `packages/candle/src/domain/chat/commands/execution.rs:628-632`  
**Action**: REPLACE tag filter

**OLD CODE**:
```rust
let filter = MemoryFilter::new()
    .with_tags(vec![
        "user_message".to_string(),
        "assistant_response".to_string(),
    ]);
```

**NEW CODE**:
```rust
let filter = MemoryFilter::new()
    .with_tags(vec![
        "message_type.user".to_string(),
        "message_type.assistant".to_string(),
        "message_type.system".to_string(),
    ]);
```

---

### Step 7: Update Role Mapping Logic

**Location**: `packages/candle/src/domain/chat/commands/execution.rs:643-647`  
**Action**: REPLACE role determination

**OLD CODE**:
```rust
// Determine role from tags
let role = if mem.metadata.tags.iter().any(|t| t.as_ref() == "user_message") {
    CandleMessageRole::User
} else {
    CandleMessageRole::Assistant
};
```

**NEW CODE**:
```rust
// Determine role from tags
let role = if mem.metadata.tags.iter().any(|t| t.as_ref() == "message_type.user") {
    CandleMessageRole::User
} else if mem.metadata.tags.iter().any(|t| t.as_ref() == "message_type.system") {
    CandleMessageRole::System
} else if mem.metadata.tags.iter().any(|t| t.as_ref() == "message_type.assistant") {
    CandleMessageRole::Assistant
} else {
    // Fallback for unrecognized tags - treat as Assistant
    CandleMessageRole::Assistant
};
```

**Purpose**: Maps all three message types to correct roles, with fallback.

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    Message Storage Flow                      │
└─────────────────────────────────────────────────────────────┘

User Input
    │
    ├─> build_system_prompt() ──> System Prompt String
    │         │
    │         ├─> model_config.system_prompt
    │         ├─> chat_config.personality.custom_instructions
    │         └─> Personality traits formatting
    │
    ├─> build_prompt_with_context() ──> Full Prompt
    │         │
    │         └─> System Prompt + Memory Context + User Message
    │
    ├─> Provider.prompt() ──> Assistant Response
    │
    └─> store_conversation_in_memory()
              │
              ├─> Store SYSTEM with tag "message_type.system"
              ├─> Store USER with tag "message_type.user"
              └─> Store ASSISTANT with tag "message_type.assistant"

┌─────────────────────────────────────────────────────────────┐
│                  Message Retrieval Flow                      │
└─────────────────────────────────────────────────────────────┘

Export Command
    │
    └─> retrieve_conversation_messages()
              │
              ├─> MemoryFilter.with_tags([
              │     "message_type.user",
              │     "message_type.assistant",
              │     "message_type.system"
              │   ])
              │
              ├─> memory.get_memories(filter)
              │
              └─> Map tags to CandleMessageRole:
                    │
                    ├─> "message_type.user" → User
                    ├─> "message_type.system" → System
                    └─> "message_type.assistant" → Assistant
```

---

## Tag Namespace Design Rationale

### Why Dots Instead of Underscores?

**Context Tags**: `context_file`, `context_directory`, `context_github`
- Flat namespace
- Source-based categorization
- Limited set of values

**Message Type Tags**: `message_type.user`, `message_type.assistant`, `message_type.system`
- Hierarchical namespace
- Role-based categorization
- Extensible for future types: `message_type.tool`, `message_type.error`

**Benefits**:
1. **Clear Separation**: Dots distinguish message types from other tag categories
2. **Scalability**: Can add subtypes like `message_type.system.initial` vs `message_type.system.update`
3. **Query Flexibility**: Can filter by prefix `message_type.*`
4. **Consistency**: Follows common naming patterns (e.g., `log.level.error`)

---

## Verification Steps

### After Implementation

1. **Compilation Check**:
   ```bash
   cargo check --package paraphym_candle
   ```
   Expected: No errors

2. **Manual Verification** (run chat session):
   - Send user message
   - Check SurrealDB for three memories with tags:
     - `message_type.system`
     - `message_type.user`
     - `message_type.assistant`

3. **Export Verification**:
   ```bash
   # Run export command in chat
   /export --format json --output test.json
   ```
   Expected: JSON file contains messages with all three roles

---

## Definition of Done

✅ Task is COMPLETE when:

1. All tag references changed from flat to namespaced format
2. `build_system_prompt()` helper function created
3. `build_prompt_with_context()` refactored to use helper
4. `store_conversation_in_memory()` stores all three message types
5. `retrieve_conversation_messages()` filters and maps all three types
6. Code compiles without errors
7. Manual verification shows system messages in memory
8. Export includes system messages with correct role

**NO TESTING REQUIRED**: This task focuses on implementation correctness verified by compilation and manual inspection.

**NO DOCUMENTATION REQUIRED**: Code is self-documenting with inline comments.

**NO BENCHMARKS REQUIRED**: Performance impact is negligible (one additional memory write per turn).

---

## Migration Notes

### Backward Compatibility

**Question**: What happens to existing messages with old tag format (`"user_message"`, `"assistant_response"`)?

**Answer**: They will NOT be retrieved by the new filter. This is acceptable because:
1. Early development phase (limited production data)
2. Clean break is simpler than dual-format support
3. Old messages remain in database, just not in exports

**If Migration Needed** (future):
```sql
-- SurrealDB migration query (for reference only)
UPDATE memory_node SET metadata.tags = ["message_type.user"] 
  WHERE metadata.tags CONTAINS "user_message";

UPDATE memory_node SET metadata.tags = ["message_type.assistant"] 
  WHERE metadata.tags CONTAINS "assistant_response";
```

### Future Extensions

**Tool Messages** (when implemented):
```rust
// In future: store tool call results
let tool_meta = MemoryMetadata {
    tags: vec!["message_type.tool".to_string()],
    ..base_meta.clone()
};
```

**Error Messages** (optional future enhancement):
```rust
// In future: store error messages
let error_meta = MemoryMetadata {
    tags: vec!["message_type.error".to_string()],
    ..base_meta.clone()
};
```

---

## Files Modified Summary

| File | Lines Changed | Type | Description |
|------|---------------|------|-------------|
| `session.rs` | +27 new | Added | New `build_system_prompt()` function |
| `session.rs` | 206-233 | Refactored | `build_prompt_with_context()` delegates to helper |
| `session.rs` | 399 | Modified | Function signature adds `system_prompt` parameter |
| `session.rs` | 400-448 | Replaced | Complete rewrite with three message types and new tags |
| `session.rs` | 605-609 | Modified | Call site builds system prompt and passes to storage |
| `execution.rs` | 630-632 | Modified | Filter includes `message_type.system` tag |
| `execution.rs` | 643-651 | Replaced | Role mapping handles three message types |

**Total Lines**: ~75 lines changed/added

**No changes needed**:
- `orchestration.rs:130` - Uses `"user_message"` as template variable, not a tag
- `message/mod.rs` - `CandleMessageRole` enum already supports all roles
- `metadata.rs` - MemoryMetadata already has `tags` field
- `filter.rs` - MemoryFilter already supports tag filtering

---

## Related Context

### Recent Work
- **STUB_6.md**: Export command implementation (completed)
  - Export now works with `retrieve_conversation_messages()`
  - This task completes the export feature by adding system messages

### Architecture References
- **Memory System**: See [`packages/candle/src/memory/README.md`](../../packages/candle/src/memory/README.md) (if exists)
- **MemoryFilter API**: Public API at [`filter.rs:21-94`](../../packages/candle/src/memory/core/ops/filter.rs#L21-L94)
- **MemoryMetadata**: Core structure at [`metadata.rs:10-170`](../../packages/candle/src/memory/core/primitives/metadata.rs#L10-L170)

---

## Execution Checklist

- [ ] Create `build_system_prompt()` helper function
- [ ] Refactor `build_prompt_with_context()` to use helper
- [ ] Update `store_conversation_in_memory()` signature
- [ ] Replace `store_conversation_in_memory()` implementation
- [ ] Update call site at session.rs:605
- [ ] Update tag filter in execution.rs:630-632
- [ ] Update role mapping in execution.rs:643-651
- [ ] Run `cargo check` - verify compilation
- [ ] Manual test: verify system messages stored
- [ ] Manual test: verify export includes system messages
- [ ] Delete this task file or move to `task/completed/`
