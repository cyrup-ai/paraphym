//! Jina-BERT embedding provider for local inference using Candle ML framework
//!
//! This provider uses jinaai/jina-embeddings-v2-base-en model for generating
//! 768-dimensional embeddings with ALiBi positional embeddings and mean pooling.

use std::collections::HashMap;
use std::sync::Mutex;
use candle_core::{DType, Device, Tensor, Module};
use candle_nn::VarBuilder;
use candle_transformers::models::jina_bert::{BertModel, Config, PositionEmbeddingType};
use tokenizers::{Tokenizer, PaddingParams, TruncationParams};
use crate::memory::utils::error::{Error as MemoryError, Result};
use crate::memory::vector::embedding_model::EmbeddingModel as EmbeddingModelTrait;

/// Configuration for Jina-BERT embedding model
#[derive(Debug, Clone)]
pub struct CandleJinaBertConfig {
    pub embed_dim: u32,
    pub max_length: usize,
    pub dtype: DType,
    pub device: Device,
}

impl Default for CandleJinaBertConfig {
    fn default() -> Self {
        Self {
            embed_dim: 768,
            max_length: 8192,
            dtype: DType::F32,
            device: Device::Cpu,
        }
    }
}

/// Jina-BERT embedding provider using Candle ML framework
#[derive(Debug)]
pub struct CandleJinaBertEmbeddingProvider {
    #[allow(dead_code)] // Used in path construction and config_info - false positive warning
    model_path: String,
    config: CandleJinaBertConfig,
    model: Mutex<BertModel>,
    tokenizer: Tokenizer,
    device: Device,
}

impl CandleJinaBertEmbeddingProvider {
    pub async fn new() -> Result<Self> {
        let config = CandleJinaBertConfig::default();
        Self::with_config(config).await
    }

    pub async fn with_config(config: CandleJinaBertConfig) -> Result<Self> {
        use crate::domain::model::download::DownloadProviderFactory;
        
        // Use factory to get download provider (works with both backends)
        let downloader = DownloadProviderFactory::create_default()
            .map_err(|e| MemoryError::ModelError(format!("Failed to create download provider: {}", e)))?;
        
        // Download model files
        let result = downloader.download_model(
            "jinaai/jina-embeddings-v2-base-en",
            vec!["model.safetensors".to_string(), "tokenizer.json".to_string(), "config.json".to_string()],
            None,
        ).await
        .map_err(|e| MemoryError::ModelError(format!("Model download failed: {}", e)))?;
        
        // Use result.cache_dir for model path
        Self::with_config_and_path(
            config,
            result.cache_dir.to_str()
                .ok_or_else(|| MemoryError::ModelError("Invalid cache directory".to_string()))?
                .to_string()
        ).await
    }

    pub async fn with_config_and_path(config: CandleJinaBertConfig, model_path: String) -> Result<Self> {
        // Load tokenizer
        let tokenizer_path = format!("{}/tokenizer.json", model_path);
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| MemoryError::ModelError(format!("Failed to load tokenizer: {}", e)))?;

        // Configure tokenizer with BatchLongest padding
        let pad_id = tokenizer.token_to_id("[PAD]")
            .ok_or_else(|| MemoryError::ModelError("Missing [PAD] token".to_string()))?;
        
        // Validate tokenizer vocabulary size matches our hardcoded config
        let vocab_size = tokenizer.get_vocab_size(false);
        if vocab_size != 30528 {
            return Err(MemoryError::ModelError(format!(
                "Tokenizer vocab size mismatch: expected 30528, got {}", 
                vocab_size
            )));
        }
        
