# DECOMP_030: Decompose `image.rs`

**File:** `packages/candle/src/builders/image.rs`  
**Current Size:** 746 lines  
**Module Area:** builders  
**Target:** Transform single file into focused module directory

## CORE OBJECTIVE

Decompose the monolithic 746-line `image.rs` file into 6 focused, maintainable modules (~25-280 lines each) while preserving ALL existing functionality, API surface, and zero-allocation patterns. The file implements a sophisticated image builder pattern with queued operations, generic type parameters for zero-allocation handlers, and integration with the Candle ML framework.

## CONSTRAINTS

- **MAINTAIN FUNCTIONALITY:** All existing functionality must be preserved exactly as-is
- **PRESERVE API:** The public API surface must remain completely unchanged
- **ZERO ALLOCATION:** Maintain the zero Box<dyn> trait pattern with generic type parameters
- **SINGLE SESSION:** This task must be completable in one focused Claude session
- **NO TESTS:** Do not write any unit tests, integration tests, or test code
- **NO BENCHMARKS:** Do not write any benchmark code
- **NO DOCUMENTATION:** Do not create new documentation files (code comments preserved)

## FILE ANALYSIS

### Current Structure (746 lines)

The file contains these major components:

1. **ImageBuilder trait** (~140 lines, lines 10-150)
   - Complete fluent API with 20+ methods
   - Methods: format(), media_type(), detail(), resize(), normalize_*(), clamp(), to_tensor(), on_error(), on_chunk(), load(), process()
   - Returns `impl ImageBuilder` for fluent chaining
   - Zero-allocation pattern (no Box<dyn>)

2. **ResizeFilter enum** (~20 lines, lines 173-191)
   - Four filter types: Triangle (CLIP), CatmullRom (LLaVA/SD), Nearest, Lanczos3
   - Used in public API

3. **ImageOperation enum** (~30 lines, lines 196-224)
   - Private enum defining queued operations
   - Variants: Resize, NormalizeSigned, NormalizeUnsigned, NormalizeWithParams, Clamp
   - Used internally by builder

4. **Helper functions** (~10 lines, lines 242-249)
   - `convert_filter()` - maps ResizeFilter to image::imageops::FilterType

5. **ImageBuilderImpl struct** (~10 lines, lines 252-261)
   - Generic over F1 (error handler) and F2 (chunk handler)
   - Fields: data, format, media_type, detail, error_handler, chunk_handler, operations: Vec<ImageOperation>
   - Zero-allocation design with generic type parameters

6. **Image static constructors** (~30 lines, lines 263-290)
   - Image::from_base64(), Image::from_url(), Image::from_path()
   - Entry points that create ImageBuilderImpl instances

7. **ImageBuilder trait implementation** (~280 lines, lines 292-580)
   - impl<F1, F2> ImageBuilder for ImageBuilderImpl<F1, F2>
   - All builder methods (format, resize, normalize, to_tensor, on_error, on_chunk, load, process)
   - Generic bounds preserved throughout

8. **Private processing methods** (~200 lines, lines 580-746)
   - impl<F1, F2> ImageBuilderImpl<F1, F2> { ... }
   - Methods: load_image_from_source(), apply_image_operations(), image_to_tensor(), apply_tensor_operations(), transfer_to_device()
   - Core image processing pipeline

### Key Patterns Identified

**Zero-Allocation Builder Pattern:**
```rust
// Generic type parameters eliminate Box<dyn> allocations
struct ImageBuilderImpl<F1 = fn(String), F2 = fn(ImageChunk) -> ImageChunk>
where
    F1: Fn(String) + Send + Sync + 'static,
    F2: FnMut(ImageChunk) -> ImageChunk + Send + 'static,
```

**Queued Operations Pattern:**
```rust
// Operations queued as enum variants, executed later
operations: Vec<ImageOperation>

// Example: resize queues operation, to_tensor executes them
builder.resize(224, 224, ResizeFilter::Triangle)  // queues
    .normalize_signed()                            // queues
    .to_tensor(&device)                           // executes all
```

**Candle ML Integration:**
- References: [tmp/candle-examples/candle-examples/examples/clip/main.rs](../tmp/candle-examples/candle-examples/examples/clip/main.rs)
- References: [tmp/candle-examples/candle-examples/examples/llava/image_processor.rs](../tmp/candle-examples/candle-examples/examples/llava/image_processor.rs)

