# DECOMP_022: Decompose `stella.rs` into Focused Modules

**File:** `packages/candle/src/capability/text_embedding/stella.rs`  
**Current Size:** 883 lines  
**Module Area:** capability / text_embedding  
**Target Structure:** `stella/` subdirectory with 5 focused modules

---

## OBJECTIVE

Decompose the monolithic `stella.rs` (883 lines) into a modular directory structure following the established **nvembed pattern**, creating 5 focused modules that preserve all existing functionality while improving maintainability.

---

## RESEARCH FINDINGS

### Established Pattern from nvembed

The [nvembed decomposition](../src/capability/text_embedding/nvembed/) provides a proven blueprint:

```
nvembed/
├── mod.rs          # Module aggregator with re-exports
├── config.rs       # Static model info + helper methods
├── instruction.rs  # Task-specific instruction formatting
├── base.rs         # Base model struct + trait implementations
└── loaded.rs       # Loaded model wrapper + trait implementations
```

Reference files:
- [nvembed/mod.rs](../src/capability/text_embedding/nvembed/mod.rs) - ~13 lines
- [nvembed/config.rs](../src/capability/text_embedding/nvembed/config.rs) - ~44 lines
- [nvembed/instruction.rs](../src/capability/text_embedding/nvembed/instruction.rs) - ~172 lines
- [nvembed/base.rs](../src/capability/text_embedding/nvembed/base.rs) - ~268 lines

### Dependencies (All Available)

From [packages/candle/Cargo.toml](../packages/candle/Cargo.toml):
```toml
candle-core = { git = "https://github.com/huggingface/candle", branch = "main" }
candle-nn = { git = "https://github.com/huggingface/candle", branch = "main" }
candle-transformers = { git = "https://github.com/huggingface/candle", branch = "main" }
tokenizers = "0.22.1"
```

Current stella.rs imports:
```rust
use candle_transformers::models::stella_en_v5::{Config, EmbedDim, EmbeddingModel, ModelVariant};
```

✅ All dependencies already available in the codebase.

---

## TARGET MODULE STRUCTURE

Create `packages/candle/src/capability/text_embedding/stella/` with 5 files:

### 1. **stella/config.rs** (~60 lines)
**Purpose:** Static model info + variant detection helpers

**Contents:**
- `pub(crate) static STELLA_MODEL_INFO: CandleModelInfo` (lines 117-162 from original)
- `pub(crate) fn detect_variant(registry_key: &str) -> ModelVariant` (lines 44-51)
- `pub(crate) fn embed_dim(dimension: u32) -> Result<EmbedDim>` (lines 54-67)

**Pattern:**
```rust
//! Stella model configuration and helpers

use crate::domain::model::CandleModelInfo;
use candle_transformers::models::stella_en_v5::{EmbedDim, ModelVariant};
use std::num::NonZeroU32;

pub(crate) static STELLA_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    // ... static config ...
};

pub(crate) fn detect_variant(registry_key: &str) -> ModelVariant {
    if registry_key.contains("1.5B") {
        ModelVariant::Large
    } else {
        ModelVariant::Small
    }
}

pub(crate) fn embed_dim(dimension: u32) -> std::result::Result<EmbedDim, Box<dyn std::error::Error + Send + Sync>> {
    match dimension {
        256 => Ok(EmbedDim::Dim256),
        // ... other dimensions ...
        _ => Err(format!("Unsupported dimension: {}", dimension).into()),
    }
}
```

---

### 2. **stella/instruction.rs** (~35 lines)
**Purpose:** Task-specific instruction formatting

**Contents:**
- `pub(crate) fn format_with_instruction(texts: &[&str], task: Option<&str>) -> Vec<String>` (lines 70-95)

**Pattern:**
```rust
//! Task-specific instruction formatting for Stella embeddings

pub(crate) fn format_with_instruction(texts: &[&str], task: Option<&str>) -> Vec<String> {
    let instruct = match task {
        Some("s2p") => "Given a web search query, retrieve relevant passages that answer the query.",
        Some("s2s") => "Retrieve semantically similar text.",
        // ... other task types ...
        _ => "Given a web search query, retrieve relevant passages that answer the query.",
    };

    texts
        .iter()
        .map(|text| format!("Instruct: {}\nQuery: {}", instruct, text))
        .collect()
}
```

