# STELLA_1: Eliminate Massive Code Duplication

## Severity: HIGH  
**Impact**: Maintenance burden, inconsistency risk, binary bloat (~400 duplicated lines)

## Core Objective

Extract ~400 lines of duplicated code between `base.rs` and `loaded.rs` into shared utility functions in a new `utils.rs` file.

**Source Files**:
- [`base.rs`](../packages/candle/src/capability/text_embedding/stella/base.rs) - 390 lines
- [`loaded.rs`](../packages/candle/src/capability/text_embedding/stella/loaded.rs) - 385 lines
- **Duplication**: ~250 lines (64% overlap)

## Duplication Analysis

### 1. Device/DType Detection (4 instances)
**Locations**: base.rs:78-87, base.rs:243-251, loaded.rs:79-90

```rust
let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
    log::warn!("Device detection failed: {}. Using CPU.", e);
    Device::Cpu
});
let dtype = if device.is_cuda() { DType::F16 } else { DType::F32 };
```

### 2. Tokenizer Configuration (3 instances)
**Locations**: base.rs:107-144, base.rs:271-303, loaded.rs:108-145

Variant-specific padding (Large=Left with `|endoftext|>`, Small=Right) + truncation setup (38 lines each).

### 3. Model Weight Loading (2 instances)
**Locations**: base.rs:153-166, loaded.rs:154-166

```rust
let base_vb = unsafe {
    VarBuilder::from_mmaped_safetensors(&[base_weights], dtype, &device)?
};
let embed_vb = unsafe {
    VarBuilder::from_mmaped_safetensors(&[projection_head], DType::F32, &device)?
};
```

### 4. Config Creation (2 instances)
**Locations**: base.rs:147-150, loaded.rs:148-151

```rust
let stella_config = match variant {
    ModelVariant::Large => Config::new_1_5_b_v5(embed_dim),
    ModelVariant::Small => Config::new_400_m_v5(embed_dim),
};
```

## Implementation Plan

### Step 1: Create utils.rs

**File**: `/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/stella/utils.rs`

**Complete implementation**:

```rust
//! Shared utilities for Stella embedding model

use candle_core::{Device, DType};
use candle_nn::VarBuilder;
use candle_transformers::models::stella_en_v5::{Config, EmbedDim, ModelVariant};
use std::path::PathBuf;
use tokenizers::{PaddingDirection, PaddingParams, PaddingStrategy, Tokenizer, TruncationParams};

/// Detect best device and dtype
///
/// Returns (Device, DType) where Device is Metal/CUDA/CPU and DType is F16 for CUDA, F32 otherwise
pub(crate) fn detect_device_and_dtype() -> (Device, DType) {
    let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
        log::warn!("Device detection failed: {}. Using CPU.", e);
        Device::Cpu
    });
    let dtype = if device.is_cuda() { DType::F16 } else { DType::F32 };
    (device, dtype)
}

/// Configure tokenizer with variant-specific padding and truncation
///
/// # Padding
/// - Large (1.5B): Left padding with |endoftext|> token
/// - Small (400M): Right padding with default token
pub(crate) fn configure_stella_tokenizer(
    tokenizer: &mut Tokenizer,
    variant: ModelVariant,
    max_length: usize,
) -> Result<(), String> {
    // Variant-specific padding
    match variant {
        ModelVariant::Large => {
            let pad_id = tokenizer
                .token_to_id("|endoftext|>")
                .ok_or("Tokenizer missing |endoftext|> token")?;
            tokenizer.with_padding(Some(PaddingParams {
                strategy: PaddingStrategy::BatchLongest,
                direction: PaddingDirection::Left,
                pad_to_multiple_of: None,
                pad_id,
                pad_type_id: 0,
                pad_token: "|endoftext|>".to_string(),
            }));
        }
        ModelVariant::Small => {
            tokenizer.with_padding(Some(PaddingParams {
                strategy: PaddingStrategy::BatchLongest,
                direction: PaddingDirection::Right,
                ..Default::default()
            }));
        }
    }

    // Set truncation if not already set
    if tokenizer.get_truncation().is_none() {
        tokenizer
            .with_truncation(Some(TruncationParams {
                max_length,
                strategy: tokenizers::TruncationStrategy::LongestFirst,
                stride: 0,
                direction: tokenizers::TruncationDirection::Right,
            }))
            .map_err(|e| format!("Failed to set truncation: {}", e))?;
    }

    Ok(())
}

/// Create Stella config based on variant and embedding dimension
pub(crate) fn create_stella_config(variant: ModelVariant, embed_dim: EmbedDim) -> Config {
    match variant {
        ModelVariant::Large => Config::new_1_5_b_v5(embed_dim),
        ModelVariant::Small => Config::new_400_m_v5(embed_dim),
    }
}

/// Load Stella model weights (base + projection head)
///
/// # Safety
/// Uses unsafe mmap - caller must ensure files are valid SafeTensors
pub(crate) fn load_stella_weights(
    base_weights: PathBuf,
    projection_head: PathBuf,
    dtype: DType,
    device: &Device,
) -> Result<(VarBuilder<'static>, VarBuilder<'static>), String> {
    let base_vb = unsafe {
        VarBuilder::from_mmaped_safetensors(&[base_weights], dtype, device)
            .map_err(|e| format!("Failed to load base model weights: {}", e))?
    };

    let embed_vb = unsafe {
        VarBuilder::from_mmaped_safetensors(&[projection_head], DType::F32, device)
            .map_err(|e| format!("Failed to load projection head weights: {}", e))?
    };

    Ok((base_vb, embed_vb))
}
```

