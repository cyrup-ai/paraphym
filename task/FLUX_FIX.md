# Task: Fix FluxSchnell to Use huggingface_file() Method

**Status**: Ready for Execution
**Priority**: CRITICAL - Blocking
**Complexity**: Low

## Overview

FluxSchnell is the ONLY model out of 12 that violates the architecture rule: all models MUST use `huggingface_file()` for downloads, NOT direct `hf_hub::api` calls.

## Problem

**File**: `packages/candle/src/capability/text_to_image/flux_schnell.rs`
**Lines**: 175-208

FluxSchnell directly uses `hf_hub::api::sync::Api`:

```rust
use hf_hub::api::sync::Api;

let api = Api::new().map_err(|e| format!("Failed to initialize HF API: {}", e))?;

// Download all model files
let flux_repo = api.model("black-forest-labs/FLUX.1-schnell".to_string());
let flux_path = flux_repo.get("flux1-schnell.safetensors")?;
let vae_path = flux_repo.get("ae.safetensors")?;

let t5_repo = api.model("google/t5-v1_1-xxl".to_string());
let t5_model_path = t5_repo.get("model.safetensors")?;
let t5_config_path = t5_repo.get("config.json")?;

let t5_tok_repo = api.model("lmz/mt5-tokenizers".to_string());
let t5_tokenizer_path = t5_tok_repo.get("t5-v1_1-xxl.tokenizer.json")?;

let clip_repo = api.model("openai/clip-vit-large-patch14".to_string());
let clip_model_path = clip_repo.get("model.safetensors")?;
let clip_tokenizer_path = clip_repo.get("tokenizer.json")?;
```

**This is wrong.** All other models use `huggingface_file()`.

## Correct Pattern (from StableDiffusion3.5Turbo)

**File**: `packages/candle/src/capability/text_to_image/stable_diffusion_35_turbo/mod.rs`
**Lines**: 72-180

StableDiffusion3.5Turbo correctly uses `huggingface_file()`:

```rust
let clip_g_path = match model_self.huggingface_file(
    model_self.info().registry_key,
    "text_encoders/clip_g.safetensors",
).await {
    Ok(p) => p,
    Err(e) => {
        let _ = tx.send(ImageGenerationChunk::Error(format!(
            "CLIP-G download failed: {}",
            e
        )));
        return;
    }
};
```

For files from OTHER repos (not the main model repo), SD3.5 uses helper structs with `CandleModel` trait:

```rust
let clip_l_tokenizer_path = match ClipLTokenizer
    .huggingface_file(ClipLTokenizer.info().registry_key, "tokenizer.json").await
{
    Ok(p) => p,
    Err(e) => { /* error handling */ }
};
```

These helper structs are defined in separate files:
- `clip_l_tokenizer.rs` - Downloads from `openai/clip-vit-large-patch14`
- `clip_g_tokenizer.rs` - Downloads from `laion/CLIP-ViT-bigG-14-laion2B-39B-b160k`
- `t5_config.rs` - Downloads from `google/t5-v1_1-xxl`
- `t5_tokenizer.rs` - Downloads from `lmz/mt5-tokenizers`

## Solution

Rewrite FluxSchnell to follow the exact pattern from StableDiffusion3.5Turbo:

### Step 1: Create Helper Structs (like SD3.5 does)

Create 3 new files in `packages/candle/src/capability/text_to_image/`:

**File**: `flux_t5_config.rs`
```rust
//! T5 Config Download Helper for FLUX
//!
//! Provides `CandleModel` implementation for downloading config files from
//! the T5 repository.

use crate::domain::model::{CandleModelInfo, CandleProvider};
use crate::domain::model::traits::CandleModel;

static T5_CONFIG_INFO: CandleModelInfo = CandleModelInfo {
    provider: CandleProvider::Google,
    name: "t5-v1_1-xxl",
    registry_key: "google/t5-v1_1-xxl",
    // ... other fields set to None/false as needed
};

/// T5 config download helper for FLUX
pub struct FluxT5Config;

impl CandleModel for FluxT5Config {
    fn info(&self) -> &'static CandleModelInfo {
        &T5_CONFIG_INFO
    }
}
```

**File**: `flux_t5_tokenizer.rs`
```rust
//! T5 Tokenizer Download Helper for FLUX

use crate::domain::model::{CandleModelInfo, CandleProvider};
use crate::domain::model::traits::CandleModel;

static T5_TOKENIZER_INFO: CandleModelInfo = CandleModelInfo {
    provider: CandleProvider::Custom,
    name: "mt5-tokenizers",
    registry_key: "lmz/mt5-tokenizers",
    // ... other fields
};

pub struct FluxT5Tokenizer;

impl CandleModel for FluxT5Tokenizer {
    fn info(&self) -> &'static CandleModelInfo {
        &T5_TOKENIZER_INFO
    }
}
```

