//! Provides streaming completion capabilities using local Kimi K2 models
//! with zero allocation patterns and AsyncStream streaming.
//!
//! This implementation uses the Candle ML framework for local model inference,
//! specifically targeting Llama-compatible models for high-performance text generation.

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

use crate::domain::model::{info::CandleModelInfo, traits::CandleModel};
use crate::domain::{
    completion::CandleCompletionParams,
    context::{CandleStringChunk, chunk::CandleCompletionChunk},
    prompt::CandlePrompt,
};

/// CandleKimiK2Model for local Kimi K2 model inference using Candle ML framework
#[derive(Debug, Clone)]
pub struct CandleKimiK2Model {
    /// Model cache directory path
    model_path: String,
    /// GGUF model file path
    gguf_file_path: String,
    /// Provider configuration
    config: CandleKimiK2Config,
    /// Model configuration for inference
    model_config: LlamaConfig,
    /// Engine for orchestration and stream conversion
    engine: Arc<Engine>,
}

/// Configuration for Kimi K2 model inference
#[derive(Debug, Clone)]
pub struct CandleKimiK2Config {
    /// Maximum context length for inference
    max_context: u32,
    /// Default temperature for sampling
    temperature: f64,
    /// Vocabulary size for tokenization
    vocab_size: u32,
    /// Enable key-value caching for faster inference
    use_kv_cache: bool,
    /// Data type for model weights (F16, BF16, F32)
    dtype: DType,
    /// Top-k sampling parameter (None = disabled)
    pub top_k: Option<usize>,
    /// Top-p nucleus sampling parameter (None = disabled)
    pub top_p: Option<f64>,
}

impl Default for CandleKimiK2Config {
    #[inline]
    fn default() -> Self {
        Self {
            max_context: 8192,
            temperature: 0.7,
            vocab_size: 32000,
            use_kv_cache: true,
            dtype: DType::F16,
            top_k: Some(50),
            top_p: Some(0.9),
        }
    }
}

impl CandleKimiK2Config {
    /// Get the temperature setting
    #[inline]
    pub fn temperature(&self) -> f64 {
        self.temperature
    }

    /// Set temperature for sampling
    #[inline]
    pub fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = temperature;
        self
    }

    /// Set maximum context length
    #[inline]
    pub fn with_max_context(mut self, max_context: u32) -> Self {
        self.max_context = max_context;
        self
    }

    /// Set data type for model weights
    #[inline]
    pub fn with_dtype(mut self, dtype: DType) -> Self {
        self.dtype = dtype;
        self
    }

    /// Enable or disable KV caching
    #[inline]
    pub fn with_kv_cache(mut self, use_kv_cache: bool) -> Self {
        self.use_kv_cache = use_kv_cache;
        self
    }

    /// Set top-k sampling parameter
    #[inline]
    pub fn with_top_k(mut self, top_k: Option<usize>) -> Self {
        self.top_k = top_k;
        self
    }

    /// Set top-p nucleus sampling parameter
    #[inline]
    pub fn with_top_p(mut self, top_p: Option<f64>) -> Self {
        self.top_p = top_p;
        self
    }
}

