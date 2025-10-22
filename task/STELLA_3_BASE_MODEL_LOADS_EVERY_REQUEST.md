# Issue: Base Model Reloads Entire Model on Every Request

## Severity: CRITICAL
**Impact**: Extreme performance degradation, 1000x+ slower than necessary if accidentally used

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/stella/base.rs`

**Lines to modify**: 48-386 (entire `TextEmbeddingCapable` implementation)

## Core Objective

**Remove the dangerous `TextEmbeddingCapable` implementation from `StellaEmbeddingModel` (base.rs) that reloads the entire model from disk on every embedding request.**

The base.rs struct should ONLY serve as:
1. A registry holder (implements `CandleModel` trait)
2. An input parameter to `LoadedStellaModel::load()`

It should NOT be usable for direct inference.

## Problem Description

The `StellaEmbeddingModel` in [base.rs](../../packages/candle/src/capability/text_embedding/stella/base.rs) is a **zero-state struct** that implements `TextEmbeddingCapable` with full `embed()` and `batch_embed()` methods:

```rust
// base.rs:23
pub struct StellaEmbeddingModel {}  // ← No state!

// base.rs:48-215
impl TextEmbeddingCapable for StellaEmbeddingModel {
    fn embed(&self, text: &str, task: Option<String>) -> ... {
        Box::pin(async move {
            // Lines 89-100: Load files from disk EVERY TIME
            let base_weights = self.huggingface_file(...).await?;
            let projection_head = self.huggingface_file(...).await?;
            let tokenizer_path = self.huggingface_file(...).await?;
            
            // Lines 103-169: Recreate tokenizer and model EVERY TIME
            let mut tokenizer = Tokenizer::from_file(&tokenizer_path)?;
            let mut model = EmbeddingModel::new(&stella_config, base_vb, embed_vb)?;
            
            // Only NOW does it actually embed
            let embeddings = model.forward_norm(...)?;
        })
    }
    