---

### 3. **stella/base.rs** (~280 lines)
**Purpose:** Base StellaEmbeddingModel struct + TextEmbeddingCapable trait

**Contents:**
- `pub struct StellaEmbeddingModel {}` (lines 28-33)
- `impl StellaEmbeddingModel` methods using config module
- `impl CandleModel for StellaEmbeddingModel` (lines 164-168)
- `impl TextEmbeddingCapable for StellaEmbeddingModel` (lines 170-449)
  - `fn embed()` - Single text embedding (lines 171-283)
  - `fn batch_embed()` - Batch embedding (lines 285-425)
  - Helper methods: `embedding_dimension()`, `supported_dimensions()`, etc.

**Key Pattern - Variant-Specific Padding:**
```rust
match variant {
    ModelVariant::Large => {
        // 1.5B: Left padding with <|endoftext|>
        let pad_id = tokenizer.token_to_id("<|endoftext|>")?;
        let padding_params = PaddingParams {
            strategy: PaddingStrategy::BatchLongest,
            direction: PaddingDirection::Left,  // ← LEFT for 1.5B
            pad_id,
            pad_token: "<|endoftext|>".to_string(),
            ..Default::default()
        };
        tokenizer.with_padding(Some(padding_params));
    }
    ModelVariant::Small => {
        // 400M: Right padding
        tokenizer.with_padding(Some(PaddingParams {
            strategy: PaddingStrategy::BatchLongest,
            direction: PaddingDirection::Right,  // ← RIGHT for 400M
            ..Default::default()
        }));
    }
}
```

**Imports:**
```rust
use super::config::{STELLA_MODEL_INFO, detect_variant, embed_dim};
use super::instruction::format_with_instruction;
use crate::capability::traits::TextEmbeddingCapable;
use crate::domain::model::traits::CandleModel;
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::stella_en_v5::{Config, EmbeddingModel};
use tokenizers::{PaddingDirection, PaddingParams, PaddingStrategy, Tokenizer, TruncationParams};
```

---

### 4. **stella/loaded.rs** (~280 lines)
**Purpose:** Loaded model wrapper with Arc<Mutex<EmbeddingModel>> for thread safety

**Contents:**
- `pub struct LoadedStellaModel` (lines 465-472)
- `impl std::fmt::Debug for LoadedStellaModel` (lines 474-483)
- `impl CandleModel for LoadedStellaModel` (lines 485-489)
- `impl LoadedStellaModel::load()` async constructor (lines 491-601)
- `impl TextEmbeddingCapable for LoadedStellaModel` (lines 685-883)
  - `fn embed()` with spawn_blocking (lines 686-779)
  - `fn batch_embed()` with spawn_blocking (lines 781-868)

**Key Pattern - Thread-Safe Interior Mutability:**
```rust
#[derive(Clone)]
pub struct LoadedStellaModel {
    tokenizer: Tokenizer,
    model: std::sync::Arc<std::sync::Mutex<EmbeddingModel>>,  // ← Thread-safe
    device: Device,
    config: Config,
    variant: ModelVariant,
}

impl TextEmbeddingCapable for LoadedStellaModel {
    fn embed(&self, text: &str, task: Option<String>) -> Pin<Box<dyn Future<Output = Result<Vec<f32>>> + Send + '_>> {
        let text = text.to_string();
        let tokenizer = self.tokenizer.clone();
        let model = self.model.clone();
        let device = self.device.clone();
        
        Box::pin(async move {
            // Wrap CPU-intensive operations in spawn_blocking
            let embedding_vec = tokio::task::spawn_blocking(move || {
                // ... tokenization ...
                
                // Lock synchronous mutex in blocking context
                let mut model_guard = model.lock()
                    .map_err(|_| "Failed to lock model mutex")?;
                let embeddings = model_guard.forward_norm(&input_ids, &attention_mask)?;
                
                // ... extract embedding ...
                Ok::<Vec<f32>, String>(embedding_vec)
            })
            .await
            .map_err(|e| format!("Spawn blocking failed: {}", e))??;
            
            Ok(embedding_vec)
        })
    }
}
```

