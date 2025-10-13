//! CLIP vision provider for image embeddings
//!
//! This provider uses ClipModel.get_image_features() for encoding images to embeddings.
//! Supports ViT-Base-Patch32 (224×224, 512-dim) and ViT-Large-Patch14 (336×336, 768-dim).

use crate::builders::image::{ImageBuilder, ResizeFilter};
use crate::domain::image::Image;
use crate::domain::model::CandleModelInfo;
use crate::domain::model::traits::CandleModel;
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::clip::text_model::ClipTextConfig;
use candle_transformers::models::clip::vision_model::ClipVisionConfig;
use candle_transformers::models::clip::{ClipConfig, ClipModel};

/// CLIP vision provider for image embeddings
///
/// Uses ClipModel.get_image_features() for encoding images to embeddings.
/// Supports ViT-Base-Patch32 (224×224, 512-dim) and ViT-Large-Patch14 (336×336, 768-dim).
///
/// Uses lazy loading pattern - model loaded on-demand via huggingface_file().
pub struct ClipVisionModel {
    dimension: usize, // 512 for Base, 768 for Large
}

impl std::fmt::Debug for ClipVisionModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClipVisionModel")
            .field("dimension", &self.dimension)
            .finish()
    }
}

impl ClipVisionModel {
    /// Create new CLIP Vision model instance with specified dimension
    ///
    /// # Arguments
    /// * `dimension` - Embedding dimension: 512 for ViT-Base-Patch32, 768 for ViT-Large-Patch14-336
    ///
    /// Uses lazy loading - model loaded on-demand via huggingface_file()
    pub fn new(dimension: usize) -> Result<Self, String> {
        if dimension != 512 && dimension != 768 {
            return Err(format!(
                "Unsupported dimension {}. CLIP supports 512 (Base) or 768 (Large)",
                dimension
            ));
        }
        Ok(Self { dimension })
    }

    /// Get CLIP configs for the specified dimension
    ///
    /// Returns (text_config, vision_config, image_size) tuple.
    /// Note: text_config is required by ClipModel but unused for vision-only inference.
    fn get_configs_for_dimension(&self) -> (ClipTextConfig, ClipVisionConfig, usize) {
        use candle_transformers::models::clip::text_model::Activation;

        match self.dimension {
            512 => (
                ClipTextConfig::vit_base_patch32(),
                ClipVisionConfig::vit_base_patch32(),
                224, // image_size for Base
            ),
            768 => (
                // Manual ClipTextConfig for Large (unused in vision-only inference)
                ClipTextConfig {
                    vocab_size: 49408,
                    embed_dim: 768,
                    intermediate_size: 3072,
                    max_position_embeddings: 77,
                    pad_with: None,
                    num_hidden_layers: 12,
                    num_attention_heads: 12,
                    projection_dim: 768,
                    activation: Activation::QuickGelu,
                },
                ClipVisionConfig::clip_vit_large_patch14_336(),
                336, // image_size for Large
            ),
            _ => unreachable!("Dimension validated in constructor"),
        }
    }