    // batch_embed() has the same problem (lines 217-386)
}
```

## Architecture Analysis

### Current Architecture (Base + Loaded Pattern)

The Stella implementation uses a two-struct pattern:

**1. base.rs - `StellaEmbeddingModel`** ([source](../../packages/candle/src/capability/text_embedding/stella/base.rs))
- Stateless struct (no fields)
- Implements `CandleModel` trait (provides `info()` and `huggingface_file()`)
- **PROBLEM**: Also implements `TextEmbeddingCapable` with full inference methods
- Used as registry holder and input to `LoadedStellaModel::load()`

**2. loaded.rs - `LoadedStellaModel`** ([source](../../packages/candle/src/capability/text_embedding/stella/loaded.rs))
- Stateful struct with fields: `tokenizer`, `model`, `device`, `config`, `variant`
- Has `load()` method that loads everything ONCE
- Implements `TextEmbeddingCapable` efficiently (reuses loaded model)
- Used by worker pool for all actual inference

### How Worker Pool Uses These Structs

From [text_embedding.rs:103-108](../../packages/candle/src/capability/registry/text_embedding.rs#L103-L108):

```rust
pool.spawn_text_embedding_worker(
    registry_key,
    move || async move {
        LoadedStellaModel::load(&m_clone)  // ← Uses base model as input
            .await
            .map_err(|e| PoolError::SpawnFailed(e.to_string()))
    },
    per_worker_mb,
    allocation_guard,
)
```

**Flow:**
1. `TextEmbeddingModel::Stella(Arc<StellaEmbeddingModel>)` stored in registry
2. When `embed()` called on enum, routes to `spawn_embed_stella()`
3. Worker spawner calls `LoadedStellaModel::load(&base_model)`
4. `LoadedStellaModel::load()` uses base model's `info()` and `huggingface_file()` methods
5. Worker pool uses `LoadedStellaModel` for all inference

### Why Base.rs Methods Are Dangerous

The base.rs `embed()` and `batch_embed()` methods are **never used in production** because:
- The worker pool always uses `LoadedStellaModel` (via the macro pattern)
- The enum routing goes through `spawn_embed_stella()` which spawns workers

**BUT** if someone accidentally calls them directly:
```rust
let model = StellaEmbeddingModel::new();
model.embed("text", None).await?;  // ← DISASTER! Reloads 400MB-1.5GB
```

This would cause:
- **stella_en_400M_v5**: 2-5 seconds per request (400MB reload) vs 50-100ms (20-100x slower)
- **stella_en_1.5B_v5**: 8-15 seconds per request (1.5GB reload) vs 150-300ms (50-100x slower)

## Performance Impact

### Model Loading Overhead

**stella_en_400M_v5:**
- Model weights: ~400MB
- Projection head: ~50MB per dimension
- Loading time: 2-5 seconds (cold)
- Inference time: 50-100ms
- **Overhead: 20-100x slower**

**stella_en_1.5B_v5:**
- Model weights: ~1.5GB
- Projection head: ~50MB per dimension
- Loading time: 8-15 seconds (cold)
- Inference time: 150-300ms
- **Overhead: 50-100x slower**

### What Gets Reloaded Every Time

From base.rs lines 89-169:

1. **File downloads** (lines 89-100):
   - `model.safetensors` (400MB or 1.5GB)
   - `2_Dense_{dimension}/model.safetensors` (~50MB)
   - `tokenizer.json` (~2MB)

2. **Tokenizer creation** (lines 103-147):
   - Load from file
   - Configure padding (variant-specific)
   - Set truncation params

3. **Model creation** (lines 149-169):
   - Create Stella config
   - Load base weights into VarBuilder
   - Load projection head weights
   - Create EmbeddingModel instance

All of this happens **on every single embed() call**.

## Related Files

This pattern exists in 3 embedding models:
- [stella/base.rs](../../packages/candle/src/capability/text_embedding/stella/base.rs) + [stella/loaded.rs](../../packages/candle/src/capability/text_embedding/stella/loaded.rs)
- [nvembed/base.rs](../../packages/candle/src/capability/text_embedding/nvembed/base.rs) + [nvembed/loaded.rs](../../packages/candle/src/capability/text_embedding/nvembed/loaded.rs)
- [gte_qwen/base.rs](../../packages/candle/src/capability/text_embedding/gte_qwen/base.rs) + [gte_qwen/loaded.rs](../../packages/candle/src/capability/text_embedding/gte_qwen/loaded.rs)

**This task focuses on Stella only.** The same fix can be applied to the others later.

## What Needs to Change

### File: base.rs

**REMOVE** the entire `TextEmbeddingCapable` implementation (lines 48-386).

**KEEP**:
- Struct definition (line 23)
- `Default` impl (lines 25-29)
- `impl StellaEmbeddingModel` block (lines 31-37)
- `CandleModel` impl (lines 39-46)

**The struct should look like this after changes:**

```rust
//! Base Stella embedding model implementation

use super::config::{STELLA_400M_MODEL_INFO, detect_variant, embed_dim};
use crate::domain::model::CandleModelInfo;
use crate::domain::model::traits::CandleModel;

/// Stella embedding provider - registry holder only
///
/// This struct serves as a registry holder and provides model metadata.
/// It is NOT meant for direct inference - use LoadedStellaModel via the worker pool.
///
/// # Usage
/// ```rust,ignore
/// // CORRECT: Via worker pool (automatic)
/// let model = TextEmbeddingModel::Stella(Arc::new(StellaEmbeddingModel::new()));
/// model.embed("text", None).await?;  // Routes through pool → LoadedStellaModel
///
/// // WRONG: Direct usage (now prevented)
/// let model = StellaEmbeddingModel::new();
/// model.embed("text", None).await?;  // ← Compile error!
/// ```
#[derive(Debug, Clone)]
pub struct StellaEmbeddingModel {}

impl Default for StellaEmbeddingModel {
    fn default() -> Self {
        Self::new()
    }
}

impl StellaEmbeddingModel {
    /// Create new Stella embedding provider
    #[inline]
    pub fn new() -> Self {
        Self {}
    }
}

