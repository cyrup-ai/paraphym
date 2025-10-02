# MEMFUCK2: Temperature Hardcoded Instead of Using Agent Configuration

## Problem
Temperature is hardcoded to `0.7` in AI response generation methods, completely ignoring the agent's configured temperature setting. The agent has a `temperature()` getter but it's never called.

## Location
- **File**: `/packages/candle/src/domain/agent/chat.rs`
- **Line 176**: In `generate_ai_response_with_sectioning`
- **Line 228**: In `generate_ai_response`

## Current Broken Code
```rust
let candle_params = CandleCompletionParams {
    temperature: 0.7,  // HARDCODED! Ignores self.temperature()
    max_tokens: NonZeroU64::new(1000),
    // ...
};
```

## What Should Happen
```rust
let candle_params = CandleCompletionParams {
    temperature: self.temperature().unwrap_or(0.7),  // Use agent config with fallback
    max_tokens: self.max_tokens()
        .and_then(NonZeroU64::new)
        .or_else(|| NonZeroU64::new(1000)),
    // ...
};
```

## Available Methods
The agent role already has these methods:
- `fn temperature(&self) -> Option<f64>` (line 199 in role.rs)
- `fn max_tokens(&self) -> Option<u64>` (line 203 in role.rs)

## Impact
- Agent temperature configuration is completely ignored
- Can't control creativity/randomness per agent
- All agents behave with same temperature regardless of config
- Builder pattern's `.temperature()` method is useless

## Fix Priority
**HIGH** - This breaks agent configuration and makes all agents behave identically