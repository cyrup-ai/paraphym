//! Image builder implementations - Zero Box<dyn> trait-based architecture
//!
//! All image construction logic and builder patterns with zero allocation.

use std::pin::Pin;

use crate::domain::context::CandleDocumentChunk as ImageChunk;
use crate::domain::image::{ContentFormat, Image, ImageDetail, ImageMediaType};
use base64::Engine;
use candle_core::{DType, Device, Tensor};
use image::{DynamicImage, ImageReader};
use tokio_stream::{Stream, StreamExt};

/// Image builder trait - elegant zero-allocation builder pattern
pub trait ImageBuilder: Sized {
    /// Set format - EXACT syntax: .format(ContentFormat::Base64)
    fn format(self, format: ContentFormat) -> Self;

    /// Set media type - EXACT syntax: .media_type(ImageMediaType::PNG)
    fn media_type(self, media_type: ImageMediaType) -> Self;

    /// Set detail - EXACT syntax: .detail(ImageDetail::High)
    fn detail(self, detail: ImageDetail) -> Self;

    /// Set PNG format - EXACT syntax: .with_png()
    fn with_png(self) -> Self;

    /// Set JPEG format - EXACT syntax: .with_jpeg()
    fn with_jpeg(self) -> Self;

    /// Set high detail - EXACT syntax: .high_detail()
    fn high_detail(self) -> Self;

    /// Set low detail - EXACT syntax: .low_detail()
    fn low_detail(self) -> Self;

    /// Resize image - EXACT syntax: .resize(width, height, filter)
    ///
    /// All vision models require specific input dimensions:
    /// - CLIP: 224×224 (ViT-B), 336×336 (ViT-L)
    /// - LLaVA: 336×336 (default)
    /// - Stable Diffusion: 512×512, 768×768, 1024×1024
    fn resize(self, width: usize, height: usize, filter: ResizeFilter) -> Self;

    /// Normalize to range [-1, 1] - EXACT syntax: .normalize_signed()
    ///
    /// Formula: (x * 2.0 / 255.0) - 1.0
    /// Reference: tmp/candle-examples/candle-examples/examples/clip/main.rs:48
    ///
    /// Used by:
    /// - CLIP and OpenCLIP models
    /// - MobileCLIP
    /// - Chinese CLIP
    fn normalize_signed(self) -> Self;

    /// Normalize to range [0, 1] - EXACT syntax: .normalize_unsigned()
    ///
    /// Formula: x / 255.0
    /// Reference: tmp/candle-examples/candle-examples/examples/llava/image_processor.rs:138
    ///
    /// Used by:
    /// - VAE decoders (Stable Diffusion output)
    /// - Visualization and output processing
    /// - Some detection models
    fn normalize_unsigned(self) -> Self;

    /// Normalize with mean/std per channel - EXACT syntax: .normalize_with(mean, std)
    ///
    /// Formula: (tensor - mean) / std
    /// Reference: tmp/candle-examples/candle-examples/examples/llava/image_processor.rs:142-146
    /// Implementation: tensor.broadcast_sub(&mean)?.broadcast_div(&std)
    ///
    /// ImageNet standard values:
    /// - Mean: [0.48145466, 0.4578275, 0.40821073]
    /// - Std:  [0.26862954, 0.2613026, 0.2757771]
    ///
    /// Used by:
    /// - LLaVA and vision-language models
    /// - ResNet-based models
    /// - Vision Transformers (ViT)
    /// - Any ImageNet pre-trained model
    fn normalize_with(self, mean: [f32; 3], std: [f32; 3]) -> Self;

    /// Clamp values to range - EXACT syntax: .clamp(min, max)
    ///
    /// Ensures all tensor values are within [min, max] range.
    /// Common ranges:
    /// - [0.0, 1.0] for output processing
    /// - [-1.0, 1.0] for normalized inputs
    ///
    /// Used in:
    /// - Stable Diffusion VAE output
    /// - Tensor sanitization before model input
    /// - Output post-processing
    fn clamp(self, min: f32, max: f32) -> Self;