impl CandleKimiK2Model {
    /// Create new Kimi K2 provider with automatic model download
    ///
    /// This method automatically downloads the Kimi-K2 model from HuggingFace
    /// using ProgressHub and returns a provider ready for inference.
    ///
    /// # Example
    /// ```rust
    /// let provider = CandleKimiK2Model::new().await?;
    /// ```
    ///
    /// # Errors
    /// Returns error if model download fails or model loading fails
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let config = CandleKimiK2Config::default();
        Self::with_config_async(config).await
    }

    /// Create provider with custom configuration and automatic download
    pub async fn with_config_async(
        config: CandleKimiK2Config,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        use crate::domain::model::download::DownloadProviderFactory;

        // Use factory to get download provider
        let downloader = DownloadProviderFactory::create_default()?;

        // Download model files
        let result = downloader
            .download_model(
                "unsloth/Kimi-K2-Instruct-GGUF",
                vec!["*.gguf".to_string(), "tokenizer.json".to_string()],
                Some("Q4_K_M".to_string()), // Default quantization
            )
            .collect()
            .map_err(|e| {
                Box::<dyn std::error::Error + Send + Sync>::from(format!(
                    "Download task failed: {}",
                    e
                ))
            })??;

        // Find GGUF file from results
        let gguf_file = result
            .files
            .iter()
            .find(|f| f.extension().and_then(|s| s.to_str()) == Some("gguf"))
            .ok_or_else(|| {
                Box::<dyn std::error::Error + Send + Sync>::from("GGUF file not found in download")
            })?;

        Self::with_config_sync_gguf(
            result
                .cache_dir
                .to_str()
                .ok_or_else(|| {
                    Box::<dyn std::error::Error + Send + Sync>::from("Invalid cache directory path")
                })?
                .to_string(),
            gguf_file
                .to_str()
                .ok_or_else(|| {
                    Box::<dyn std::error::Error + Send + Sync>::from("Invalid GGUF file path")
                })?
                .to_string(),
            config,
        )
    }

    /// Create default provider instance for builder pattern
    /// Uses download provider to download model if not already cached
    pub fn default_for_builder() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let config = CandleKimiK2Config::default();
        let runtime = crate::runtime::shared_runtime()
            .ok_or("Runtime unavailable for provider initialization")?;
        runtime.block_on(Self::with_config_async(config))
    }

    /// Create provider with custom configuration and existing model path
    pub fn with_config_sync(
        model_path: String,
        config: CandleKimiK2Config,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Log SIMD capabilities for performance debugging
        let cpu_info = get_cpu_features();
        log::info!(
            "KimiK2 Provider initialized with SIMD support: {} (vector width: {} elements)",
            cpu_info.has_simd(),
            cpu_info.vector_width()
        );

        // Create model configuration for Kimi K2 (Llama-based architecture)
        // This is used only for configuration - actual model loading handled by core engine
        let model_config = LlamaConfig {
            vocab_size: config.vocab_size as usize,
            hidden_size: 4096,
            intermediate_size: 11008,
            num_hidden_layers: 32,
            num_attention_heads: 32,
            num_key_value_heads: Some(32),
            max_position_embeddings: config.max_context as usize,
            rms_norm_eps: 1e-6,
            rope_theta: 10000.0,
            bos_token_id: Some(1),
            eos_token_id: Some(candle_transformers::models::llama::LlamaEosToks::Single(2)),
            rope_scaling: None,
            tie_word_embeddings: Some(false),
        };

        // Create engine configuration
        let engine_config = EngineConfig::new("kimi-k2", "candle-kimi")
            .with_streaming()
            .with_max_tokens(config.max_context)
            .with_temperature(config.temperature as f32);

        let engine = Arc::new(Engine::new(engine_config)?);

        Ok(Self {
            model_path: model_path.clone(),
            gguf_file_path: model_path, // For sync method, assume model_path is the GGUF file
            config,
            model_config,
            engine,
        })
    }

    /// Create provider with custom configuration and GGUF metadata extraction
    ///
    /// This method reads the GGUF file metadata to extract real model configuration
    /// instead of using hardcoded values, ensuring accurate model parameters.
    #[inline]
    pub fn with_config_sync_gguf(
        model_cache_dir: String,
        gguf_file_path: String,
        config: CandleKimiK2Config,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Log SIMD capabilities for performance debugging
        let cpu_info = get_cpu_features();
        log::info!(
            "KimiK2 Provider initialized with SIMD support: {} (vector width: {} elements)",
            cpu_info.has_simd(),
            cpu_info.vector_width()
        );

        // Read GGUF file metadata for real model configuration
        let mut file = std::fs::File::open(&gguf_file_path)?;
        let content = gguf_file::Content::read(&mut file)?;

        // Extract metadata values with fallbacks - zero allocation parsing
        let hidden_size = content
            .metadata
            .get("llama.embedding_length")
            .or_else(|| content.metadata.get("kimi.embedding_length"))
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as usize)
            .unwrap_or(4096); // Fallback for Kimi K2

        let intermediate_size = content
            .metadata
            .get("llama.feed_forward_length")
            .or_else(|| content.metadata.get("kimi.feed_forward_length"))
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as usize)
            .unwrap_or(11008); // Fallback for Kimi K2

        let num_hidden_layers = content
            .metadata
            .get("llama.block_count")
            .or_else(|| content.metadata.get("kimi.block_count"))
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as usize)
            .unwrap_or(32); // Fallback for Kimi K2

        let num_attention_heads = content
            .metadata
            .get("llama.attention.head_count")
            .or_else(|| content.metadata.get("kimi.attention.head_count"))
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as usize)
            .unwrap_or(32); // Fallback for Kimi K2

        let num_key_value_heads = content
            .metadata
            .get("llama.attention.head_count_kv")
            .or_else(|| content.metadata.get("kimi.attention.head_count_kv"))
            .and_then(|v| v.to_u64().ok())
            .map(|v| Some(v as usize))
            .unwrap_or(Some(32)); // Fallback for Kimi K2

        let rope_theta = content
            .metadata
            .get("llama.rope.freq_base")
            .or_else(|| content.metadata.get("kimi.rope.freq_base"))
            .and_then(|v| v.to_f64().ok())
            .unwrap_or(10000.0) as f32; // Standard RoPE theta for Kimi K2

        // Extract vocab_size from metadata or use config default
        let vocab_size = content
            .metadata
            .get("tokenizer.ggml.token_count")
            .or_else(|| content.metadata.get("llama.vocab_size"))
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as usize)
            .unwrap_or(config.vocab_size as usize);

        // Create model configuration with real GGUF metadata
        let model_config = LlamaConfig {
            vocab_size,
            hidden_size,
            intermediate_size,
            num_hidden_layers,
            num_attention_heads,
            num_key_value_heads,
            max_position_embeddings: config.max_context as usize,
            rms_norm_eps: 1e-6,
            rope_theta,
            bos_token_id: Some(1),
            eos_token_id: Some(candle_transformers::models::llama::LlamaEosToks::Single(2)),
            rope_scaling: None,
            tie_word_embeddings: Some(false),
        };

        // Log extracted configuration for debugging
        log::debug!(
            "Extracted GGUF metadata for Kimi K2: hidden_size={}, layers={}, heads={}, rope_theta={}",
            hidden_size,
            num_hidden_layers,
            num_attention_heads,
            rope_theta
        );

        // Create engine configuration
        let engine_config = EngineConfig::new("kimi-k2", "candle-kimi")
            .with_streaming()
            .with_max_tokens(config.max_context)
            .with_temperature(config.temperature as f32);

        let engine = Arc::new(Engine::new(engine_config)?);

        Ok(Self {
            model_path: model_cache_dir,
            gguf_file_path,
            config,
            model_config,
            engine,
        })
    }

    // Unused helper functions removed - model loading now handled by core engine

    /// Get vocabulary size
    #[inline]
    pub fn vocab_size(&self) -> u32 {
        self.config.vocab_size
    }

    /// Set temperature for generation
    #[inline]
    pub fn with_temperature(mut self, temperature: f64) -> Self {
        self.config.temperature = temperature;
        self
    }

    /// Set maximum context length
    #[inline]
    pub fn with_max_context(mut self, max_context: u32) -> Self {
        self.config.max_context = max_context;
        self
    }

    /// Get tokenizer path (embedded in model file for GGUF)
    #[inline]
    pub fn tokenizer_path(&self) -> &str {
        // For GGUF models, tokenizer is embedded in the model file
        &self.model_path
    }

    /// Get maximum tokens (input tokens)
    #[inline]
    pub fn max_tokens(&self) -> u32 {
        self.config.max_context
    }

    /// Get temperature setting
    #[inline]
    pub fn temperature(&self) -> f64 {
        self.config.temperature
    }
}

