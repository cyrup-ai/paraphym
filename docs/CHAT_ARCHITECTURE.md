# Chat Architecture: Loop Control vs Hooks

## Overview

The Candle chat system provides three distinct mechanisms for controlling and extending chat behavior:

1. **`chat()` closure** - Primary loop control and user input
2. **`on_conversation_turn` hook** - Post-generation tool calling and modifications
3. **`on_chunk` handler** - Real-time stream processing

Understanding when and how to use each mechanism is critical for building correct interactive applications.

## Three Distinct Mechanisms

### 1. chat() Closure - Primary Loop Control

**Purpose**: Control the interactive chat loop, read user input, decide when to continue/exit

**When Called**: Asynchronously INSIDE the stream loop (async task)

**Signature**:
```rust
fn chat<F, Fut>(self, handler: F) -> Result<Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>, AgentError>
where
    F: Fn(&CandleAgentConversation) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = CandleChatLoop> + Send + 'static
```

**Required Operations**:
- ✅ Async I/O (tokio::io::stdin)
- ✅ Await futures
- ✅ Access to conversation history
- ❌ Do NOT use blocking std::io operations

**Returns**: `Future<Output = CandleChatLoop>` enum
- `Break` - Exit loop
- `Reprompt(String)` - Display text and continue without model
- `UserPrompt(String)` - Send message to model

**Example - Interactive Chat**:
```rust
agent.chat(|conversation| async {
    use tokio::io::{AsyncBufReadExt, BufReader};
    
    print!("You: ");
    io::stdout().flush().unwrap();
    
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut input = String::new();
    
    reader.read_line(&mut input).await.unwrap();
    
    match input.trim() {
        "exit" => CandleChatLoop::Break,
        "" => CandleChatLoop::Reprompt(String::new()),
        msg => CandleChatLoop::UserPrompt(msg.to_string()),
    }
})
```

**Implementation Location**: `src/builders/agent_role.rs:1286-1314`


### 2. on_conversation_turn Hook - Post-Generation Tool Calling

**Purpose**: Inject additional inference cycles or tool calls AFTER model responds

**When Called**: Asynchronously AFTER assistant generates response

**Signature**:
```rust
fn on_conversation_turn<F, Fut>(self, handler: F) -> impl CandleAgentRoleBuilder
where
    F: Fn(&CandleAgentConversation, &CandleAgentRoleAgent) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>> + Send + 'static
```

**⚠️ Unsafe Operations**:
- ❌ Do NOT use for user input
- ❌ Do NOT use for loop control
- ❌ Do NOT block on stdin

**Use Cases**:
- Tool calling based on response content
- Multi-agent conversations
- Conversation logging/modification
- Follow-up queries based on assistant's response

**Example - Tool Calling**:
```rust
.on_conversation_turn(|conversation, agent| async move {
    let last_msg = conversation.latest_user_message();
    
    if last_msg.contains("search") {
        // Call search tool, return additional stream
        agent.chat(CandleChatLoop::UserPrompt("search results...".to_string()))
    } else {
        // Return empty stream
        Box::pin(futures::stream::empty())
    }
})
```

**Implementation Location**: 
- Definition: `src/builders/agent_role.rs:368-373`
- Invocation: `src/builders/agent_role.rs:243-259`


### 3. on_chunk Handler - Real-time Stream Processing

**Purpose**: Process/modify streaming tokens as they're generated

**When Called**: During text generation for each chunk

**Signature**:
```rust
fn on_chunk<F, Fut>(self, handler: F) -> impl CandleAgentRoleBuilder
where
    F: Fn(CandleMessageChunk) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = CandleMessageChunk> + Send + 'static
```

**Use Cases**:
- Real-time output to stdout
- Logging
- Token counting
- Chunk modification/filtering
- Progress indicators

**Example - Real-time Output**:
```rust
.on_chunk(|chunk| async move {
    if let CandleMessageChunk::Text(ref text) = chunk {
        print!("{}", text);
        io::stdout().flush().unwrap();
    }
    chunk
})
```

