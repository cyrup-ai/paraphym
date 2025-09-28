//! Stella embedding provider for local inference using Candle ML framework
//!
//! This provider uses dunzhang/stella_en_400M_v5 or dunzhang/stella_en_1.5B_v5 models 
//! for generating configurable-dimensional embeddings with ProgressHub download and Candle inference.

use std::collections::HashMap;
use std::sync::Mutex;

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::stella_en_v5::{Config, EmbedDim, EmbeddingModel, ModelVariant};
use progresshub::{ProgressHub, types::ZeroOneOrMany as ProgressHubZeroOneOrMany};
use tokenizers::{Tokenizer, PaddingParams, PaddingDirection, PaddingStrategy, TruncationParams};

use crate::memory::utils::error::{Error as MemoryError, Result};
use crate::memory::vector::embedding_model::EmbeddingModel as EmbeddingModelTrait;

/// Configuration for Stella embedding model
#[derive(Debug, Clone)]
pub struct StellaConfig {
    /// Maximum sequence length for tokenization
    pub max_length: usize,
    /// Model dimension (configurable: 256/768/1024/2048/4096/6144/8192)
    pub dimension: usize,
    /// Model variant (400M or 1.5B)
    pub variant: ModelVariant,
    /// Data type for inference
    pub dtype: DType,
    /// Device for inference
    pub device: Device,
}

impl StellaConfig {
    /// Create config for 400M model with specified dimension
    pub fn new_400m(dimension: usize, device: Device) -> Result<Self> {
        Self::validate_dimension(dimension)?;
        Ok(Self {
            max_length: 8192,
            dimension,
            variant: ModelVariant::Small,
            dtype: DType::F32,
            device,
        })
    }

    /// Create config for 1.5B model with specified dimension
    pub fn new_1_5b(dimension: usize, device: Device) -> Result<Self> {
        Self::validate_dimension(dimension)?;
        Ok(Self {
            max_length: 131072,
            dimension,
            variant: ModelVariant::Large,
            dtype: DType::F32,
            device,
        })
    }

    /// Validate dimension is supported
    fn validate_dimension(dimension: usize) -> Result<()> {
        match dimension {
            256 | 768 | 1024 | 2048 | 4096 | 6144 | 8192 => Ok(()),
            _ => Err(MemoryError::Config(format!(
                "Unsupported dimension: {}. Supported: 256, 768, 1024, 2048, 4096, 6144, 8192",
                dimension
            ))),
        }
    }

    /// Convert dimension to EmbedDim enum
    fn embed_dim(&self) -> EmbedDim {
        match self.dimension {
            256 => EmbedDim::Dim256,
            768 => EmbedDim::Dim768,
            1024 => EmbedDim::Dim1024,
            2048 => EmbedDim::Dim2048,
            4096 => EmbedDim::Dim4096,
            6144 => EmbedDim::Dim6144,
            8192 => EmbedDim::Dim8192,
            _ => EmbedDim::Dim1024, // fallback
        }
    }

    /// Get embedding head directory name for this dimension
    fn embed_head_dir(&self) -> &'static str {
        match self.dimension {
            256 => "2_Dense_256",
            768 => "2_Dense_768", 
            1024 => "2_Dense_1024",
            2048 => "2_Dense_2048",
            4096 => "2_Dense_4096",
            6144 => "2_Dense_6144",
            8192 => "2_Dense_8192",
            _ => "2_Dense_1024", // fallback
        }
    }

    /// Get model repository name
    fn repo_name(&self) -> &'static str {
        match self.variant {
            ModelVariant::Large => "dunzhang/stella_en_1.5B_v5",
            ModelVariant::Small => "dunzhang/stella_en_400M_v5",
        }
    }
}

impl Default for StellaConfig {
    fn default() -> Self {
        Self {
            max_length: 8192,
            dimension: 1024,
            variant: ModelVariant::Small,
            dtype: DType::F32,
            device: Device::Cpu,
        }
    }
}

/// Stella embedding provider using Candle ML framework
///
/// Provides high-performance local embeddings using dunzhang/stella models
/// with automatic model download via ProgressHub and configurable output dimensions.
pub struct StellaEmbeddingProvider {
    /// Model cache directory path
    model_path: String,
    /// Model configuration
    config: StellaConfig,
    /// Loaded Stella model (thread-safe)
    model: Mutex<EmbeddingModel>,
    /// Tokenizer for text processing
    tokenizer: Tokenizer,
    /// Device for inference
    device: Device,
}

