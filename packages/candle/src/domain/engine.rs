//! Local Candle Engine
//!
//! Provides local model inference engine using the Candle ML framework.
//! This engine routes requests to appropriate local model providers without any cloud dependencies.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use ystream::AsyncStream;

use crate::domain::completion::CandleCompletionResponse;
use crate::capability::traits::TextToTextCapable;
use crate::domain::model::traits::CandleModel;
use crate::capability::text_to_text::{CandleKimiK2Model, CandleQwen3CoderModel};

/// Local engine errors
#[derive(Error, Debug, Clone)]
pub enum LocalEngineError {
    #[error("Model not supported: {0}")]
    ModelNotSupported(String),

    #[error("Provider error: {0}")]
    ProviderError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

/// Local engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalEngineConfig {
    /// Model identifier (e.g., "kimi-k2", "qwen3-coder")
    pub model_name: String,
    /// Maximum tokens for completion
    pub max_tokens: Option<u32>,
    /// Temperature for response randomness (0.0 - 1.0)
    pub temperature: Option<f32>,
    /// Whether to enable streaming responses
    pub enable_streaming: bool,
}

impl Default for LocalEngineConfig {
    fn default() -> Self {
        Self {
            model_name: "kimi-k2".to_string(),
            max_tokens: Some(1000),
            temperature: Some(0.7),
            enable_streaming: true,
        }
    }
}

/// Local Candle engine for model inference
pub struct LocalEngine {
    config: LocalEngineConfig,
    request_count: AtomicU64,
    is_healthy: AtomicBool,
}

impl LocalEngine {
    /// Create new local engine
    #[must_use]
    pub fn new(config: LocalEngineConfig) -> Self {
        Self {
            config,
            request_count: AtomicU64::new(0),
            is_healthy: AtomicBool::new(true),
        }
    }

    /// Process completion request using local providers
    #[inline]
    pub fn complete(
        &self,
        prompt: String,
    ) -> AsyncStream<CandleCompletionResponse<'static>> {
        // Increment request counter
        self.request_count.fetch_add(1, Ordering::Relaxed);

        // Capture values from self before creating the async stream
        let model_name = self.config.model_name.clone();
        let max_tokens_param = self.config.max_tokens;
        let temperature_param = self.config.temperature;