    /// Encode image from file path to embedding vector
    ///
    /// Uses lazy loading pattern:
    /// - Gets config from ModelInfo (single source of truth)
    /// - Auto-detects device at runtime
    /// - Loads model on-demand via huggingface_file()
    /// - Uses correct CLIP normalization: normalize_unsigned() + normalize_with(mean, std)
    ///
    /// Returns projected embeddings via model.get_image_features()
    pub async fn encode_image(&self, image_path: &str) -> Result<Tensor, String> {
        // 1. GET CONFIG FROM ModelInfo - Single source of truth
        let image_size = self
            .info()
            .image_size
            .ok_or("image_size missing from ModelInfo")? as usize;
        let image_mean = self
            .info()
            .image_mean
            .ok_or("image_mean missing from ModelInfo")?;
        let image_std = self
            .info()
            .image_std
            .ok_or("image_std missing from ModelInfo")?;

        // 2. AUTO-DETECT DEVICE - Runtime decision
        let device = Device::cuda_if_available(0).unwrap_or(Device::Cpu);

        // 3. LAZY MODEL LOADING - Load model file on-demand
        let model_path = self
            .huggingface_file(self.info().registry_key, "model.safetensors")
            .map_err(|e| format!("Failed to get model file: {}", e))?;

        // 4. BUILD CLIP CONFIG - Select configs based on dimension
        let (text_config, vision_config, _) = self.get_configs_for_dimension();
        let config = ClipConfig {
            text_config,
            vision_config,
            logit_scale_init_value: 2.6592,
            image_size,
        };

        // 5. LOAD MODEL - From huggingface_file path
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[model_path], candle_core::DType::F32, &device)
                .map_err(|e| format!("Failed to load model: {}", e))?
        };
        let model =
            ClipModel::new(vb, &config).map_err(|e| format!("Failed to create model: {}", e))?;

        // 6. PREPROCESS IMAGE - Use CORRECT normalization pipeline
        let image_tensor = Image::from_path(image_path)
            .resize(image_size, image_size, ResizeFilter::Triangle)
            .normalize_unsigned() // Step 1: [0, 255] → [0, 1]
            .normalize_with(image_mean, image_std) // Step 2: (x - mean) / std
            .to_tensor(&device)
            .await?;

        // 7. ADD BATCH DIMENSION - (C,H,W) → (1,C,H,W)
        let batched = image_tensor
            .unsqueeze(0)
            .map_err(|e| format!("Failed to add batch dimension: {}", e))?;

        // 8. ENCODE - Use get_image_features() for vision embedding
        model
            .get_image_features(&batched)
            .map_err(|e| format!("CLIP encoding failed: {}", e))
    }

    /// Encode image from URL
    pub async fn encode_url(&self, url: &str) -> Result<Tensor, String> {
        // 1. GET CONFIG FROM ModelInfo
        let image_size = self
            .info()
            .image_size
            .ok_or("image_size missing from ModelInfo")? as usize;
        let image_mean = self
            .info()
            .image_mean
            .ok_or("image_mean missing from ModelInfo")?;
        let image_std = self
            .info()
            .image_std
            .ok_or("image_std missing from ModelInfo")?;

        // 2. AUTO-DETECT DEVICE
        let device = Device::cuda_if_available(0).unwrap_or(Device::Cpu);

        // 3. LAZY MODEL LOADING
        let model_path = self
            .huggingface_file(self.info().registry_key, "model.safetensors")
            .map_err(|e| format!("Failed to get model file: {}", e))?;

        // 4. BUILD CLIP CONFIG - Select configs based on dimension
        let (text_config, vision_config, _) = self.get_configs_for_dimension();
        let config = ClipConfig {
            text_config,
            vision_config,
            logit_scale_init_value: 2.6592,
            image_size,
        };

        // 5. LOAD MODEL
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[model_path], candle_core::DType::F32, &device)
                .map_err(|e| format!("Failed to load model: {}", e))?
        };
        let model =
            ClipModel::new(vb, &config).map_err(|e| format!("Failed to create model: {}", e))?;

        // 6. PREPROCESS IMAGE - CORRECT normalization
        let image_tensor = Image::from_url(url)
            .resize(image_size, image_size, ResizeFilter::Triangle)
            .normalize_unsigned()
            .normalize_with(image_mean, image_std)
            .to_tensor(&device)
            .await?;

        // 7. ADD BATCH DIMENSION
        let batched = image_tensor
            .unsqueeze(0)
            .map_err(|e| format!("Failed to add batch dimension: {}", e))?;

        // 8. ENCODE
        model
            .get_image_features(&batched)
            .map_err(|e| format!("CLIP encoding failed: {}", e))
    }

    /// Encode image from base64 data (for API usage)
    pub async fn encode_base64(&self, base64_data: &str) -> Result<Tensor, String> {
        // 1. GET CONFIG FROM ModelInfo
        let image_size = self
            .info()
            .image_size
            .ok_or("image_size missing from ModelInfo")? as usize;
        let image_mean = self
            .info()
            .image_mean
            .ok_or("image_mean missing from ModelInfo")?;
        let image_std = self
            .info()
            .image_std
            .ok_or("image_std missing from ModelInfo")?;

        // 2. AUTO-DETECT DEVICE
        let device = Device::cuda_if_available(0).unwrap_or(Device::Cpu);

        // 3. LAZY MODEL LOADING
        let model_path = self
            .huggingface_file(self.info().registry_key, "model.safetensors")
            .map_err(|e| format!("Failed to get model file: {}", e))?;

        // 4. BUILD CLIP CONFIG - Select configs based on dimension
        let (text_config, vision_config, _) = self.get_configs_for_dimension();
        let config = ClipConfig {
            text_config,
            vision_config,
            logit_scale_init_value: 2.6592,
            image_size,
        };

        // 5. LOAD MODEL
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[model_path], candle_core::DType::F32, &device)
                .map_err(|e| format!("Failed to load model: {}", e))?
        };
        let model =
            ClipModel::new(vb, &config).map_err(|e| format!("Failed to create model: {}", e))?;

        // 6. PREPROCESS IMAGE - CORRECT normalization
        let image_tensor = Image::from_base64(base64_data)
            .resize(image_size, image_size, ResizeFilter::Triangle)
            .normalize_unsigned()
            .normalize_with(image_mean, image_std)
            .to_tensor(&device)
            .await?;

        // 7. ADD BATCH DIMENSION
        let batched = image_tensor
            .unsqueeze(0)
            .map_err(|e| format!("Failed to add batch dimension: {}", e))?;

        // 8. ENCODE
        model
            .get_image_features(&batched)
            .map_err(|e| format!("CLIP encoding failed: {}", e))
    }

    /// Encode multiple images in batch
    pub async fn encode_batch(&self, image_paths: Vec<&str>) -> Result<Tensor, String> {
        // 1. GET CONFIG FROM ModelInfo
        let image_size = self
            .info()
            .image_size
            .ok_or("image_size missing from ModelInfo")? as usize;
        let image_mean = self
            .info()
            .image_mean
            .ok_or("image_mean missing from ModelInfo")?;
        let image_std = self
            .info()
            .image_std
            .ok_or("image_std missing from ModelInfo")?;

        // 2. AUTO-DETECT DEVICE
        let device = Device::cuda_if_available(0).unwrap_or(Device::Cpu);

        // 3. LAZY MODEL LOADING
        let model_path = self
            .huggingface_file(self.info().registry_key, "model.safetensors")
            .map_err(|e| format!("Failed to get model file: {}", e))?;

        // 4. BUILD CLIP CONFIG - Select configs based on dimension
        let (text_config, vision_config, _) = self.get_configs_for_dimension();
        let config = ClipConfig {
            text_config,
            vision_config,
            logit_scale_init_value: 2.6592,
            image_size,
        };

        // 5. LOAD MODEL
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[model_path], candle_core::DType::F32, &device)
                .map_err(|e| format!("Failed to load model: {}", e))?
        };
        let model =
            ClipModel::new(vb, &config).map_err(|e| format!("Failed to create model: {}", e))?;

        // 6. PREPROCESS ALL IMAGES - CORRECT normalization
        let mut tensors = Vec::new();
        for path in image_paths {
            let tensor = Image::from_path(path)
                .resize(image_size, image_size, ResizeFilter::Triangle)
                .normalize_unsigned()
                .normalize_with(image_mean, image_std)
                .to_tensor(&device)
                .await?;
            tensors.push(tensor);
        }

        // 7. STACK INTO BATCH: [(C,H,W), (C,H,W), ...] → (N,C,H,W)
        let batched =
            Tensor::stack(&tensors, 0).map_err(|e| format!("Failed to batch tensors: {}", e))?;

        // 8. ENCODE ENTIRE BATCH
        model
            .get_image_features(&batched)
            .map_err(|e| format!("Batch encoding failed: {}", e))
    }
}

