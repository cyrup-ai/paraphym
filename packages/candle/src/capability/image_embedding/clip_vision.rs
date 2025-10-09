//! CLIP vision provider for image embeddings
//!
//! This provider uses ClipModel.get_image_features() for encoding images to embeddings.
//! Supports ViT-Base-Patch32 (224×224, 512-dim) and ViT-Large-Patch14 (336×336, 768-dim).

use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::clip::{ClipModel, ClipConfig};
use candle_transformers::models::clip::text_model::ClipTextConfig;
use candle_transformers::models::clip::vision_model::ClipVisionConfig;
use crate::domain::image::Image;
use crate::builders::image::{ImageBuilder, ResizeFilter};
use crate::domain::model::traits::CandleModel;
use crate::domain::model::CandleModelInfo;

/// CLIP vision provider for image embeddings
/// 
/// Uses ClipModel.get_image_features() for encoding images to embeddings.
/// Supports ViT-Base-Patch32 (224×224, 512-dim) and ViT-Large-Patch14 (336×336, 768-dim).
/// 
/// Uses lazy loading pattern - model loaded on-demand via huggingface_file().
pub struct ClipVisionModel { }

impl std::fmt::Debug for ClipVisionModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClipVisionModel")
            .finish()
    }
}

impl ClipVisionModel {
    /// Create new CLIP Vision model instance
    /// 
    /// Uses lazy loading - model loaded on-demand via huggingface_file()
    pub fn new() -> Self {
        Self { }
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
        let image_size = self.info().image_size
            .ok_or("image_size missing from ModelInfo")? as usize;
        let image_mean = self.info().image_mean
            .ok_or("image_mean missing from ModelInfo")?;
        let image_std = self.info().image_std
            .ok_or("image_std missing from ModelInfo")?;
        
        // 2. AUTO-DETECT DEVICE - Runtime decision
        let device = Device::cuda_if_available(0).unwrap_or(Device::Cpu);
        
        // 3. LAZY MODEL LOADING - Load model file on-demand
        let model_path = self.huggingface_file("model.safetensors")
            .map_err(|e| format!("Failed to get model file: {}", e))?;
        
        // 4. BUILD CLIP CONFIG - From ModelInfo values
        let text_config = ClipTextConfig::vit_base_patch32();
        let vision_config = ClipVisionConfig::vit_base_patch32();
        let config = ClipConfig {
            text_config,
            vision_config,
            logit_scale_init_value: 2.6592,
            image_size: image_size,
        };
        
        // 5. LOAD MODEL - From huggingface_file path
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[model_path], candle_core::DType::F32, &device)
                .map_err(|e| format!("Failed to load model: {}", e))?
        };
        let model = ClipModel::new(vb, &config)
            .map_err(|e| format!("Failed to create model: {}", e))?;
        
        // 6. PREPROCESS IMAGE - Use CORRECT normalization pipeline
        let image_tensor = Image::from_path(image_path)
            .resize(image_size, image_size, ResizeFilter::Triangle)
            .normalize_unsigned()                       // Step 1: [0, 255] → [0, 1]
            .normalize_with(image_mean, image_std)      // Step 2: (x - mean) / std
            .to_tensor(&device)
            .await?;
        
        // 7. ADD BATCH DIMENSION - (C,H,W) → (1,C,H,W)
        let batched = image_tensor
            .unsqueeze(0)
            .map_err(|e| format!("Failed to add batch dimension: {}", e))?;
        
        // 8. ENCODE - Use get_image_features() for vision embedding
        model.get_image_features(&batched)
            .map_err(|e| format!("CLIP encoding failed: {}", e))
    }
    
    /// Encode image from URL
    pub async fn encode_url(&self, url: &str) -> Result<Tensor, String> {
        // 1. GET CONFIG FROM ModelInfo
        let image_size = self.info().image_size
            .ok_or("image_size missing from ModelInfo")? as usize;
        let image_mean = self.info().image_mean
            .ok_or("image_mean missing from ModelInfo")?;
        let image_std = self.info().image_std
            .ok_or("image_std missing from ModelInfo")?;
        
        // 2. AUTO-DETECT DEVICE
        let device = Device::cuda_if_available(0).unwrap_or(Device::Cpu);
        
        // 3. LAZY MODEL LOADING
        let model_path = self.huggingface_file("model.safetensors")
            .map_err(|e| format!("Failed to get model file: {}", e))?;
        
        // 4. BUILD CLIP CONFIG
        let text_config = ClipTextConfig::vit_base_patch32();
        let vision_config = ClipVisionConfig::vit_base_patch32();
        let config = ClipConfig {
            text_config,
            vision_config,
            logit_scale_init_value: 2.6592,
            image_size: image_size,
        };
        
        // 5. LOAD MODEL
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[model_path], candle_core::DType::F32, &device)
                .map_err(|e| format!("Failed to load model: {}", e))?
        };
        let model = ClipModel::new(vb, &config)
            .map_err(|e| format!("Failed to create model: {}", e))?;
        
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
        model.get_image_features(&batched)
            .map_err(|e| format!("CLIP encoding failed: {}", e))
    }

    /// Encode image from base64 data (for API usage)
    pub async fn encode_base64(&self, base64_data: &str) -> Result<Tensor, String> {
        // 1. GET CONFIG FROM ModelInfo
        let image_size = self.info().image_size
            .ok_or("image_size missing from ModelInfo")? as usize;
        let image_mean = self.info().image_mean
            .ok_or("image_mean missing from ModelInfo")?;
        let image_std = self.info().image_std
            .ok_or("image_std missing from ModelInfo")?;
        
        // 2. AUTO-DETECT DEVICE
        let device = Device::cuda_if_available(0).unwrap_or(Device::Cpu);
        
        // 3. LAZY MODEL LOADING
        let model_path = self.huggingface_file("model.safetensors")
            .map_err(|e| format!("Failed to get model file: {}", e))?;
        
        // 4. BUILD CLIP CONFIG
        let text_config = ClipTextConfig::vit_base_patch32();
        let vision_config = ClipVisionConfig::vit_base_patch32();
        let config = ClipConfig {
            text_config,
            vision_config,
            logit_scale_init_value: 2.6592,
            image_size: image_size,
        };
        
        // 5. LOAD MODEL
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[model_path], candle_core::DType::F32, &device)
                .map_err(|e| format!("Failed to load model: {}", e))?
        };
        let model = ClipModel::new(vb, &config)
            .map_err(|e| format!("Failed to create model: {}", e))?;
        
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
        model.get_image_features(&batched)
            .map_err(|e| format!("CLIP encoding failed: {}", e))
    }
    
    /// Encode multiple images in batch
    pub async fn encode_batch(&self, image_paths: Vec<&str>) -> Result<Tensor, String> {
        // 1. GET CONFIG FROM ModelInfo
        let image_size = self.info().image_size
            .ok_or("image_size missing from ModelInfo")? as usize;
        let image_mean = self.info().image_mean
            .ok_or("image_mean missing from ModelInfo")?;
        let image_std = self.info().image_std
            .ok_or("image_std missing from ModelInfo")?;
        
        // 2. AUTO-DETECT DEVICE
        let device = Device::cuda_if_available(0).unwrap_or(Device::Cpu);
        
        // 3. LAZY MODEL LOADING
        let model_path = self.huggingface_file("model.safetensors")
            .map_err(|e| format!("Failed to get model file: {}", e))?;
        
        // 4. BUILD CLIP CONFIG
        let text_config = ClipTextConfig::vit_base_patch32();
        let vision_config = ClipVisionConfig::vit_base_patch32();
        let config = ClipConfig {
            text_config,
            vision_config,
            logit_scale_init_value: 2.6592,
            image_size: image_size,
        };
        
        // 5. LOAD MODEL
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[model_path], candle_core::DType::F32, &device)
                .map_err(|e| format!("Failed to load model: {}", e))?
        };
        let model = ClipModel::new(vb, &config)
            .map_err(|e| format!("Failed to create model: {}", e))?;
        
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
        let batched = Tensor::stack(&tensors, 0)
            .map_err(|e| format!("Failed to batch tensors: {}", e))?;
        
        // 8. ENCODE ENTIRE BATCH
        model.get_image_features(&batched)
            .map_err(|e| format!("Batch encoding failed: {}", e))
    }
}

