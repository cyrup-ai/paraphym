//! Context extraction using agents for structured data extraction
//!
//! This module provides extraction capabilities for processing unstructured text
//! into structured data using AI agents and completion models.

use std::fmt;
use serde::de::DeserializeOwned;
use ystream::AsyncStream;

use crate::domain::agent::Agent;
use crate::domain::completion::{CompletionRequest, CompletionParams};
use crate::domain::model::prompt::{Prompt, MessageRole};
use crate::domain::model::completion::AgentCompletionModel;

/// Result type for extraction operations
type ExtractionResult<T> = Result<T, ExtractionError>;

/// Error types for extraction operations
#[derive(Debug, thiserror::Error)]
pub enum ExtractionError {
    /// Model inference error
    #[error("Model inference failed: {0}")]
    ModelError(String),
    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

/// Generic extractor trait for structured data extraction
pub trait Extractor<T> {
    /// Extract structured data from text
    fn extract(&self, text: &str) -> AsyncStream<ExtractionResult<T>>;
}

/// Document extractor for file-based extraction
pub struct DocumentExtractor<T> 
where
    T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + Default + 'static,
{
    provider: Option<std::sync::Arc<dyn crate::domain::completion::traits::CandleCompletionModel>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + Default + 'static> Default for DocumentExtractor<T> {
    fn default() -> Self {
        Self {
            provider: None,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + Default + 'static> Extractor<T> for DocumentExtractor<T> {
    fn extract(&self, text: &str) -> AsyncStream<ExtractionResult<T>> {
        let text = text.to_string();
        let provider = self.provider.clone();
        
        AsyncStream::with_channel(move |sender| {
            std::thread::spawn(move || {
                if let Some(provider) = provider {
                    let prompt = crate::domain::prompt::CandlePrompt::new(
                        format!("Extract structured data from: {}", text)
                    );
                    let params = crate::domain::completion::types::CandleCompletionParams {
                        temperature: 0.2,
                        max_tokens: std::num::NonZeroU64::new(2000),
                        n: match std::num::NonZeroU8::new(1) {
                            Some(n) => n,
                            None => {
                                let _ = sender.send(Err(ExtractionError::ConfigError("Invalid completion parameter".to_string())));
                                return;
                            }
                        },
                        stream: true,
                        tools: None,
                        additional_params: None,
                    };
                    
                    let mut stream = provider.prompt(prompt, &params);
                    let mut accumulated = String::new();
                    
                    while let Some(chunk) = stream.try_next() {
                        use crate::domain::context::chunk::CandleCompletionChunk;
                        match chunk {
                            CandleCompletionChunk::Text(text) => accumulated.push_str(&text),
                            CandleCompletionChunk::Complete { text, .. } => {
                                accumulated.push_str(&text);
                                break;
                            }
                            CandleCompletionChunk::Error(e) => {
                                let _ = sender.send(Err(ExtractionError::ModelError(e)));
                                return;
                            }
                            _ => {}
                        }
                    }
                    
                    match serde_json::from_str::<T>(&accumulated) {
                        Ok(result) => { let _ = sender.send(Ok(result)); }
                        Err(e) => { let _ = sender.send(Err(ExtractionError::SerializationError(e.to_string()))); }
                    }
                } else {
                    let _ = sender.send(Err(ExtractionError::ConfigError("No provider configured".to_string())));
                }
            });
        })
    }
}

/// Context extractor implementation
pub struct ExtractorImpl<T> 
where
    T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + Default + 'static,
{
    _phantom: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + Default + 'static> Default for ExtractorImpl<T> {
    fn default() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + Default + 'static> ExtractorImpl<T> {
    /// Execute extraction with agent (planned feature)
    pub fn execute_extraction(
        agent: Agent,
        completion_request: CompletionRequest,
        _text_input: String,
    ) -> AsyncStream<ExtractionResult<T>> {
        AsyncStream::with_channel(move |sender| {
            std::thread::spawn(move || {
                let model = AgentCompletionModel::new(agent);
                let prompt = Prompt {
                    content: completion_request.system.as_deref().unwrap_or("").to_string(),
                    role: MessageRole::System};
                let params = CompletionParams {
                    temperature: completion_request.temperature.unwrap_or(0.2),
                    max_tokens: completion_request.max_tokens.and_then(|t| std::num::NonZeroU64::new(t as u64)),
                    n: std::num::NonZeroU8::MIN,
                    stream: true};
                let mut stream = model.prompt(prompt, &params);

                let mut full_response = String::new();

                while let Some(chunk) = stream.try_next() {
                    use crate::domain::context::chunk::CandleCompletionChunk;
                    match chunk {
                        CandleCompletionChunk::Text(text) => {
                            full_response.push_str(&text);
                        }
                        CandleCompletionChunk::Complete { text, finish_reason, .. } => {
                            full_response.push_str(&text);
                            if finish_reason == Some(crate::domain::context::chunk::FinishReason::Stop) {
                                break;
                            }
                        }
                        CandleCompletionChunk::Error(e) => {
                            let _ = sender.send(Err(ExtractionError::ModelError(e)));
                            return;
                        }
                        _ => {}
                    }
                }

                match serde_json::from_str::<T>(&full_response) {
                    Ok(result) => { let _ = sender.send(Ok(result)); }
                    Err(e) => { let _ = sender.send(Err(ExtractionError::SerializationError(e.to_string()))); }
                }
            });
        })
    }

    /// Extract structured data with streaming support
    pub fn extract_with_streaming(
        &self, 
        text: &str,
        agent: Agent,
    ) -> AsyncStream<ExtractionResult<T>> {
        let text = text.to_string();
        AsyncStream::with_channel(move |sender| {
            std::thread::spawn(move || {
                // Create completion request for extraction
                let completion_request = CompletionRequest::new(&text);
                
                // Execute extraction using the agent
                let mut extraction_stream = Self::execute_extraction(agent, completion_request, text);
                
                // Forward results from the extraction
                while let Some(result) = extraction_stream.try_next() {
                    let _ = sender.send(result);
                }
            });
        })
    }
}

impl<T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + Default + 'static> Extractor<T> for ExtractorImpl<T> {
    fn extract(&self, text: &str) -> AsyncStream<ExtractionResult<T>> {
        let text = text.to_string();
        AsyncStream::with_channel(move |sender| {
            std::thread::spawn(move || {
                let agent = crate::domain::agent::Agent::default();
                let completion_request = CompletionRequest::new(&text);
                
                let mut extraction_stream = Self::execute_extraction(agent, completion_request, text);
                
                while let Some(result) = extraction_stream.try_next() {
                    let _ = sender.send(result);
                }
            });
        })
    }
}

/// Batch extractor for processing multiple documents
pub struct BatchExtractor<T> 
where
    T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + Default + 'static,
{
    extractor: ExtractorImpl<T>,
}

impl<T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + Default + 'static> BatchExtractor<T> {
    /// Create a new batch extractor
    pub fn new() -> Self {
        Self {
            extractor: ExtractorImpl::default(),
        }
    }

    /// Extract from multiple texts
    pub fn extract_batch(&self, texts: Vec<String>) -> AsyncStream<Vec<ExtractionResult<T>>> {
        AsyncStream::with_channel(move |sender| {
            std::thread::spawn(move || {
                let mut results = Vec::with_capacity(texts.len());
                
                for text in texts {
                    let mut extraction_stream = self.extractor.extract(&text);
                    if let Some(result) = extraction_stream.try_next() {
                        results.push(result);
                    } else {
                        results.push(Err(ExtractionError::ModelError("Extraction failed".to_string())));
                    }
                }
                
                let _ = sender.send(results);
            });
        })
    }
}

impl<T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + Default + 'static> Default for BatchExtractor<T> {
    fn default() -> Self {
        Self::new()
    }
}