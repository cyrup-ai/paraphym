//! NVEmbed v2 embedding provider for local inference using Candle ML framework
//!
//! This provider uses nvidia/NV-Embed-v2 model for generating
//! 4096-dimensional embeddings with Mistral decoder and latent attention.

use std::sync::Mutex;
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::nvembed_v2::model::Model as NvEmbedModel;
use tokenizers::{Tokenizer, PaddingParams, TruncationParams};
use crate::memory::utils::error::{Error as MemoryError, Result};
use crate::domain::model::traits::CandleModel;
use crate::domain::model::CandleModelInfo;
use std::num::NonZeroU32;

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
pub struct CandleNvEmbedEmbeddingModel {
    #[allow(dead_code)] // Used in path construction and config_info - false positive warning
    model_path: String,
    config: CandleNvEmbedConfig,
    model: Mutex<NvEmbedModel>,
    tokenizer: Tokenizer,
    device: Device,
}

impl std::fmt::Debug for CandleNvEmbedEmbeddingModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CandleNvEmbedEmbeddingModel")
            .field("model_path", &self.model_path)
            .field("config", &self.config)
            .field("model", &"<NvEmbedModel>")
            .field("tokenizer", &"<Tokenizer>")
            .field("device", &self.device)
            .finish()
    }
}

impl Default for CandleNvEmbedEmbeddingModel {
    fn default() -> Self {
        crate::runtime::shared_runtime()
            .unwrap_or_else(|| panic!("Shared runtime unavailable"))
            .block_on(Self::new())
            .unwrap_or_else(|e| panic!("Failed to initialize CandleNvEmbedEmbeddingModel: {}", e))
    }
}

impl CandleNvEmbedEmbeddingModel {
    pub async fn new() -> Result<Self> {
        let config = CandleNvEmbedConfig::default();
        Self::with_config(config).await
    }

    /// Format text with task-specific instruction prefix for NVEmbed v2
    fn format_with_instruction(&self, text: &str, task: Option<&str>) -> String {
        match task {
            Some("search_query") => format!("Instruct: Given a web search query, retrieve relevant passages that answer the query.\nQuery: {}", text),
            Some("search_document") => format!("Instruct: Given a web search query, retrieve relevant passages that answer the query.\nPassage: {}", text),
            Some("classification") => format!("Instruct: Retrieve semantically similar text.\nText: {}", text),
            Some("clustering") => format!("Instruct: Identify and group similar text.\nText: {}", text),
            Some("retrieval") => format!("Instruct: Given a question, retrieve passages that answer the question.\nPassage: {}", text),
            _ => text.to_string(), // No instruction for default case
        }
    }

    /// Create instruction mask that excludes instruction tokens from pooling
    /// Returns a mask where 1.0 indicates content tokens and 0.0 indicates instruction tokens
    fn create_instruction_mask(&self, token_ids: &Tensor, formatted_texts: &[String], original_texts: &[&str]) -> Result<Tensor> {
        let (batch_size, seq_len) = token_ids.dims2()
            .map_err(|e| MemoryError::ModelError(format!("Invalid token_ids shape: {}", e)))?;

        let mut instruction_mask_data = vec![vec![1.0f32; seq_len]; batch_size];

        for (batch_idx, (formatted_text, original_text)) in formatted_texts.iter().zip(original_texts.iter()).enumerate() {
            // If text was formatted with instruction, find where original content starts
            if formatted_text != *original_text {
                // Find the last occurrence of original text to correctly identify instruction boundary
                if let Some(content_start_pos) = formatted_text.rfind(original_text) {
                    // Tokenize both full text and content-only to find instruction token boundary
                    let full_tokens = self.tokenizer
                        .encode(formatted_text.as_str(), false)
                        .map_err(|e| MemoryError::ModelError(format!("Failed to tokenize full text: {}", e)))?;

                    let content_only = &formatted_text[content_start_pos..];
                    let content_tokens = self.tokenizer
                        .encode(content_only, false)
                        .map_err(|e| MemoryError::ModelError(format!("Failed to tokenize content: {}", e)))?;

                    let full_token_count = full_tokens.get_ids().len();
                    let content_token_count = content_tokens.get_ids().len();

                    // Calculate instruction token count by difference
                    let instruction_token_count = if full_token_count >= content_token_count {
                        full_token_count - content_token_count
                    } else {
                        // Fallback: use character-based estimation if tokenization is inconsistent
                        let instruction_char_ratio = content_start_pos as f32 / formatted_text.len() as f32;
                        (instruction_char_ratio * full_token_count as f32).ceil() as usize
                    };

                    // Mark instruction tokens as 0.0 (exclude from pooling)
                    for item in instruction_mask_data[batch_idx].iter_mut().take(instruction_token_count.min(seq_len)) {
                        *item = 0.0;
                    }
                }
            }
        }

        // Convert to tensor
        let flat_data: Vec<f32> = instruction_mask_data.into_iter().flatten().collect();
        Tensor::from_vec(flat_data, (batch_size, seq_len), &self.device)
            .map_err(|e| MemoryError::ModelError(format!("Failed to create instruction mask tensor: {}", e)))
    }

