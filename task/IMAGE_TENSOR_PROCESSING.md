# Image Tensor Processing: Vision Model Pipeline Disconnection

## Status  
**DISCONNECTED** - Complete vision model preprocessing pipeline exists but never used

## Problem
Comprehensive image-to-tensor processing infrastructure built for CLIP, LLaVA, and Stable Diffusion models but never integrated. Zero-allocation builder pattern with full tensor pipeline (resize, normalize, device transfer) remains unused.

## Disconnected Components (25+ items)

### 1. Image Builder Entry Points (Never Called)
**File**: `packages/candle/src/builders/image.rs`
- `Image::from_base64()` never called (line 255)
- `Image::from_url()` never called (line 265)
- `Image::from_path()` never called (line 275)

### 2. Image Processing Methods (Never Called)
**File**: `packages/candle/src/builders/image.rs`
- `.format()` never called (line 303)
- `.media_type()` never called (line 309)
- `.detail()` never called (line 315)
- `.as_png()` never called (line 321)
- `.as_jpeg()` never called (line 327)
- `.high_detail()` never called (line 333)
- `.low_detail()` never called (line 339)

### 3. Tensor Conversion Pipeline (Never Executed)
**File**: `packages/candle/src/builders/image.rs`

**Resize operations** (lines 345-348):
- `.resize(width, height, filter)` never called
- `ResizeFilter::Triangle` - CLIP models (never used)
- `ResizeFilter::CatmullRom` - LLaVA/SD (never used)
- `ResizeFilter::Lanczos3` - High quality (never used)

**Normalization operations**:
- `.normalize_signed()` never called (line 365) - CLIP [-1, 1] range
- `.normalize_unsigned()` never called (line 375) - LLaVA [0, 1] range  
- `.normalize_with(mean, std)` never called (line 385) - ImageNet normalization
- `.clamp(min, max)` never called (line 399)

**Tensor conversion**:
- `.to_tensor(device)` never called (line 404-445) - Complete pipeline execution

### 4. Stream Processing (Never Used)
**File**: `packages/candle/src/builders/image.rs`
- `.on_error(handler)` never called (line 447)
- `.on_chunk(handler)` never called (line 458)
- `.load()` never called (line 469)
- `.process(f)` never called (line 494)

### 5. Internal Processing Pipeline (Unreachable)
**File**: `packages/candle/src/builders/image.rs`

**Image loading** (lines 570-592):
- `load_image_from_source()` - Base64/URL/path loading
- Supports ContentFormat::Base64, Url, Raw
- Never executed

**Image operations** (lines 608-643):
- `apply_image_operations()` - Resize, RGB conversion
- Uses image crate's resize_exact()
- Never executed

**Tensor conversion** (lines 657-684):
- `image_to_tensor()` - RGB8 → Tensor (HWC→CHW)
- Permute dimensions
- Convert u8 → f32
- Never executed

**Tensor operations** (lines 698-765):
- `apply_tensor_operations()` - Normalization, clamping
- CLIP affine: `(x * 2/255) - 1`
- LLaVA two-stage: `(x/255 - mean) / std`
- Never executed

**Device transfer** (lines 773-778):
- `transfer_to_device()` - CPU → GPU/Metal
- Never executed

## Architecture

### ImageOperation Queue (Never Populated)
```rust
enum ImageOperation {
    Resize { width, height, filter },           // CLIP: 224×224
    NormalizeSigned,                            // CLIP: [-1, 1]
    NormalizeUnsigned,                          // LLaVA step 1: [0, 1]
    NormalizeWithParams { mean, std },          // LLaVA step 2: (x-mean)/std
    Clamp { min, max },                         // SD VAE: [0, 1]
    ToRGB,                                      // Ensure 3 channels
    Permute { from, to },                       // HWC ↔ CHW
}
```
Operations are queued but queue is never processed.

