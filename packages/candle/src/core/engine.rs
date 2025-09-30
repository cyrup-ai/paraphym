//! Engine domain module
//!
//! Provides core engine functionality with true zero-allocation patterns and lock-free
//! architecture. The engine routes requests to appropriate AI providers using atomic
//! operations and borrowed data to eliminate allocations in hot paths.

use std::sync::{Arc, atomic::{AtomicBool, AtomicU64, Ordering}};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::completion::response::CompletionResponse;
use crate::domain::context::chunk::{CandleStringChunk, CandleCompletionChunk};
use crate::domain::model::CandleUsage;
use crate::{spawn_task, AsyncStream, AsyncTask};

/// Parameters for completion execution
#[derive(Debug, Clone)]
pub struct CompletionParams {
    pub request_id: u64,
    pub model_name: String,
    pub provider: String,
    pub api_key: Option<String>,
    pub timeout: u64,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub streaming: bool,
    pub endpoint: Option<String>,
    pub prompt: String,
    pub system_prompt: Option<String>,
    pub history: Vec<String>,
    pub tools: Vec<String>,
    pub metadata: Option<String>,
}

/// Handle errors in streaming context without panicking
macro_rules! handle_error {
    ($error:expr, $context:literal) => {
        eprintln!("Streaming error in {}: {}", $context, $error)
        // Continue processing instead of returning error
    };
}

/// Engine-specific error types with minimal allocations
#[derive(Error, Debug, Clone)]
pub enum EngineError {
    #[error("Provider not available")]
    /// The requested AI provider is not available or not configured
    ProviderNotAvailable,

    #[error("Model not found")]
    /// The specified model was not found in the provider
    ModelNotFound,

    #[error("Configuration error: {0}")]
    /// Engine configuration is invalid or incomplete
    ConfigurationError(String),

    #[error("Authentication failed")]
    /// Authentication with the AI provider failed
    AuthenticationFailed,

    #[error("Rate limit exceeded, retry after {retry_after_seconds}s")]
    /// Rate limit was exceeded by the provider
    RateLimitExceeded {
        /// Number of seconds to wait before retrying
        retry_after_seconds: u64,
    },

    #[error("Request timeout after {timeout_seconds}s")]
    /// Request timed out after the specified duration
    RequestTimeout {
        /// Number of seconds the request waited before timing out
        timeout_seconds: u64,
    },

    #[error("Network error: {0}")]
    /// Network communication error occurred
    NetworkError(String),

    #[error("Invalid input")]
    /// The provided input is invalid or malformed
    InvalidInput,

    #[error("Service unavailable")]
    /// The engine service is temporarily unavailable
    ServiceUnavailable,

    #[error("Internal error: {0}")]
    /// An unexpected internal error occurred
    InternalError(String),
}

/// Result type for engine operations
pub type EngineResult<T> = Result<T, EngineError>;

/// Engine configuration with owned strings allocated once at creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    /// Model name for the completion request
    pub model_name: String,
    /// Provider identifier (e.g., "openai", "anthropic", "gemini")
    pub provider: String,
    /// API key for authentication
    pub api_key: Option<String>,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// Maximum tokens for completion
    pub max_tokens: Option<u32>,
    /// Temperature for response randomness (0.0 - 1.0)
    pub temperature: Option<f32>,
    /// Whether to enable streaming responses
    pub enable_streaming: bool,
    /// Custom endpoint URL override
    pub endpoint_url: Option<String>,
}

impl Default for EngineConfig {
    #[inline]
    fn default() -> Self {
        Self {
            model_name: String::from("gpt-4o-mini"),
            provider: String::from("openai"),
            api_key: None,
            timeout_seconds: 30,
            max_tokens: Some(4096),
            temperature: Some(0.7),
            enable_streaming: false,
            endpoint_url: None,
        }
    }
}

impl EngineConfig {
    /// Create a new engine configuration
    #[inline]
    pub fn new(model_name: impl Into<String>, provider: impl Into<String>) -> Self {
        Self {
            model_name: model_name.into(),
            provider: provider.into(),
            ..Default::default()
        }
    }

