//! Stella embedding provider for local inference using Candle ML framework
//!
//! This provider uses dunzhang/stella_en_400M_v5 or dunzhang/stella_en_1.5B_v5 models
//! for generating MRL-trained dimensional embeddings with ProgressHub download and Candle inference.
//!
//! Supports only trained MRL projection dimensions: 256, 768, 1024, 2048, 4096, 6144, 8192.
//! Architecture follows the real Candle EmbeddingModel pattern with native lm_head projections.

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::stella_en_v5::{Config, EmbedDim, EmbeddingModel, ModelVariant};
use std::num::NonZeroU32;
use tokenizers::{PaddingDirection, PaddingParams, PaddingStrategy, Tokenizer, TruncationParams};

use crate::domain::model::CandleModelInfo;
use crate::domain::model::traits::CandleModel;

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

    /// Detect model variant from registry_key
    fn detect_variant(&self) -> ModelVariant {
        let registry_key = self.info().registry_key;
        if registry_key.contains("1.5B") {
            ModelVariant::Large
        } else {
            ModelVariant::Small // Default to 400M
        }
    }

    /// Convert dimension to EmbedDim enum
    fn embed_dim(
        &self,
        dimension: u32,
    ) -> std::result::Result<EmbedDim, Box<dyn std::error::Error + Send + Sync>> {
        match dimension {
            256 => Ok(EmbedDim::Dim256),
            768 => Ok(EmbedDim::Dim768),
            1024 => Ok(EmbedDim::Dim1024),
            2048 => Ok(EmbedDim::Dim2048),
            4096 => Ok(EmbedDim::Dim4096),
            6144 => Ok(EmbedDim::Dim6144),
            8192 => Ok(EmbedDim::Dim8192),
            _ => Err(format!("Unsupported dimension: {}", dimension).into()),
        }
    }

    /// Format texts with task-specific instruction prefix following canonical Stella example
    fn format_with_instruction(&self, texts: &[&str], task: Option<&str>) -> Vec<String> {
        let instruct = match task {
            Some("s2p") => {
                "Given a web search query, retrieve relevant passages that answer the query."
            }
            Some("s2s") => "Retrieve semantically similar text.",
            Some("search_query") => {
                "Given a web search query, retrieve relevant passages that answer the query."
            } // Map to s2p
            Some("search_document") => {
                "Given a web search query, retrieve relevant passages that answer the query."
            } // Map to s2p
            Some("classification") => "Retrieve semantically similar text.", // Map to s2s
            Some("clustering") => "Retrieve semantically similar text.",     // Map to s2s
            Some("retrieval") => {
                "Given a web search query, retrieve relevant passages that answer the query."
            } // Map to s2p
            _ => "Given a web search query, retrieve relevant passages that answer the query.", // Default to s2p
        };

        texts
            .iter()
            .map(|text| format!("Instruct: {}\nQuery: {}", instruct, text))
            .collect()
    }
}

// Static model info for Stella
static STELLA_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::Dunzhang,
    name: "stella_en_400M_v5",
    registry_key: "dunzhang/stella_en_400M_v5",
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
    model_id: "stella-en-400m-v5",
    quantization: "none",
    patch: None,
    embedding_dimension: Some(1024),
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

impl CandleModel for StellaEmbeddingModel {
    fn info(&self) -> &'static CandleModelInfo {
        &STELLA_MODEL_INFO
    }
}

impl crate::capability::traits::TextEmbeddingCapable for StellaEmbeddingModel {
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

        // ═══════════════════════════════════════════════════════════════
        // STEP 1: Get ALL config from self.info() - SINGLE SOURCE OF TRUTH
        // ═══════════════════════════════════════════════════════════════

        let max_length = self
            .info()
            .max_input_tokens
            .ok_or("max_input_tokens missing in ModelInfo")?
            .get() as usize;

        let dimension = self
            .info()
            .embedding_dimension
            .ok_or("embedding_dimension missing in ModelInfo")? as usize;

        let variant = self.detect_variant();
        let embed_dim = self.embed_dim(dimension as u32)?;

        // ═══════════════════════════════════════════════════════════════
        // STEP 2: Auto-detect runtime values (ONLY these, nothing else)
        // ═══════════════════════════════════════════════════════════════