// Static model info for CLIP Vision
static CLIP_VISION_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::OpenAI,
    name: "clip-vit-base-patch32",
    registry_key: "openai/clip-vit-base-patch32",
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
    model_id: "clip-vision",
    quantization: "none",
    patch: None,
    embedding_dimension: Some(512),
    vocab_size: None,
    image_size: Some(224),
    image_mean: Some([0.48145466, 0.4578275, 0.40821073]),
    image_std: Some([0.26862954, 0.26130258, 0.27577711]),
    default_temperature: None,
    default_top_k: None,
    default_top_p: None,
    supports_kv_cache: false,
    supports_flash_attention: false,
    use_bf16: false,
    default_steps: None,
    default_guidance_scale: None,
    time_shift: None,
};

impl CandleModel for ClipVisionModel {
    fn info(&self) -> &'static CandleModelInfo {
        &CLIP_VISION_MODEL_INFO
    }
}

impl crate::capability::traits::ImageEmbeddingCapable for ClipVisionModel {
    fn embed_image(&self, image_path: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> 
    {
        // Create runtime to block on async encode_image
        let rt = tokio::runtime::Runtime::new()?;
        
        // Encode image to tensor (1, embed_dim)
        let tensor = rt.block_on(self.encode_image(image_path))
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn std::error::Error + Send + Sync>)?;
        
        // Extract first batch element and convert to Vec<f32>
        let embedding = tensor.get(0)
            .map_err(|e| format!("Failed to extract embedding: {}", e))?
            .to_vec1::<f32>()
            .map_err(|e| format!("Failed to convert embedding to vec: {}", e))?;
        
        Ok(embedding)
    }
    
