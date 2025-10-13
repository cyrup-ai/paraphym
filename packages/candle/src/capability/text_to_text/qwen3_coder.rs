//! Provides streaming completion capabilities using local Qwen3-Coder-30B models
//! with zero allocation patterns and AsyncStream streaming.
//!
//! This implementation uses the Candle ML framework for local model inference,
//! specifically targeting Qwen architecture models optimized for code generation.

use std::num::NonZeroU32;
use std::path::Path;
use std::sync::Arc;

use candle_core::DType;
use candle_core::quantized::gguf_file;
use candle_transformers::models::llama::LlamaConfig;
use ystream::AsyncStream;
// SIMD optimizations for high-performance inference
use paraphym_simd::get_cpu_features;
use serde::{Deserialize, Serialize};

use crate::core::{Engine, EngineConfig};

use crate::domain::completion::{CandleCompletionChunk, CandleCompletionParams};
use crate::domain::context::CandleStringChunk;
use crate::domain::model::{info::CandleModelInfo, traits::CandleModel};
use crate::domain::prompt::CandlePrompt;

/// Builder trait for Qwen3 Coder completion providers
pub trait BuilderCandleQwen3CoderModel: Send + Sync + 'static {
    // Default implementations for all builders
}

/// High-performance Qwen3 Coder-30B provider for local inference using Candle
///
/// Provides streaming text generation capabilities using the Qwen3-Coder-30B-A3B-Instruct model
/// with automatic model downloading via ProgressHub.
#[derive(Debug, Clone)]
pub struct CandleQwen3CoderModel {
    /// Model cache directory path
    model_path: String,
    /// GGUF model file path
    gguf_file_path: String,
    /// Model configuration for inference
    model_config: LlamaConfig,
    /// Engine for orchestration and stream conversion
    engine: Arc<Engine>,
}

impl CandleQwen3CoderModel {
    /// Create new Qwen3 Coder provider with automatic model download
    ///
    /// This method automatically downloads the Qwen3-Coder-30B model from HuggingFace
    /// using ProgressHub and returns a provider ready for inference.
    ///
    /// # Example
    /// ```rust
    /// let provider = CandleQwen3CoderModel::new().await?;
    /// ```
    ///
    /// # Errors
    /// Returns error if model download fails or model loading fails
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        use hf_hub::api::tokio::Api;

        // Create HuggingFace API instance
        let api = Api::new().map_err(|e| {
            Box::<dyn std::error::Error + Send + Sync>::from(format!(
                "Failed to create HF API: {}",
                e
            ))
        })?;

        // Get the model repository
        let repo = api.model(QWEN3_CODER_MODEL_INFO.registry_key.to_string());

        // List files in the repo to find GGUF file
        let repo_info = repo.info().await.map_err(|e| {
            Box::<dyn std::error::Error + Send + Sync>::from(format!(
                "Failed to get repo info: {}",
                e
            ))
        })?;

        // Find a GGUF file (prefer Q4_K_M quantization)
        let gguf_filename = repo_info
            .siblings
            .iter()
            .filter(|f| f.rfilename.ends_with(".gguf"))
            .find(|f| f.rfilename.contains("Q4_K_M"))
            .or_else(|| {
                // Fallback to any .gguf file if Q4_K_M not found
                repo_info
                    .siblings
                    .iter()
                    .find(|f| f.rfilename.ends_with(".gguf"))
            })
            .ok_or_else(|| {
                Box::<dyn std::error::Error + Send + Sync>::from("No GGUF file found in repository")
            })?
            .rfilename
            .clone();

        // Download GGUF file
        let gguf_path = repo.get(&gguf_filename).await.map_err(|e| {
            Box::<dyn std::error::Error + Send + Sync>::from(format!(
                "Failed to download GGUF file: {}",
                e
            ))
        })?;

        // Download tokenizer
        let _tokenizer_path = repo.get("tokenizer.json").await.map_err(|e| {
            Box::<dyn std::error::Error + Send + Sync>::from(format!(
                "Failed to download tokenizer: {}",
                e
            ))
        })?;

