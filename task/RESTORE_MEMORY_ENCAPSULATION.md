# RESTORE_MEMORY_ENCAPSULATION.md

## Task: Migrate Message Tags to Namespaced Format and Add System Message Support

### Priority: HIGH
**Category**: Architecture Improvement + Bug Fix  
**Estimated Effort**: Medium (2-3 hours)  
**Status**: Not Started

---

## Problem Statement

### Issue 1: Inconsistent Tag Naming Convention
Current message tags use flat naming without namespace:
- `"user_message"` - User messages
- `"assistant_response"` - Assistant responses

**Should be**:
- `"message_type.user"`
- `"message_type.assistant"`
- `"message_type.system"`
- Future: `"message_type.tool"`

### Issue 2: Missing System Message Storage
**Critical gap**: System prompts are constructed dynamically but **NEVER stored in memory**.

**Evidence**:
- `session.rs:206-233`: `build_prompt_with_context()` creates system prompt with personality traits
- `session.rs:399-448`: `store_conversation_in_memory()` only stores user and assistant messages
- `execution.rs:622-667`: `retrieve_conversation_messages()` only filters for user/assistant tags
- **Result**: Exported conversations and memory history are missing system context

**Infrastructure exists**:
- `CandleMessageRole` enum already has `System` variant (message/mod.rs:36)
- Memory system supports arbitrary tags via `MemoryFilter`

---

## Root Cause Analysis

### Current Message Flow

1. **System Prompt Construction** (session.rs:206-233)
   ```rust
   fn build_prompt_with_context(...) -> String {
       let mut system_prompt = model_config.system_prompt.clone().unwrap_or_default();
       // Adds personality, traits, memory context
       // Returns concatenated prompt string
   }
   ```
   ❌ System prompt is built but discarded after use

2. **Message Storage** (session.rs:399-448)
   ```rust
   fn store_conversation_in_memory(...) {
       let user_meta = MemoryMetadata {
           tags: vec!["user_message".to_string()],  // ❌ Flat naming
           // ...
       };
       let assistant_meta = MemoryMetadata {
           tags: vec!["assistant_response".to_string()],  // ❌ Flat naming
           // ...
       };
       // ❌ No system message storage
   }
   ```

3. **Message Retrieval** (execution.rs:622-667)
   ```rust
   async fn retrieve_conversation_messages(...) {
       let filter = MemoryFilter::new()
           .with_tags(vec![
               "user_message".to_string(),      // ❌ Flat naming
               "assistant_response".to_string(), // ❌ Flat naming
           ]);
       // ❌ System messages not retrieved
   }
   ```

