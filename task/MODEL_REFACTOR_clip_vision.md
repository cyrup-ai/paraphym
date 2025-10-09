# Task: Fix Critical Runtime Bugs in CLIP Vision Embedding Wrapper

## QA RATING: 3/10 - CRITICAL RUNTIME BUGS

## Files
- `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/image_embedding/clip_vision.rs` ✅ CORRECT
- `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/image_embedding/clip_vision_embedding.rs` ❌ CRITICALLY BROKEN

## Summary

The underlying `ClipVisionModel` implementation is **correct and consistent** (all Base model, 512D, 224x224). 

However, the `ClipVisionEmbeddingModel` wrapper has **critical runtime bugs** that will cause panics and incorrect behavior.

## CRITICAL BUGS

### 1. Default::default() PANICS on Construction 🔥

**Location**: clip_vision_embedding.rs lines 30-45, 51-62

**Problem**: Default construction path will panic at runtime:
```rust
// Line 30-37: Default impl
impl Default for ClipVisionEmbeddingModel {
    fn default() -> Self {
        runtime.block_on(Self::new())  // calls new()
    }
}

// Line 42-45: new() calls with_dimension(512)
pub async fn new() -> Result<Self> {
    Self::with_dimension(512).await  // ← Passes 512
}

// Line 51-62: Validation REJECTS 512!
pub async fn with_dimension(dimension: usize) -> Result<Self> {
    if dimension != 768 {  // ← ONLY accepts 768
        return Err(MemoryError::Config(format!(
            "CLIP Vision currently configured for 768D (ViT-Large). Requested: {}",
            dimension  // ← Will error with "Requested: 512"
        )));
    }
}
```

**Result**: `ClipVisionEmbeddingModel::default()` will **PANIC** because:
- Default → new() → with_dimension(512) → Error("currently configured for 768D")

**Fix Required**:
```rust
pub async fn new() -> Result<Self> {
    Self::with_dimension(768).await  // Match the validation logic
}
```

### 2. Dimension Mismatch: Reports 768D but Returns 512D Embeddings 🔥

**Location**: clip_vision_embedding.rs lines 51-68, 244-247

**Problem**: Wrapper accepts dimension=768 but underlying model produces 512D embeddings:

```rust
// Wrapper stores dimension = 768
pub async fn with_dimension(dimension: usize) -> Result<Self> {
    if dimension != 768 { return Err(...); }
    let provider = ClipVisionModel::new();  // ← Uses Base model (512D)
    Ok(Self { provider: Arc::new(provider), dimension })  // ← Stores 768
}

// Wrapper reports 768D
fn embedding_dimension(&self) -> usize {
    self.dimension  // Returns 768
}

// But actual embeddings are 512D (from ClipVisionModel)
```

**Data Flow**:
1. User calls `with_dimension(768)` → validation passes
2. Creates `ClipVisionModel::new()` → uses Base model (512D)
3. Wrapper stores `dimension = 768`
4. `embed_image()` returns 512D vector (from underlying model)
5. `embedding_dimension()` claims 768D

**Result**: **Silently corrupts data** - downstream code expects 768D vectors but gets 512D.

**Fix Required**: Make wrapper dimension match underlying model:
```rust
pub async fn with_dimension(dimension: usize) -> Result<Self> {
    // Validate dimension (CLIP supports 512D for Base, 768D for Large)
    if dimension != 512 && dimension != 768 {
        return Err(MemoryError::Config(format!(
            "CLIP Vision supports 512D (Base) or 768D (Large). Requested: {}", dimension
        )));
    }
    
    let provider = ClipVisionModel::new();
    let actual_dim = provider.info().embedding_dimension.unwrap_or(512) as usize;
    
    if dimension != actual_dim {
        return Err(MemoryError::Config(format!(
            "Requested {}D but model produces {}D", dimension, actual_dim
        )));
    }
    
    Ok(Self { provider: Arc::new(provider), dimension: actual_dim })
}
```

### 3. ModelInfo Metadata Lies 🔥

**Location**: clip_vision_embedding.rs lines 197-213

**Problem**: Static ModelInfo claims Base model but has Large model dimensions:

```rust
static CLIP_VISION_EMBEDDING_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    name: "clip-vit-base-patch32",        // ← CLAIMS Base model
    embedding_dimension: Some(768),        // ← ACTUALLY Large (512 for Base)
    image_size: Some(336),                 // ← ACTUALLY Large (224 for Base)
    // ... BUT underlying model uses 512D and 224x224!
};
```

**Truth Table**:
| Model Variant | Image Size | Embedding Dim |
|--------------|------------|---------------|
| ViT-Base-Patch32 | 224×224 | 512D |
| ViT-Large-Patch14-336 | 336×336 | 768D |

**Current State**: Claims "base-patch32" but specifies Large dimensions.

**Actual Behavior**: Underlying ClipVisionModel uses Base (512D, 224×224).

**Result**: Metadata completely misrepresents actual model behavior.

**Fix Required**: Match metadata to actual implementation:
```rust
static CLIP_VISION_EMBEDDING_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    name: "clip-vit-base-patch32",         // ✓ Correct
    embedding_dimension: Some(512),        // ✓ Fix: Base uses 512D
    image_size: Some(224),                 // ✓ Fix: Base uses 224×224
    // ...
};
```

## MODERATE ISSUES

### 4. Architecture Inflexibility

**Problem**: System designed to support both Base (512D) and Large (768D) variants, but implementation hardcoded to Base only.

**Evidence**:
- `EmbeddingConfig.get_supported_dimensions()` claims: `vec![512, 768]`
- But `ClipVisionModel` hardcoded: `ClipVisionConfig::vit_base_patch32()`
- No way to instantiate Large model variant

**Fix Required**: Make model architecture configurable or clearly document as Base-only.

## VERIFICATION CHECKLIST

After fixes, verify:

1. **Default construction works**:
   ```rust
   let model = ClipVisionEmbeddingModel::default(); // Should not panic
   ```

2. **Dimension reporting matches actual embeddings**:
   ```rust
   let model = ClipVisionEmbeddingModel::with_dimension(512).await?;
   let embedding = model.embed_image("test.jpg")?;
   assert_eq!(embedding.len(), model.embedding_dimension()); // Must match!
   ```

3. **ModelInfo accuracy**:
   ```rust
   assert_eq!(model.info().embedding_dimension, Some(512)); // Match actual
   assert_eq!(model.info().image_size, Some(224));          // Match actual
   assert_eq!(model.info().name, "clip-vit-base-patch32"); // Match actual
   ```

4. **No compilation warnings**:
   ```bash
   cargo check -p paraphym_candle  # Should pass cleanly
   ```

## ROOT CAUSE

The wrapper was configured for ViT-Large (768D, 336×336) while the underlying model uses ViT-Base (512D, 224×224), creating a fundamental mismatch between wrapper metadata and actual model behavior.
