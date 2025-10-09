//! Jina-BERT embedding provider for local inference using Candle ML framework
//!
//! This provider uses jinaai/jina-embeddings-v2-base-en model for generating
//! 768-dimensional embeddings with ALiBi positional embeddings and mean pooling.

use candle_core::{DType, Device, Tensor, Module};
use candle_nn::VarBuilder;
use candle_transformers::models::jina_bert::{BertModel, Config, PositionEmbeddingType};
use tokenizers::{Tokenizer, PaddingParams, TruncationParams};
use crate::memory::utils::error::{Error as MemoryError, Result};
use crate::domain::model::traits::CandleModel;
use crate::domain::model::CandleModelInfo;
use std::num::NonZeroU32;

/// Jina-BERT embedding provider using Candle ML framework
#[derive(Debug, Clone)]
pub struct CandleJinaBertEmbeddingModel {}

impl Default for CandleJinaBertEmbeddingModel {
    fn default() -> Self {
        Self::new()
    }
}

impl CandleJinaBertEmbeddingModel {
    #[inline]
    pub fn new() -> Self {
        Self {}
    }

    #[inline]
    fn mean_pooling(hidden_states: &Tensor, attention_mask: &Tensor, device: &Device) -> Result<Tensor> {
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
        let epsilon_val = Tensor::new(&[1e-9f32], device)
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

    #[inline]
    fn forward_pass(
        tokenizer: &Tokenizer,
        model: &BertModel,
        device: &Device,
        texts: &[&str],
    ) -> Result<Vec<Vec<f32>>> {
        // Tokenize texts
        let tokens = tokenizer
            .encode_batch(texts.to_vec(), true)
            .map_err(|e| MemoryError::ModelError(format!("Tokenization failed: {}", e)))?;

        let token_ids = tokens.iter().map(|tokens| {
            let tokens = tokens.get_ids().to_vec();
            Tensor::new(tokens.as_slice(), device)
                .map_err(|e| MemoryError::ModelError(format!("Tensor creation failed: {}", e)))
        }).collect::<Result<Vec<_>>>()?;

        let attention_mask = tokens.iter().map(|tokens| {
            let tokens = tokens.get_attention_mask().to_vec();
            Tensor::new(tokens.as_slice(), device)
                .map_err(|e| MemoryError::ModelError(format!("Tensor creation failed: {}", e)))
        }).collect::<Result<Vec<_>>>()?;

        let token_ids = Tensor::stack(&token_ids, 0)
            .map_err(|e| MemoryError::ModelError(format!("Token IDs tensor stack failed: {}", e)))?;
        let attention_mask = Tensor::stack(&attention_mask, 0)
            .map_err(|e| MemoryError::ModelError(format!("Attention mask tensor stack failed: {}", e)))?;

        // Forward pass
        let hidden_states = model.forward(&token_ids)
            .map_err(|e| MemoryError::ModelError(format!("Forward pass failed: {}", e)))?;

        // Apply mean pooling over all tokens
        let pooled_embeddings = Self::mean_pooling(&hidden_states, &attention_mask, device)?;

        let embeddings_data = pooled_embeddings.to_vec2::<f32>()
            .map_err(|e| MemoryError::ModelError(format!("Failed to convert embeddings: {}", e)))?;

        Ok(embeddings_data)
    }
}


// Static model info for Jina-BERT
static JINA_BERT_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::JinaAI,
    name: "jina-embeddings-v2-base-en",
    registry_key: "jinaai/jina-embeddings-v2-base-en",
    max_input_tokens: NonZeroU32::new(8192),
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
    model_id: "jina-bert-v2",
    quantization: "none",
    patch: None,
    embedding_dimension: Some(768),
    vocab_size: None,
    image_size: None,
    image_mean: None,
    image_std: None,
    default_temperature: None,
    default_top_k: None,
    default_top_p: None,
    supports_kv_cache: false,
    supports_flash_attention: false,
    use_bf16: false,
    default_steps: None,
    default_guidance_scale: None,
    time_shift: None,
    est_memory_allocation_mb: 0,
};

impl CandleModel for CandleJinaBertEmbeddingModel {
    fn info(&self) -> &'static CandleModelInfo {
        &JINA_BERT_MODEL_INFO
    }
}

