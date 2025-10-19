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
    context::chunk::CandleCompletionChunk,
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
        let default_temperature = KIMI_K2_MODEL_INFO.default_temperature.unwrap_or(0.0);

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
        let model = self.clone();
        let params = params.clone();

        Box::pin(async_stream::spawn_stream(move |tx| async move {
            // Get file paths inside async context
            let gguf_file_path = match model.huggingface_file(model.info().registry_key, "*.gguf").await {
                Ok(p) => p,
                Err(e) => {
                    let _ = tx.send(CandleCompletionChunk::Error(format!(
                        "Failed to get GGUF file: {}",
                        e
                    )));
                    return;
                }
            };

            let tokenizer_path = match model.huggingface_file(model.info().registry_key, "tokenizer.json").await
            {
                Ok(p) => p,
                Err(e) => {
                    let _ = tx.send(CandleCompletionChunk::Error(format!(
                        "Failed to get tokenizer file: {}",
                        e
                    )));
                    return;
                }
            };

            // Extract model directory from tokenizer path
            let _model_path = match tokenizer_path.parent() {
                Some(p) => p.to_string_lossy().to_string(),
                None => {
                    let _ = tx.send(CandleCompletionChunk::Error(
                        "Failed to determine model directory".to_string(),
                    ));
                    return;
                }
            };

            // Convert gguf_file_path to string
            let gguf_file_path = gguf_file_path.to_string_lossy().to_string();

            // Clone engine Arc for the coordinate_generation call
            let engine = Arc::clone(&model.engine);
            
            // Clone data needed for the generation closure
            let model_config = model.model_config.clone();

            // Get configuration from ModelInfo
            let max_context = model
                .info()
                .max_input_tokens
                .map(|t| t.get())
                .unwrap_or(131072);
            let _use_kv_cache = model.info().supports_kv_cache;
            let _vocab_size = model.info().vocab_size.unwrap_or(32000);

            // Extract top_k and top_p with priority: params > ModelInfo > None
            // This allows runtime override via additional_params while respecting ModelInfo defaults
            let top_k = params
                .additional_params
                .as_ref()
                .and_then(|p| p.get("top_k"))
                .and_then(|v| v.as_u64())
                .map(|v| v as usize)
                .or(model.info().default_top_k.map(|k| k as usize));

            let top_p = params
                .additional_params
                .as_ref()
                .and_then(|p| p.get("top_p"))
                .and_then(|v| v.as_f64())
                .or(model.info().default_top_p);

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
            
            // Convert u64 to u32, capping at u32::MAX if necessary
            let max_tokens_u32 = max_tokens.try_into().unwrap_or_else(|_| {
                log::warn!(
                    "max_tokens value {} exceeds u32::MAX, capping at {}",
                    max_tokens,
                    u32::MAX
                );
                u32::MAX
            });

            // Load device (prefer GPU if available)
            use candle_core::{Device, DType};
            let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
                log::warn!("Device detection failed: {}. Using CPU.", e);
                Device::Cpu
            });

            // Load tokenizer - CRITICAL: Use spawn_blocking for sync I/O
            // This prevents blocking the async runtime worker thread
            use tokenizers::Tokenizer;
            let tokenizer_path_clone = tokenizer_path.clone();
            let tokenizer = match tokio::task::spawn_blocking(move || {
                // Runs on dedicated blocking thread pool
                Tokenizer::from_file(&tokenizer_path_clone)
            }).await {
                // Double-Result pattern:
                // Outer Ok/Err: Did the spawned task complete?
                // Inner Ok/Err: Did the tokenizer load successfully?
                Ok(Ok(t)) => t,  // Task completed + tokenizer loaded successfully
                Ok(Err(e)) => {  // Task completed but tokenizer loading failed
                    let _ = tx.send(CandleCompletionChunk::Error(format!(
                        "Failed to load tokenizer: {}",
                        e
                    )));
                    return;
                }
                Err(e) => {  // Spawned task panicked or was cancelled
                    let _ = tx.send(CandleCompletionChunk::Error(format!(
                        "Failed to spawn blocking task: {}",
                        e
                    )));
                    return;
                }
            };

            // Create model configuration for the quantized model
            use crate::core::ModelConfig as CandleConfig;
            let candle_model_config = Arc::new(
                CandleConfig::new(
                    &gguf_file_path,
                    tokenizer_path.to_string_lossy().to_string(),
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

            // Load the quantized model
            use crate::core::generation::models::CandleQuantizedLlamaModel;
            let quantized_model = match CandleQuantizedLlamaModel::from_gguf_path(
                &gguf_file_path,
                device.clone(),
                candle_model_config,
            ).await {
                Ok(model) => model,
                Err(e) => {
                    let _ = tx.send(CandleCompletionChunk::Error(format!(
                        "Failed to load quantized model: {}",
                        e
                    )));
                    return;
                }
            };

            // Create TextGenerator
            use crate::core::generation::generator::TextGenerator;
            let text_generator = TextGenerator::new(
                Box::new(quantized_model),
                tokenizer,
                device,
                sampling_config,
            );

            // Set up special tokens
            use crate::core::generation::tokens::SpecialTokens;
            let special_tokens = SpecialTokens {
                bos_token_id: Some(model_config.bos_token_id.unwrap_or(1)),
                eos_token_id: match &model_config.eos_token_id {
                    Some(candle_transformers::models::llama::LlamaEosToks::Single(id)) => Some(*id),
                    _ => Some(2),
                },
                pad_token_id: None,
            };

            // Use Engine's coordinate_generation for automatic metrics and stream conversion
            let stream = engine.coordinate_generation(move || {
                text_generator.generate(prompt_text, max_tokens_u32, special_tokens)
            });

            // Forward chunks from the coordinate_generation stream
            use tokio_stream::StreamExt;
            tokio::pin!(stream);
            while let Some(chunk) = stream.next().await {
                let _ = tx.send(chunk);
            }
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
    default_temperature: Some(0.0),  // Greedy sampling for deterministic output
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
/// This model pre-loads the actual model into memory with safe async mutable access,
/// avoiding disk I/O on every request.
#[derive(Clone)]
pub struct LoadedKimiK2Model {
    /// The ACTUAL loaded model - cached in memory with Mutex for safe async mutable access
    model: Arc<tokio::sync::Mutex<crate::core::generation::models::CandleQuantizedLlamaModel>>,
    tokenizer: tokenizers::Tokenizer,
    device: candle_core::Device,
    engine: Arc<Engine>,
}

impl LoadedKimiK2Model {
    /// Load model resources into memory (called once per worker)
    ///
    /// This method loads EVERYTHING once: model, tokenizer, device.
    /// The model stays in memory for all subsequent requests.
    pub async fn load(
        base: &CandleKimiK2Model,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        log::info!("🔄 LoadedKimiK2Model::load() - Loading model into memory ONCE");
        
        // Get file paths
        let gguf_file_path = base
            .huggingface_file(base.info().registry_key, "*.gguf")
            .await
            .map_err(|e| {
                Box::from(format!("Failed to get GGUF file: {}", e))
                    as Box<dyn std::error::Error + Send + Sync>
            })?;

        let tokenizer_path = base
            .huggingface_file(base.info().registry_key, "tokenizer.json")
            .await
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

        // Load tokenizer - CRITICAL: Use spawn_blocking for sync I/O
        log::info!("📝 Loading tokenizer from {}", tokenizer_path.display());
        let tokenizer = tokio::task::spawn_blocking(move || {
            tokenizers::Tokenizer::from_file(&tokenizer_path)
        })
        .await
        .map_err(|e| {
            Box::from(format!("Failed to spawn blocking task: {}", e))
                as Box<dyn std::error::Error + Send + Sync>
        })?
        .map_err(|e| {
            Box::from(format!("Failed to load tokenizer: {}", e))
                as Box<dyn std::error::Error + Send + Sync>
        })?;

        let max_context = base
            .info()
            .max_input_tokens
            .map(|t| t.get() as u64)
            .unwrap_or(131072);

        // CRITICAL: Load the model ONCE and cache it
        log::info!("🔥 Loading Kimi K2 model from {} - THIS HAPPENS ONCE", gguf_file_path.display());
        
        // Create model configuration for the quantized model
        use crate::core::ModelConfig as CandleConfig;
        let model_config = base.model_config.clone();
        let gguf_file_path_str = gguf_file_path.to_string_lossy().to_string();
        
        let candle_model_config = Arc::new(
            CandleConfig::new(
                &gguf_file_path_str,
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
            .with_dtype(DType::F16),
        );

        let model = crate::core::generation::models::CandleQuantizedLlamaModel::from_gguf_path(
            &gguf_file_path_str,
            device.clone(),
            candle_model_config,
        ).await.map_err(|e| {
            Box::from(format!("Failed to load model: {}", e))
                as Box<dyn std::error::Error + Send + Sync>
        })?;
        
        log::info!("✅ Model loaded into memory! All future requests will reuse this cached model.");

        Ok(Self {
            model: Arc::new(tokio::sync::Mutex::new(model)),  // Cache the loaded model with Mutex for safe async access!
            tokenizer,
            device,
            engine: Arc::clone(&base.engine),
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
        let engine = self.engine.clone();
        let model = self.model.clone();  // ✅ Use CACHED model
        let device = self.device.clone();
        let tokenizer = self.tokenizer.clone(); // ✅ Clone pre-loaded tokenizer
        
        log::info!("🚀 Using CACHED model from memory - no loading needed!");

        // Build sampling config
        // Use temperature from params directly
        let temperature = params.temperature;

        // Extract additional params or use defaults
        let top_k = params
            .additional_params
            .as_ref()
            .and_then(|p| p.get("top_k"))
            .and_then(|v| v.as_u64())
            .map(|v| v as usize);

        let top_p = params
            .additional_params
            .as_ref()
            .and_then(|p| p.get("top_p"))
            .and_then(|v| v.as_f64())
            .or(KIMI_K2_MODEL_INFO.default_top_p);

        // Format prompt text
        let prompt_text = format!("User: {}\nAssistant: ", prompt);
        let max_tokens = params.max_tokens.map(|n| n.get()).unwrap_or(1000);

        // Use Engine's coordinate_generation for automatic metrics and stream conversion
        Box::pin(engine.coordinate_generation(move || {
            use crate::core::generation::{
                SamplingConfig, generator::TextGenerator,
                tokens::SpecialTokens,
                models::CandleModel as CandleModelTrait,
            };
            use tokio_stream::StreamExt;

            async_stream::spawn_stream(move |tx| async move {
                // Use CACHED model - NO LOADING!
                log::info!("✅ Using cached model from memory - no disk I/O!");

                // Get vocab_size from the model (need to lock mutex briefly)
                let vocab_size = {
                    let model_guard = model.lock().await;
                    model_guard.vocab_size()
                };

                // Build sampling config with extracted parameters
                let mut sampling_config = SamplingConfig::new(temperature as f32);

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

                // Create TextGenerator with CACHED model and pre-loaded tokenizer
                // Use SharedKimiModel wrapper to share the Arc<Mutex<Model>> across generate() calls
                let text_generator = TextGenerator::new(
                    Box::new(SharedKimiModel { 
                        model: model.clone(),
                        device: device.clone(),
                        vocab_size,
                    }),
                    tokenizer, // ✅ Use pre-loaded tokenizer (no disk I/O)
                    device,
                    sampling_config,
                );

                // Set up special tokens for Kimi K2
                let special_tokens = SpecialTokens {
                    bos_token_id: Some(1),
                    eos_token_id: Some(2),
                    pad_token_id: None,
                };

                // Convert max_tokens to u32
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

/// Wrapper to share Arc<Mutex<CandleQuantizedLlamaModel>> safely across generations
/// This provides safe async mutable access to the cached model
struct SharedKimiModel {
    model: Arc<tokio::sync::Mutex<crate::core::generation::models::CandleQuantizedLlamaModel>>,
    device: candle_core::Device,
    vocab_size: usize,
}

#[async_trait::async_trait]
impl crate::core::generation::models::CandleModel for SharedKimiModel {
    async fn forward(&mut self, input: &candle_core::Tensor, index_pos: usize) -> Result<candle_core::Tensor, crate::domain::model::error::CandleModelError> {
        // Lock the mutex to get mutable access to the model
        let mut model = self.model.lock().await;
        // Call the async forward method on the locked model
        model.forward(input, index_pos).await
    }

    fn device(&self) -> &candle_core::Device {
        &self.device
    }

    fn vocab_size(&self) -> usize {
        self.vocab_size
    }
}

impl std::fmt::Debug for LoadedKimiK2Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoadedKimiK2Model")
            .field("device", &self.device)
            .field("model", &"Arc<Mutex<CandleQuantizedLlamaModel>>")
            .finish()
    }
}

impl CandleModel for LoadedKimiK2Model {
    #[inline]
    fn info(&self) -> &'static CandleModelInfo {
        &KIMI_K2_MODEL_INFO
    }
}
