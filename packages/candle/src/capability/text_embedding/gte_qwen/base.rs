//! Base GTE-Qwen model implementation
//!
//! This model loads from disk on every call due to trait constraints.

use super::config::GTE_QWEN_MODEL_INFO;
use super::instruction;
use crate::capability::traits::TextEmbeddingCapable;
use crate::domain::model::CandleModelInfo;
use crate::domain::model::traits::CandleModel;
use candle_core::{DType, Device};
use candle_nn::VarBuilder;
use candle_transformers::models::qwen2::{Config, Model};
use tokenizers::{PaddingParams, Tokenizer, TruncationParams};

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

    /// Load model and tokenizer from disk
    ///
    /// Helper method to eliminate duplication between embed() and batch_embed()
    async fn load_model_and_tokenizer(
        &self,
    ) -> std::result::Result<(Tokenizer, Model, Device), Box<dyn std::error::Error + Send + Sync>>
    {
        // Get configuration from ModelInfo
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
        let config_path = self
            .huggingface_file(self.info().registry_key, "config.json")
            .await?;
        let index_path = self
            .huggingface_file(self.info().registry_key, "model.safetensors.index.json")
            .await?;

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
        tokenizer
            .with_truncation(Some(truncation_params))
            .map_err(|e| format!("Failed to set truncation: {}", e))?;

        // Load config.json
        let config_str = tokio::fs::read_to_string(&config_path)
            .await
            .map_err(|e| format!("Failed to read config: {}", e))?;
        let qwen_config: Config = serde_json::from_str(&config_str)
            .map_err(|e| format!("Failed to parse config: {}", e))?;

        // Load model weights from index
        let model_dir = index_path.parent().ok_or("Failed to get model directory")?;

        let index_content = tokio::fs::read_to_string(&index_path)
            .await
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
        let model =
            Model::new(&qwen_config, vb).map_err(|e| format!("Failed to create model: {}", e))?;

        Ok((tokenizer, model, device))
    }
}

impl CandleModel for CandleGteQwenEmbeddingModel {
    fn info(&self) -> &'static CandleModelInfo {
        &GTE_QWEN_MODEL_INFO
    }
}

impl TextEmbeddingCapable for CandleGteQwenEmbeddingModel {
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

            let (tokenizer, model, device) = self.load_model_and_tokenizer().await?;

            let (_model, embeddings) =
                instruction::forward_pass_with_task(tokenizer, model, device, vec![text], task)
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

            let (tokenizer, model, device) = self.load_model_and_tokenizer().await?;

            let (_model, embeddings) =
                instruction::forward_pass_with_task(tokenizer, model, device, texts, task)
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

            Ok(embeddings)
        })
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
