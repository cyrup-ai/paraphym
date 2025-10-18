# Task 019: Complete Async Conversion for CLIP Vision

## Core Objective

Convert CLIP Vision image embedding model to be fully asynchronous by:
1. Creating a `LoadedClipVisionModel` following the **BERT pattern** (Arc without Mutex)
2. Wrapping all CPU-intensive operations in `spawn_blocking`
3. Updating `ImageEmbeddingCapable` trait implementation to use the loaded model pattern

**Pattern Reference:** Follow [`src/capability/text_embedding/bert.rs`](../src/capability/text_embedding/bert.rs) - BERT uses `Arc<BertModel>` without Mutex because `BertModel::forward()` takes `&self` (immutable reference). CLIP's `get_image_features()` likely follows the same pattern.

## Current State Analysis

### ClipVisionModel (clip_vision.rs)
**Current Issues:**
- Uses **lazy loading** - model loaded on every inference call
- Image preprocessing (resize, normalize) runs on async runtime (blocks)
- Model inference (`get_image_features()`) runs on async runtime (blocks)
- Tensor conversion (`to_vec1()`) runs on async runtime (blocks)
- No `LoadedClipVisionModel` pattern for repeated inference

**Current Structure:**
```rust
pub struct ClipVisionModel {
    dimension: usize,  // 512 for Base, 768 for Large
}

// Lazy loading on every call:
pub async fn encode_image(&self, image_path: &str) -> Result<Tensor, String> {
    let model_path = self.huggingface_file(...).await?;  // Download on demand
    let vb = VarBuilder::from_mmaped_safetensors(...)?;  // Load weights
    let model = ClipModel::new(vb, &config)?;            // Create model
    
    // CPU-intensive operations on async runtime:
    let image_tensor = Image::from_path(image_path)
        .resize(...)                  // ❌ CPU work
        .normalize_unsigned()         // ❌ CPU work
        .normalize_with(...)          // ❌ CPU work
        .to_tensor(&device).await?;   // ❌ CPU work
    
    model.get_image_features(&batched)  // ❌ CPU inference
}
```

### ClipVisionEmbeddingModel (clip_vision_embedding.rs)
**Current Issues:**
- Wrapper around `ClipVisionModel`
- Delegates to lazy loading pattern
- No optimization for repeated inference

## Implementation Plan

### Step 1: Create LoadedClipVisionModel

**Location:** `src/capability/image_embedding/clip_vision.rs`

**Pattern:** Follow BERT's Arc pattern (no Mutex needed)

```rust
/// Loaded CLIP Vision model for repeated inference with no I/O overhead
///
/// Pattern: Arc<ClipModel> (no Mutex) - ClipModel::get_image_features() takes &self
/// Reference: src/capability/text_embedding/bert.rs (LoadedBertModel)
#[derive(Clone)]
pub struct LoadedClipVisionModel {
    model: std::sync::Arc<ClipModel>,
    device: Device,
    config: ClipConfig,
    dimension: usize,
}

impl std::fmt::Debug for LoadedClipVisionModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoadedClipVisionModel")
            .field("device", &self.device)
            .field("dimension", &self.dimension)
            .field("model", &"Arc<ClipModel>")
            .finish()
    }
}

impl LoadedClipVisionModel {
    /// Load CLIP model once for repeated inference
    pub async fn load(
        dimension: usize,
    ) -> std::result::Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Validate dimension
        if dimension != 512 && dimension != 768 {
            return Err(format!("Unsupported dimension: {}", dimension).into());
        }
        
        // Get ModelInfo
        let base_model = ClipVisionModel::new(dimension)?;
        let model_info = base_model.info();
        
        // Auto-detect device
        let device = crate::core::device_util::detect_best_device()
            .unwrap_or_else(|e| {
                log::warn!("Device detection failed: {}. Using CPU.", e);
                Device::Cpu
            });
        
        // Download model file
        let model_path = base_model
            .huggingface_file(model_info.registry_key, "model.safetensors")
            .await?;
        
        // Build CLIP config
        let (text_config, vision_config, image_size) = 
            base_model.get_configs_for_dimension();
        let config = ClipConfig {
            text_config,
            vision_config,
            logit_scale_init_value: 2.6592,
            image_size: model_info.image_size.unwrap() as usize,
        };
        
        // Load model weights
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(
                &[model_path],
                candle_core::DType::F32,
                &device,
            )?
        };
        
        let model = ClipModel::new(vb, &config)?;
        
        Ok(Self {
            model: std::sync::Arc::new(model),
            device,
            config,
            dimension,
        })
    }
}
```

