# MEMFUCK3: Max Tokens Hardcoded Instead of Using Agent Configuration

## Problem
Max tokens is hardcoded to `1000` in AI response generation methods, completely ignoring the agent's configured max_tokens setting. The agent has a `max_tokens()` getter but it's never called.

## Location
- **File**: `/packages/candle/src/domain/agent/chat.rs`
- **Line 177**: In `generate_ai_response_with_sectioning`
- **Line 229**: In `generate_ai_response`

## Current Broken Code
```rust
let candle_params = CandleCompletionParams {
    temperature: 0.7,
    max_tokens: NonZeroU64::new(1000),  // HARDCODED! Ignores self.max_tokens()
    // ...
};
```

## What Should Happen
```rust
let candle_params = CandleCompletionParams {
    temperature: self.temperature().unwrap_or(0.7),
    max_tokens: self.max_tokens()
        .and_then(NonZeroU64::new)
        .or_else(|| NonZeroU64::new(1000)),  // Use agent config with fallback
    // ...
};
```

## Available Method
The agent role already has:
- `fn max_tokens(&self) -> Option<u64>` (line 203 in role.rs)

## Impact
- Agent max_tokens configuration is completely ignored
- Can't control response length per agent
- All agents limited to 1000 tokens regardless of config
- Builder pattern's `.max_tokens()` method is useless
- Can't handle use cases requiring longer responses

## Related Issue
Same pattern as MEMFUCK2 (temperature) - both parameters are hardcoded when they should use agent configuration.

## Fix Priority
**HIGH** - This breaks agent configuration and prevents handling different response length requirements