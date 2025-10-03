# IMGGEN_1: Image Generation Domain Types

## OBJECTIVE
Create core domain types and traits for text-to-image generation in paraphym's type system.

## PRIORITY
🔴 CRITICAL - Foundation for image generation providers

## SUBTASK 1: Create Domain Module Structure

**What needs to change**:
- Create new domain module for image generation types
- Define configuration, chunk types, and provider trait

**Where changes happen**:
- NEW: `packages/candle/src/domain/image_generation/mod.rs`

**Why this is needed**:
- Image generation has different semantics than text completion (denoising steps vs tokens)
- Separate types prevent confusion with completion domain
- Provides clean interface for providers to implement

## SUBTASK 2: Implement ImageGenerationConfig

**Create struct**:
```rust
#[derive(Debug, Clone)]
pub struct ImageGenerationConfig {
    pub width: usize,              // Image width in pixels
    pub height: usize,             // Image height in pixels
    pub steps: usize,              // Denoising steps
    pub guidance_scale: f64,       // CFG scale
    pub negative_prompt: Option<String>,
    pub seed: Option<u64>,
    pub use_flash_attn: bool,
}
```

**With sensible defaults**:
- width/height: 1024 (native for SD3.5/FLUX)
- steps: 4 (SD3.5 Turbo default)
- guidance_scale: 3.5 (SD3.5 default)
- use_flash_attn: false (opt-in optimization)

## SUBTASK 3: Implement ImageGenerationChunk Enum

**Create enum for streaming**:
```rust
pub enum ImageGenerationChunk {
    /// Progress during denoising
    Step { 
        step: usize,      // Current step number
        total: usize,     // Total steps
        latent: Tensor    // Intermediate latent (optional for preview)
    },
    
    /// Final generated image
    Complete { 
        image: Tensor     // Final image tensor (CHW format, f32, [0-1] range)
    },
    
    /// Generation error
    Error(String),
}
```

**Why this design**:
- Step chunks allow progress monitoring during slow generation
- Complete chunk delivers final tensor result
- Error chunk for graceful error handling in streams

## SUBTASK 4: Implement ImageGenerationModel Trait

**Create provider trait**:
```rust
pub trait ImageGenerationModel: Send + Sync + 'static {
    /// Generate image from text prompt
    fn generate(
        &self,
        prompt: &str,
        config: &ImageGenerationConfig,
        device: &candle_core::Device,
    ) -> ystream::AsyncStream<ImageGenerationChunk>;
    
    /// Model identifier
    fn model_name(&self) -> &str;
    
    /// Recommended default steps
    fn default_steps(&self) -> usize;
}
```

**Why this interface**:
- Follows paraphym's async streaming pattern
- Device parameter allows GPU/CPU flexibility
- Model-specific defaults via default_steps()

## SUBTASK 5: Add Tensor to Image Helper

**Implement conversion utility**:
```rust
/// Convert image tensor (CHW, f32, [0-1]) to DynamicImage
pub fn tensor_to_image(tensor: &Tensor) -> Result<DynamicImage, String> {
    // 1. Validate 3D tensor (C, H, W)
    let (channels, height, width) = tensor.dims3()
        .map_err(|e| format!("Expected 3D CHW tensor: {}", e))?;
    
    if channels != 3 {
        return Err(format!("Expected RGB (3 channels), got {}", channels));
    }
    
    // 2. Permute CHW → HWC for image crate
    let hwc = tensor.permute((1, 2, 0))
        .map_err(|e| format!("Permute failed: {}", e))?;
    
    // 3. Flatten and extract f32 pixels
    let flat = hwc.flatten_all()
        .map_err(|e| format!("Flatten failed: {}", e))?;
    let pixels_f32 = flat.to_vec1::<f32>()
        .map_err(|e| format!("Tensor extraction failed: {}", e))?;
    
    // 4. Scale [0,1] → [0,255] and convert to u8
    let pixels_u8: Vec<u8> = pixels_f32
        .iter()
        .map(|&x| (x.clamp(0.0, 1.0) * 255.0) as u8)
        .collect();
    
    // 5. Create RGB image
    let rgb = image::RgbImage::from_raw(width as u32, height as u32, pixels_u8)
        .ok_or("Failed to create image from pixels")?;
    
    Ok(DynamicImage::ImageRgb8(rgb))
}
```

**Why this is needed**:
- Providers output tensors, users need images
- Handles CHW→HWC conversion (Candle format → image crate format)
- Proper scaling and clamping for valid image data

## REQUIRED IMPORTS

Add to top of `domain/image_generation/mod.rs`:
```rust
use candle_core::Tensor;
use image::DynamicImage;
use ystream::AsyncStream;
```

## DEFINITION OF DONE

- [ ] File created: `packages/candle/src/domain/image_generation/mod.rs`
- [ ] ImageGenerationConfig struct with Default impl
- [ ] ImageGenerationChunk enum with 3 variants (Step, Complete, Error)
- [ ] ImageGenerationModel trait with generate(), model_name(), default_steps()
- [ ] tensor_to_image() helper function implemented
- [ ] All types properly documented with /// comments
- [ ] No unwrap() or expect() - use Result and map_err()
- [ ] cargo check passes for paraphym_candle package

## REFERENCES

**Pattern References**:
- Completion domain types: `./packages/candle/src/domain/completion/` (structure reference only)
- Image preprocessing: `./packages/candle/src/builders/image.rs` (tensor conversion patterns)

**Why Not Use Existing**:
- `domain/completion/` is for text generation (different chunk semantics)
- `builders/image.rs` is for INPUT preprocessing (vision model inputs, not outputs)
- New domain needed for OUTPUT generation (text → image)

## IMPORTANT CONSTRAINTS

- ❌ NO unit tests (separate team handles testing)
- ❌ NO benchmarks (separate team handles performance)
- ❌ NO integration with Engine (that's a future task)
- ✅ YES proper error handling (Result types, map_err with context)
- ✅ YES clear documentation comments
- ✅ YES zero unwrap/expect in production code

## VERIFICATION

```bash
# Build check
cargo check -p paraphym_candle

# Verify no unwrap/expect
grep -n "unwrap\|expect" packages/candle/src/domain/image_generation/mod.rs
# Should return nothing
```