        // Extract model directory from GGUF file path
        let model_cache_dir = gguf_path
            .parent()
            .ok_or_else(|| {
                Box::<dyn std::error::Error + Send + Sync>::from(
                    "Could not determine model directory",
                )
            })?
            .to_str()
            .ok_or_else(|| {
                Box::<dyn std::error::Error + Send + Sync>::from("Invalid model directory path")
            })?
            .to_string();

        Self::from_gguf(
            model_cache_dir,
            gguf_path
                .to_str()
                .ok_or_else(|| {
                    Box::<dyn std::error::Error + Send + Sync>::from("Invalid GGUF file path")
                })?
                .to_string(),
        )
    }

    /// Create provider from GGUF file with metadata extraction for Qwen3-Coder
    ///
    /// This method reads the GGUF file metadata to extract real Qwen3-Coder model configuration
    /// instead of using hardcoded values, ensuring accurate model parameters for code generation.
    /// All configuration values come from QWEN3_CODER_MODEL_INFO (self.info()).
    #[inline]
    pub fn from_gguf(
        model_cache_dir: String,
        gguf_file_path: String,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Log SIMD capabilities for performance debugging
        let cpu_info = get_cpu_features();
        log::info!(
            "Qwen3 Coder Provider initialized with SIMD support: {} (vector width: {} elements)",
            cpu_info.has_simd(),
            cpu_info.vector_width()
        );

        // Get configuration from ModelInfo
        let max_context = QWEN3_CODER_MODEL_INFO
            .max_input_tokens
            .map(|t| t.get())
            .unwrap_or(32768);
        let default_temperature = QWEN3_CODER_MODEL_INFO.default_temperature.unwrap_or(0.7);
        let info_vocab_size = QWEN3_CODER_MODEL_INFO.vocab_size.unwrap_or(151936);

        // Read GGUF file metadata for real model configuration
        let mut file = std::fs::File::open(&gguf_file_path)?;
        let content = gguf_file::Content::read(&mut file)?;

        // Extract Qwen3-specific metadata with Llama fallbacks - zero allocation parsing
        let hidden_size = content
            .metadata
            .get("qwen3.embedding_length")
            .or_else(|| content.metadata.get("llama.embedding_length"))
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as usize)
            .unwrap_or(8192); // Fallback for Qwen3-Coder-30B

        let intermediate_size = content
            .metadata
            .get("qwen3.feed_forward_length")
            .or_else(|| content.metadata.get("llama.feed_forward_length"))
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as usize)
            .unwrap_or(29568); // Fallback for Qwen3-Coder-30B FFN

        let num_hidden_layers = content
            .metadata
            .get("qwen3.block_count")
            .or_else(|| content.metadata.get("llama.block_count"))
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as usize)
            .unwrap_or(64); // Fallback for Qwen3-Coder-30B layers

        let num_attention_heads = content
            .metadata
            .get("qwen3.attention.head_count")
            .or_else(|| content.metadata.get("llama.attention.head_count"))
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as usize)
            .unwrap_or(64); // Fallback for Qwen3-Coder-30B heads

        let num_key_value_heads = content
            .metadata
            .get("qwen3.attention.head_count_kv")
            .or_else(|| content.metadata.get("llama.attention.head_count_kv"))
            .and_then(|v| v.to_u64().ok())
            .map(|v| Some(v as usize))
            .unwrap_or(Some(8)); // Fallback for Qwen3-Coder-30B GQA

        let rope_theta = content
            .metadata
            .get("qwen3.rope.freq_base")
            .or_else(|| content.metadata.get("llama.rope.freq_base"))
            .and_then(|v| v.to_f64().ok())
            .unwrap_or(1000000.0) as f32; // Qwen3 uses higher rope_theta than standard Llama

        // Extract vocab_size from metadata or use ModelInfo default
        let vocab_size = content
            .metadata
            .get("tokenizer.ggml.token_count")
            .or_else(|| content.metadata.get("qwen3.vocab_size"))
            .or_else(|| content.metadata.get("llama.vocab_size"))
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as usize)
            .unwrap_or(info_vocab_size as usize);

        // Extract Qwen3-specific token IDs with fallbacks
        let bos_token_id = content
            .metadata
            .get("tokenizer.ggml.bos_token_id")
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as u32)
            .unwrap_or(151643); // Qwen3 BOS token

        let eos_token_id = content
            .metadata
            .get("tokenizer.ggml.eos_token_id")
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as u32)
            .unwrap_or(151645); // Qwen3 EOS token

        // Create model configuration with real GGUF metadata
        let model_config = LlamaConfig {
            vocab_size,
            hidden_size,
            intermediate_size,
            num_hidden_layers,
            num_attention_heads,
            num_key_value_heads,
            max_position_embeddings: max_context as usize,
            rms_norm_eps: 1e-6,
            rope_theta,
            bos_token_id: Some(bos_token_id),
            eos_token_id: Some(candle_transformers::models::llama::LlamaEosToks::Single(
                eos_token_id,
            )),
            rope_scaling: None,
            tie_word_embeddings: Some(false),
        };

        // Log extracted configuration for debugging
        log::debug!(
            "Extracted GGUF metadata for Qwen3-Coder: hidden_size={}, layers={}, heads={}, kv_heads={:?}, rope_theta={}",
            hidden_size,
            num_hidden_layers,
            num_attention_heads,
            num_key_value_heads,
            rope_theta
        );

        // Create engine configuration using ModelInfo values
        let engine_config = EngineConfig::new("qwen3-coder", "candle-qwen")
            .with_streaming()
            .with_max_tokens(max_context)
            .with_temperature(default_temperature as f32);

        let engine = Arc::new(Engine::new(engine_config)?);

        Ok(Self {
            model_path: model_cache_dir,
            gguf_file_path,
            model_config,
            engine,
        })
    }

    /// Get the model path
    #[inline]
    pub fn model_path(&self) -> &str {
        &self.model_path
    }
}