**Imports:**
```rust
use super::config::{STELLA_MODEL_INFO, detect_variant, embed_dim};
use super::instruction::format_with_instruction;
use crate::capability::traits::TextEmbeddingCapable;
use crate::domain::model::traits::CandleModel;
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::stella_en_v5::{Config, EmbeddingModel, ModelVariant};
use tokenizers::{PaddingDirection, PaddingParams, PaddingStrategy, Tokenizer, TruncationParams};
```

---

### 5. **stella/mod.rs** (~10 lines)
**Purpose:** Module aggregator maintaining public API

**Contents:**
```rust
//! Stella embedding provider for local inference using Candle ML framework
//!
//! This provider uses dunzhang/stella_en_400M_v5 or dunzhang/stella_en_1.5B_v5 models
//! for generating MRL-trained dimensional embeddings with ProgressHub download and Candle inference.
//!
//! Supports only trained MRL projection dimensions: 256, 768, 1024, 2048, 4096, 6144, 8192.
//! Architecture follows the real Candle EmbeddingModel pattern with native lm_head projections.

mod config;
mod instruction;
mod base;
mod loaded;

// Public API - maintain exact same exports as original stella.rs
pub use base::StellaEmbeddingModel;
pub use loaded::LoadedStellaModel;
```

---

## STEP-BY-STEP EXECUTION PLAN

### Phase 1: Create Directory Structure
```bash
mkdir -p packages/candle/src/capability/text_embedding/stella
```

### Phase 2: Create Files in Order

#### Step 1: Create `stella/config.rs`
Extract lines 117-162 (STELLA_MODEL_INFO) + helper methods (lines 44-67).

**Source lines from stella.rs:**
- Lines 117-162: Static STELLA_MODEL_INFO
- Lines 44-51: detect_variant() method
- Lines 54-67: embed_dim() method

**New location:** `packages/candle/src/capability/text_embedding/stella/config.rs`

---

#### Step 2: Create `stella/instruction.rs`
Extract lines 70-95 (format_with_instruction).

**Source lines from stella.rs:**
- Lines 70-95: format_with_instruction() method

**New location:** `packages/candle/src/capability/text_embedding/stella/instruction.rs`

---

#### Step 3: Create `stella/base.rs`
Extract StellaEmbeddingModel struct and trait implementations.

**Source lines from stella.rs:**
- Lines 28-33: struct StellaEmbeddingModel {}
- Lines 35-42: impl Default + new()
- Lines 164-168: impl CandleModel
- Lines 170-449: impl TextEmbeddingCapable
  - Lines 171-283: embed()
  - Lines 285-425: batch_embed()
  - Lines 427-449: helper methods

**Modifications:**
- Replace `self.detect_variant()` with `config::detect_variant(self.info().registry_key)`
- Replace `self.embed_dim()` with `config::embed_dim()`
- Replace `self.format_with_instruction()` with `instruction::format_with_instruction()`

**New location:** `packages/candle/src/capability/text_embedding/stella/base.rs`

---

#### Step 4: Create `stella/loaded.rs`
Extract LoadedStellaModel struct and implementations.

**Source lines from stella.rs:**
- Lines 465-472: struct LoadedStellaModel
- Lines 474-483: impl Debug
- Lines 485-489: impl CandleModel
- Lines 491-601: impl LoadedStellaModel::load()
- Lines 603-683: helper methods
- Lines 685-883: impl TextEmbeddingCapable
  - Lines 686-779: embed()
  - Lines 781-868: batch_embed()
  - Lines 870-883: helper methods

**Modifications:**
- Replace helper calls with config module functions
- Replace format_with_instruction calls with instruction module

**New location:** `packages/candle/src/capability/text_embedding/stella/loaded.rs`

---

#### Step 5: Create `stella/mod.rs`
Create module aggregator with re-exports.

**New location:** `packages/candle/src/capability/text_embedding/stella/mod.rs`

---

