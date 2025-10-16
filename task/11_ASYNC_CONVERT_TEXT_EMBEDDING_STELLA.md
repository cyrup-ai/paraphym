# Task: Convert Stella Text Embedding to Use Async huggingface_file

## Location
`packages/candle/src/capability/text_embedding/stella.rs`

## Dependencies
- ✅ **UNBLOCKED**: Base trait is now async (ASYNC_FIX_HUGGINGFACE.md complete)

## Overview
Convert all `huggingface_file()` calls in Stella-EN-1.5B-v5 text embedding model to use async/await pattern. Stella uses MRL (Matryoshka Representation Learning) with dimension-specific projection heads.

## Call Sites to Convert

Total: **9 call sites** across embed, batch_embed, and load methods

### Files to Download
- Base model weights: `model.safetensors`
- MRL projection heads: `2_Dense_{dimension}/model.safetensors` (dimension-specific)
- Tokenizer: `tokenizer.json`

## Solution Pattern

**Current (broken)**:
```rust
let base_weights = self.huggingface_file(self.info().registry_key, "model.safetensors")?;
let mrl_head = self.huggingface_file(
    self.info().registry_key,
    &format!("2_Dense_{}/model.safetensors", dimension),
)?;
let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json")?;
```

**Fixed**:
```rust
let base_weights = self.huggingface_file(self.info().registry_key, "model.safetensors").await?;
let mrl_head = self.huggingface_file(
    self.info().registry_key,
    &format!("2_Dense_{}/model.safetensors", dimension),
).await?;
let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json").await?;
```

## Steps
1. Add `.await` after each `huggingface_file()` call
2. Ensure parent methods are `async` or use async closures
3. Handle multiple dimension sizes (256, 512, 768, 1024, etc.)
4. Test compilation
5. Verify Stella embedding functionality with different dimensions

## Priority
🔴 **HIGH** - Stella is a key embedding model with MRL support

## Status
⏳ TODO