**CLIP-style preprocessing** (lines 35-47 in clip/main.rs):
```rust
// Resize with Triangle filter
let img = img.resize_to_fill(width, height, FilterType::Triangle);
// Convert to RGB8
let img = img.to_rgb8();
// HWC → CHW permute
let img = Tensor::from_vec(img.into_raw(), (height, width, 3), &Device::Cpu)?
    .permute((2, 0, 1))?
    .to_dtype(DType::F32)?
    // Normalize to [-1, 1]
    .affine(2. / 255., -1.)?;
```

**LLaVA-style preprocessing** (lines 138-150 in llava/image_processor.rs):
```rust
// Step 1: Convert to tensor
let tensor = Tensor::from_vec(img, (height, width, 3), &Device::Cpu)?
    .to_dtype(DType::F32)?;
// Step 2: Rescale to [0, 1]
let tensor = tensor.affine(rescale_factor, 0.0)?;
// Step 3: Normalize with mean/std
let tensor = tensor.broadcast_sub(&mean)?.broadcast_div(&std)?;
// Step 4: HWC → CHW
tensor.permute((2, 0, 1))?
```

## DECOMPOSITION PLAN

Transform single file into module directory with 6 focused files:

### BEFORE
```
packages/candle/src/builders/
├── image.rs (746 lines)
└── mod.rs
```

### AFTER
```
packages/candle/src/builders/
├── image/
│   ├── mod.rs          (~25 lines)  - module aggregator with re-exports
│   ├── api.rs          (~140 lines) - ImageBuilder trait + ResizeFilter enum
│   ├── operations.rs   (~60 lines)  - ImageOperation enum + helpers
│   ├── constructors.rs (~40 lines)  - Image::from_* entry points
│   ├── builder_impl.rs (~280 lines) - ImageBuilderImpl + trait implementation
│   └── processing.rs   (~200 lines) - private processing methods
└── mod.rs (UPDATED to reference image/)
```

**Total:** 745 lines preserved across 6 modules (avg ~124 lines per module)

## DETAILED MODULE SPECIFICATIONS

### MODULE 1: `builders/image/mod.rs` (~25 lines)

**Purpose:** Module aggregator and public API re-exports

**Location:** `packages/candle/src/builders/image/mod.rs`

**Contents:**
```rust
//! Image builder implementations - Zero Box<dyn> trait-based architecture
//!
//! All image construction logic and builder patterns with zero allocation.
//!
//! # Module Organization
//! - `api`: Public ImageBuilder trait and ResizeFilter enum
//! - `operations`: Internal operation types and converters
//! - `constructors`: Image::from_* entry points
//! - `builder_impl`: ImageBuilderImpl struct and trait implementation
//! - `processing`: Image processing pipeline methods

mod api;
mod operations;
mod constructors;
mod builder_impl;
mod processing;

// Re-export public API
pub use api::{ImageBuilder, ResizeFilter};

// Note: Image::from_* constructors are automatically available via trait implementation
// Note: ImageBuilderImpl is private, no re-export needed
```

**Lines:** ~25 (module declarations + docs + re-exports)

**Dependencies:** None (top-level module)

**Changes to `builders/mod.rs`:**
- No change needed - `pub mod image;` already references either image.rs or image/mod.rs

---

### MODULE 2: `builders/image/api.rs` (~140 lines)

**Purpose:** Public trait definition and related types

**Location:** `packages/candle/src/builders/image/api.rs`

**Contents:**
- Complete ImageBuilder trait with all method signatures (~120 lines)
- ResizeFilter enum (~20 lines)
- All trait documentation, examples, and references to Candle examples

**Source Lines:** 10-150 from current image.rs

**Key Sections:**
```rust
use std::pin::Pin;
use crate::domain::context::CandleDocumentChunk as ImageChunk;
use candle_core::Tensor;
use tokio_stream::Stream;

/// Image builder trait - elegant zero-allocation builder pattern
pub trait ImageBuilder: Sized {
    // All 20+ method signatures
    fn format(self, format: ContentFormat) -> Self;
    fn resize(self, width: usize, height: usize, filter: ResizeFilter) -> Self;
    fn normalize_signed(self) -> Self;
    // ... etc
}

/// Image resize filter types matching image crate filters
#[derive(Debug, Clone, Copy)]
pub enum ResizeFilter {
    Triangle,    // CLIP models
    CatmullRom,  // SD/LLaVA
    Nearest,
    Lanczos3,
}
```