**Implementation Location**: `src/builders/agent_role.rs:234-240`

## Comparison Table

| Aspect | chat() closure | on_conversation_turn | on_chunk |
|--------|----------------|---------------------|----------|
| **Called When** | BEFORE each turn | AFTER assistant responds | DURING generation |
| **Execution** | Synchronous | Asynchronous | Asynchronous |
| **Purpose** | Loop control, user input | Tool calling, hooks | Stream processing |
| **Can Block** | ✅ Yes (stdin safe) | ❌ No | ❌ No |
| **Returns** | `CandleChatLoop` | `Stream<CandleMessageChunk>` | `CandleMessageChunk` |
| **Use For** | Interactive chat | External tools | Real-time output |


## Decision Tree: Which Mechanism to Use?

```
Need to read user input for interactive chat?
└─> Use chat() closure

Need to call tools AFTER model responds?
└─> Use on_conversation_turn hook

Need to display tokens as they stream?
└─> Use on_chunk handler

Need to control when chat loop exits?
└─> Use chat() closure

Need to add follow-up AI inference after response?
└─> Use on_conversation_turn hook

Need to inspect conversation history before prompting?
└─> Use chat() closure (receives &CandleAgentConversation)

Need to modify streaming output in real-time?
└─> Use on_chunk handler
```

## Common Anti-Patterns

### ❌ WRONG: Using on_conversation_turn for User Input

```rust
// DO NOT DO THIS
.on_conversation_turn(|conversation, agent| async move {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();  // WRONG TIMING!
    agent.chat(CandleChatLoop::UserPrompt(input))
})
```

**Why Wrong**: `on_conversation_turn` is called AFTER generation, not before. This would cause input to be read at the wrong time in the flow.

**Correct Alternative**:
```rust
// DO THIS INSTEAD
agent.chat(|_conversation| {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();  // CORRECT TIMING!
    CandleChatLoop::UserPrompt(input.trim().to_string())
})
```


### ❌ WRONG: Trying to Loop with on_conversation_turn

```rust
// DO NOT DO THIS
.on_conversation_turn(|conversation, agent| async move {
    // Trying to implement recursive chat here is an anti-pattern
    loop {
        let input = get_input();  // WRONG PLACE!
        agent.chat(CandleChatLoop::UserPrompt(input)).await;
    }
})
```

**Why Wrong**: The chat loop is already implemented. Use the `chat()` closure which is called automatically by the loop infrastructure.

### ❌ WRONG: Blocking in on_chunk

```rust
// DO NOT DO THIS
.on_chunk(|chunk| async move {
    if let CandleMessageChunk::Text(ref text) = chunk {
        // Blocking I/O in async context - BAD!
        std::thread::sleep(Duration::from_secs(1));  // WRONG!
    }
    chunk
})
```

**Why Wrong**: `on_chunk` is called in async context during streaming. Blocking operations will stall the entire stream.

**Correct Alternative**:
```rust
// DO THIS INSTEAD
.on_chunk(|chunk| async move {
    if let CandleMessageChunk::Text(ref text) = chunk {
        // Async operations are fine
        tokio::time::sleep(Duration::from_millis(10)).await;  // CORRECT!
    }
    chunk
})
```


## Execution Flow Diagram

Understanding the exact timing and sequence is crucial:

