//! Loaded Stella model wrapper with thread-safe interior mutability

use super::config::{STELLA_MODEL_INFO, detect_variant, embed_dim};
use super::instruction::format_with_instruction;
use crate::capability::traits::TextEmbeddingCapable;
use crate::domain::model::traits::CandleModel;
use crate::domain::model::CandleModelInfo;
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::stella_en_v5::{Config, EmbeddingModel, ModelVariant};
use tokenizers::{PaddingDirection, PaddingParams, PaddingStrategy, Tokenizer, TruncationParams};

/// Loaded Stella model that keeps model/tokenizer in memory.
///
/// This wrapper is designed for use in model pool workers where the model is loaded once
/// during worker spawn and reused across many inference calls, eliminating repeated disk I/O.
///
/// ## Usage Pattern
/// ```rust,ignore
/// // In worker spawn:
/// let loaded_model = LoadedStellaModel::load(&base_model)?;
///
/// // In worker loop (no I/O):
/// let embedding = loaded_model.embed("some text", None)?;
/// ```
///
/// ## Memory Layout
/// - tokenizer: Tokenizer (configured with variant-specific padding)
/// - model: Arc<Mutex<EmbeddingModel>> (thread-safe interior mutability)
/// - device: Device (CUDA or CPU)
/// - config: Config (Stella model configuration)
/// - variant: ModelVariant (Large=1.5B or Small=400M)
#[derive(Clone)]
pub struct LoadedStellaModel {
    tokenizer: Tokenizer,
    model: std::sync::Arc<std::sync::Mutex<EmbeddingModel>>,
    device: Device,
    config: Config,
    variant: ModelVariant,
}

impl std::fmt::Debug for LoadedStellaModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoadedStellaModel")
            .field("device", &self.device)
            .field("variant", &self.variant)
            .field("model", &"Arc<Mutex<EmbeddingModel>>")
            .finish()
    }
}

impl CandleModel for LoadedStellaModel {
    fn info(&self) -> &'static CandleModelInfo {
        &STELLA_MODEL_INFO
    }
}

impl LoadedStellaModel {
    /// Load model and tokenizer from disk once, returning loaded instance ready for inference.
    pub async fn load(
        base_model: &super::base::StellaEmbeddingModel,
    ) -> std::result::Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Get config from ModelInfo
        let max_length = base_model
            .info()
            .max_input_tokens
            .ok_or("max_input_tokens missing in ModelInfo")?
            .get() as usize;

        let dimension = base_model
            .info()
            .embedding_dimension
            .ok_or("embedding_dimension missing in ModelInfo")? as usize;

        let variant = detect_variant(base_model.info().registry_key);
        let embed_dim = embed_dim(dimension as u32)?;