**Imports Needed:**
- `std::pin::Pin`
- `crate::domain::context::CandleDocumentChunk`
- `crate::domain::image::{ContentFormat, ImageDetail, ImageMediaType}`
- `candle_core::{Device, Tensor}`
- `tokio_stream::Stream`

**Visibility:** All items `pub`

---

### MODULE 3: `builders/image/operations.rs` (~60 lines)

**Purpose:** Internal operation types and conversion helpers

**Location:** `packages/candle/src/builders/image/operations.rs`

**Contents:**
- ImageOperation enum (~40 lines)
- convert_filter() helper function (~8 lines)
- Documentation (~12 lines)

**Source Lines:** 196-249 from current image.rs

**Key Sections:**
```rust
use super::api::ResizeFilter;

/// Image processing operations that can be queued
///
/// These operations are stored and executed in sequence during tensor conversion.
/// Private to builders module - not exposed in public API.
#[derive(Debug, Clone)]
pub(super) enum ImageOperation {
    Resize { width: usize, height: usize, filter: ResizeFilter },
    NormalizeSigned,
    NormalizeUnsigned,
    NormalizeWithParams { mean: [f32; 3], std: [f32; 3] },
    Clamp { min: f32, max: f32 },
}

/// Convert ResizeFilter to image crate FilterType
pub(super) fn convert_filter(filter: ResizeFilter) -> image::imageops::FilterType {
    match filter {
        ResizeFilter::Triangle => image::imageops::FilterType::Triangle,
        ResizeFilter::CatmullRom => image::imageops::FilterType::CatmullRom,
        ResizeFilter::Nearest => image::imageops::FilterType::Nearest,
        ResizeFilter::Lanczos3 => image::imageops::FilterType::Lanczos3,
    }
}
```

**Imports Needed:**
- `super::api::ResizeFilter`
- `image::imageops::FilterType` (for convert_filter)

**Visibility:** `pub(super)` (visible within builders/image/ only)

---

### MODULE 4: `builders/image/constructors.rs` (~40 lines)

**Purpose:** Image static constructor methods (entry points)

**Location:** `packages/candle/src/builders/image/constructors.rs`

**Contents:**
- Image::from_base64() (~12 lines)
- Image::from_url() (~12 lines)
- Image::from_path() (~12 lines)

**Source Lines:** 263-290 from current image.rs

**Key Sections:**
```rust
use crate::domain::image::{ContentFormat, Image};
use super::api::ImageBuilder;
use super::builder_impl::ImageBuilderImpl;

impl Image {
    /// Semantic entry point - EXACT syntax: Image::from_base64(data)
    pub fn from_base64(data: impl Into<String>) -> impl ImageBuilder {
        ImageBuilderImpl::<fn(String), fn(ImageChunk) -> ImageChunk> {
            data: data.into(),
            format: Some(ContentFormat::Base64),
            media_type: None,
            detail: None,
            error_handler: None,
            chunk_handler: None,
            operations: Vec::new(),
        }
    }

    // from_url() and from_path() similar
}
```

**Imports Needed:**
- `crate::domain::image::{ContentFormat, Image, ImageDetail, ImageMediaType}`
- `crate::domain::context::CandleDocumentChunk as ImageChunk`
- `super::api::ImageBuilder`
- `super::builder_impl::ImageBuilderImpl`
- `super::operations::ImageOperation`

**Visibility:** `impl Image` methods are `pub`

**Note:** Must create ImageBuilderImpl directly - cannot move struct definition here

---

### MODULE 5: `builders/image/builder_impl.rs` (~280 lines)

**Purpose:** ImageBuilderImpl struct and complete trait implementation

**Location:** `packages/candle/src/builders/image/builder_impl.rs`

**Contents:**
- ImageBuilderImpl struct definition (~10 lines)
- Complete impl<F1, F2> ImageBuilder for ImageBuilderImpl (~270 lines)
- All trait method implementations with full generic bounds