### Step 2: Implement ImageEmbeddingCapable with spawn_blocking

**Location:** `src/capability/image_embedding/clip_vision.rs`

**Pattern:** Wrap ALL CPU-intensive work in spawn_blocking

```rust
impl crate::capability::traits::ImageEmbeddingCapable for LoadedClipVisionModel {
    fn embed_image(
        &self,
        image_path: &str,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = std::result::Result<
                        Vec<f32>,
                        Box<dyn std::error::Error + Send + Sync>,
                    >,
                > + Send
                + '_,
        >,
    > {
        let image_path = image_path.to_string();
        // Clone for move into spawn_blocking
        let model = self.model.clone();
        let device = self.device.clone();
        let image_size = self.config.image_size;
        let image_mean = self.info().image_mean.unwrap();
        let image_std = self.info().image_std.unwrap();
        
        Box::pin(async move {
            // Wrap ALL CPU-intensive operations in spawn_blocking
            let embedding = tokio::task::spawn_blocking(move || {
                // Image preprocessing (CPU-intensive)
                let image_tensor = Image::from_path(&image_path)
                    .resize(image_size, image_size, ResizeFilter::Triangle)
                    .normalize_unsigned()
                    .normalize_with(image_mean, image_std)
                    .to_tensor_sync(&device)?;  // Synchronous tensor creation
                
                // Add batch dimension
                let batched = image_tensor
                    .unsqueeze(0)
                    .map_err(|e| format!("Failed to add batch dimension: {}", e))?;
                
                // Model inference (CPU-intensive)
                let features = model
                    .get_image_features(&batched)
                    .map_err(|e| format!("CLIP encoding failed: {}", e))?;
                
                // Tensor conversion (CPU-intensive)
                features
                    .to_vec1::<f32>()
                    .map_err(|e| format!("Failed to convert to vec: {}", e))
            })
            .await
            .map_err(|e| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Spawn blocking failed: {}", e),
                )) as Box<dyn std::error::Error + Send + Sync>
            })?
            .map_err(|e: String| {
                Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))
                    as Box<dyn std::error::Error + Send + Sync>
            })?;
            
            Ok(embedding)
        })
    }
    
    // Similar pattern for embed_image_url, embed_image_base64, batch_embed_images
}
```

### Step 3: Add Synchronous Tensor Creation Helper

**Location:** `src/builders/image.rs` or `src/domain/image.rs`

**New Method Required:**
```rust
impl Image {
    /// Synchronous tensor creation for use in spawn_blocking
    pub fn to_tensor_sync(&self, device: &Device) -> Result<Tensor, String> {
        // Existing async to_tensor logic but without .await
        // This runs in spawn_blocking context
    }
}
```

**Note:** If `to_tensor()` is already sync internally (just marked async), can call it directly in spawn_blocking.

### Step 4: Update ClipVisionEmbeddingModel

**Location:** `src/capability/image_embedding/clip_vision_embedding.rs`

**Changes:**
1. Add `loaded_model: Option<Arc<LoadedClipVisionModel>>` field
2. Provide `load()` method to pre-load model
3. Delegate to loaded model when available, fall back to lazy loading

```rust
#[derive(Clone)]
pub struct ClipVisionEmbeddingModel {
    provider: Arc<ClipVisionModel>,        // Lazy loading fallback
    loaded_model: Option<Arc<LoadedClipVisionModel>>,  // Pre-loaded for performance
    dimension: usize,
}

impl ClipVisionEmbeddingModel {
    /// Create with pre-loaded model for optimal performance
    pub async fn load(dimension: usize) -> Result<Self> {
        let loaded = LoadedClipVisionModel::load(dimension).await
            .map_err(|e| MemoryError::ModelError(format!("Failed to load CLIP model: {}", e)))?;
        
        let provider = ClipVisionModel::new(dimension)
            .map_err(MemoryError::Config)?;
        
        Ok(Self {
            provider: Arc::new(provider),
            loaded_model: Some(Arc::new(loaded)),
            dimension,
        })
    }
    
    /// Legacy constructor with lazy loading (for backward compatibility)
    pub async fn new() -> Result<Self> {
        Self::with_dimension(512).await
    }
    
    pub async fn with_dimension(dimension: usize) -> Result<Self> {
        // Lazy loading - no pre-load
        let provider = ClipVisionModel::new(dimension)
            .map_err(MemoryError::Config)?;
        
        Ok(Self {
            provider: Arc::new(provider),
            loaded_model: None,  // Will lazy-load on first use
            dimension,
        })
    }
}
```

