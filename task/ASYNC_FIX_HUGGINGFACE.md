# Task: Convert CandleModel::huggingface_file to Async

## Status
✅ **TRAIT METHOD COMPLETE** - Async conversion successful and production-ready

## Remaining Work
⏳ **Call site updates in progress** - Covered by dependent task files (see below)

## Current Implementation
`packages/candle/src/domain/model/traits.rs` lines 91-105

```rust
async fn huggingface_file(
    &self,
    repo_key: &str,
    filename: &str,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>>
where
    Self: Sized,
{
    use hf_hub::api::tokio::Api;

    let api = Api::new()?;                    // ✅ Returns Result (no .await)
    let repo = api.model(repo_key.to_string());
    let path = repo.get(filename).await?;     // ✅ Returns Future (.await required)

    Ok(path)
}
```

**✅ Completed Changes:**
1. ✅ Method signature is `async fn`
2. ✅ Uses `hf_hub::api::tokio::Api` (non-blocking async API)
3. ✅ `Api::new()` correctly has NO `.await` (returns `Result<Api, ApiError>`)
4. ✅ `repo.get(filename).await?` correctly HAS `.await` (returns `Future`)

**⚠️ Critical Note**: The tokio API's `Api::new()` returns `Result`, NOT `Future`, so it must NOT be awaited.

## Next Steps - Call Site Updates

**Current Status**: Trait method is complete and production-ready. However, ~95 call sites across multiple files need `.await` added.

**Compilation Status**: ❌ 50+ errors from call sites missing `.await`

The following dependent tasks document locations that need async conversion:

### Text-to-Text Models
- 📄 [ASYNC_CONVERT_TEXT_TO_TEXT_KIMI_K2.md](./ASYNC_CONVERT_TEXT_TO_TEXT_KIMI_K2.md)
  - **File**: `capability/text_to_text/kimi_k2.rs`
  - **Call Sites**: 4 (completion method + load method)
  - **Impact**: GGUF model downloads (multi-GB files)

- 📄 [ASYNC_CONVERT_TEXT_TO_TEXT_PHI4.md](./ASYNC_CONVERT_TEXT_TO_TEXT_PHI4.md)
  - **File**: `capability/text_to_text/phi4_reasoning.rs`
  - **Call Sites**: 4 (completion method + load method)
  - **Impact**: Quantized model downloads (multi-GB files)

- 📄 [ASYNC_CONVERT_TEXT_TO_TEXT_QWEN3_CODER.md](./ASYNC_CONVERT_TEXT_TO_TEXT_QWEN3_CODER.md)
  - **File**: `capability/text_to_text/qwen3_coder.rs`
  - **Call Sites**: 2 (uses tokio API directly - architectural violation)
  - **Impact**: GGUF model + tokenizer downloads
  - **Priority**: CRITICAL - Must use trait method, not bypass abstraction

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

- 📄 [ASYNC_CONVERT_TEXT_TO_IMAGE_FLUX_SCHNELL.md](./ASYNC_CONVERT_TEXT_TO_IMAGE_FLUX_SCHNELL.md)
  - **File**: `capability/text_to_image/flux_schnell.rs`
  - **Call Sites**: 7 (uses BLOCKING SYNC API - catastrophic!)
  - **Impact**: ~12GB of models (FLUX, VAE, T5, CLIP) downloaded SYNCHRONOUSLY
  - **Priority**: 🔥 CRITICAL - Blocks entire tokio runtime during downloads

### Image Embedding Models
- 📄 [ASYNC_CONVERT_IMAGE_EMBEDDING_CLIP_VISION.md](./ASYNC_CONVERT_IMAGE_EMBEDDING_CLIP_VISION.md)
  - **File**: `capability/image_embedding/clip_vision.rs`
  - **Call Sites**: 4
  - **Impact**: CLIP vision model weights

### Additional Text Embedding Models
- 📄 [ASYNC_CONVERT_TEXT_EMBEDDING_BERT.md](./ASYNC_CONVERT_TEXT_EMBEDDING_BERT.md)
  - **File**: `capability/text_embedding/bert.rs`
  - **Call Sites**: 9 (model weights + tokenizer + config)
  - **Impact**: BERT-based embedding models

- 📄 [ASYNC_CONVERT_TEXT_EMBEDDING_JINA_BERT.md](./ASYNC_CONVERT_TEXT_EMBEDDING_JINA_BERT.md)
  - **File**: `capability/text_embedding/jina_bert.rs`
  - **Call Sites**: 6
  - **Impact**: Jina BERT embedding models

- 📄 [ASYNC_CONVERT_TEXT_EMBEDDING_NVEMBED.md](./ASYNC_CONVERT_TEXT_EMBEDDING_NVEMBED.md)
  - **File**: `capability/text_embedding/nvembed.rs`
  - **Call Sites**: 4
  - **Impact**: NVIDIA NVEmbed models

### Vision Models
- 📄 [ASYNC_CONVERT_VISION_LLAVA.md](./ASYNC_CONVERT_VISION_LLAVA.md)
  - **File**: `capability/vision/llava.rs`
  - **Call Sites**: 3
  - **Impact**: LLaVA multimodal vision model

### Summary
- **Total Files**: 16 capability implementations identified
- **Total Call Sites**: ~85+ (69 trait method calls + 2 direct tokio API + 7 blocking sync API + others)
- **Task Files Created**: 12 conversion task files
- **Conversion Order**: ✅ Base trait complete → Now update dependent call sites in parallel

### Critical Architectural Violations Found
⚠️ **Files bypassing CandleModel trait**:
- `flux_schnell.rs` - Uses **BLOCKING sync API** (🔥 catastrophic)
- `qwen3_coder.rs` - Uses tokio API directly (architectural violation)

## Priority
🔴 **HIGH** - Codebase doesn't compile until call sites are updated

## Core Implementation Status
✅ **COMPLETE** - Trait method is production-ready and correct
