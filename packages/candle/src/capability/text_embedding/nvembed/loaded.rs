//! Loaded NVEmbed model for worker pool pattern
//!
//! This wrapper keeps the model in memory and is used in model pool workers
//! where the model is loaded once during worker spawn and reused across many calls.

use super::base::CandleNvEmbedEmbeddingModel;
use super::config::NVEMBED_MODEL_INFO;
use super::instruction;
use crate::capability::traits::TextEmbeddingCapable;
use crate::domain::model::traits::CandleModel;
use crate::domain::model::CandleModelInfo;
use candle_core::{DType, Device};
use candle_nn::VarBuilder;
use candle_transformers::models::nvembed_v2::model::Model as NvEmbedModel;
use tokenizers::{PaddingParams, Tokenizer, TruncationParams};

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
/// - model: Arc<std::sync::Mutex<NvEmbedModel>> (thread-safe interior mutability)
/// - device: Device (CUDA or CPU)
///
/// Uses Arc<std::sync::Mutex> for thread-safe interior mutability in spawn_blocking context.
#[derive(Clone)]
pub struct LoadedNvEmbedModel {
    tokenizer: Tokenizer,
    model: std::sync::Arc<std::sync::Mutex<NvEmbedModel>>,
    device: Device,
}

impl std::fmt::Debug for LoadedNvEmbedModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoadedNvEmbedModel")
            .field("device", &self.device)
            .field("model", &"Arc<Mutex<NvEmbedModel>>")
            .finish()
    }
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

        let index_content = tokio::fs::read_to_string(&index_path).await
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
            model: std::sync::Arc::new(std::sync::Mutex::new(model)),
            device,
        })
    }
}

impl CandleModel for LoadedNvEmbedModel {
    fn info(&self) -> &'static CandleModelInfo {
        &NVEMBED_MODEL_INFO
    }
}

impl TextEmbeddingCapable for LoadedNvEmbedModel {
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
        // Clone for move into spawn_blocking
        let tokenizer = self.tokenizer.clone();
        let model = self.model.clone();
        let device = self.device.clone();
        
        Box::pin(async move {
            // Wrap CPU-intensive operations in spawn_blocking to avoid blocking async runtime
            // This includes: tokenization (I/O), model forward pass (CPU), and tensor ops (CPU)
            let embeddings = tokio::task::spawn_blocking(move || {
                // Lock mutex to get mutable access to model (synchronous lock in blocking context)
                let mut model_guard = model
                    .lock()
                    .map_err(|e| crate::memory::utils::error::Error::ModelError(format!("Failed to lock model mutex: {}", e)))?;
                
                instruction::forward_pass_with_instruction(
                    &tokenizer,
                    &mut model_guard,
                    &device,
                    &[&text],
                    task.as_deref(),
                )
            })
            .await
            .map_err(|e| {
                Box::new(std::io::Error::other(
                    format!("Spawn blocking failed: {}", e),
                )) as Box<dyn std::error::Error + Send + Sync>
            })?
            .map_err(|e| Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error + Send + Sync>)?;

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
        // Clone for move into spawn_blocking
        let tokenizer = self.tokenizer.clone();
        let model = self.model.clone();
        let device = self.device.clone();
        
        Box::pin(async move {
            // Wrap CPU-intensive operations in spawn_blocking to avoid blocking async runtime
            // This includes: tokenization (I/O), model forward pass (CPU), and tensor ops (CPU)
            let embeddings = tokio::task::spawn_blocking(move || {
                // Lock mutex to get mutable access to model (synchronous lock in blocking context)
                let mut model_guard = model
                    .lock()
                    .map_err(|e| crate::memory::utils::error::Error::ModelError(format!("Failed to lock model mutex: {}", e)))?;
                
                let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
                instruction::forward_pass_with_instruction(
                    &tokenizer,
                    &mut model_guard,
                    &device,
                    &text_refs,
                    task.as_deref(),
                )
            })
            .await
            .map_err(|e| {
                Box::new(std::io::Error::other(
                    format!("Spawn blocking failed: {}", e),
                )) as Box<dyn std::error::Error + Send + Sync>
            })?
            .map_err(|e| Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error + Send + Sync>)?;

            Ok(embeddings)
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