```
┌─────────────────────────────────────────────────┐
│ LOOP START (spawn_stream async block)          │
└─────────────────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────────────────┐
│ ① chat() closure called SYNCHRONOUSLY           │
│    - Can block on stdin                         │
│    - Returns CandleChatLoop enum                │
└─────────────────────────────────────────────────┘
           │
           ├─► Break? ──────────────────────────────► EXIT
           │
           ├─► Reprompt(msg)? ──► Send msg ───────┐
           │                                       │
           ▼                                       │
     UserPrompt(msg)                               │
           │                                       │
           ▼                                       │
┌─────────────────────────────────────────────────┤
│ ──── ASYNC BOUNDARY ────                        │
│                                                  │
│ ② Model generation begins (async)               │
│    - Memory initialization                      │
│    - Context loading                            │
│    - Streaming inference                        │
└─────────────────────────────────────────────────┘
           │
           ▼ (for each chunk)
┌─────────────────────────────────────────────────┐
│ ③ on_chunk() called (async)                     │
│    - Process/modify chunk                       │
│    - Send to stream consumer                    │
└─────────────────────────────────────────────────┘
           │
           ▼ (generation complete)
┌─────────────────────────────────────────────────┐
│ ④ on_conversation_turn() called (async)         │
│    - Receives full conversation                 │
│    - Can call tools or add inference            │
│    - Returns additional stream chunks           │
└─────────────────────────────────────────────────┘
           │
           └──────────────► LOOP to step ①
```


## Key Insights

### Why chat() Can Block

The `chat()` closure is called **outside** the async stream context:

```rust
// From src/builders/agent_role.rs:1307-1314
Ok(Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
    let mut conversation = CandleAgentConversation::new();
    
    loop {
        // ✅ Handler called SYNCHRONOUSLY here - not in async context
        let chat_loop_result = handler(&conversation);
        
        // THEN async work begins based on return value
        match chat_loop_result {
            CandleChatLoop::UserPrompt(user_message) => {
                // NOW async operations begin
            }
        }
    }
})))
```

The handler is called in a **synchronous context** before any async operations, making blocking I/O safe.

### Why on_conversation_turn Can't Block

The `on_conversation_turn` hook is invoked **inside** the async stream processing:

```rust
// From src/builders/agent_role.rs:243-259
// (Inside async stream processing after generation)
if let Some(ref handler) = state.on_conversation_turn_handler {
    // Already in async context
    let handler_stream = handler(&conversation, &agent).await;
    // Blocking here would stall the stream
}
```


## Implementation Reference

### Canonical Interactive Chat Pattern

**File**: `examples/interactive_chat.rs`

This is the **reference implementation** showing:
- ✅ Correct use of `chat()` closure for stdin reading
- ✅ Proper handling of exit commands
- ✅ Stream consumption with proper chunk handling
- ✅ Real-time output via `on_chunk`

**Key Code** (lines 83-116):
```rust
let stream = agent.chat(|_conversation| {
    print!("You: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim();
            
            if input.eq_ignore_ascii_case("exit") {
                return CandleChatLoop::Break;
            }
            
            if input.is_empty() {
                return CandleChatLoop::Reprompt(String::new());
            }
            
            CandleChatLoop::UserPrompt(input.to_string())
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            CandleChatLoop::Break
        }
    }
})?;
```

### Production CLI Implementation

**File**: `src/cli/runner.rs`

Production-quality implementation with:
- ✅ Command handling (`/help`, `/save`, etc.)
- ✅ Input validation
- ✅ Colored output
- ✅ Graceful shutdown (Ctrl+C handling)


### Programmatic (Non-Interactive) Pattern

**File**: `examples/fluent_builder.rs`

Shows programmatic usage with hardcoded queries:

```rust
let query = args.query.clone();
let stream = agent.chat(move |_conversation| {
    // No stdin read - just return hardcoded query
    CandleChatLoop::UserPrompt(query.clone())
})?;
```

This pattern is useful for:
- Automated testing
- Batch processing
- Single-query applications
- Scripts and automation

## Testing Your Understanding

If you can answer these correctly, you understand the architecture:

1. **Where should you read from stdin for interactive chat?**
   - ✅ Answer: In the `chat()` closure (before async work)
   - ❌ NOT in `on_conversation_turn` (called after response)
   - ❌ NOT in `on_chunk` (called during streaming)

2. **When is `on_conversation_turn` called?**
   - ✅ Answer: AFTER the assistant generates a response
   - ❌ NOT before each turn
   - ❌ NOT during streaming