        // Auto-detect device
        let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
            log::warn!("Device detection failed: {}. Using CPU.", e);
            Device::Cpu
        });

        let dtype = if device.is_cuda() {
            DType::F16
        } else {
            DType::F32
        };

        // Load files from HuggingFace
        let base_weights =
            base_model.huggingface_file(base_model.info().registry_key, "model.safetensors").await?;
        let projection_head = base_model.huggingface_file(
            base_model.info().registry_key,
            &format!("2_Dense_{}/model.safetensors", dimension),
        ).await?;
        let tokenizer_path =
            base_model.huggingface_file(base_model.info().registry_key, "tokenizer.json").await?;

        // Load tokenizer with variant-specific padding
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;

        match variant {
            ModelVariant::Large => {
                // 1.5B: Left padding with <|endoftext|>
                let pad_id = tokenizer
                    .token_to_id("<|endoftext|>")
                    .ok_or("Tokenizer missing <|endoftext|> token")?;

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
                // 400M: Right padding
                tokenizer.with_padding(Some(PaddingParams {
                    strategy: PaddingStrategy::BatchLongest,
                    direction: PaddingDirection::Right,
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

        // Create Stella config from detected variant
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
        let model = EmbeddingModel::new(&stella_config, base_vb, embed_vb)
            .map_err(|e| format!("Failed to create Stella model: {}", e))?;

        Ok(Self {
            tokenizer,
            model: std::sync::Arc::new(std::sync::Mutex::new(model)),
            device,
            config: stella_config,
            variant,
        })
    }

    /// Get the embedding output dimension
    pub fn embedding_dimension(&self) -> usize {
        self.config.embed_head.out_features
    }

    /// Get supported MRL dimensions (Matryoshka Representation Learning)
    pub fn supported_dimensions(&self) -> Vec<usize> {
        vec![256, 768, 1024, 2048, 4096, 6144, 8192]
    }

    /// Get recommended batch size for this variant
    pub fn recommended_batch_size(&self) -> usize {
        match self.variant {
            ModelVariant::Large => 2, // 1.5B model - conservative
            ModelVariant::Small => 8, // 400M model - more aggressive
        }
    }

    /// Get maximum safe batch size for this variant
    pub fn max_batch_size(&self) -> usize {
        match self.variant {
            ModelVariant::Large => 8,  // 1.5B model - GPU memory limit
            ModelVariant::Small => 32, // 400M model - more headroom
        }
    }
}

impl TextEmbeddingCapable for LoadedStellaModel {
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
            let embedding_vec = tokio::task::spawn_blocking(move || {
                // Format with instruction prefix
                let formatted_text = format_with_instruction(
                    &[&text],
                    task.as_deref(),
                )[0]
                .clone();

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

                // Forward pass - lock synchronous mutex in blocking context
                let mut model_guard = model
                    .lock()
                    .map_err(|_| "Failed to lock model mutex".to_string())?;
                let embeddings = model_guard
                    .forward_norm(
                        &input_ids
                            .unsqueeze(0)
                            .map_err(|e| format!("Failed to unsqueeze input_ids: {}", e))?,
                        &attention_mask
                            .unsqueeze(0)
                            .map_err(|e| format!("Failed to unsqueeze attention_mask: {}", e))?,
                    )
                    .map_err(|e| format!("Stella forward pass failed: {}", e))?;

                // Extract first embedding
                let embedding_vec = embeddings
                    .to_vec2::<f32>()
                    .map_err(|e| format!("Failed to convert embeddings to vec: {}", e))?
                    .into_iter()
                    .next()
                    .ok_or("No embeddings generated")?;

                Ok::<Vec<f32>, String>(embedding_vec)
            })
            .await
            .map_err(|e| {
                Box::new(std::io::Error::other(
                    format!("Spawn blocking failed: {}", e),
                )) as Box<dyn std::error::Error + Send + Sync>
            })?
            .map_err(|e| Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error + Send + Sync>)?;

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
        // Clone for move into spawn_blocking
        let tokenizer = self.tokenizer.clone();
        let model = self.model.clone();
        let device = self.device.clone();
        
        Box::pin(async move {
            // Wrap CPU-intensive operations in spawn_blocking to avoid blocking async runtime
            let embeddings_vec = tokio::task::spawn_blocking(move || {
                let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();

                // Format with instruction prefix
                let formatted_texts = format_with_instruction(
                    &text_refs,
                    task.as_deref(),
                );

                // Tokenize batch
                let encodings = tokenizer
                    .encode_batch(formatted_texts, true)
                    .map_err(|e| format!("Batch tokenization failed: {}", e))?;

                // Create batch tensors
                let ids_vecs: Vec<Vec<u32>> = encodings.iter().map(|e| e.get_ids().to_vec()).collect();
                let mask_vecs: Vec<Vec<u32>> = encodings
                    .iter()
                    .map(|e| e.get_attention_mask().to_vec())
                    .collect();

                let input_ids = Tensor::new(ids_vecs, &device)
                    .map_err(|e| format!("Failed to create batch input tensor: {}", e))?;
                let attention_mask = Tensor::new(mask_vecs, &device)
                    .map_err(|e| format!("Failed to create batch attention mask: {}", e))?
                    .to_dtype(DType::U8)
                    .map_err(|e| format!("Failed to convert mask dtype: {}", e))?;

                // Forward pass - lock synchronous mutex in blocking context
                let mut model_guard = model
                    .lock()
                    .map_err(|_| "Failed to lock model mutex".to_string())?;
                let embeddings = model_guard
                    .forward_norm(&input_ids, &attention_mask)
                    .map_err(|e| format!("Stella batch forward pass failed: {}", e))?;

                // Convert to Vec<Vec<f32>>
                embeddings
                    .to_vec2::<f32>()
                    .map_err(|e| format!("Failed to convert batch embeddings to vec: {}", e))
            })
            .await
            .map_err(|e| {
                Box::new(std::io::Error::other(
                    format!("Spawn blocking failed: {}", e),
                )) as Box<dyn std::error::Error + Send + Sync>
            })?
            .map_err(|e| Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error + Send + Sync>)?;

            Ok(embeddings_vec)
        })
    }

    fn embedding_dimension(&self) -> usize {
        self.config.embed_head.out_features
    }

    fn recommended_batch_size(&self) -> usize {
        match self.variant {
            ModelVariant::Large => 8,
            ModelVariant::Small => 16,
        }
    }

    fn max_batch_size(&self) -> usize {
        match self.variant {
            ModelVariant::Large => 32,
            ModelVariant::Small => 64,
        }
    }
}
