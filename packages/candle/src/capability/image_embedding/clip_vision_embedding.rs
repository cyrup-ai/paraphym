//! CLIP Vision Embedding Provider - Sync Wrapper for EmbeddingModel Trait
//!
//! This module provides a synchronous wrapper around ClipVisionProvider to enable
//! integration with the EmbeddingModel trait system and EmbeddingModelFactory.

use std::collections::HashMap;
use std::sync::Arc;

use candle_core::Device;

use crate::core::device_util::detect_best_device;
use crate::memory::utils::error::{Error as MemoryError, Result};
use crate::memory::vector::embedding_model::EmbeddingModel;
use crate::capability::vision::ClipVisionProvider;

/// Synchronous wrapper for ClipVisionProvider implementing EmbeddingModel trait
///
/// This adapter bridges the async ClipVisionProvider with the sync EmbeddingModel trait
/// by using tokio runtime to execute async operations synchronously.
pub struct ClipVisionEmbeddingProvider {
    provider: Arc<ClipVisionProvider>,
    dimension: usize,
}

impl std::fmt::Debug for ClipVisionEmbeddingProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClipVisionEmbeddingProvider")
            .field("provider", &"ClipVisionProvider { .. }")
            .field("dimension", &self.dimension)
            .finish()
    }
}

impl ClipVisionEmbeddingProvider {
    /// Create new CLIP vision embedding provider with ViT-Base configuration (512D)
    pub async fn new() -> Result<Self> {
        Self::with_dimension(512).await
    }

    /// Create CLIP vision provider with specific dimension
    ///
    /// # Arguments
    /// * `dimension` - 512 for ViT-Base-Patch32 or 768 for ViT-Large-Patch14
    pub async fn with_dimension(dimension: usize) -> Result<Self> {
        // Determine model path based on dimension
        let model_path = match dimension {
            512 => "openai/clip-vit-base-patch32",  // ViT-Base: 512D
            768 => "openai/clip-vit-large-patch14-336",  // ViT-Large: 768D
            _ => {
                return Err(MemoryError::Config(format!(
                    "CLIP only supports 512D (ViT-Base) or 768D (ViT-Large). Requested: {}",
                    dimension
                )));
            }
        };

        // Create ClipVisionProvider
        let device = detect_best_device()
            .unwrap_or_else(|e| {
                log::warn!("Device detection failed: {}. Using CPU.", e);
                Device::Cpu
            });
        let provider = if dimension == 512 {
            ClipVisionProvider::from_pretrained(model_path, device)
                .map_err(|e| MemoryError::ModelError(format!("Failed to create CLIP provider: {}", e)))?
        } else {
            ClipVisionProvider::from_pretrained_large(model_path, device)
                .map_err(|e| MemoryError::ModelError(format!("Failed to create CLIP Large provider: {}", e)))?
        };

        Ok(Self {
            provider: Arc::new(provider),
            dimension,
        })
    }

    /// Create from existing ClipVisionProvider
    pub fn from_provider(provider: ClipVisionProvider, dimension: usize) -> Self {
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

impl EmbeddingModel for ClipVisionEmbeddingProvider {
    /// CLIP Vision only supports image encoding, not text
    ///
    /// This method returns an error to indicate that text embedding is not supported.
    /// Use `embed_image()` instead for encoding images.
    fn embed(&self, _text: &str, _task: Option<String>) -> Result<Vec<f32>> {
        Err(MemoryError::InvalidInput(
            "CLIP Vision only supports image encoding, not text. Use embed_image() instead.".to_string()
        ))
    }

    /// CLIP Vision does not support batch text embedding
    ///
    /// This method returns an error. Use `batch_embed_images()` for batch image encoding.
    fn batch_embed(&self, _texts: &[String], _task: Option<String>) -> Result<Vec<Vec<f32>>> {
        Err(MemoryError::InvalidInput(
            "CLIP Vision only supports image encoding, not text. Use batch_embed_images() instead.".to_string()
        ))
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn name(&self) -> &str {
        "clip-vision"
    }

    fn supported_dimensions(&self) -> Vec<usize> {
        vec![512, 768]  // ViT-Base-Patch32 (512) and ViT-Large-Patch14 (768)
    }

    fn config_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("name".to_string(), self.name().to_string());
        info.insert("dimension".to_string(), self.dimension().to_string());
        info.insert("type".to_string(), "vision".to_string());
        info.insert(
            "model".to_string(),
            if self.dimension == 512 {
                "openai/clip-vit-base-patch32".to_string()
            } else {
                "openai/clip-vit-large-patch14-336".to_string()
            }
        );
        info
    }

    fn recommended_batch_size(&self) -> usize {
        8  // Vision models are more memory-intensive
    }

    fn max_batch_size(&self) -> usize {
        32  // Conservative limit for vision models
    }

    fn health_check(&self) -> Result<()> {
        // Vision models don't need standard health check since they don't support text
        Ok(())
    }
}