        AsyncStream::with_channel(move |sender| {
            // Route to appropriate provider based on model name
            match model_name.as_str() {
                "kimi-k2" => {
                    Self::process_kimi_k2(&model_name, &prompt, max_tokens_param, temperature_param, &sender);
                }
                "qwen3-coder" => {
                    Self::process_qwen3_coder(&model_name, &prompt, max_tokens_param, temperature_param, &sender);
                }
                _ => {
                    let _ = sender.send(Self::create_error_response(
                        model_name,
                        "Unsupported model. Supported models: kimi-k2, qwen3-coder".to_string(),
                        0,
                    ));
                }
            }
        })
    }

    /// Process Kimi-K2 provider completion
    #[inline]
    fn process_kimi_k2(
        model_name: &str,
        prompt: &str,
        max_tokens_param: Option<u32>,
        temperature_param: Option<f32>,
        sender: &ystream::AsyncStreamSender<CandleCompletionResponse<'static>>,
    ) {
        use ystream::spawn_task;
        
        let provider_task = spawn_task(|| -> Result<CandleKimiK2Model, String> {
            CandleKimiK2Model::new()
                .map_err(|e| format!("Kimi K2 model initialization failed: {e}"))
        });
        
        let provider_result = match provider_task.collect() {
            Ok(result) => result,
            Err(task_error) => {
                let _ = sender.send(Self::create_error_response(
                    model_name.to_string(),
                    format!("Kimi-K2 provider task execution failed: {task_error}"),
                    0,
                ));
                return;
            }
        };
        
        match provider_result {
            Ok(provider) => {
                if let Err(e) = Self::process_provider_stream(
                    &provider,
                    prompt,
                    max_tokens_param,
                    temperature_param,
                    model_name,
                    sender,
                ) {
                    let _ = sender.send(Self::create_error_response(
                        model_name.to_string(),
                        format!("Stream processing failed: {e}"),
                        0,
                    ));
                }
            }
            Err(e) => {
                let _ = sender.send(Self::create_error_response(
                    model_name.to_string(),
                    format!("Error creating Kimi-K2 provider: {e}"),
                    0,
                ));
            }
        }
    }

    /// Process Qwen3-Coder provider completion
    #[inline]
    fn process_qwen3_coder(
        model_name: &str,
        prompt: &str,
        max_tokens_param: Option<u32>,
        temperature_param: Option<f32>,
        sender: &ystream::AsyncStreamSender<CandleCompletionResponse<'static>>,
    ) {
        use ystream::spawn_task;
        
        let provider_task = spawn_task(|| async {
            CandleQwen3CoderModel::new().await
                .map_err(|e| format!("Qwen3Coder model initialization failed: {e}"))
        });
        
        let provider_result = match provider_task.collect() {
            Ok(result) => result,
            Err(task_error) => {
                let _ = sender.send(Self::create_error_response(
                    model_name.to_string(),
                    format!("Qwen3-Coder provider task execution failed: {task_error}"),
                    0,
                ));
                return;
            }
        };
        
        match provider_result {
            Ok(provider) => {
                if let Err(e) = Self::process_provider_stream(
                    &provider,
                    prompt,
                    max_tokens_param,
                    temperature_param,
                    model_name,
                    sender,
                ) {
                    let _ = sender.send(Self::create_error_response(
                        model_name.to_string(),
                        format!("Stream processing failed: {e}"),
                        0,
                    ));
                }
            }
            Err(e) => {
                let _ = sender.send(Self::create_error_response(
                    model_name.to_string(),
                    format!("Error creating Qwen3-Coder provider: {e}"),
                    0,
                ));
            }
        }
    }

    /// Process completion stream from a provider - generic over provider type
    #[inline]
    fn process_provider_stream<P: TextToTextCapable + CandleModel>(
        provider: &P,
        prompt: &str,
        max_tokens_param: Option<u32>,
        temperature_param: Option<f32>,
        model_name: &str,
        sender: &ystream::AsyncStreamSender<CandleCompletionResponse<'static>>,
    ) -> Result<(), String> {
        // Create completion parameters with safe non-zero conversions
        let max_tokens = max_tokens_param.and_then(|t| std::num::NonZeroU64::new(u64::from(t)));
        let Some(n) = std::num::NonZeroU8::new(1) else {
            let _ = sender.send(Self::create_error_response(
                model_name.to_string(),
                "Internal error: failed to create NonZeroU8 value".to_string(),
                0,
            ));
            return Err("NonZeroU8 creation failed".to_string());
        };
        
        let completion_params = crate::domain::completion::CandleCompletionParams {
            temperature: f64::from(temperature_param.unwrap_or(0.7)),
            max_tokens,
            n,
            stream: true,
            tools: None,
            additional_params: None,
        };
        
        // Create prompt
        let candle_prompt = crate::domain::prompt::CandlePrompt::new(prompt.to_string());
        
        // Get completion stream from provider
        let completion_stream = provider.prompt(candle_prompt, &completion_params);
        
        // Collect chunks using ystream pattern
        let chunks: Vec<_> = completion_stream.collect();
        
        // Process collected chunks
        for chunk in chunks {
            let response = Self::process_chunk(chunk, model_name);
            if sender.send(response).is_err() {
                break; // Client disconnected
            }
        }
        
        Ok(())
    }

    /// Process a single completion chunk into a response
    #[inline]
    fn process_chunk(
        chunk: crate::domain::context::chunk::CandleCompletionChunk,
        model_name: &str,
    ) -> CandleCompletionResponse<'static> {
        match chunk {
            crate::domain::context::chunk::CandleCompletionChunk::Text(text) => {
                CandleCompletionResponse {
                    text: text.into(),
                    model: model_name.to_string().into(),
                    provider: Some("candle-local".into()),
                    usage: None,
                    finish_reason: None,
                    response_time_ms: None,
                    generation_time_ms: None,
                    tokens_per_second: None,
                }
            }
            crate::domain::context::chunk::CandleCompletionChunk::Error(err) => {
                CandleCompletionResponse {
                    text: format!("Error: {err}").into(),
                    model: model_name.to_string().into(),
                    provider: Some("candle-local".into()),
                    usage: None,
                    finish_reason: Some("error".into()),
                    response_time_ms: None,
                    generation_time_ms: None,
                    tokens_per_second: None,
                }
            }
            crate::domain::context::chunk::CandleCompletionChunk::Complete { text, .. } => {
                CandleCompletionResponse {
                    text: text.into(),
                    model: model_name.to_string().into(),
                    provider: Some("candle-local".into()),
                    usage: None,
                    finish_reason: Some("complete".into()),
                    response_time_ms: None,
                    generation_time_ms: None,
                    tokens_per_second: None,
                }
            }
            _ => {
                // Skip other chunk types - return empty response that caller can filter
                CandleCompletionResponse {
                    text: "".into(),
                    model: model_name.to_string().into(),
                    provider: Some("candle-local".into()),
                    usage: None,
                    finish_reason: None,
                    response_time_ms: None,
                    generation_time_ms: None,
                    tokens_per_second: None,
                }
            }
        }
    }

    /// Create an error response with consistent formatting
    #[inline]
    fn create_error_response(
        model_name: String,
        error_message: String,
        generation_time_ms: u32,
    ) -> CandleCompletionResponse<'static> {
        CandleCompletionResponse {
            text: error_message.into(),
            model: model_name.into(),
            provider: Some("candle-local".into()),
            usage: None,
            finish_reason: Some("error".into()),
            response_time_ms: None,
            generation_time_ms: Some(generation_time_ms),
            tokens_per_second: Some(0.0),
        }
    }

    /// Get request count
    pub fn request_count(&self) -> u64 {
        self.request_count.load(Ordering::Relaxed)
    }

    /// Check if engine is healthy
    pub fn is_healthy(&self) -> bool {
        self.is_healthy.load(Ordering::Relaxed)
    }
}