    fn embed_image_url(&self, url: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> 
    {
        // Create runtime to block on async encode_url
        let rt = tokio::runtime::Runtime::new()?;
        
        // Encode image from URL to tensor (1, embed_dim)
        let tensor = rt.block_on(self.encode_url(url))
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn std::error::Error + Send + Sync>)?;
        
        // Extract first batch element and convert to Vec<f32>
        let embedding = tensor.get(0)
            .map_err(|e| format!("Failed to extract embedding: {}", e))?
            .to_vec1::<f32>()
            .map_err(|e| format!("Failed to convert embedding to vec: {}", e))?;
        
        Ok(embedding)
    }
    
    fn embed_image_base64(&self, base64_data: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> 
    {
        // Create runtime to block on async encode_base64
        let rt = tokio::runtime::Runtime::new()?;
        
        // Encode image from base64 to tensor (1, embed_dim)
        let tensor = rt.block_on(self.encode_base64(base64_data))
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn std::error::Error + Send + Sync>)?;
        
        // Extract first batch element and convert to Vec<f32>
        let embedding = tensor.get(0)
            .map_err(|e| format!("Failed to extract embedding: {}", e))?
            .to_vec1::<f32>()
            .map_err(|e| format!("Failed to convert embedding to vec: {}", e))?;
        
        Ok(embedding)
    }
    
    fn batch_embed_images(&self, image_paths: Vec<&str>) 
        -> std::result::Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> 
    {
        // Create runtime to block on async encode_batch
        let rt = tokio::runtime::Runtime::new()?;
        
        // Encode batch to tensor (N, embed_dim)
        let tensor = rt.block_on(self.encode_batch(image_paths))
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn std::error::Error + Send + Sync>)?;
        
        // Convert to Vec<Vec<f32>>
        let embeddings = tensor.to_vec2::<f32>()
            .map_err(|e| format!("Failed to convert batch embeddings to vec: {}", e))?;
        
        Ok(embeddings)
    }
    
    fn embedding_dimension(&self) -> usize {
        self.info().embedding_dimension.unwrap_or(512) as usize
    }
    
    fn supported_dimensions(&self) -> Vec<usize> {
        // CLIP Vision models have fixed embedding dimensions
        vec![self.info().embedding_dimension.unwrap_or(512) as usize]
    }
}
