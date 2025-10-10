//! GTE-Qwen embedding provider for local inference using Candle ML framework
//! GTE-Qwen2 Embedding Provider
//!
//! This provider uses Alibaba-NLP/gte-Qwen2-1.5B-instruct model for generating
//! 1536-dimensional embeddings with lazy-loading via huggingface_file().

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::qwen2::{Config, Model};
use tokenizers::{Tokenizer, PaddingParams, TruncationParams};
use crate::memory::utils::error::{Error as MemoryError, Result};
use crate::domain::model::traits::CandleModel;
use crate::domain::model::CandleModelInfo;
use std::num::NonZeroU32;

/// GTE-Qwen2 embedding provider using Candle ML framework
#[derive(Debug, Clone)]
pub struct CandleGteQwenEmbeddingModel {}

impl Default for CandleGteQwenEmbeddingModel {
    fn default() -> Self {
        Self::new()
    }
}

impl CandleGteQwenEmbeddingModel {
    #[inline]
    pub fn new() -> Self {
        Self {}
    }

    /// Forward pass with task-specific formatting
    #[inline]
    fn forward_pass_with_task(
        tokenizer: &Tokenizer,
        model: &mut Model,
        device: &Device,
        texts: &[&str],
        task: Option<&str>,
    ) -> Result<Vec<Vec<f32>>> {
        // Format input with task-specific instruction prefix
        let formatted_texts: Vec<String> = match task {
            Some("search_query") => texts.iter()
                .map(|text| format!("Instruct: Given a web search query, retrieve relevant passages that answer the query.\nQuery: {}", text))
                .collect(),
            Some("search_document") | None => texts.iter()
                .map(|text| format!("Instruct: Given a web search query, retrieve relevant passages that answer the query.\nPassage: {}", text))
                .collect(),
            Some(custom_task) => texts.iter()
                .map(|text| format!("Instruct: {}.\nText: {}", custom_task, text))
                .collect(),
        };

        // Tokenize
        let tokens = tokenizer
            .encode_batch(formatted_texts, true)
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
            .map_err(|e| MemoryError::ModelError(format!("Tensor stack failed: {}", e)))?;
        let attention_mask = Tensor::stack(&attention_mask, 0)
            .map_err(|e| MemoryError::ModelError(format!("Attention mask tensor stack failed: {}", e)))?;

        // Forward pass
        let logits = model.forward(&token_ids, 0, None)
            .map_err(|e| MemoryError::ModelError(format!("Forward pass failed: {}", e)))?;

        // Apply attention-masked last token pooling
        let (_batch_size, _seq_len, _hidden_size) = logits.dims3()
            .map_err(|e| MemoryError::ModelError(format!("Invalid logits shape: {}", e)))?;

        // Find actual last tokens using attention mask
        let last_indices = attention_mask.sum(1)
            .map_err(|e| MemoryError::ModelError(format!("Failed to sum attention mask: {}", e)))?
            .to_vec1::<u32>()
            .map_err(|e| MemoryError::ModelError(format!("Failed to convert indices: {}", e)))?
            .into_iter()
            .map(|len| (len - 1) as usize)
            .collect::<Vec<_>>();

        // Extract embeddings for each sequence's actual last token
        let mut batch_embeddings = Vec::new();
        for (i, &last_idx) in last_indices.iter().enumerate() {
            let seq_embeddings = logits.get(i)
                .map_err(|e| MemoryError::ModelError(format!("Failed to get sequence {}: {}", i, e)))?
                .get(last_idx)
                .map_err(|e| MemoryError::ModelError(format!("Failed to get token {}: {}", last_idx, e)))?;
            batch_embeddings.push(seq_embeddings);
        }

        let embeddings = Tensor::stack(&batch_embeddings, 0)
            .map_err(|e| MemoryError::ModelError(format!("Failed to stack embeddings: {}", e)))?;

        let embeddings_data = embeddings.to_vec2::<f32>()
            .map_err(|e| MemoryError::ModelError(format!("Failed to convert embeddings: {}", e)))?;

        Ok(embeddings_data)
    }
}

// Static model info for GTE-Qwen2
static GTE_QWEN_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::AlibabaNLP,
    name: "gte-Qwen2-1.5B-instruct",
    registry_key: "Alibaba-NLP/gte-Qwen2-1.5B-instruct",
    max_input_tokens: NonZeroU32::new(32768),
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
    model_id: "gte-qwen2-1.5b",
    quantization: "none",
    patch: None,
    embedding_dimension: Some(1536),
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

