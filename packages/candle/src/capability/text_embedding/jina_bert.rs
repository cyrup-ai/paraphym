//! Jina-BERT embedding provider for local inference using Candle ML framework
//!
//! This provider uses jinaai/jina-embeddings-v2-base-en model for generating
//! 768-dimensional embeddings with ALiBi positional embeddings and mean pooling.

use crate::domain::model::CandleModelInfo;
use crate::domain::model::traits::CandleModel;
use crate::memory::utils::error::{Error as MemoryError, Result};
use candle_core::{DType, Device, Module, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::jina_bert::{BertModel, Config, PositionEmbeddingType};
use std::num::NonZeroU32;
use tokenizers::{PaddingParams, Tokenizer, TruncationParams};

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
    fn mean_pooling(
        hidden_states: &Tensor,
        attention_mask: &Tensor,
        device: &Device,
    ) -> Result<Tensor> {
        // Convert attention mask to float and expand dimensions
        let attention_mask_f32 = attention_mask.to_dtype(DType::F32).map_err(|e| {
            MemoryError::ModelError(format!("Failed to convert attention mask: {}", e))
        })?;

        let expanded_mask = attention_mask_f32
            .unsqueeze(2)
            .map_err(|e| {
                MemoryError::ModelError(format!("Failed to expand attention mask: {}", e))
            })?
            .expand(hidden_states.shape())
            .map_err(|e| MemoryError::ModelError(format!("Failed to expand mask shape: {}", e)))?;

        // Apply mask to hidden states
        let masked_hidden = (hidden_states * &expanded_mask)
            .map_err(|e| MemoryError::ModelError(format!("Failed to apply mask: {}", e)))?;

        // Sum along sequence dimension
        let sum_hidden = masked_hidden
            .sum(1)
            .map_err(|e| MemoryError::ModelError(format!("Failed to sum hidden states: {}", e)))?;

        // Sum attention mask for normalization
        let sum_mask = expanded_mask
            .sum(1)
            .map_err(|e| MemoryError::ModelError(format!("Failed to sum mask: {}", e)))?;

        // Add small epsilon to avoid division by zero
        let epsilon_val = Tensor::new(&[1e-9f32], device).map_err(|e| {
            MemoryError::ModelError(format!("Failed to create epsilon value: {}", e))
        })?;
        let ones = Tensor::ones_like(&sum_mask)
            .map_err(|e| MemoryError::ModelError(format!("Failed to create ones tensor: {}", e)))?;
        let epsilon = ones
            .mul(&epsilon_val)
            .map_err(|e| MemoryError::ModelError(format!("Failed to multiply epsilon: {}", e)))?;

        let sum_mask_safe = sum_mask
            .add(&epsilon)
            .map_err(|e| MemoryError::ModelError(format!("Failed to add epsilon: {}", e)))?;

        // Calculate mean pooling
        let mean_pooled = sum_hidden
            .div(&sum_mask_safe)
            .map_err(|e| MemoryError::ModelError(format!("Failed to calculate mean: {}", e)))?;

        Ok(mean_pooled)
    }

    #[inline]
    async fn forward_pass(
        tokenizer: Tokenizer,
        model: BertModel,
        device: Device,
        texts: Vec<String>,
    ) -> Result<Vec<Vec<f32>>> {
        // Tokenize texts - wrap in spawn_blocking for CPU-intensive operation
        let texts_clone = texts.clone();
        let tokenizer_clone = tokenizer.clone();
        let tokens = tokio::task::spawn_blocking(move || {
            tokenizer_clone
                .encode_batch(texts_clone.iter().map(|s| s.as_str()).collect(), true)
                .map_err(|e| MemoryError::ModelError(format!("Tokenization failed: {}", e)))
        })
        .await
        .map_err(|e| MemoryError::ModelError(format!("Spawn blocking failed: {}", e)))??;

        let device_for_tensors = device.clone();
        let token_ids = tokens
            .iter()
            .map(|tokens| {
                let tokens = tokens.get_ids().to_vec();
                Tensor::new(tokens.as_slice(), &device_for_tensors)
                    .map_err(|e| MemoryError::ModelError(format!("Tensor creation failed: {}", e)))
            })
            .collect::<Result<Vec<_>>>()?;

        let attention_mask = tokens
            .iter()
            .map(|tokens| {
                let tokens = tokens.get_attention_mask().to_vec();
                Tensor::new(tokens.as_slice(), &device_for_tensors)
                    .map_err(|e| MemoryError::ModelError(format!("Tensor creation failed: {}", e)))
            })
            .collect::<Result<Vec<_>>>()?;

        let token_ids = Tensor::stack(&token_ids, 0).map_err(|e| {
            MemoryError::ModelError(format!("Token IDs tensor stack failed: {}", e))
        })?;
        let attention_mask = Tensor::stack(&attention_mask, 0).map_err(|e| {
            MemoryError::ModelError(format!("Attention mask tensor stack failed: {}", e))
        })?;

        // Forward pass - wrap in spawn_blocking for CPU-intensive operation
        let token_ids_clone = token_ids.clone();
        let hidden_states = tokio::task::spawn_blocking(move || {
            model
                .forward(&token_ids_clone)
                .map_err(|e| MemoryError::ModelError(format!("Forward pass failed: {}", e)))
        })
        .await
        .map_err(|e| MemoryError::ModelError(format!("Spawn blocking failed: {}", e)))??;

        // Apply mean pooling over all tokens
        let pooled_embeddings = Self::mean_pooling(&hidden_states, &attention_mask, &device)?;

        // Extract embeddings - wrap in spawn_blocking for CPU-intensive operation
        let embeddings_data = tokio::task::spawn_blocking(move || {
            pooled_embeddings
                .to_vec2::<f32>()
                .map_err(|e| MemoryError::ModelError(format!("Failed to convert embeddings: {}", e)))
        })
        .await
        .map_err(|e| MemoryError::ModelError(format!("Spawn blocking failed: {}", e)))??;

        Ok(embeddings_data)
    }
}

