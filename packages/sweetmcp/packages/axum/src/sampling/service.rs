// use futures_util::StreamExt; // Temporarily unused
use std::num::NonZeroU64;

use log::{self, error};
use rpc_router::HandlerResult;
use tokio::sync::{mpsc, oneshot};

// Candle inference imports
use cyrup_candle::capability::registry::{self, TextToTextModel};
use cyrup_candle::capability::traits::TextToTextCapable;
use cyrup_candle::domain::completion::{
    CandleCompletionParams, CandleCompletionChunk,
};
use cyrup_candle::domain::prompt::CandlePrompt;
use cyrup_candle::domain::model::CandleModel;
use cyrup_candle::StreamExt;

// use fluent_ai::{FluentAi, Providers, Models}; // Temporarily disabled due to dependency issues
use super::model::*;
// use crate::auth::JwtAuth; // Auth module not available

/// Build prompt from request with content type validation
///
/// Validates message content types per MCP spec (text, image, audio).
/// Logs warnings for unsupported types (image, audio) and returns error if no text content found.
fn build_prompt_from_request(request: &CreateMessageRequest) -> Result<String, String> {
    let mut full_prompt = String::new();

    // Add system prompt if present
    if let Some(system_prompt) = &request.system_prompt {
        full_prompt.push_str(system_prompt);
        full_prompt.push_str("\n\n");
    }

    // Track successful text message count
    let mut text_message_count: usize = 0;

    // Process messages with content type validation
    for msg in &request.messages {
        match &msg.content {
            McpMessageContent { type_, text: Some(text), .. } if type_ == "text" => {
                full_prompt.push_str(&format!("{}: {}\n", msg.role, text));
                text_message_count += 1;
            }
            McpMessageContent { type_, .. } if type_ == "image" => {
                log::warn!(
                    "Skipping message with role '{}' - image content not supported by model",
                    msg.role
                );
            }
            McpMessageContent { type_, .. } if type_ == "audio" => {
                log::warn!(
                    "Skipping message with role '{}' - audio content not supported by model",
                    msg.role
                );
            }
            _ => {
                log::warn!(
                    "Skipping message with role '{}' - missing or invalid text content",
                    msg.role
                );
            }
        }
    }

    // Validate at least one text message was processed
    if text_message_count == 0 {
        return Err("Request must contain at least one text message".to_string());
    }

    Ok(full_prompt)
}

/// Handler for the sampling/createMessage method (returns AsyncSamplingResult).
pub fn sampling_create_message_pending(request: CreateMessageRequest) -> AsyncSamplingResult {
    let (tx_result, rx_result) = oneshot::channel();

    tokio::spawn(async move {
        log::info!("Received sampling/createMessage request: {:?}", request);

        // Validate messages exist
        if request.messages.is_empty() {
            let _ = tx_result.send(Err(rpc_router::HandlerError::new("No messages provided")));
            return;
        }

        // Get model from registry (uses worker pool for efficiency)
        let provider = match registry::get::<TextToTextModel>("unsloth/Kimi-K2-Instruct-GGUF") {
            Some(p) => p,
            None => {
                error!("Failed to get KimiK2 model from registry");
                let _ = tx_result.send(Err(rpc_router::HandlerError::new(
                    "Model not available in registry",
                )));
                return;
            }
        };

        // Build and validate prompt from request
        let full_prompt = match build_prompt_from_request(&request) {
            Ok(prompt) => prompt,
            Err(e) => {
                error!("{}", e);
                let _ = tx_result.send(Err(rpc_router::HandlerError::new(e)));
                return;
            }
        };

        let prompt = CandlePrompt::new(full_prompt);

        // Configure completion parameters from request
        let temperature = request.temperature.unwrap_or(0.7) as f64;
        let max_tokens = request.max_tokens.unwrap_or(2048);
        let mut params = CandleCompletionParams::default();
        params.temperature = temperature;
        params.max_tokens = NonZeroU64::new(max_tokens as u64);

        // Perform inference with streaming collection
        let completion_stream = provider.prompt(prompt, &params);
        let completion_results: Vec<CandleCompletionChunk> = completion_stream.collect().await;

        // Accumulate response text and extract usage
        let mut response_text = String::new();
        let mut actual_usage: Option<CompletionUsage> = None;
        let mut stop_reason = "endTurn".to_string();

        for chunk in completion_results {
            match chunk {
                CandleCompletionChunk::Text(text) => {
                    response_text.push_str(&text);
                }
                CandleCompletionChunk::Complete { text, finish_reason, usage, .. } => {
                    response_text.push_str(&text);
                    if let Some(reason) = finish_reason {
                        stop_reason = format!("{:?}", reason);
                    }
                    if let Some(u) = usage {
                        actual_usage = Some(CompletionUsage {
                            prompt_tokens: u.input_tokens,
                            completion_tokens: u.output_tokens,
                            total_tokens: u.total_tokens,
                        });
                    }
                }
                CandleCompletionChunk::Error(error) => {
                    error!("Inference error: {}", error);
                    let _ = tx_result.send(Err(rpc_router::HandlerError::new(
                        format!("Inference failed: {}", error),
                    )));
                    return;
                }
                _ => {} // Ignore other chunk types
            }
        }

        // Get model name from provider
        let model_name = provider.name().to_string();

        // Create the result
        let result = CreateMessageResult {
            role: "assistant".to_string(),
            content: McpMessageContent {
                type_: "text".to_string(),
                text: Some(response_text),
                data: None,
                mime_type: None,
            },
            model: model_name,
            stop_reason: Some(stop_reason),
            usage: actual_usage,
        };

        log::info!("Returning sampling result: {:?}", result);

        // Send CompletionUsage to result channel
        let usage = match result.usage {
            Some(usage) => usage,
            None => {
                error!("Sampling result missing usage data");
                let _ = tx_result.send(Err(rpc_router::HandlerError::new(
                    "Internal error: Missing usage data",
                )));
                return;
            }
        };
        let _ = tx_result.send(Ok(usage));
    });

    // Return AsyncSamplingResult which expects Result<CompletionUsage, ...>
    AsyncSamplingResult { rx: rx_result }
}