impl CandleModel for CandleGteQwenEmbeddingModel {
    fn info(&self) -> &'static CandleModelInfo {
        &GTE_QWEN_MODEL_INFO
    }
}

impl crate::capability::traits::TextEmbeddingCapable for CandleGteQwenEmbeddingModel {
    fn embed(&self, text: &str, task: Option<String>) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> 
    {
        self.validate_input(text)?;
        
        // Get configuration from ModelInfo
        let max_length = self.info().max_input_tokens
            .ok_or("max_input_tokens missing in ModelInfo")?.get() as usize;
        
        // Auto-detect runtime environment
        let device = crate::core::device_util::detect_best_device()
            .unwrap_or_else(|e| {
                log::warn!("Device detection failed: {}. Using CPU.", e);
                Device::Cpu
            });
        
        // Auto-detect dtype based on device
        let dtype = if device.is_cuda() { DType::F16 } else { DType::F32 };
        
        // Get file paths via huggingface_file
        let tokenizer_path = self.huggingface_file("tokenizer.json")?;
        let config_path = self.huggingface_file("config.json")?;
        let index_path = self.huggingface_file("model.safetensors.index.json")?;
        
        // Load tokenizer
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;
        
        // Configure tokenizer for left padding with EOS token
        let eos_token_id = 151643;
        if tokenizer.token_to_id("|endoftext|>") != Some(eos_token_id) {
            return Err(Box::from(format!(
                "Tokenizer EOS token mismatch: expected {}, got {:?}",
                eos_token_id,
                tokenizer.token_to_id("|endoftext|>")
            )));
        }
        
        let padding_params = PaddingParams {
            strategy: tokenizers::PaddingStrategy::BatchLongest,
            direction: tokenizers::PaddingDirection::Left,
            pad_id: eos_token_id,
            pad_token: "|endoftext|>".to_string(),
            ..Default::default()
        };
        tokenizer.with_padding(Some(padding_params));
        
        let truncation_params = TruncationParams {
            max_length,
            ..Default::default()
        };
        tokenizer.with_truncation(Some(truncation_params))
            .map_err(|e| format!("Failed to set truncation: {}", e))?;
        
        // Load config.json
        let config_str = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config: {}", e))?;
        let qwen_config: Config = serde_json::from_str(&config_str)
            .map_err(|e| format!("Failed to parse config: {}", e))?;
        
        // Load model weights from index
        let model_dir = index_path.parent()
            .ok_or("Failed to get model directory")?;
        
        let index_content = std::fs::read_to_string(&index_path)
            .map_err(|e| format!("Failed to read index: {}", e))?;
        let index: serde_json::Value = serde_json::from_str(&index_content)
            .map_err(|e| format!("Failed to parse index: {}", e))?;
        
        let weight_map = index["weight_map"].as_object()
            .ok_or("Missing weight_map in index")?;
        
        let mut unique_files: std::collections::HashSet<String> = std::collections::HashSet::new();
        for filename in weight_map.values() {
            if let Some(filename_str) = filename.as_str() {
                unique_files.insert(filename_str.to_string());
            }
        }
        
        let weight_files: Vec<std::path::PathBuf> = unique_files
            .into_iter()
            .map(|f| model_dir.join(f))
            .collect();
        
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&weight_files, dtype, &device)
                .map_err(|e| format!("Failed to load weights: {}", e))?
        };
        
        // Create model
        let mut model = Model::new(&qwen_config, vb)
            .map_err(|e| format!("Failed to create model: {}", e))?;
        
        // Run inference
        let task_ref = task.as_deref();
        let embeddings = Self::forward_pass_with_task(&tokenizer, &mut model, &device, &[text], task_ref)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        embeddings.into_iter().next()
            .ok_or_else(|| "No embeddings generated".into())
    }
    
    fn batch_embed(&self, texts: &[String], task: Option<String>) 
        -> std::result::Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> 
    {
        self.validate_batch(texts)?;
        
        // Get configuration from ModelInfo
        let max_length = self.info().max_input_tokens
            .ok_or("max_input_tokens missing in ModelInfo")?.get() as usize;
        
        // Auto-detect runtime environment
        let device = crate::core::device_util::detect_best_device()
            .unwrap_or_else(|e| {
                log::warn!("Device detection failed: {}. Using CPU.", e);
                Device::Cpu
            });
        
        // Auto-detect dtype based on device
        let dtype = if device.is_cuda() { DType::F16 } else { DType::F32 };
        
        // Get file paths via huggingface_file
        let tokenizer_path = self.huggingface_file("tokenizer.json")?;
        let config_path = self.huggingface_file("config.json")?;
        let index_path = self.huggingface_file("model.safetensors.index.json")?;
        
        // Load tokenizer
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;
        
        // Configure tokenizer for left padding with EOS token
        let eos_token_id = 151643;
        if tokenizer.token_to_id("|endoftext|>") != Some(eos_token_id) {
            return Err(Box::from(format!(
                "Tokenizer EOS token mismatch: expected {}, got {:?}",
                eos_token_id,
                tokenizer.token_to_id("|endoftext|>")
            )));
        }
        
        let padding_params = PaddingParams {
            strategy: tokenizers::PaddingStrategy::BatchLongest,
            direction: tokenizers::PaddingDirection::Left,
            pad_id: eos_token_id,
            pad_token: "|endoftext|>".to_string(),
            ..Default::default()
        };
        tokenizer.with_padding(Some(padding_params));
        
        let truncation_params = TruncationParams {
            max_length,
            ..Default::default()
        };
        tokenizer.with_truncation(Some(truncation_params))
            .map_err(|e| format!("Failed to set truncation: {}", e))?;
        
        // Load config.json
        let config_str = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config: {}", e))?;
        let qwen_config: Config = serde_json::from_str(&config_str)
            .map_err(|e| format!("Failed to parse config: {}", e))?;
        
        // Load model weights from index
        let model_dir = index_path.parent()
            .ok_or("Failed to get model directory")?;
        
        let index_content = std::fs::read_to_string(&index_path)
            .map_err(|e| format!("Failed to read index: {}", e))?;
        let index: serde_json::Value = serde_json::from_str(&index_content)
            .map_err(|e| format!("Failed to parse index: {}", e))?;
        
        let weight_map = index["weight_map"].as_object()
            .ok_or("Missing weight_map in index")?;
        
        let mut unique_files: std::collections::HashSet<String> = std::collections::HashSet::new();
        for filename in weight_map.values() {
            if let Some(filename_str) = filename.as_str() {
                unique_files.insert(filename_str.to_string());
            }
        }
        
        let weight_files: Vec<std::path::PathBuf> = unique_files
            .into_iter()
            .map(|f| model_dir.join(f))
            .collect();
        
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&weight_files, dtype, &device)
                .map_err(|e| format!("Failed to load weights: {}", e))?
        };
        
        // Create model
        let mut model = Model::new(&qwen_config, vb)
            .map_err(|e| format!("Failed to create model: {}", e))?;
        
        // Run inference
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        let task_ref = task.as_deref();
        Self::forward_pass_with_task(&tokenizer, &mut model, &device, &text_refs, task_ref)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
    
    fn embedding_dimension(&self) -> usize {
        self.info().embedding_dimension.unwrap_or(1536) as usize
    }
    
    fn supported_dimensions(&self) -> Vec<usize> {
        vec![1536]
    }
    
    fn recommended_batch_size(&self) -> usize {
        4
    }
    
    fn max_batch_size(&self) -> usize {
        16
    }
}