**File**: `flux_clip_tokenizer.rs`
```rust
//! CLIP Tokenizer Download Helper for FLUX

use crate::domain::model::{CandleModelInfo, CandleProvider};
use crate::domain::model::traits::CandleModel;

static CLIP_TOKENIZER_INFO: CandleModelInfo = CandleModelInfo {
    provider: CandleProvider::OpenAI,
    name: "clip-vit-large-patch14",
    registry_key: "openai/clip-vit-large-patch14",
    // ... other fields
};

pub struct FluxClipTokenizer;

impl CandleModel for FluxClipTokenizer {
    fn info(&self) -> &'static CandleModelInfo {
        &CLIP_TOKENIZER_INFO
    }
}
```

### Step 2: Update flux_schnell.rs Imports

Add at top of file:
```rust
mod flux_t5_config;
mod flux_t5_tokenizer;
mod flux_clip_tokenizer;

use flux_t5_config::FluxT5Config;
use flux_t5_tokenizer::FluxT5Tokenizer;
use flux_clip_tokenizer::FluxClipTokenizer;
```

### Step 3: Replace Download Logic (lines 175-208)

**Remove**:
```rust
use hf_hub::api::sync::Api;

let api = Api::new().map_err(|e| format!("Failed to initialize HF API: {}", e))?;
let dtype = device.bf16_default_to_f32();

// Download all model files
let flux_repo = api.model("black-forest-labs/FLUX.1-schnell".to_string());
let flux_path = flux_repo.get("flux1-schnell.safetensors")?;
let vae_path = flux_repo.get("ae.safetensors")?;

let t5_repo = api.model("google/t5-v1_1-xxl".to_string());
let t5_model_path = t5_repo.get("model.safetensors")?;
let t5_config_path = t5_repo.get("config.json")?;

let t5_tok_repo = api.model("lmz/mt5-tokenizers".to_string());
let t5_tokenizer_path = t5_tok_repo.get("t5-v1_1-xxl.tokenizer.json")?;

let clip_repo = api.model("openai/clip-vit-large-patch14".to_string());
let clip_model_path = clip_repo.get("model.safetensors")?;
let clip_tokenizer_path = clip_repo.get("tokenizer.json")?;
```

**Replace with** (using huggingface_file pattern):
```rust
let dtype = device.bf16_default_to_f32();

// Download FLUX model files (from main repo)
let flux_path = self.huggingface_file(
    self.info().registry_key,
    "flux1-schnell.safetensors"
).await.map_err(|e| format!("Failed to download FLUX model: {}", e))?;

let vae_path = self.huggingface_file(
    self.info().registry_key,
    "ae.safetensors"
).await.map_err(|e| format!("Failed to download VAE: {}", e))?;

// Download T5 files (from separate repos via helper structs)
let t5_model_path = FluxT5Config.huggingface_file(
    FluxT5Config.info().registry_key,
    "model.safetensors"
).await.map_err(|e| format!("Failed to download T5 model: {}", e))?;

let t5_config_path = FluxT5Config.huggingface_file(
    FluxT5Config.info().registry_key,
    "config.json"
).await.map_err(|e| format!("Failed to download T5 config: {}", e))?;

let t5_tokenizer_path = FluxT5Tokenizer.huggingface_file(
    FluxT5Tokenizer.info().registry_key,
    "t5-v1_1-xxl.tokenizer.json"
).await.map_err(|e| format!("Failed to download T5 tokenizer: {}", e))?;

// Download CLIP files
let clip_model_path = FluxClipTokenizer.huggingface_file(
    FluxClipTokenizer.info().registry_key,
    "model.safetensors"
).await.map_err(|e| format!("Failed to download CLIP model: {}", e))?;

let clip_tokenizer_path = FluxClipTokenizer.huggingface_file(
    FluxClipTokenizer.info().registry_key,
    "tokenizer.json"
).await.map_err(|e| format!("Failed to download CLIP tokenizer: {}", e))?;
```

### Step 4: Remove hf_hub Dependency

**Remove** from flux_schnell.rs imports:
```rust
use hf_hub::api::sync::Api;
```

All downloads now go through `huggingface_file()` method (from `CandleModel` trait).

## Implementation Checklist

- [ ] Create `flux_t5_config.rs` with T5 config download helper
- [ ] Create `flux_t5_tokenizer.rs` with T5 tokenizer download helper
- [ ] Create `flux_clip_tokenizer.rs` with CLIP tokenizer download helper
- [ ] Add mod declarations and imports to flux_schnell.rs
- [ ] Replace lines 175-208 with huggingface_file() calls
- [ ] Remove `use hf_hub::api::sync::Api;` import
- [ ] Run `cargo check -p paraphym_candle --color=never`
- [ ] Verify all downloads work correctly

## Success Criteria

✅ FluxSchnell uses ONLY `huggingface_file()` for all downloads
✅ NO direct `hf_hub::api` usage
✅ Follows exact same pattern as StableDiffusion3.5Turbo
✅ Helper structs implement `CandleModel` trait
✅ All 12 models now compliant with architecture
✅ `cargo check` passes

## Notes

- This matches the architecture used by StableDiffusion3.5Turbo exactly
- Helper structs are zero-allocation (just implement trait)
- `huggingface_file()` handles caching, progress display, and error handling
- After this fix, ALL models will use the unified download system