### Step 2: Update mod.rs

**File**: `/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/stella/mod.rs`

Add module declaration:

```rust
mod base;
mod config;
mod instruction;
mod loaded;
mod utils;  // ← ADD THIS LINE

pub use base::StellaEmbeddingModel;
pub use loaded::LoadedStellaModel;
```

### Step 3: Update base.rs

**File**: `/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/stella/base.rs`

**3a. Add import** (after line 3):

```rust
use super::config::{STELLA_400M_MODEL_INFO, detect_variant, embed_dim, get_model_info};
use super::instruction::format_with_instruction;
use super::utils::{  // ← ADD THIS
    configure_stella_tokenizer, create_stella_config, 
    detect_device_and_dtype, load_stella_weights
};
```

**3b. Replace in `embed()` method** (lines 78-166):

BEFORE (89 lines):
```rust
// Auto-detect runtime values
let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
    log::warn!("Device detection failed: {}. Using CPU.", e);
    Device::Cpu
});

let dtype = if device.is_cuda() {
    DType::F16
} else {
    DType::F32
};

// Load files via huggingface_file()
let base_weights = self
    .huggingface_file(self.info().registry_key, "model.safetensors")
    .await?;
let projection_head = self
    .huggingface_file(
        self.info().registry_key,
        &format!("2_Dense_{}/model.safetensors", dimension),
    )
    .await?;
let tokenizer_path = self
    .huggingface_file(self.info().registry_key, "tokenizer.json")
    .await?;

// Load tokenizer with variant-specific padding
let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
    .map_err(|e| format!("Failed to load tokenizer: {}", e))?;

// Configure padding based on variant
match variant {
    ModelVariant::Large => {
        // 1.5B: Left padding with |endoftext|>
        let pad_id = tokenizer
            .token_to_id("|endoftext|>")
            .ok_or("Tokenizer missing |endoftext|> token")?;

        let padding_params = PaddingParams {
            strategy: PaddingStrategy::BatchLongest,
            direction: PaddingDirection::Left,
            pad_to_multiple_of: None,
            pad_id,
            pad_type_id: 0,
            pad_token: "|endoftext|>".to_string(),
        };
        tokenizer.with_padding(Some(padding_params));
    }
    ModelVariant::Small => {
        tokenizer.with_padding(Some(PaddingParams {
            strategy: PaddingStrategy::BatchLongest,
            direction: PaddingDirection::Right,
            ..Default::default()
        }));
    }
}

// Set truncation
if tokenizer.get_truncation().is_none() {
    tokenizer
        .with_truncation(Some(TruncationParams {
            max_length,
            strategy: tokenizers::TruncationStrategy::LongestFirst,
            stride: 0,
            direction: tokenizers::TruncationDirection::Right,
        }))
        .map_err(|e| format!("Failed to set truncation: {}", e))?;
}

// Create Stella model config from detected variant
let stella_config = match variant {
    ModelVariant::Large => Config::new_1_5_b_v5(embed_dim),
    ModelVariant::Small => Config::new_400_m_v5(embed_dim),
};

// Load model weights (base + projection head)
let base_vb = unsafe {
    VarBuilder::from_mmaped_safetensors(&[base_weights], dtype, &device)
        .map_err(|e| format!("Failed to load base model weights: {}", e))?
};

let embed_vb = unsafe {
    VarBuilder::from_mmaped_safetensors(
        &[projection_head],
        DType::F32,
        &device,
    )
    .map_err(|e| format!("Failed to load projection head weights: {}", e))?
};
```

