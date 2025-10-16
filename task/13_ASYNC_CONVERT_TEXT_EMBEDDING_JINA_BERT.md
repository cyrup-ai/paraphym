# Task: Convert Jina BERT Text Embedding to Use Async huggingface_file

## Location
`packages/candle/src/capability/text_embedding/jina_bert.rs`

## Dependencies
- ✅ **UNBLOCKED**: Base trait is now async (ASYNC_FIX_HUGGINGFACE.md complete)

## Overview
Convert all `huggingface_file()` calls in Jina BERT text embedding model to use async/await pattern.

## Call Sites to Convert

Total: **6 call sites** missing `.await`

## Solution Pattern

**Current (broken)**:
```rust
let model_path = self.huggingface_file(self.info().registry_key, "model.safetensors")?;
let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json")?;
let config_path = self.huggingface_file(self.info().registry_key, "config.json")?;
```

**Fixed**:
```rust
let model_path = self.huggingface_file(self.info().registry_key, "model.safetensors").await?;
let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json").await?;
let config_path = self.huggingface_file(self.info().registry_key, "config.json").await?;
```

## Steps
1. Identify all `huggingface_file()` calls in the file
2. Add `.await` after each call before `?` or `.map_err()`
3. Ensure parent method/function is `async` or uses async closure
4. Test compilation
5. Verify Jina BERT embedding functionality

## Priority
🔴 **MEDIUM-HIGH** - Blocking compilation of Jina BERT capability

## Status
⏳ TODO
