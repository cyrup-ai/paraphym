# MEMFUCK6: PromptFormatter Missing System Prompt Support

## Problem
The `PromptFormatter` class is designed to properly format prompts with memory vs context sectioning, but it completely lacks support for system prompts. This is an architectural gap that prevents proper prompt construction.

## Location
- **File**: `/packages/candle/src/domain/completion/prompt_formatter.rs`
- **Lines 74-102**: `format_prompt` method has no system_prompt parameter

## Current Broken Code
```rust
pub fn format_prompt(
    &self,
    memories: &ZeroOneOrMany<RetrievalResult>,
    documents: &ZeroOneOrMany<Document>,
    chat_history: &ZeroOneOrMany<ChatMessage>,
    user_message: &str,
) -> String {
    // NO SYSTEM PROMPT PARAMETER OR HANDLING!
    let mut prompt_parts = Vec::new();
    // ... builds prompt WITHOUT system instructions ...
}
```

## What Should Happen
```rust
pub fn format_prompt(
    &self,
    system_prompt: Option<&str>,  // ADD THIS PARAMETER
    memories: &ZeroOneOrMany<RetrievalResult>,
    documents: &ZeroOneOrMany<Document>,
    chat_history: &ZeroOneOrMany<ChatMessage>,
    user_message: &str,
) -> String {
    let mut prompt_parts = Vec::new();

    // 1. System prompt FIRST (most important for LLM attention)
    if let Some(system) = system_prompt {
        if self.include_headers {
            prompt_parts.push(format!("--- SYSTEM INSTRUCTIONS ---\n{system}"));
        } else {
            prompt_parts.push(system.to_string());
        }
    }

    // 2. Memory section
    if let Some(memory_section) = self.format_memory_section(memories) {
        prompt_parts.push(memory_section);
    }

    // ... rest of the method ...
}
```

## Related to MEMFUCK1
This architectural gap makes it impossible to properly fix MEMFUCK1. Even if we want to include the system prompt in `generate_ai_response`, the PromptFormatter can't handle it.

## Impact
- Can't use PromptFormatter for proper prompt construction
- Forces workarounds and inconsistent prompt formatting
- System prompts can't benefit from proper sectioning
- Breaks the abstraction - PromptFormatter should handle ALL prompt parts

## LLM Best Practices Violated
According to LLM prompt engineering best practices:
1. System prompt should come FIRST
2. Clear sectioning improves understanding
3. U-shaped attention pattern means beginning is crucial
4. System instructions set the context for everything else

## Fix Priority
**CRITICAL** - This is an architectural issue that blocks proper prompt construction