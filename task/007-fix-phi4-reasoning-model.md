# Task: Fix Phi4 Reasoning Model

## Objective
After proving the architecture works with Qwen3, apply the same fixes to Phi4 so it works correctly.

## Root Cause
Phi4 had the same issues as the original broken code:
1. Excessive spawn_blocking calls in generator.rs (now fixed)
2. Complicated async wrapping causing deadlocks

## Files to Fix

### `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/text_to_text/phi4_reasoning.rs`

Apply the same patterns used in Qwen3:

1. **Simplify Model Loading**
   - Remove unnecessary async complexity
   - Use direct GGUF loading like Candle examples
   - Extract EOS token: `100265` from GGUF metadata

2. **Chat Template**
   ```
   <|im_start|>user\n{prompt}<|im_end|>\n<|im_start|>assistant\n
   ```

3. **Remove spawn_blocking Wrapping**
   - Let generator.rs handle operations directly
   - Don't wrap tokenizer operations
   - Don't wrap tensor operations

4. **Match Qwen3 Structure**
   - Use same loading pattern
   - Use same error handling
   - Use same stream structure

## Key Fixes

```rust
// BEFORE (BROKEN):
let model = match tokio::task::spawn_blocking(move || {
    // Complex loading logic
}).await { ... }

// AFTER (WORKING - like Qwen3):
let model = quantized_phi3::ModelWeights::from_gguf(content, &mut file, &device)?;
```

## Success Criteria
- Phi4 loads in < 5 seconds (it's 7.8GB vs Qwen3's 1.1GB)
- Generates tokens immediately
- Performance comparable to size difference
- No hangs or deadlocks
- Matches Candle's pattern
