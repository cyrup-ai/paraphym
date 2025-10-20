# FIX: conversation_history() No-Op - Context Lost, Broken Templating

**Status**: BROKEN - Conversation history NOT injected into prompts  
**Impact**: CRITICAL - Multi-turn conversations have no context continuity  
**Evidence**: Full code analysis of [`chat.rs`](../packages/candle/src/builders/agent_role/chat.rs)

---

## VERIFIED BY READING ACTUAL CODE

**I read the FULL 812-line chat.rs implementation.** Here's what EXISTS vs what's MISSING:

### What EXISTS: Memory Context Injection (Line ~280)

```rust
let memory_context: Option<String> = if let Some(ref mem_manager) = memory {
    let memory_stream = mem_manager.search_by_content(&user_message);
    // ... collect search results ...
    let memories: Vec<MemoryNode> = results.into_iter()
        .filter_map(|r| r.ok())
        .collect();
    
    if !memories.is_empty() {
        Some(format_memory_context(&memories, 1000))  // ← Semantic search results
    } else { None }
} else { None };
```

**This is**: Embedding-based semantic search of stored memories (episodic/semantic/procedural nodes in SurrealDB)

**This is NOT**: Conversation history from this conversation

### What's MISSING: Conversation History in Prompt (Line ~300)

**Current prompt template**:
```rust
let full_prompt = if let Some(mem_ctx) = memory_context {
    format!("{}\n\n{}\n\nUser: {}", system_prompt, mem_ctx, user_message)
} else {
    format!("{}\n\nUser: {}", system_prompt, user_message)
};
```

**Structure**:
1. `system_prompt` - Sys instructions
2. `memory_context` - Semantic search results (if available)
3. `User: {user_message}` - Current message

**MISSING**: Prior conversation turns (User/Assistant exchanges)

### Proof conversation_history is NOT used (Line ~144)

```rust
fn conversation_history(self, _history: impl ConversationHistoryArgs) -> Self {
    self  // ← Does NOTHING
}
```

**grep search confirms**: Zero references to `conversation_history` field anywhere in chat.rs

---

## Two Different Concepts (Not the Same)

| Concept | What It Is | When Retrieved | Storage |
|---------|-----------|----------------|---------|
| **Memory Context** | Semantic search results from embedding DB | Per-turn via `search_by_content()` | SurrealDB memory tables |
| **Conversation History** | Prior User/Assistant exchanges from THIS chat | Stored in builder, injected at prompt | Builder field (NOT IMPLEMENTED) |

**Example**:

**Memory context** might retrieve:
```
# Relevant Context from Memory:
[Relevance: 0.95] Yesterday user asked about Rust ownership
[Relevance: 0.87] User prefers functional programming style
```

**Conversation history** should inject:
```
User: What is Rust?
Assistant: Rust is a systems programming language focusing on safety.
User: How does borrowing work?
Assistant: Borrowing allows references without ownership transfer.
User: Tell me more.  ← Current prompt
```

**Both are needed** for full context!

---

## Where Conversation History SHOULD Be Injected

### Current Broken Template (Line ~300)

```rust
let full_prompt = if let Some(mem_ctx) = memory_context {
    format!("{}\n\n{}\n\nUser: {}", system_prompt, mem_ctx, user_message)
} else {
    format!("{}\n\nUser: {}", system_prompt, user_message)
};
```

### Fixed Template (ADD conversation_history)

```rust
// Format conversation history if provided
let history_context = if !conversation_history.is_empty() {
    let mut hist = String::from("\n# Conversation History:\n");
    for (role, message) in conversation_history.iter() {
        match role {
            CandleMessageRole::User => hist.push_str(&format!("User: {}\n", message)),
            CandleMessageRole::Assistant => hist.push_str(&format!("Assistant: {}\n", message)),
            _ => {}
        }
    }
    hist
} else {
    String::new()
};

let full_prompt = match (memory_context, !history_context.is_empty()) {
    (Some(mem_ctx), true) => {
        format!("{}\n\n{}{}\n\nUser: {}", system_prompt, mem_ctx, history_context, user_message)
    }
    (Some(mem_ctx), false) => {
        format!("{}\n\n{}\n\nUser: {}", system_prompt, mem_ctx, user_message)
    }
    (None, true) => {
        format!("{}{}\n\nUser: {}", system_prompt, history_context, user_message)
    }
    (None, false) => {
        format!("{}\n\nUser: {}", system_prompt, user_message)
    }
};
```

**Prompt order**:
1. System prompt (role/instructions)
2. Memory context (semantic search results)
3. Conversation history (prior turns)
4. Current user message

---

## Complete Implementation

### Step 1: Add Builder Field