/// Loaded GTE-Qwen model that stays in memory for repeated inference
///
/// This struct holds the tokenizer, model, device, and config in memory
/// to eliminate repeated disk I/O on every inference call. Designed for
/// use in worker threads that process many requests.
///
/// Uses Mutex for interior mutability to satisfy both:
/// - TextEmbeddingCapable trait (&self methods)
/// - Qwen2 forward pass requirements (&mut Model for KV cache)
#[derive(Debug)]
pub struct LoadedGteQwenModel {
    tokenizer: Tokenizer,
    model: std::sync::Mutex<Model>,
    device: Device,
}

impl crate::domain::model::traits::CandleModel for LoadedGteQwenModel {
    fn info(&self) -> &'static CandleModelInfo {
        &GTE_QWEN_MODEL_INFO
    }
}

impl LoadedGteQwenModel {
    /// Load model into memory from base model reference
    ///
    /// Extracts all initialization logic that was previously done on each
    /// embed() call. The loaded model can then be used for many inferences
    /// without reloading from disk.
    pub fn load(base_model: &CandleGteQwenEmbeddingModel)
        -> std::result::Result<Self, Box<dyn std::error::Error + Send + Sync>>
    {
        // Get configuration from ModelInfo
        let max_length = base_model.info().max_input_tokens
            .ok_or("max_input_tokens missing in ModelInfo")?.get() as usize;
        
        // Auto-detect runtime environment
        let device = crate::core::device_util::detect_best_device()
            .unwrap_or_else(|e| {
                log::warn!("Device detection failed: {}. Using CPU.", e);
                Device::Cpu
            });
        
        // Auto-detect dtype based on device
        let dtype = if device.is_cuda() { DType::F16 } else { DType::F32 };
        
        // Get file paths via huggingface_file
        let tokenizer_path = base_model.huggingface_file("tokenizer.json")?;
        let config_path = base_model.huggingface_file("config.json")?;
        let index_path = base_model.huggingface_file("model.safetensors.index.json")?;
        
        // Load tokenizer
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;
        
        // Configure tokenizer for left padding with EOS token
        let eos_token_id = 151643;
        if tokenizer.token_to_id("|endoftext|>") != Some(eos_token_id) {
            return Err(Box::from(format!(
                "Tokenizer EOS token mismatch: expected {}, got {:?}",
                eos_token_id,
                tokenizer.token_to_id("|endoftext|>")
            )));
        }
        
        let padding_params = PaddingParams {
            strategy: tokenizers::PaddingStrategy::BatchLongest,
            direction: tokenizers::PaddingDirection::Left,
            pad_id: eos_token_id,
            pad_token: "|endoftext|>".to_string(),
            ..Default::default()
        };
        tokenizer.with_padding(Some(padding_params));
        
        let truncation_params = TruncationParams {
            max_length,
            ..Default::default()
        };
        tokenizer.with_truncation(Some(truncation_params))
            .map_err(|e| format!("Failed to set truncation: {}", e))?;
        
        // Load config.json
        let config_str = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config: {}", e))?;
        let qwen_config: Config = serde_json::from_str(&config_str)
            .map_err(|e| format!("Failed to parse config: {}", e))?;
        
        // Load model weights from index
        let model_dir = index_path.parent()
            .ok_or("Failed to get model directory")?;
        
        let index_content = std::fs::read_to_string(&index_path)
            .map_err(|e| format!("Failed to read index: {}", e))?;
        let index: serde_json::Value = serde_json::from_str(&index_content)
            .map_err(|e| format!("Failed to parse index: {}", e))?;
        
        let weight_map = index["weight_map"].as_object()
            .ok_or("Missing weight_map in index")?;
        
        let mut unique_files: std::collections::HashSet<String> = std::collections::HashSet::new();
        for filename in weight_map.values() {
            if let Some(filename_str) = filename.as_str() {
                unique_files.insert(filename_str.to_string());
            }
        }
        
        let weight_files: Vec<std::path::PathBuf> = unique_files
            .into_iter()
            .map(|f| model_dir.join(f))
            .collect();
        
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&weight_files, dtype, &device)
                .map_err(|e| format!("Failed to load weights: {}", e))?
        };
        
        // Create model
        let model = Model::new(&qwen_config, vb)
            .map_err(|e| format!("Failed to create model: {}", e))?;
        
        Ok(Self {
            tokenizer,
            model: std::sync::Mutex::new(model),
            device,
        })
    }

}

