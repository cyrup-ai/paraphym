//! Base Stella embedding model implementation

use super::config::{STELLA_400M_MODEL_INFO, detect_variant, embed_dim, get_model_info};
use super::instruction::format_with_instruction;
use crate::capability::traits::TextEmbeddingCapable;
use crate::domain::model::CandleModelInfo;
use crate::domain::model::traits::CandleModel;
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::stella_en_v5::{Config, EmbeddingModel, ModelVariant};
use tokenizers::{PaddingDirection, PaddingParams, PaddingStrategy, Tokenizer, TruncationParams};

/// Stella embedding provider using proper Candle EmbeddingModel architecture
///
/// Provides high-performance local embeddings using dunzhang/stella models
/// with automatic model download via ProgressHub and configurable output dimensions.
/// Uses integrated lm_head projection following real Candle patterns.
#[derive(Debug, Clone)]
pub struct StellaEmbeddingModel {}

impl Default for StellaEmbeddingModel {
    fn default() -> Self {
        Self::new()
    }
}

impl StellaEmbeddingModel {
    /// Create new Stella embedding provider
    #[inline]
    pub fn new() -> Self {
        Self {}
    }
}

impl CandleModel for StellaEmbeddingModel {
    fn info(&self) -> &'static CandleModelInfo {
        // Default to 400M variant
        // Note: This is only used for registry lookup. Actual variant is detected
        // from registry_key during model loading.
        &STELLA_400M_MODEL_INFO
    }
}

impl TextEmbeddingCapable for StellaEmbeddingModel {
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

            // Get ALL config from self.info() - SINGLE SOURCE OF TRUTH
            let max_length = self
                .info()
                .max_input_tokens
                .ok_or("max_input_tokens missing in ModelInfo")?
                .get() as usize;

            let dimension =
                self.info()
                    .embedding_dimension
                    .ok_or("embedding_dimension missing in ModelInfo")? as usize;

            let variant = detect_variant(self.info().registry_key);
            let embed_dim = embed_dim(dimension as u32)?;