**Source Lines:** 252-261 (struct) + 292-580 (trait impl) from current image.rs

**Key Sections:**
```rust
use super::api::{ImageBuilder, ResizeFilter};
use super::operations::{ImageOperation, convert_filter};
use crate::domain::context::CandleDocumentChunk as ImageChunk;
use crate::domain::image::{ContentFormat, ImageDetail, ImageMediaType};
use std::pin::Pin;
use tokio_stream::Stream;
use candle_core::{DType, Device, Tensor};

/// Hidden implementation struct - zero-allocation builder state
pub(super) struct ImageBuilderImpl<F1 = fn(String), F2 = fn(ImageChunk) -> ImageChunk>
where
    F1: Fn(String) + Send + Sync + 'static,
    F2: FnMut(ImageChunk) -> ImageChunk + Send + 'static,
{
    pub(super) data: String,
    pub(super) format: Option<ContentFormat>,
    pub(super) media_type: Option<ImageMediaType>,
    pub(super) detail: Option<ImageDetail>,
    pub(super) error_handler: Option<F1>,
    pub(super) chunk_handler: Option<F2>,
    pub(super) operations: Vec<ImageOperation>,
}

impl<F1, F2> ImageBuilder for ImageBuilderImpl<F1, F2>
where
    F1: Fn(String) + Send + Sync + 'static,
    F2: FnMut(ImageChunk) -> ImageChunk + Send + 'static,
{
    fn format(mut self, format: ContentFormat) -> Self {
        self.format = Some(format);
        self
    }

    // ... all other trait methods (~25 methods total)
    
    fn to_tensor(self, device: &Device) 
        -> Pin<Box<dyn std::future::Future<Output = Result<Tensor, String>> + Send + '_>> 
    {
        let device = device.clone();
        Box::pin(async move {
            tokio::task::spawn_blocking(move || {
                self.to_tensor_sync(&device)
            })
            .await
            .map_err(|e| format!("Image processing spawn_blocking failed: {}", e))?
        })
    }

    // to_tensor_sync, on_error, on_chunk, load, process...
}
```

**Imports Needed:**
- `super::api::{ImageBuilder, ResizeFilter}`
- `super::operations::{ImageOperation, convert_filter}`
- `super::processing::*` (if processing methods called directly)
- `crate::domain::context::CandleDocumentChunk as ImageChunk`
- `crate::domain::image::{ContentFormat, ImageDetail, ImageMediaType}`
- `std::pin::Pin`
- `tokio_stream::{Stream, StreamExt}`
- `candle_core::{DType, Device, Tensor}`
- `image::{DynamicImage, ImageReader}`
- `base64::Engine`

**Visibility:** 
- Struct: `pub(super)` (visible in image/ module)
- Fields: `pub(super)` (accessed by constructors)
- Trait impl: `pub` via trait

**Critical:** Preserve ALL generic type parameters and bounds exactly

---

### MODULE 6: `builders/image/processing.rs` (~200 lines)

**Purpose:** Private image processing pipeline methods

**Location:** `packages/candle/src/builders/image/processing.rs`

**Contents:**
- Second impl block with processing methods
- impl<F1, F2> ImageBuilderImpl<F1, F2> { ... } (~200 lines)
- Five private methods: load_image_from_source, apply_image_operations, image_to_tensor, apply_tensor_operations, transfer_to_device

**Source Lines:** 580-746 from current image.rs

