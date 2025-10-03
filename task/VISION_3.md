# VISION_3: LLaVA Vision-Language Provider - KV Cache Optimization

## CRITICAL BUG FOUND - PRODUCTION BLOCKER

The LLaVA implementation is **architecturally correct** but has a **critical performance bug** in the autoregressive generation loop that makes it unsuitable for production use.

---

## BUG DESCRIPTION

**Location**: `packages/candle/src/providers/llava.rs`
- `ask()` method (lines ~270-325)  
- `ask_url()` method (lines ~340-395)

**Issue**: The generation loop passes **ALL accumulated embeddings** to `model.forward()` on every iteration, instead of slicing to pass only the newest embedding when KV cache is enabled.

**Impact**: 
- O(n²) complexity instead of O(n)
- Makes KV cache completely useless
- Extremely slow for responses longer than a few tokens
- Unacceptable performance for production use

---

## CORRECT PATTERN FROM REFERENCE

From `tmp/candle/candle-examples/examples/llava/main.rs:268-292`:

```rust
let mut index_pos = 0;
for index in 0..args.max_new_tokens {
    let (_, input_embeds_len, _) = input_embeds.dims3()?;
    
    // KEY LOGIC: Only pass new embeddings when cache is active
    let (context_size, context_index) = if cache.use_kv_cache && index > 0 {
        (1, index_pos)  // After first iteration, pass only 1 new embedding
    } else {
        (input_embeds_len, 0)  // First iteration passes all embeddings
    };
    
    // Slice to get only relevant embeddings
    let input = input_embeds.i((.., input_embeds_len.saturating_sub(context_size).., ..))?;
    
    let logits = llava.forward(&input, context_index, &mut cache)?;
    let (_, input_len, _) = input.dims3()?;
    index_pos += input_len;  // Track position for cache
    
    // ... token sampling and embedding concatenation
}
```

---

## CURRENT BROKEN IMPLEMENTATION

```rust
// WRONG - No slicing, passes all embeddings every time
for (position_id, _) in (0..self.config.max_new_tokens).enumerate() {
    let logits = self.model.forward(&current_embeds, position_id, &mut cache)?;
    
    // ... rest of loop
}
```

---

## REQUIRED FIX

**File**: `packages/candle/src/providers/llava.rs`

### Fix ask() method (around line 270):

```rust
// Replace the entire generation loop with:
let mut index_pos = 0;
for index in 0..self.config.max_new_tokens {
    // Get current embedding dimensions
    let (_, input_embeds_len, _) = current_embeds.dims3()
        .map_err(|e| format!("Failed to get embed dims: {}", e))?;
    
    // Determine context size and index based on cache state
    let (context_size, context_index) = if self.config.use_kv_cache && index > 0 {
        (1, index_pos)  // Only new embedding after first iteration
    } else {
        (input_embeds_len, 0)  // All embeddings on first iteration
    };
    
    // Slice embeddings to pass only relevant portion
    let input = current_embeds.i((.., input_embeds_len.saturating_sub(context_size).., ..))?;
    
    // Forward pass with sliced input
    let logits = self.model.forward(&input, context_index, &mut cache)
        .map_err(|e| format!("Generation failed: {}", e))?;
    
    // Update position tracking
    let (_, input_len, _) = input.dims3()
        .map_err(|e| format!("Failed to get input dims: {}", e))?;
    index_pos += input_len;
    
    // Sample next token
    let next_token = self.sample_token(&logits)?;
    
    // Check EOS
    if next_token == self.llava_config.eos_token_id as u32 {
        break;
    }
    
    // Decode token
    if let Ok(text) = self.tokenizer.decode(&[next_token], false) {
        generated_text.push_str(&text);
    }
    
    // Embed next token and append
    let next_token_tensor = Tensor::new(&[next_token], &self.device)
        .map_err(|e| format!("Token tensor failed: {}", e))?;
    let next_embeds = self.model.llama.embed(&next_token_tensor)?
        .unsqueeze(0)?;
    
    current_embeds = Tensor::cat(&[current_embeds, next_embeds], 1)
        .map_err(|e| format!("Embedding concat failed: {}", e))?;
}
```

### Apply same fix to ask_url() method (around line 340)

The exact same pattern must be applied to the `ask_url()` method's generation loop.

---

## VERIFICATION STEPS

After fixing:

1. **Compile**: 
   ```bash
   cargo check -p paraphym_candle
   ```

2. **Verify KV cache logic**:
   - First iteration: passes all embeddings with context_index=0
   - Subsequent iterations: passes only 1 newest embedding with context_index=index_pos
   - Position tracking increments correctly

3. **Test performance**:
   - Short responses (10-20 tokens) should work correctly
   - Long responses (100+ tokens) should be fast with KV cache
   - Compare with/without cache to verify optimization works

---

## MINOR CLEANUP (Optional)

While fixing the critical bug, also address:

1. Remove unused import at line 12:
   ```rust
   use std::sync::Arc;  // DELETE THIS
   ```

2. Remove underscore prefix from `_preprocess_image_base64` (line 189) or delete if truly unused

---

## DEFINITION OF DONE

- ✅ KV cache optimization implemented correctly in `ask()`
- ✅ KV cache optimization implemented correctly in `ask_url()`
- ✅ Embedding slicing logic matches reference pattern exactly
- ✅ Position tracking with `index_pos` variable works correctly
- ✅ First iteration passes all embeddings, subsequent iterations pass only newest
- ✅ Code compiles without errors
- ✅ Performance is O(n) not O(n²) for long responses
- ✅ Optional cleanup items addressed

---

## ASSESSMENT

**Current Rating**: 6/10

**Blocking Issues**: 
- Critical KV cache performance bug

**What's Already Correct** (DO NOT CHANGE):
- ✅ Candle LLaVA model wrapping
- ✅ Two-stage image normalization (normalize_unsigned → normalize_with)
- ✅ Image builder integration
- ✅ Tokenization with <image> placeholder handling  
- ✅ LogitsProcessor sampling
- ✅ Module exports
- ✅ Auto-download with DownloadProviderFactory
- ✅ Overall architecture and patterns

**Once the KV cache bug is fixed**, this implementation will be production-ready and deserve a 10/10 rating.