        let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
            log::warn!("Device detection failed: {}. Using CPU.", e);
            Device::Cpu
        });

        let dtype = if device.is_cuda() {
            DType::F16
        } else {
            DType::F32
        };

        // ═══════════════════════════════════════════════════════════════
        // STEP 3: Load files via huggingface_file() - NO manual paths
        // ═══════════════════════════════════════════════════════════════

        // Base model weights
        let base_weights = self.huggingface_file(self.info().registry_key, "model.safetensors").await?;

        // MRL projection head (dimension-specific)
        let projection_head = self.huggingface_file(
            self.info().registry_key,
            &format!("2_Dense_{}/model.safetensors", dimension),
        ).await?;

        // Tokenizer
        let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json").await?;

        // ═══════════════════════════════════════════════════════════════
        // STEP 4: Load tokenizer with variant-specific padding
        // ═══════════════════════════════════════════════════════════════

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

        // ═══════════════════════════════════════════════════════════════
        // STEP 5: Create Stella model config from detected variant
        // ═══════════════════════════════════════════════════════════════

        let stella_config = match variant {
            ModelVariant::Large => Config::new_1_5_b_v5(embed_dim),
            ModelVariant::Small => Config::new_400_m_v5(embed_dim),
        };

        // ═══════════════════════════════════════════════════════════════
        // STEP 6: Load model weights (base + projection head)
        // ═══════════════════════════════════════════════════════════════

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

        // ═══════════════════════════════════════════════════════════════
        // STEP 7: Run inference with task-specific formatting
        // ═══════════════════════════════════════════════════════════════

        // Format with instruction prefix
        let formatted_text = self.format_with_instruction(&[&text], task.as_deref())[0].clone();

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
        let variant = self.detect_variant();
        let embed_dim = self.embed_dim(dimension as u32)?;

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
        let base_weights = self.huggingface_file(self.info().registry_key, "model.safetensors").await?;
        let projection_head = self.huggingface_file(
            self.info().registry_key,
            &format!("2_Dense_{}/model.safetensors", dimension),
        ).await?;
        let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json").await?;

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
        let formatted_texts = self.format_with_instruction(&text_refs, task.as_deref());
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
        match self.detect_variant() {
            ModelVariant::Large => 8,
            ModelVariant::Small => 16,
        }
    }

    fn max_batch_size(&self) -> usize {
        match self.detect_variant() {
            ModelVariant::Large => 32,
            ModelVariant::Small => 64,
        }
    }
}

// ============================================================================
// LOADED MODEL WRAPPER (MPOOL_4.md)
// ============================================================================

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
/// - model: RefCell<EmbeddingModel> (interior mutability for &mut forward_norm)
/// - device: Device (CUDA or CPU)
/// - variant: ModelVariant (Large=1.5B or Small=400M)
/// - dimension: usize (embedding output dimension)
///
/// Loaded model with thread-safe interior mutability.
#[derive(Debug)]
pub struct LoadedStellaModel {
    tokenizer: Tokenizer,
    model: std::sync::Mutex<EmbeddingModel>,
    device: Device,
    config: Config,
    variant: ModelVariant,
}

impl crate::domain::model::traits::CandleModel for LoadedStellaModel {
    fn info(&self) -> &'static CandleModelInfo {
        &STELLA_MODEL_INFO
    }
}

impl LoadedStellaModel {
    /// Load model and tokenizer from disk once, returning loaded instance ready for inference.
    ///
    /// This extracts the loading logic from embed() (lines 136-255) so it can be called
    /// once during worker spawn instead of on every inference.
    pub async fn load(
        base_model: &StellaEmbeddingModel,
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

        let variant = base_model.detect_variant();
        let embed_dim = base_model.embed_dim(dimension as u32)?;

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
            model: std::sync::Mutex::new(model),
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

impl crate::capability::traits::TextEmbeddingCapable for LoadedStellaModel {
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
            // No I/O - use loaded state

        // Format with instruction prefix (using base model's static method)
        let formatted_text = StellaEmbeddingModel::format_with_instruction(
            &StellaEmbeddingModel {},
            &[&text],
            task.as_deref(),
        )[0]
        .clone();

        // Tokenize
        let tokens = self
            .tokenizer
            .encode(formatted_text, true)
            .map_err(|e| format!("Tokenization failed: {}", e))?;

        let input_ids = Tensor::new(tokens.get_ids(), &self.device)
            .map_err(|e| format!("Failed to create input tensor: {}", e))?;
        let attention_mask = Tensor::new(tokens.get_attention_mask(), &self.device)
            .map_err(|e| format!("Failed to create attention mask: {}", e))?
            .to_dtype(DType::U8)
            .map_err(|e| format!("Failed to convert mask dtype: {}", e))?;

        // Forward pass
        let mut model = self
            .model
            .lock()
            .map_err(|e| format!("Failed to acquire model lock: {}", e))?;
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

        // Extract first embedding
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
            // No I/O - use loaded state
            let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();

        // Format with instruction prefix
        let formatted_texts = StellaEmbeddingModel::format_with_instruction(
            &StellaEmbeddingModel {},
            &text_refs,
            task.as_deref(),
        );

        // Tokenize batch
        let encodings = self
            .tokenizer
            .encode_batch(formatted_texts, true)
            .map_err(|e| format!("Batch tokenization failed: {}", e))?;

        // Create batch tensors
        let ids_vecs: Vec<Vec<u32>> = encodings.iter().map(|e| e.get_ids().to_vec()).collect();
        let mask_vecs: Vec<Vec<u32>> = encodings
            .iter()
            .map(|e| e.get_attention_mask().to_vec())
            .collect();

        let input_ids = Tensor::new(ids_vecs, &self.device)
            .map_err(|e| format!("Failed to create batch input tensor: {}", e))?;
        let attention_mask = Tensor::new(mask_vecs, &self.device)
            .map_err(|e| format!("Failed to create batch attention mask: {}", e))?
            .to_dtype(DType::U8)
            .map_err(|e| format!("Failed to convert mask dtype: {}", e))?;

        // Forward pass
        let mut model = self
            .model
            .lock()
            .map_err(|e| format!("Failed to acquire model lock: {}", e))?;
        let embeddings = model
            .forward_norm(&input_ids, &attention_mask)
            .map_err(|e| format!("Stella batch forward pass failed: {}", e))?;

        // Convert to Vec<Vec<f32>>
        embeddings
            .to_vec2::<f32>()
            .map_err(|e| format!("Failed to convert batch embeddings to vec: {}", e).into())
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