**Key Sections:**
```rust
use super::builder_impl::ImageBuilderImpl;
use super::operations::{ImageOperation, convert_filter};
use crate::domain::image::ContentFormat;
use candle_core::{DType, Device, Tensor};
use image::{DynamicImage, ImageReader};
use base64::Engine;

/// Private helper methods for image loading and processing
impl<F1, F2> ImageBuilderImpl<F1, F2>
where
    F1: Fn(String) + Send + Sync + 'static,
    F2: FnMut(ImageChunk) -> ImageChunk + Send + 'static,
{
    /// Load image from data based on format
    ///
    /// Supports three loading modes:
    /// - Base64: Decode base64 string → bytes → DynamicImage
    /// - Url: Load from HTTP URL
    /// - Raw: Load from file path
    ///
    /// Pattern reference: tmp/candle-examples/.../clip/main.rs:35
    pub(super) fn load_image_from_source(&self) -> Result<DynamicImage, String> {
        match &self.format {
            Some(ContentFormat::Url) | Some(ContentFormat::Raw) => {
                ImageReader::open(&self.data)
                    .map_err(|e| format!("Failed to open image: {}", e))?
                    .decode()
                    .map_err(|e| format!("Failed to decode image: {}", e))
            }
            Some(ContentFormat::Base64) => {
                let bytes = base64::engine::general_purpose::STANDARD
                    .decode(&self.data)
                    .map_err(|e| format!("Failed to decode base64: {}", e))?;
                image::load_from_memory(&bytes)
                    .map_err(|e| format!("Failed to load image from memory: {}", e))
            }
            None => Err("No format specified".to_string()),
        }
    }

    /// Apply image-level operations (resize, RGB conversion)
    ///
    /// Pattern reference: tmp/candle-examples/.../clip/main.rs:38-42
    pub(super) fn apply_image_operations(&self, mut img: DynamicImage) 
        -> Result<DynamicImage, String> 
    {
        for op in &self.operations {
            img = match op {
                ImageOperation::Resize { width, height, filter } => {
                    let filter_type = convert_filter(*filter);
                    img.resize_exact(*width as u32, *height as u32, filter_type)
                }
                _ => img,  // Skip tensor operations
            };
        }
        Ok(img)
    }

    /// Convert RGB8 image to Candle tensor in CHW format
    ///
    /// Pattern reference: tmp/candle-examples/.../clip/main.rs:44-47
    pub(super) fn image_to_tensor(&self, img: DynamicImage) -> Result<Tensor, String> {
        let img = img.to_rgb8();
        let (width, height) = img.dimensions();
        let data = img.into_raw();
        let tensor = Tensor::from_vec(data, (height as usize, width as usize, 3), &Device::Cpu)
            .map_err(|e| format!("Failed to create tensor: {}", e))?;
        let tensor = tensor.permute((2, 0, 1))
            .map_err(|e| format!("Failed to permute tensor: {}", e))?;
        let tensor = tensor.to_dtype(DType::F32)
            .map_err(|e| format!("Failed to convert dtype: {}", e))?;
        Ok(tensor)
    }

    /// Apply tensor-level operations (normalize, clamp)
    ///
    /// Pattern reference: tmp/candle-examples/.../llava/image_processor.rs:142-150
    pub(super) fn apply_tensor_operations(&self, mut tensor: Tensor) 
        -> Result<Tensor, String> 
    {
        for op in &self.operations {
            tensor = match op {
                ImageOperation::NormalizeSigned => {
                    // CLIP-style: [0,255] → [-1,1]
                    tensor.affine(2.0 / 255.0, -1.0)
                        .map_err(|e| format!("Signed normalization failed: {}", e))?
                }
                ImageOperation::NormalizeUnsigned => {
                    // Simple: [0,255] → [0,1]
                    tensor.affine(1.0 / 255.0, 0.0)
                        .map_err(|e| format!("Unsigned normalization failed: {}", e))?
                }
                ImageOperation::NormalizeWithParams { mean, std } => {
                    // LLaVA/ImageNet-style
                    let normalized = tensor.affine(1.0 / 255.0, 0.0)
                        .map_err(|e| format!("Pre-normalization failed: {}", e))?;
                    let mean_tensor = Tensor::from_vec(mean.to_vec(), (3,), &Device::Cpu)
                        .map_err(|e| format!("Failed to create mean tensor: {}", e))?;
                    let std_tensor = Tensor::from_vec(std.to_vec(), (3,), &Device::Cpu)
                        .map_err(|e| format!("Failed to create std tensor: {}", e))?;
                    let subtracted = normalized.broadcast_sub(&mean_tensor)
                        .map_err(|e| format!("Mean subtraction failed: {}", e))?;
                    subtracted.broadcast_div(&std_tensor)
                        .map_err(|e| format!("Std division failed: {}", e))?
                }
                ImageOperation::Clamp { min, max } => {
                    tensor.clamp(*min as f64, *max as f64)
                        .map_err(|e| format!("Clamp failed: {}", e))?
                }
                _ => tensor,  // Skip image operations
            };
        }
        Ok(tensor)
    }

    /// Transfer tensor to target device
    pub(super) fn transfer_to_device(&self, tensor: Tensor, device: &Device) 
        -> Result<Tensor, String> 
    {
        tensor.to_device(device)
            .map_err(|e| format!("Failed to transfer to device: {}", e))
    }
}
```