## Files to Modify

### Primary Files
1. **`src/capability/image_embedding/clip_vision.rs`**
   - Add `LoadedClipVisionModel` struct
   - Implement `ImageEmbeddingCapable` with spawn_blocking
   - Make `get_configs_for_dimension()` public for LoadedClipVisionModel

2. **`src/capability/image_embedding/clip_vision_embedding.rs`**
   - Add `loaded_model` field
   - Add `load()` method
   - Update trait implementations to use loaded model when available

### Supporting Changes
3. **`src/builders/image.rs` or `src/domain/image.rs`**
   - Add `to_tensor_sync()` method for synchronous tensor creation in spawn_blocking

## CPU-Intensive Operations to Wrap

All operations inside `spawn_blocking`:

### Image Preprocessing
- `Image::from_path()` - File I/O + image decoding
- `.resize()` - Image resizing (CPU-intensive)
- `.normalize_unsigned()` - Pixel value normalization
- `.normalize_with()` - Mean/std normalization  
- `.to_tensor_sync()` - Array to tensor conversion

### Model Inference
- `model.get_image_features()` - Neural network forward pass

### Tensor Operations
- `.unsqueeze()` - Tensor shape manipulation
- `.to_vec1()` - Tensor to Vec conversion
- `.flatten_all()` - Tensor flattening

## Architecture Pattern

**Follow BERT, NOT NVEmbed/Stella:**

| Model | Pattern | Reason |
|-------|---------|--------|
| BERT | `Arc<BertModel>` | `forward(&self)` - immutable |
| CLIP | `Arc<ClipModel>` | `get_image_features(&self)` - immutable |
| NVEmbed | `Arc<Mutex<Model>>` | `forward(&mut self)` - mutable |
| Stella | `Arc<Mutex<Model>>` | `forward_norm(&mut self)` - mutable |

**CLIP uses BERT pattern** - no Mutex needed because inference doesn't mutate model state.

## Definition of Done

✅ **LoadedClipVisionModel created** following BERT's `Arc` pattern (no Mutex)
✅ **All CPU work in spawn_blocking**: image preprocessing, model inference, tensor ops
✅ **ImageEmbeddingCapable implemented** for LoadedClipVisionModel with spawn_blocking
✅ **ClipVisionEmbeddingModel updated** with optional pre-loaded model
✅ **Backward compatibility maintained** - lazy loading still works
✅ **Code compiles** with zero errors and warnings
✅ **No unwrap() or expect()** in implementation (production-grade error handling)

## Code References

- **BERT Pattern:** [`src/capability/text_embedding/bert.rs`](../src/capability/text_embedding/bert.rs) lines 458-462, 554-667
- **spawn_blocking Pattern:** [`src/capability/text_embedding/bert.rs`](../src/capability/text_embedding/bert.rs) lines 581-617
- **Current CLIP:** [`src/capability/image_embedding/clip_vision.rs`](../src/capability/image_embedding/clip_vision.rs)
- **ImageEmbeddingCapable Trait:** [`src/capability/traits.rs`](../src/capability/traits.rs)

## Dependencies

- Task 001 (async model.forward) - **N/A for CLIP** (ClipModel::get_image_features takes &self, not &mut)
- Requires `to_tensor_sync()` helper method in Image builder

## Estimated Effort

**3-4 hours** (simpler than NVEmbed/Stella because no Mutex needed)

## Notes

- CLIP Vision uses immutable inference like BERT (no Mutex needed)
- Lazy loading preserved for backward compatibility
- `LoadedClipVisionModel` provides zero-I/O repeated inference
- All CPU work moved to blocking thread pool via spawn_blocking
- Image preprocessing is the most CPU-intensive part (resize + normalize)