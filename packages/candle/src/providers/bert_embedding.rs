//! BERT embedding provider for local inference using Candle ML framework
//! 
//! This provider uses sentence-transformers/all-MiniLM-L6-v2 model for generating
//! 384-dimensional embeddings with ProgressHub download and Candle inference.


use std::collections::HashMap;
use std::sync::Mutex;

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, DTYPE};
use progresshub::{ProgressHub, types::ZeroOneOrMany as ProgressHubZeroOneOrMany};
use tokenizers::{Tokenizer, PaddingParams, TruncationParams};

use crate::memory::utils::error::{Error as MemoryError, Result};
use crate::memory::vector::embedding_model::EmbeddingModel;

/// Configuration for BERT embedding model
#[derive(Debug, Clone)]
pub struct CandleBertConfig {
    /// Maximum sequence length for tokenization
    pub max_length: usize,
    /// Model dimension (384 for all-MiniLM-L6-v2)
    pub dimension: usize,
    /// Data type for inference
    pub dtype: DType,
    /// Device for inference
    pub device: Device,
}

impl Default for CandleBertConfig {
    fn default() -> Self {
        Self {
            max_length: 512,
            dimension: 384,
            dtype: DType::F32,
            device: Device::Cpu,
        }
    }
}

/// BERT embedding provider using Candle ML framework
/// 
/// Provides high-performance local embeddings using sentence-transformers/all-MiniLM-L6-v2
/// with automatic model download via ProgressHub and zero-allocation inference patterns.
pub struct CandleBertEmbeddingProvider {
    /// Model cache directory path
    model_path: String,
    /// Model configuration
    config: CandleBertConfig,
    /// Loaded BERT model (thread-safe)
    model: Mutex<BertModel>,
    /// Tokenizer for text processing
    tokenizer: Tokenizer,
    /// Device for inference
    device: Device,
}

impl std::fmt::Debug for CandleBertEmbeddingProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CandleBertEmbeddingProvider")
            .field("model_path", &self.model_path)
            .field("config", &self.config)
            .field("model", &"BertModel { .. }")
            .field("tokenizer", &"Tokenizer { .. }")
            .field("device", &format!("{:?}", self.device))
            .finish()
    }
}

impl CandleBertEmbeddingProvider {
    /// Create new BERT embedding provider with automatic model download
    /// 
    /// Downloads sentence-transformers/all-MiniLM-L6-v2 via ProgressHub and loads
    /// the model for local inference using Candle ML framework.
    /// 
    /// # Returns
    /// Result containing initialized provider ready for embedding generation
    /// 
    /// # Errors
    /// Returns error if model download fails or model loading fails
    pub async fn new() -> Result<Self> {
        let config = CandleBertConfig::default();
        Self::with_config(config).await
    }

    /// Create provider with custom configuration
    pub async fn with_config(config: CandleBertConfig) -> Result<Self> { // Config parameter reserved for future use
        {
            // Download model using ProgressHub (following CandleKimiK2Provider pattern)
            let results = ProgressHub::builder()
                .model("sentence-transformers/all-MiniLM-L6-v2")
                .with_cli_progress()
                .download()
                .await
                .map_err(|e| MemoryError::ModelError(format!("ProgressHub download failed: {}", e)))?;

            // Extract the model path from download results (following KimiK2 pattern lines 137-163)
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
    }

    /// Create provider with custom configuration and existing model path
    pub async fn with_config_and_path(config: CandleBertConfig, model_path: String) -> Result<Self> {
        // Load tokenizer from model directory
        let tokenizer_path = format!("{}/tokenizer.json", model_path);
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| MemoryError::ModelError(format!("Failed to load tokenizer: {}", e)))?;

        // Configure tokenizer for padding and truncation
        if let Some(pad_token) = tokenizer.get_vocab(true).get("[PAD]").copied() {
            let padding_params = PaddingParams {
                strategy: tokenizers::PaddingStrategy::BatchLongest,
                direction: tokenizers::PaddingDirection::Right,
                pad_to_multiple_of: None,
                pad_id: pad_token,
                pad_type_id: 0,
                pad_token: "[PAD]".to_string(),
            };
            tokenizer.with_padding(Some(padding_params));
        }

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

        // Load BERT model configuration
        let config_path = format!("{}/config.json", model_path);
        let config_json = std::fs::read_to_string(&config_path)
            .map_err(|e| MemoryError::ModelError(format!("Failed to read config.json: {}", e)))?;
        
        let bert_config: Config = serde_json::from_str(&config_json)
            .map_err(|e| MemoryError::ModelError(format!("Failed to parse config.json: {}", e)))?;

        // Load model weights using safetensors (following approved BERT example)
        let weights_path = format!("{}/model.safetensors", model_path);
        let vb = unsafe { 
            VarBuilder::from_mmaped_safetensors(&[weights_path], DTYPE, &config.device)
                .map_err(|e| MemoryError::ModelError(format!("Failed to load model weights: {}", e)))?
        };

        // Create BERT model (exact pattern from approved example)
        let model = BertModel::load(vb, &bert_config)
            .map_err(|e| MemoryError::ModelError(format!("Failed to create BERT model: {}", e)))?;

        let device = config.device.clone();
        Ok(Self {
            model_path,
            config,
            model: Mutex::new(model),
            tokenizer,
            device,
        })
    }