**File**: [`src/builders/agent_role/role_builder.rs`](../packages/candle/src/builders/agent_role/role_builder.rs)

```rust
pub struct CandleAgentRoleBuilderImpl {
    // ... existing 21 fields ...
    pub(super) conversation_history: ZeroOneOrMany<(CandleMessageRole, String)>,  // ADD
}
```

Initialize in `new()`:

```rust
impl CandleAgentRoleBuilderImpl {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            // ... existing fields ...
            conversation_history: ZeroOneOrMany::None,  // ADD - defaults to empty
        }
    }
}
```

---

### Step 2: Store History in Method

**File**: [`src/builders/agent_role/role_builder_impl.rs`](../packages/candle/src/builders/agent_role/role_builder_impl.rs) line ~207

```rust
fn conversation_history(
    mut self,
    history: impl ConversationHistoryArgs,
) -> impl CandleAgentRoleBuilder {
    self.conversation_history = history.into_history();
    self
}
```

---

### Step 3: Fix ConversationHistoryArgs Trait

**File**: [`src/builders/agent_role/helpers.rs`](../packages/candle/src/builders/agent_role/helpers.rs) line ~251

**Change return type**:

```rust
pub trait ConversationHistoryArgs {
    fn into_history(self) -> ZeroOneOrMany<(CandleMessageRole, String)>;  // NOT Option!
}
```

**Update all impls** (remove `Some()` wrappers):

```rust
impl ConversationHistoryArgs for (CandleMessageRole, &str) {
    fn into_history(self) -> ZeroOneOrMany<(CandleMessageRole, String)> {
        ZeroOneOrMany::one((self.0, self.1.to_string()))
    }
}

impl ConversationHistoryArgs for (CandleMessageRole, String) {
    fn into_history(self) -> ZeroOneOrMany<(CandleMessageRole, String)> {
        ZeroOneOrMany::one(self)
    }
}

// Tuple merge implementations...
impl<T1, T2> ConversationHistoryArgs for (T1, T2)
where
    T1: ConversationHistoryArgs,
    T2: ConversationHistoryArgs,
{
    fn into_history(self) -> ZeroOneOrMany<(CandleMessageRole, String)> {
        let h1 = self.0.into_history();
        let h2 = self.1.into_history();
        
        match (h1, h2) {
            (ZeroOneOrMany::None, h) | (h, ZeroOneOrMany::None) => h,
            (ZeroOneOrMany::One(m1), ZeroOneOrMany::One(m2)) => {
                ZeroOneOrMany::Many(vec![m1, m2])
            }
            (ZeroOneOrMany::One(m), ZeroOneOrMany::Many(mut msgs)) => {
                msgs.insert(0, m);
                ZeroOneOrMany::Many(msgs)
            }
            (ZeroOneOrMany::Many(mut msgs), ZeroOneOrMany::One(m)) => {
                msgs.push(m);
                ZeroOneOrMany::Many(msgs)
            }
            (ZeroOneOrMany::Many(mut msgs1), ZeroOneOrMany::Many(msgs2)) => {
                msgs1.extend(msgs2);
                ZeroOneOrMany::Many(msgs1)
            }
        }
    }
}

impl<T1, T2, T3> ConversationHistoryArgs for (T1, T2, T3)
where
    T1: ConversationHistoryArgs,
    T2: ConversationHistoryArgs,
    T3: ConversationHistoryArgs,
{
    fn into_history(self) -> ZeroOneOrMany<(CandleMessageRole, String)> {
        let combined_12 = (self.0, self.1).into_history();
        (combined_12, self.2).into_history()
    }
}
```

---

### Step 4: Propagate to CandleAgentBuilderImpl

**File**: [`src/builders/agent_role/agent_builder.rs`](../packages/candle/src/builders/agent_role/agent_builder.rs)

```rust
pub struct CandleAgentBuilderImpl {
    // ... existing fields ...
    pub(super) conversation_history: ZeroOneOrMany<(CandleMessageRole, String)>,  // ADD
}
```

**File**: [`src/builders/agent_role/role_builder_impl.rs`](../packages/candle/src/builders/agent_role/role_builder_impl.rs) line ~52

Propagate in `.model()`:

```rust
fn model(self, model: TextToTextModel) -> impl CandleAgentRoleBuilder {
    // ... existing code ...
    
    CandleAgentBuilderImpl {
        // ... existing fields ...
        conversation_history: self.conversation_history,  // ADD
    }
}
```

And in `into_agent()` (line ~218):

```rust
fn into_agent(self) -> impl CandleAgentBuilder {
    // ... existing code ...
    
    CandleAgentBuilderImpl {
        // ... existing fields ...
        conversation_history: self.conversation_history,  // ADD
    }
}
```