// Static model info for Qwen3-Coder-30B
pub static QWEN3_CODER_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::Unsloth,
    name: "qwen3-coder-30b-instruct",
    registry_key: "unsloth/Qwen3-Coder-30B-A3B-Instruct-GGUF",
    quantization_url: None,
    max_input_tokens: NonZeroU32::new(32768), // 32K context
    max_output_tokens: NonZeroU32::new(8192),
    input_price: None, // Local model - no pricing
    output_price: None,
    supports_vision: false,
    supports_function_calling: true,
    supports_streaming: true,
    supports_embeddings: false,
    requires_max_tokens: false,
    supports_thinking: false,
    optimal_thinking_budget: None,
    system_prompt_prefix: None,
    real_name: None,
    model_type: None,
    model_id: "qwen-coder",
    quantization: "Q4_0",
    patch: None,
    embedding_dimension: None,
    vocab_size: Some(151936),
    image_size: None,
    image_mean: None,
    image_std: None,
    default_temperature: Some(0.7),
    default_top_k: Some(50),
    default_top_p: Some(0.9),
    supports_kv_cache: true,
    supports_flash_attention: false,
    use_bf16: false,
    default_steps: None,
    default_guidance_scale: None,
    time_shift: None,
    est_memory_allocation_mb: 0,
};

