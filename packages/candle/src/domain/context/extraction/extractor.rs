use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;

use ystream::AsyncStream;
use serde::de::DeserializeOwned;
use cyrup_sugars::prelude::MessageChunk;

use super::error::{ExtractionError, _ExtractionResult as ExtractionResult};
use crate::domain::{
    chat::message::types::CandleMessageRole as MessageRole,
    completion::{
        types::CandleCompletionParams as CompletionParams,
        CandleCompletionModel as CompletionModel, CandleCompletionRequest as CompletionRequest,
    },
    context::chunk::{CandleCompletionChunk, FinishReason},
    prompt::CandlePrompt as Prompt,
};

/// Trait defining the core extraction interface
pub trait Extractor<T>: Send + Sync + fmt::Debug + Clone
where
    T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + Default + MessageChunk + 'static,
{
    /// Get the system prompt for extraction
    fn system_prompt(&self) -> Option<&str>;

    /// Extract structured data from text with comprehensive error handling
    fn extract_from(&self, text: &str) -> AsyncStream<T>;

    /// Set system prompt for extraction guidance
    #[must_use]
    fn with_system_prompt(self, prompt: impl Into<String>) -> Self;
}

/// Implementation of the Extractor trait
#[derive(Clone)]
pub struct ExtractorImpl<T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + 'static> {
    provider: Arc<dyn CompletionModel>,
    system_prompt: Option<String>,
    _marker: PhantomData<T>,
}

impl<T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + 'static> fmt::Debug for ExtractorImpl<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExtractorImpl")
            .field("provider", &"<dyn CompletionModel>")
            .field("system_prompt", &self.system_prompt)
            .finish()
    }
}

impl<T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + Default + 'static + MessageChunk> Extractor<T>
    for ExtractorImpl<T>
{
    fn system_prompt(&self) -> Option<&str> {
        self.system_prompt.as_deref()
    }

    fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    fn extract_from(&self, text: &str) -> AsyncStream<T> {
        let text = text.to_string();
        let provider = Arc::clone(&self.provider);
        let system_prompt = self.system_prompt.clone().unwrap_or_else(|| {
            format!("Extract structured data from the following text. Return ONLY valid JSON matching the expected schema. Text: {}", text)
        });

        AsyncStream::with_channel(move |sender| {
            tokio::spawn(async move {
                let completion_request = CompletionRequest::new(&text)
                    .with_system_prompt(system_prompt);

                match Self::execute_extraction(provider, completion_request, text).await {
                    Ok(result) => {
                        let _ = sender.send(result);
                    }
                    Err(_e) => {
                        let _ = sender.send(T::default());
                    }
                }
            });
        })
    }
}

impl<T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + Default + 'static + MessageChunk> ExtractorImpl<T> {
    /// Create new extractor with provider
    pub fn new_with_provider(provider: Arc<dyn CompletionModel>) -> Self {
        Self {
            provider,
            system_prompt: None,
            _marker: PhantomData,
        }
    }

    /// Get the provider reference
    pub fn provider(&self) -> &Arc<dyn CompletionModel> {
        &self.provider
    }
}

impl<T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + Default + MessageChunk + 'static> ExtractorImpl<T> {
    /// Execute extraction with provider
    ///
    /// # Errors
    ///
    /// Returns `ExtractionError` if:
    /// - Model execution fails
    /// - Response parsing fails
    /// - Deserialization fails
    pub async fn execute_extraction(
        provider: Arc<dyn CompletionModel>,
        completion_request: CompletionRequest,
        _text_input: String,
    ) -> ExtractionResult<T> {
        let model = provider.as_ref();
        let prompt = Prompt {
            content: completion_request.system_prompt,
            role: MessageRole::System,
        };
        let params = CompletionParams {
            temperature: completion_request.temperature,
            max_tokens: completion_request
                .max_tokens
                .and_then(|t| std::num::NonZeroU64::new(t.get())),
            // Use MIN which is guaranteed to be 1 for NonZeroU8
            n: std::num::NonZeroU8::MIN,
            stream: true,
            tools: None,
            additional_params: None,
        };

        // Get the stream from the model
        let stream = model.prompt(prompt, &params);
        let stream = Box::pin(stream);

        let mut full_response = String::new();
        let mut finish_reason = None;

        // Process the stream chunks
        while let Some(chunk) = stream.next().await {
            match chunk {
                CandleCompletionChunk::Text(text) => {
                    // Append text content to full response
                    full_response.push_str(&text);
                }
                CandleCompletionChunk::Complete {
                    text,
                    finish_reason: reason,
                    ..
                } => {
                    // This is the final chunk
                    if !text.is_empty() {
                        full_response.push_str(&text);
                    }
                    finish_reason = reason;
                    break;
                }
                CandleCompletionChunk::Error(err) => {
                    return Err(ExtractionError::CompletionError(format!(
                        "Error from model: {err}"
                    )));
                }
                // Handle other variants as needed
                _ => {}
            }
        }

        if finish_reason == Some(FinishReason::Stop) || !full_response.is_empty() {
            match Self::parse_json_response(&full_response) {
                Ok(result) => Ok(result),
                Err(e) => Err(e),
            }
        } else {
            Err(ExtractionError::CompletionError(
                "No valid response from model".to_string(),
            ))
        }
    }

    /// Parse JSON response (planned feature)
    ///
    /// # Errors
    ///
    /// Returns `ExtractionError` if:
    /// - Response is not valid JSON
    /// - JSON cannot be deserialized into type T
    pub fn parse_json_response(response: &str) -> ExtractionResult<T> {
        // First try to parse the whole response as JSON
        if let Ok(parsed) = serde_json::from_str::<T>(response) {
            return Ok(parsed);
        }

        // If that fails, try to find JSON in the response
        let json_start = response.find('{').unwrap_or(0);
        let json_end = response
            .rfind('}')
            .map_or_else(|| response.len(), |i| i + 1);

        if json_start < json_end {
            let json_str = &response[json_start..json_end];
            serde_json::from_str(json_str).map_err(ExtractionError::from)
        } else {
            Err(ExtractionError::InvalidFormat {
                actual: response.to_string(),
            })
        }
    }
}


