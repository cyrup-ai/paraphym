# Task: Convert BERT Text Embedding to Use Async huggingface_file

## Location
`packages/candle/src/capability/text_embedding/bert.rs`

## Dependencies
- ✅ **UNBLOCKED**: Base trait is now async (ASYNC_FIX_HUGGINGFACE.md complete)

## Overview
Convert all `huggingface_file()` calls in BERT text embedding model to use async/await pattern.

## Call Sites to Convert

Total: **9 call sites** missing `.await`

### Compilation Errors
```
error[E0277]: the `?` operator can only be applied to values that implement `Try`
   --> packages/candle/src/capability/text_embedding/bert.rs:259:13
   --> packages/candle/src/capability/text_embedding/bert.rs:260:30
   --> packages/candle/src/capability/text_embedding/bert.rs:261:27
   --> packages/candle/src/capability/text_embedding/bert.rs:361:13
   --> packages/candle/src/capability/text_embedding/bert.rs:362:30
   --> packages/candle/src/capability/text_embedding/bert.rs:363:27
   (and 3 more...)
```

## Solution Pattern

**Current (broken)**:
```rust
let model_weights_path = self.huggingface_file(self.info().registry_key, "model.safetensors")?;
let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json")?;
let config_path = self.huggingface_file(self.info().registry_key, "config.json")?;
```

**Fixed**:
```rust
let model_weights_path = self.huggingface_file(self.info().registry_key, "model.safetensors").await?;
let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json").await?;
let config_path = self.huggingface_file(self.info().registry_key, "config.json").await?;
```

## Steps
1. Add `.await` after each `huggingface_file()` call before `?` operator
2. Ensure parent method/function is `async` or uses async closure  
3. Likely affects multiple methods (embed, batch_embed, load)
4. Test compilation
5. Verify BERT embedding functionality

## Files Involved
- Model weights loading
- Tokenizer loading
- Config file loading

## Priority
🔴 **HIGH** - Blocking compilation of BERT embedding capability

## Status
⏳ TODO
