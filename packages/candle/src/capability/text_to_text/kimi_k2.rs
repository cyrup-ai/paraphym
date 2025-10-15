//! Provides streaming completion capabilities using local Kimi K2 models
//! with zero allocation patterns and tokio stream streaming.
//!
//! This implementation uses the Candle ML framework for local model inference,
//! specifically targeting Llama-compatible models for high-performance text generation.

use std::num::NonZeroU32;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;

use candle_core::DType;
use candle_transformers::models::llama::LlamaConfig;
use tokio_stream::Stream;
use crate::async_stream;

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
    /// Model configuration for inference
    model_config: LlamaConfig,
    /// Engine for orchestration and stream conversion
    engine: Arc<Engine>,
}

impl CandleKimiK2Model {
    /// Create new Kimi K2 provider
    #[inline]
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Get configuration from ModelInfo
        let max_context = KIMI_K2_MODEL_INFO
            .max_input_tokens
            .map(|t| t.get())
            .unwrap_or(131072);
        let vocab_size = KIMI_K2_MODEL_INFO.vocab_size.unwrap_or(32000);
        let default_temperature = KIMI_K2_MODEL_INFO.default_temperature.unwrap_or(0.7);

        // Create model configuration for Kimi K2 (Llama-based architecture)
        let model_config = LlamaConfig {
            vocab_size: vocab_size as usize,
            hidden_size: 4096,
            intermediate_size: 11008,
            num_hidden_layers: 32,
            num_attention_heads: 32,
            num_key_value_heads: Some(32),
            max_position_embeddings: max_context as usize,
            rms_norm_eps: 1e-6,
            rope_theta: 10000.0,
            bos_token_id: Some(1),
            eos_token_id: Some(candle_transformers::models::llama::LlamaEosToks::Single(2)),
            rope_scaling: None,
            tie_word_embeddings: Some(false),
        };

        // Create engine configuration using ModelInfo values
        let engine_config = EngineConfig::new("kimi-k2", "candle-kimi")
            .with_streaming()
            .with_max_tokens(max_context)
            .with_temperature(default_temperature as f32);

        let engine = Arc::new(Engine::new(engine_config)?);

        Ok(Self {
            model_config,
            engine,
        })
    }

    // Helper methods removed - configuration now comes from ModelInfo
}

