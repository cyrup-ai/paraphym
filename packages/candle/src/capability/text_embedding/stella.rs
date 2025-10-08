//! Stella embedding provider for local inference using Candle ML framework
//!
//! This provider uses dunzhang/stella_en_400M_v5 or dunzhang/stella_en_1.5B_v5 models 
//! for generating MRL-trained dimensional embeddings with ProgressHub download and Candle inference.
//! 
//! Supports only trained MRL projection dimensions: 256, 768, 1024, 2048, 4096, 6144, 8192.
//! Architecture follows the real Candle EmbeddingModel pattern with native lm_head projections.

use std::sync::Mutex;

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::stella_en_v5::{Config, EmbedDim, EmbeddingModel, ModelVariant};
use tokenizers::{Tokenizer, PaddingParams, PaddingDirection, PaddingStrategy, TruncationParams};

use crate::memory::utils::error::{Error as MemoryError, Result};
use crate::domain::model::traits::CandleModel;
use crate::domain::model::CandleModelInfo;
use std::num::NonZeroU32;

/// Configuration for Stella embedding model with proper embed_head support
#[derive(Debug, Clone)]
pub struct StellaConfig {
    /// Maximum sequence length for tokenization
    pub max_length: usize,
    /// Model dimension (native EmbedDim only: 256/768/1024/2048/4096/6144/8192)
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
                "Unsupported dimension: {}. Stella natively supports: 256, 768, 1024, 2048, 4096, 6144, 8192",
                dimension
            ))),
        }
    }

    /// Convert dimension to EmbedDim enum for MRL trained projection heads
    fn embed_dim(&self) -> Option<EmbedDim> {
        match self.dimension {
            256 => Some(EmbedDim::Dim256),
            768 => Some(EmbedDim::Dim768),
            1024 => Some(EmbedDim::Dim1024),
            2048 => Some(EmbedDim::Dim2048),
            4096 => Some(EmbedDim::Dim4096),
            6144 => Some(EmbedDim::Dim6144),
            8192 => Some(EmbedDim::Dim8192),
            _ => None,
        }
    }



    /// Get embedding head directory name for MRL trained projection weights
    fn embed_head_dir(&self) -> String {
        format!("2_Dense_{}", self.dimension)
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



/// Stella embedding provider using proper Candle EmbeddingModel architecture
///
/// Provides high-performance local embeddings using dunzhang/stella models
/// with automatic model download via ProgressHub and configurable output dimensions.
/// Uses integrated lm_head projection following real Candle patterns.
pub struct StellaEmbeddingModel {
    /// Model cache directory path
    model_path: String,
    /// Model configuration
    config: StellaConfig,
    /// Stella embedding model with MRL projection head (thread-safe)
    model: Mutex<EmbeddingModel>,
    /// Tokenizer for text processing
    tokenizer: Tokenizer,
    /// Device for inference
    device: Device,
}

impl std::fmt::Debug for StellaEmbeddingModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StellaEmbeddingModel")
            .field("model_path", &self.model_path)
            .field("config", &self.config)
            .field("model", &"EmbeddingModel { .. }")
            .field("tokenizer", &"Tokenizer { .. }")
            .field("device", &format!("{:?}", self.device))
            .finish()
    }
}

impl Default for StellaEmbeddingModel {
    fn default() -> Self {
        crate::runtime::shared_runtime()
            .unwrap_or_else(|| panic!("Shared runtime unavailable"))
            .block_on(Self::new())
            .unwrap_or_else(|e| panic!("Failed to initialize StellaEmbeddingModel: {}", e))
    }
}

impl StellaEmbeddingModel {
    /// Create new Stella embedding provider with 1024-dimensional embeddings (400M model)
    pub async fn new() -> Result<Self> {
        let config = StellaConfig::default();
        Self::with_config(config).await
    }

    /// Create provider with custom configuration
    pub async fn with_config(config: StellaConfig) -> Result<Self> {
        use crate::domain::model::download::DownloadProviderFactory;
        
        // Validate dimension support before proceeding
        StellaConfig::validate_dimension(config.dimension)?;
        
        // Use factory to get download provider (works with both backends)
        let downloader = DownloadProviderFactory::create_default()
            .map_err(|e| MemoryError::ModelError(format!("Failed to create download provider: {}", e)))?;
        
        // Download model files including dimension-specific embedding heads
        let result = downloader.download_model(
            config.repo_name(),
            vec![
                "*.safetensors".to_string(), 
                "tokenizer.json".to_string(), 
                "config.json".to_string(),
                format!("{}/*", config.embed_head_dir()), // MRL projection heads
            ],
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
    pub async fn with_config_and_path(config: StellaConfig, model_path: String) -> Result<Self> {
        // Load tokenizer from model directory
        let tokenizer_path = format!("{}/tokenizer.json", model_path);
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| MemoryError::ModelError(format!("Failed to load tokenizer: {}", e)))?;

        // Configure tokenizer based on model variant (following exact Candle example pattern)
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

        // Create embedding model following exact Candle pattern
        let model = Self::create_embedding_model(&config, &model_path)?;
        let device = config.device.clone();

        Ok(Self {
            model_path,
            config,
            model: Mutex::new(model),
            tokenizer,
            device,
        })
    }

    /// Create embedding model following exact Candle EmbeddingModel pattern
    fn create_embedding_model(config: &StellaConfig, model_path: &str) -> Result<EmbeddingModel> {
        let embed_dim = config.embed_dim()
            .ok_or_else(|| MemoryError::Config(format!("Unsupported dimension: {}", config.dimension)))?;

        // Load Stella model configuration (following exact pattern from stella_en_v5.rs)
        let stella_config = match config.variant {
            ModelVariant::Large => Config::new_1_5_b_v5(embed_dim),
            ModelVariant::Small => Config::new_400_m_v5(embed_dim),
        };

        // Load base model weights (following exact VarBuilder pattern from example)
        let base_weights_path = format!("{}/model.safetensors", model_path);
        let base_vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[base_weights_path], config.dtype, &config.device)
                .map_err(|e| MemoryError::ModelError(format!("Failed to load base model weights: {}", e)))?
        };

