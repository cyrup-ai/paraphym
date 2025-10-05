//! Phi-4-Reasoning Provider for local Phi-4-reasoning Q4_K_M model inference
//!
//! This provider implements the CandleCompletionModel trait for the Phi-4-reasoning
//! model with integrated chain-of-thought reasoning capabilities.

use std::num::NonZeroU32;

use anyhow::{Context, Result};
use candle_core::{DType, Device, Tensor};
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::mixformer::Config as MixFormerConfig;
use candle_transformers::models::quantized_mixformer::MixFormerSequentialForCausalLM as QMixFormer;
use std::path::Path;
use tokenizers::Tokenizer;
use ystream::AsyncStream;

use crate::builders::agent_role::CandleCompletionProvider as BuilderCandleCompletionProvider;
use crate::domain::model::{info::CandleModelInfo, traits::CandleModel};
use crate::domain::{
    completion::{CandleCompletionModel, CandleCompletionParams},
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

/// Chat message for template processing
#[derive(Debug, Clone, serde::Serialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    /// Create a user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    /// Create an assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }

    /// Create a system message
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }
}

/// Phi-4-Reasoning model with integrated chat template and reasoning parsing
pub struct Phi4ReasoningModel {
    model: QMixFormer,
    tokenizer: Tokenizer,
    device: Device,
    logits_processor: LogitsProcessor,
    repeat_penalty: f32,
    repeat_last_n: usize,
}

impl Phi4ReasoningModel {
    /// Load Phi-4-Reasoning model from GGUF file
    pub fn load_from_gguf(
        model_path: impl AsRef<Path>,
        tokenizer_path: impl AsRef<Path>,
        device: &Device,
    ) -> Result<Self> {
        // Load GGUF quantized model
        let vb = candle_transformers::quantized_var_builder::VarBuilder::from_gguf(
            model_path.as_ref(),
            device,
        )
        .context("Failed to load GGUF model")?;

        // Create Phi-4-reasoning configuration using v2 base
        let config = MixFormerConfig::v2();

        // Initialize quantized model
        let model = QMixFormer::new_v2(&config, vb)
            .context("Failed to initialize Phi-4-reasoning model")?;

        // Load tokenizer
        let tokenizer = Tokenizer::from_file(tokenizer_path.as_ref())
            .map_err(|e| anyhow::anyhow!("Failed to load tokenizer: {}", e))?;

        // Initialize logits processor with deterministic settings for reasoning
        let logits_processor = LogitsProcessor::new(299792458, Some(0.0), Some(1.0));

        Ok(Self {
            model,
            tokenizer,
            device: device.clone(),
            logits_processor,
            repeat_penalty: 1.0,
            repeat_last_n: 64,
        })
    }

    /// Apply chat template to format messages
    pub fn apply_chat_template(&self, messages: &[ChatMessage]) -> Result<String> {
        let mut env = minijinja::Environment::new();
        env.add_template("chat", CHAT_TEMPLATE)
            .context("Failed to add chat template")?;

        let template = env
            .get_template("chat")
            .context("Failed to get chat template")?;

        let mut map = std::collections::HashMap::new();
        map.insert("messages", minijinja::value::Value::from_serialize(messages));
        map.insert("add_generation_prompt", minijinja::value::Value::from(true));

        let rendered = template
            .render(map)
            .context("Failed to render chat template")?;

        Ok(rendered)
    }

    /// Generate response with reasoning
    pub fn generate(&mut self, prompt: &str, max_tokens: usize) -> Result<String> {
        let tokens = self
            .tokenizer
            .encode(prompt, true)
            .map_err(|e| anyhow::anyhow!("Tokenization failed: {}", e))?;

        let mut token_ids = tokens.get_ids().to_vec();
        if token_ids.is_empty() {
            anyhow::bail!("Empty prompt not supported");
        }

        let eos_token = self
            .tokenizer
            .token_to_id("<|im_end|>")
            .or_else(|| self.tokenizer.token_to_id("|endoftext|>"))
            .context("Failed to find EOS token")?;

        let mut generated_text = String::new();

        // Generation loop
        for index in 0..max_tokens {
            let context_size = if index > 0 { 1 } else { token_ids.len() };
            let ctxt = &token_ids[token_ids.len().saturating_sub(context_size)..];
            let input = Tensor::new(ctxt, &self.device)
                .context("Failed to create input tensor")?
                .unsqueeze(0)
                .context("Failed to unsqueeze tensor")?;

            let logits = self
                .model
                .forward(&input)
                .context("Forward pass failed")?
                .squeeze(0)
                .context("Failed to squeeze output")?
                .to_dtype(DType::F32)
                .context("Failed to convert to F32")?;

            let logits = if self.repeat_penalty == 1.0 {
                logits
            } else {
                let start_at = token_ids.len().saturating_sub(self.repeat_last_n);
                candle_transformers::utils::apply_repeat_penalty(
                    &logits,
                    self.repeat_penalty,
                    &token_ids[start_at..],
                )
                .context("Failed to apply repeat penalty")?
            };

            let next_token = self
                .logits_processor
                .sample(&logits)
                .context("Sampling failed")?;

            token_ids.push(next_token);

            if next_token == eos_token {
                break;
            }

            if let Ok(decoded) = self.tokenizer.decode(&[next_token], false) {
                generated_text.push_str(&decoded);
            }
        }

        // Decode full token sequence to get complete text
        let full_text = self
            .tokenizer
            .decode(&token_ids, false)
            .map_err(|e| anyhow::anyhow!("Decoding failed: {}", e))?;

        Ok(full_text)
    }