impl crate::capability::traits::TextToTextCapable for CandleKimiK2Model {
    fn prompt(
        &self,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> AsyncStream<CandleCompletionChunk> {
        // Clone data needed for the generation closure
        let engine = Arc::clone(&self.engine);
        let model_path = self.model_path.clone();
        let gguf_file_path = self.gguf_file_path.clone();
        let config = self.config.clone();
        let model_config = self.model_config.clone();

        // Extract top_k and top_p with priority: params > config > None
        // This allows runtime override via additional_params while respecting config defaults
        let top_k = params
            .additional_params
            .as_ref()
            .and_then(|p| p.get("top_k"))
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .or(config.top_k);

        let top_p = params
            .additional_params
            .as_ref()
            .and_then(|p| p.get("top_p"))
            .and_then(|v| v.as_f64())
            .or(config.top_p);

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
                    "kimi-k2",
                    "kimi-k2",
                )
                .with_vocab_size(model_config.vocab_size)
                .with_context_length(config.max_context as usize)
                .with_dtype(config.dtype),
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

            // Generate and return text stream - Engine handles conversion to CandleCompletionChunk
            text_generator.generate(prompt_text, max_tokens_u32, special_tokens)
        })
    }
}

// Static model info for Kimi-K2
pub static KIMI_K2_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::Unsloth,
    name: "kimi-k2-instruct",
    registry_key: "unsloth/Kimi-K2-Instruct-GGUF",
    max_input_tokens: NonZeroU32::new(131072), // 128K context
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
    model_id: "kimi-k2",
    quantization: "Q4_0",
    patch: None,
};