**Imports Needed:**
- `super::builder_impl::ImageBuilderImpl`
- `super::operations::{ImageOperation, convert_filter}`
- `crate::domain::image::ContentFormat`
- `candle_core::{DType, Device, Tensor}`
- `image::{DynamicImage, ImageReader}`
- `base64::Engine`

**Visibility:** 
- impl block: inherits from struct
- All methods: `pub(super)` (visible within image/ module only)

**Critical:** Preserve ALL generic bounds from ImageBuilderImpl

---

## IMPLEMENTATION STEPS

### STEP 1: Create Module Directory

```bash
# Create new directory structure
mkdir -p packages/candle/src/builders/image

# Verify structure
ls -la packages/candle/src/builders/image/
```

**Expected:** Empty directory ready for new files

---

### STEP 2: Create mod.rs (Module Aggregator)

**File:** `packages/candle/src/builders/image/mod.rs`

**Action:** Create new file with module declarations and re-exports (~25 lines)

**Verification:**
```bash
wc -l packages/candle/src/builders/image/mod.rs
# Expected: ~25 lines
```

---

### STEP 3: Create api.rs (Public API)

**File:** `packages/candle/src/builders/image/api.rs`

**Action:** Copy lines 10-150 from image.rs (ImageBuilder trait + ResizeFilter enum)

**Critical Sections:**
- Complete trait definition with all 20+ methods
- ResizeFilter enum with all variants
- All documentation and examples

**Verification:**
```bash
wc -l packages/candle/src/builders/image/api.rs
# Expected: ~140 lines

grep "pub trait ImageBuilder" packages/candle/src/builders/image/api.rs
# Expected: Found

grep "pub enum ResizeFilter" packages/candle/src/builders/image/api.rs
# Expected: Found
```

---

### STEP 4: Create operations.rs (Internal Types)

**File:** `packages/candle/src/builders/image/operations.rs`

**Action:** Copy lines 196-249 from image.rs (ImageOperation enum + convert_filter)

**Critical Sections:**
- ImageOperation enum with all 5 variants
- convert_filter() helper function
- Proper visibility: pub(super)

**Verification:**
```bash
wc -l packages/candle/src/builders/image/operations.rs
# Expected: ~60 lines

grep "pub(super) enum ImageOperation" packages/candle/src/builders/image/operations.rs
# Expected: Found

grep "pub(super) fn convert_filter" packages/candle/src/builders/image/operations.rs
# Expected: Found
```

---

### STEP 5: Create builder_impl.rs (Builder Implementation)

**File:** `packages/candle/src/builders/image/builder_impl.rs`

**Action:** Copy lines 252-580 from image.rs (ImageBuilderImpl struct + trait impl)

**Critical Sections:**
- ImageBuilderImpl struct with generic type parameters
- Complete impl<F1, F2> ImageBuilder for ImageBuilderImpl block
- All trait methods (~25 methods)
- Preserve ALL generic bounds exactly

**Verification:**
```bash
wc -l packages/candle/src/builders/image/builder_impl.rs
# Expected: ~280 lines

grep "pub(super) struct ImageBuilderImpl<F1" packages/candle/src/builders/image/builder_impl.rs
# Expected: Found

grep "impl<F1, F2> ImageBuilder for ImageBuilderImpl" packages/candle/src/builders/image/builder_impl.rs
# Expected: Found

grep "fn to_tensor(" packages/candle/src/builders/image/builder_impl.rs
# Expected: Found
```

---

### STEP 6: Create processing.rs (Processing Pipeline)

**File:** `packages/candle/src/builders/image/processing.rs`

**Action:** Copy lines 580-746 from image.rs (private processing methods)

**Critical Sections:**
- impl<F1, F2> ImageBuilderImpl<F1, F2> block
- All 5 processing methods: load_image_from_source, apply_image_operations, image_to_tensor, apply_tensor_operations, transfer_to_device
- All methods pub(super)