AFTER (15 lines):
```rust
// Auto-detect device and dtype
let (device, dtype) = detect_device_and_dtype();

// Load files via huggingface_file()
let base_weights = self
    .huggingface_file(self.info().registry_key, "model.safetensors")
    .await?;
let projection_head = self
    .huggingface_file(
        self.info().registry_key,
        &format!("2_Dense_{}/model.safetensors", dimension),
    )
    .await?;
let tokenizer_path = self
    .huggingface_file(self.info().registry_key, "tokenizer.json")
    .await?;

// Load and configure tokenizer
let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
    .map_err(|e| format!("Failed to load tokenizer: {}", e))?;
configure_stella_tokenizer(&mut tokenizer, variant, max_length)?;

// Create config and load weights
let stella_config = create_stella_config(variant, embed_dim);
let (base_vb, embed_vb) = load_stella_weights(base_weights, projection_head, dtype, &device)?;
```

**Reduction**: 89 lines → 24 lines (73% reduction)

**3c. Repeat for `batch_embed()` method** (lines 243-318) - same pattern

### Step 4: Update loaded.rs

**File**: `/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/stella/loaded.rs`

**4a. Add import** (after line 3):

```rust
use super::config::{STELLA_400M_MODEL_INFO, detect_variant, embed_dim, get_model_info};
use super::instruction::format_with_instruction;
use super::utils::{  // ← ADD THIS
    configure_stella_tokenizer, create_stella_config,
    detect_device_and_dtype, load_stella_weights
};
```

**4b. Replace in `load()` method** (lines 79-170):

Same replacement pattern as base.rs - replace 92 lines with ~25 lines.

## Definition of Done

- [ ] `utils.rs` created with 4 functions (detect_device_and_dtype, configure_stella_tokenizer, create_stella_config, load_stella_weights)
- [ ] `mod.rs` updated with `mod utils;`
- [ ] `base.rs` imports utils and uses functions in `embed()` and `batch_embed()`
- [ ] `loaded.rs` imports utils and uses functions in `load()`
- [ ] Code compiles: `cargo check --package cyrup_candle`
- [ ] No functionality changes - purely refactoring
- [ ] Line count reduced by ~200-250 lines total

## Impact

**Before**: 775 lines (base.rs 390 + loaded.rs 385)
**After**: ~525 lines (base.rs 290 + loaded.rs 285 + utils.rs 100)
**Reduction**: 250 lines (32% reduction)

## Future Work

**Note**: The same duplication pattern exists in:
- `gte_qwen/` (base.rs 230 lines + loaded.rs 258 lines)
- `nvembed/` (base.rs 273 lines + loaded.rs 293 lines)

Consider creating `/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/common.rs` for shared utilities across all three models in a future task.

## Related Tasks

- **STELLA_3**: Recommends removing base.rs entirely (it's not used in production)
- If STELLA_3 is implemented first, this task becomes simpler (only need to extract from loaded.rs for future use)