    /// Normalize embeddings using L2 normalization (from BERT example)
    fn normalize_l2(embeddings: &Tensor) -> Result<Tensor> {
        // Use exact pattern from Candle BERT example
        Ok(embeddings.broadcast_div(&embeddings.sqr()?.sum_keepdim(1)?.sqrt()?)?)
    }

    /// Attention-mask-aware mean pooling (fixes critical correctness bug)
    /// This is the CORRECT way to pool BERT embeddings - excluding padding tokens
    fn mean_pooling(&self, hidden_states: &Tensor, attention_mask: &Tensor) -> Result<Tensor> {
        // Convert attention mask to float and expand dimensions to match hidden states
        let attention_mask_f32 = attention_mask.to_dtype(DType::F32)
            .map_err(|e| MemoryError::ModelError(format!("Failed to convert attention mask to F32: {}", e)))?;
        
        let expanded_mask = attention_mask_f32.unsqueeze(2)
            .map_err(|e| MemoryError::ModelError(format!("Failed to expand attention mask dimensions: {}", e)))?
            .expand(hidden_states.shape())
            .map_err(|e| MemoryError::ModelError(format!("Failed to expand mask to hidden states shape: {}", e)))?;

        // Apply mask to hidden states (zero out padding tokens)
        let masked_hidden = (hidden_states * &expanded_mask)
            .map_err(|e| MemoryError::ModelError(format!("Failed to apply attention mask to hidden states: {}", e)))?;

        // Sum along sequence dimension (dim=1)
        let sum_hidden = masked_hidden.sum(1)
            .map_err(|e| MemoryError::ModelError(format!("Failed to sum masked hidden states: {}", e)))?;

        // Sum attention mask for proper normalization (count of non-padding tokens)
        let sum_mask = expanded_mask.sum(1)
            .map_err(|e| MemoryError::ModelError(format!("Failed to sum attention mask: {}", e)))?;

        // Add small epsilon to avoid division by zero for sequences with all padding
        let epsilon_val = Tensor::new(&[1e-9f32], &self.device)
            .map_err(|e| MemoryError::ModelError(format!("Failed to create epsilon tensor: {}", e)))?;
        let ones = Tensor::ones_like(&sum_mask)
            .map_err(|e| MemoryError::ModelError(format!("Failed to create ones tensor: {}", e)))?;
        let epsilon = ones.mul(&epsilon_val)
            .map_err(|e| MemoryError::ModelError(format!("Failed to broadcast epsilon: {}", e)))?;

        let sum_mask_safe = sum_mask.add(&epsilon)
            .map_err(|e| MemoryError::ModelError(format!("Failed to add epsilon to mask sum: {}", e)))?;

        // Calculate mean pooling: sum_hidden / sum_mask (proper mean of non-padding tokens)
        let mean_pooled = sum_hidden.div(&sum_mask_safe)
            .map_err(|e| MemoryError::ModelError(format!("Failed to calculate mean pooling: {}", e)))?;

        Ok(mean_pooled)
    }