pub fn sampling_create_message(request: CreateMessageRequest) -> AsyncSamplingResult {
    sampling_create_message_pending(request)
}

/// Create a streaming sampling result with real-time token generation
pub fn sampling_create_message_stream(request: CreateMessageRequest) -> SamplingStream {
    let (tx_stream, rx_stream) = mpsc::channel::<HandlerResult<CreateMessageResult>>(16);

    tokio::spawn(async move {
        // Get model from registry (uses worker pool for efficiency)
        let provider = match registry::get::<TextToTextModel>("unsloth/Kimi-K2-Instruct-GGUF") {
            Some(p) => p,
            None => {
                error!("Failed to get KimiK2 model from registry for streaming");
                let _ = tx_stream.send(Err(rpc_router::HandlerError::new(
                    "Model not available in registry",
                ))).await;
                return;
            }
        };

        // Build and validate prompt from request
        let full_prompt = match build_prompt_from_request(&request) {
            Ok(prompt) => prompt,
            Err(e) => {
                error!("{}", e);
                let _ = tx_stream.send(Err(rpc_router::HandlerError::new(e))).await;
                return;
            }
        };

        let prompt = CandlePrompt::new(full_prompt);

        // Configure completion parameters
        let temperature = request.temperature.unwrap_or(0.7) as f64;
        let max_tokens = request.max_tokens.unwrap_or(2048);
        let mut params = CandleCompletionParams::default();
        params.temperature = temperature;
        params.max_tokens = NonZeroU64::new(max_tokens as u64);

        // Get model name from provider
        let model_name = provider.name().to_string();

        // Stream tokens as they're generated
        let mut completion_stream = provider.prompt(prompt, &params);
        let mut accumulated_text = String::new();

        while let Some(chunk) = completion_stream.next().await {
            match chunk {
                CandleCompletionChunk::Text(text) => {
                    accumulated_text.push_str(&text);
                    // Send incremental result
                    let partial_result = CreateMessageResult {
                        role: "assistant".to_string(),
                        content: McpMessageContent {
                            type_: "text".to_string(),
                            text: Some(accumulated_text.clone()),
                            data: None,
                            mime_type: None,
                        },
                        model: model_name.clone(),
                        stop_reason: None, // Still streaming
                        usage: None, // Usage only available at end
                    };
                    if tx_stream.send(Ok(partial_result)).await.is_err() {
                        break; // Receiver closed
                    }
                }
                CandleCompletionChunk::Complete { text, finish_reason, usage, .. } => {
                    accumulated_text.push_str(&text);
                    // Send final result
                    let final_result = CreateMessageResult {
                        role: "assistant".to_string(),
                        content: McpMessageContent {
                            type_: "text".to_string(),
                            text: Some(accumulated_text),
                            data: None,
                            mime_type: None,
                        },
                        model: model_name,
                        stop_reason: finish_reason.map(|r| format!("{:?}", r)),
                        usage: usage.map(|u| CompletionUsage {
                            prompt_tokens: u.input_tokens,
                            completion_tokens: u.output_tokens,
                            total_tokens: u.total_tokens,
                        }),
                    };
                    let _ = tx_stream.send(Ok(final_result)).await;
                    break;
                }
                CandleCompletionChunk::Error(error) => {
                    error!("Streaming inference error: {}", error);
                    let _ = tx_stream.send(Err(rpc_router::HandlerError::new(
                        format!("Streaming inference failed: {}", error),
                    ))).await;
                    break;
                }
                _ => {} // Ignore other chunk types
            }
        }
    });

    SamplingStream::new(rx_stream)
}