// Static model info for CLIP Vision Base (512D)
static CLIP_VISION_BASE_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::OpenAI,
    name: "clip-vit-base-patch32",
    registry_key: "openai/clip-vit-base-patch32",
    quantization_url: None,
    max_input_tokens: None,
    max_output_tokens: None,
    input_price: None,
    output_price: None,
    supports_vision: true,
    supports_function_calling: false,
    supports_streaming: false,
    supports_embeddings: true,
    requires_max_tokens: false,
    supports_thinking: false,
    optimal_thinking_budget: None,
    system_prompt_prefix: None,
    real_name: None,
    model_type: None,
    model_id: "clip-vision-base",
    quantization: "none",
    patch: None,
    embedding_dimension: Some(512),
    vocab_size: None,
    image_size: Some(224),
    image_mean: Some([0.48145466, 0.4578275, 0.40821073]),
    image_std: Some([0.26862954, 0.261_302_6, 0.275_777_1]),
    default_temperature: None,
    default_top_k: None,
    default_top_p: None,
    supports_kv_cache: false,
    supports_flash_attention: false,
    use_bf16: false,
    default_steps: None,
    default_guidance_scale: None,
    time_shift: None,
    est_memory_allocation_mb: 0,
};

// Static model info for CLIP Vision Large (768D)
static CLIP_VISION_LARGE_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::OpenAI,
    name: "clip-vit-large-patch14-336",
    registry_key: "openai/clip-vit-large-patch14-336",
    quantization_url: None,
    max_input_tokens: None,
    max_output_tokens: None,
    input_price: None,
    output_price: None,
    supports_vision: true,
    supports_function_calling: false,
    supports_streaming: false,
    supports_embeddings: true,
    requires_max_tokens: false,
    supports_thinking: false,
    optimal_thinking_budget: None,
    system_prompt_prefix: None,
    real_name: None,
    model_type: None,
    model_id: "clip-vision-large",
    quantization: "none",
    patch: None,
    embedding_dimension: Some(768),
    vocab_size: None,
    image_size: Some(336),
    image_mean: Some([0.48145466, 0.4578275, 0.40821073]),
    image_std: Some([0.26862954, 0.261_302_6, 0.275_777_1]),
    default_temperature: None,
    default_top_k: None,
    default_top_p: None,
    supports_kv_cache: false,
    supports_flash_attention: false,
    use_bf16: false,
    default_steps: None,
    default_guidance_scale: None,
    time_shift: None,
    est_memory_allocation_mb: 0,
};

