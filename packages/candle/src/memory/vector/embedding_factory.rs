//! Embedding Factory Implementation
//!
//! This module provides a factory pattern that instantiates the correct embedding provider 
//! based on EmbeddingConfig with comprehensive error handling and validation.

use std::sync::Arc;

use crate::domain::embedding::config::EmbeddingConfig;
use crate::memory::utils::error::{Error as MemoryError, Result};
use crate::memory::vector::embedding_model::EmbeddingModel;
use crate::providers::{
    bert_embedding::CandleBertEmbeddingProvider,
    clip_vision_embedding::ClipVisionEmbeddingProvider,
    gte_qwen_embedding::CandleGteQwenEmbeddingProvider,
    jina_bert_embedding::CandleJinaBertEmbeddingProvider,
    nvembed_embedding::CandleNvEmbedEmbeddingProvider,
    stella_embedding::StellaEmbeddingProvider,
};

/// Factory for creating embedding models based on configuration
pub struct EmbeddingModelFactory;

impl EmbeddingModelFactory {
    /// Create an embedding model based on the provided configuration
    pub async fn create_embedding_model(config: EmbeddingConfig) -> Result<Arc<dyn EmbeddingModel>> {
        // Validate configuration before attempting to create model
        Self::validate_config(&config)?;
        
        // Determine model type from config.model field
        let model_name = config.model.as_deref().unwrap_or("stella");
        
        match Self::normalize_model_name(model_name) {
            "bert" | "sentence-transformers" => {
                // BERT only supports 384 dimensions - configuration is ignored but validated
                let provider = CandleBertEmbeddingProvider::new().await
                    .map_err(|e| MemoryError::ModelError(format!("Failed to create BERT provider: {}", e)))?;
                Ok(Arc::new(provider) as Arc<dyn EmbeddingModel>)
            },
            
            "stella" => {
                // Stella supports configurable dimensions via MRL framework
                if let Some(dims) = config.dimensions {
                    let stella_config = crate::providers::stella_embedding::StellaConfig::new_1_5b(dims, candle_core::Device::Cpu)
                        .map_err(|e| MemoryError::ModelError(format!("Failed to create Stella config with {}D: {}", dims, e)))?;
                    let provider = StellaEmbeddingProvider::with_config(stella_config).await
                        .map_err(|e| MemoryError::ModelError(format!("Failed to create Stella provider: {}", e)))?;
                    Ok(Arc::new(provider) as Arc<dyn EmbeddingModel>)
                } else {
                    // Use default 1024D configuration
                    let provider = StellaEmbeddingProvider::new().await
                        .map_err(|e| MemoryError::ModelError(format!("Failed to create Stella provider: {}", e)))?;
                    Ok(Arc::new(provider) as Arc<dyn EmbeddingModel>)
                }
            },
            
            "gte-qwen" | "gte-qwen2" => {
                // GTE-Qwen only supports 1536 dimensions - configuration is ignored but validated
                let provider = CandleGteQwenEmbeddingProvider::new().await
                    .map_err(|e| MemoryError::ModelError(format!("Failed to create GTE-Qwen provider: {}", e)))?;
                Ok(Arc::new(provider) as Arc<dyn EmbeddingModel>)
            },
            
            "jina-bert" | "jina" => {
                // Jina-BERT only supports 768 dimensions - configuration is ignored but validated
                let provider = CandleJinaBertEmbeddingProvider::new().await
                    .map_err(|e| MemoryError::ModelError(format!("Failed to create Jina-BERT provider: {}", e)))?;
                Ok(Arc::new(provider) as Arc<dyn EmbeddingModel>)
            },
            
            "nvembed" | "nv-embed-v2" | "nvidia/nv-embed-v2" => {
                // NVEmbed only supports 4096 dimensions - configuration is ignored but validated
                let provider = CandleNvEmbedEmbeddingProvider::new().await
                    .map_err(|e| MemoryError::ModelError(format!("Failed to create NVEmbed provider: {}", e)))?;
                Ok(Arc::new(provider) as Arc<dyn EmbeddingModel>)
            },
            
            "clip-vision" | "clip" => {
                // CLIP Vision supports 512D (ViT-Base) or 768D (ViT-Large)
                let dimension = config.dimensions.unwrap_or(512);  // Default to ViT-Base
                let provider = ClipVisionEmbeddingProvider::with_dimension(dimension).await
                    .map_err(|e| MemoryError::ModelError(format!("Failed to create CLIP Vision provider: {}", e)))?;
                Ok(Arc::new(provider) as Arc<dyn EmbeddingModel>)
            },
            
            _ => {
                Err(MemoryError::Config(format!(
                    "Unknown embedding model: '{}'. Supported models: bert, stella, gte-qwen, jina-bert, nvembed, clip-vision", 
                    model_name
                )))
            }
        }
    }
    