// Static model info for Jina-BERT
static JINA_BERT_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::JinaAI,
    name: "jina-embeddings-v2-base-en",
    registry_key: "jinaai/jina-embeddings-v2-base-en",
    quantization_url: None,
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
    fn embed(
        &self,
        text: &str,
        task: Option<String>,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = std::result::Result<
                        Vec<f32>,
                        Box<dyn std::error::Error + Send + Sync>,
                    >,
                > + Send
                + '_,
        >,
    > {
        let text = text.to_string();
        Box::pin(async move {
            self.validate_input(&text)?;
            let _ = task; // Jina-BERT doesn't use task-specific instructions

        // Get max_length from ModelInfo - single source of truth
        let max_length = self
            .info()
            .max_input_tokens
            .ok_or("max_input_tokens missing in ModelInfo")?
            .get() as usize;

        // Auto-detect device
        let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
            log::warn!("Device detection failed: {}. Using CPU.", e);
            Device::Cpu
        });

        // Auto-detect dtype based on device
        let dtype = if device.is_cuda() {
            DType::F16
        } else {
            DType::F32
        };

        // Get file paths via huggingface_file
        let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json").await?;
        let weights_path = self.huggingface_file(self.info().registry_key, "model.safetensors").await?;

        // Load tokenizer
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;

        // Configure tokenizer
        let pad_id = tokenizer
            .token_to_id("[PAD]")
            .ok_or("Missing [PAD] token")?;

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
        tokenizer
            .with_truncation(Some(truncation_params))
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

        // Run inference with async forward_pass
        let embeddings = Self::forward_pass(tokenizer, model, device, vec![text])
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        embeddings
            .into_iter()
            .next()
            .ok_or_else(|| "No embeddings generated".into())
        })
    }

    fn batch_embed(
        &self,
        texts: &[String],
        task: Option<String>,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = std::result::Result<
                        Vec<Vec<f32>>,
                        Box<dyn std::error::Error + Send + Sync>,
                    >,
                > + Send
                + '_,
        >,
    > {
        let texts = texts.to_vec();
        Box::pin(async move {
            self.validate_batch(&texts)?;
            let _ = task; // Jina-BERT doesn't use task-specific instructions

        // Get max_length from ModelInfo - single source of truth
        let max_length = self
            .info()
            .max_input_tokens
            .ok_or("max_input_tokens missing in ModelInfo")?
            .get() as usize;

        // Auto-detect device
        let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
            log::warn!("Device detection failed: {}. Using CPU.", e);
            Device::Cpu
        });

        // Auto-detect dtype based on device
        let dtype = if device.is_cuda() {
            DType::F16
        } else {
            DType::F32
        };

        // Get file paths via huggingface_file
        let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json").await?;
        let weights_path = self.huggingface_file(self.info().registry_key, "model.safetensors").await?;

        // Load tokenizer
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;

        // Configure tokenizer
        let pad_id = tokenizer
            .token_to_id("[PAD]")
            .ok_or("Missing [PAD] token")?;

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
        tokenizer
            .with_truncation(Some(truncation_params))
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

        // Run inference with async forward_pass
        Self::forward_pass(tokenizer, model, device, texts)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        })
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

/// Loaded Jina-BERT model that stays in memory for repeated inference
///
/// This struct holds the tokenizer, model, and device in memory
/// to eliminate repeated disk I/O on every inference call. Designed for
/// use in worker threads that process many requests.
#[derive(Debug)]
pub struct LoadedJinaBertModel {
    tokenizer: Tokenizer,
    model: BertModel,
    device: Device,
}

impl crate::domain::model::traits::CandleModel for LoadedJinaBertModel {
    fn info(&self) -> &'static crate::domain::model::CandleModelInfo {
        &JINA_BERT_MODEL_INFO
    }
}