impl CandleModel for ClipVisionModel {
    fn info(&self) -> &'static CandleModelInfo {
        match self.dimension {
            512 => &CLIP_VISION_BASE_INFO,
            768 => &CLIP_VISION_LARGE_INFO,
            _ => unreachable!("Dimension validated in constructor"),
        }
    }
}

impl crate::capability::traits::ImageEmbeddingCapable for ClipVisionModel {
    fn embed_image(
        &self,
        image_path: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>>> + Send + '_>> {
        let image_path = image_path.to_string();
        Box::pin(async move {
            // Encode image to tensor (1, embed_dim)
            let tensor = self.encode_image(&image_path).await.map_err(|e| {
                Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error + Send + Sync>
            })?;

            // Convert to Vec<f32>
            let embedding = tensor
                .to_vec1::<f32>()
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

            Ok(embedding)
        })
    }

    fn embed_image_url(
        &self,
        url: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>>> + Send + '_>> {
        let url = url.to_string();
        Box::pin(async move {
            // Encode image from URL
            let tensor = self.encode_url(&url).await.map_err(|e| {
                Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error + Send + Sync>
            })?;

            // Convert to Vec<f32>
            let embedding = tensor
                .to_vec1::<f32>()
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

            Ok(embedding)
        })
    }

    fn embed_image_base64(
        &self,
        base64_data: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>>> + Send + '_>> {
        let base64_data = base64_data.to_string();
        Box::pin(async move {
            // Encode image from base64
            let tensor = self.encode_base64(&base64_data).await.map_err(|e| {
                Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error + Send + Sync>
            })?;

            // Convert to Vec<f32>
            let embedding = tensor
                .to_vec1::<f32>()
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

            Ok(embedding)
        })
    }

    fn batch_embed_images(
        &self,
        image_paths: Vec<&str>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::result::Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>>> + Send + '_>> {
        let paths: Vec<String> = image_paths.iter().map(|s| s.to_string()).collect();
        Box::pin(async move {
            let mut embeddings = Vec::with_capacity(paths.len());
            for path in &paths {
                let embedding = self.embed_image(path).await?;
                embeddings.push(embedding);
            }
            Ok(embeddings)
        })
    }

    fn embedding_dimension(&self) -> usize {
        self.info().embedding_dimension.unwrap_or(512) as usize
    }

    fn supported_dimensions(&self) -> Vec<usize> {
        // CLIP Vision supports both Base (512D) and Large (768D) variants
        vec![512, 768]
    }
}