impl crate::capability::traits::TextToTextCapable for CandleKimiK2Model {
    fn prompt(
        &self,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> Pin<Box<dyn Stream<Item = CandleCompletionChunk> + Send>> {
        // Get file paths BEFORE the closure (self is available here)
        let gguf_file_path = match self.huggingface_file(self.info().registry_key, "*.gguf") {
            Ok(p) => p,
            Err(e) => {
                return Box::pin(async_stream::spawn_stream(move |tx| async move {
                    let _ = tx.send(CandleCompletionChunk::Error(format!(
                        "Failed to get GGUF file: {}",
                        e
                    )));
                }));
            }
        };

        let tokenizer_path = match self.huggingface_file(self.info().registry_key, "tokenizer.json")
        {
            Ok(p) => p,
            Err(e) => {
                return Box::pin(async_stream::spawn_stream(move |tx| async move {
                    let _ = tx.send(CandleCompletionChunk::Error(format!(
                        "Failed to get tokenizer file: {}",
                        e
                    )));
                }));
            }
        };

        // Extract model directory from tokenizer path
        let model_path = match tokenizer_path.parent() {
            Some(p) => p.to_string_lossy().to_string(),
            None => {
                return Box::pin(async_stream::spawn_stream(move |tx| async move {
                    let _ = tx.send(CandleCompletionChunk::Error(
                        "Failed to determine model directory".to_string(),
                    ));
                }));
            }
        };

        // Convert gguf_file_path to string
        let gguf_file_path = gguf_file_path.to_string_lossy().to_string();

        // Clone engine Arc for the coordinate_generation call
        let engine = Arc::clone(&self.engine);
        
        // Clone data needed for the generation closure
        let model_config = self.model_config.clone();

        // Get configuration from ModelInfo
        let max_context = self
            .info()
            .max_input_tokens
            .map(|t| t.get())
            .unwrap_or(131072);
        let _use_kv_cache = self.info().supports_kv_cache;
        let _vocab_size = self.info().vocab_size.unwrap_or(32000);

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
        Box::pin(engine.coordinate_generation(move || {
            use crate::core::ModelConfig as CandleConfig;
            use crate::core::generation::{
                generator::TextGenerator, models::CandleQuantizedLlamaModel, tokens::SpecialTokens,
            };
            use candle_core::Device;
            use std::sync::Arc;
            use tokenizers::Tokenizer;
            use tokio_stream::StreamExt;

            async_stream::spawn_stream(move |tx| async move {
                // Load device (prefer GPU if available)
                let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
                    log::warn!("Device detection failed: {}. Using CPU.", e);
                    Device::Cpu
                });

                // Load tokenizer - send error and return on failure
                let tokenizer = match Tokenizer::from_file(format!("{}/tokenizer.json", model_path)) {
                    Ok(t) => t,
                    Err(e) => {
                        let _ = tx.send(CandleStringChunk(format!(
                            "ERROR: Failed to load tokenizer: {}",
                            e
                        )));
                        return;
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
                    .with_context_length(max_context as usize)
                    .with_dtype(DType::F16), // GGUF models use F16
                );

                // Load the real quantized model - send error and return on failure
                let quantized_model = match CandleQuantizedLlamaModel::from_gguf_path(
                    &gguf_file_path,
                    device.clone(),
                    candle_model_config,
                ) {
                    Ok(model) => model,
                    Err(e) => {
                        let _ = tx.send(CandleStringChunk(format!(
                            "ERROR: Failed to load quantized model: {}",
                            e
                        )));
                        return;
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

                // Generate and forward text stream
                let mut stream = text_generator.generate(prompt_text, max_tokens_u32, special_tokens);
                while let Some(chunk) = stream.next().await {
                    if tx.send(chunk).is_err() {
                        break;
                    }
                }
            })
        }))
    }
}

// Static model info for Kimi-K2
pub static KIMI_K2_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::Unsloth,
    name: "kimi-k2-instruct",
    registry_key: "unsloth/Kimi-K2-Instruct-GGUF",
    quantization_url: None,
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
    embedding_dimension: None,
    vocab_size: Some(32000),
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

impl CandleModel for CandleKimiK2Model {
    #[inline]
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
        Self::new().unwrap_or_else(|e| panic!("Failed to initialize Kimi K2 model: {}", e))
    }
}

/// Loaded Kimi K2 model that keeps resources in memory for worker threads
///
/// This model pre-loads the tokenizer and device configuration, avoiding
/// disk I/O on every request. The GGUF model is still loaded lazily due to size.
#[derive(Clone, Debug)]
pub struct LoadedKimiK2Model {
    tokenizer: tokenizers::Tokenizer,
    gguf_file_path: String,
    model_path: String,
    device: candle_core::Device,
    model_config: LlamaConfig,
    engine: Arc<Engine>,
    max_context: u64,
}

impl LoadedKimiK2Model {
    /// Load model resources into memory (called once per worker)
    ///
    /// This method loads the tokenizer and detects the device once,
    /// storing them for reuse across multiple requests.
    pub fn load(
        base: &CandleKimiK2Model,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Get file paths
        let gguf_file_path = base
            .huggingface_file(base.info().registry_key, "*.gguf")
            .map_err(|e| {
                Box::from(format!("Failed to get GGUF file: {}", e))
                    as Box<dyn std::error::Error + Send + Sync>
            })?;

        let tokenizer_path = base
            .huggingface_file(base.info().registry_key, "tokenizer.json")
            .map_err(|e| {
                Box::from(format!("Failed to get tokenizer file: {}", e))
                    as Box<dyn std::error::Error + Send + Sync>
            })?;

        let model_path = tokenizer_path
            .parent()
            .ok_or_else(|| {
                Box::from("Failed to determine model directory")
                    as Box<dyn std::error::Error + Send + Sync>
            })?
            .to_string_lossy()
            .to_string();

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

        let max_context = base
            .info()
            .max_input_tokens
            .map(|t| t.get() as u64)
            .unwrap_or(131072);

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

impl crate::capability::traits::TextToTextCapable for LoadedKimiK2Model {
    fn prompt(
        &self,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> Pin<Box<dyn Stream<Item = CandleCompletionChunk> + Send>> {
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
            .or(KIMI_K2_MODEL_INFO.default_top_k.map(|k| k as usize));

        let top_p = params
            .additional_params
            .as_ref()
            .and_then(|p| p.get("top_p"))
            .and_then(|v| v.as_f64())
            .or(KIMI_K2_MODEL_INFO.default_top_p);

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
        Box::pin(engine.coordinate_generation(move || {
            use crate::core::ModelConfig as CandleConfig;
            use crate::core::generation::{
                generator::TextGenerator, models::CandleQuantizedLlamaModel, tokens::SpecialTokens,
            };
            use std::sync::Arc;
            use tokio_stream::StreamExt;

            async_stream::spawn_stream(move |tx| async move {
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
                .with_context_length(max_context as usize)
                .with_dtype(DType::F16), // GGUF models use F16
            );

                // Load the quantized model - send error and return on failure
                let quantized_model = match CandleQuantizedLlamaModel::from_gguf_path(
                    &gguf_file_path,
                    device.clone(),
                    candle_model_config,
                ) {
                    Ok(model) => model,
                    Err(e) => {
                        let _ = tx.send(CandleStringChunk(format!(
                            "ERROR: Failed to load quantized model: {}",
                            e
                        )));
                        return;
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

                // Generate and forward text stream
                let mut stream = text_generator.generate(prompt_text, max_tokens_u32, special_tokens);
                while let Some(chunk) = stream.next().await {
                    if tx.send(chunk).is_err() {
                        break;
                    }
                }
            })
        }))
    }
}

impl CandleModel for LoadedKimiK2Model {
    #[inline]
    fn info(&self) -> &'static CandleModelInfo {
        &KIMI_K2_MODEL_INFO
    }
}
