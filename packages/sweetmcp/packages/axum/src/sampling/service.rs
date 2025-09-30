// use futures_util::StreamExt; // Temporarily unused
use std::num::NonZeroU64;

use log::{self, error};
use rpc_router::HandlerResult;
use tokio::sync::{mpsc, oneshot};

// Candle inference imports
use paraphym_candle::providers::kimi_k2::CandleKimiK2Provider;
use paraphym_candle::domain::completion::{
    CandleCompletionModel, CandlePrompt, CandleCompletionParams, CandleCompletionChunk,
};

// use fluent_ai::{FluentAi, Providers, Models}; // Temporarily disabled due to dependency issues
use super::model::*;
// use crate::auth::JwtAuth; // Auth module not available
use crate::sampling::notifications::SamplingProgressNotification;

/// Handler for the sampling/createMessage method (returns AsyncSamplingResult).
pub fn sampling_create_message_pending(request: CreateMessageRequest) -> AsyncSamplingResult {
    let (tx_result, rx_result) = oneshot::channel();
    // Channel for streaming results (if needed in the future)
    let (_tx_stream, rx_stream) = mpsc::channel::<HandlerResult<CreateMessageResult>>(16);

    tokio::spawn(async move {
        log::info!("Received sampling/createMessage request: {:?}", request);

        // Stub implementation: Replace with real LLM calls via MCP client requests.

        // Extract the last user message for demonstration
        let last_message = request
            .messages
            .last()
            .ok_or_else(|| rpc_router::HandlerError::new("No messages provided"));

        let result = match last_message {
            Ok(last_message) => {
                // Get the text from the last message (if it's a text message)
                let prompt_text = match &last_message.content {
                    McpMessageContent { type_, text, .. } if type_ == "text" && text.is_some() => {
                        text.as_ref().unwrap()
                    }
                    _ => {
                        return {
                            let _ = tx_result.send(Err(rpc_router::HandlerError::new(
                                "Last message must be text",
                            )));
                            ()
                        };
                    }
                };

                // Report initial progress if request has meta params
                if let Some(meta) = &request.meta {
                    // Create a progress channel
                    let (tx_progress, _rx_progress) =
                        mpsc::channel::<HandlerResult<SamplingProgressNotification>>(16);
                    report_sampling_progress(&tx_progress, meta.progress_token.clone(), 0, 150);
                }

                // Initialize CandleKimiK2Provider for local inference
                let provider = match CandleKimiK2Provider::default_for_builder() {
                    Ok(p) => p,
                    Err(e) => {
                        error!("Failed to initialize CandleKimiK2Provider: {}", e);
                        return {
                            let _ = tx_result.send(Err(rpc_router::HandlerError::new(
                                "Failed to initialize local model provider",
                            )));
                            ()
                        };
                    }
                };

                // Build prompt from system prompt + messages
                let mut full_prompt = String::new();
                if let Some(system_prompt) = &request.system_prompt {
                    full_prompt.push_str(system_prompt);
                    full_prompt.push_str("\n\n");
                }

                // Add all messages to context
                for msg in &request.messages {
                    if let McpMessageContent { text: Some(text), .. } = &msg.content {
                        full_prompt.push_str(&format!("{}: {}\n", msg.role, text));
                    }
                }

                let prompt = CandlePrompt::new(full_prompt);

                // Configure completion parameters from request
                let temperature = request.temperature.unwrap_or(0.7);
                let max_tokens = request.max_tokens.unwrap_or(2048);
                let params = CandleCompletionParams {
                    temperature,
                    max_tokens: NonZeroU64::new(max_tokens as u64),
                    ..Default::default()
                };

                // Perform inference with streaming collection
                let completion_stream = provider.prompt(prompt, &params);
                let completion_results: Vec<CandleCompletionChunk> = completion_stream.collect();

                // Accumulate response text and extract usage
                let mut response_text = String::new();
                let mut actual_usage: Option<CompletionUsage> = None;
                let mut stop_reason = "endTurn".to_string();

                for chunk in completion_results {
                    match chunk {
                        CandleCompletionChunk::Text(text) => {
                            response_text.push_str(&text);
                        }
                        CandleCompletionChunk::Complete { text, finish_reason, usage } => {
                            if let Some(text) = text {
                                response_text.push_str(&text);
                            }
                            if let Some(reason) = finish_reason {
                                stop_reason = reason;
                            }
                            if let Some(u) = usage {
                                actual_usage = Some(CompletionUsage {
                                    prompt_tokens: u.prompt_tokens,
                                    completion_tokens: u.completion_tokens,
                                    total_tokens: u.total_tokens,
                                });
                            }
                        }
                        CandleCompletionChunk::Error(error) => {
                            error!("Inference error: {}", error);
                            return {
                                let _ = tx_result.send(Err(rpc_router::HandlerError::new(
                                    format!("Inference failed: {}", error),
                                )));
                                ()
                            };
                        }
                        _ => {} // Ignore other chunk types
                    }
                }

                let model_name = "kimi-k2-instruct-q4_0".to_string();

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
                Ok(result)
            }
            Err(e) => Err(e),
        };

        match result {
            Ok(value) => {
                // Assuming `value` here is the CreateMessageResult
                // We need to send CompletionUsage
                let usage = match value.usage.clone() {
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
            }
            Err(e) => {
                error!("Sampling message creation failed: {}", e);
                // Ensure error type matches receiver expectation if needed
                let _ = tx_result.send(Err(e)); // Send the original HandlerError
            }
        }
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
        // Initialize CandleKimiK2Provider
        let provider = match CandleKimiK2Provider::default_for_builder() {
            Ok(p) => p,
            Err(e) => {
                error!("Failed to initialize CandleKimiK2Provider for streaming: {}", e);
                let _ = tx_stream.send(Err(rpc_router::HandlerError::new(
                    "Failed to initialize local model provider",
                ))).await;
                return;
            }
        };

        // Build prompt from system prompt + messages
        let mut full_prompt = String::new();
        if let Some(system_prompt) = &request.system_prompt {
            full_prompt.push_str(system_prompt);
            full_prompt.push_str("\n\n");
        }

        for msg in &request.messages {
            if let McpMessageContent { text: Some(text), .. } = &msg.content {
                full_prompt.push_str(&format!("{}: {}\n", msg.role, text));
            }
        }

        let prompt = CandlePrompt::new(full_prompt);

        // Configure completion parameters
        let temperature = request.temperature.unwrap_or(0.7);
        let max_tokens = request.max_tokens.unwrap_or(2048);
        let params = CandleCompletionParams {
            temperature,
            max_tokens: NonZeroU64::new(max_tokens as u64),
            ..Default::default()
        };

        // Stream tokens as they're generated
        let completion_stream = provider.prompt(prompt, &params);
        let mut accumulated_text = String::new();

        for chunk in completion_stream {
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
                        model: "kimi-k2-instruct-q4_0".to_string(),
                        stop_reason: None, // Still streaming
                        usage: None, // Usage only available at end
                    };
                    if tx_stream.send(Ok(partial_result)).await.is_err() {
                        break; // Receiver closed
                    }
                }
                CandleCompletionChunk::Complete { text, finish_reason, usage } => {
                    if let Some(text) = text {
                        accumulated_text.push_str(&text);
                    }
                    // Send final result
                    let final_result = CreateMessageResult {
                        role: "assistant".to_string(),
                        content: McpMessageContent {
                            type_: "text".to_string(),
                            text: Some(accumulated_text),
                            data: None,
                            mime_type: None,
                        },
                        model: "kimi-k2-instruct-q4_0".to_string(),
                        stop_reason: finish_reason,
                        usage: usage.map(|u| CompletionUsage {
                            prompt_tokens: u.prompt_tokens,
                            completion_tokens: u.completion_tokens,
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

// Restore unused function - signature updated
fn report_sampling_progress(
    tx_progress: &mpsc::Sender<HandlerResult<SamplingProgressNotification>>,
    request_id: String, // Added request_id
    tokens: u32,        // Renamed progress to tokens for clarity?
    total_tokens: u32,  // Renamed total to total_tokens for clarity?
) {
    // Correctly initialize SamplingProgressNotification
    let progress_notification = SamplingProgressNotification {
        request_id,
        progress: tokens,    // Map tokens to progress field
        total: total_tokens, // Map total_tokens to total field
        message: None,       // No message for now
    };
    // Removed incorrect SamplingProgressData usage
    // let progress_notification = SamplingProgressNotification {
    // progress: SamplingProgressData { // Now resolved
    // tokens,
    // total_tokens,
    // estimated_completion_time: None, // Not implemented
    // },
    // };

    // Try to send, but ignore error if receiver is closed
    let _ = tx_progress.try_send(Ok(progress_notification));
}