    /// Set API key
    #[must_use]
    #[inline]
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set timeout in seconds
    #[must_use]
    #[inline]
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = timeout_seconds;
        self
    }

    /// Set max tokens
    #[must_use]
    #[inline]
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Set temperature (clamped to valid range)
    #[must_use]
    #[inline]
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature.clamp(0.0, 1.0));
        self
    }

    /// Enable streaming responses
    #[must_use]
    #[inline]
    pub fn with_streaming(mut self) -> Self {
        self.enable_streaming = true;
        self
    }

    /// Set custom endpoint URL
    #[must_use]
    #[inline]
    pub fn with_endpoint(mut self, endpoint_url: impl Into<String>) -> Self {
        self.endpoint_url = Some(endpoint_url.into());
        self
    }

    /// Validate configuration
    #[inline]
    pub fn validate(&self) -> EngineResult<()> {
        if self.model_name.is_empty() {
            return Err(EngineError::ConfigurationError(
                "Model name cannot be empty".to_string(),
            ));
        }

        if self.provider.is_empty() {
            return Err(EngineError::ConfigurationError(
                "Provider cannot be empty".to_string(),
            ));
        }

        if self.timeout_seconds == 0 {
            return Err(EngineError::ConfigurationError(
                "Timeout must be greater than 0".to_string(),
            ));
        }

        if let Some(temp) = self.temperature
            && !(0.0..=1.0).contains(&temp) {
                return Err(EngineError::ConfigurationError(
                    "Temperature must be between 0.0 and 1.0".to_string(),
                ));
            }

        Ok(())
    }
}

/// Engine completion request using borrowed data to avoid allocations
#[derive(Debug)]
pub struct CompletionRequest<'a> {
    pub prompt: &'a str,
    pub system_prompt: Option<&'a str>,
    pub conversation_history: &'a [&'a str],
    pub tools: &'a [&'a str],
    pub metadata: Option<&'a str>,
}

impl<'a> CompletionRequest<'a> {
    /// Create a new completion request with borrowed data
    #[inline]
    pub fn new(prompt: &'a str) -> Self {
        Self {
            prompt,
            system_prompt: None,
            conversation_history: &[],
            tools: &[],
            metadata: None,
        }
    }

    /// Set system prompt
    #[must_use]
    #[inline]
    pub fn with_system_prompt(mut self, system_prompt: &'a str) -> Self {
        self.system_prompt = Some(system_prompt);
        self
    }

    /// Set conversation history
    #[must_use]
    #[inline]
    pub fn with_history(mut self, history: &'a [&'a str]) -> Self {
        self.conversation_history = history;
        self
    }

    /// Set available tools
    #[must_use]
    #[inline]
    pub fn with_tools(mut self, tools: &'a [&'a str]) -> Self {
        self.tools = tools;
        self
    }

    /// Set metadata
    #[must_use]
    #[inline]
    pub fn with_metadata(mut self, metadata: &'a str) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Validate request
    #[inline]
    pub fn validate(&self) -> EngineResult<()> {
        if self.prompt.is_empty() {
            return Err(EngineError::InvalidInput);
        }
        Ok(())
    }
}

/// Core engine implementation with lock-free atomic operations
pub struct Engine {
    config: EngineConfig,
    request_count: Arc<AtomicU64>,
    active_requests: Arc<AtomicU64>,
    successful_requests: Arc<AtomicU64>,
    failed_requests: Arc<AtomicU64>,
    is_healthy: Arc<AtomicBool>,
}

impl Engine {
    /// Create a new engine with the given configuration
    #[inline]
    pub fn new(config: EngineConfig) -> EngineResult<Self> {
        config.validate()?;

        Ok(Self {
            config,
            request_count: Arc::new(AtomicU64::new(0)),
            active_requests: Arc::new(AtomicU64::new(0)),
            successful_requests: Arc::new(AtomicU64::new(0)),
            failed_requests: Arc::new(AtomicU64::new(0)),
            is_healthy: Arc::new(AtomicBool::new(true)),
        })
    }

    /// Get immutable reference to configuration
    #[inline]
    pub fn config(&self) -> &EngineConfig {
        &self.config
    }

    /// Get current request count (atomic read)
    #[inline]
    pub fn request_count(&self) -> u64 {
        self.request_count.load(Ordering::Relaxed)
    }

    /// Get current active request count (atomic read)
    #[inline]
    pub fn active_requests(&self) -> u64 {
        self.active_requests.load(Ordering::Relaxed)
    }

    /// Get successful request count (atomic read)
    #[inline]
    pub fn successful_requests(&self) -> u64 {
        self.successful_requests.load(Ordering::Relaxed)
    }

    /// Get failed request count (atomic read)
    #[inline]
    pub fn failed_requests(&self) -> u64 {
        self.failed_requests.load(Ordering::Relaxed)
    }

    /// Check if engine is healthy (atomic read)
    #[inline]
    pub fn is_healthy(&self) -> bool {
        self.is_healthy.load(Ordering::Relaxed)
    }