impl crate::capability::traits::TextToTextCapable for CandleQwen3CoderModel {
    fn prompt(
        &self,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> AsyncStream<CandleCompletionChunk> {
        // Clone data needed for the generation closure
        let engine = Arc::clone(&self.engine);
        let model_path = self.model_path.clone();
        let gguf_file_path = self.gguf_file_path.clone();
        let model_config = self.model_config.clone();

        // Get configuration from ModelInfo
        let max_context = self
            .info()
            .max_input_tokens
            .map(|t| t.get())
            .unwrap_or(32768);
        let _use_kv_cache = self.info().supports_kv_cache;
        let _vocab_size = self.info().vocab_size.unwrap_or(151936);

        // Create SIMD-optimized SamplingConfig from params
        // Extract top_k and top_p with priority: params > ModelInfo > None
        // This allows runtime override via additional_params while respecting ModelInfo defaults
        let top_k = params
            .additional_params
            .as_ref()
            .and_then(|p| p.get("top_k"))
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .or(self.info().default_top_k.map(|k| k as usize));

        let top_p = params
            .additional_params
            .as_ref()
            .and_then(|p| p.get("top_p"))
            .and_then(|v| v.as_f64())
            .or(self.info().default_top_p);

        // Build sampling config with extracted parameters
        let mut sampling_config =
            crate::core::generation::SamplingConfig::new(params.temperature as f32);

        if let Some(k) = top_k {
            sampling_config = sampling_config.with_top_k(k);
        }
        if let Some(p) = top_p {
            sampling_config = sampling_config.with_top_p(p);
        }

        sampling_config = sampling_config
            .with_repetition_penalty(1.0)
            .with_frequency_penalty(0.0)
            .with_presence_penalty(0.0);

        // Format prompt
        let prompt_text = format!("User: {}\nAssistant: ", prompt);
        let max_tokens = params.max_tokens.map(|n| n.get()).unwrap_or(1000);

        // Use Engine's coordinate_generation for automatic metrics and stream conversion
        engine.coordinate_generation(move || {
            use crate::core::ModelConfig as CandleConfig;
            use crate::core::generation::{
                generator::TextGenerator, models::CandleQuantizedLlamaModel, tokens::SpecialTokens,
            };
            use candle_core::Device;
            use std::sync::Arc;
            use tokenizers::Tokenizer;

            // Load device (prefer GPU if available)
            let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
                log::warn!("Device detection failed: {}. Using CPU.", e);
                Device::Cpu
            });

            // Load tokenizer - return error stream on failure
            let tokenizer = match Tokenizer::from_file(format!("{}/tokenizer.json", model_path)) {
                Ok(t) => t,
                Err(e) => {
                    return AsyncStream::with_channel(move |sender| {
                        let _ = sender.send(CandleStringChunk(format!(
                            "ERROR: Failed to load tokenizer: {}",
                            e
                        )));
                    });
                }
            };

            // Create model configuration for the quantized model
            let candle_model_config = Arc::new(
                CandleConfig::new(
                    &gguf_file_path,
                    format!("{}/tokenizer.json", model_path),
                    crate::core::ModelArchitecture::Llama(
                        candle_transformers::models::llama::Config {
                            hidden_size: model_config.hidden_size,
                            intermediate_size: model_config.intermediate_size,
                            vocab_size: model_config.vocab_size,
                            num_hidden_layers: model_config.num_hidden_layers,
                            num_attention_heads: model_config.num_attention_heads,
                            num_key_value_heads: model_config
                                .num_key_value_heads
                                .unwrap_or(model_config.num_attention_heads),
                            use_flash_attn: false,
                            rms_norm_eps: model_config.rms_norm_eps,
                            rope_theta: model_config.rope_theta,
                            bos_token_id: model_config.bos_token_id,
                            eos_token_id: model_config.eos_token_id.clone(),
                            rope_scaling: model_config.rope_scaling.clone(),
                            max_position_embeddings: model_config.max_position_embeddings,
                            tie_word_embeddings: model_config.tie_word_embeddings.unwrap_or(false),
                        },
                    ),
                    "qwen3-coder",
                    "qwen3-coder",
                )
                .with_vocab_size(model_config.vocab_size)
                .with_context_length(max_context as usize)
                .with_dtype(DType::F16), // GGUF models use F16
            );

            // Load the real quantized model - return error stream on failure
            let quantized_model = match CandleQuantizedLlamaModel::from_gguf_path(
                &gguf_file_path,
                device.clone(),
                candle_model_config,
            ) {
                Ok(model) => model,
                Err(e) => {
                    return AsyncStream::with_channel(move |sender| {
                        let _ = sender.send(CandleStringChunk(format!(
                            "ERROR: Failed to load quantized model: {}",
                            e
                        )));
                    });
                }
            };

            // Create TextGenerator with real model
            let text_generator = TextGenerator::new(
                Box::new(quantized_model),
                tokenizer,
                device,
                sampling_config,
            );

            // Set up special tokens
            let special_tokens = SpecialTokens {
                bos_token_id: Some(model_config.bos_token_id.unwrap_or(151643)), // Qwen3 BOS
                eos_token_id: match &model_config.eos_token_id {
                    Some(candle_transformers::models::llama::LlamaEosToks::Single(id)) => Some(*id),
                    _ => Some(151645), // Qwen3 EOS
                },
                pad_token_id: None,
            };

            // Convert u64 to u32, capping at u32::MAX if necessary
            let max_tokens_u32 = max_tokens.try_into().unwrap_or_else(|_| {
                log::warn!(
                    "max_tokens value {} exceeds u32::MAX, capping at {}",
                    max_tokens,
                    u32::MAX
                );
                u32::MAX
            });

            // Generate and return text stream - Engine handles conversion to CandleCompletionChunk
            text_generator.generate(prompt_text, max_tokens_u32, special_tokens)
        })
    }
}