    /// Validate that a configuration is supported and properly formed
    pub fn validate_config(config: &EmbeddingConfig) -> Result<()> {
        // Check batch size is reasonable
        if config.batch_size == 0 {
            return Err(MemoryError::Config("Batch size cannot be zero".to_string()));
        }
        
        if config.batch_size > 1000 {
            return Err(MemoryError::Config("Batch size too large (max 1000)".to_string()));
        }
        
        // Validate dimensions against model-specific native support
        if let Some(dims) = config.dimensions {
            let model_name = config.model.as_deref().unwrap_or("stella");
            Self::validate_dimension_for_model(dims, model_name)?;
        }
        
        Ok(())
    }

    /// Validate dimension is supported by the specific model
    fn validate_dimension_for_model(dimension: usize, model_name: &str) -> Result<()> {
        let normalized_name = Self::normalize_model_name(model_name);
        
        match normalized_name {
            "bert" | "sentence-transformers" => {
                if dimension != 384 {
                    return Err(MemoryError::Config(format!(
                        "BERT (sentence-transformers/all-MiniLM-L6-v2) only supports 384 dimensions. Requested: {}",
                        dimension
                    )));
                }
            },
            "stella" => {
                match dimension {
                    256 | 768 | 1024 | 2048 | 4096 | 6144 | 8192 => {},
                    _ => return Err(MemoryError::Config(format!(
                        "Stella natively supports: 256, 768, 1024, 2048, 4096, 6144, 8192 dimensions. Requested: {}",
                        dimension
                    ))),
                }
            },
            "gte-qwen" | "gte-qwen2" => {
                if dimension != 1536 {
                    return Err(MemoryError::Config(format!(
                        "GTE-Qwen2-1.5B-instruct only supports 1536 dimensions. Requested: {}",
                        dimension
                    )));
                }
            },
            "jina-bert" | "jina" => {
                if dimension != 768 {
                    return Err(MemoryError::Config(format!(
                        "Jina-BERT only supports 768 dimensions. Requested: {}",
                        dimension
                    )));
                }
            },
            "nvembed" | "nv-embed-v2" | "nvidia/nv-embed-v2" => {
                if dimension != 4096 {
                    return Err(MemoryError::Config(format!(
                        "NVEmbed-v2 only supports 4096 dimensions. Requested: {}",
                        dimension
                    )));
                }
            },
            "clip-vision" | "clip" => {
                if dimension != 512 && dimension != 768 {
                    return Err(MemoryError::Config(format!(
                        "CLIP Vision supports 512 dimensions (ViT-Base-Patch32) or 768 dimensions (ViT-Large-Patch14). Requested: {}",
                        dimension
                    )));
                }
            },
            _ => {
                return Err(MemoryError::Config(format!(
                    "Unknown model '{}' for dimension validation",
                    model_name
                )));
            }
        }
        
        Ok(())
    }
    
    /// Get model information without instantiating the model
    pub fn get_model_info(config: &EmbeddingConfig) -> ModelInfo {
        let model_name = config.model.as_deref().unwrap_or("bert");
        let normalized_name = Self::normalize_model_name(model_name);
        
        ModelInfo {
            model_name: normalized_name.to_string(),
            dimensions: match normalized_name {
                "bert" => vec![384], // BERT all-MiniLM-L6-v2 native dimension only
                "stella" => vec![256, 768, 1024, 2048, 4096, 6144, 8192], // Stella MRL framework supported dimensions
                "gte-qwen" => vec![1536], // GTE-Qwen2 1.5B native dimension only
                "jina-bert" => vec![768], // Jina-BERT v2 native dimension only
                "nvembed" => vec![4096], // NVEmbed v2 native dimension only
                "clip-vision" => vec![512, 768], // CLIP ViT-Base (512) and ViT-Large (768)
                _ => vec![],
            },
            default_model_path: match normalized_name {
                "bert" => "sentence-transformers/all-MiniLM-L6-v2",
                "stella" => "dunzhang/stella_en_1.5B_v5",
                "gte-qwen" => "Alibaba-NLP/gte-Qwen2-1.5B-instruct",
                "jina-bert" => "jinaai/jina-embeddings-v2-base-en",
                "nvembed" => "nvidia/NV-Embed-v2",
                "clip-vision" => "openai/clip-vit-base-patch32",
                _ => "unknown",
            }.to_string(),
            supports_custom_path: true,
            recommended_device: match normalized_name {
                "stella" => "GPU", // Large model benefits from GPU
                "clip-vision" => "GPU/CPU", // Vision models work on both
                _ => "GPU/CPU", // Others work well on both
            }.to_string(),
        }
    }
    
