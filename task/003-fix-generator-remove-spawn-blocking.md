# Task: Fix Generator - Remove Excessive spawn_blocking

## Problem
`generator.rs` has excessive `spawn_blocking` calls that add latency and cause issues:
- Tensor operations wrapped in spawn_blocking
- Tokenizer encode/decode wrapped in spawn_blocking  
- Logits conversion wrapped in spawn_blocking

Candle's examples run these operations directly and are FAST. Our code is SLOW.

## Files to Fix

### `/Volumes/samsung_t9/paraphym/packages/candle/src/core/generation/generator.rs`

## Changes Required

### 1. Remove spawn_blocking from Prompt Encoding
```rust
// BEFORE (SLOW):
let tokens = match tokio::task::spawn_blocking(move || {
    tokenizer.encode(prompt_str.as_str(), true)
}).await { ... }

// AFTER (FAST - like Candle):
let tokens = tokenizer
    .encode(prompt_str.as_str(), true)
    .map_err(|e| anyhow::Error::msg)?;
```

### 2. Remove spawn_blocking from Tensor Operations
```rust
// BEFORE (SLOW):
let initial_input = match tokio::task::spawn_blocking(move || {
    let tensor = Tensor::new(tokens_clone.as_slice(), &device_clone)?;
    tensor.unsqueeze(0)
}).await { ... }

// AFTER (FAST - like Candle):
let input = Tensor::new(tokens, &device)?.unsqueeze(0)?;
```

### 3. Remove spawn_blocking from Logits Conversion
```rust
// BEFORE (SLOW):
let logits_vec = match tokio::task::spawn_blocking(move || {
    logits_clone.to_vec1::<f32>()
}).await { ... }

// AFTER (FAST - like Candle):
let logits = logits.squeeze(0)?;
// Use logits directly
```

### 4. Remove spawn_blocking from Token Decoding
```rust
// BEFORE (SLOW):
match tokio::task::spawn_blocking(move || {
    tokenizer.decode(&[token], false)
}).await { ... }

// AFTER (FAST - like Candle):
if let Some(t) = token_output_stream.next_token(next_token)? {
    let _ = tx.send(CandleStringChunk(t));
}
```

## Rationale

Candle's quantized models run on CPU/Metal. These operations are:
1. **Not blocking** - they're just function calls
2. **Fast** - milliseconds, not seconds
3. **Thread-safe** - Candle handles this

spawn_blocking adds:
- Task spawning overhead
- Context switching
- Channel communication overhead
- Async complexity

Result: 10-100x slower for no benefit.

## Success Criteria
- Generator matches Candle example's simple synchronous loop
- Token generation starts immediately
- No spawn_blocking calls in hot path
- Performance matches baseline
