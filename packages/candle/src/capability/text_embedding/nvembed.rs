//! NVEmbed v2 embedding provider for local inference using Candle ML framework
//!
//! This provider uses nvidia/NV-Embed-v2 model for generating
//! 4096-dimensional embeddings with Mistral decoder and latent attention.

use crate::domain::model::CandleModelInfo;
use crate::domain::model::traits::CandleModel;
use crate::memory::utils::error::{Error as MemoryError, Result};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::nvembed_v2::model::Model as NvEmbedModel;
use std::num::NonZeroU32;
use tokenizers::{PaddingParams, Tokenizer, TruncationParams};

/// NVEmbed v2 embedding provider using Candle ML framework
#[derive(Debug, Clone)]
pub struct CandleNvEmbedEmbeddingModel {}

impl Default for CandleNvEmbedEmbeddingModel {
    fn default() -> Self {
        Self::new()
    }
}

impl CandleNvEmbedEmbeddingModel {
    /// Create new NVEmbed v2 embedding provider
    #[inline]
    pub fn new() -> Self {
        Self {}
    }

    /// Format text with task-specific instruction prefix for NVEmbed v2
    #[inline]
    fn format_with_instruction(text: &str, task: Option<&str>) -> String {
        match task {
            Some("search_query") => format!(
                "Instruct: Given a web search query, retrieve relevant passages that answer the query.\nQuery: {}",
                text
            ),
            Some("search_document") => format!(
                "Instruct: Given a web search query, retrieve relevant passages that answer the query.\nPassage: {}",
                text
            ),
            Some("classification") => format!(
                "Instruct: Retrieve semantically similar text.\nText: {}",
                text
            ),
            Some("clustering") => {
                format!("Instruct: Identify and group similar text.\nText: {}", text)
            }
            Some("retrieval") => format!(
                "Instruct: Given a question, retrieve passages that answer the question.\nPassage: {}",
                text
            ),
            _ => text.to_string(), // No instruction for default case
        }
    }

    /// Create instruction mask that excludes instruction tokens from pooling
    /// Returns a mask where 1.0 indicates content tokens and 0.0 indicates instruction tokens
    #[inline]
    fn create_instruction_mask(
        tokenizer: &Tokenizer,
        token_ids: &Tensor,
        formatted_texts: &[String],
        original_texts: &[&str],
        device: &Device,
    ) -> Result<Tensor> {
        let (batch_size, seq_len) = token_ids
            .dims2()
            .map_err(|e| MemoryError::ModelError(format!("Invalid token_ids shape: {}", e)))?;

        let mut instruction_mask_data = vec![vec![1.0f32; seq_len]; batch_size];

        for (batch_idx, (formatted_text, original_text)) in formatted_texts
            .iter()
            .zip(original_texts.iter())
            .enumerate()
        {
            // If text was formatted with instruction, find where original content starts
            if formatted_text != *original_text {
                // Find the last occurrence of original text to correctly identify instruction boundary
                if let Some(content_start_pos) = formatted_text.rfind(original_text) {
                    // Tokenize both full text and content-only to find instruction token boundary
                    let full_tokens =
                        tokenizer
                            .encode(formatted_text.as_str(), false)
                            .map_err(|e| {
                                MemoryError::ModelError(format!(
                                    "Failed to tokenize full text: {}",
                                    e
                                ))
                            })?;

                    let content_only = &formatted_text[content_start_pos..];
                    let content_tokens = tokenizer.encode(content_only, false).map_err(|e| {
                        MemoryError::ModelError(format!("Failed to tokenize content: {}", e))
                    })?;

                    let full_token_count = full_tokens.get_ids().len();
                    let content_token_count = content_tokens.get_ids().len();

                    // Calculate instruction token count by difference
                    let instruction_token_count = if full_token_count >= content_token_count {
                        full_token_count - content_token_count
                    } else {
                        // Fallback: use character-based estimation if tokenization is inconsistent
                        let instruction_char_ratio =
                            content_start_pos as f32 / formatted_text.len() as f32;
                        (instruction_char_ratio * full_token_count as f32).ceil() as usize
                    };

                    // Mark instruction tokens as 0.0 (exclude from pooling)
                    for item in instruction_mask_data[batch_idx]
                        .iter_mut()
                        .take(instruction_token_count.min(seq_len))
                    {
                        *item = 0.0;
                    }
                }
            }
        }

        // Convert to tensor
        let flat_data: Vec<f32> = instruction_mask_data.into_iter().flatten().collect();
        Tensor::from_vec(flat_data, (batch_size, seq_len), device).map_err(|e| {
            MemoryError::ModelError(format!("Failed to create instruction mask tensor: {}", e))
        })
    }

