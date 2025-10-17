//! Phi-4-Reasoning Provider for local Phi-4-reasoning Q4_K_M model inference
//!
//! This provider implements the TextToTextCapable trait for the Phi-4-reasoning
//! model with integrated chain-of-thought reasoning capabilities.

use std::num::NonZeroU32;
use std::pin::Pin;
use std::sync::{Arc, OnceLock};

use async_trait::async_trait;
use tokio_stream::Stream;
use crate::async_stream;

use crate::core::Engine;
use crate::domain::model::{info::CandleModelInfo, traits::CandleModel};
use crate::domain::{
    completion::CandleCompletionParams, context::chunk::CandleCompletionChunk, prompt::CandlePrompt,
};

/// Chat template constant for Phi-4-reasoning
/// This template enforces the structured reasoning format with <think> tags
const CHAT_TEMPLATE: &str = r#"{% if messages|length == 0 or messages[0]['role'] != 'system' %}
{{'<|im_start|>system<|im_sep|>You are Phi, a language model trained by Microsoft to help users. Your role as an assistant involves thoroughly exploring questions through a systematic thinking process before providing the final precise and accurate solutions. This requires engaging in a comprehensive cycle of analysis, summarizing, exploration, reassessment, reflection, backtracing, and iteration to develop well-considered thinking process. Please structure your response into two main sections: Thought and Solution using the specified format: <think> {Thought section} </think> {Solution section}. In the Thought section, detail your reasoning process in steps. Each step should include detailed considerations such as analysing questions, summarizing relevant findings, brainstorming new ideas, verifying the accuracy of the current steps, refining any errors, and revisiting previous steps. In the Solution section, based on various attempts, explorations, and reflections from the Thought section, systematically present the final solution that you deem correct. The Solution section should be logical, accurate, and concise and detail necessary steps needed to reach the conclusion. Now, try to solve the following question through the above guidelines:<|im_end|>'}}
{% endif %}
{% for message in messages %}
{% if messages[0]['role'] == 'system' %}
{{'<|im_start|>system<|im_sep|>You are Phi, a language model trained by Microsoft to help users. Your role as an assistant involves thoroughly exploring questions through a systematic thinking process before providing the final precise and accurate solutions. This requires engaging in a comprehensive cycle of analysis, summarizing, exploration, reassessment, reflection, backtracing, and iteration to develop well-considered thinking process. Please structure your response into two main sections: Thought and Solution using the specified format: <think> {Thought section} </think> {Solution section}. In the Thought section, detail your reasoning process in steps. Each step should include detailed considerations such as analysing questions, summarizing relevant findings, brainstorming new ideas, verifying the accuracy of the current steps, refining any errors, and revisiting previous steps. In the Solution section, based on various attempts, explorations, and reflections from the Thought section, systematically present the final solution that you deem correct. The Solution section should be logical, accurate, and concise and detail necessary steps needed to reach the conclusion. Now, try to solve the following question through the above guidelines:<|im_end|>'}}
{% elif message['role'] == 'user' %}
{{'<|im_start|>user<|im_sep|>' + message['content'] + '<|im_end|>'}}
{% elif message['role'] == 'assistant' %}
{{'<|im_start|>assistant<|im_sep|>' + message['content'] + '<|im_end|>'}}
{% endif %}
{% endfor %}
{% if add_generation_prompt %}
{{ '<|im_start|>assistant<|im_sep|>' }}
{% endif %}"#;

/// Cached template environment to avoid recompiling template on every request
static TEMPLATE_ENV: OnceLock<minijinja::Environment<'static>> = OnceLock::new();

/// Get or initialize the cached template environment
///
/// Returns a static reference to the compiled template environment.
/// Template compilation happens once on first call, subsequent calls are zero-cost.
/// If template compilation fails (programming error), the error surfaces when get_template is called.
#[inline]
fn get_template_env() -> &'static minijinja::Environment<'static> {
    TEMPLATE_ENV.get_or_init(|| {
        let mut env = minijinja::Environment::new();
        if let Err(e) = env.add_template("chat", CHAT_TEMPLATE) {
            log::error!("Failed to compile chat template: {}", e);
        }
        env
    })
}