---

### Step 5: Inject History into Prompt Template

**File**: [`src/builders/agent_role/chat.rs`](../packages/candle/src/builders/agent_role/chat.rs) line ~145

**Extract conversation_history early**:

```rust
let conversation_history = self.conversation_history;  // ADD THIS at line ~157
```

**Replace prompt template** at line ~300:

```rust
// Format conversation history if provided
let history_context = match &conversation_history {
    ZeroOneOrMany::None => String::new(),
    ZeroOneOrMany::One((role, message)) => {
        format!("\n# Conversation History:\n{}: {}\n", 
            match role {
                CandleMessageRole::User => "User",
                CandleMessageRole::Assistant => "Assistant",
                _ => "System",
            },
            message
        )
    }
    ZeroOneOrMany::Many(messages) => {
        let mut hist = String::from("\n# Conversation History:\n");
        for (role, message) in messages {
            match role {
                CandleMessageRole::User => hist.push_str(&format!("User: {}\n", message)),
                CandleMessageRole::Assistant => hist.push_str(&format!("Assistant: {}\n", message)),
                _ => {}
            }
        }
        hist
    }
};

// Build full prompt with history
let full_prompt = match (memory_context, !history_context.is_empty()) {
    (Some(mem_ctx), true) => {
        format!("{}\n\n{}{}\n\nUser: {}", system_prompt, mem_ctx, history_context, user_message)
    }
    (Some(mem_ctx), false) => {
        format!("{}\n\n{}\n\nUser: {}", system_prompt, mem_ctx, user_message)
    }
    (None, true) => {
        format!("{}{}\n\nUser: {}", system_prompt, history_context, user_message)
    }
    (None, false) => {
        format!("{}\n\nUser: {}", system_prompt, user_message)
    }
};
```

---

## Definition of Done

### Code Changes
- [ ] `conversation_history` field added to both builder structs
- [ ] Field defaults to `ZeroOneOrMany::None` in `new()`
- [ ] `conversation_history()` method stores history
- [ ] `ConversationHistoryArgs::into_history()` returns `ZeroOneOrMany` (no Option)
- [ ] All trait impls updated
- [ ] History propagated through `into_agent()` and `.model()`
- [ ] History extracted in `chat()` method

### Template Injection
- [ ] History formatting logic added in chat.rs
- [ ] Prompt template matches on (memory_context, has_history)
- [ ] Order: system → memory → history → current message
- [ ] Empty history → no extra content in prompt

### Compilation
- [ ] `cargo check --lib` exits 0
- [ ] All files compile without errors
- [ ] No unused variable warnings

### Functional
- [ ] Empty history → agent works normally
- [ ] Provided history → appears in LLM prompt
- [ ] Multi-turn context preserved
- [ ] Memory + history both work together

---

## Example Prompt with Both Memory and History

**User provides history**:
```rust
let history = vec![
    (CandleMessageRole::User, "What is Rust?".to_string()),
    (CandleMessageRole::Assistant, "Rust is a systems language.".to_string()),
];

let agent = CandleFluentAi::agent_role("agent")
    .conversation_history(ZeroOneOrMany::from(history))
    .model(qwen3)
    .chat(|_| async { CandleChatLoop::UserPrompt("Tell me more.".to_string()) });
```

**Resulting LLM prompt**:
```
# Well-Informed Software Architect
You think out loud as you work through problems...
[system prompt continues]

# Relevant Context from Memory:
[Relevance: 0.92] User previously asked about programming languages
[Relevance: 0.85] User prefers strongly-typed systems

# Conversation History:
User: What is Rust?
Assistant: Rust is a systems language.

User: Tell me more.
```

**All context provided**: System instructions + Memory retrieval + Conversation history + Current message

---

## File Change Summary

| File | Changes | Lines |
|------|---------|-------|
| `role_builder.rs` | Add field + initialize | +2 |
| `role_builder_impl.rs` | Store + propagate (2 places) | +3 |
| `agent_builder.rs` | Add field | +1 |
| `helpers.rs` | Fix trait + impls | ~35 |
| `chat.rs` | Extract field + format + inject | ~30 |
| **TOTAL** | | **~71 lines** |

---

## Critical Distinction

**Memory System** (what user confused this with):
- Embedding-based semantic search
- Retrieves similar content from database
- Line ~280: `mem_manager.search_by_content()`
- Line ~227 helpers.rs: `format_memory_context()`

**Conversation History** (what's missing):
- Sequential prior turns from THIS chat
- Provides temporal context continuity
- NOT stored in memory DB (yet)
- Builder field → Prompt injection

**Both are needed** for complete context awareness.