#### Step 6: Update Parent Module
Update `packages/candle/src/capability/text_embedding/mod.rs`:

**Change from:**
```rust
pub mod stella;
pub use stella::StellaEmbeddingModel;
```

**Change to:**
```rust
pub mod stella;
pub use stella::{StellaEmbeddingModel, LoadedStellaModel};
```

---

#### Step 7: Delete Original File
```bash
rm packages/candle/src/capability/text_embedding/stella.rs
```

---

### Phase 3: Verification
```bash
cargo check -p cyrup_candle
```

**Expected outcome:** No compilation errors, all existing dependents still compile.

---

## KEY TECHNICAL DETAILS

### Variant Detection
- **Large (1.5B)**: registry_key contains "1.5B" → Left padding with `<|endoftext|>`
- **Small (400M)**: Default → Right padding with standard pad token

### MRL Projection Dimensions
Supported dimensions (Matryoshka Representation Learning):
- 256, 768, 1024, 2048, 4096, 6144, 8192

Each dimension has a separate projection head file:
```
2_Dense_{dimension}/model.safetensors
```

### Thread Safety Pattern
`LoadedStellaModel` uses `Arc<Mutex<EmbeddingModel>>` for thread-safe interior mutability in spawn_blocking context. This is necessary because:
1. Trait signature uses `&self` (immutable reference)
2. `forward_norm()` requires `&mut self`
3. `spawn_blocking` requires `Send + 'static`

### File Loading Pattern
Uses `huggingface_file()` from `CandleModel` trait:
```rust
let base_weights = self.huggingface_file(self.info().registry_key, "model.safetensors").await?;
let projection_head = self.huggingface_file(
    self.info().registry_key,
    &format!("2_Dense_{}/model.safetensors", dimension),
).await?;
```

---

## LINE COUNT BREAKDOWN

| File | Lines | Responsibility |
|------|-------|----------------|
| **stella/config.rs** | ~60 | Static info + helpers |
| **stella/instruction.rs** | ~35 | Task formatting |
| **stella/base.rs** | ~280 | Base model + traits |
| **stella/loaded.rs** | ~280 | Loaded model + traits |
| **stella/mod.rs** | ~10 | Module aggregator |
| **Total** | ~665 | (Original: 883 lines) |

Reduction: ~218 lines through:
- Eliminated duplication in helper methods
- Consolidated repeated patterns
- Clearer module boundaries

---

## DEFINITION OF DONE

✅ **Structural Requirements:**
- [ ] `stella.rs` deleted
- [ ] `stella/` directory created with 5 files
- [ ] All files < 300 lines
- [ ] Module hierarchy matches nvembed pattern

✅ **Functional Requirements:**
- [ ] Public API unchanged: `StellaEmbeddingModel` and `LoadedStellaModel` exported
- [ ] All trait implementations preserved
- [ ] Variant detection works for both 400M and 1.5B models
- [ ] Padding strategy correct for each variant (Left vs Right)
- [ ] MRL projection loading functional for all dimensions

✅ **Compilation Requirements:**
- [ ] `cargo check -p cyrup_candle` passes
- [ ] No broken imports in dependent modules
- [ ] No visibility issues (pub/pub(crate) correct)

✅ **Code Quality:**
- [ ] Module-level documentation present
- [ ] Helper functions have pub(crate) visibility
- [ ] Imports organized and minimal

---

## CONSTRAINTS

- ❌ **NO TESTS:** Do not write any unit tests, integration tests, or test code
- ❌ **NO BENCHMARKS:** Do not write any benchmark code
- ❌ **NO DOCUMENTATION FILES:** Do not create separate markdown docs
- ✅ **PRESERVE FUNCTIONALITY:** All existing behavior must remain identical
- ✅ **SINGLE SESSION:** Completable in one focused work session

---

## SUCCESS CRITERIA

This task is successful when:
1. The original monolithic file is decomposed into 5 focused modules
2. Each module has a single, clear responsibility
3. Public API surface is unchanged
4. All functionality preserved without behavior changes
5. `cargo check -p cyrup_candle` passes without errors
6. Code is more maintainable and follows established patterns