impl crate::capability::traits::TextEmbeddingCapable for LoadedGteQwenModel {
    fn embed(&self, text: &str, task: Option<String>) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> 
    {
        self.validate_input(text)?;
        
        let mut model_guard = self.model.lock()
            .map_err(|e| Box::from(format!("Failed to lock model: {}", e)) as Box<dyn std::error::Error + Send + Sync>)?;
        
        let embeddings = CandleGteQwenEmbeddingModel::forward_pass_with_task(
            &self.tokenizer,
            &mut *model_guard,
            &self.device,
            &[text],
            task.as_deref(),
        )?;

        embeddings.into_iter().next()
            .ok_or_else(|| Box::from("No embeddings generated") as Box<dyn std::error::Error + Send + Sync>)
    }

    fn batch_embed(&self, texts: &[String], task: Option<String>) 
        -> std::result::Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> 
    {
        self.validate_batch(texts)?;
        
        let mut model_guard = self.model.lock()
            .map_err(|e| Box::from(format!("Failed to lock model: {}", e)) as Box<dyn std::error::Error + Send + Sync>)?;
        
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        CandleGteQwenEmbeddingModel::forward_pass_with_task(
            &self.tokenizer,
            &mut *model_guard,
            &self.device,
            &text_refs,
            task.as_deref(),
        ).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    fn embedding_dimension(&self) -> usize {
        GTE_QWEN_MODEL_INFO.embedding_dimension.unwrap_or(1536) as usize
    }

    fn supported_dimensions(&self) -> Vec<usize> {
        vec![1536]
    }

    fn recommended_batch_size(&self) -> usize {
        4
    }

    fn max_batch_size(&self) -> usize {
        16
    }
}

