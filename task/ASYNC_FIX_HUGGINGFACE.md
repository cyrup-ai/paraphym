# Task: Convert CandleModel::huggingface_file to Async

## Location
`packages/candle/src/domain/model/traits.rs` lines 92-107

## Problem
```rust
fn huggingface_file(
    &self,
    repo_key: &str,
    filename: &str,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>>
{
    use hf_hub::api::sync::Api;  // ❌ BLOCKING API
    
    let api = Api::new()?;
    let repo = api.model(repo_key.to_string());
    let path = repo.get(filename)?;  // ❌ BLOCKING NETWORK I/O
    
    Ok(path)
}
```

**Issue**: Downloads files from HuggingFace using blocking sync API - this will block the entire tokio runtime!

## Solution
Convert to async using tokio API:

```rust
async fn huggingface_file(
    &self,
    repo_key: &str,
    filename: &str,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>>
{
    use hf_hub::api::tokio::Api;  // ✅ ASYNC API
    
    let api = Api::new().await?;
    let repo = api.model(repo_key.to_string());
    let path = repo.get(filename).await?;  // ✅ ASYNC NETWORK I/O
    
    Ok(path)
}
```

## Steps
1. Change method signature to `async fn`
2. Replace `hf_hub::api::sync::Api` with `hf_hub::api::tokio::Api`
3. Add `.await` to `Api::new()`
4. Add `.await` to `repo.get(filename)`
5. Find all call sites and update to await the call
6. Test with actual HuggingFace downloads

## Dependent Tasks
**⚠️ IMPORTANT**: Once this trait method is converted to async, ALL implementations and call sites must be updated.

The following tasks document all locations that need async conversion:

### Text-to-Text Models
- 📄 [ASYNC_CONVERT_TEXT_TO_TEXT_KIMI_K2.md](./ASYNC_CONVERT_TEXT_TO_TEXT_KIMI_K2.md)
  - **File**: `capability/text_to_text/kimi_k2.rs`
  - **Call Sites**: 4 (completion method + load method)
  - **Impact**: GGUF model downloads (multi-GB files)

- 📄 [ASYNC_CONVERT_TEXT_TO_TEXT_PHI4.md](./ASYNC_CONVERT_TEXT_TO_TEXT_PHI4.md)
  - **File**: `capability/text_to_text/phi4_reasoning.rs`
  - **Call Sites**: 4 (completion method + load method)
  - **Impact**: Quantized model downloads (multi-GB files)

### Text Embedding Models
- 📄 [ASYNC_CONVERT_TEXT_EMBEDDING_STELLA.md](./ASYNC_CONVERT_TEXT_EMBEDDING_STELLA.md)
  - **File**: `capability/text_embedding/stella.rs`
  - **Call Sites**: 9 (embed + batch_embed + load methods)
  - **Impact**: Base weights + MRL projection heads + tokenizers

- 📄 [ASYNC_CONVERT_TEXT_EMBEDDING_GTE_QWEN.md](./ASYNC_CONVERT_TEXT_EMBEDDING_GTE_QWEN.md)
  - **File**: `capability/text_embedding/gte_qwen.rs`
  - **Call Sites**: 10+ (embed + batch_embed + load methods)
  - **Impact**: Sharded model downloads (multiple files)

### Text-to-Image Models
- 📄 [ASYNC_CONVERT_TEXT_TO_IMAGE_SD35.md](./ASYNC_CONVERT_TEXT_TO_IMAGE_SD35.md)
  - **File**: `capability/text_to_image/stable_diffusion_35_turbo/mod.rs`
  - **Call Sites**: 8 (3 text encoders + main model + 4 tokenizers)
  - **Impact**: Multiple multi-GB model files (CLIP-G, CLIP-L, T5-XXL, MMDiT)
  - **Priority**: CRITICAL - Largest download impact

### Summary
- **Total Files**: 5 capability implementations
- **Total Call Sites**: 35+ synchronous calls
- **Conversion Order**: Must complete this task first, then all dependent tasks can be completed in parallel

## Priority
🔴 **CRITICAL** - This blocks the async runtime during network I/O

## Status
⏳ TODO
