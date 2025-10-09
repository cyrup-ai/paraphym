//! Phi-4-Reasoning Provider for local Phi-4-reasoning Q4_K_M model inference
//!
//! This provider implements the TextToTextCapable trait for the Phi-4-reasoning
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

use crate::domain::model::{info::CandleModelInfo, traits::CandleModel};
use crate::domain::model::download::ModelDownloadProvider;
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
    temperature: f64,
    top_p: Option<f64>,
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
            temperature: 0.0,
            top_p: Some(1.0),
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

    /// Set sampling temperature for LogitsProcessor
    pub fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = temperature;
        self.logits_processor = LogitsProcessor::new(299792458, Some(temperature), self.top_p);
        self
    }

    /// Set top-p (nucleus sampling) for LogitsProcessor
    pub fn with_top_p(mut self, top_p: Option<f64>) -> Self {
        self.top_p = top_p;
        self.logits_processor = LogitsProcessor::new(299792458, Some(self.temperature), top_p);
        self
    }

    /// Clear KV cache (if needed for stateful generation)
    pub fn clear_kv_cache(&mut self) {
        self.model.clear_kv_cache();
    }
}

/// CandlePhi4ReasoningModel for local Phi-4-reasoning model inference
#[derive(Debug, Clone)]
pub struct CandlePhi4ReasoningModel {}

impl Default for CandlePhi4ReasoningModel {
    fn default() -> Self {
        Self::new()
    }
}



impl CandlePhi4ReasoningModel {
    /// Create new Phi-4-Reasoning provider
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::capability::traits::TextToTextCapable for CandlePhi4ReasoningModel {
    fn prompt(
        &self,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> AsyncStream<CandleCompletionChunk> {
        // Get file paths BEFORE the closure (self is available here)
        let gguf_path = match self.huggingface_file("phi-4-reasoning-Q4_K_M.gguf") {
            Ok(path) => path,
            Err(e) => {
                return AsyncStream::with_channel(move |sender| {
                    let _ = sender.send(CandleCompletionChunk::Error(
                        format!("Failed to get GGUF file: {}", e)
                    ));
                });
            }
        };
        
        let tokenizer_path = match self.huggingface_file("tokenizer.json") {
            Ok(path) => path,
            Err(e) => {
                return AsyncStream::with_channel(move |sender| {
                    let _ = sender.send(CandleCompletionChunk::Error(
                        format!("Failed to get tokenizer file: {}", e)
                    ));
                });
            }
        };
        
        // Get config values from ModelInfo and CompletionParams
        let max_context = self.info().max_input_tokens
            .map(|t| t.get())
            .unwrap_or(32768);
        
        // Use params temperature if not default, otherwise ModelInfo default, otherwise 0.0
        let temperature = if params.temperature != 1.0 {
            params.temperature
        } else {
            self.info().default_temperature.unwrap_or(0.0)
        };
        
        // These aren't in CandleCompletionParams, use ModelInfo defaults
        let top_p = self.info().default_top_p;
        let repeat_penalty = 1.0f32;
        
        let prompt_text = prompt.to_string();

        AsyncStream::with_channel(move |sender| {
            // Load model and generate (blocking operation in stream context)
            let device = Device::cuda_if_available(0).unwrap_or(Device::Cpu);
            
            let model_result = Phi4ReasoningModel::load_from_gguf(
                &gguf_path,
                &tokenizer_path,
                &device,
            );

            let mut model = match model_result {
                Ok(m) => m.with_repeat_penalty(repeat_penalty)
                           .with_temperature(temperature)
                           .with_top_p(top_p),
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
            let max_tokens = max_context as usize;
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

// Static model info for Phi-4-Reasoning
static PHI4_REASONING_MODEL_INFO: CandleModelInfo = CandleModelInfo {
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
    supports_thinking: true,  // This model has reasoning capabilities
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
    default_temperature: Some(0.0),
    default_top_k: None,
    default_top_p: None,
    supports_kv_cache: true,
    supports_flash_attention: false,
    use_bf16: false,
    default_steps: None,
    default_guidance_scale: None,
    time_shift: None,
};

impl CandleModel for CandlePhi4ReasoningModel {
    fn info(&self) -> &'static CandleModelInfo {
        &PHI4_REASONING_MODEL_INFO
    }
}