    pub async fn with_config(config: CandleNvEmbedConfig) -> Result<Self> {
        use crate::domain::model::download::DownloadProviderFactory;
        
        // Use factory to get download provider (works with both backends)
        let downloader = DownloadProviderFactory::create_default()
            .map_err(|e| MemoryError::ModelError(format!("Failed to create download provider: {}", e)))?;
        
        // Download model files
        let result = downloader.download_model(
            "nvidia/NV-Embed-v2",
            vec!["*.safetensors".to_string(), "tokenizer.json".to_string(), "config.json".to_string()],
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



    fn forward_pass_with_instruction(&self, texts: &[&str], task: Option<&str>) -> Result<Vec<Vec<f32>>> {
        // Format texts with task-specific instructions
        let formatted_texts: Vec<String> = texts.iter()
            .map(|text| self.format_with_instruction(text, task))
            .collect();

        // Tokenize formatted texts
        let tokens = self.tokenizer
            .encode_batch(formatted_texts.clone(), true)
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

        // Create instruction-aware pool_mask that excludes instruction tokens
        let instruction_mask = self.create_instruction_mask(&token_ids, &formatted_texts, texts)?;
        let pool_mask = (&attention_mask * &instruction_mask)
            .map_err(|e| MemoryError::ModelError(format!("Failed to apply instruction mask: {}", e)))?;

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


// Static model info for NV-Embed
static NVEMBED_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::Nvidia,
    name: "NV-Embed-v2",
    registry_key: "nvidia/NV-Embed-v2",
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
    model_id: "nvembed-v2",
    quantization: "none",
    patch: None,
};

impl CandleModel for CandleNvEmbedEmbeddingModel {
    fn info(&self) -> &'static CandleModelInfo {
        &NVEMBED_MODEL_INFO
    }
}

impl crate::capability::traits::TextEmbeddingCapable for CandleNvEmbedEmbeddingModel {
    fn embed(&self, text: &str, task: Option<String>) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> 
    {
        self.validate_input(text)?;
        let task_ref = task.as_deref();
        let embeddings = self.forward_pass_with_instruction(&[text], task_ref)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        embeddings.into_iter().next()
            .ok_or_else(|| "No embeddings generated".into())
    }
    
    fn batch_embed(&self, texts: &[String], task: Option<String>) 
        -> std::result::Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> 
    {
        self.validate_batch(texts)?;
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        let task_ref = task.as_deref();
        self.forward_pass_with_instruction(&text_refs, task_ref)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
    
    fn embedding_dimension(&self) -> usize {
        self.config.embed_dim as usize
    }
    
    fn supported_dimensions(&self) -> Vec<usize> {
        vec![4096]
    }
    
    fn recommended_batch_size(&self) -> usize {
        2
    }
    
    fn max_batch_size(&self) -> usize {
        8
    }
}