    #[inline]
    fn forward_pass_with_instruction(
        tokenizer: &Tokenizer,
        model: &mut NvEmbedModel,
        device: &Device,
        texts: &[&str],
        task: Option<&str>,
    ) -> Result<Vec<Vec<f32>>> {
        // Format texts with task-specific instructions
        let formatted_texts: Vec<String> = texts
            .iter()
            .map(|text| Self::format_with_instruction(text, task))
            .collect();

        // Tokenize formatted texts
        let tokens = tokenizer
            .encode_batch(formatted_texts.clone(), true)
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

        let token_ids = Tensor::stack(&token_ids, 0).map_err(|e| {
            MemoryError::ModelError(format!("Token IDs tensor stack failed: {}", e))
        })?;
        let attention_mask = Tensor::stack(&attention_mask, 0).map_err(|e| {
            MemoryError::ModelError(format!("Attention mask tensor stack failed: {}", e))
        })?;

        // Create instruction-aware pool_mask that excludes instruction tokens
        let instruction_mask =
            Self::create_instruction_mask(tokenizer, &token_ids, &formatted_texts, texts, device)?;
        let pool_mask = (&attention_mask * &instruction_mask).map_err(|e| {
            MemoryError::ModelError(format!("Failed to apply instruction mask: {}", e))
        })?;

        // Forward pass using real NVEmbed API
        let embeddings = model
            .forward(&token_ids, &attention_mask, &pool_mask)
            .map_err(|e| MemoryError::ModelError(format!("Forward pass failed: {}", e)))?;

        let embeddings_data = embeddings
            .to_vec2::<f32>()
            .map_err(|e| MemoryError::ModelError(format!("Failed to convert embeddings: {}", e)))?;

        Ok(embeddings_data)
    }