impl LoadedJinaBertModel {
    /// Load model into memory from base model reference
    ///
    /// Extracts all initialization logic that was previously done on each
    /// embed() call. The loaded model can then be used for many inferences
    /// without reloading from disk.
    pub async fn load(
        base_model: &CandleJinaBertEmbeddingModel,
    ) -> std::result::Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Get configuration from ModelInfo
        let max_length = base_model
            .info()
            .max_input_tokens
            .ok_or("max_input_tokens missing in ModelInfo")?
            .get() as usize;

        // Auto-detect device
        let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
            log::warn!("Device detection failed: {}. Using CPU.", e);
            Device::Cpu
        });

        // Auto-detect dtype based on device
        let dtype = if device.is_cuda() {
            DType::F16
        } else {
            DType::F32
        };

        // Get file paths via huggingface_file
        let tokenizer_path =
            base_model.huggingface_file(base_model.info().registry_key, "tokenizer.json").await?;
        let weights_path =
            base_model.huggingface_file(base_model.info().registry_key, "model.safetensors").await?;

        // Load tokenizer
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;

        // Configure tokenizer
        let pad_id = tokenizer
            .token_to_id("[PAD]")
            .ok_or("Missing [PAD] token")?;

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
        tokenizer
            .with_truncation(Some(truncation_params))
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

        Ok(Self {
            tokenizer,
            model,
            device,
        })
    }

    /// Generate embedding for a single text using pre-loaded model
    ///
    /// # Arguments
    /// * `text` - The input text to embed
    /// * `_task` - Optional task type (unused by Jina-BERT)
    ///
    /// # Returns
    /// 768-dimensional embedding vector or error
    pub async fn embed(
        &self,
        text: &str,
        _task: Option<String>,
    ) -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
        let embeddings = CandleJinaBertEmbeddingModel::forward_pass(
            self.tokenizer.clone(),
            self.model.clone(),
            self.device.clone(),
            vec![text.to_string()],
        )
        .await?;

        embeddings
            .into_iter()
            .next()
            .ok_or_else(|| "No embeddings generated".into())
    }

    /// Generate embeddings for multiple texts in batch
    ///
    /// # Arguments
    /// * `texts` - Slice of input texts to embed
    /// * `_task` - Optional task type (unused by Jina-BERT)
    ///
    /// # Returns
    /// Vector of 768-dimensional embedding vectors or error
    pub async fn batch_embed(
        &self,
        texts: &[String],
        _task: Option<String>,
    ) -> std::result::Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> {
        CandleJinaBertEmbeddingModel::forward_pass(
            self.tokenizer.clone(),
            self.model.clone(),
            self.device.clone(),
            texts.to_vec(),
        )
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Get embedding dimensionality
    pub fn embedding_dimension(&self) -> usize {
        JINA_BERT_MODEL_INFO.embedding_dimension.unwrap_or(768) as usize
    }

    /// Get list of supported embedding dimensions
    pub fn supported_dimensions(&self) -> Vec<usize> {
        vec![768]
    }

    /// Get recommended batch size for optimal performance
    pub fn recommended_batch_size(&self) -> usize {
        8
    }

    /// Get maximum supported batch size
    pub fn max_batch_size(&self) -> usize {
        32
    }
}

// ============================================================================
// Trait Implementation
// ============================================================================

impl crate::capability::traits::TextEmbeddingCapable for LoadedJinaBertModel {
    fn embed(
        &self,
        text: &str,
        _task: Option<String>,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = std::result::Result<
                        Vec<f32>,
                        Box<dyn std::error::Error + Send + Sync>,
                    >,
                > + Send
                + '_,
        >,
    > {
        let text = text.to_string();
        let tokenizer = self.tokenizer.clone();
        let model = self.model.clone();
        let device = self.device.clone();
        Box::pin(async move {
            let embeddings = CandleJinaBertEmbeddingModel::forward_pass(
                tokenizer,
                model,
                device,
                vec![text],
            )
            .await?;

            embeddings
                .into_iter()
                .next()
                .ok_or_else(|| "No embeddings generated".into())
        })
    }

    fn batch_embed(
        &self,
        texts: &[String],
        _task: Option<String>,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = std::result::Result<
                        Vec<Vec<f32>>,
                        Box<dyn std::error::Error + Send + Sync>,
                    >,
                > + Send
                + '_,
        >,
    > {
        let texts = texts.to_vec();
        let tokenizer = self.tokenizer.clone();
        let model = self.model.clone();
        let device = self.device.clone();
        Box::pin(async move {
            CandleJinaBertEmbeddingModel::forward_pass(
                tokenizer,
                model,
                device,
                texts,
            )
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        })
    }

    fn embedding_dimension(&self) -> usize {
        self.embedding_dimension()
    }
}