    /// Extract reasoning and solution from model response
    /// Returns (reasoning_content, solution_content)
    pub fn extract_reasoning(&self, response: &str) -> (Option<String>, String) {
        // Use regex to extract content between <think> tags
        let re = regex::Regex::new(r"<think>(.*?)</think>").ok();

        if let Some(regex) = re
            && let Some(captures) = regex.captures(response)
        {
            let reasoning = captures.get(1).map(|m| m.as_str().trim().to_string());
            let solution = regex.replace(response, "").trim().to_string();
            return (reasoning, solution);
        }

        // Fallback: no thinking tags found
        (None, response.trim().to_string())
    }

    /// Set custom repeat penalty
    pub fn with_repeat_penalty(mut self, penalty: f32) -> Self {
        self.repeat_penalty = penalty;
        self
    }

    /// Set custom repeat last n tokens
    pub fn with_repeat_last_n(mut self, n: usize) -> Self {
        self.repeat_last_n = n;
        self
    }

    /// Clear KV cache (if needed for stateful generation)
    pub fn clear_kv_cache(&mut self) {
        self.model.clear_kv_cache();
    }
}

/// CandlePhi4ReasoningProvider for local Phi-4-reasoning model inference
#[derive(Debug, Clone)]
pub struct CandlePhi4ReasoningProvider {
    /// Model cache directory path
    model_path: String,
    /// Tokenizer path
    tokenizer_path: String,
    /// Provider configuration
    config: CandlePhi4ReasoningConfig,
}

/// Configuration for Phi-4-Reasoning model inference
#[derive(Debug, Clone)]
pub struct CandlePhi4ReasoningConfig {
    /// Maximum context length for inference
    max_context: u32,
    /// Default temperature for sampling (0.0 for deterministic reasoning)
    temperature: f64,
    /// Top-p nucleus sampling parameter
    pub top_p: Option<f64>,
    /// Repeat penalty
    pub repeat_penalty: f32,
}

impl Default for CandlePhi4ReasoningConfig {
    fn default() -> Self {
        Self {
            max_context: 32768,
            temperature: 0.0,  // Deterministic for consistent reasoning
            top_p: Some(1.0),
            repeat_penalty: 1.0,
        }
    }
}

impl CandlePhi4ReasoningConfig {
    /// Get the temperature setting
    pub fn temperature(&self) -> f64 {
        self.temperature
    }

    /// Set temperature for sampling
    pub fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = temperature;
        self
    }

    /// Set maximum context length
    pub fn with_max_context(mut self, max_context: u32) -> Self {
        self.max_context = max_context;
        self
    }
}

impl CandlePhi4ReasoningProvider {
    /// Create provider with async model download
    pub async fn with_config_async(
        config: CandlePhi4ReasoningConfig,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        use crate::domain::model::download::DownloadProviderFactory;

        // Use factory to get download provider
        let downloader = DownloadProviderFactory::create_default()?;

        // Download model files
        let result = downloader.download_model(
            "unsloth/Phi-4-reasoning-GGUF",
            vec!["phi-4-reasoning-Q4_K_M.gguf".to_string(), "tokenizer.json".to_string()],
            Some("Q4_K_M".to_string()),
        ).await?;

        // Find GGUF and tokenizer files
        let gguf_file = result.files.iter()
            .find(|f| f.extension().and_then(|s| s.to_str()) == Some("gguf"))
            .ok_or_else(|| Box::<dyn std::error::Error + Send + Sync>::from("GGUF file not found in download"))?;

        let tokenizer_file = result.files.iter()
            .find(|f| f.file_name().and_then(|s| s.to_str()) == Some("tokenizer.json"))
            .ok_or_else(|| Box::<dyn std::error::Error + Send + Sync>::from("Tokenizer file not found in download"))?;

        Self::with_config_sync_paths(
            gguf_file.to_str()
                .ok_or_else(|| Box::<dyn std::error::Error + Send + Sync>::from("Invalid GGUF file path"))?
                .to_string(),
            tokenizer_file.to_str()
                .ok_or_else(|| Box::<dyn std::error::Error + Send + Sync>::from("Invalid tokenizer path"))?
                .to_string(),
            config,
        )
    }

