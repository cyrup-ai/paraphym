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
    gte_qwen_embedding::CandleGteQwenEmbeddingProvider,
    jina_bert_embedding::CandleJinaBertEmbeddingProvider,
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
        let model_name = config.model.as_deref().unwrap_or("bert");
        
        match Self::normalize_model_name(model_name) {
            "bert" | "sentence-transformers" => {
                let provider = CandleBertEmbeddingProvider::new().await
                    .map_err(|e| MemoryError::ModelError(format!("Failed to create BERT provider: {}", e)))?;
                Ok(Arc::new(provider) as Arc<dyn EmbeddingModel>)
            },
            
            "stella" => {
                let provider = StellaEmbeddingProvider::new().await
                    .map_err(|e| MemoryError::ModelError(format!("Failed to create Stella provider: {}", e)))?;
                Ok(Arc::new(provider) as Arc<dyn EmbeddingModel>)
            },
            
            "gte-qwen" | "gte-qwen2" => {
                let provider = CandleGteQwenEmbeddingProvider::new().await
                    .map_err(|e| MemoryError::ModelError(format!("Failed to create GTE-Qwen provider: {}", e)))?;
                Ok(Arc::new(provider) as Arc<dyn EmbeddingModel>)
            },
            
            "jina-bert" | "jina" => {
                let provider = CandleJinaBertEmbeddingProvider::new().await
                    .map_err(|e| MemoryError::ModelError(format!("Failed to create Jina-BERT provider: {}", e)))?;
                Ok(Arc::new(provider) as Arc<dyn EmbeddingModel>)
            },
            
            _ => {
                Err(MemoryError::Config(format!(
                    "Unknown embedding model: '{}'. Supported models: bert, stella, gte-qwen, jina-bert", 
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
        
        // Check dimensions if specified
        if let Some(dims) = config.dimensions {
            if dims == 0 {
                return Err(MemoryError::Config("Dimensions cannot be zero".to_string()));
            }
            if dims > 8192 {
                return Err(MemoryError::Config("Dimensions too large (max 8192)".to_string()));
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
                "bert" => 384,
                "stella" => 1024,
                "gte-qwen" => 1536,
                "jina-bert" => 768,
                _ => 0,
            },
            default_model_path: match normalized_name {
                "bert" => "sentence-transformers/all-MiniLM-L6-v2",
                "stella" => "dunzhang/stella_en_1.5B_v5",
                "gte-qwen" => "Alibaba-NLP/gte-Qwen2-1.5B-instruct",
                "jina-bert" => "jinaai/jina-embeddings-v2-base-en",
                _ => "unknown",
            }.to_string(),
            supports_custom_path: true,
            recommended_device: match normalized_name {
                "stella" => "GPU", // Large model benefits from GPU
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
            
            // Default fallback
            _ => model_name,
        }
    }
}

/// Information about an embedding model configuration
#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub model_name: String,
    pub dimensions: usize,
    pub default_model_path: String,
    pub supports_custom_path: bool,
    pub recommended_device: String,
}