impl CandleModel for CandleQwen3CoderModel {
    #[inline]
    fn info(&self) -> &'static CandleModelInfo {
        &QWEN3_CODER_MODEL_INFO
    }
}

/// Qwen3 Coder completion request format for HTTP API compatibility
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct CandleQwenCompletionRequest {
    prompt: String,
    temperature: f64,
    max_tokens: u64,
    stream: bool,
    model: String,
}

/// Validate that the model path exists and is accessible
///
/// # Errors
/// Returns error if the path does not exist or is not accessible
#[allow(dead_code)]
fn validate_model_path(path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let model_path = Path::new(path);

    if !model_path.exists() {
        return Err(format!("Model path does not exist: {}", path).into());
    }

    if !model_path.is_dir() && !model_path.is_file() {
        return Err(format!("Model path is neither file nor directory: {}", path).into());
    }

    Ok(())
}

/// Loaded Qwen3 Coder model that keeps resources in memory for worker threads
///
/// This model pre-loads the tokenizer and device configuration, avoiding
/// disk I/O on every request. The GGUF model is still loaded lazily due to size.
#[derive(Clone, Debug)]
pub struct LoadedQwen3CoderModel {
    tokenizer: tokenizers::Tokenizer,
    gguf_file_path: String,
    model_path: String,
    device: candle_core::Device,
    model_config: LlamaConfig,
    engine: Arc<Engine>,
    max_context: u64,
}

impl LoadedQwen3CoderModel {
    /// Load model resources into memory (called once per worker)
    ///
    /// This method loads the tokenizer and detects the device once,
    /// storing them for reuse across multiple requests.
    pub fn load(
        base: &CandleQwen3CoderModel,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Get file paths
        let gguf_file_path = std::path::PathBuf::from(&base.gguf_file_path);
        let tokenizer_path = std::path::PathBuf::from(&base.model_path).join("tokenizer.json");

        if !tokenizer_path.exists() {
            return Err(
                Box::from(format!("Tokenizer file not found: {:?}", tokenizer_path))
                    as Box<dyn std::error::Error + Send + Sync>,
            );
        }

        let model_path = base.model_path.clone();

        // Load device (prefer GPU if available)
        let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
            log::warn!("Device detection failed: {}. Using CPU.", e);
            candle_core::Device::Cpu
        });

        // Load tokenizer
        let tokenizer = tokenizers::Tokenizer::from_file(&tokenizer_path).map_err(|e| {
            Box::from(format!("Failed to load tokenizer: {}", e))
                as Box<dyn std::error::Error + Send + Sync>
        })?;

        let max_context = QWEN3_CODER_MODEL_INFO
            .max_input_tokens
            .map(|t| t.get() as u64)
            .unwrap_or(32768);

        Ok(Self {
            tokenizer,
            gguf_file_path: gguf_file_path.to_string_lossy().to_string(),
            model_path,
            device,
            model_config: base.model_config.clone(),
            engine: Arc::clone(&base.engine),
            max_context,
        })
    }
}

