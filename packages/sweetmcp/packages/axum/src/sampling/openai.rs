//! OpenAI-compatible chat completions endpoint
//! 
//! Provides OpenAI API compatibility by bridging OpenAI format requests to the MCP backend.
//! Enables integration with OpenAI SDK, LangChain, and other OpenAI-compatible tools.
//!
//! Architecture:
//! - Accepts OpenAI ChatRequest format
//! - Translates to MCP CreateMessageRequest
//! - Streams results from local inference backend
//! - Translates back to OpenAI ChatResponse format

use super::chat::{ChatRequest, ChatResponse, translate_openai_to_mcp, translate_mcp_to_openai};
use super::model::CreateMessageRequest;
use super::service::sampling_create_message_stream;
use futures::StreamExt;
use log::{error, info};

/// Handle OpenAI-compatible chat completion request
/// 
/// This is the main entry point for OpenAI API compatibility. It performs a complete
/// translation pipeline from OpenAI format through MCP backend to OpenAI response.
///
/// # Pipeline Steps
///
/// 1. **Translation**: Convert OpenAI ChatRequest → MCP intermediate format
/// 2. **Mapping**: Build MCP CreateMessageRequest with required fields
/// 3. **Inference**: Call streaming MCP backend (required to get text content)
/// 4. **Collection**: Consume stream to get final result with complete text
/// 5. **Serialization**: Convert CreateMessageResult → serde_json::Value
/// 6. **Translation**: Convert MCP response → OpenAI ChatResponse
///
/// # Why Streaming Collection?
///
/// The non-streaming `sampling_create_message()` only returns CompletionUsage (token counts),
/// not the actual generated text. The streaming variant accumulates text as tokens are
/// generated and returns the complete result with both text and usage statistics.
///
/// # Arguments
///
/// * `request` - OpenAI-format chat completion request
///
/// # Returns
///
/// * `Ok(ChatResponse)` - OpenAI-format response with generated text
/// * `Err(String)` - Error message (router converts to HTTP 500)
///
/// # Example
///
/// ```rust
/// let request = ChatRequest {
///     messages: vec![ChatMessage {
///         role: Role::User,
///         content: Some("Hello!".to_string()),
///         ..Default::default()
///     }],
///     temperature: Some(0.7),
///     ..Default::default()
/// };
/// 
/// let response = openai_chat_completions(request).await?;
/// assert_eq!(response.choices[0].message.role, Role::Assistant);
/// ```
pub async fn openai_chat_completions(request: ChatRequest) -> Result<ChatResponse, String> {
    info!("OpenAI chat completions request: model={:?}, messages={}", 
          request.model, request.messages.len());

    // Step 1: Translate OpenAI format → MCP intermediate format
    // Uses translate_openai_to_mcp from chat.rs:197
    // Converts ChatMessage → McpMessage, Role enum → string, etc.
    let mcp_params = translate_openai_to_mcp(&request);
    
    // Step 2: Build MCP CreateMessageRequest from intermediate format
    // McpSamplingParams is missing 'metadata' and 'meta' fields that
    // CreateMessageRequest requires, so we map manually and set to None
    let mcp_request = CreateMessageRequest {
        messages: mcp_params.messages,
        system_prompt: mcp_params.system_prompt,
        model_preferences: mcp_params.model_preferences,
        include_context: mcp_params.include_context,
        max_tokens: mcp_params.max_tokens,
        temperature: mcp_params.temperature,
        stop_sequences: mcp_params.stop_sequences,
        metadata: None,  // Not used for OpenAI compatibility
        meta: None,      // Progress tracking not needed
    };
    
    // Step 3: Call MCP backend using STREAMING variant
    // CRITICAL: Must use streaming variant because non-streaming only returns
    // CompletionUsage (token counts), NOT the generated text content!
    // See service.rs:191 for implementation details
    let mut stream = sampling_create_message_stream(mcp_request);
    
    // Step 4: Collect stream to get final result with text content
    // The stream yields intermediate results as tokens are generated.
    // Each result has accumulated_text so far. The final result has:
    // - Complete text in content.text field
    // - stop_reason set (e.g., "StopReason")
    // - usage statistics (token counts)
    //
    // We keep overwriting final_result with each chunk, so we end up
    // with the last (complete) result.
    let mut final_result = None;
    while let Some(result) = stream.next().await {
        match result {
            Ok(create_result) => {
                // Keep overwriting until we get the final complete result
                final_result = Some(create_result);
            }
            Err(e) => {
                error!("MCP backend error: {:?}", e);
                return Err(format!("Inference failed: {:?}", e));
            }
        }
    }
    
    // Ensure we got at least one result from the stream
    let mcp_result = final_result
        .ok_or_else(|| "No response from MCP backend".to_string())?;
    
    // Step 5: Serialize MCP result to Value
    // translate_mcp_to_openai() expects &Value, not &CreateMessageResult
    // This is because the translation function was designed to work with
    // JSON-RPC responses which are already in Value format
    let mcp_value = serde_json::to_value(&mcp_result)
        .map_err(|e| format!("Failed to serialize MCP result: {}", e))?;
    
    // Step 6: Translate MCP format → OpenAI format
    // Uses translate_mcp_to_openai from chat.rs:248
    // Converts CreateMessageResult → ChatResponse, generates ID, etc.
    let openai_response = translate_mcp_to_openai(&mcp_value)
        .map_err(|e| format!("Failed to translate to OpenAI format: {}", e))?;
    
    info!("OpenAI response generated: id={}", openai_response.id);
    Ok(openai_response)
}
