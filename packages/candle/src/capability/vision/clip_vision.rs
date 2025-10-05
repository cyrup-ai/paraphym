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

/// CLIP vision provider for image embeddings
/// 
/// Uses ClipModel.get_image_features() for encoding images to embeddings.
/// Supports ViT-Base-Patch32 (224×224, 512-dim) and ViT-Large-Patch14 (336×336, 768-dim).
pub struct ClipVisionProvider {
    model: ClipModel,
    config: ClipConfig,
    device: Device,
}

impl ClipVisionProvider {
    pub fn from_pretrained(model_path: &str, device: Device) -> Result<Self, String> {
        // Use ViT-Base-Patch32 configuration (224×224 → 512-dim embeddings)
        let config = ClipConfig::vit_base_patch32();
        
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(
                &[std::path::PathBuf::from(model_path)],
                candle_core::DType::F32,
                &device
            )
        }.map_err(|e| format!("Failed to load CLIP model from {}: {}", model_path, e))?;
        
        let model = ClipModel::new(vb, &config)
            .map_err(|e| format!("Failed to create CLIP model: {}", e))?;
        
        Ok(Self { model, config, device })
    }
    
    /// Create provider with ViT-Large configuration (336×336 → 768-dim embeddings)
    pub fn from_pretrained_large(model_path: &str, device: Device) -> Result<Self, String> {
        // Manually construct large config since ClipConfig doesn't have a large preset
        let text_config = ClipTextConfig::vit_base_patch32();
        let vision_config = ClipVisionConfig::clip_vit_large_patch14_336();
        let config = ClipConfig {
            text_config,
            vision_config,
            logit_scale_init_value: 2.6592,
            image_size: 336,
        };
        
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(
                &[std::path::PathBuf::from(model_path)],
                candle_core::DType::F32,
                &device
            )
        }.map_err(|e| format!("Failed to load CLIP model from {}: {}", model_path, e))?;
        
        let model = ClipModel::new(vb, &config)
            .map_err(|e| format!("Failed to create CLIP model: {}", e))?;
        
        Ok(Self { model, config, device })
    }
    
    /// Encode image from file path to embedding vector
    /// 
    /// Uses Image builder with CLIP preprocessing:
    /// - Resize to config.image_size with Triangle filter
    /// - Normalize to [-1, 1] range (CLIP standard)
    /// - Convert to CHW tensor on target device
    /// 
    /// Returns projected embeddings via model.get_image_features()
    pub async fn encode_image(&self, image_path: &str) -> Result<Tensor, String> {
        // Use Image builder for preprocessing (ASYNC!)
        let image_tensor = Image::from_path(image_path)
            .resize(
                self.config.image_size,    // 224 for ViT-Base, 336 for ViT-Large
                self.config.image_size,
                ResizeFilter::Triangle      // CLIP uses Triangle filter
            )
            .normalize_signed()             // [-1, 1] normalization (replaces affine)
            .to_tensor(&self.device)
            .await?;                        // MUST await - to_tensor returns Future
        
        // Add batch dimension: (C, H, W) → (1, C, H, W)
        let batched = image_tensor
            .unsqueeze(0)
            .map_err(|e| format!("Failed to add batch dimension: {}", e))?;
        
        // Encode through CLIP (vision + projection)
        // CRITICAL: Use get_image_features(), NOT vision_model.forward()
        self.model.get_image_features(&batched)
            .map_err(|e| format!("CLIP vision encoding failed: {}", e))
    }
    
    /// Encode image from URL
    pub async fn encode_url(&self, url: &str) -> Result<Tensor, String> {
        let image_tensor = Image::from_url(url)
            .resize(self.config.image_size, self.config.image_size, ResizeFilter::Triangle)
            .normalize_signed()
            .to_tensor(&self.device)
            .await?;  // ASYNC
        
        let batched = image_tensor
            .unsqueeze(0)
            .map_err(|e| format!("Failed to add batch dimension: {}", e))?;
        
        self.model.get_image_features(&batched)
            .map_err(|e| format!("CLIP vision encoding failed: {}", e))
    }

    /// Encode image from base64 data (for API usage)
    pub async fn encode_base64(&self, base64_data: &str) -> Result<Tensor, String> {
        let image_tensor = Image::from_base64(base64_data)
            .resize(self.config.image_size, self.config.image_size, ResizeFilter::Triangle)
            .normalize_signed()
            .to_tensor(&self.device)
            .await?;  // ASYNC
        
        let batched = image_tensor
            .unsqueeze(0)
            .map_err(|e| format!("Failed to add batch dimension: {}", e))?;
        
        self.model.get_image_features(&batched)
            .map_err(|e| format!("CLIP vision encoding failed: {}", e))
    }
    
    /// Encode multiple images in batch
    pub async fn encode_batch(&self, image_paths: Vec<&str>) -> Result<Tensor, String> {
        let mut tensors = Vec::new();
        
        for path in image_paths {
            let tensor = Image::from_path(path)
                .resize(self.config.image_size, self.config.image_size, ResizeFilter::Triangle)
                .normalize_signed()
                .to_tensor(&self.device)
                .await?;  // ASYNC
            tensors.push(tensor);
        }
        
        // Stack into batch: [(C,H,W), (C,H,W), ...] → (N,C,H,W)
        let batched = Tensor::stack(&tensors, 0)
            .map_err(|e| format!("Failed to batch tensors: {}", e))?;
        
        // Encode entire batch
        self.model.get_image_features(&batched)
            .map_err(|e| format!("Batch encoding failed: {}", e))
    }
}