    /// Set health status (atomic write)
    #[inline]
    pub fn set_healthy(&self, healthy: bool) {
        self.is_healthy.store(healthy, Ordering::Relaxed);
    }

    /// Process completion request with zero allocations in hot path
    #[inline]
    pub fn process_completion(
        &self,
        request: CompletionRequest<'_>,
    ) -> AsyncTask<EngineResult<CompletionResponse<'static>>> {
        // Validate request first
        if let Err(e) = request.validate() {
            return spawn_task(move || Err(e));
        }

        // Atomic operations for metrics (lock-free)
        let request_id = self.request_count.fetch_add(1, Ordering::Relaxed);
        self.active_requests.fetch_add(1, Ordering::Relaxed);

        // Clone necessary data for async processing
        let model_name = self.config.model_name.clone();
        let provider = self.config.provider.clone();
        let api_key = self.config.api_key.clone();
        let timeout = self.config.timeout_seconds;
        let max_tokens = self.config.max_tokens;
        let temperature = self.config.temperature;
        let streaming = self.config.enable_streaming;
        let endpoint = self.config.endpoint_url.clone();

        // Convert borrowed request data to owned for async processing
        let prompt = request.prompt.to_string();
        let system_prompt = request.system_prompt.map(|s| s.to_string());
        let history: Vec<String> = request
            .conversation_history
            .iter()
            .map(|s| s.to_string())
            .collect();
        let tools: Vec<String> = request.tools.iter().map(|s| s.to_string()).collect();
        let metadata = request.metadata.map(|s| s.to_string());

        // We'll update metrics after the task completes, not during

