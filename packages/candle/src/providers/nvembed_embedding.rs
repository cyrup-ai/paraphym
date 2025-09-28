//! NVEmbed v2 embedding provider for local inference using Candle ML framework
//!
//! This provider uses nvidia/NV-Embed-v2 model for generating
//! 4096-dimensional embeddings with Mistral decoder and latent attention.

use std::collections::HashMap;
use std::sync::Mutex;
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::nvembed_v2::model::Model as NvEmbedModel;
use progresshub::{ProgressHub, types::ZeroOneOrMany as ProgressHubZeroOneOrMany};
use tokenizers::{Tokenizer, PaddingParams, TruncationParams};
use crate::memory::utils::error::{Error as MemoryError, Result};
use crate::memory::vector::embedding_model::EmbeddingModel as EmbeddingModelTrait;

/// Configuration for NVEmbed v2 embedding model
#[derive(Debug, Clone)]
pub struct CandleNvEmbedConfig {
    pub embed_dim: u32,
    pub max_length: usize,
    pub dtype: DType,
    pub device: Device,
}

impl Default for CandleNvEmbedConfig {
    fn default() -> Self {
        Self {
            embed_dim: 4096,
            max_length: 32768, // NVEmbed v2 supports very long contexts
            dtype: DType::F32,
            device: Device::Cpu,
        }
    }
}

/// NVEmbed v2 embedding provider using Candle ML framework
pub struct CandleNvEmbedEmbeddingProvider {
    #[allow(dead_code)] // Used in path construction and config_info - false positive warning
    model_path: String,
    config: CandleNvEmbedConfig,
    model: Mutex<NvEmbedModel>,
    tokenizer: Tokenizer,
    device: Device,
}

impl std::fmt::Debug for CandleNvEmbedEmbeddingProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CandleNvEmbedEmbeddingProvider")
            .field("model_path", &self.model_path)
            .field("config", &self.config)
            .field("model", &"<NvEmbedModel>")
            .field("tokenizer", &"<Tokenizer>")
            .field("device", &self.device)
            .finish()
    }
}

impl CandleNvEmbedEmbeddingProvider {
    pub async fn new() -> Result<Self> {
        let config = CandleNvEmbedConfig::default();
        Self::with_config(config).await
    }

    pub async fn with_config(config: CandleNvEmbedConfig) -> Result<Self> {
        // Download model using ProgressHub
        let results = ProgressHub::builder()
            .model("nvidia/NV-Embed-v2")
            .with_cli_progress()
            .download()
            .await
            .map_err(|e| MemoryError::ModelError(format!("ProgressHub download failed: {}", e)))?;

        // Extract model path
        let model_cache_dir = if let Some(result) = results.into_iter().next() {
            match &result.models {
                ProgressHubZeroOneOrMany::One(model) => {
                    model.model_cache_path.display().to_string()
                }
                _ => return Err(MemoryError::ModelError("Invalid download result".to_string())),
            }
        } else {
            return Err(MemoryError::ModelError("No download results".to_string()));
        };

        Self::with_config_and_path(config, model_cache_dir).await
    }

    pub async fn with_config_and_path(config: CandleNvEmbedConfig, model_path: String) -> Result<Self> {
        // Load tokenizer
        let tokenizer_path = format!("{}/tokenizer.json", model_path);
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| MemoryError::ModelError(format!("Failed to load tokenizer: {}", e)))?;

        // Configure tokenizer for right padding with </s> token
        let eos_pad_id = 2; // </s> token ID for NVEmbed
        
        // Validate that the tokenizer actually has this token ID
        if tokenizer.token_to_id("</s>") != Some(eos_pad_id) {
            return Err(MemoryError::ModelError(format!(
                "Tokenizer EOS token mismatch: expected {}, got {:?}", 
                eos_pad_id, 
                tokenizer.token_to_id("</s>")
            )));
        }
        
        let padding_params = PaddingParams {
            strategy: tokenizers::PaddingStrategy::BatchLongest,
            direction: tokenizers::PaddingDirection::Right, // Right padding for NVEmbed
            pad_id: eos_pad_id,
            pad_token: "</s>".to_string(),
            ..Default::default()
        };
        tokenizer.with_padding(Some(padding_params));

        // Set truncation
        let truncation_params = TruncationParams {
            max_length: config.max_length,
            ..Default::default()
        };
        tokenizer.with_truncation(Some(truncation_params)).map_err(|e| {
            MemoryError::ModelError(format!("Failed to set truncation: {}", e))
        })?;

        // Load model weights using multiple safetensors files like other large models
        let index_path = format!("{}/model.safetensors.index.json", model_path);
        let index_content = std::fs::read_to_string(&index_path)
            .map_err(|e| MemoryError::ModelError(format!("Failed to read index: {}", e)))?;
        