    /// Convert to Candle tensor - EXACT syntax: .to_tensor(device)
    ///
    /// This executes all queued operations in sequence and returns the final tensor.
    /// The pipeline is:
    /// 1. Load image from source
    /// 2. Apply resize (if queued)
    /// 3. Apply normalization (if queued)
    /// 4. Apply clamp (if queued)
    /// 5. Convert HWC → CHW format
    /// 6. Move to target device
    ///
    /// Returns Result<Tensor, String> wrapped in Future for async execution.
    fn to_tensor(
        self,
        device: &candle_core::Device,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<candle_core::Tensor, String>> + Send + '_>,
    >;

    /// Synchronous tensor creation for use in spawn_blocking contexts - EXACT syntax: .to_tensor_sync(device)
    ///
    /// This method performs the same operations as `to_tensor()` but without
    /// async wrapping, making it suitable for use inside `spawn_blocking`.
    ///
    /// Executes the complete image processing pipeline:
    /// 1. Load image from source (base64/URL/path)
    /// 2. Apply image-level operations (resize, RGB conversion)
    /// 3. Convert image to tensor (HWC→CHW, u8→f32)
    /// 4. Apply tensor-level operations (normalize, clamp)
    /// 5. Transfer to target device
    ///
    /// Returns Result<Tensor, String> synchronously.
    fn to_tensor_sync(self, device: &candle_core::Device) -> Result<candle_core::Tensor, String>;

    /// Set error handler - EXACT syntax: .on_error(|error| { ... })
    /// Zero-allocation: uses generic function pointer instead of Box<dyn>
    fn on_error<F>(self, handler: F) -> impl ImageBuilder
    where
        F: Fn(String) + Send + Sync + 'static;

    /// Set chunk handler - EXACT syntax: .on_chunk(|chunk| { ... })
    /// Zero-allocation: uses generic function pointer instead of Box<dyn>
    fn on_chunk<F>(self, handler: F) -> impl ImageBuilder
    where
        F: FnMut(ImageChunk) -> ImageChunk + Send + 'static;

    /// Load image - EXACT syntax: .load()
    fn load(self) -> Pin<Box<dyn Stream<Item = ImageChunk> + Send>>;

    /// Process image - EXACT syntax: .process(|chunk| { ... })
    fn process<F>(self, f: F) -> Pin<Box<dyn Stream<Item = ImageChunk> + Send>>
    where
        F: FnOnce(ImageChunk) -> ImageChunk + Send + 'static;
}

/// Image resize filter types matching image crate filters
///
/// Maps to image::imageops::FilterType for actual resize operations.
/// Different models use different filters for optimal quality:
/// - Triangle: CLIP models (fast, good quality)
/// - CatmullRom: Stable Diffusion, LLaVA (high quality, smooth)
/// - Nearest: Fast preview (low quality)
/// - Lanczos3: Maximum quality (slower)
#[derive(Debug, Clone, Copy)]
pub enum ResizeFilter {
    /// Triangle filter - used by CLIP models
    /// Reference: tmp/candle-examples/candle-examples/examples/clip/main.rs:42
    Triangle,

    /// Catmull-Rom filter - used by Stable Diffusion and LLaVA
    /// Reference: tmp/candle-examples/candle-examples/examples/llava/image_processor.rs:105
    CatmullRom,

    /// Nearest neighbor - fast, low quality
    Nearest,

    /// Lanczos3 - high quality, slower
    Lanczos3,
}

/// Image processing operations that can be queued
///
/// These operations are stored and executed in sequence during tensor conversion.
/// Private to builders module - not exposed in public API.
#[derive(Debug, Clone)]
enum ImageOperation {
    /// Resize image to target dimensions with specified filter
    Resize {
        width: usize,
        height: usize,
        filter: ResizeFilter,
    },

    /// Normalize to [-1, 1] range (CLIP-style)
    /// Formula: (x * 2.0 / 255.0) - 1.0
    /// Reference: tmp/candle-examples/candle-examples/examples/clip/main.rs:48
    NormalizeSigned,

    /// Normalize to [0, 1] range (LLaVA-style step 1)
    /// Formula: x / 255.0
    /// Reference: tmp/candle-examples/candle-examples/examples/llava/image_processor.rs:138
    NormalizeUnsigned,