### Processing Pipeline (Never Runs)
```
1. load_image_from_source()  → DynamicImage
2. apply_image_operations()   → Resized RGB8
3. image_to_tensor()          → Tensor (CHW, f32)
4. apply_tensor_operations()  → Normalized Tensor
5. transfer_to_device()       → GPU/Metal Tensor
```
Complete pipeline exists but entry points never called.

## Reference Implementations (Unused)

### CLIP Pattern (Never Used)
**Reference**: `tmp/candle-examples/examples/clip/main.rs:35-48`
```rust
// Pattern exists in image.rs but never called:
Image::from_path("image.jpg")
    .resize(224, 224, ResizeFilter::Triangle)    // Line 39-42
    .normalize_signed()                          // Line 48: affine(2/255, -1)
    .to_tensor(&device)
```

### LLaVA Pattern (Never Used)
**Reference**: `tmp/candle-examples/examples/llava/image_processor.rs:138-150`
```rust
// Pattern exists in image.rs but never called:
Image::from_url("image.png")
    .resize(336, 336, ResizeFilter::CatmullRom)  // Line 105-114
    .normalize_unsigned()                        // Line 141: / 255
    .normalize_with(                             // Line 146-150
        [0.485, 0.456, 0.406],                   // ImageNet mean
        [0.229, 0.224, 0.225]                    // ImageNet std
    )
    .to_tensor(&device)
```

### Stable Diffusion Pattern (Never Used)
```rust
// Pattern would work but never called:
Image::from_path("input.png")
    .resize(512, 512, ResizeFilter::CatmullRom)
    .normalize_unsigned()
    .clamp(0.0, 1.0)
    .to_tensor(&device)
```

## Reconnection Steps

### 1. Find Vision Model Integration Points
```bash
# Search for CLIP/LLaVA/SD usage
grep -r "CLIP\\|LLaVA\\|StableDiffusion" packages/candle/src/
grep -r "vision.*model\\|image.*model" packages/candle/src/
grep -r "VisionTransformer\\|ViT" packages/candle/src/
```

### 2. Integrate Image Preprocessing
**File**: Vision model module
```rust
use crate::builders::image::{Image, ResizeFilter};

async fn preprocess_image_for_clip(image_path: &str, device: &Device) -> Result<Tensor> {
    Image::from_path(image_path)
        .resize(224, 224, ResizeFilter::Triangle)
        .normalize_signed()
        .to_tensor(device)
        .await
}

async fn preprocess_image_for_llava(image_url: &str, device: &Device) -> Result<Tensor> {
    Image::from_url(image_url)
        .resize(336, 336, ResizeFilter::CatmullRom)
        .normalize_unsigned()
        .normalize_with(
            [0.485, 0.456, 0.406],  // ImageNet mean
            [0.229, 0.224, 0.225]   // ImageNet std
        )
        .to_tensor(device)
        .await
}
```

### 3. Add Base64 Support for API
**File**: Image processing module
```rust
// For API endpoints that receive base64 images
async fn process_base64_image(
    base64_data: &str,
    model_type: ModelType,
    device: &Device
) -> Result<Tensor> {
    let (width, height, filter, normalize) = match model_type {
        ModelType::CLIP => (224, 224, ResizeFilter::Triangle, NormalizeType::Signed),
        ModelType::LLaVA => (336, 336, ResizeFilter::CatmullRom, NormalizeType::ImageNet),
        ModelType::SD => (512, 512, ResizeFilter::CatmullRom, NormalizeType::Unsigned),
    };
    
    let mut builder = Image::from_base64(base64_data)
        .resize(width, height, filter);
    
    builder = match normalize {
        NormalizeType::Signed => builder.normalize_signed(),
        NormalizeType::Unsigned => builder.normalize_unsigned(),
        NormalizeType::ImageNet => builder
            .normalize_unsigned()
            .normalize_with([0.485, 0.456, 0.406], [0.229, 0.224, 0.225]),
    };
    
    builder.to_tensor(device).await
}
```

### 4. Add Error Handling
**File**: Image processing module
```rust
fn preprocess_with_error_handling(image_path: &str) -> impl ImageBuilder {
    Image::from_path(image_path)
        .on_error(|err| {
            error!("Image preprocessing failed: {}", err);
            // Log to monitoring system
        })
        .resize(224, 224, ResizeFilter::Triangle)
        .normalize_signed()
}
```