    /// Load model and tokenizer from disk
    ///
    /// # Model Loading Pattern
    ///
    /// This method loads the model and tokenizer from disk on EVERY call. This is intentional
    /// given the current architecture where the `TextEmbeddingCapable` trait uses `&self`
    /// (immutable reference), which prevents caching the loaded model in the struct.
    ///
    /// ## Why Reload Per Call?
    /// - Trait signature `fn embed(&self, ...)` prevents mutable state
    /// - Empty struct `CandleNvEmbedEmbeddingModel {}` has no fields to cache
    /// - Configuration comes from static `NVEMBED_MODEL_INFO` (zero-cost abstraction)
    ///
    /// ## Performance Trade-offs
    /// Each call performs:
    /// - Disk I/O: tokenizer.json, model.safetensors.index.json
    /// - JSON parsing: index file
    /// - Memory-mapping: safetensor weight files
    /// - Model construction: NvEmbedModel::new()
    ///
    /// ## Alternative Caching Approaches (Not Implemented)
    /// To implement model caching, would require one of:
    /// 1. **Trait redesign**: Change `&self` to `&mut self` (breaking API change)
    /// 2. **LazyStatic/OnceLock**: Requires `'static` lifetime, complex with device selection
    /// 3. **Arc<Mutex<Model>>**: Thread-safe shared ownership, adds runtime overhead
    /// 4. **Per-thread caching**: thread_local! storage, memory overhead per thread
    ///
    /// ## Consistency Note
    /// This reload-per-call pattern is consistent across ALL embedding models:
    /// - bert.rs
    /// - gte_qwen.rs
    /// - jina_bert.rs
    /// - stella.rs
    /// - nvembed.rs (this file)
    ///
    /// If caching is implemented, it should be done at the architecture level across
    /// all models, not in individual implementations.
    fn load_model_and_tokenizer(
        &self,
    ) -> std::result::Result<
        (Tokenizer, NvEmbedModel, Device),
        Box<dyn std::error::Error + Send + Sync>,
    > {
        // Get configuration from ModelInfo - single source of truth
        let max_length = self
            .info()
            .max_input_tokens
            .ok_or("max_input_tokens missing in ModelInfo")?
            .get() as usize;

        // Auto-detect device (not in ModelInfo - runtime detection)
        let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
            log::warn!("Device detection failed: {}. Using CPU.", e);
            Device::Cpu
        });

        // Auto-detect dtype based on device (not in ModelInfo)
        let dtype = if device.is_cuda() {
            DType::F16
        } else {
            DType::F32
        };

        // Get file paths via huggingface_file
        let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json")?;
        let index_path =
            self.huggingface_file(self.info().registry_key, "model.safetensors.index.json")?;

        // Load tokenizer
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;

        // Configure tokenizer
        // EOS token ID for NV-Embed-v2 tokenizer (Mistral-based vocabulary)
        let eos_pad_id = 2;
        let padding_params = PaddingParams {
            strategy: tokenizers::PaddingStrategy::BatchLongest,
            direction: tokenizers::PaddingDirection::Right,
            pad_id: eos_pad_id,
            pad_token: "</s>".to_string(),
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

        // Load model weights
        let model_dir = index_path.parent().ok_or("Failed to get model directory")?;

        let index_content = std::fs::read_to_string(&index_path)
            .map_err(|e| format!("Failed to read index: {}", e))?;
        let index: serde_json::Value = serde_json::from_str(&index_content)
            .map_err(|e| format!("Failed to parse index: {}", e))?;

        let weight_map = index["weight_map"]
            .as_object()
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
        let model = NvEmbedModel::new(vb).map_err(|e| format!("Failed to create model: {}", e))?;

        Ok((tokenizer, model, device))
    }
}

// Static model info for NV-Embed
static NVEMBED_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::Nvidia,
    name: "NV-Embed-v2",
    registry_key: "nvidia/NV-Embed-v2",
    quantization_url: None,
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
    embedding_dimension: Some(4096),
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

impl CandleModel for CandleNvEmbedEmbeddingModel {
    fn info(&self) -> &'static CandleModelInfo {
        &NVEMBED_MODEL_INFO
    }
}

impl crate::capability::traits::TextEmbeddingCapable for CandleNvEmbedEmbeddingModel {
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

        // Load model and tokenizer from disk
        let (tokenizer, mut model, device) = self.load_model_and_tokenizer()?;

        // Run inference with instruction masking
        let task_ref = task.as_deref();
        let embeddings =
            Self::forward_pass_with_instruction(&tokenizer, &mut model, &device, &[&text], task_ref)
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

        // Load model and tokenizer from disk
        let (tokenizer, mut model, device) = self.load_model_and_tokenizer()?;