**Verification:**
```bash
wc -l packages/candle/src/builders/image/processing.rs
# Expected: ~200 lines

grep "impl<F1, F2> ImageBuilderImpl" packages/candle/src/builders/image/processing.rs
# Expected: Found

grep "pub(super) fn load_image_from_source" packages/candle/src/builders/image/processing.rs
# Expected: Found

grep "pub(super) fn apply_tensor_operations" packages/candle/src/builders/image/processing.rs
# Expected: Found
```

---

### STEP 7: Create constructors.rs (Entry Points)

**File:** `packages/candle/src/builders/image/constructors.rs`

**Action:** Copy lines 263-290 from image.rs (Image::from_* methods)

**Critical Sections:**
- impl Image block
- from_base64(), from_url(), from_path() constructors
- Direct instantiation of ImageBuilderImpl

**Verification:**
```bash
wc -l packages/candle/src/builders/image/constructors.rs
# Expected: ~40 lines

grep "impl Image" packages/candle/src/builders/image/constructors.rs
# Expected: Found

grep "pub fn from_base64" packages/candle/src/builders/image/constructors.rs
# Expected: Found
```

---

### STEP 8: Remove Original File

**Action:** Delete or rename the original image.rs

```bash
# Safe approach: rename first
mv packages/candle/src/builders/image.rs packages/candle/src/builders/image.rs.backup

# After verification, delete backup
# rm packages/candle/src/builders/image.rs.backup
```

**Note:** `builders/mod.rs` needs NO changes - `pub mod image;` works with both image.rs and image/mod.rs

---

### STEP 9: Verify Compilation

```bash
# Full workspace check
cargo check

# Specific package check
cargo check -p paraphym_candle

# Look for any image-related errors
cargo check 2>&1 | grep -i "image"
```

**Expected Output:** No errors

**Common Issues to Fix:**
1. **Missing imports** - Add use statements at top of each module
2. **Visibility errors** - Ensure pub(super) on ImageBuilderImpl struct and fields
3. **Generic bounds** - Preserve F1, F2 bounds exactly in all impl blocks
4. **Circular dependencies** - Verify import order (api → operations → builder_impl → processing)

---

### STEP 10: Verify Public API Unchanged

**Test Pattern:**
```rust
// This code should compile EXACTLY as before
use paraphym_candle::builders::image::{Image, ImageBuilder, ResizeFilter};
use candle_core::Device;

let device = Device::Cpu;
let tensor = Image::from_path("test.jpg")
    .resize(224, 224, ResizeFilter::Triangle)
    .normalize_signed()
    .to_tensor(&device)
    .await?;
```

**Verification:**
```bash
# Search for usage in codebase
rg "Image::from_" packages/candle/src/ --type rust
rg "ImageBuilder" packages/candle/src/ --type rust

# Check that examples still reference the API correctly
cargo build --examples
```

---

## IMPORT DEPENDENCY GRAPH

```
api.rs (no internal deps)
  ↓
operations.rs (imports api::ResizeFilter)
  ↓
builder_impl.rs (imports api::*, operations::*)
  ↓  
processing.rs (imports builder_impl::*, operations::*)
  ↓
constructors.rs (imports all above)
  ↓
mod.rs (imports all, re-exports api::*)
```

**Critical Rules:**
1. No circular imports
2. api.rs has no internal dependencies (only external crates)
3. All modules use `super::` for sibling imports
4. mod.rs only does `pub use api::{...}`

---

## VISIBILITY STRATEGY

| Item | Visibility | Rationale |
|------|-----------|-----------|
| ImageBuilder trait | `pub` | Public API |
| ResizeFilter enum | `pub` | Used in public trait methods |
| ImageOperation enum | `pub(super)` | Internal implementation detail |
| convert_filter fn | `pub(super)` | Helper for operations |
| ImageBuilderImpl struct | `pub(super)` | Constructors need access |
| ImageBuilderImpl fields | `pub(super)` | Constructors need direct access |
| Processing methods | `pub(super)` | Called from builder_impl |
| Image::from_* methods | `pub` | Public entry points |

---

## COMMON PITFALLS TO AVOID

### Pitfall 1: Generic Type Parameter Mismatch
**Issue:** Different generic bounds in impl blocks
**Solution:** Copy-paste the EXACT generic bounds with where clauses:
```rust
impl<F1, F2> ImageBuilderImpl<F1, F2>
where
    F1: Fn(String) + Send + Sync + 'static,
    F2: FnMut(ImageChunk) -> ImageChunk + Send + 'static,
```