impl CandleModel for CandleKimiK2Model {
    fn info(&self) -> &'static CandleModelInfo {
        &KIMI_K2_MODEL_INFO
    }
}

/// Kimi K2 completion request format for HTTP API compatibility
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct CandleKimiCompletionRequest {
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
pub fn validate_model_path(path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let model_path = Path::new(path);

    if !model_path.exists() {
        return Err(format!("Model path does not exist: {}", path).into());
    }

    if !model_path.is_dir() && !model_path.is_file() {
        return Err(format!("Model path is neither file nor directory: {}", path).into());
    }

    Ok(())
}

impl Default for CandleKimiK2Model {
    fn default() -> Self {
        let config = CandleKimiK2Config::default();
        // KimiK2 IS the default instance - use its configuration
        let model_config = LlamaConfig {
            vocab_size: config.vocab_size as usize,
            hidden_size: 4096,
            intermediate_size: 11008,
            num_hidden_layers: 32,
            num_attention_heads: 32,
            num_key_value_heads: Some(32),
            max_position_embeddings: config.max_context as usize,
            rms_norm_eps: 1e-6,
            rope_theta: 10000.0,
            bos_token_id: Some(1),
            eos_token_id: Some(candle_transformers::models::llama::LlamaEosToks::Single(2)),
            rope_scaling: None,
            tie_word_embeddings: Some(false),
        };

        // Create default engine configuration
        let engine_config = EngineConfig::new("kimi-k2", "candle-kimi")
            .with_streaming()
            .with_max_tokens(config.max_context)
            .with_temperature(config.temperature as f32);

        let engine = Arc::new(
            Engine::new(engine_config)
                // APPROVED BY DAVID MAPLE 09/30/2025: Panic is appropriate for initialization failure
                .expect("Engine configuration is valid and should never fail"),
        );

        Self {
            model_path: String::new(),
            gguf_file_path: String::new(),
            config,
            model_config,
            engine,
        }
    }
}