### Impact
- ✗ Incomplete conversation exports (missing system context)
- ✗ Lost configuration history (can't see what system prompt was used)
- ✗ Inconsistent tag naming makes filtering harder to maintain
- ✗ Can't distinguish message types from other tag categories

---

## Solution Design

### Phase 1: Update Tag Naming Convention

#### File 1: `session.rs`
**Location**: `/Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/session.rs`

**Change 1** (Line 411):
```rust
// OLD
tags: vec!["user_message".to_string()],

// NEW
tags: vec!["message_type.user".to_string()],
```

**Change 2** (Line 421):
```rust
// OLD
tags: vec!["assistant_response".to_string()],

// NEW
tags: vec!["message_type.assistant".to_string()],
```

#### File 2: `execution.rs`
**Location**: `/Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/commands/execution.rs`

**Change 1** (Lines 630-632):
```rust
// OLD
let filter = MemoryFilter::new()
    .with_tags(vec![
        "user_message".to_string(),
        "assistant_response".to_string(),
    ]);

// NEW
let filter = MemoryFilter::new()
    .with_tags(vec![
        "message_type.user".to_string(),
        "message_type.assistant".to_string(),
        "message_type.system".to_string(),
    ]);
```

**Change 2** (Lines 643-647):
```rust
// OLD
let role = if mem.metadata.tags.iter().any(|t| t.as_ref() == "user_message") {
    CandleMessageRole::User
} else {
    CandleMessageRole::Assistant
};

// NEW
let role = if mem.metadata.tags.iter().any(|t| t.as_ref() == "message_type.user") {
    CandleMessageRole::User
} else if mem.metadata.tags.iter().any(|t| t.as_ref() == "message_type.system") {
    CandleMessageRole::System
} else {
    CandleMessageRole::Assistant
};
```

---

### Phase 2: Add System Message Storage

#### Refactor `store_conversation_in_memory()`

**Current signature** (session.rs:399):
```rust
fn store_conversation_in_memory<S: std::hash::BuildHasher>(
    user_message: &str,
    assistant_response: &str,
    memory: &Arc<MemoryCoordinator>,
    metadata: &HashMap<String, String, S>,
)
```

**New signature**:
```rust
fn store_conversation_in_memory<S: std::hash::BuildHasher>(
    system_prompt: &str,
    user_message: &str,
    assistant_response: &str,
    memory: &Arc<MemoryCoordinator>,
    metadata: &HashMap<String, String, S>,
)
```

**Implementation**:
```rust
fn store_conversation_in_memory<S: std::hash::BuildHasher>(
    system_prompt: &str,
    user_message: &str,
    assistant_response: &str,
    memory: &Arc<MemoryCoordinator>,
    metadata: &HashMap<String, String, S>,
) {
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
        tags: vec![], // Will be set per message type
    };

    // Store system message
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

    // Store user message
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

    // Store assistant message
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

#### Update Call Site

**Location**: `session.rs:605`

**Change**:
```rust
// OLD
if !assistant_response.is_empty() {
    store_conversation_in_memory(&user_message, &assistant_response, memory, metadata);
}

// NEW
if !assistant_response.is_empty() {
    // Extract system prompt from build_prompt_with_context
    let system_prompt = {
        let mut sp = model_config.system_prompt.clone().unwrap_or_default();
        if let Some(custom) = &chat_config.personality.custom_instructions {
            sp.push_str("\n\n");
            sp.push_str(custom);
        }
        let _ = write!(
            sp,
            "\n\nPersonality: {} (creativity: {:.1}, formality: {:.1}, empathy: {:.1})",
            chat_config.personality.personality_type,
            chat_config.personality.creativity,
            chat_config.personality.formality,
            chat_config.personality.empathy
        );
        sp
    };
    
    store_conversation_in_memory(
        &system_prompt,
        &user_message,
        &assistant_response,
        memory,
        metadata
    );
}
```

---

## Files Modified Summary

| File | Lines Modified | Description |
|------|----------------|-------------|
| `session.rs` | 411, 421, 399-448, 605 | Update tags, refactor storage function, add system message |
| `execution.rs` | 630-632, 643-647 | Update filter tags, add System role mapping |

**No changes needed**:
- `orchestration.rs:130` - Uses `"user_message"` as template variable, not a tag

---

## Testing Plan

### 1. Compilation Test
```bash
cargo check --package paraphym_candle
```
**Expected**: No errors

### 2. Functional Test - Message Storage
**Test case**: Create a conversation with system prompt, user message, assistant response

**Verification**:
```rust
// Query all conversation messages
let filter = MemoryFilter::new()
    .with_tags(vec![
        "message_type.user".to_string(),
        "message_type.assistant".to_string(),
        "message_type.system".to_string(),
    ]);
let messages = memory.get_memories(filter).await?;

// Should contain 3 messages per turn:
assert!(messages.iter().any(|m| m.metadata.tags.contains(&"message_type.system".to_string())));
assert!(messages.iter().any(|m| m.metadata.tags.contains(&"message_type.user".to_string())));
assert!(messages.iter().any(|m| m.metadata.tags.contains(&"message_type.assistant".to_string())));
```

### 3. Export Test
```bash
# Run export command
/export --format json --output test_export.json
```

**Expected output structure**:
```json
{
  "messages": [
    {
      "role": "system",
      "content": "You are a helpful assistant. Personality: ...",
      "id": "...",
      "timestamp": 1234567890
    },
    {
      "role": "user",
      "content": "Hello",
      "id": "...",
      "timestamp": 1234567891
    },
    {
      "role": "assistant",
      "content": "Hi there!",
      "id": "...",
      "timestamp": 1234567892
    }
  ]
}
```

---

## Migration Considerations

### Backward Compatibility
**Question**: What happens to existing messages with old tag format?

**Options**:
1. **Keep old messages as-is** (they won't be retrieved with new filter)
2. **Run migration script** to update old tags
3. **Support both formats** during transition period

**Recommendation**: Option 1 (clean break) since this is early development.

### Future Extensions
- Add `"message_type.tool"` for tool call results
- Add `"message_type.error"` for error messages
- Consider `"message_type.system.initial"` vs `"message_type.system.update"` to track prompt changes

---

## Acceptance Criteria

✅ **Phase 1 Complete** when:
- [ ] All tags use `message_type.*` namespace
- [ ] Compilation succeeds with no errors
- [ ] Retrieval logic handles all three message types

✅ **Phase 2 Complete** when:
- [ ] System messages are stored with each conversation turn
- [ ] Exports include system messages with correct role
- [ ] Memory queries return system messages correctly sorted

✅ **Task Complete** when:
- [ ] All acceptance criteria met
- [ ] Manual testing confirms correct behavior
- [ ] This task file deleted (or moved to `task/completed/`)

---

## References

**Code Locations**:
- Message role enum: `packages/candle/src/domain/chat/message/mod.rs:32-43`
- Tag creation: `packages/candle/src/domain/chat/session.rs:399-448`
- Tag filtering: `packages/candle/src/domain/chat/commands/execution.rs:622-667`
- System prompt building: `packages/candle/src/domain/chat/session.rs:206-233`

**Related Issues**:
- STUB_6.md - Export command implementation (recently completed)
- Memory architecture uses public `MemoryFilter` API

**Documentation**:
- MemoryFilter: `packages/candle/src/memory/core/ops/filter.rs:21-94`
- MemoryMetadata: `packages/candle/src/memory/mod.rs` (check struct definition)
