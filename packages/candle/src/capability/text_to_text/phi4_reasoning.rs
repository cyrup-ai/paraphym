//! Phi-4-Reasoning Provider for local Phi-4-reasoning Q4_K_M model inference
//!
//! This provider implements the TextToTextCapable trait for the Phi-4-reasoning
//! model with integrated chain-of-thought reasoning capabilities.

use std::num::NonZeroU32;
use std::sync::OnceLock;

use ystream::AsyncStream;

use crate::core::Engine;
use crate::domain::model::{info::CandleModelInfo, traits::CandleModel};
use crate::domain::{
    completion::CandleCompletionParams,
    context::chunk::CandleCompletionChunk,
    prompt::CandlePrompt,
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
    ) -> AsyncStream<CandleCompletionChunk> {
        // Get file paths before the closure
        let gguf_path = match self.huggingface_file("phi-4-reasoning-Q4_K_M.gguf") {
            Ok(path) => path,
            Err(e) => {
                return AsyncStream::with_channel(move |sender| {
                    let _ = sender.send(CandleCompletionChunk::Error(format!(
                        "Failed to get GGUF file: {}",
                        e
                    )));
                });
            }
        };

        let tokenizer_path = match self.huggingface_file("tokenizer.json") {
            Ok(path) => path,
            Err(e) => {
                return AsyncStream::with_channel(move |sender| {
                    let _ = sender.send(CandleCompletionChunk::Error(format!(
                        "Failed to get tokenizer file: {}",
                        e
                    )));
                });
            }
        };

        // Build sampling config
        let temperature = if params.temperature != 1.0 {
            params.temperature
        } else {
            self.info().default_temperature.unwrap_or(0.7)
        };

        // Clone engine for closure
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
                return AsyncStream::with_channel(move |sender| {
                    let _ = sender.send(CandleCompletionChunk::Error(format!(
                        "Failed to apply chat template: {}",
                        e
                    )));
                });
            }
        };
        let max_tokens = params.max_tokens.map(|n| n.get()).unwrap_or(2000);

        // Use Engine's coordinate_generation for automatic metrics and stream conversion
        engine.coordinate_generation(move || {
            use crate::core::generation::{
                generator::TextGenerator, models::CandleQuantizedMixFormerModel,
                tokens::SpecialTokens, SamplingConfig,
            };
            use crate::domain::context::chunk::CandleStringChunk;
            use candle_core::Device;
            use tokenizers::Tokenizer;

            // Load device (prefer GPU if available)
            let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
                log::warn!("Device detection failed: {}. Using CPU.", e);
                Device::Cpu
            });

            // Load tokenizer - return error stream on failure
            let tokenizer = match Tokenizer::from_file(&tokenizer_path) {
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

            // Load the quantized MixFormer model
            let quantized_model = match CandleQuantizedMixFormerModel::from_gguf_path(
                &gguf_path,
                device.clone(),
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

            // Generate and return text stream - Engine handles conversion to CandleCompletionChunk
            text_generator.generate(prompt_text, max_tokens_u32, special_tokens)
        })
    }
}

// Static model info for Phi-4-Reasoning
pub static PHI4_REASONING_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::Unsloth,
    name: "phi-4-reasoning-q4-k-m",
    registry_key: "unsloth/Phi-4-reasoning-GGUF",
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
/// This model pre-loads the tokenizer and device configuration, avoiding
/// disk I/O on every request. The GGUF model is still loaded lazily due to size.
#[derive(Clone, Debug)]
pub struct LoadedPhi4ReasoningModel {
    tokenizer: tokenizers::Tokenizer,
    gguf_file_path: std::path::PathBuf,
    device: candle_core::Device,
    engine: Engine,
}

impl LoadedPhi4ReasoningModel {
    /// Load model resources into memory (called once per worker)
    ///
    /// This method loads the tokenizer and detects the device once,
    /// storing them for reuse across multiple requests.
    pub fn load(base: &CandlePhi4ReasoningModel) 
        -> Result<Self, Box<dyn std::error::Error + Send + Sync>> 
    {
        // Get file paths
        let gguf_file_path = base.huggingface_file("phi-4-reasoning-Q4_K_M.gguf")
            .map_err(|e| Box::from(format!("Failed to get GGUF file: {}", e)) as Box<dyn std::error::Error + Send + Sync>)?;
        
        let tokenizer_path = base.huggingface_file("tokenizer.json")
            .map_err(|e| Box::from(format!("Failed to get tokenizer file: {}", e)) as Box<dyn std::error::Error + Send + Sync>)?;
        
        // Load device (prefer GPU if available)
        let device = crate::core::device_util::detect_best_device()
            .unwrap_or_else(|e| {
                log::warn!("Device detection failed: {}. Using CPU.", e);
                candle_core::Device::Cpu
            });
        
        // Load tokenizer
        let tokenizer = tokenizers::Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| Box::from(format!("Failed to load tokenizer: {}", e)) as Box<dyn std::error::Error + Send + Sync>)?;
        
        Ok(Self {
            tokenizer,
            gguf_file_path,
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
    ) -> AsyncStream<CandleCompletionChunk> {
        // Clone pre-loaded resources for the generation closure
        let engine = self.engine.clone();
        let gguf_path = self.gguf_file_path.clone();
        let device = self.device.clone();
        let tokenizer = self.tokenizer.clone();  // ✅ Clone pre-loaded tokenizer

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
                return AsyncStream::with_channel(move |sender| {
                    let _ = sender.send(CandleCompletionChunk::Error(format!(
                        "Failed to apply chat template: {}",
                        e
                    )));
                });
            }
        };
        let max_tokens = params.max_tokens.map(|n| n.get()).unwrap_or(2000);

        // Use Engine's coordinate_generation for automatic metrics and stream conversion
        engine.coordinate_generation(move || {
            use crate::core::generation::{
                generator::TextGenerator, models::CandleQuantizedMixFormerModel,
                tokens::SpecialTokens, SamplingConfig,
            };
            use crate::domain::context::chunk::CandleStringChunk;

            // Load the quantized MixFormer model
            let quantized_model = match CandleQuantizedMixFormerModel::from_gguf_path(
                &gguf_path,
                device.clone(),
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

            // Create TextGenerator with real model and pre-loaded tokenizer
            let text_generator = TextGenerator::new(
                Box::new(quantized_model),
                tokenizer,  // ✅ Use pre-loaded tokenizer (no disk I/O)
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

            // Generate and return text stream
            text_generator.generate(prompt_text, max_tokens_u32, special_tokens)
        })
    }
}


impl CandleModel for LoadedPhi4ReasoningModel {
    #[inline]
    fn info(&self) -> &'static CandleModelInfo {
        &PHI4_REASONING_MODEL_INFO
    }
}