        let padding_params = PaddingParams {
            strategy: tokenizers::PaddingStrategy::BatchLongest,
            direction: tokenizers::PaddingDirection::Right,
            pad_id,
            pad_token: "[PAD]".to_string(),
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

        // Create hardcoded Jina-BERT config with ALiBi positional embeddings
        let jina_config = Config {
            vocab_size: 30528,
            hidden_size: 768,
            num_hidden_layers: 12,
            num_attention_heads: 12,
            intermediate_size: 3072,
            hidden_act: candle_nn::Activation::Gelu,
            max_position_embeddings: 8192,
            type_vocab_size: 2,
            initializer_range: 0.02,
            layer_norm_eps: 1e-12,
            pad_token_id: 0,
            position_embedding_type: PositionEmbeddingType::Alibi, // Key difference
        };

        // Load model weights (single safetensors file)
        let weights_path = format!("{}/model.safetensors", model_path);
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[weights_path], config.dtype, &config.device)
                .map_err(|e| MemoryError::ModelError(format!("Failed to load weights: {}", e)))?
        };

        // Create model
        let model = BertModel::new(vb, &jina_config)
            .map_err(|e| MemoryError::ModelError(format!("Failed to create model: {}", e)))?;

        let device = config.device.clone();
        Ok(Self {
            model_path,
            config,
            model: Mutex::new(model),
            tokenizer,
            device,
        })
    }

    fn mean_pooling(&self, hidden_states: &Tensor, attention_mask: &Tensor) -> Result<Tensor> {
        // Convert attention mask to float and expand dimensions
        let attention_mask_f32 = attention_mask.to_dtype(DType::F32)
            .map_err(|e| MemoryError::ModelError(format!("Failed to convert attention mask: {}", e)))?;
        
        let expanded_mask = attention_mask_f32.unsqueeze(2)
            .map_err(|e| MemoryError::ModelError(format!("Failed to expand attention mask: {}", e)))?
            .expand(hidden_states.shape())
            .map_err(|e| MemoryError::ModelError(format!("Failed to expand mask shape: {}", e)))?;

        // Apply mask to hidden states
        let masked_hidden = (hidden_states * &expanded_mask)
            .map_err(|e| MemoryError::ModelError(format!("Failed to apply mask: {}", e)))?;

        // Sum along sequence dimension
        let sum_hidden = masked_hidden.sum(1)
            .map_err(|e| MemoryError::ModelError(format!("Failed to sum hidden states: {}", e)))?;

        // Sum attention mask for normalization
        let sum_mask = expanded_mask.sum(1)
            .map_err(|e| MemoryError::ModelError(format!("Failed to sum mask: {}", e)))?;

        // Add small epsilon to avoid division by zero
        let epsilon_val = Tensor::new(&[1e-9f32], &self.device)
            .map_err(|e| MemoryError::ModelError(format!("Failed to create epsilon value: {}", e)))?;
        let ones = Tensor::ones_like(&sum_mask)
            .map_err(|e| MemoryError::ModelError(format!("Failed to create ones tensor: {}", e)))?;
        let epsilon = ones.mul(&epsilon_val)
            .map_err(|e| MemoryError::ModelError(format!("Failed to multiply epsilon: {}", e)))?;

        let sum_mask_safe = sum_mask.add(&epsilon)
            .map_err(|e| MemoryError::ModelError(format!("Failed to add epsilon: {}", e)))?;

        // Calculate mean pooling
        let mean_pooled = sum_hidden.div(&sum_mask_safe)
            .map_err(|e| MemoryError::ModelError(format!("Failed to calculate mean: {}", e)))?;

        Ok(mean_pooled)
    }

    fn forward_pass(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
        // Tokenize texts
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

        // Forward pass with thread-safe model access
        let model = self.model.lock()
            .map_err(|e| MemoryError::ModelError(format!("Failed to acquire model lock: {}", e)))?;
        let hidden_states = model.forward(&token_ids)
            .map_err(|e| MemoryError::ModelError(format!("Forward pass failed: {}", e)))?;

        // Apply mean pooling over all tokens
        let pooled_embeddings = self.mean_pooling(&hidden_states, &attention_mask)?;

        let embeddings_data = pooled_embeddings.to_vec2::<f32>()
            .map_err(|e| MemoryError::ModelError(format!("Failed to convert embeddings: {}", e)))?;

        Ok(embeddings_data)
    }
}

impl EmbeddingModelTrait for CandleJinaBertEmbeddingProvider {
    fn embed(&self, text: &str, task: Option<String>) -> Result<Vec<f32>> {
        self.validate_input(text)?;
        // Jina-BERT doesn't use task-specific instructions, but parameter should be accepted
        let _ = task; // Explicitly acknowledge parameter
        let embeddings = self.forward_pass(&[text])?;
        embeddings.into_iter().next()
            .ok_or_else(|| MemoryError::ModelError("No embeddings generated".to_string()))
    }

    fn batch_embed(&self, texts: &[String], task: Option<String>) -> Result<Vec<Vec<f32>>> {
        self.validate_batch(texts)?;
        // Jina-BERT doesn't use task-specific instructions, but parameter should be accepted
        let _ = task; // Explicitly acknowledge parameter
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        self.forward_pass(&text_refs)
    }

    fn dimension(&self) -> usize {
        self.config.embed_dim as usize
    }

    fn name(&self) -> &str {
        "jina-bert-embedding"
    }

    fn supported_dimensions(&self) -> Vec<usize> {
        vec![768] // Jina-BERT v2 produces 768-dimensional embeddings only
    }

    fn config_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("name".to_string(), self.name().to_string());
        info.insert("dimension".to_string(), self.dimension().to_string());
        info.insert("model".to_string(), "jinaai/jina-embeddings-v2-base-en".to_string());
        info.insert("model_path".to_string(), self.model_path.clone());
        info.insert("embed_dim".to_string(), self.config.embed_dim.to_string());
        info.insert("max_length".to_string(), self.config.max_length.to_string());
        info.insert("dtype".to_string(), format!("{:?}", self.config.dtype));
        info.insert("device".to_string(), format!("{:?}", self.device));
        info.insert("position_embedding".to_string(), "alibi".to_string());
        info.insert("pooling".to_string(), "mean".to_string());
        info
    }

    fn recommended_batch_size(&self) -> usize {
        16 // Moderate batch size for 768-dim model
    }

    fn max_batch_size(&self) -> usize {
        64
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