### Pitfall 2: Incorrect Visibility
**Issue:** Making ImageBuilderImpl pub causes API exposure
**Solution:** Keep as `pub(super)` - only visible within image/ module

### Pitfall 3: Missing Re-exports
**Issue:** External code can't find ImageBuilder
**Solution:** Ensure mod.rs has `pub use api::{ImageBuilder, ResizeFilter};`

### Pitfall 4: Circular Import
**Issue:** constructors.rs imports builder_impl.rs which imports constructors.rs
**Solution:** Constructors are in separate module, only instantiate struct directly

### Pitfall 5: Forgetting to Update Imports
**Issue:** Old code imports from `crate::builders::image::ImageBuilder`
**Solution:** No change needed! Re-export in mod.rs maintains same path

---

## DEFINITION OF DONE

This task is COMPLETE when ALL of the following are true:

- [ ] Directory `packages/candle/src/builders/image/` exists with 6 .rs files
- [ ] `image/mod.rs` exists with module declarations and re-exports (~25 lines)
- [ ] `image/api.rs` contains ImageBuilder trait + ResizeFilter (~140 lines)
- [ ] `image/operations.rs` contains ImageOperation enum + helpers (~60 lines)
- [ ] `image/constructors.rs` contains Image::from_* methods (~40 lines)
- [ ] `image/builder_impl.rs` contains ImageBuilderImpl + trait impl (~280 lines)
- [ ] `image/processing.rs` contains 5 private processing methods (~200 lines)
- [ ] Original `packages/candle/src/builders/image.rs` is deleted or renamed
- [ ] `cargo check` passes without errors
- [ ] `cargo check -p paraphym_candle` passes without errors
- [ ] No compilation warnings related to unused imports or visibility
- [ ] Public API remains EXACTLY the same (no breaking changes)
- [ ] All 746 lines of functionality are preserved
- [ ] Generic type parameters preserved exactly
- [ ] Zero-allocation pattern maintained (no Box<dyn> added)
- [ ] All Candle example references preserved in comments

---

## REFERENCE LINKS

**Candle Examples (in ./tmp):**
- CLIP preprocessing: [tmp/candle-examples/candle-examples/examples/clip/main.rs:35-47](../tmp/candle-examples/candle-examples/examples/clip/main.rs)
- LLaVA preprocessing: [tmp/candle-examples/candle-examples/examples/llava/image_processor.rs:100-160](../tmp/candle-examples/candle-examples/examples/llava/image_processor.rs)

**Related Source Files:**
- Image types: [packages/candle/src/domain/image.rs](../packages/candle/src/domain/image.rs)
- Document chunks: [packages/candle/src/domain/context/mod.rs](../packages/candle/src/domain/context/mod.rs)
- Builders module: [packages/candle/src/builders/mod.rs](../packages/candle/src/builders/mod.rs)

**External Crates:**
- candle-core: `candle_core::{Device, DType, Tensor}`
- image: `image::{DynamicImage, ImageReader, imageops::FilterType}`
- tokio-stream: `tokio_stream::{Stream, StreamExt}`
- base64: `base64::Engine`

---

## SUCCESS VERIFICATION

After completing all steps, verify success with:

```bash
# 1. Check file structure
ls -la packages/candle/src/builders/image/
# Expected: 6 .rs files

# 2. Check line counts
wc -l packages/candle/src/builders/image/*.rs
# Expected: mod ~25, api ~140, operations ~60, constructors ~40, builder_impl ~280, processing ~200

# 3. Verify compilation
cargo check -p paraphym_candle
# Expected: Success with no errors

# 4. Verify public API
rg "pub trait ImageBuilder" packages/candle/src/builders/image/
# Expected: Found in api.rs

# 5. Verify re-exports
rg "pub use api::" packages/candle/src/builders/image/mod.rs
# Expected: Found

# 6. Check for any remaining references to old file
rg "builders::image::" packages/candle/src/ --type rust
# Expected: All paths still work (mod.rs handles directory vs file)
```

**Final Confirmation:** The public API `Image::from_*().resize().normalize_*().to_tensor()` compiles and works identically to before decomposition.
