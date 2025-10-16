//! BERT embedding provider for local inference using Candle ML framework
//!
//! This provider uses sentence-transformers/all-MiniLM-L6-v2 model for generating
//! 384-dimensional embeddings with ProgressHub download and Candle inference.

use crate::domain::model::CandleModelInfo;
use crate::domain::model::traits::CandleModel;
use crate::memory::utils::error::{Error as MemoryError, Result};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config};
use std::num::NonZeroU32;
use tokenizers::{PaddingParams, Tokenizer, TruncationParams};

/// BERT embedding provider using Candle ML framework
///
/// Provides high-performance local embeddings using sentence-transformers/all-MiniLM-L6-v2
/// with automatic model download via ProgressHub and zero-allocation inference patterns.
#[derive(Debug, Clone)]
pub struct CandleBertEmbeddingModel {}

impl Default for CandleBertEmbeddingModel {
    fn default() -> Self {
        Self::new()
    }
}

impl CandleBertEmbeddingModel {
    /// Create new BERT embedding provider
    #[inline]
    pub fn new() -> Self {
        Self {}
    }

    /// Normalize embeddings using L2 normalization (from BERT example)
    fn normalize_l2(embeddings: &Tensor) -> Result<Tensor> {
        // Use exact pattern from Candle BERT example
        Ok(embeddings.broadcast_div(&embeddings.sqr()?.sum_keepdim(1)?.sqrt()?)?)
    }

    /// Attention-mask-aware mean pooling (fixes critical correctness bug)
    /// This is the CORRECT way to pool BERT embeddings - excluding padding tokens
    #[inline]
    fn mean_pooling(
        hidden_states: &Tensor,
        attention_mask: &Tensor,
        device: &Device,
    ) -> Result<Tensor> {
        // Convert attention mask to float and expand dimensions to match hidden states
        let attention_mask_f32 = attention_mask.to_dtype(DType::F32).map_err(|e| {
            MemoryError::ModelError(format!("Failed to convert attention mask to F32: {}", e))
        })?;

        let expanded_mask = attention_mask_f32
            .unsqueeze(2)
            .map_err(|e| {
                MemoryError::ModelError(format!(
                    "Failed to expand attention mask dimensions: {}",
                    e
                ))
            })?
            .expand(hidden_states.shape())
            .map_err(|e| {
                MemoryError::ModelError(format!(
                    "Failed to expand mask to hidden states shape: {}",
                    e
                ))
            })?;

        // Apply mask to hidden states (zero out padding tokens)
        let masked_hidden = (hidden_states * &expanded_mask).map_err(|e| {
            MemoryError::ModelError(format!(
                "Failed to apply attention mask to hidden states: {}",
                e
            ))
        })?;

        // Sum along sequence dimension (dim=1)
        let sum_hidden = masked_hidden.sum(1).map_err(|e| {
            MemoryError::ModelError(format!("Failed to sum masked hidden states: {}", e))
        })?;

        // Sum attention mask for proper normalization (count of non-padding tokens)
        let sum_mask = expanded_mask
            .sum(1)
            .map_err(|e| MemoryError::ModelError(format!("Failed to sum attention mask: {}", e)))?;

        // Add small epsilon to avoid division by zero for sequences with all padding
        let epsilon_val = Tensor::new(&[1e-9f32], device).map_err(|e| {
            MemoryError::ModelError(format!("Failed to create epsilon tensor: {}", e))
        })?;
        let ones = Tensor::ones_like(&sum_mask)
            .map_err(|e| MemoryError::ModelError(format!("Failed to create ones tensor: {}", e)))?;
        let epsilon = ones
            .mul(&epsilon_val)
            .map_err(|e| MemoryError::ModelError(format!("Failed to broadcast epsilon: {}", e)))?;

        let sum_mask_safe = sum_mask.add(&epsilon).map_err(|e| {
            MemoryError::ModelError(format!("Failed to add epsilon to mask sum: {}", e))
        })?;

        // Calculate mean pooling: sum_hidden / sum_mask (proper mean of non-padding tokens)
        let mean_pooled = sum_hidden.div(&sum_mask_safe).map_err(|e| {
            MemoryError::ModelError(format!("Failed to calculate mean pooling: {}", e))
        })?;

        Ok(mean_pooled)
    }