        let index: serde_json::Value = serde_json::from_str(&index_content)
            .map_err(|e| MemoryError::ModelError(format!("Failed to parse index: {}", e)))?;

        // Collect all weight files
        let weight_map = index["weight_map"].as_object()
            .ok_or_else(|| MemoryError::ModelError("Missing weight_map in index".to_string()))?;
        
        let mut unique_files: std::collections::HashSet<String> = std::collections::HashSet::new();
        for filename in weight_map.values() {
            if let Some(filename_str) = filename.as_str() {
                unique_files.insert(filename_str.to_string());
            }
        }

        let weight_files: Vec<String> = unique_files
            .into_iter()
            .map(|f| format!("{}/{}", model_path, f))
            .collect();

        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&weight_files, config.dtype, &config.device)
                .map_err(|e| MemoryError::ModelError(format!("Failed to load weights: {}", e)))?
        };

        // Create real NVEmbed model
        let model = NvEmbedModel::new(vb)
            .map_err(|e| MemoryError::ModelError(format!("Failed to create model: {}", e)))?;
        
        // Validate that our config embedding dimension matches NVEmbed v2's expected 4096
        if config.embed_dim != 4096 {
            return Err(MemoryError::ModelError(format!(
                "NVEmbed v2 embedding dimension mismatch: expected 4096, got {}", 
                config.embed_dim
            )));
        }

        let device = config.device.clone();
        Ok(Self {
            model_path,
            config,
            model: Mutex::new(model),
            tokenizer,
            device,
        })
    }

    fn forward_pass(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
        // Tokenize texts for NVEmbed (no special instruction formatting needed)
        let tokens = self.tokenizer
            .encode_batch(texts.to_vec(), true)
            .map_err(|e| MemoryError::ModelError(format!("Tokenization failed: {}", e)))?;

        let token_ids = tokens.iter().map(|tokens| {
            let tokens = tokens.get_ids().to_vec();
            Tensor::new(tokens.as_slice(), &self.device)
                .map_err(|e| MemoryError::ModelError(format!("Tensor creation failed: {}", e)))
        }).collect::<Result<Vec<_>>>()?;

        let attention_mask = tokens.iter().map(|tokens| {
            let tokens = tokens.get_attention_mask().to_vec();
            Tensor::new(tokens.as_slice(), &self.device)
                .map_err(|e| MemoryError::ModelError(format!("Tensor creation failed: {}", e)))
        }).collect::<Result<Vec<_>>>()?;

        let token_ids = Tensor::stack(&token_ids, 0)
            .map_err(|e| MemoryError::ModelError(format!("Token IDs tensor stack failed: {}", e)))?;
        let attention_mask = Tensor::stack(&attention_mask, 0)
            .map_err(|e| MemoryError::ModelError(format!("Attention mask tensor stack failed: {}", e)))?;

        // Create pool_mask (same as attention_mask for mean pooling)
        let pool_mask = attention_mask.clone();

        // Forward pass using real NVEmbed API
        let mut model = self.model.lock()
            .map_err(|e| MemoryError::ModelError(format!("Failed to acquire model lock: {}", e)))?;
        let embeddings = model.forward(&token_ids, &attention_mask, &pool_mask)
            .map_err(|e| MemoryError::ModelError(format!("Forward pass failed: {}", e)))?;

        let embeddings_data = embeddings.to_vec2::<f32>()
            .map_err(|e| MemoryError::ModelError(format!("Failed to convert embeddings: {}", e)))?;

        Ok(embeddings_data)
    }
}

impl EmbeddingModelTrait for CandleNvEmbedEmbeddingProvider {
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
        self.config.embed_dim as usize
    }

    fn name(&self) -> &str {
        "nvembed-v2-embedding"
    }

    fn config_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("name".to_string(), self.name().to_string());
        info.insert("dimension".to_string(), self.dimension().to_string());
        info.insert("model".to_string(), "nvidia/NV-Embed-v2".to_string());
        info.insert("embed_dim".to_string(), self.config.embed_dim.to_string());
        info.insert("max_length".to_string(), self.config.max_length.to_string());
        info.insert("dtype".to_string(), format!("{:?}", self.config.dtype));
        info.insert("device".to_string(), format!("{:?}", self.device));
        info.insert("padding".to_string(), "right".to_string());
        info.insert("architecture".to_string(), "bert-based".to_string());
        info
    }

    fn recommended_batch_size(&self) -> usize {
        2 // Very conservative for large 4096-dim model
    }

    fn max_batch_size(&self) -> usize {
        8
    }
}