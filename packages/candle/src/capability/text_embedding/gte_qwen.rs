//! GTE-Qwen embedding provider for local inference using Candle ML framework
//! GTE-Qwen2 Embedding Provider
//!
//! This provider uses Alibaba-NLP/gte-Qwen2-1.5B-instruct model for generating
//! 1536-dimensional embeddings with ProgressHub download and Candle inference.

use std::sync::Mutex;

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::qwen2::{Config, Model};
use tokenizers::{Tokenizer, PaddingParams, TruncationParams};
use crate::memory::utils::error::{Error as MemoryError, Result};
use crate::domain::model::traits::CandleModel;
use crate::domain::model::CandleModelInfo;
use std::num::NonZeroU32;

/// Configuration for GTE-Qwen2 embedding model
#[derive(Debug, Clone)]
pub struct CandleGteQwenConfig {
    pub max_length: usize,
    pub dimension: usize,
    pub dtype: DType,
    pub device: Device,
}

impl Default for CandleGteQwenConfig {
    fn default() -> Self {
        Self {
            dimension: 1536,
            max_length: 32768,
            dtype: DType::F32,
            device: Device::Cpu,
        }
    }
}

impl CandleGteQwenConfig {
    /// Create configuration with device-specific dtype optimization
    pub fn with_device(device: Device) -> Self {
        let dtype = if device.is_cuda() {
            DType::BF16 // Use BF16 for CUDA
        } else {
            DType::F32  // Use F32 for CPU
        };
        
        Self {
            dimension: 1536,
            max_length: 32768,
            dtype,
            device,
        }
    }

    /// Validate dimension is supported (GTE-Qwen2 only supports 1536 dimensions)
    fn validate_dimension(dimension: usize) -> Result<()> {
        if dimension != 1536 {
            return Err(MemoryError::Config(format!(
                "Unsupported dimension: {}. GTE-Qwen2-1.5B-instruct natively supports only 1536 dimensions",
                dimension
            )));
        }
        Ok(())
    }
}

/// GTE-Qwen2 embedding provider using Candle ML framework
#[derive(Debug)]
pub struct CandleGteQwenEmbeddingModel {
    #[allow(dead_code)] // Used in path construction and config_info - false positive warning
    model_path: String,
    config: CandleGteQwenConfig,
    model: Mutex<Model>,
    tokenizer: Tokenizer,
    device: Device,
}

impl Default for CandleGteQwenEmbeddingModel {
    fn default() -> Self {
        crate::runtime::shared_runtime()
            .unwrap_or_else(|| panic!("Shared runtime unavailable"))
            .block_on(Self::new())
            .unwrap_or_else(|e| panic!("Failed to initialize CandleGteQwenEmbeddingModel: {}", e))
    }
}

impl CandleGteQwenEmbeddingModel {
    pub async fn new() -> Result<Self> {
        let config = CandleGteQwenConfig::default();
        Self::with_config(config).await
    }

    pub async fn with_device(device: Device) -> Result<Self> {
        let config = CandleGteQwenConfig::with_device(device);
        Self::with_config(config).await
    }

    pub async fn with_config(config: CandleGteQwenConfig) -> Result<Self> {
        use crate::domain::model::download::DownloadProviderFactory;
        
        // Validate dimension support before proceeding
        CandleGteQwenConfig::validate_dimension(config.dimension)?;
        
        // Use factory to get download provider (works with both backends)
        let downloader = DownloadProviderFactory::create_default()
            .map_err(|e| MemoryError::ModelError(format!("Failed to create download provider: {}", e)))?;
        
        // Download model files (GTE-Qwen2 uses multiple safetensors with index.json)
        let result = downloader.download_model(
            "Alibaba-NLP/gte-Qwen2-1.5B-instruct",
            vec!["*.safetensors".to_string(), "*.json".to_string()],
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

    pub async fn with_config_and_path(config: CandleGteQwenConfig, model_path: String) -> Result<Self> {
        // Load tokenizer
        let tokenizer_path = format!("{}/tokenizer.json", model_path);
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| MemoryError::ModelError(format!("Failed to load tokenizer: {}", e)))?;

        // Configure tokenizer for left padding with EOS token
        let eos_token_id = 151643; // <|endoftext|> token ID for Qwen
        
        // Validate that the tokenizer actually has this token ID
        if tokenizer.token_to_id("<|endoftext|>") != Some(eos_token_id) {
            return Err(MemoryError::ModelError(format!(
                "Tokenizer EOS token mismatch: expected {}, got {:?}", 
                eos_token_id, 
                tokenizer.token_to_id("<|endoftext|>")
            )));
        }
        
        let padding_params = PaddingParams {
            strategy: tokenizers::PaddingStrategy::BatchLongest,
            direction: tokenizers::PaddingDirection::Left, // Left padding for decoder-only model
            pad_id: eos_token_id,
            pad_token: "<|endoftext|>".to_string(),
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

        // Load model configuration
        let config_path = format!("{}/config.json", model_path);
        let config_str = std::fs::read_to_string(&config_path)
            .map_err(|e| MemoryError::ModelError(format!("Failed to read config: {}", e)))?;
        let qwen_config: Config = serde_json::from_str(&config_str)
            .map_err(|e| MemoryError::ModelError(format!("Failed to parse config: {}", e)))?;

        // Load model weights using index.json (multiple safetensors files)
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

        // Load weights
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&weight_files, config.dtype, &config.device)
                .map_err(|e| MemoryError::ModelError(format!("Failed to load weights: {}", e)))?
        };

        // Create model
        let model = Model::new(&qwen_config, vb)
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



    fn forward_pass_with_task(&self, texts: &[&str], task: Option<&str>) -> Result<Vec<Vec<f32>>> {
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
        let tokens = self.tokenizer
            .encode_batch(formatted_texts, true)
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
            .map_err(|e| MemoryError::ModelError(format!("Tensor stack failed: {}", e)))?;
        let attention_mask = Tensor::stack(&attention_mask, 0)
            .map_err(|e| MemoryError::ModelError(format!("Attention mask tensor stack failed: {}", e)))?;

        // Forward pass with Mutex for thread-safe interior mutability
        let mut model = self.model.lock()
            .map_err(|e| MemoryError::ModelError(format!("Failed to acquire model lock: {}", e)))?;
        let logits = model.forward(&token_ids, 0, None)
            .map_err(|e| MemoryError::ModelError(format!("Forward pass failed: {}", e)))?;

        // Apply attention-masked last token pooling (decoder-only pattern with proper masking)
        let (_batch_size, _seq_len, _hidden_size) = logits.dims3()
            .map_err(|e| MemoryError::ModelError(format!("Invalid logits shape: {}", e)))?;

        // Find actual last tokens using attention mask
        let last_indices = attention_mask.sum(1)
            .map_err(|e| MemoryError::ModelError(format!("Failed to sum attention mask: {}", e)))?
            .to_vec1::<u32>()
            .map_err(|e| MemoryError::ModelError(format!("Failed to convert indices: {}", e)))?
            .into_iter()
            .map(|len| (len - 1) as usize)  // Convert to 0-based last token index
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
        let embeddings = self.forward_pass_with_task(&[text], task.as_deref())
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        embeddings.into_iter().next()
            .ok_or_else(|| "No embeddings generated".into())
    }
    
    fn batch_embed(&self, texts: &[String], task: Option<String>) 
        -> std::result::Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> 
    {
        self.validate_batch(texts)?;
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        self.forward_pass_with_task(&text_refs, task.as_deref())
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
    
    fn embedding_dimension(&self) -> usize {
        self.config.dimension
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