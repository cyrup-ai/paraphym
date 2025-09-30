//! Local Candle Engine
//!
//! Provides local model inference engine using the Candle ML framework.
//! This engine routes requests to appropriate local model providers without any cloud dependencies.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use ystream::AsyncStream;

use crate::domain::completion::CandleCompletionResponse;
use crate::domain::completion::traits::CandleCompletionModel;
use crate::providers::{CandleKimiK2Provider, CandleQwen3CoderProvider};

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
    pub fn new(config: LocalEngineConfig) -> Self {
        Self {
            config,
            request_count: AtomicU64::new(0),
            is_healthy: AtomicBool::new(true),
        }
    }

    /// Process completion request using local providers
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
                    // Use spawn_task pattern for provider initialization (no async runtime conflicts)
                    use ystream::spawn_task;
                    
                    let provider_task = spawn_task(|| {
                        // Initialize provider synchronously in background thread
                        let config = crate::providers::kimi_k2::CandleKimiK2Config::default();
                        let model_path = std::env::var("KIMI_MODEL_PATH").unwrap_or_else(|_| "./models/kimi-k2".to_string());
                        CandleKimiK2Provider::with_config_sync(model_path, config)
                            .map_err(|e| format!("Provider initialization failed: {e}"))
                    });
                    
                    let provider_result = match provider_task.collect() {
                        Ok(result) => result,
                        Err(task_error) => {
                            let error_response = CandleCompletionResponse {
                                text: format!("Kimi-K2 provider task execution failed: {task_error}").into(),
                                model: model_name.clone().into(),
                                provider: Some("candle-local".into()),
                                usage: None,
                                finish_reason: Some("error".into()),
                                response_time_ms: None,
                                generation_time_ms: Some(0),
                                tokens_per_second: Some(0.0),
                            };
                            let _ = sender.send(error_response);
                            return;
                        }
                    };
                    
                    match provider_result {
                        Ok(provider) => {
                            // Create completion parameters with safe non-zero conversions
                            let max_tokens = max_tokens_param.and_then(|t| std::num::NonZeroU64::new(t as u64));
                            let n = if let Some(val) = std::num::NonZeroU8::new(1) {
                                val
                            } else {
                                // This should never happen for value 1, but handle gracefully
                                let error_response = CandleCompletionResponse {
                                    text: "Internal error: failed to create NonZeroU8 value".into(),
                                    model: model_name.clone().into(),
                                    provider: Some("candle-local".into()),
                                    usage: None,
                                    finish_reason: Some("error".into()),
                                    response_time_ms: None,
                                    generation_time_ms: Some(0),
                                    tokens_per_second: Some(0.0),
                                };
                                let _ = sender.send(error_response);
                                return;
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
                            let candle_prompt = crate::domain::prompt::CandlePrompt::new(prompt.clone());
                            
                            // Get real completion stream from provider
                            let completion_stream = provider.prompt(candle_prompt, &completion_params);

                            // Use ystream collect pattern instead of async runtime
                            let chunks: Vec<_> = completion_stream.collect();

                            // Process collected chunks using ystream pattern (no async runtime conflicts)
                            for chunk in chunks {
                                let response = match chunk {
                                    crate::domain::context::chunk::CandleCompletionChunk::Text(text) => {
                                        CandleCompletionResponse {
                                            text: text.into(),
                                            model: model_name.clone().into(),
                                            provider: Some("candle-local".into()),
                                            usage: None,
                                            finish_reason: None,
                                            response_time_ms: None,
                                            generation_time_ms: None,
                                            tokens_per_second: None,
                                        }
                                    },
                                    crate::domain::context::chunk::CandleCompletionChunk::Error(err) => {
                                        CandleCompletionResponse {
                                            text: format!("Error: {err}").into(),
                                            model: model_name.clone().into(),
                                            provider: Some("candle-local".into()),
                                            usage: None,
                                            finish_reason: Some("error".into()),
                                            response_time_ms: None,
                                            generation_time_ms: None,
                                            tokens_per_second: None,
                                        }
                                    },
                                    crate::domain::context::chunk::CandleCompletionChunk::Complete { text, .. } => {
                                        CandleCompletionResponse {
                                            text: text.into(),
                                            model: model_name.clone().into(),
                                            provider: Some("candle-local".into()),
                                            usage: None,
                                            finish_reason: Some("complete".into()),
                                            response_time_ms: None,
                                            generation_time_ms: None,
                                            tokens_per_second: None,
                                        }
                                    },
                                    _ => continue, // Skip other chunk types like tool calls
                                };

                                if sender.send(response).is_err() {
                                    break; // Client disconnected
                                }
                            }
                        }
                        Err(e) => {
                            let error_response = CandleCompletionResponse {
                                text: format!("Error creating Kimi-K2 provider: {e}").into(),
                                model: model_name.into(),
                                provider: Some("candle-local".into()),
                                usage: None,
                                finish_reason: Some("error".into()),
                                response_time_ms: None,
                                generation_time_ms: Some(0),
                                tokens_per_second: Some(0.0),
                            };
                            let _ = sender.send(error_response);
                        }
                    }
                }
                "qwen3-coder" => {
                    // Use spawn_task pattern for provider initialization (no async runtime conflicts)
                    use ystream::spawn_task;
                    
                    let provider_task = spawn_task(|| {
                        // Initialize provider synchronously in background thread
                        let config = crate::providers::qwen3_coder::CandleQwen3CoderConfig::default();
                        let model_path = std::env::var("QWEN3_MODEL_PATH").unwrap_or_else(|_| "./models/qwen3-coder".to_string());
                        CandleQwen3CoderProvider::with_config_sync(model_path, config)
                            .map_err(|e| format!("Qwen3Coder provider initialization failed: {e}"))
                    });
                    let provider_result = match provider_task.collect() {
                        Ok(result) => result,
                        Err(task_error) => {
                            let error_response = CandleCompletionResponse {
                                text: format!("Qwen3-Coder provider task execution failed: {task_error}").into(),
                                model: model_name.clone().into(),
                                provider: Some("candle-local".into()),
                                usage: None,
                                finish_reason: Some("error".into()),
                                response_time_ms: None,
                                generation_time_ms: Some(0),
                                tokens_per_second: Some(0.0),
                            };
                            let _ = sender.send(error_response);
                            return;
                        }
                    };
                    
                    match provider_result {
                        Ok(provider) => {
                            // Create completion parameters with safe non-zero conversions
                            let max_tokens = max_tokens_param.and_then(|t| std::num::NonZeroU64::new(t as u64));
                            let n = if let Some(val) = std::num::NonZeroU8::new(1) {
                                val
                            } else {
                                // This should never happen for value 1, but handle gracefully
                                let error_response = CandleCompletionResponse {
                                    text: "Internal error: failed to create NonZeroU8 value".into(),
                                    model: model_name.clone().into(),
                                    provider: Some("candle-local".into()),
                                    usage: None,
                                    finish_reason: Some("error".into()),
                                    response_time_ms: None,
                                    generation_time_ms: Some(0),
                                    tokens_per_second: Some(0.0),
                                };
                                let _ = sender.send(error_response);
                                return;
                            };
                            
                            let completion_params = crate::domain::completion::CandleCompletionParams {
                                temperature: f64::from(temperature_param.unwrap_or(0.7)),
                                max_tokens,
                                n,
                                stream: true,
                                tools: None,
                                additional_params: None,
                            };

                            // Create prompt for Qwen3-Coder
                            let candle_prompt = crate::domain::prompt::CandlePrompt::new(prompt.clone());

                            // Get real completion stream from Qwen3-Coder provider
                            let completion_stream = provider.prompt(candle_prompt, &completion_params);

                            // Use ystream collect pattern instead of async runtime blocking
                            let chunks: Vec<_> = completion_stream.collect();

                            // Process collected chunks using ystream pattern (no async runtime conflicts)
                            for chunk in chunks {
                                let response = match chunk {
                                    crate::domain::context::chunk::CandleCompletionChunk::Text(text) => {
                                        CandleCompletionResponse {
                                            text: text.into(),
                                            model: model_name.clone().into(),
                                            provider: Some("candle-local".into()),
                                            usage: None,
                                            finish_reason: None,
                                            response_time_ms: None,
                                            generation_time_ms: None,
                                            tokens_per_second: None,
                                        }
                                    },
                                    crate::domain::context::chunk::CandleCompletionChunk::Error(err) => {
                                        CandleCompletionResponse {
                                            text: format!("Error: {err}").into(),
                                            model: model_name.clone().into(),
                                            provider: Some("candle-local".into()),
                                            usage: None,
                                            finish_reason: Some("error".into()),
                                            response_time_ms: None,
                                            generation_time_ms: None,
                                            tokens_per_second: None,
                                        }
                                    },
                                    crate::domain::context::chunk::CandleCompletionChunk::Complete { text, .. } => {
                                        CandleCompletionResponse {
                                            text: text.into(),
                                            model: model_name.clone().into(),
                                            provider: Some("candle-local".into()),
                                            usage: None,
                                            finish_reason: Some("complete".into()),
                                            response_time_ms: None,
                                            generation_time_ms: None,
                                            tokens_per_second: None,
                                        }
                                    },
                                    _ => continue, // Skip other chunk types like tool calls
                                };

                                if sender.send(response).is_err() {
                                    break; // Client disconnected
                                }
                            }
                        }
                        Err(e) => {
                            let error_response = CandleCompletionResponse {
                                text: format!("Error creating Qwen3-Coder provider: {e}").into(),
                                model: model_name.into(),
                                provider: Some("candle-local".into()),
                                usage: None,
                                finish_reason: Some("error".into()),
                                response_time_ms: None,
                                generation_time_ms: Some(0),
                                tokens_per_second: Some(0.0),
                            };
                            let _ = sender.send(error_response);
                        }
                    }
                }
                _ => {
                    let error_response = CandleCompletionResponse {
                        text: format!("Unsupported model: {model_name}. Supported models: kimi-k2, qwen3-coder").into(),
                        model: model_name.into(),
                        provider: Some("candle-local".into()),
                        usage: None,
                        finish_reason: Some("error".into()),
                        response_time_ms: None,
                        generation_time_ms: Some(0),
                        tokens_per_second: Some(0.0),
                    };
                    let _ = sender.send(error_response);
                }
            }
        })
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