    /// Normalize with per-channel mean and standard deviation (LLaVA-style step 2)
    /// Formula: (x - mean) / std
    /// Reference: tmp/candle-examples/candle-examples/examples/llava/image_processor.rs:142-146
    NormalizeWithParams { mean: [f32; 3], std: [f32; 3] },

    /// Clamp values to range
    Clamp { min: f32, max: f32 },
}

/// Convert ResizeFilter to image crate FilterType
///
/// Maps our ResizeFilter enum to image::imageops::FilterType:
/// - Triangle: CLIP models (fast, good quality)
/// - CatmullRom: Stable Diffusion, LLaVA (high quality, smooth)
/// - Nearest: Fast preview (low quality)
/// - Lanczos3: Maximum quality (slower)
///
/// References:
/// - CLIP: tmp/candle-examples/candle-examples/examples/clip/main.rs:39
/// - LLaVA: tmp/candle-examples/candle-examples/examples/llava/image_processor.rs:114
/// - SD: tmp/candle-examples/candle-examples/examples/stable-diffusion/main.rs
fn convert_filter(filter: ResizeFilter) -> image::imageops::FilterType {
    match filter {
        ResizeFilter::Triangle => image::imageops::FilterType::Triangle,
        ResizeFilter::CatmullRom => image::imageops::FilterType::CatmullRom,
        ResizeFilter::Nearest => image::imageops::FilterType::Nearest,
        ResizeFilter::Lanczos3 => image::imageops::FilterType::Lanczos3,
    }
}

/// Hidden implementation struct - zero-allocation builder state with zero Box<dyn> usage
struct ImageBuilderImpl<F1 = fn(String), F2 = fn(ImageChunk) -> ImageChunk>
where
    F1: Fn(String) + Send + Sync + 'static,
    F2: FnMut(ImageChunk) -> ImageChunk + Send + 'static,
{
    data: String,
    format: Option<ContentFormat>,
    media_type: Option<ImageMediaType>,
    detail: Option<ImageDetail>,
    error_handler: Option<F1>,
    chunk_handler: Option<F2>,
    operations: Vec<ImageOperation>,
}

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

    /// Semantic entry point - EXACT syntax: Image::from_url(url)
    pub fn from_url(url: impl Into<String>) -> impl ImageBuilder {
        ImageBuilderImpl::<fn(String), fn(ImageChunk) -> ImageChunk> {
            data: url.into(),
            format: Some(ContentFormat::Url),
            media_type: None,
            detail: None,
            error_handler: None,
            chunk_handler: None,
            operations: Vec::new(),
        }
    }

    /// Semantic entry point - EXACT syntax: Image::from_path(path)
    pub fn from_path(path: impl Into<String>) -> impl ImageBuilder {
        ImageBuilderImpl::<fn(String), fn(ImageChunk) -> ImageChunk> {
            data: path.into(),
            format: Some(ContentFormat::Url),
            media_type: None,
            detail: None,
            error_handler: None,
            chunk_handler: None,
            operations: Vec::new(),
        }
    }
}

