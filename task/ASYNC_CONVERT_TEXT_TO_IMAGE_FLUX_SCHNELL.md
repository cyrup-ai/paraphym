# Task: Convert FLUX Schnell to Use Async CandleModel::huggingface_file

## Location
`packages/candle/src/capability/text_to_image/flux_schnell.rs`

## Dependencies
- ✅ **UNBLOCKED**: Base trait is now async (ASYNC_FIX_HUGGINGFACE.md complete)

## Problem
**CRITICAL ARCHITECTURAL VIOLATION + PERFORMANCE DISASTER**

FLUX Schnell uses **BLOCKING SYNC API** inside an async stream, downloading **7 multi-GB files synchronously**. This:
1. ❌ Blocks the entire tokio runtime during downloads
2. ❌ Violates the `CandleModel` trait abstraction
3. ❌ Downloads happen serially instead of concurrently
4. ❌ No retry logic or error recovery

### Current Implementation (CATASTROPHIC)
```rust
use hf_hub::api::sync::Api;  // ❌ BLOCKING API IN ASYNC CONTEXT

let api = Api::new()?;
let flux_repo = api.model(FLUX_SCHNELL_MODEL_INFO.registry_key.to_string());
let flux_path = flux_repo.get("flux1-schnell.safetensors")?;  // ❌ BLOCKS RUNTIME
let vae_path = flux_repo.get("ae.safetensors")?;              // ❌ BLOCKS RUNTIME

let t5_repo = api.model("google/t5-v1_1-xxl".to_string());
let t5_model_path = t5_repo.get("model.safetensors")?;        // ❌ BLOCKS RUNTIME
let t5_config_path = t5_repo.get("config.json")?;             // ❌ BLOCKS RUNTIME

let t5_tok_repo = api.model("lmz/mt5-tokenizers".to_string());
let t5_tokenizer_path = t5_tok_repo.get("t5-v1_1-xxl.tokenizer.json")?;  // ❌ BLOCKS

let clip_repo = api.model("openai/clip-vit-large-patch14".to_string());
let clip_model_path = clip_repo.get("model.safetensors")?;    // ❌ BLOCKS RUNTIME
let clip_tokenizer_path = clip_repo.get("tokenizer.json")?;   // ❌ BLOCKS RUNTIME
```

## Solution
Convert to use trait method with proper async pattern:

```rust
// Remove blocking import
// use hf_hub::api::sync::Api; // ❌ DELETE

// Use trait method for each file
let flux_path = self.huggingface_file(
    FLUX_SCHNELL_MODEL_INFO.registry_key,
    "flux1-schnell.safetensors",
).await?;

let vae_path = self.huggingface_file(
    FLUX_SCHNELL_MODEL_INFO.registry_key,
    "ae.safetensors",
).await?;

let t5_model_path = self.huggingface_file(
    "google/t5-v1_1-xxl",
    "model.safetensors",
).await?;

let t5_config_path = self.huggingface_file(
    "google/t5-v1_1-xxl",
    "config.json",
).await?;

let t5_tokenizer_path = self.huggingface_file(
    "lmz/mt5-tokenizers",
    "t5-v1_1-xxl.tokenizer.json",
).await?;

let clip_model_path = self.huggingface_file(
    "openai/clip-vit-large-patch14",
    "model.safetensors",
).await?;

let clip_tokenizer_path = self.huggingface_file(
    "openai/clip-vit-large-patch14",
    "tokenizer.json",
).await?;
```

## Call Sites to Convert
**7 blocking downloads** (line ~94-180):
1. FLUX model: `flux1-schnell.safetensors` (~2GB)
2. VAE: `ae.safetensors` (~300MB)
3. T5 model: `model.safetensors` (~10GB)
4. T5 config: `config.json`
5. T5 tokenizer: `t5-v1_1-xxl.tokenizer.json`
6. CLIP model: `model.safetensors` (~500MB)
7. CLIP tokenizer: `tokenizer.json`

## Impact
- **Performance**: Currently blocks runtime for ~12GB of serial downloads
- **Correctness**: Violates async contract inside `spawn_stream`
- **Architecture**: Breaks `CandleModel` abstraction layer

## Steps
1. Remove `use hf_hub::api::sync::Api`
2. Replace all 7 `repo.get()` calls with `self.huggingface_file().await`
3. Update error handling to use trait method's Result type
4. Test compilation
5. Verify FLUX Schnell image generation works
6. Measure performance improvement (should be massive with proper async)

## Priority
🔴 **CRITICAL** - Blocking multi-GB downloads in async context is a severe bug

## Status
⏳ TODO
