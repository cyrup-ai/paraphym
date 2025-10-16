# Task: Convert GTE-Qwen Text Embedding to Use Async huggingface_file

## Location
`packages/candle/src/capability/text_embedding/gte_qwen.rs`

## Dependencies
- ✅ **UNBLOCKED**: Base trait is now async (ASYNC_FIX_HUGGINGFACE.md complete)

## Overview
Convert all `huggingface_file()` calls in GTE-Qwen2-1.5B-instruct text embedding model to use async/await pattern. GTE-Qwen uses sharded model files for efficient loading.

## Call Sites to Convert

Total: **10+ call sites** across embed, batch_embed, and load methods

### Files to Download
- Sharded model weights: `model-00001-of-00002.safetensors`, `model-00002-of-00002.safetensors`
- Model index: `model.safetensors.index.json`
- Tokenizer: `tokenizer.json`
- Config: `config.json`

## Solution Pattern

**Current (broken)**:
```rust
let shard1 = self.huggingface_file(self.info().registry_key, "model-00001-of-00002.safetensors")?;
let shard2 = self.huggingface_file(self.info().registry_key, "model-00002-of-00002.safetensors")?;
let index_path = self.huggingface_file(self.info().registry_key, "model.safetensors.index.json")?;
let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json")?;
let config_path = self.huggingface_file(self.info().registry_key, "config.json")?;
```

**Fixed**:
```rust
let shard1 = self.huggingface_file(self.info().registry_key, "model-00001-of-00002.safetensors").await?;
let shard2 = self.huggingface_file(self.info().registry_key, "model-00002-of-00002.safetensors").await?;
let index_path = self.huggingface_file(self.info().registry_key, "model.safetensors.index.json").await?;
let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json").await?;
let config_path = self.huggingface_file(self.info().registry_key, "config.json").await?;
```

## Steps
1. Add `.await` after each `huggingface_file()` call
2. Ensure parent methods are `async` or use async closures
3. Handle all sharded file downloads
4. Test compilation
5. Verify GTE-Qwen embedding functionality

## Priority
🔴 **HIGH** - GTE-Qwen is a high-performance embedding model with sharded weights

## Status
⏳ TODO