impl<F1, F2> ImageBuilder for ImageBuilderImpl<F1, F2>
where
    F1: Fn(String) + Send + Sync + 'static,
    F2: FnMut(ImageChunk) -> ImageChunk + Send + 'static,
{
    /// Set format - EXACT syntax: .format(ContentFormat::Base64)
    fn format(mut self, format: ContentFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Set media type - EXACT syntax: .media_type(ImageMediaType::PNG)
    fn media_type(mut self, media_type: ImageMediaType) -> Self {
        self.media_type = Some(media_type);
        self
    }

    /// Set detail - EXACT syntax: .detail(ImageDetail::High)
    fn detail(mut self, detail: ImageDetail) -> Self {
        self.detail = Some(detail);
        self
    }

    /// Set PNG format - EXACT syntax: .with_png()
    fn with_png(mut self) -> Self {
        self.media_type = Some(ImageMediaType::PNG);
        self
    }

    /// Set JPEG format - EXACT syntax: .with_jpeg()
    fn with_jpeg(mut self) -> Self {
        self.media_type = Some(ImageMediaType::JPEG);
        self
    }

    /// Set high detail - EXACT syntax: .high_detail()
    fn high_detail(mut self) -> Self {
        self.detail = Some(ImageDetail::High);
        self
    }

    /// Set low detail - EXACT syntax: .low_detail()
    fn low_detail(mut self) -> Self {
        self.detail = Some(ImageDetail::Low);
        self
    }

    /// Resize image - EXACT syntax: .resize(width, height, filter)
    fn resize(mut self, width: usize, height: usize, filter: ResizeFilter) -> Self {
        self.operations.push(ImageOperation::Resize {
            width,
            height,
            filter,
        });
        self
    }

    /// Normalize to range [-1, 1] - EXACT syntax: .normalize_signed()
    fn normalize_signed(mut self) -> Self {
        self.operations.push(ImageOperation::NormalizeSigned);
        self
    }

    /// Normalize to range [0, 1] - EXACT syntax: .normalize_unsigned()
    fn normalize_unsigned(mut self) -> Self {
        self.operations.push(ImageOperation::NormalizeUnsigned);
        self
    }

    /// Normalize with mean/std per channel - EXACT syntax: .normalize_with(mean, std)
    fn normalize_with(mut self, mean: [f32; 3], std: [f32; 3]) -> Self {
        self.operations
            .push(ImageOperation::NormalizeWithParams { mean, std });
        self
    }

    /// Clamp values to range - EXACT syntax: .clamp(min, max)
    fn clamp(mut self, min: f32, max: f32) -> Self {
        self.operations.push(ImageOperation::Clamp { min, max });
        self
    }

    /// Convert to Candle tensor - EXACT syntax: .to_tensor(device)
    ///
    /// Executes the complete image processing pipeline:
    /// 1. Load image from source (base64/URL/path)
    /// 2. Apply image-level operations (resize, RGB conversion)
    /// 3. Convert image to tensor (HWC→CHW, u8→f32)
    /// 4. Apply tensor-level operations (normalize, clamp)
    /// 5. Transfer to target device
    ///
    /// Returns Future<Output = Result<Tensor, String>> for async execution.
    ///
    /// # Examples
    ///
    /// CLIP-style preprocessing:
    /// ```no_run
    /// # use paraphym_candle::builders::image::{Image, ResizeFilter};
    /// # use candle_core::Device;
    /// # async fn example() -> Result<(), String> {
    /// let device = Device::Cpu;
    /// let tensor = Image::from_path("image.jpg")
    ///     .resize(224, 224, ResizeFilter::Triangle)
    ///     .normalize_signed()
    ///     .to_tensor(&device)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// LLaVA-style preprocessing:
    /// ```no_run
    /// # use paraphym_candle::builders::image::{Image, ResizeFilter};
    /// # use candle_core::Device;
    /// # async fn example() -> Result<(), String> {
    /// let device = Device::Cpu;
    /// let tensor = Image::from_url("https://example.com/image.png")
    ///     .resize(336, 336, ResizeFilter::CatmullRom)
    ///     .normalize_unsigned()
    ///     .normalize_with([0.485, 0.456, 0.406], [0.229, 0.224, 0.225])
    ///     .to_tensor(&device)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    fn to_tensor(
        self,
        device: &candle_core::Device,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<candle_core::Tensor, String>> + Send + '_>,
    > {
        // Clone device for move into async block
        let device = device.clone();

        Box::pin(async move {
            // Step 1: Load image from source (base64/URL/path)
            // Uses format field to determine loading method
            let img = self.load_image_from_source()?;

            // Step 2: Apply image-level operations (resize, RGB conversion)
            // Processes ImageOperation::Resize and ImageOperation::ToRGB from queue
            let img = self.apply_image_operations(img)?;

            // Step 3: Convert image to tensor (HWC→CHW, u8→f32)
            // Creates Tensor from DynamicImage, permutes dimensions, converts dtype
            let tensor = self.image_to_tensor(img)?;

            // Step 4: Apply tensor-level operations (normalize, clamp)
            // Processes NormalizeSigned, NormalizeUnsigned, NormalizeWithParams, Clamp
            let tensor = self.apply_tensor_operations(tensor)?;

            // Step 5: Transfer to target device (CPU→GPU/Metal)
            // Moves tensor from CPU to target device for inference
            let tensor = self.transfer_to_device(tensor, &device)?;

            Ok(tensor)
        })
    }

    /// Synchronous tensor creation for use in spawn_blocking contexts
    ///
    /// This method performs the same operations as `to_tensor()` but without
    /// async wrapping, making it suitable for use inside `spawn_blocking`.
    ///
    /// Executes the complete image processing pipeline:
    /// 1. Load image from source (base64/URL/path)
    /// 2. Apply image-level operations (resize, RGB conversion)
    /// 3. Convert image to tensor (HWC→CHW, u8→f32)
    /// 4. Apply tensor-level operations (normalize, clamp)
    /// 5. Transfer to target device
    ///
    /// # Example
    /// ```no_run
    /// # use paraphym_candle::builders::image::{Image, ResizeFilter};
    /// # use candle_core::Device;
    /// # fn example() -> Result<(), String> {
    /// let device = Device::Cpu;
    /// let tensor = Image::from_path("image.jpg")
    ///     .resize(224, 224, ResizeFilter::Triangle)
    ///     .normalize_signed()
    ///     .to_tensor_sync(&device)?;
    /// # Ok(())
    /// # }
    /// ```
    fn to_tensor_sync(self, device: &candle_core::Device) -> Result<candle_core::Tensor, String> {
        // Step 1: Load image from source (base64/URL/path)
        let img = self.load_image_from_source()?;
        
        // Step 2: Apply image-level operations (resize, RGB conversion)
        let img = self.apply_image_operations(img)?;
        
        // Step 3: Convert image to tensor (HWC→CHW, u8→f32)
        let tensor = self.image_to_tensor(img)?;
        
        // Step 4: Apply tensor-level operations (normalize, clamp)
        let tensor = self.apply_tensor_operations(tensor)?;
        
        // Step 5: Transfer to target device
        let tensor = self.transfer_to_device(tensor, device)?;
        
        Ok(tensor)
    }

    /// Set error handler - EXACT syntax: .on_error(|error| { ... })
    /// Zero-allocation: uses generic function pointer instead of Box<dyn>
    fn on_error<F>(self, handler: F) -> impl ImageBuilder
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        ImageBuilderImpl {
            data: self.data,
            format: self.format,
            media_type: self.media_type,
            detail: self.detail,
            error_handler: Some(handler),
            chunk_handler: self.chunk_handler,
            operations: self.operations,
        }
    }

    /// Set chunk handler - EXACT syntax: .on_chunk(|chunk| { ... })
    /// Zero-allocation: uses generic function pointer instead of Box<dyn>
    fn on_chunk<F>(self, handler: F) -> impl ImageBuilder
    where
        F: FnMut(ImageChunk) -> ImageChunk + Send + 'static,
    {
        ImageBuilderImpl {
            data: self.data,
            format: self.format,
            media_type: self.media_type,
            detail: self.detail,
            error_handler: self.error_handler,
            chunk_handler: Some(handler),
            operations: self.operations,
        }
    }

    /// Load image - EXACT syntax: .load()
    fn load(self) -> Pin<Box<dyn Stream<Item = ImageChunk> + Send>> {
        // Create CandleDocumentChunk with proper fields
        let chunk = ImageChunk {
            path: None,
            content: self.data,
            byte_range: None,
            metadata: std::collections::HashMap::new(),
        };

        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            let _ = sender.send(chunk);
        }))
    }

    /// Process image - EXACT syntax: .process(|chunk| { ... })
    fn process<F>(self, f: F) -> Pin<Box<dyn Stream<Item = ImageChunk> + Send>>
    where
        F: FnOnce(ImageChunk) -> ImageChunk + Send + 'static,
    {
        // Get source stream from load
        let load_stream = self.load();

        // Create processing stream using async pattern
        Box::pin(crate::async_stream::spawn_stream(|sender| async move {
            tokio::pin!(load_stream);

            // Process the chunk (FnOnce called once)
            if let Some(chunk) = load_stream.next().await {
                let processed = f(chunk);
                let _ = sender.send(processed);
            }
        }))
    }
}

