//! Base NVEmbed model implementation
//!
//! This model loads from disk on every call due to trait constraints.
//! See documentation for load_model_and_tokenizer() for details on this pattern.

use super::config::NVEMBED_MODEL_INFO;
use super::instruction;
use crate::capability::traits::TextEmbeddingCapable;
use crate::domain::model::traits::CandleModel;
use crate::domain::model::CandleModelInfo;
use candle_core::{DType, Device};
use candle_nn::VarBuilder;
use candle_transformers::models::nvembed_v2::model::Model as NvEmbedModel;
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
    async fn load_model_and_tokenizer(
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
        let tokenizer_path = self
            .huggingface_file(self.info().registry_key, "tokenizer.json")
            .await?;
        let index_path = self
            .huggingface_file(self.info().registry_key, "model.safetensors.index.json")
            .await?;

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

        let index_content = tokio::fs::read_to_string(&index_path)
            .await
            .map_err(|e| format!("Failed to read index: {}", e))?;
        let index: serde_json::Value =
            serde_json::from_str(&index_content).map_err(|e| format!("Failed to parse index: {}", e))?;

        let weight_map = index["weight_map"]
            .as_object()
            .ok_or("Missing weight_map in index")?;

        let mut unique_files: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        for filename in weight_map.values() {
            if let Some(filename_str) = filename.as_str() {
                unique_files.insert(filename_str.to_string());
            }
        }

        let weight_files: Vec<std::path::PathBuf> =
            unique_files.into_iter().map(|f| model_dir.join(f)).collect();

        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&weight_files, dtype, &device)
                .map_err(|e| format!("Failed to load weights: {}", e))?
        };

        // Create model using NvEmbed API (new, not load)
        let model = NvEmbedModel::new(vb).map_err(|e| format!("Failed to create model: {}", e))?;

        Ok((tokenizer, model, device))
    }
}

impl CandleModel for CandleNvEmbedEmbeddingModel {
    fn info(&self) -> &'static CandleModelInfo {
        &NVEMBED_MODEL_INFO
    }
}

impl TextEmbeddingCapable for CandleNvEmbedEmbeddingModel {
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
            let (tokenizer, mut model, device) = self.load_model_and_tokenizer().await?;

            let embeddings = tokio::task::spawn_blocking(move || {
                let task_ref = task.as_deref();
                instruction::forward_pass_with_instruction(
                    &tokenizer,
                    &mut model,
                    &device,
                    &[&text],
                    task_ref,
                )
            })
            .await
            .map_err(|e| {
                Box::new(std::io::Error::other(format!("Spawn blocking failed: {}", e)))
                    as Box<dyn std::error::Error + Send + Sync>
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
        Box::pin(async move {
            let (tokenizer, mut model, device) = self.load_model_and_tokenizer().await?;

            tokio::task::spawn_blocking(move || {
                let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
                let task_ref = task.as_deref();
                instruction::forward_pass_with_instruction(
                    &tokenizer,
                    &mut model,
                    &device,
                    &text_refs,
                    task_ref,
                )
            })
            .await
            .map_err(|e| {
                Box::new(std::io::Error::other(format!("Spawn blocking failed: {}", e)))
                    as Box<dyn std::error::Error + Send + Sync>
            })?
            .map_err(|e| Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error + Send + Sync>)
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