        // Load embedding head weights (following exact pattern from example)
        let embed_head_path = format!("{}/{}/model.safetensors", model_path, config.embed_head_dir());
        let embed_vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[embed_head_path], DType::F32, &config.device)
                .map_err(|e| MemoryError::ModelError(format!("Failed to load embedding head weights: {}", e)))?
        };

        // Create Stella model (following exact EmbeddingModel::new pattern)
        EmbeddingModel::new(&stella_config, base_vb, embed_vb)
            .map_err(|e| MemoryError::ModelError(format!("Failed to create Stella model: {}", e)))
    }



    /// Format texts with task-specific instruction prefix following canonical Stella example
    fn format_with_instruction(&self, texts: &[&str], task: Option<&str>) -> Vec<String> {
        let instruct = match task {
            Some("s2p") => "Given a web search query, retrieve relevant passages that answer the query.",
            Some("s2s") => "Retrieve semantically similar text.",
            Some("search_query") => "Given a web search query, retrieve relevant passages that answer the query.", // Map to s2p
            Some("search_document") => "Given a web search query, retrieve relevant passages that answer the query.", // Map to s2p
            Some("classification") => "Retrieve semantically similar text.", // Map to s2s
            Some("clustering") => "Retrieve semantically similar text.", // Map to s2s
            Some("retrieval") => "Given a web search query, retrieve relevant passages that answer the query.", // Map to s2p
            _ => "Given a web search query, retrieve relevant passages that answer the query.", // Default to s2p
        };

        texts.iter()
            .map(|text| format!("Instruct: {}\nQuery: {}", instruct, text))
            .collect()
    }

    /// Process text through Stella model with integrated projection and task support
    fn forward_pass_with_task(&self, texts: &[&str], task: Option<&str>) -> Result<Vec<Vec<f32>>> {
        // Format texts with task-specific instructions (following canonical example)
        let formatted_texts = self.format_with_instruction(texts, task);
        let text_refs: Vec<&str> = formatted_texts.iter().map(|s| s.as_str()).collect();
        
        // Tokenize formatted texts
        let tokens = self.tokenizer
            .encode_batch(text_refs, true)
            .map_err(|e| MemoryError::ModelError(format!("Tokenization failed: {}", e)))?;

        if tokens.is_empty() {
            return Ok(vec![]);
        }

        // Create input tensors (following exact pattern from example)
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

        // Forward pass using native Stella EmbeddingModel (thread-safe)
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


// Static model info for Stella
static STELLA_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::Dunzhang,
    name: "stella_en_400M_v5",
    registry_key: "dunzhang/stella_en_400M_v5",
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
    model_id: "stella-en-400m-v5",
    quantization: "none",
    patch: None,
};

impl CandleModel for StellaEmbeddingModel {
    fn info(&self) -> &'static CandleModelInfo {
        &STELLA_MODEL_INFO
    }
}

impl crate::capability::traits::TextEmbeddingCapable for StellaEmbeddingModel {
    fn embed(&self, text: &str, task: Option<String>) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> 
    {
        self.validate_input(text)?;
        
        let task_ref = task.as_deref();
        let embeddings = self.forward_pass_with_task(&[text], task_ref)
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
        self.forward_pass_with_task(&text_refs, task_ref)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
    
    fn embedding_dimension(&self) -> usize {
        self.config.dimension
    }
    
    fn supported_dimensions(&self) -> Vec<usize> {
        vec![256, 768, 1024, 2048, 4096, 6144, 8192]
    }
    
    fn recommended_batch_size(&self) -> usize {
        match self.config.variant {
            ModelVariant::Large => 8,
            ModelVariant::Small => 16,
        }
    }
    
    fn max_batch_size(&self) -> usize {
        match self.config.variant {
            ModelVariant::Large => 32,
            ModelVariant::Small => 64,
        }
    }
}