//! CLIP Vision Embedding Provider - Sync Wrapper for EmbeddingModel Trait
//!
//! This module provides a synchronous wrapper around ClipVisionProvider to enable
//! integration with the EmbeddingModel trait system and EmbeddingModelFactory.

use std::sync::Arc;

use crate::memory::utils::error::{Error as MemoryError, Result};
use super::ClipVisionModel;
use crate::domain::model::traits::CandleModel;
use crate::domain::model::CandleModelInfo;

/// Synchronous wrapper for ClipVisionModel implementing EmbeddingModel trait
///
/// This adapter bridges the async ClipVisionModel with the sync EmbeddingModel trait
/// by using tokio runtime to execute async operations synchronously.
pub struct ClipVisionEmbeddingModel {
    provider: Arc<ClipVisionModel>,
    dimension: usize,
}

impl std::fmt::Debug for ClipVisionEmbeddingModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClipVisionEmbeddingModel")
            .field("provider", &"ClipVisionModel { .. }")
            .field("dimension", &self.dimension)
            .finish()
    }
}

impl Default for ClipVisionEmbeddingModel {
    fn default() -> Self {
        crate::runtime::shared_runtime()
            .unwrap_or_else(|| panic!("Shared runtime unavailable"))
            .block_on(Self::new())
            .unwrap_or_else(|e| panic!("Failed to initialize ClipVisionEmbeddingModel: {}", e))
    }
}

impl ClipVisionEmbeddingModel {
    /// Create new CLIP vision embedding provider with ViT-Base configuration (512D)
    pub async fn new() -> Result<Self> {
        Self::with_dimension(512).await
    }

    /// Create CLIP vision provider with specific dimension
    ///
    /// # Arguments
    /// * `dimension` - 512 for ViT-Base-Patch32 or 768 for ViT-Large-Patch14-336
    /// 
    /// Note: ClipVisionModel now uses lazy loading, so no explicit download needed.
    /// Model files are downloaded on-demand via huggingface_file().
    pub async fn with_dimension(dimension: usize) -> Result<Self> {
        // Validate dimension (CLIP supports 512D for Base, 768D for Large)
        if dimension != 512 && dimension != 768 {
            return Err(MemoryError::Config(format!(
                "CLIP Vision supports 512D (Base) or 768D (Large). Requested: {}",
                dimension
            )));
        }
        
        // Create provider with specified dimension
        // Model will be downloaded and loaded on-demand via huggingface_file()
        let provider = ClipVisionModel::new(dimension)
            .map_err(|e| MemoryError::Config(e))?;

        Ok(Self {
            provider: Arc::new(provider),
            dimension,
        })
    }

    /// Create from existing ClipVisionModel
    pub fn from_model(model: ClipVisionModel, dimension: usize) -> Self {
        Self {
            provider: Arc::new(model),
            dimension,
        }
    }

    /// Deprecated: Use from_model instead
    #[deprecated(since = "0.1.0", note = "Use from_model instead")]
    pub fn from_provider(provider: ClipVisionModel, dimension: usize) -> Self {
        Self {
            provider: Arc::new(provider),
            dimension,
        }
    }

    /// Encode image from file path (public API for direct image embedding)
    ///
    /// This is the primary method for encoding images to embeddings.
    pub fn embed_image(&self, image_path: &str) -> Result<Vec<f32>> {
        // Create tokio runtime for async execution
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| MemoryError::ModelError(format!("Failed to create runtime: {}", e)))?;

        // Execute async operation synchronously
        let provider = self.provider.clone();
        let tensor = runtime
            .block_on(provider.encode_image(image_path))
            .map_err(|e| MemoryError::ModelError(format!("Image encoding failed: {}", e)))?;

        // Convert Tensor to Vec<f32>
        tensor
            .flatten_all()
            .and_then(|t| t.to_vec1::<f32>())
            .map_err(|e| MemoryError::ModelError(format!("Failed to convert tensor to vector: {}", e)))
    }

    /// Encode image from URL
    pub fn embed_image_url(&self, url: &str) -> Result<Vec<f32>> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| MemoryError::ModelError(format!("Failed to create runtime: {}", e)))?;

        let provider = self.provider.clone();
        let tensor = runtime
            .block_on(provider.encode_url(url))
            .map_err(|e| MemoryError::ModelError(format!("Image URL encoding failed: {}", e)))?;

        tensor
            .flatten_all()
            .and_then(|t| t.to_vec1::<f32>())
            .map_err(|e| MemoryError::ModelError(format!("Failed to convert tensor to vector: {}", e)))
    }

    /// Encode image from base64 data
    pub fn embed_image_base64(&self, base64_data: &str) -> Result<Vec<f32>> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| MemoryError::ModelError(format!("Failed to create runtime: {}", e)))?;

        let provider = self.provider.clone();
        let tensor = runtime
            .block_on(provider.encode_base64(base64_data))
            .map_err(|e| MemoryError::ModelError(format!("Base64 image encoding failed: {}", e)))?;

        tensor
            .flatten_all()
            .and_then(|t| t.to_vec1::<f32>())
            .map_err(|e| MemoryError::ModelError(format!("Failed to convert tensor to vector: {}", e)))
    }

    /// Batch encode multiple images
    pub fn batch_embed_images(&self, image_paths: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| MemoryError::ModelError(format!("Failed to create runtime: {}", e)))?;

        let provider = self.provider.clone();
        let batch_tensor = runtime
            .block_on(provider.encode_batch(image_paths))
            .map_err(|e| MemoryError::ModelError(format!("Batch image encoding failed: {}", e)))?;

        // Convert batch tensor (N, D) to Vec<Vec<f32>>
        let batch_size = batch_tensor.dim(0)
            .map_err(|e| MemoryError::ModelError(format!("Failed to get batch size: {}", e)))?;

        let mut embeddings = Vec::with_capacity(batch_size);

        for i in 0..batch_size {
            let row = batch_tensor
                .get(i)
                .and_then(|t| t.flatten_all())
                .and_then(|t| t.to_vec1::<f32>())
                .map_err(|e| MemoryError::ModelError(format!("Failed to extract embedding {}: {}", i, e)))?;
            embeddings.push(row);
        }

        Ok(embeddings)
    }
}


// Static model info for CLIP Vision Embedding
#[allow(dead_code)] // Reserved for future vision embedding model registry
static CLIP_VISION_EMBEDDING_MODEL_INFO: CandleModelInfo = CandleModelInfo {
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
    model_id: "clip-vision-embedding",
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

impl CandleModel for ClipVisionEmbeddingModel {
    fn info(&self) -> &'static CandleModelInfo {
        // Delegate to provider which has correct ModelInfo based on dimension
        self.provider.info()
    }
}

impl crate::capability::traits::ImageEmbeddingCapable for ClipVisionEmbeddingModel {
    fn embed_image(&self, image_path: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> 
    {
        ClipVisionEmbeddingModel::embed_image(self, image_path)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
    
    fn embed_image_url(&self, url: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> 
    {
        ClipVisionEmbeddingModel::embed_image_url(self, url)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
    
    fn embed_image_base64(&self, base64_data: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> 
    {
        ClipVisionEmbeddingModel::embed_image_base64(self, base64_data)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
    
    fn batch_embed_images(&self, image_paths: Vec<&str>) 
        -> std::result::Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> 
    {
        ClipVisionEmbeddingModel::batch_embed_images(self, image_paths)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
    
    fn embedding_dimension(&self) -> usize {
        self.dimension
    }
    
    fn supported_dimensions(&self) -> Vec<usize> {
        vec![512, 768]
    }
}
