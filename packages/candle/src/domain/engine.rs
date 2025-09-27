//! Local Candle Engine
//!
//! Provides local model inference engine using the Candle ML framework.
//! This engine routes requests to appropriate local model providers without any cloud dependencies.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use ystream::AsyncStream;

use crate::domain::completion::CandleCompletionResponse;
// use crate::domain::completion::traits::CandleCompletionModel; // Reserved for future completion model integration
#[cfg(feature = "progresshub")]
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
        _prompt: String,
    ) -> AsyncStream<CandleCompletionResponse<'static>> {
        // Increment request counter
        self.request_count.fetch_add(1, Ordering::Relaxed);

        // Capture values from self before creating the async stream
        let model_name = self.config.model_name.clone();
        let _max_tokens = self.config.max_tokens;
        let _temperature = self.config.temperature;

        AsyncStream::with_channel(move |sender| {

            // Route to appropriate provider based on model name
            match model_name.as_str() {
                "kimi-k2" => {
                    // Create runtime for async provider creation
                    let _rt = match tokio::runtime::Runtime::new() {
                        Ok(_rt) => _rt,
                        Err(e) => {
                            let error_response = CandleCompletionResponse {
                                text: format!("Failed to create async runtime: {}", e).into(),
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

                    // Use KimiK2 provider with real inference
                    #[cfg(feature = "progresshub")]
                    match _rt.block_on(CandleKimiK2Provider::new()) {
                        Ok(provider) => {
                            // Create completion parameters
                            let completion_params = crate::domain::completion::CandleCompletionParams {
                                temperature: _temperature.unwrap_or(0.7) as f64,
                                max_tokens: _max_tokens.map(|t| std::num::NonZeroU64::new(t as u64).unwrap()),
                                n: std::num::NonZeroU8::new(1).unwrap(),
                                stream: true,
                                additional_params: None,
                            };
                            
                            // Create prompt
                            let candle_prompt = crate::domain::prompt::CandlePrompt::new(prompt.clone());
                            
                            // Get real completion stream from provider
                            let completion_stream = provider.prompt(candle_prompt, &completion_params);
                            
                            // Process completion stream using proper ystream pattern
                            ystream::spawn_task(move || async move {
                                let mut stream = completion_stream;
                                while let Some(chunk) = stream.next().await {
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
                                                text: format!("Error: {}", err).into(),
                                                model: model_name.clone().into(),
                                                provider: Some("candle-local".into()),
                                                usage: None,
                                                finish_reason: Some("error".into()),
                                                response_time_ms: None,
                                                generation_time_ms: None,
                                                tokens_per_second: None,
                                            }
                                        },
                                    };
                                    
                                    if sender.send(response).is_err() {
                                        break; // Client disconnected
                                    }
                                }
                            });
                        }
                        Err(e) => {
                            let error_response = CandleCompletionResponse {
                                text: format!("Error creating Kimi-K2 provider: {}", e).into(),
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

                    #[cfg(not(feature = "progresshub"))]
                    {
                        let error_response = CandleCompletionResponse {
                            text: "KimiK2 provider requires progresshub feature to be enabled".into(),
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
                "qwen3-coder" => {
                    // Create runtime for async provider creation
                    let _rt = match tokio::runtime::Runtime::new() {
                        Ok(_rt) => _rt,
                        Err(e) => {
                            let error_response = CandleCompletionResponse {
                                text: format!("Failed to create async runtime: {}", e).into(),
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

                    // Use Qwen3Coder provider
                    #[cfg(feature = "progresshub")]
                    match _rt.block_on(CandleQwen3CoderProvider::new()) {
                        Ok(provider) => {
                            // Create completion parameters for Qwen3-Coder
                            let completion_params = crate::domain::completion::CandleCompletionParams {
                                temperature: _temperature.unwrap_or(0.7) as f64,
                                max_tokens: _max_tokens.map(|t| std::num::NonZeroU64::new(t as u64).unwrap()),
                                n: std::num::NonZeroU8::new(1).unwrap(),
                                stream: true,
                                additional_params: None,
                            };

                            // Create prompt for Qwen3-Coder
                            let candle_prompt = crate::domain::prompt::CandlePrompt::new(prompt.clone());

                            // Get real completion stream from Qwen3-Coder provider
                            let completion_stream = provider.prompt(candle_prompt, &completion_params);

                            // Process completion stream using proper ystream pattern
                            ystream::spawn_task(move || async move {
                                use futures_util::StreamExt;

                                let mut stream = completion_stream;
                                while let Some(chunk) = stream.next().await {
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
                                                text: format!("Error: {}", err).into(),
                                                model: model_name.clone().into(),
                                                provider: Some("candle-local".into()),
                                                usage: None,
                                                finish_reason: Some("error".into()),
                                                response_time_ms: None,
                                                generation_time_ms: None,
                                                tokens_per_second: None,
                                            }
                                        },
                                        _ => continue, // Skip other chunk types
                                    };

                                    if sender.send(response).is_err() {
                                        break; // Client disconnected
                                    }
                                }
                            });
                        }
                        Err(e) => {
                            let error_response = CandleCompletionResponse {
                                text: format!("Error creating Qwen3-Coder provider: {}", e).into(),
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

                    #[cfg(not(feature = "progresshub"))]
                    {
                        let error_response = CandleCompletionResponse {
                            text: "Qwen3-Coder provider requires progresshub feature to be enabled".into(),
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
                _ => {
                    let error_response = CandleCompletionResponse {
                        text: format!("Unsupported model: {}. Supported models: kimi-k2, qwen3-coder", model_name).into(),
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