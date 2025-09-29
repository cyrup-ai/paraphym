//! GTE-Qwen embedding provider for local inference using Candle ML framework
//! GTE-Qwen2 Embedding Provider
//!
//! This provider uses Alibaba-NLP/gte-Qwen2-1.5B-instruct model for generating
//! 1536-dimensional embeddings with ProgressHub download and Candle inference.

use std::collections::HashMap;
use std::sync::Mutex;

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::qwen2::{Config, Model};
use progresshub::{ProgressHub, types::ZeroOneOrMany as ProgressHubZeroOneOrMany};
use tokenizers::{Tokenizer, PaddingParams, TruncationParams};
use crate::memory::utils::error::{Error as MemoryError, Result};
use crate::memory::vector::embedding_model::EmbeddingModel as EmbeddingModelTrait;

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
pub struct CandleGteQwenEmbeddingProvider {
    #[allow(dead_code)] // Used in path construction and config_info - false positive warning
    model_path: String,
    config: CandleGteQwenConfig,
    model: Mutex<Model>,
    tokenizer: Tokenizer,
    device: Device,
}

impl CandleGteQwenEmbeddingProvider {
    pub async fn new() -> Result<Self> {
        let config = CandleGteQwenConfig::default();
        Self::with_config(config).await
    }

    pub async fn with_device(device: Device) -> Result<Self> {
        let config = CandleGteQwenConfig::with_device(device);
        Self::with_config(config).await
    }

    pub async fn with_config(config: CandleGteQwenConfig) -> Result<Self> {
        // Validate dimension support before proceeding
        CandleGteQwenConfig::validate_dimension(config.dimension)?;
        
        // Download model using ProgressHub
        let results = ProgressHub::builder()
            .model("Alibaba-NLP/gte-Qwen2-1.5B-instruct")
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

impl EmbeddingModelTrait for CandleGteQwenEmbeddingProvider {
    fn embed(&self, text: &str, task: Option<String>) -> Result<Vec<f32>> {
        self.validate_input(text)?;
        let embeddings = self.forward_pass_with_task(&[text], task.as_deref())?;
        embeddings.into_iter().next()
            .ok_or_else(|| MemoryError::ModelError("No embeddings generated".to_string()))
    }

    fn batch_embed(&self, texts: &[String], task: Option<String>) -> Result<Vec<Vec<f32>>> {
        self.validate_batch(texts)?;
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        self.forward_pass_with_task(&text_refs, task.as_deref())
    }

    fn dimension(&self) -> usize {
        self.config.dimension
    }

    fn name(&self) -> &str {
        "gte-qwen2-1.5b-instruct"
    }

    fn supported_dimensions(&self) -> Vec<usize> {
        vec![1536] // GTE-Qwen2 1.5B produces 1536-dimensional embeddings only
    }

    fn config_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("name".to_string(), self.name().to_string());
        info.insert("dimension".to_string(), self.dimension().to_string());
        info.insert("model".to_string(), "Alibaba-NLP/gte-Qwen2-1.5B-instruct".to_string());
        info.insert("model_path".to_string(), self.model_path.clone());
        info.insert("max_length".to_string(), self.config.max_length.to_string());
        info.insert("dtype".to_string(), format!("{:?}", self.config.dtype));
        info.insert("device".to_string(), format!("{:?}", self.device));
        info.insert("architecture".to_string(), "decoder-only".to_string());
        info.insert("padding".to_string(), "left".to_string());
        info.insert("pooling".to_string(), "attention-masked-last-token".to_string());
        info.insert("instruction_masking".to_string(), "enabled".to_string());
        info.insert("supported_tasks".to_string(), "search_query,search_document".to_string());
        info
    }

    fn recommended_batch_size(&self) -> usize {
        4 // Conservative for 7B parameter model
    }

    fn max_batch_size(&self) -> usize {
        16
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