impl std::fmt::Debug for StellaEmbeddingProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StellaEmbeddingProvider")
            .field("model_path", &self.model_path)
            .field("config", &self.config)
            .field("model", &"EmbeddingModel { .. }")
            .field("tokenizer", &"Tokenizer { .. }")
            .field("device", &format!("{:?}", self.device))
            .finish()
    }
}

impl StellaEmbeddingProvider {
    /// Create new Stella embedding provider with 1024-dimensional embeddings (400M model)
    pub async fn new() -> Result<Self> {
        let config = StellaConfig::default();
        Self::with_config(config).await
    }

    /// Create provider with custom configuration
    pub async fn with_config(config: StellaConfig) -> Result<Self> {
        // Download model using ProgressHub
        let results = ProgressHub::builder()
            .model(config.repo_name())
            .with_cli_progress()
            .download()
            .await
            .map_err(|e| MemoryError::ModelError(format!("ProgressHub download failed: {}", e)))?;

        // Extract the model path from download results
        let model_cache_dir = if let Some(result) = results.into_iter().next() {
            match &result.models {
                ProgressHubZeroOneOrMany::One(model) => {
                    model.model_cache_path.display().to_string()
                }
                ProgressHubZeroOneOrMany::Zero => {
                    return Err(MemoryError::ModelError("No models were downloaded".to_string()));
                }
                ProgressHubZeroOneOrMany::Many(_) => {
                    return Err(MemoryError::ModelError("Expected exactly one model, got multiple".to_string()));
                }
            }
        } else {
            return Err(MemoryError::ModelError("No download results returned".to_string()));
        };

        Self::with_config_and_path(config, model_cache_dir).await
    }

    /// Create provider with custom configuration and existing model path
    pub async fn with_config_and_path(config: StellaConfig, model_path: String) -> Result<Self> {
        // Load tokenizer from model directory
        let tokenizer_path = format!("{}/tokenizer.json", model_path);
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| MemoryError::ModelError(format!("Failed to load tokenizer: {}", e)))?;

        // Configure tokenizer based on model variant (following example pattern)
        match config.variant {
            ModelVariant::Large => {
                // 1.5B model uses left padding with <|endoftext|> token
                let pad_id = tokenizer.token_to_id("<|endoftext|>")
                    .ok_or_else(|| MemoryError::ModelError("Tokenizer missing <|endoftext|> token".to_string()))?;
                
                let padding_params = PaddingParams {
                    strategy: PaddingStrategy::BatchLongest,
                    direction: PaddingDirection::Left,
                    pad_to_multiple_of: None,
                    pad_id,
                    pad_type_id: 0,
                    pad_token: "<|endoftext|>".to_string(),
                };
                tokenizer.with_padding(Some(padding_params));
            }
            ModelVariant::Small => {
                // 400M model uses right padding
                tokenizer.with_padding(Some(PaddingParams {
                    strategy: PaddingStrategy::BatchLongest,
                    direction: PaddingDirection::Right,
                    ..Default::default()
                }));
            }
        }

        // Set truncation
        if tokenizer.get_truncation().is_none() {
            let truncation_params = TruncationParams {
                max_length: config.max_length,
                strategy: tokenizers::TruncationStrategy::LongestFirst,
                stride: 0,
                direction: tokenizers::TruncationDirection::Right,
            };
            tokenizer.with_truncation(Some(truncation_params)).map_err(|e| 
                MemoryError::ModelError(format!("Failed to set tokenizer truncation: {}", e)))?;
        }

        // Load Stella model configuration
        let stella_config = match config.variant {
            ModelVariant::Large => Config::new_1_5_b_v5(config.embed_dim()),
            ModelVariant::Small => Config::new_400_m_v5(config.embed_dim()),
        };