    /// Create embedding model with automatic fallback on failure
    pub async fn create_with_fallback(
        primary_config: EmbeddingConfig,
        fallback_config: Option<EmbeddingConfig>,
    ) -> Result<Arc<dyn EmbeddingModel>> {
        // Try primary configuration first
        match Self::create_embedding_model(primary_config.clone()).await {
            Ok(model) => Ok(model),
            Err(primary_error) => {
                if let Some(fallback) = fallback_config {
                    match Self::create_embedding_model(fallback).await {
                        Ok(model) => {
                            tracing::warn!(
                                "Primary embedding model failed ({}), using fallback", 
                                primary_error
                            );
                            Ok(model)
                        },
                        Err(fallback_error) => {
                            Err(MemoryError::ModelError(format!(
                                "Both primary and fallback embedding models failed. Primary: {}. Fallback: {}",
                                primary_error, fallback_error
                            )))
                        }
                    }
                } else {
                    Err(primary_error)
                }
            }
        }
    }
    
    /// Create default BERT embedding model for backward compatibility
    pub async fn create_default_bert() -> Result<Arc<dyn EmbeddingModel>> {
        Self::create_embedding_model(EmbeddingConfig::default().with_model("bert")).await
    }
    
    /// Create Stella model with specific dimensions
    pub async fn create_stella(dimensions: Option<usize>) -> Result<Arc<dyn EmbeddingModel>> {
        let mut config = EmbeddingConfig::default().with_model("stella");
        if let Some(dims) = dimensions {
            config = config.with_dimensions(dims);
        }
        Self::create_embedding_model(config).await
    }
    
    /// Create NVEmbed model for 4096-dimensional embeddings
    pub async fn create_nvembed() -> Result<Arc<dyn EmbeddingModel>> {
        Self::create_embedding_model(EmbeddingConfig::default().with_model("nvembed")).await
    }
    
    /// Create GTE-Qwen model for 1536-dimensional embeddings
    pub async fn create_gte_qwen() -> Result<Arc<dyn EmbeddingModel>> {
        Self::create_embedding_model(EmbeddingConfig::default().with_model("gte-qwen")).await
    }
    
    /// Create Jina-BERT model for 768-dimensional embeddings
    pub async fn create_jina_bert() -> Result<Arc<dyn EmbeddingModel>> {
        Self::create_embedding_model(EmbeddingConfig::default().with_model("jina-bert")).await
    }
    
    /// Create model from string configuration for external APIs
    pub async fn create_from_string(model_type: &str, model_path: Option<String>) -> Result<Arc<dyn EmbeddingModel>> {
        let mut config = EmbeddingConfig::default().with_model(model_type);
        
        // Add model path as additional parameter if provided
        if let Some(path) = model_path {
            config = config.with_param("model_path", path);
        }
        
        Self::create_embedding_model(config).await
    }
    
    /// Normalize model name for consistent matching
    fn normalize_model_name(model_name: &str) -> &str {
        let lower = model_name.to_lowercase();
        match lower.as_str() {
            // BERT variants
            "bert" | "sentence-transformers" | "all-minilm-l6-v2" => "bert",
            
            // Stella variants  
            "stella" | "stella_en_1.5b_v5" | "dunzhang/stella_en_1.5b_v5" => "stella",
            
            // GTE-Qwen variants
            "gte-qwen" | "gte-qwen2" | "gte-qwen2-1.5b-instruct" | "alibaba-nlp/gte-qwen2-1.5b-instruct" => "gte-qwen",
            
            // Jina-BERT variants
            "jina-bert" | "jina" | "jina-embeddings-v2-base-en" | "jinaai/jina-embeddings-v2-base-en" => "jina-bert",
            
            // NVEmbed variants
            "nvembed" | "nv-embed-v2" | "nvidia/nv-embed-v2" => "nvembed",
            
            // CLIP Vision variants
            "clip-vision" | "clip" | "clip-vit-base-patch32" | "clip-vit-large-patch14" | "openai/clip-vit-base-patch32" | "openai/clip-vit-large-patch14-336" => "clip-vision",
            
            // Default fallback
            _ => model_name,
        }
    }
}