impl crate::capability::traits::TextToTextCapable for LoadedQwen3CoderModel {
    fn prompt(
        &self,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> AsyncStream<CandleCompletionChunk> {
        // Clone pre-loaded resources for the generation closure
        let engine = Arc::clone(&self.engine);
        let gguf_file_path = self.gguf_file_path.clone();
        let model_path = self.model_path.clone();
        let device = self.device.clone();
        let tokenizer = self.tokenizer.clone(); // ✅ Clone pre-loaded tokenizer
        let model_config = self.model_config.clone();
        let max_context = self.max_context;

        // Extract top_k and top_p with priority: params > ModelInfo > None
        let top_k = params
            .additional_params
            .as_ref()
            .and_then(|p| p.get("top_k"))
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .or(QWEN3_CODER_MODEL_INFO.default_top_k.map(|k| k as usize));

        let top_p = params
            .additional_params
            .as_ref()
            .and_then(|p| p.get("top_p"))
            .and_then(|v| v.as_f64())
            .or(QWEN3_CODER_MODEL_INFO.default_top_p);

        // Build sampling config with extracted parameters
        let mut sampling_config =
            crate::core::generation::SamplingConfig::new(params.temperature as f32);

        if let Some(k) = top_k {
            sampling_config = sampling_config.with_top_k(k);
        }
        if let Some(p) = top_p {
            sampling_config = sampling_config.with_top_p(p);
        }

        sampling_config = sampling_config
            .with_repetition_penalty(1.0)
            .with_frequency_penalty(0.0)
            .with_presence_penalty(0.0);

        // Format prompt
        let prompt_text = format!("User: {}\nAssistant: ", prompt);
        let max_tokens = params.max_tokens.map(|n| n.get()).unwrap_or(1000);

        // Use Engine's coordinate_generation for automatic metrics and stream conversion
        engine.coordinate_generation(move || {
            use crate::core::ModelConfig as CandleConfig;
            use crate::core::generation::{
                generator::TextGenerator, models::CandleQuantizedLlamaModel, tokens::SpecialTokens,
            };
            use std::sync::Arc;

            // Create model configuration for the quantized model
            let candle_model_config = Arc::new(
                CandleConfig::new(
                    &gguf_file_path,
                    format!("{}/tokenizer.json", model_path),
                    crate::core::ModelArchitecture::Llama(
                        candle_transformers::models::llama::Config {
                            hidden_size: model_config.hidden_size,
                            intermediate_size: model_config.intermediate_size,
                            vocab_size: model_config.vocab_size,
                            num_hidden_layers: model_config.num_hidden_layers,
                            num_attention_heads: model_config.num_attention_heads,
                            num_key_value_heads: model_config
                                .num_key_value_heads
                                .unwrap_or(model_config.num_attention_heads),
                            use_flash_attn: false,
                            rms_norm_eps: model_config.rms_norm_eps,
                            rope_theta: model_config.rope_theta,
                            bos_token_id: model_config.bos_token_id,
                            eos_token_id: model_config.eos_token_id.clone(),
                            rope_scaling: model_config.rope_scaling.clone(),
                            max_position_embeddings: model_config.max_position_embeddings,
                            tie_word_embeddings: model_config.tie_word_embeddings.unwrap_or(false),
                        },
                    ),
                    "qwen-coder",
                    "qwen-coder",
                )
                .with_vocab_size(model_config.vocab_size)
                .with_context_length(max_context as usize)
                .with_dtype(DType::F16),
            );

            // Load the quantized model - return error stream on failure
            let quantized_model = match CandleQuantizedLlamaModel::from_gguf_path(
                &gguf_file_path,
                device.clone(),
                candle_model_config,
            ) {
                Ok(model) => model,
                Err(e) => {
                    return AsyncStream::with_channel(move |sender| {
                        let _ = sender.send(CandleStringChunk(format!(
                            "ERROR: Failed to load quantized model: {}",
                            e
                        )));
                    });
                }
            };

            // Create TextGenerator with real model and pre-loaded tokenizer
            let text_generator = TextGenerator::new(
                Box::new(quantized_model),
                tokenizer, // ✅ Use pre-loaded tokenizer (no disk I/O)
                device,
                sampling_config,
            );

            // Set up special tokens
            let special_tokens = SpecialTokens {
                bos_token_id: Some(model_config.bos_token_id.unwrap_or(1)),
                eos_token_id: match &model_config.eos_token_id {
                    Some(candle_transformers::models::llama::LlamaEosToks::Single(id)) => Some(*id),
                    _ => Some(2),
                },
                pad_token_id: None,
            };

            // Convert u64 to u32, capping at u32::MAX if necessary
            let max_tokens_u32 = max_tokens.try_into().unwrap_or_else(|_| {
                log::warn!(
                    "max_tokens value {} exceeds u32::MAX, capping at {}",
                    max_tokens,
                    u32::MAX
                );
                u32::MAX
            });

            // Generate and return text stream
            text_generator.generate(prompt_text, max_tokens_u32, special_tokens)
        })
    }
}

impl CandleModel for LoadedQwen3CoderModel {
    #[inline]
    fn info(&self) -> &'static CandleModelInfo {
        &QWEN3_CODER_MODEL_INFO
    }
}