    /// Process text through BERT model
    #[inline]
    fn forward_pass(
        tokenizer: &Tokenizer,
        model: &BertModel,
        device: &Device,
        texts: &[&str],
    ) -> Result<Vec<Vec<f32>>> {
        // Tokenize
        let tokens = tokenizer
            .encode_batch(texts.to_vec(), true)
            .map_err(|e| MemoryError::ModelError(format!("Tokenization failed: {}", e)))?;

        let token_ids = tokens
            .iter()
            .map(|tokens| {
                let tokens = tokens.get_ids().to_vec();
                Tensor::new(tokens.as_slice(), device)
                    .map_err(|e| MemoryError::ModelError(format!("Tensor creation failed: {}", e)))
            })
            .collect::<Result<Vec<_>>>()?;

        let attention_mask = tokens
            .iter()
            .map(|tokens| {
                let tokens = tokens.get_attention_mask().to_vec();
                Tensor::new(tokens.as_slice(), device)
                    .map_err(|e| MemoryError::ModelError(format!("Tensor creation failed: {}", e)))
            })
            .collect::<Result<Vec<_>>>()?;

        // Stack tensors
        let token_ids = Tensor::stack(&token_ids, 0)
            .map_err(|e| MemoryError::ModelError(format!("Tensor stack failed: {}", e)))?;
        let attention_mask = Tensor::stack(&attention_mask, 0)
            .map_err(|e| MemoryError::ModelError(format!("Tensor stack failed: {}", e)))?;
        let token_type_ids = token_ids
            .zeros_like()
            .map_err(|e| MemoryError::ModelError(format!("Tensor creation failed: {}", e)))?;

        // Forward pass
        let embeddings = model
            .forward(&token_ids, &token_type_ids, Some(&attention_mask))
            .map_err(|e| MemoryError::ModelError(format!("BERT forward pass failed: {}", e)))?;

        // Apply attention-mask-aware mean pooling
        let pooled_embeddings = Self::mean_pooling(&embeddings, &attention_mask, device)?;

        // L2 normalization
        let normalized = Self::normalize_l2(&pooled_embeddings)?;

        // Convert to Vec<Vec<f32>>
        let embeddings_data = normalized.to_vec2::<f32>().map_err(|e| {
            MemoryError::ModelError(format!("Failed to convert embeddings to vec: {}", e))
        })?;

        Ok(embeddings_data)
    }
}

// Static model info for BERT embedding
static BERT_EMBEDDING_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::SentenceTransformers,
    name: "all-MiniLM-L6-v2",
    registry_key: "sentence-transformers/all-MiniLM-L6-v2",
    quantization_url: None,
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
    embedding_dimension: Some(384),
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

impl CandleModel for CandleBertEmbeddingModel {
    fn info(&self) -> &'static CandleModelInfo {
        &BERT_EMBEDDING_MODEL_INFO
    }
}

impl crate::capability::traits::TextEmbeddingCapable for CandleBertEmbeddingModel {
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
        let _ = task; // BERT doesn't use task-specific instructions

        // Get configuration from ModelInfo
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
        let model_weights_path =
            self.huggingface_file(self.info().registry_key, "model.safetensors").await?;
        let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json").await?;
        let config_path = self.huggingface_file(self.info().registry_key, "config.json").await?;

