//! Loaded GTE-Qwen model that stays in memory for repeated inference
//!
//! This struct holds the tokenizer, model, device, and config in memory
//! to eliminate repeated disk I/O on every inference call. Designed for
//! use in worker threads that process many requests.
//!
//! Uses tokio::sync::Mutex<Option<Model>> for interior mutability to satisfy both:
//! - TextEmbeddingCapable trait (&self methods)
//! - Qwen2 forward pass requirements (&mut Model for KV cache)
//! - Async spawn_blocking pattern (requires owned Model)

use super::base::CandleGteQwenEmbeddingModel;
use super::config::GTE_QWEN_MODEL_INFO;
use super::instruction;
use crate::capability::traits::TextEmbeddingCapable;
use crate::domain::model::traits::CandleModel;
use crate::domain::model::CandleModelInfo;
use candle_core::{DType, Device};
use candle_nn::VarBuilder;
use candle_transformers::models::qwen2::{Config, Model};
use tokenizers::{PaddingParams, Tokenizer, TruncationParams};

/// Loaded GTE-Qwen model that stays in memory for repeated inference
#[derive(Debug)]
pub struct LoadedGteQwenModel {
    tokenizer: Tokenizer,
    model: tokio::sync::Mutex<Option<Model>>,
    device: Device,
}

impl CandleModel for LoadedGteQwenModel {
    fn info(&self) -> &'static CandleModelInfo {
        &GTE_QWEN_MODEL_INFO
    }
}

impl LoadedGteQwenModel {
    /// Load model into memory from base model reference
    ///
    /// Extracts all initialization logic that was previously done on each
    /// embed() call. The loaded model can then be used for many inferences
    /// without reloading from disk.
    pub async fn load(
        base_model: &CandleGteQwenEmbeddingModel,
    ) -> std::result::Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Get configuration from ModelInfo
        let max_length = base_model
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
        let tokenizer_path =
            base_model.huggingface_file(base_model.info().registry_key, "tokenizer.json").await?;
        let config_path =
            base_model.huggingface_file(base_model.info().registry_key, "config.json").await?;
        let index_path = base_model.huggingface_file(
            base_model.info().registry_key,
            "model.safetensors.index.json",
        ).await?;

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
        let config_str = tokio::fs::read_to_string(&config_path).await
            .map_err(|e| format!("Failed to read config: {}", e))?;
        let qwen_config: Config = serde_json::from_str(&config_str)
            .map_err(|e| format!("Failed to parse config: {}", e))?;

        // Load model weights from index
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

        // Create model
        let model =
            Model::new(&qwen_config, vb).map_err(|e| format!("Failed to create model: {}", e))?;

        Ok(Self {
            tokenizer,
            model: tokio::sync::Mutex::new(Some(model)),
            device,
        })
    }
}

impl TextEmbeddingCapable for LoadedGteQwenModel {
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
        let tokenizer = self.tokenizer.clone();
        let device = self.device.clone();
        Box::pin(async move {
            self.validate_input(&text)?;

        // Lock mutex and extract model
        let mut model_guard = self.model.lock().await;
        let model = model_guard.take().ok_or_else(|| {
            Box::from("Model already in use") as Box<dyn std::error::Error + Send + Sync>
        })?;

        // Run async forward pass
        let (returned_model, embeddings) = instruction::forward_pass_with_task(
            tokenizer,
            model,
            device,
            vec![text],
            task,
        )
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        // Put model back
        *model_guard = Some(returned_model);

        embeddings.into_iter().next().ok_or_else(|| {
            Box::from("No embeddings generated") as Box<dyn std::error::Error + Send + Sync>
        })
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
        let tokenizer = self.tokenizer.clone();
        let device = self.device.clone();
        Box::pin(async move {
            self.validate_batch(&texts)?;

        // Lock mutex and extract model
        let mut model_guard = self.model.lock().await;
        let model = model_guard.take().ok_or_else(|| {
            Box::from("Model already in use") as Box<dyn std::error::Error + Send + Sync>
        })?;

        // Run async forward pass
        let (returned_model, embeddings) = instruction::forward_pass_with_task(
            tokenizer,
            model,
            device,
            texts,
            task,
        )
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        // Put model back
        *model_guard = Some(returned_model);

        Ok(embeddings)
        })
    }

    fn embedding_dimension(&self) -> usize {
        GTE_QWEN_MODEL_INFO.embedding_dimension.unwrap_or(1536) as usize
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