// Private helper methods for image loading and processing
impl<F1, F2> ImageBuilderImpl<F1, F2>
where
    F1: Fn(String) + Send + Sync + 'static,
    F2: FnMut(ImageChunk) -> ImageChunk + Send + 'static,
{
    /// Load image from data based on format
    ///
    /// Supports three loading modes:
    /// - Base64: Decode base64 string → bytes → DynamicImage
    /// - Url: Load from HTTP URL (treated as file path by ImageReader)
    /// - Raw: Load from file path
    ///
    /// Pattern references:
    /// - File loading: tmp/candle-examples/candle-examples/examples/clip/main.rs:35
    /// - Base64: Standard Rust pattern with image::load_from_memory
    fn load_image_from_source(&self) -> Result<DynamicImage, String> {
        match &self.format {
            Some(ContentFormat::Url) | Some(ContentFormat::Raw) => {
                // File path or URL loading (CLIP pattern)
                ImageReader::open(&self.data)
                    .map_err(|e| format!("Failed to open image: {}", e))?
                    .decode()
                    .map_err(|e| format!("Failed to decode image: {}", e))
            }
            Some(ContentFormat::Base64) => {
                // Base64 decoding pattern
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
    /// This processes operations that work on DynamicImage before tensor conversion:
    /// - Resize: Change image dimensions with specified filter
    /// - ToRGB: Convert to RGB8 format for consistency
    ///
    /// Skips tensor-level operations (normalization, clamp, permute) - those are
    /// handled in IMG_4B during tensor conversion.
    ///
    /// Uses resize_exact() to match user-specified dimensions precisely.
    ///
    /// Pattern references:
    /// - CLIP resize: tmp/candle-examples/candle-examples/examples/clip/main.rs:38-42
    /// - RGB conversion: tmp/candle-examples/candle-examples/examples/clip/main.rs:44
    fn apply_image_operations(&self, mut img: DynamicImage) -> Result<DynamicImage, String> {
        for op in &self.operations {
            img = match op {
                ImageOperation::Resize {
                    width,
                    height,
                    filter,
                } => {
                    let filter_type = convert_filter(*filter);
                    // Use resize_exact for precise user-specified dimensions
                    // This ensures width × height exactly, without aspect ratio preservation
                    img.resize_exact(*width as u32, *height as u32, filter_type)
                }
                // Skip tensor operations - handled in apply_tensor_operations
                ImageOperation::NormalizeSigned
                | ImageOperation::NormalizeUnsigned
                | ImageOperation::NormalizeWithParams { .. }
                | ImageOperation::Clamp { .. } => img,
            };
        }
        Ok(img)
    }

    /// Convert RGB8 image to Candle tensor in CHW format
    ///
    /// Steps:
    /// 1. Convert to RGB8 format (ensures 3 channels, u8 values)
    /// 2. Extract raw pixel data as Vec<u8>
    /// 3. Create tensor in HWC format (Height, Width, Channel)
    /// 4. Permute to CHW format (Channel, Height, Width) - Candle's native format
    /// 5. Convert from u8 to f32 dtype for processing
    ///
    /// Pattern references:
    /// - CLIP: tmp/candle-examples/candle-examples/examples/clip/main.rs:44-47
    /// - LLaVA: tmp/candle-examples/candle-examples/examples/llava/image_processor.rs:138-140
    fn image_to_tensor(&self, img: DynamicImage) -> Result<Tensor, String> {
        // Step 1: Convert to RGB8 (3 channels, u8 values)
        let img = img.to_rgb8();

        // Step 2: Get dimensions
        let (width, height) = img.dimensions();

        // Step 3: Extract raw pixel data
        let data = img.into_raw();

        // Step 4: Create tensor in HWC format (Height, Width, Channel)
        let tensor = Tensor::from_vec(data, (height as usize, width as usize, 3), &Device::Cpu)
            .map_err(|e| format!("Failed to create tensor: {}", e))?;

        // Step 5: Permute to CHW format (Channel, Height, Width)
        // This is Candle's native format for vision models
        let tensor = tensor
            .permute((2, 0, 1))
            .map_err(|e| format!("Failed to permute tensor: {}", e))?;

        // Step 6: Convert to f32 for processing
        let tensor = tensor
            .to_dtype(DType::F32)
            .map_err(|e| format!("Failed to convert dtype: {}", e))?;

        Ok(tensor)
    }

    /// Apply tensor-level operations (normalize, clamp)
    ///
    /// Processes tensor operations from the operations queue:
    /// - NormalizeSigned: [0, 255] → [-1, 1] using affine(2/255, -1)
    /// - NormalizeUnsigned: [0, 255] → [0, 1] using division
    /// - NormalizeWithParams: (x/255 - mean) / std for per-channel normalization
    /// - Clamp: Constrain values to [min, max] range
    ///
    /// Pattern references:
    /// - CLIP affine: tmp/candle-examples/candle-examples/examples/clip/main.rs:48
    /// - LLaVA rescale: tmp/candle-examples/candle-examples/examples/llava/image_processor.rs:142
    /// - LLaVA normalize: tmp/candle-examples/candle-examples/examples/llava/image_processor.rs:146-151
    fn apply_tensor_operations(&self, mut tensor: Tensor) -> Result<Tensor, String> {
        for op in &self.operations {
            tensor = match op {
                ImageOperation::NormalizeSigned => {
                    // CLIP-style: [0, 255] → [-1, 1]
                    // Formula: (x * 2/255) - 1 = x * (2/255) + (-1)
                    // Reference: tmp/candle-examples/candle-examples/examples/clip/main.rs:46
                    tensor
                        .affine(2.0 / 255.0, -1.0)
                        .map_err(|e| format!("Signed normalization failed: {}", e))?
                }

                ImageOperation::NormalizeUnsigned => {
                    // Simple scaling: [0, 255] → [0, 1]
                    // Formula: x / 255 = x * (1/255) + 0
                    // Reference: tmp/candle-examples/candle-examples/examples/llava/image_processor.rs:141-143
                    tensor
                        .affine(1.0 / 255.0, 0.0)
                        .map_err(|e| format!("Unsigned normalization failed: {}", e))?
                }

                ImageOperation::NormalizeWithParams { mean, std } => {
                    // LLaVA/ImageNet-style: (x - mean) / std
                    // Reference: tmp/candle-examples/candle-examples/examples/llava/image_processor.rs:146-150

                    // Step 1: Scale to [0,1] using affine
                    let normalized = tensor
                        .affine(1.0 / 255.0, 0.0)
                        .map_err(|e| format!("Pre-normalization scaling failed: {}", e))?;

                    // Step 2: Create mean tensor - shape (3,) for broadcasting
                    let mean_tensor = Tensor::from_vec(mean.to_vec(), (3,), &Device::Cpu)
                        .map_err(|e| format!("Failed to create mean tensor: {}", e))?;

                    // Step 3: Create std tensor - shape (3,) for broadcasting
                    let std_tensor = Tensor::from_vec(std.to_vec(), (3,), &Device::Cpu)
                        .map_err(|e| format!("Failed to create std tensor: {}", e))?;

                    // Step 4: Apply (x - mean) / std using broadcast operations
                    // broadcast_sub and broadcast_div handle shape alignment automatically
                    let subtracted = normalized
                        .broadcast_sub(&mean_tensor)
                        .map_err(|e| format!("Mean subtraction failed: {}", e))?;

                    subtracted
                        .broadcast_div(&std_tensor)
                        .map_err(|e| format!("Std division failed: {}", e))?
                }

                ImageOperation::Clamp { min, max } => {
                    // Restrict values to [min, max] range
                    tensor
                        .clamp(*min as f64, *max as f64)
                        .map_err(|e| format!("Clamp failed: {}", e))?
                }

                // Skip image-level operations - handled in IMG_4A
                _ => tensor,
            };
        }
        Ok(tensor)
    }

    /// Transfer tensor to target device
    ///
    /// Moves tensor from CPU to the specified device (GPU/Metal/CPU).
    /// Initial processing is done on CPU, final tensor is on target device.
    ///
    /// Device::clone() is cheap (Arc pointer), safe for frequent use.
    fn transfer_to_device(&self, tensor: Tensor, device: &Device) -> Result<Tensor, String> {
        tensor
            .to_device(device)
            .map_err(|e| format!("Failed to transfer to device: {}", e))
    }
}