    /// Process text through BERT model (exact pattern from approved example)
    fn forward_pass(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
        // Tokenize using exact pattern from approved BERT example (lines 156-176)
        let tokens = self.tokenizer
            .encode_batch(texts.to_vec(), true)
            .map_err(|e| MemoryError::ModelError(format!("Tokenization failed: {}", e)))?;
            
        let token_ids = tokens
            .iter()
            .map(|tokens| {
                let tokens = tokens.get_ids().to_vec();
                Tensor::new(tokens.as_slice(), &self.device)
                    .map_err(|e| MemoryError::ModelError(format!("Tensor creation failed: {}", e)))
            })
            .collect::<Result<Vec<_>>>()?;
            
        let attention_mask = tokens
            .iter()
            .map(|tokens| {
                let tokens = tokens.get_attention_mask().to_vec();
                Tensor::new(tokens.as_slice(), &self.device)
                    .map_err(|e| MemoryError::ModelError(format!("Tensor creation failed: {}", e)))
            })
            .collect::<Result<Vec<_>>>()?;

        // Stack tensors (exact pattern from example line 174-176)
        let token_ids = Tensor::stack(&token_ids, 0)
            .map_err(|e| MemoryError::ModelError(format!("Tensor stack failed: {}", e)))?;
        let attention_mask = Tensor::stack(&attention_mask, 0)
            .map_err(|e| MemoryError::ModelError(format!("Tensor stack failed: {}", e)))?;
        let token_type_ids = token_ids.zeros_like()
            .map_err(|e| MemoryError::ModelError(format!("Tensor creation failed: {}", e)))?;

        // Forward pass with thread-safe model access
        let model = self.model.lock()
            .map_err(|e| MemoryError::ModelError(format!("Failed to acquire model lock: {}", e)))?;
        let embeddings = model
            .forward(&token_ids, &token_type_ids, Some(&attention_mask))
            .map_err(|e| MemoryError::ModelError(format!("BERT forward pass failed: {}", e)))?;

        // Apply attention-mask-aware mean pooling (CRITICAL FIX - excludes padding tokens)
        let pooled_embeddings = self.mean_pooling(&embeddings, &attention_mask)?;

        // L2 normalization (exact pattern from example)
        let normalized = Self::normalize_l2(&pooled_embeddings)?;

        // Convert to Vec<Vec<f32>>
        let embeddings_data = normalized.to_vec2::<f32>()
            .map_err(|e| MemoryError::ModelError(format!("Failed to convert embeddings to vec: {}", e)))?;

        Ok(embeddings_data)
    }
}

impl EmbeddingModel for CandleBertEmbeddingProvider {
    fn embed(&self, text: &str, task: Option<String>) -> Result<Vec<f32>> {
        self.validate_input(text)?;
        
        // BERT doesn't use task-specific instructions, but parameter should be accepted
        let _ = task; // Explicitly acknowledge parameter
        let embeddings = self.forward_pass(&[text])?;
        embeddings.into_iter().next()
            .ok_or_else(|| MemoryError::ModelError("No embeddings generated".to_string()))
    }

    fn batch_embed(&self, texts: &[String], task: Option<String>) -> Result<Vec<Vec<f32>>> {
        self.validate_batch(texts)?;
        
        // BERT doesn't use task-specific instructions, but parameter should be accepted
        let _ = task; // Explicitly acknowledge parameter
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        self.forward_pass(&text_refs)
    }

    fn dimension(&self) -> usize {
        self.config.dimension
    }

    fn name(&self) -> &str {
        "bert-embedding"
    }

    fn supported_dimensions(&self) -> Vec<usize> {
        vec![384] // BERT all-MiniLM-L6-v2 produces 384-dimensional embeddings only
    }

    fn config_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("name".to_string(), self.name().to_string());
        info.insert("dimension".to_string(), self.dimension().to_string());
        info.insert("model".to_string(), "sentence-transformers/all-MiniLM-L6-v2".to_string());
        info.insert("max_length".to_string(), self.config.max_length.to_string());
        info.insert("device".to_string(), format!("{:?}", self.device));
        info
    }

    fn recommended_batch_size(&self) -> usize {
        16 // Optimal for BERT inference
    }

    fn max_batch_size(&self) -> usize {
        128 // Maximum for memory efficiency
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