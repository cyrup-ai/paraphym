//! BERT embedding provider for local inference using Candle ML framework
//! 
//! This provider uses sentence-transformers/all-MiniLM-L6-v2 model for generating
//! 384-dimensional embeddings with ProgressHub download and Candle inference.


use std::sync::Mutex;

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, DTYPE};
use tokenizers::{Tokenizer, PaddingParams, TruncationParams};
use crate::memory::utils::error::{Error as MemoryError, Result};
use crate::domain::model::traits::CandleModel;
use crate::domain::model::CandleModelInfo;
use std::num::NonZeroU32;

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
pub struct CandleBertEmbeddingModel {
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

impl std::fmt::Debug for CandleBertEmbeddingModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CandleBertEmbeddingModel")
            .field("model_path", &self.model_path)
            .field("config", &self.config)
            .field("model", &"BertModel { .. }")
            .field("tokenizer", &"Tokenizer { .. }")
            .field("device", &format!("{:?}", self.device))
            .finish()
    }
}

impl Default for CandleBertEmbeddingModel {
    fn default() -> Self {
        crate::runtime::shared_runtime()
            .unwrap_or_else(|| panic!("Shared runtime unavailable"))
            .block_on(Self::new())
            .unwrap_or_else(|e| panic!("Failed to initialize CandleBertEmbeddingModel: {}", e))
    }
}

impl CandleBertEmbeddingModel {
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
    pub async fn with_config(config: CandleBertConfig) -> Result<Self> {
        use crate::domain::model::download::DownloadProviderFactory;
        
        // Use factory to get download provider (works with both backends)
        let downloader = DownloadProviderFactory::create_default()
            .map_err(|e| MemoryError::ModelError(format!("Failed to create download provider: {}", e)))?;
        
        // Download model files
        let result = downloader.download_model(
            "sentence-transformers/all-MiniLM-L6-v2",
            vec!["model.safetensors".to_string(), "tokenizer.json".to_string(), "config.json".to_string()],
            None,
        ).collect()
        .map_err(|e| MemoryError::ModelError(format!("Download task failed: {}", e)))?
        .map_err(|e| MemoryError::ModelError(format!("Model download failed: {}", e)))?;
        
        // Use result.cache_dir for model path
        Self::with_config_and_path(
            config,
            result.cache_dir.to_str()
                .ok_or_else(|| MemoryError::ModelError("Invalid cache directory".to_string()))?
                .to_string()
        ).await
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


// Static model info for BERT embedding
static BERT_EMBEDDING_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::SentenceTransformers,
    name: "all-MiniLM-L6-v2",
    registry_key: "sentence-transformers/all-MiniLM-L6-v2",
    max_input_tokens: NonZeroU32::new(512),
    max_output_tokens: None,
    input_price: None,
    output_price: None,
    supports_vision: false,
    supports_function_calling: false,
    supports_streaming: false,
    supports_embeddings: true,
    requires_max_tokens: false,
    supports_thinking: false,
    optimal_thinking_budget: None,
    system_prompt_prefix: None,
    real_name: None,
    model_type: None,
    model_id: "bert-minilm-l6-v2",
    quantization: "none",
    patch: None,
};

impl CandleModel for CandleBertEmbeddingModel {
    fn info(&self) -> &'static CandleModelInfo {
        &BERT_EMBEDDING_MODEL_INFO
    }
}

impl crate::capability::traits::TextEmbeddingCapable for CandleBertEmbeddingModel {
    fn embed(&self, text: &str, task: Option<String>) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> 
    {
        self.validate_input(text)?;
        
        let _ = task; // BERT doesn't use task-specific instructions
        let embeddings = self.forward_pass(&[text])
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        embeddings.into_iter().next()
            .ok_or_else(|| "No embeddings generated".into())
    }
    
    fn batch_embed(&self, texts: &[String], task: Option<String>) 
        -> std::result::Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> 
    {
        self.validate_batch(texts)?;
        
        let _ = task; // BERT doesn't use task-specific instructions
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        self.forward_pass(&text_refs)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
    
    fn embedding_dimension(&self) -> usize {
        self.config.dimension
    }
    
    fn supported_dimensions(&self) -> Vec<usize> {
        vec![384]
    }
    
    fn recommended_batch_size(&self) -> usize {
        16
    }
    
    fn max_batch_size(&self) -> usize {
        128
    }
}