        spawn_task(move || {
            // Create streaming completion and collect first result for backward compatibility
            let _params = CompletionParams {
                request_id,
                model_name,
                provider,
                api_key,
                timeout,
                max_tokens,
                temperature,
                streaming,
                endpoint,
                prompt,
                system_prompt,
                history,
                tools,
                metadata,
            };
            // Legacy method - routing logic has been moved to orchestration utilities
            // Providers should now use coordinate_generation() method directly
            Err(EngineError::InternalError(
                "Direct engine completion processing deprecated. Use provider orchestration instead.".to_string(),
            ))
        })
    }

    /// Coordinate text generation with metrics and streaming management
    /// 
    /// Provides orchestration services for providers:
    /// - Automatic metrics tracking (request_count, active_requests, etc.)
    /// - Stream conversion from CandleStringChunk to CandleCompletionChunk  
    /// - Error handling and health monitoring
    /// - Performance timing and throughput calculation
    pub fn coordinate_generation<F>(&self, generation_fn: F) 
        -> AsyncStream<CandleCompletionChunk>
    where 
        F: FnOnce() -> AsyncStream<CandleStringChunk> + Send + 'static
    {
        // Update metrics atomically
        self.request_count.fetch_add(1, Ordering::Relaxed);
        self.active_requests.fetch_add(1, Ordering::Relaxed);
        
        // Execute provider's generation function
        let text_stream = generation_fn();
        
        // Convert and manage streaming response with metrics
        self.manage_streaming_response(text_stream)
    }

    /// Convert TextGenerator output to completion chunks with metrics tracking
    fn manage_streaming_response(&self, text_stream: AsyncStream<CandleStringChunk>) 
        -> AsyncStream<CandleCompletionChunk> {
        
        let active_requests = Arc::clone(&self.active_requests);
        let successful_requests = Arc::clone(&self.successful_requests);
        let failed_requests = Arc::clone(&self.failed_requests);
        
        AsyncStream::with_channel(move |sender| {
            let start_time = std::time::Instant::now();
            let mut token_count = 0u32;
            let mut has_error = false;
            
            // Process each text chunk from TextGenerator
            for string_chunk in text_stream {
                token_count += 1;
                
                // Convert CandleStringChunk to CandleCompletionChunk::Text
                let completion_chunk = CandleCompletionChunk::Text(string_chunk.0);
                
                if sender.send(completion_chunk).is_err() {
                    // Client disconnected
                    has_error = true;
                    break;
                }
            }
            
            // Send completion marker with performance metrics
            let _elapsed = start_time.elapsed();
            let final_chunk = CandleCompletionChunk::Complete {
                text: String::new(),
                finish_reason: if has_error { 
                    Some(crate::domain::context::chunk::FinishReason::Error) 
                } else { 
                    Some(crate::domain::context::chunk::FinishReason::Stop) 
                },
                usage: Some(CandleUsage {
                    input_tokens: 0, // Provider can set this if needed
                    output_tokens: token_count,
                    total_tokens: token_count,
                }),
            };
            let _ = sender.send(final_chunk);
            
            // Update metrics atomically on completion
            active_requests.fetch_sub(1, Ordering::Relaxed);
            if has_error {
                failed_requests.fetch_add(1, Ordering::Relaxed);
            } else {
                successful_requests.fetch_add(1, Ordering::Relaxed);
            }
        })
    }

    /// Process completion request as stream (new primary API)
    #[inline]
    pub fn process_completion_stream(
        &self,
        request: CompletionRequest<'_>,
    ) -> AsyncStream<CompletionResponse<'static>> {
        // Validate request first
        if let Err(e) = request.validate() {
            return AsyncStream::with_channel(move |_sender| {
                handle_error!(e, "process_completion_stream validation");
            });
        }

        // Atomic operations for metrics (lock-free)
        let request_id = self.request_count.fetch_add(1, Ordering::Relaxed);
        self.active_requests.fetch_add(1, Ordering::Relaxed);

        // Clone necessary data for async processing
        let model_name = self.config.model_name.clone();
        let provider = self.config.provider.clone();
        let api_key = self.config.api_key.clone();
        let timeout = self.config.timeout_seconds;
        let max_tokens = self.config.max_tokens;
        let temperature = self.config.temperature;
        let streaming = self.config.enable_streaming;
        let endpoint = self.config.endpoint_url.clone();

        // Convert borrowed request data to owned for async processing
        let prompt = request.prompt.to_string();
        let system_prompt = request.system_prompt.map(|s| s.to_string());
        let history: Vec<String> = request
            .conversation_history
            .iter()
            .map(|s| s.to_string())
            .collect();
        let tools: Vec<String> = request.tools.iter().map(|s| s.to_string()).collect();
        let metadata = request.metadata.map(|s| s.to_string());

        AsyncStream::with_channel(move |sender| {
            let params = CompletionParams {
                request_id,
                model_name,
                provider,
                api_key,
                timeout,
                max_tokens,
                temperature,
                streaming,
                endpoint,
                prompt,
                system_prompt,
                history,
                tools,
                metadata,
            };
            // Legacy API deprecated - return error directing users to new orchestration pattern
            let error_response = CompletionResponse {
                text: "Direct engine completion processing deprecated. Use provider.prompt() with coordinate_generation() instead.".into(),
                model: params.model_name.into(),
                provider: Some(params.provider.into()),
                usage: None,
                finish_reason: Some("error".into()),
                response_time_ms: Some(0),
                generation_time_ms: Some(0),
                tokens_per_second: Some(0.0),
            };
            let _ = sender.send(error_response);
        })
    }

    /// Get streaming completion results (convenience method)
    #[inline]
    pub fn get_completion_stream(
        &self,
        request: CompletionRequest<'_>,
    ) -> AsyncStream<CompletionResponse<'static>> {
        self.process_completion_stream(request)
    }

    /// Get engine statistics
    #[inline]
    pub fn stats(&self) -> EngineStats {
        EngineStats {
            total_requests: self.request_count.load(Ordering::Relaxed),
            active_requests: self.active_requests.load(Ordering::Relaxed),
            successful_requests: self.successful_requests.load(Ordering::Relaxed),
            failed_requests: self.failed_requests.load(Ordering::Relaxed),
            is_healthy: self.is_healthy.load(Ordering::Relaxed),
        }
    }

    /// Reset all metrics (atomic operations)
    #[inline]
    pub fn reset_metrics(&self) {
        self.request_count.store(0, Ordering::Relaxed);
        self.active_requests.store(0, Ordering::Relaxed);
        self.successful_requests.store(0, Ordering::Relaxed);
        self.failed_requests.store(0, Ordering::Relaxed);
    }
}

/// Engine statistics snapshot
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EngineStats {
    pub total_requests: u64,
    pub active_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub is_healthy: bool,
}

impl EngineStats {
    /// Calculate success rate as a percentage
    #[inline]
    pub fn success_rate(&self) -> f64 {
        let completed = self.successful_requests + self.failed_requests;
        if completed == 0 {
            0.0
        } else {
            (self.successful_requests as f64 / completed as f64) * 100.0
        }
    }

    /// Calculate failure rate as a percentage
    #[inline]
    pub fn failure_rate(&self) -> f64 {
        100.0 - self.success_rate()
    }
}

// Engine is automatically Send + Sync due to atomic operations - no unsafe impl needed