impl crate::capability::traits::TextEmbeddingCapable for CandleJinaBertEmbeddingModel {
    fn embed(&self, text: &str, task: Option<String>) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> 
    {
        self.validate_input(text)?;
        let _ = task; // Jina-BERT doesn't use task-specific instructions
        
        // Get max_length from ModelInfo - single source of truth
        let max_length = self.info().max_input_tokens
            .ok_or("max_input_tokens missing in ModelInfo")?
            .get() as usize;
        
        // Auto-detect device
        let device = crate::core::device_util::detect_best_device()
            .unwrap_or_else(|e| {
                log::warn!("Device detection failed: {}. Using CPU.", e);
                Device::Cpu
            });
        
        // Auto-detect dtype based on device
        let dtype = if device.is_cuda() { DType::F16 } else { DType::F32 };
        
        // Get file paths via huggingface_file
        let tokenizer_path = self.huggingface_file("tokenizer.json")?;
        let weights_path = self.huggingface_file("model.safetensors")?;
        
        // Load tokenizer
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;
        
        // Configure tokenizer
        let pad_id = tokenizer.token_to_id("[PAD]")
            .ok_or_else(|| "Missing [PAD] token")?;
        
        let padding_params = PaddingParams {
            strategy: tokenizers::PaddingStrategy::BatchLongest,
            direction: tokenizers::PaddingDirection::Right,
            pad_id,
            pad_token: "[PAD]".to_string(),
            ..Default::default()
        };
        tokenizer.with_padding(Some(padding_params));
        
        let truncation_params = TruncationParams {
            max_length,
            ..Default::default()
        };
        tokenizer.with_truncation(Some(truncation_params))
            .map_err(|e| format!("Failed to set truncation: {}", e))?;
        
        // Create Jina-BERT config
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
            position_embedding_type: PositionEmbeddingType::Alibi,
        };
        
        // Load model weights
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[weights_path], dtype, &device)
                .map_err(|e| format!("Failed to load weights: {}", e))?
        };
        
        // Create model
        let model = BertModel::new(vb, &jina_config)
            .map_err(|e| format!("Failed to create model: {}", e))?;
        
        // Run inference
        let embeddings = Self::forward_pass(&tokenizer, &model, &device, &[text])
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        embeddings.into_iter().next()
            .ok_or_else(|| "No embeddings generated".into())
    }
    
    fn batch_embed(&self, texts: &[String], task: Option<String>) 
        -> std::result::Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> 
    {
        self.validate_batch(texts)?;
        let _ = task; // Jina-BERT doesn't use task-specific instructions
        
        // Get max_length from ModelInfo - single source of truth
        let max_length = self.info().max_input_tokens
            .ok_or("max_input_tokens missing in ModelInfo")?
            .get() as usize;
        
        // Auto-detect device
        let device = crate::core::device_util::detect_best_device()
            .unwrap_or_else(|e| {
                log::warn!("Device detection failed: {}. Using CPU.", e);
                Device::Cpu
            });
        
        // Auto-detect dtype based on device
        let dtype = if device.is_cuda() { DType::F16 } else { DType::F32 };
        
        // Get file paths via huggingface_file
        let tokenizer_path = self.huggingface_file("tokenizer.json")?;
        let weights_path = self.huggingface_file("model.safetensors")?;
        
        // Load tokenizer
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;
        
        // Configure tokenizer
        let pad_id = tokenizer.token_to_id("[PAD]")
            .ok_or_else(|| "Missing [PAD] token")?;
        
        let padding_params = PaddingParams {
            strategy: tokenizers::PaddingStrategy::BatchLongest,
            direction: tokenizers::PaddingDirection::Right,
            pad_id,
            pad_token: "[PAD]".to_string(),
            ..Default::default()
        };
        tokenizer.with_padding(Some(padding_params));
        
        let truncation_params = TruncationParams {
            max_length,
            ..Default::default()
        };
        tokenizer.with_truncation(Some(truncation_params))
            .map_err(|e| format!("Failed to set truncation: {}", e))?;
        
        // Create Jina-BERT config
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
            position_embedding_type: PositionEmbeddingType::Alibi,
        };
        
        // Load model weights
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[weights_path], dtype, &device)
                .map_err(|e| format!("Failed to load weights: {}", e))?
        };
        
        // Create model
        let model = BertModel::new(vb, &jina_config)
            .map_err(|e| format!("Failed to create model: {}", e))?;
        
        // Run inference
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        Self::forward_pass(&tokenizer, &model, &device, &text_refs)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
    
    fn embedding_dimension(&self) -> usize {
        self.info().embedding_dimension.unwrap_or(768) as usize
    }
    
    fn supported_dimensions(&self) -> Vec<usize> {
        vec![768]
    }
    
    fn recommended_batch_size(&self) -> usize {
        16
    }
    
    fn max_batch_size(&self) -> usize {
        64
    }
}