impl CandleModel for StellaEmbeddingModel {
    fn info(&self) -> &'static CandleModelInfo {
        // Default to 400M variant
        // Note: This is only used for registry lookup. Actual variant is detected
        // from registry_key during model loading.
        &STELLA_400M_MODEL_INFO
    }
}

// TextEmbeddingCapable implementation REMOVED
// Use LoadedStellaModel via worker pool instead
```

### Why This Works

**The base.rs struct is still needed because:**
1. `LoadedStellaModel::load()` takes `&StellaEmbeddingModel` as parameter (line 63 of loaded.rs)
2. The load method uses `base_model.info()` and `base_model.huggingface_file()` (lines 65-100 of loaded.rs)
3. The registry stores `TextEmbeddingModel::Stella(Arc<StellaEmbeddingModel>)` (line 55-57 of storage.rs)

**Removing TextEmbeddingCapable is safe because:**
1. The enum `TextEmbeddingModel` implements `TextEmbeddingCapable` (line 16-75 of text_embedding.rs)
2. The enum routes to `spawn_embed_stella()` which uses the worker pool (line 35 of text_embedding.rs)
3. The worker pool always uses `LoadedStellaModel` (line 106 of text_embedding.rs)
4. Direct usage of `StellaEmbeddingModel::embed()` would now be a compile error

## Implementation Steps

### Step 1: Edit base.rs

Open [base.rs](../../packages/candle/src/capability/text_embedding/stella/base.rs):

1. **Delete lines 48-386** (entire `impl TextEmbeddingCapable for StellaEmbeddingModel` block)
2. **Update imports** - remove unused imports:
   - Remove `TextEmbeddingCapable` from imports (line 8)
   - Remove `candle_core`, `candle_nn`, `candle_transformers` imports (lines 9-12)
   - Remove `tokenizers` imports (line 13)
3. **Update doc comment** (lines 17-21) to clarify this is registry-only

### Step 2: Verify Compilation

```bash
cd /Volumes/samsung_t9/cyrup
cargo check -p cyrup_candle
```

Should compile successfully because:
- `LoadedStellaModel` still has `TextEmbeddingCapable` impl
- Worker pool uses `LoadedStellaModel`, not base model
- Enum routing still works

### Step 3: Verify No Direct Usage

Search for any direct calls to base model methods:

```bash
cd /Volumes/samsung_t9/cyrup/packages/candle
rg "StellaEmbeddingModel.*\.embed" --type rust
```

Should return NO results (except in this task file).

## Definition of Done

✅ `StellaEmbeddingModel` struct exists with `CandleModel` impl  
✅ `TextEmbeddingCapable` impl removed from base.rs  
✅ `LoadedStellaModel` still has `TextEmbeddingCapable` impl (unchanged)  
✅ Code compiles: `cargo check -p cyrup_candle`  
✅ Worker pool still uses `LoadedStellaModel` (no code changes needed)  
✅ Direct usage of `StellaEmbeddingModel::embed()` is now a compile error

## Why This Fix Is Correct

**Before:**
- base.rs: Implements both `CandleModel` AND `TextEmbeddingCapable` (dangerous)
- loaded.rs: Implements `TextEmbeddingCapable` (efficient)
- Worker pool uses loaded.rs ✓
- Direct usage of base.rs possible but disastrous ✗

**After:**
- base.rs: Implements only `CandleModel` (safe)
- loaded.rs: Implements `TextEmbeddingCapable` (efficient) - unchanged
- Worker pool uses loaded.rs ✓
- Direct usage of base.rs is compile error ✓

## Notes

- This same pattern exists in `nvembed` and `gte_qwen` but is out of scope for this task
- The `bert` and `jina_bert` models do NOT have this pattern (they only have base.rs, no loaded.rs)
- After this fix, `StellaEmbeddingModel` becomes a pure registry holder with no inference capability
- All inference goes through `LoadedStellaModel` via the worker pool (as intended)