        // Load tokenizer
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;

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
                max_length,
                strategy: tokenizers::TruncationStrategy::LongestFirst,
                stride: 0,
                direction: tokenizers::TruncationDirection::Right,
            };
            tokenizer
                .with_truncation(Some(truncation_params))
                .map_err(|e| format!("Failed to set tokenizer truncation: {}", e))?;
        }

        // Load BERT model configuration
        let config_json = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config.json: {}", e))?;
        let bert_config: Config = serde_json::from_str(&config_json)
            .map_err(|e| format!("Failed to parse config.json: {}", e))?;

        // Load model weights
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[model_weights_path], dtype, &device)
                .map_err(|e| format!("Failed to load model weights: {}", e))?
        };

        // Create BERT model
        let model = BertModel::load(vb, &bert_config)
            .map_err(|e| format!("Failed to create BERT model: {}", e))?;

        // Run inference
        let embeddings = Self::forward_pass(&tokenizer, &model, &device, &[&text])
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
        let _ = task; // BERT doesn't use task-specific instructions

        // Get configuration from ModelInfo
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
        let model_weights_path =
            self.huggingface_file(self.info().registry_key, "model.safetensors").await?;
        let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json").await?;
        let config_path = self.huggingface_file(self.info().registry_key, "config.json").await?;

        // Load tokenizer
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;

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
                max_length,
                strategy: tokenizers::TruncationStrategy::LongestFirst,
                stride: 0,
                direction: tokenizers::TruncationDirection::Right,
            };
            tokenizer
                .with_truncation(Some(truncation_params))
                .map_err(|e| format!("Failed to set tokenizer truncation: {}", e))?;
        }

        // Load BERT model configuration
        let config_json = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config.json: {}", e))?;
        let bert_config: Config = serde_json::from_str(&config_json)
            .map_err(|e| format!("Failed to parse config.json: {}", e))?;

        // Load model weights
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[model_weights_path], dtype, &device)
                .map_err(|e| format!("Failed to load model weights: {}", e))?
        };

        // Create BERT model
        let model = BertModel::load(vb, &bert_config)
            .map_err(|e| format!("Failed to create BERT model: {}", e))?;

        // Run inference
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        Self::forward_pass(&tokenizer, &model, &device, &text_refs)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        })
    }

    fn embedding_dimension(&self) -> usize {
        self.info().embedding_dimension.unwrap_or(384) as usize
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

// ============================================================================
// LOADED MODEL WRAPPER (MPOOL_4.md)
// ============================================================================

/// Loaded BERT model that keeps model/tokenizer in memory.
///
/// This wrapper is designed for use in model pool workers where the model is loaded once
/// during worker spawn and reused across many inference calls, eliminating repeated disk I/O.
///
/// ## Usage Pattern
/// ```rust,ignore
/// // In worker spawn:
/// let loaded_model = LoadedBertModel::load(&base_model)?;
///
/// // In worker loop (no I/O):
/// let embedding = loaded_model.embed("some text", None)?;
/// ```
///
/// ## Memory Layout
/// - tokenizer: Tokenizer (configured with [PAD] token padding)
/// - model: BertModel (immutable - no RefCell needed)
/// - device: Device (CUDA or CPU)
///
/// No RefCell needed because forward_pass takes &BertModel (immutable reference).
pub struct LoadedBertModel {
    tokenizer: Tokenizer,
    model: BertModel,
    device: Device,
}

impl std::fmt::Debug for LoadedBertModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoadedBertModel")
            .field("device", &self.device)
            .field("model", &"<BertModel>")
            .finish()
    }
}

impl crate::domain::model::traits::CandleModel for LoadedBertModel {
    fn info(&self) -> &'static crate::domain::model::CandleModelInfo {
        &BERT_EMBEDDING_MODEL_INFO
    }
}

impl LoadedBertModel {
    /// Load model and tokenizer from disk once, returning loaded instance ready for inference.
    ///
    /// This extracts the loading logic from embed() (lines 197-259) so it can be called
    /// once during worker spawn instead of on every inference.
    pub async fn load(
        base_model: &CandleBertEmbeddingModel,
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
        let model_weights_path =
            base_model.huggingface_file(base_model.info().registry_key, "model.safetensors").await?;
        let tokenizer_path =
            base_model.huggingface_file(base_model.info().registry_key, "tokenizer.json").await?;
        let config_path =
            base_model.huggingface_file(base_model.info().registry_key, "config.json").await?;

        // Load tokenizer
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;

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
                max_length,
                strategy: tokenizers::TruncationStrategy::LongestFirst,
                stride: 0,
                direction: tokenizers::TruncationDirection::Right,
            };
            tokenizer
                .with_truncation(Some(truncation_params))
                .map_err(|e| format!("Failed to set tokenizer truncation: {}", e))?;
        }

        // Load BERT model configuration
        let config_json = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config.json: {}", e))?;
        let bert_config: Config = serde_json::from_str(&config_json)
            .map_err(|e| format!("Failed to parse config.json: {}", e))?;

        // Load model weights
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[model_weights_path], dtype, &device)
                .map_err(|e| format!("Failed to load model weights: {}", e))?
        };

        // Create BERT model
        let model = BertModel::load(vb, &bert_config)
            .map_err(|e| format!("Failed to create BERT model: {}", e))?;

        Ok(Self {
            tokenizer,
            model,
            device,
        })
    }
}

impl crate::capability::traits::TextEmbeddingCapable for LoadedBertModel {
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
        Box::pin(async move {
            // No I/O - use loaded state
            // BERT doesn't use task-specific instructions, ignore task parameter

            let embeddings = CandleBertEmbeddingModel::forward_pass(
                &self.tokenizer,
                &self.model,
                &self.device,
                &[&text],
            )
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
        Box::pin(async move {
            // No I/O - use loaded state
            let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();

            CandleBertEmbeddingModel::forward_pass(
                &self.tokenizer,
                &self.model,
                &self.device,
                &text_refs,
            )
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        })
    }

    fn embedding_dimension(&self) -> usize {
        384
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