### 5. Add Streaming Support
**File**: Batch processing module
```rust
async fn process_image_batch(paths: Vec<String>, device: &Device) {
    for path in paths {
        let stream = Image::from_path(&path)
            .resize(224, 224, ResizeFilter::Triangle)
            .on_chunk(|chunk| {
                // Track progress
                println!("Processing chunk: {:?}", chunk.dimensions);
                chunk
            })
            .load();
        
        // Stream chunks for incremental processing
        while let Some(chunk) = stream.next().await {
            // Process chunk
        }
    }
}
```

### 6. Create Vision Model Factory
**File**: New `domain/vision/mod.rs`
```rust
pub struct VisionPreprocessor {
    model_type: ModelType,
    device: Device,
}

impl VisionPreprocessor {
    pub async fn preprocess(&self, image_source: ImageSource) -> Result<Tensor> {
        let builder = match image_source {
            ImageSource::Path(p) => Image::from_path(p),
            ImageSource::Url(u) => Image::from_url(u),
            ImageSource::Base64(b) => Image::from_base64(b),
        };
        
        let (w, h, filter) = self.get_dimensions();
        
        builder
            .resize(w, h, filter)
            .apply_normalization(self.model_type)
            .to_tensor(&self.device)
            .await
    }
}
```

## Investigation Required

### Find Vision Model Usage
```bash
# Check if vision models are implemented
grep -r "impl.*Model" packages/candle/src/ | grep -i "vision\\|image\\|clip\\|llava"
grep -r "DynamicImage\\|Tensor" packages/candle/src/domain/
```

### Check for Alternative Image Processing
```bash
# See if there's duplicate image processing
grep -r "resize.*image\\|normalize.*tensor" packages/candle/src/
grep -r "load_from_memory\\|ImageReader" packages/candle/src/
```

## Files to Modify
- `domain/vision/` - Create vision model integration (NEW)
- Vision model implementations - Use Image builder for preprocessing
- API handlers - Use Image::from_base64() for incoming images
- Batch processors - Use streaming .load() and .on_chunk()

## Testing After Reconnection
1. ✅ CLIP preprocessing: 224×224, Triangle filter, [-1, 1] normalization
2. ✅ LLaVA preprocessing: 336×336, CatmullRom, ImageNet normalization
3. ✅ SD preprocessing: 512×512, CatmullRom, [0, 1] clamp
4. ✅ Base64 images decoded and processed correctly
5. ✅ URL images downloaded and preprocessed
6. ✅ Error handlers catch invalid images
7. ✅ Streaming processes large batches incrementally
8. ✅ Device transfer works (CPU → GPU/Metal)

## Benefits Unlocked
1. **Unified preprocessing** - Single builder for all vision models
2. **Zero allocation** - Generic functions instead of Box<dyn>
3. **Model compatibility** - CLIP, LLaVA, SD preprocessing patterns
4. **Flexible inputs** - Base64, URL, file path support
5. **Async processing** - Non-blocking tensor conversion
6. **Error handling** - Callbacks for preprocessing failures
7. **Batch streaming** - Incremental processing of large datasets
8. **Device agnostic** - Automatic transfer to GPU/Metal

## Model-Specific Dimensions

### CLIP (224×224 or 336×336)
- Filter: Triangle
- Normalization: [-1, 1] via affine(2/255, -1)
- Format: CHW (3, 224, 224) or (3, 336, 336)

### LLaVA (336×336)
- Filter: CatmullRom  
- Normalization: Two-stage
  1. [0, 1] via / 255
  2. (x - mean) / std with ImageNet values
- Format: CHW (3, 336, 336)

### Stable Diffusion (512×512, 768×768, 1024×1024)
- Filter: CatmullRom
- Normalization: [0, 1] via / 255
- Clamp: [0, 1]
- Format: CHW (3, H, W)