        // Load base model weights
        let base_weights_path = format!("{}/model.safetensors", model_path);
        let base_vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[base_weights_path], config.dtype, &config.device)
                .map_err(|e| MemoryError::ModelError(format!("Failed to load base model weights: {}", e)))?
        };

        // Load embedding head weights  
        let embed_head_path = format!("{}/{}/model.safetensors", model_path, config.embed_head_dir());
        let embed_vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[embed_head_path], DType::F32, &config.device)
                .map_err(|e| MemoryError::ModelError(format!("Failed to load embedding head weights: {}", e)))?
        };

        // Create Stella model
        let model = EmbeddingModel::new(&stella_config, base_vb, embed_vb)
            .map_err(|e| MemoryError::ModelError(format!("Failed to create Stella model: {}", e)))?;

        let device = config.device.clone();
        Ok(Self {
            model_path,
            config,
            model: Mutex::new(model),
            tokenizer,
            device,
        })
    }

    /// Process text through Stella model
    fn forward_pass(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
        // Tokenize texts
        let tokens = self.tokenizer
            .encode_batch(texts.to_vec(), true)
            .map_err(|e| MemoryError::ModelError(format!("Tokenization failed: {}", e)))?;

        if tokens.is_empty() {
            return Ok(vec![]);
        }

        // Create input tensors
        
        let mut input_ids = Vec::new();
        let mut attention_masks = Vec::new();

        for token in &tokens {
            let ids = Tensor::new(token.get_ids(), &self.device)
                .map_err(|e| MemoryError::ModelError(format!("Failed to create input tensor: {}", e)))?;
            let mask = Tensor::new(token.get_attention_mask(), &self.device)
                .map_err(|e| MemoryError::ModelError(format!("Failed to create mask tensor: {}", e)))?
                .to_dtype(DType::U8)
                .map_err(|e| MemoryError::ModelError(format!("Failed to convert mask dtype: {}", e)))?;
            
            input_ids.push(ids);
            attention_masks.push(mask);
        }

        // Stack tensors into batches
        let input_ids = Tensor::stack(&input_ids, 0)
            .map_err(|e| MemoryError::ModelError(format!("Failed to stack input_ids: {}", e)))?;
        let attention_mask = Tensor::stack(&attention_masks, 0)
            .map_err(|e| MemoryError::ModelError(format!("Failed to stack attention_mask: {}", e)))?;

        // Forward pass with thread-safe model access
        let mut model = self.model.lock()
            .map_err(|e| MemoryError::ModelError(format!("Failed to acquire model lock: {}", e)))?;
        
        let embeddings = model.forward_norm(&input_ids, &attention_mask)
            .map_err(|e| MemoryError::ModelError(format!("Stella forward pass failed: {}", e)))?;

        // Convert to Vec<Vec<f32>>
        let embeddings_data = embeddings.to_vec2::<f32>()
            .map_err(|e| MemoryError::ModelError(format!("Failed to convert embeddings to vec: {}", e)))?;

        Ok(embeddings_data)
    }
}

impl EmbeddingModelTrait for StellaEmbeddingProvider {
    fn embed(&self, text: &str, _task: Option<String>) -> Result<Vec<f32>> {
        self.validate_input(text)?;
        
        let embeddings = self.forward_pass(&[text])?;
        embeddings.into_iter().next()
            .ok_or_else(|| MemoryError::ModelError("No embeddings generated".to_string()))
    }

    fn batch_embed(&self, texts: &[String], _task: Option<String>) -> Result<Vec<Vec<f32>>> {
        self.validate_batch(texts)?;
        
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        self.forward_pass(&text_refs)
    }

    fn dimension(&self) -> usize {
        self.config.dimension
    }

    fn name(&self) -> &str {
        match self.config.variant {
            ModelVariant::Large => "stella-1.5B-embedding",
            ModelVariant::Small => "stella-400M-embedding",
        }
    }

    fn config_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("name".to_string(), self.name().to_string());
        info.insert("dimension".to_string(), self.dimension().to_string());
        info.insert("model".to_string(), self.config.repo_name().to_string());
        info.insert("variant".to_string(), format!("{:?}", self.config.variant));
        info.insert("max_length".to_string(), self.config.max_length.to_string());
        info.insert("device".to_string(), format!("{:?}", self.device));
        info
    }

    fn recommended_batch_size(&self) -> usize {
        match self.config.variant {
            ModelVariant::Large => 8,  // Larger model needs smaller batches
            ModelVariant::Small => 16, // Smaller model can handle larger batches
        }
    }

    fn max_batch_size(&self) -> usize {
        match self.config.variant {
            ModelVariant::Large => 32,
            ModelVariant::Small => 64,
        }
    }

    fn health_check(&self) -> Result<()> {
        // Verify model is loaded and ready
        let test_embedding = self.embed("test", None)?;
        if test_embedding.len() != self.dimension() {
            return Err(MemoryError::ModelError(
                format!("Health check failed: expected {} dimensions, got {}", 
                        self.dimension(), test_embedding.len())
            ));
        }
        Ok(())
    }
}