3. **Can you block on I/O in the `chat()` closure?**
   - ✅ Answer: YES - it's called synchronously before async work
   - Blocking stdin is completely safe and expected

4. **What does `on_chunk` receive?**
   - ✅ Answer: Each streaming `CandleMessageChunk` as it's generated
   - Used for real-time output and processing

5. **How do you exit the chat loop?**
   - ✅ Answer: Return `CandleChatLoop::Break` from the `chat()` closure
   - This is the only proper way to exit


6. **What does `chat()` closure return?**
   - ✅ Answer: `CandleChatLoop` enum (Break, Reprompt, or UserPrompt)
   - This controls loop flow

7. **Can you modify chunks in real-time?**
   - ✅ Answer: YES - use `on_chunk` handler
   - Receives chunk, returns modified chunk

## Quick Reference Card

```rust
// ✅ Interactive chat with stdin
agent.chat(|_conversation| {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();  // SAFE
    CandleChatLoop::UserPrompt(input.trim().to_string())
})

// ✅ Real-time output
.on_chunk(|chunk| async move {
    if let CandleMessageChunk::Text(ref text) = chunk {
        print!("{}", text);
    }
    chunk
})

// ✅ Tool calling after response
.on_conversation_turn(|conversation, agent| async move {
    let last_msg = conversation.latest_user_message();
    if last_msg.contains("search") {
        agent.chat(CandleChatLoop::UserPrompt("search...".to_string()))
    } else {
        Box::pin(futures::stream::empty())
    }
})
```


## Common Patterns

### Pattern: Exit on Keywords

```rust
.chat(|conversation| {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    
    // Smart exit detection
    if input.to_lowercase().contains("goodbye")
        || input.to_lowercase().contains("bye")
        || input.starts_with("/exit")
    {
        println!("Goodbye!");
        return CandleChatLoop::Break;
    }
    
    CandleChatLoop::UserPrompt(input.to_string())
})
```

### Pattern: Turn Limiting

```rust
let mut turns = std::sync::Arc::new(std::sync::Mutex::new(0));
let turns_clone = turns.clone();

.chat(move |_| {
    let mut count = turns_clone.lock().unwrap();
    *count += 1;
    
    if *count > 5 {
        println!("Conversation limit reached.");
        return CandleChatLoop::Break;
    }
    
    // Get input...
    CandleChatLoop::UserPrompt(input)
})
```


### Pattern: Context-Based Logic

```rust
.chat(|conversation| {
    let last_msg = conversation.latest_user_message();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    // If user repeated same question, provide different prompt
    if input.trim() == last_msg {
        println!("(I noticed you asked the same thing. Let me rephrase...)");
    }
    
    CandleChatLoop::UserPrompt(input.trim().to_string())
})
```

## See Also

### Documentation
- [Task: FIX_CLI.md](../task/FIX_CLI.md) - Detailed analysis and findings
- [Task: no_model_output.md](../task/no_model_output.md) - Interactive chat requirements

### Examples
- `examples/interactive_chat.rs` - Canonical stdin reading pattern
- `examples/fluent_builder.rs` - Programmatic (non-interactive) usage

### Source Code
- `src/cli/runner.rs` - Production CLI implementation
- `src/builders/agent_role.rs:1286-1838` - `chat()` implementation
- `src/builders/agent_role.rs:243-259` - `on_conversation_turn` invocation
- `src/builders/agent_role.rs:234-240` - `on_chunk` invocation

### Tests
- `tests/cli/` - CLI integration tests
- `tests/domain/chat/` - Chat loop unit tests

## Summary

The key to understanding Candle's chat architecture:

1. **`chat()` closure** = Loop control + User input (synchronous, can block)
2. **`on_conversation_turn`** = Post-generation hooks (async, no blocking)
3. **`on_chunk`** = Stream processing (async, no blocking)

Use the right mechanism for the right purpose, and your interactive applications will be clean, correct, and maintainable.