        // Run inference with instruction masking
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        let task_ref = task.as_deref();
        Self::forward_pass_with_instruction(&tokenizer, &mut model, &device, &text_refs, task_ref)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        })
    }

    fn embedding_dimension(&self) -> usize {
        self.info().embedding_dimension.unwrap_or(4096) as usize
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

// ============================================================================
// LOADED MODEL WRAPPER (MPOOL_4.md)
// ============================================================================

/// Loaded NvEmbed model that keeps model/tokenizer in memory.
///
/// This wrapper is designed for use in model pool workers where the model is loaded once
/// during worker spawn and reused across many inference calls, eliminating repeated disk I/O.
///
/// ## Usage Pattern
/// ```rust,ignore
/// // In worker spawn:
/// let loaded_model = LoadedNvEmbedModel::load(&base_model)?;
///
/// // In worker loop (no I/O):
/// let embedding = loaded_model.embed("some text", None)?;
/// ```
///
/// ## Memory Layout
/// - tokenizer: Tokenizer (configured with EOS padding)
/// - model: Mutex<NvEmbedModel> (interior mutability for &self -> &mut forwarding)
/// - device: Device (CUDA or CPU)
///
/// Uses Mutex for interior mutability to implement TextEmbeddingCapable trait.
#[derive(Debug)]
pub struct LoadedNvEmbedModel {
    tokenizer: Tokenizer,
    model: std::sync::Mutex<NvEmbedModel>,
    device: Device,
}

impl LoadedNvEmbedModel {
    /// Load model and tokenizer from disk once, returning loaded instance ready for inference.
    ///
    /// This extracts the loading logic from load_model_and_tokenizer() (lines 188-267)
    /// so it can be called once during worker spawn instead of on every inference.
    pub async fn load(
        base_model: &CandleNvEmbedEmbeddingModel,
    ) -> std::result::Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Auto-detect runtime environment
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
        let index_path = base_model.huggingface_file(
            base_model.info().registry_key,
            "model.safetensors.index.json",
        ).await?;

        // Load tokenizer
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;

        // Configure tokenizer
        // EOS token ID for NV-Embed-v2 tokenizer (Mistral-based vocabulary)
        let eos_pad_id = 2;
        let padding_params = PaddingParams {
            strategy: tokenizers::PaddingStrategy::BatchLongest,
            direction: tokenizers::PaddingDirection::Right,
            pad_id: eos_pad_id,
            pad_token: "</s>".to_string(),
            ..Default::default()
        };
        tokenizer.with_padding(Some(padding_params));

        let max_length = base_model
            .info()
            .max_input_tokens
            .ok_or("max_input_tokens missing in ModelInfo")?
            .get() as usize;
        let truncation_params = TruncationParams {
            max_length,
            ..Default::default()
        };
        tokenizer
            .with_truncation(Some(truncation_params))
            .map_err(|e| format!("Failed to set truncation: {}", e))?;

        // Load model weights using index.json to find all shards
        let model_dir = index_path.parent().ok_or("Failed to get model directory")?;

        let index_content = std::fs::read_to_string(&index_path)
            .map_err(|e| format!("Failed to read index: {}", e))?;
        let index: serde_json::Value = serde_json::from_str(&index_content)
            .map_err(|e| format!("Failed to parse index: {}", e))?;

        let weight_map = index["weight_map"]
            .as_object()
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

        // Create model using NvEmbed API (new, not load)
        let model = NvEmbedModel::new(vb).map_err(|e| format!("Failed to create model: {}", e))?;

        Ok(Self {
            tokenizer,
            model: std::sync::Mutex::new(model),
            device,
        })
    }
}

impl crate::domain::model::traits::CandleModel for LoadedNvEmbedModel {
    fn info(&self) -> &'static CandleModelInfo {
        &NVEMBED_MODEL_INFO
    }
}

impl crate::capability::traits::TextEmbeddingCapable for LoadedNvEmbedModel {
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
            // Lock mutex to get mutable access to model
            let mut model = self
                .model
                .lock()
                .map_err(|e| format!("Failed to lock model mutex: {}", e))?;

        let embeddings = CandleNvEmbedEmbeddingModel::forward_pass_with_instruction(
            &self.tokenizer,
            &mut model,
            &self.device,
            &[&text],
            task.as_deref(),
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
            // Lock mutex to get mutable access to model
            let mut model = self
                .model
                .lock()
                .map_err(|e| format!("Failed to lock model mutex: {}", e))?;

            let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
            CandleNvEmbedEmbeddingModel::forward_pass_with_instruction(
                &self.tokenizer,
                &mut model,
                &self.device,
                &text_refs,
                task.as_deref(),
            )
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        })
    }

    fn embedding_dimension(&self) -> usize {
        NVEMBED_MODEL_INFO.embedding_dimension.unwrap_or(4096) as usize
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