    /// Create default provider instance for builder pattern
    pub fn default_for_builder() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let config = CandlePhi4ReasoningConfig::default();
        let runtime = crate::runtime::shared_runtime()
            .ok_or("Runtime unavailable for provider initialization")?;
        runtime.block_on(Self::with_config_async(config))
    }

    /// Create provider with custom configuration and existing model path
    pub fn with_config_sync(
        model_path: String,
        config: CandlePhi4ReasoningConfig,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Construct tokenizer path (assumed to be in same directory)
        let tokenizer_path = format!("{}/tokenizer.json", model_path);
        
        Self::with_config_sync_paths(model_path, tokenizer_path, config)
    }

    /// Create provider with explicit paths
    pub fn with_config_sync_paths(
        model_path: String,
        tokenizer_path: String,
        config: CandlePhi4ReasoningConfig,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        log::info!(
            "Phi4Reasoning Provider initialized with model: {}, tokenizer: {}",
            model_path,
            tokenizer_path
        );

        Ok(Self {
            model_path,
            tokenizer_path,
            config,
        })
    }
}

impl CandleCompletionModel for CandlePhi4ReasoningProvider {
    fn prompt(
        &self,
        prompt: CandlePrompt,
        _params: &CandleCompletionParams,
    ) -> AsyncStream<CandleCompletionChunk> {
        let model_path = self.model_path.clone();
        let tokenizer_path = self.tokenizer_path.clone();
        let config = self.config.clone();
        let prompt_text = prompt.to_string();

        AsyncStream::with_channel(move |sender| {
            // Load model and generate (blocking operation in stream context)
            let device = Device::cuda_if_available(0).unwrap_or(Device::Cpu);
            
            let model_result = Phi4ReasoningModel::load_from_gguf(
                &model_path,
                &tokenizer_path,
                &device,
            );

            let mut model = match model_result {
                Ok(m) => m,
                Err(e) => {
                    let error_chunk = CandleCompletionChunk::Error(
                        format!("Failed to load Phi-4-reasoning model: {}", e)
                    );
                    let _ = sender.send(error_chunk);
                    return;
                }
            };

            // Apply chat template
            let messages = vec![ChatMessage::user(prompt_text)];
            let formatted_prompt = match model.apply_chat_template(&messages) {
                Ok(p) => p,
                Err(e) => {
                    let error_chunk = CandleCompletionChunk::Error(
                        format!("Failed to apply chat template: {}", e)
                    );
                    let _ = sender.send(error_chunk);
                    return;
                }
            };

            // Generate response
            let max_tokens = config.max_context as usize;
            let response = match model.generate(&formatted_prompt, max_tokens) {
                Ok(r) => r,
                Err(e) => {
                    let error_chunk = CandleCompletionChunk::Error(
                        format!("Generation failed: {}", e)
                    );
                    let _ = sender.send(error_chunk);
                    return;
                }
            };

            // Extract reasoning and solution
            let (reasoning, solution) = model.extract_reasoning(&response);

            // Send reasoning as separate chunk if present
            if let Some(reasoning_content) = reasoning {
                let reasoning_text = format!("🧠 REASONING:\n{}\n\n", reasoning_content);
                let reasoning_chunk = CandleCompletionChunk::Text(reasoning_text);
                let _ = sender.send(reasoning_chunk);
            }

            // Send solution
            let solution_chunk = CandleCompletionChunk::Text(solution);
            let _ = sender.send(solution_chunk);

            // Send completion
            let complete_chunk = CandleCompletionChunk::Complete {
                text: response,
                finish_reason: None,
                usage: None,
            };
            let _ = sender.send(complete_chunk);
        })
    }
}

// Implement builder trait
impl BuilderCandleCompletionProvider for CandlePhi4ReasoningProvider {}

// Static model info for Phi-4-Reasoning
static PHI4_REASONING_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider_name: "candle-phi4-reasoning",
    name: "phi-4-reasoning-q4-k-m",
    max_input_tokens: NonZeroU32::new(32768),
    max_output_tokens: NonZeroU32::new(32768),
    input_price: None,
    output_price: None,
    supports_vision: false,
    supports_function_calling: false,
    supports_streaming: true,
    supports_embeddings: false,
    requires_max_tokens: false,
    supports_thinking: true,  // This model has reasoning capabilities
    optimal_thinking_budget: Some(2048),
    system_prompt_prefix: None,
    real_name: None,
    model_type: None,
    model_id: "phi-4-reasoning",
    hf_repo_url: "unsloth/Phi-4-reasoning-GGUF",
    quantization: "Q4_K_M",
    patch: None,
};

impl CandleModel for CandlePhi4ReasoningProvider {
    fn info(&self) -> &'static CandleModelInfo {
        &PHI4_REASONING_MODEL_INFO
    }
}