/// Chat message for template processing
#[derive(Debug, Clone, serde::Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

/// CandlePhi4ReasoningModel for local Phi-4-reasoning model inference using TextGenerator
#[derive(Debug, Clone)]
pub struct CandlePhi4ReasoningModel {
    engine: Engine,
}

impl Default for CandlePhi4ReasoningModel {
    fn default() -> Self {
        Self::new()
    }
}

impl CandlePhi4ReasoningModel {
    /// Create new Phi-4-Reasoning provider
    pub fn new() -> Self {
        Self {
            engine: Engine::default(),
        }
    }

    /// Apply chat template to format prompt with reasoning instructions
    fn apply_chat_template(prompt: &CandlePrompt) -> Result<String, String> {
        let env = get_template_env();

        let template = env
            .get_template("chat")
            .map_err(|e| format!("Failed to get chat template: {}", e))?;

        // Create message from prompt
        let message = ChatMessage {
            role: "user".to_string(),
            content: prompt.content().to_string(),
        };

        let messages = vec![message];
        let context = minijinja::context! {
            messages => messages,
            add_generation_prompt => true,
        };

        let rendered = template
            .render(context)
            .map_err(|e| format!("Failed to render chat template: {}", e))?;

        Ok(rendered)
    }
}

impl crate::capability::traits::TextToTextCapable for CandlePhi4ReasoningModel {
    fn prompt(
        &self,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> Pin<Box<dyn Stream<Item = CandleCompletionChunk> + Send>> {
        // Clone self for async context
        let model_clone = self.clone();

        // Build sampling config
        let temperature = if params.temperature != 1.0 {
            params.temperature
        } else {
            self.info().default_temperature.unwrap_or(0.7)
        };

        // Clone engine Arc for the coordinate_generation call
        let engine = self.engine.clone();

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
            .or(self.info().default_top_p);

        // Format prompt text with chat template for reasoning
        let prompt_text = match Self::apply_chat_template(&prompt) {
            Ok(text) => text,
            Err(e) => {
                return Box::pin(async_stream::spawn_stream(move |tx| async move {
                    let _ = tx.send(CandleCompletionChunk::Error(format!(
                        "Failed to apply chat template: {}",
                        e
                    )));
                }));
            }
        };
        let max_tokens = params.max_tokens.map(|n| n.get()).unwrap_or(2000);

        // Use Engine's coordinate_generation for automatic metrics and stream conversion
        Box::pin(engine.coordinate_generation(move || {
                use crate::core::generation::{
                    SamplingConfig, generator::TextGenerator, models::CandleQuantizedPhiModel,
                    tokens::SpecialTokens,
                };
                use crate::domain::context::chunk::CandleStringChunk;
                use candle_core::Device;
                use tokenizers::Tokenizer;
                use tokio_stream::StreamExt;

                async_stream::spawn_stream(move |tx| async move {
                    // Load file paths asynchronously
                    let gguf_repo = model_clone.info().quantization_url.unwrap();
                    log::info!("Requesting GGUF from repo: '{}', file: 'phi-4-reasoning-Q4_K_M.gguf'", gguf_repo);
                    let gguf_path = match model_clone.huggingface_file(
                        gguf_repo,
                        "phi-4-reasoning-Q4_K_M.gguf",
                    ).await {
                        Ok(path) => path,
                        Err(e) => {
                            let _ = tx.send(CandleStringChunk(format!(
                                "ERROR: Failed to get GGUF file: {}",
                                e
                            )));
                            return;
                        }
                    };

                    let tokenizer_repo = model_clone.info().registry_key;
                    log::info!("Requesting tokenizer from repo: '{}', file: 'tokenizer.json'", tokenizer_repo);
                    let tokenizer_path = match model_clone.huggingface_file(tokenizer_repo, "tokenizer.json").await {
                        Ok(path) => path,
                        Err(e) => {
                            let _ = tx.send(CandleStringChunk(format!(
                                "ERROR: Failed to get tokenizer file: {}",
                                e
                            )));
                            return;
                        }
                    };

                    // Load device (prefer GPU if available)
                    let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
                        log::warn!("Device detection failed: {}. Using CPU.", e);
                        Device::Cpu
                    });

                    // Load tokenizer - CRITICAL: Use spawn_blocking for sync I/O
                    let tokenizer = match tokio::task::spawn_blocking(move || {
                        Tokenizer::from_file(&tokenizer_path)
                    }).await {
                        Ok(Ok(t)) => t,
                        Ok(Err(e)) => {
                            let _ = tx.send(CandleStringChunk(format!(
                                "ERROR: Failed to load tokenizer: {}",
                                e
                            )));
                            return;
                        }
                        Err(e) => {
                            let _ = tx.send(CandleStringChunk(format!(
                                "ERROR: Failed to spawn blocking task: {}",
                                e
                            )));
                            return;
                        }
                    };

                    // Load the quantized Phi model
                    let quantized_model =
                        match CandleQuantizedPhiModel::from_gguf_path(&gguf_path, device.clone()).await {
                            Ok(model) => model,
                            Err(e) => {
                                let _ = tx.send(CandleStringChunk(format!(
                                    "ERROR: Failed to load quantized model: {}",
                                    e
                                )));
                                return;
                            }
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

                    // Create TextGenerator with real model
                    let text_generator = TextGenerator::new(
                        Box::new(quantized_model),
                        tokenizer,
                        device,
                        sampling_config,
                    );

                    // Set up special tokens for Phi-4
                    let special_tokens = SpecialTokens {
                        bos_token_id: None, // Phi doesn't use BOS
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

// Static model info for Phi-4-Reasoning
pub static PHI4_REASONING_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::Unsloth,
    name: "phi-4-reasoning-q4-k-m",
    registry_key: "unsloth/phi-4-reasoning",
    quantization_url: Some("unsloth/Phi-4-reasoning-GGUF"),
    max_input_tokens: NonZeroU32::new(32768),
    max_output_tokens: NonZeroU32::new(32768),
    input_price: None,
    output_price: None,
    supports_vision: false,
    supports_function_calling: false,
    supports_streaming: true,
    supports_embeddings: false,
    requires_max_tokens: false,
    supports_thinking: true, // This model has reasoning capabilities
    optimal_thinking_budget: Some(2048),
    system_prompt_prefix: None,
    real_name: None,
    model_type: None,
    model_id: "phi-4-reasoning",
    quantization: "Q4_K_M",
    patch: None,
    embedding_dimension: None,
    vocab_size: None,
    image_size: None,
    image_mean: None,
    image_std: None,
    default_temperature: Some(0.7),
    default_top_k: Some(50),
    default_top_p: Some(0.95),
    supports_kv_cache: true, // MixFormer uses internal KV cache
    supports_flash_attention: false,
    use_bf16: false,
    default_steps: None,
    default_guidance_scale: None,
    time_shift: None,
    est_memory_allocation_mb: 0,
};

impl CandleModel for CandlePhi4ReasoningModel {
    #[inline]
    fn info(&self) -> &'static CandleModelInfo {
        &PHI4_REASONING_MODEL_INFO
    }
}

/// Loaded Phi-4-Reasoning model that keeps resources in memory for worker threads
///
/// This model ACTUALLY caches the loaded model in memory to avoid reloading
/// the 7.8GB GGUF file on every request.
#[derive(Clone)]
pub struct LoadedPhi4ReasoningModel {
    /// The ACTUAL loaded model - cached in memory with Mutex for safe async mutable access
    model: Arc<tokio::sync::Mutex<crate::core::generation::models::CandleQuantizedPhiModel>>,
    tokenizer: tokenizers::Tokenizer,
    device: candle_core::Device,
    engine: Engine,
}

impl std::fmt::Debug for LoadedPhi4ReasoningModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoadedPhi4ReasoningModel")
            .field("model", &"<CandleQuantizedPhiModel>")
            .field("tokenizer", &"<Tokenizer>")
            .field("device", &self.device)
            .field("engine", &self.engine)
            .finish()
    }
}

impl LoadedPhi4ReasoningModel {
    /// Load model resources into memory (called once per worker)
    ///
    /// This method loads EVERYTHING once: model, tokenizer, device.
    /// The model stays in memory for all subsequent requests.
    pub async fn load(
        base: &CandlePhi4ReasoningModel,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        log::info!("🔄 LoadedPhi4ReasoningModel::load() - Loading model into memory ONCE");
        
        // Get file paths
        let gguf_file_path = base
            .huggingface_file(
                base.info().quantization_url.unwrap(),
                "phi-4-reasoning-Q4_K_M.gguf",
            ).await
            .map_err(|e| {
                Box::from(format!("Failed to get GGUF file: {}", e))
                    as Box<dyn std::error::Error + Send + Sync>
            })?;

        let tokenizer_path = base
            .huggingface_file(base.info().registry_key, "tokenizer.json").await
            .map_err(|e| {
                Box::from(format!("Failed to get tokenizer file: {}", e))
                    as Box<dyn std::error::Error + Send + Sync>
            })?;

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

        // CRITICAL: Load the model ONCE and cache it
        log::info!("🔥 Loading 7.8GB model from {} - THIS HAPPENS ONCE", gguf_file_path.display());
        let model = crate::core::generation::models::CandleQuantizedPhiModel::from_gguf_path(
            &gguf_file_path,
            device.clone()
        ).await.map_err(|e| {
            Box::from(format!("Failed to load model: {}", e))
                as Box<dyn std::error::Error + Send + Sync>
        })?;
        
        log::info!("✅ Model loaded into memory! All future requests will reuse this cached model.");

        Ok(Self {
            model: Arc::new(tokio::sync::Mutex::new(model)),  // Cache the loaded model with Mutex for safe async access!
            tokenizer,
            device,
            engine: base.engine.clone(),
        })
    }
}

impl crate::capability::traits::TextToTextCapable for LoadedPhi4ReasoningModel {
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
        let temperature = if params.temperature != 1.0 {
            params.temperature
        } else {
            PHI4_REASONING_MODEL_INFO.default_temperature.unwrap_or(0.7)
        };

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
            .or(PHI4_REASONING_MODEL_INFO.default_top_p);

        // Format prompt text with chat template for reasoning
        let prompt_text = match CandlePhi4ReasoningModel::apply_chat_template(&prompt) {
            Ok(text) => text,
            Err(e) => {
                return Box::pin(async_stream::spawn_stream(move |tx| async move {
                    let _ = tx.send(CandleCompletionChunk::Error(format!(
                        "Failed to apply chat template: {}",
                        e
                    )));
                }));
            }
        };
        let max_tokens = params.max_tokens.map(|n| n.get()).unwrap_or(2000);

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
                // Use SharedModel wrapper to share the Arc<Mutex<Model>> across generate() calls
                let text_generator = TextGenerator::new(
                    Box::new(SharedPhiModel { 
                        model: model.clone(),
                        device: device.clone(),
                        vocab_size,
                    }),
                    tokenizer, // ✅ Use pre-loaded tokenizer (no disk I/O)
                    device,
                    sampling_config,
                );

                // Set up special tokens for Phi-4
                let special_tokens = SpecialTokens {
                    bos_token_id: None, // Phi doesn't use BOS
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

impl CandleModel for LoadedPhi4ReasoningModel {
    #[inline]
    fn info(&self) -> &'static CandleModelInfo {
        &PHI4_REASONING_MODEL_INFO
    }
}

/// Wrapper to share Arc<Mutex<CandleQuantizedPhiModel>> safely across generations
/// This provides safe async mutable access to the cached model
struct SharedPhiModel {
    model: Arc<tokio::sync::Mutex<crate::core::generation::models::CandleQuantizedPhiModel>>,
    device: candle_core::Device,
    vocab_size: usize,
}

#[async_trait]
impl crate::core::generation::models::CandleModel for SharedPhiModel {
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
