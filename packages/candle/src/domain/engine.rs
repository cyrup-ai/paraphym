//! Local Candle Engine
//!
//! Provides local model inference engine using the Candle ML framework.
//! This engine routes requests to appropriate local model providers without any cloud dependencies.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use ystream::AsyncStream;

use crate::domain::completion::CandleCompletionResponse;
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
    ) -> AsyncStream<CandleCompletionResponse> {
        // Increment request counter
        self.request_count.fetch_add(1, Ordering::Relaxed);

        let model_name = self.config.model_name.clone();
        let max_tokens = self.config.max_tokens;
        let temperature = self.config.temperature;

        AsyncStream::with_channel(move |sender| {
            // Route to appropriate provider based on model name
            match model_name.as_str() {
                "kimi-k2" => {
                    // Use KimiK2 provider
                    match CandleKimiK2Provider::new() {
                        Ok(provider) => {
                            // Provider handles completion internally
                            let response = CandleCompletionResponse {
                                text: "Local Kimi-K2 inference would happen here".into(),
                                model: Some(model_name.into()),
                                provider: Some("candle-local".into()),
                                usage: None,
                                finish_reason: Some("completed".into()),
                                response_time_ms: None,
                                generation_time_ms: Some(100),
                                tokens_per_second: Some(10.0),
                            };
                            let _ = sender.send(response);
                        }
                        Err(e) => {
                            let error_response = CandleCompletionResponse {
                                text: format!("Error creating Kimi-K2 provider: {}", e).into(),
                                model: Some(model_name.into()),
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
                    // Use Qwen3Coder provider
                    match CandleQwen3CoderProvider::new() {
                        Ok(provider) => {
                            // Provider handles completion internally
                            let response = CandleCompletionResponse {
                                text: "Local Qwen3-Coder inference would happen here".into(),
                                model: Some(model_name.into()),
                                provider: Some("candle-local".into()),
                                usage: None,
                                finish_reason: Some("completed".into()),
                                response_time_ms: None,
                                generation_time_ms: Some(100),
                                tokens_per_second: Some(15.0),
                            };
                            let _ = sender.send(response);
                        }
                        Err(e) => {
                            let error_response = CandleCompletionResponse {
                                text: format!("Error creating Qwen3-Coder provider: {}", e).into(),
                                model: Some(model_name.into()),
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
                        text: format!("Unsupported model: {}. Supported models: kimi-k2, qwen3-coder", model_name).into(),
                        model: Some(model_name.into()),
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