/// Information about an embedding model configuration
#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub model_name: String,
    pub dimensions: Vec<usize>,
    pub default_model_path: String,
    pub supports_custom_path: bool,
    pub recommended_device: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::embedding::config::EmbeddingConfig;

    #[test]
    fn test_dimension_validation_rejects_artificial_projections() {
        // Test that we properly reject dimensions not natively supported
        
        // BERT should reject anything other than 384
        assert!(EmbeddingModelFactory::validate_dimension_for_model(512, "bert").is_err());
        assert!(EmbeddingModelFactory::validate_dimension_for_model(1024, "bert").is_err());
        assert!(EmbeddingModelFactory::validate_dimension_for_model(384, "bert").is_ok());
        
        // Stella should reject 512 (the old artificial projection)
        assert!(EmbeddingModelFactory::validate_dimension_for_model(512, "stella").is_err());
        assert!(EmbeddingModelFactory::validate_dimension_for_model(1024, "stella").is_ok());
        assert!(EmbeddingModelFactory::validate_dimension_for_model(256, "stella").is_ok());
        
        // GTE-Qwen should only accept 1536
        assert!(EmbeddingModelFactory::validate_dimension_for_model(512, "gte-qwen").is_err());
        assert!(EmbeddingModelFactory::validate_dimension_for_model(1024, "gte-qwen").is_err());
        assert!(EmbeddingModelFactory::validate_dimension_for_model(1536, "gte-qwen").is_ok());
        
        // Jina-BERT should only accept 768
        assert!(EmbeddingModelFactory::validate_dimension_for_model(512, "jina-bert").is_err());
        assert!(EmbeddingModelFactory::validate_dimension_for_model(384, "jina-bert").is_err());
        assert!(EmbeddingModelFactory::validate_dimension_for_model(768, "jina-bert").is_ok());
        
        // NVEmbed should only accept 4096
        assert!(EmbeddingModelFactory::validate_dimension_for_model(512, "nvembed").is_err());
        assert!(EmbeddingModelFactory::validate_dimension_for_model(1024, "nvembed").is_err());
        assert!(EmbeddingModelFactory::validate_dimension_for_model(4096, "nvembed").is_ok());
    }

    #[test]
    fn test_model_info_reflects_native_dimensions_only() {
        // Test that ModelInfo correctly reflects only native dimensions
        
        let bert_config = EmbeddingConfig::default().with_model("bert");
        let bert_info = EmbeddingModelFactory::get_model_info(&bert_config);
        assert_eq!(bert_info.dimensions, vec![384]);
        
        let stella_config = EmbeddingConfig::default().with_model("stella");
        let stella_info = EmbeddingModelFactory::get_model_info(&stella_config);
        assert_eq!(stella_info.dimensions, vec![256, 768, 1024, 2048, 4096, 6144, 8192]);
        // Importantly, 512 should NOT be in this list anymore
        assert!(!stella_info.dimensions.contains(&512));
        
        let gte_config = EmbeddingConfig::default().with_model("gte-qwen");
        let gte_info = EmbeddingModelFactory::get_model_info(&gte_config);
        assert_eq!(gte_info.dimensions, vec![1536]);
        
        let jina_config = EmbeddingConfig::default().with_model("jina-bert");
        let jina_info = EmbeddingModelFactory::get_model_info(&jina_config);
        assert_eq!(jina_info.dimensions, vec![768]);
        
        let nvembed_config = EmbeddingConfig::default().with_model("nvembed");
        let nvembed_info = EmbeddingModelFactory::get_model_info(&nvembed_config);
        assert_eq!(nvembed_info.dimensions, vec![4096]);
    }

    #[test]
    fn test_config_validation_enforces_native_dimensions() {
        // Test that validate_config properly rejects artificial dimensions
        
        // Should reject 512D for any model (the old artificial projection size)
        let bert_512_config = EmbeddingConfig::default().with_model("bert").with_dimensions(512);
        assert!(EmbeddingModelFactory::validate_config(&bert_512_config).is_err());
        
        let stella_512_config = EmbeddingConfig::default().with_model("stella").with_dimensions(512);
        assert!(EmbeddingModelFactory::validate_config(&stella_512_config).is_err());
        
        // Should accept native dimensions
        let bert_384_config = EmbeddingConfig::default().with_model("bert").with_dimensions(384);
        assert!(EmbeddingModelFactory::validate_config(&bert_384_config).is_ok());
        
        let stella_1024_config = EmbeddingConfig::default().with_model("stella").with_dimensions(1024);
        assert!(EmbeddingModelFactory::validate_config(&stella_1024_config).is_ok());
    }
}