            // Auto-detect runtime values
            let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
                log::warn!("Device detection failed: {}. Using CPU.", e);
                Device::Cpu
            });

            let dtype = if device.is_cuda() {
                DType::F16
            } else {
                DType::F32
            };

            // Load files via huggingface_file()
            let base_weights = self
                .huggingface_file(self.info().registry_key, "model.safetensors")
                .await?;
            let projection_head = self
                .huggingface_file(
                    self.info().registry_key,
                    &format!("2_Dense_{}/model.safetensors", dimension),
                )
                .await?;
            let tokenizer_path = self
                .huggingface_file(self.info().registry_key, "tokenizer.json")
                .await?;

            // Load tokenizer with variant-specific padding
            let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
                .map_err(|e| format!("Failed to load tokenizer: {}", e))?;

            // Configure padding based on variant
            match variant {
                ModelVariant::Large => {
                    // 1.5B: Left padding with <|endoftext|>
                    let pad_id = tokenizer
                        .token_to_id("<|endoftext|>")
                        .ok_or("Tokenizer missing <|endoftext|> token")?;

                    let padding_params = PaddingParams {
                        strategy: PaddingStrategy::BatchLongest,
                        direction: PaddingDirection::Left, // ← LEFT for 1.5B
                        pad_to_multiple_of: None,
                        pad_id,
                        pad_type_id: 0,
                        pad_token: "<|endoftext|>".to_string(),
                    };
                    tokenizer.with_padding(Some(padding_params));
                }
                ModelVariant::Small => {
                    // 400M: Right padding
                    tokenizer.with_padding(Some(PaddingParams {
                        strategy: PaddingStrategy::BatchLongest,
                        direction: PaddingDirection::Right, // ← RIGHT for 400M
                        ..Default::default()
                    }));
                }
            }

            // Set truncation
            if tokenizer.get_truncation().is_none() {
                tokenizer
                    .with_truncation(Some(TruncationParams {
                        max_length,
                        strategy: tokenizers::TruncationStrategy::LongestFirst,
                        stride: 0,
                        direction: tokenizers::TruncationDirection::Right,
                    }))
                    .map_err(|e| format!("Failed to set truncation: {}", e))?;
            }

            // Create Stella model config from detected variant
            let stella_config = match variant {
                ModelVariant::Large => Config::new_1_5_b_v5(embed_dim),
                ModelVariant::Small => Config::new_400_m_v5(embed_dim),
            };

            // Load model weights (base + projection head)
            let base_vb = unsafe {
                VarBuilder::from_mmaped_safetensors(&[base_weights], dtype, &device)
                    .map_err(|e| format!("Failed to load base model weights: {}", e))?
            };

            let embed_vb = unsafe {
                VarBuilder::from_mmaped_safetensors(
                    &[projection_head],
                    DType::F32, // Projection heads always F32
                    &device,
                )
                .map_err(|e| format!("Failed to load projection head weights: {}", e))?
            };

            // Create Stella model with MRL projection
            let mut model = EmbeddingModel::new(&stella_config, base_vb, embed_vb)
                .map_err(|e| format!("Failed to create Stella model: {}", e))?;

            // Run inference with task-specific formatting
            let formatted_text = format_with_instruction(&[&text], task.as_deref())[0].clone();

            // Tokenize
            let tokens = tokenizer
                .encode(formatted_text, true)
                .map_err(|e| format!("Tokenization failed: {}", e))?;

            let input_ids = Tensor::new(tokens.get_ids(), &device)
                .map_err(|e| format!("Failed to create input tensor: {}", e))?;
            let attention_mask = Tensor::new(tokens.get_attention_mask(), &device)
                .map_err(|e| format!("Failed to create attention mask: {}", e))?
                .to_dtype(DType::U8)
                .map_err(|e| format!("Failed to convert mask dtype: {}", e))?;

            // Forward pass
            let embeddings = model
                .forward_norm(
                    &input_ids
                        .unsqueeze(0)
                        .map_err(|e| format!("Failed to unsqueeze input_ids: {}", e))?,
                    &attention_mask
                        .unsqueeze(0)
                        .map_err(|e| format!("Failed to unsqueeze attention_mask: {}", e))?,
                )
                .map_err(|e| format!("Stella forward pass failed: {}", e))?;

            // Extract first (and only) embedding
            let embedding_vec = embeddings
                .to_vec2::<f32>()
                .map_err(|e| format!("Failed to convert embeddings to vec: {}", e))?
                .into_iter()
                .next()
                .ok_or("No embeddings generated")?;

            Ok(embedding_vec)
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

            // Same pattern as embed() but batch processing
            let max_length = self
                .info()
                .max_input_tokens
                .ok_or("max_input_tokens missing")?
                .get() as usize;
            let dimension = self
                .info()
                .embedding_dimension
                .ok_or("embedding_dimension missing")? as usize;
            let variant = detect_variant(self.info().registry_key);
            let embed_dim = embed_dim(dimension as u32)?;

            // Auto-detect device/dtype
            let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
                log::warn!("Device detection failed: {}. Using CPU.", e);
                Device::Cpu
            });
            let dtype = if device.is_cuda() {
                DType::F16
            } else {
                DType::F32
            };

            // Load files
            let base_weights = self
                .huggingface_file(self.info().registry_key, "model.safetensors")
                .await?;
            let projection_head = self
                .huggingface_file(
                    self.info().registry_key,
                    &format!("2_Dense_{}/model.safetensors", dimension),
                )
                .await?;
            let tokenizer_path = self
                .huggingface_file(self.info().registry_key, "tokenizer.json")
                .await?;

            // Load and configure tokenizer (variant-specific padding)
            let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
                .map_err(|e| format!("Tokenizer load failed: {}", e))?;

            match variant {
                ModelVariant::Large => {
                    let pad_id = tokenizer
                        .token_to_id("<|endoftext|>")
                        .ok_or("Missing <|endoftext|> token")?;
                    tokenizer.with_padding(Some(PaddingParams {
                        strategy: PaddingStrategy::BatchLongest,
                        direction: PaddingDirection::Left,
                        pad_to_multiple_of: None,
                        pad_id,
                        pad_type_id: 0,
                        pad_token: "<|endoftext|>".to_string(),
                    }));
                }
                ModelVariant::Small => {
                    tokenizer.with_padding(Some(PaddingParams {
                        strategy: PaddingStrategy::BatchLongest,
                        direction: PaddingDirection::Right,
                        ..Default::default()
                    }));
                }
            }

            if tokenizer.get_truncation().is_none() {
                tokenizer
                    .with_truncation(Some(TruncationParams {
                        max_length,
                        strategy: tokenizers::TruncationStrategy::LongestFirst,
                        stride: 0,
                        direction: tokenizers::TruncationDirection::Right,
                    }))
                    .map_err(|e| format!("Failed to set truncation: {}", e))?;
            }

            // Create model config and load weights
            let stella_config = match variant {
                ModelVariant::Large => Config::new_1_5_b_v5(embed_dim),
                ModelVariant::Small => Config::new_400_m_v5(embed_dim),
            };

            let base_vb = unsafe {
                VarBuilder::from_mmaped_safetensors(&[base_weights], dtype, &device)
                    .map_err(|e| format!("Failed to load base model weights: {}", e))?
            };
            let embed_vb = unsafe {
                VarBuilder::from_mmaped_safetensors(&[projection_head], DType::F32, &device)
                    .map_err(|e| format!("Failed to load projection head weights: {}", e))?
            };

            let mut model = EmbeddingModel::new(&stella_config, base_vb, embed_vb)
                .map_err(|e| format!("Failed to create Stella model: {}", e))?;

            // Format and tokenize batch
            let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
            let formatted_texts = format_with_instruction(&text_refs, task.as_deref());
            let formatted_refs: Vec<&str> = formatted_texts.iter().map(|s| s.as_str()).collect();

            let tokens = tokenizer
                .encode_batch(formatted_refs, true)
                .map_err(|e| format!("Tokenization failed: {}", e))?;

            // Stack input tensors
            let mut input_ids = Vec::new();
            let mut attention_masks = Vec::new();

            for token in &tokens {
                let ids = Tensor::new(token.get_ids(), &device)
                    .map_err(|e| format!("Failed to create input tensor: {}", e))?;
                let mask = Tensor::new(token.get_attention_mask(), &device)
                    .map_err(|e| format!("Failed to create attention mask: {}", e))?
                    .to_dtype(DType::U8)
                    .map_err(|e| format!("Failed to convert mask dtype: {}", e))?;
                input_ids.push(ids);
                attention_masks.push(mask);
            }

            let input_ids = Tensor::stack(&input_ids, 0)
                .map_err(|e| format!("Failed to stack input_ids: {}", e))?;
            let attention_mask = Tensor::stack(&attention_masks, 0)
                .map_err(|e| format!("Failed to stack attention_mask: {}", e))?;

            // Forward pass
            let embeddings = model
                .forward_norm(&input_ids, &attention_mask)
                .map_err(|e| format!("Stella forward pass failed: {}", e))?;

            // Convert to Vec<Vec<f32>>
            let embeddings_data = embeddings
                .to_vec2::<f32>()
                .map_err(|e| format!("Failed to convert embeddings to vec: {}", e))?;

            Ok(embeddings_data)
        })
    }

    fn embedding_dimension(&self) -> usize {
        self.info().embedding_dimension.unwrap_or(1024) as usize
    }

    fn supported_dimensions(&self) -> Vec<usize> {
        vec![256, 768, 1024, 2048, 4096, 6144, 8192]
    }

    fn recommended_batch_size(&self) -> usize {
        match detect_variant(self.info().registry_key) {
            ModelVariant::Large => 8,
            ModelVariant::Small => 16,
        }
    }

    fn max_batch_size(&self) -> usize {
        match detect_variant(self.info().registry_key) {
            ModelVariant::Large => 32,
            ModelVariant::Small => 64,
        }
    }
}
