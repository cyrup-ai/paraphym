use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// --- Request Structures (Mirroring OpenAI) ---

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatRequest {
    // Messages for the *current* turn.
    // For the first turn, this is the initial prompt (user message, maybe system).
    // For subsequent turns (e.g., after a tool call), this would contain
    // just the new message(s), like the tool result message.
    pub messages: Vec<ChatMessage>,

    // Optional token to continue a previous conversation sequence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,

    // Parameters (can be resent on each turn if needed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>, // Hint for MCP client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>, // Note: MCP likely only supports n=1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopSequence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>, // Note: Streaming not directly supported via this proxy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub role: Role,
    pub content: Option<String>, // Simplified: OpenAI allows array for multimodal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>, // Function/Tool name for role='tool'
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "tool")]
    Tool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String, // Should typically be "function"
    pub function: FunctionCall,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String, // JSON string arguments
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub type_: String, // e.g., "text", "json_object"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum StopSequence {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool {
    #[serde(rename = "type")]
    pub type_: String, // Only "function" is supported typically
    pub function: FunctionDefinition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>, // JSON Schema object
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ToolChoice {
    String(String), // "none", "auto", or {"type": "function", "function": {"name": "my_func"}}
    Object(ToolChoiceObject),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolChoiceObject {
    #[serde(rename = "type")]
    pub type_: String, // Typically "function"
    pub function: ToolChoiceFunction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolChoiceFunction {
    pub name: String,
}

// --- Response Structures (Mirroring OpenAI) ---

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatResponse {
    pub id: String, // Unique ID for *this specific response*
    pub choices: Vec<ChatChoice>,
    pub created: u64,  // Unix timestamp for *this response*
    pub model: String, // Model used by the client

    // The session token to use for the *next* request in this conversation.
    // This will be generated by the host on the first turn, and potentially
    // refreshed/re-issued on subsequent turns.
    pub session_token: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    pub object: String, // Typically "chat.completion"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<super::model::CompletionUsage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatChoice {
    pub finish_reason: Option<String>, // e.g., "stop", "length", "tool_calls"
    pub index: u32,
    pub message: ChatMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Value>, // Complex object if requested
}

// --- Internal Helper for MCP Translation ---

// This structure can be used within handle_chat_proxy to build
// the params for the MCP sampling/createMessage request.
#[derive(Serialize, Debug)]
pub struct McpSamplingParams {
    pub messages: Vec<super::model::McpMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_preferences: Option<super::model::McpModelPreferences>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_context: Option<String>, // "none", "thisServer", "allServers"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    // Add other MCP sampling params as needed
}

// Helper function to translate OpenAI format to MCP format
pub(super) fn translate_openai_to_mcp(request: &ChatRequest) -> McpSamplingParams {
    let messages = request
        .messages
        .iter()
        .map(|msg| {
            let role = match msg.role {
                Role::System => "system",
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::Tool => "tool",
            }
            .to_string();

            super::model::McpMessage {
                role,
                content: super::model::McpMessageContent {
                    type_: "text".to_string(),
                    text: msg.content.clone(),
                    data: None,
                    mime_type: None,
                },
            }
        })
        .collect();

    McpSamplingParams {
        messages,
        model_preferences: request
            .model
            .as_ref()
            .map(|model| super::model::McpModelPreferences {
                hints: Some(vec![super::model::McpModelHint {
                    name: model.clone(),
                }]),
                cost_priority: None,
                speed_priority: None,
                intelligence_priority: None,
            }),
        system_prompt: None,
        include_context: Some("thisServer".to_string()),
        max_tokens: request.max_tokens,
        temperature: request.temperature,
        stop_sequences: request.stop.as_ref().map(|s| match s {
            StopSequence::Single(seq) => vec![seq.clone()],
            StopSequence::Multiple(seqs) => seqs.clone(),
        }),
    }
}

/// Translates MCP CreateMessageResult to OpenAI ChatResponse format
pub(super) fn translate_mcp_to_openai(mcp_response: &Value) -> Result<ChatResponse, anyhow::Error> {
    // 1. Deserialize MCP response to typed struct
    let mcp_result: super::model::CreateMessageResult = serde_json::from_value(mcp_response.clone())
        .context("Failed to deserialize MCP response")?;
    
    // 2. Extract text content from single content object
    let text_content = mcp_result.content.text
        .unwrap_or_else(|| String::from(""));
    
    if text_content.is_empty() {
        return Err(anyhow::anyhow!("MCP response contains no text content"));
    }
    
    // 3. Create ChatMessage for the choice
    let message = ChatMessage {
        role: Role::Assistant,
        content: Some(text_content.clone()),
        tool_calls: None,
        tool_call_id: None,
        name: None,
    };
    
    // 4. Map MCP stop_reason to OpenAI finish_reason
    let finish_reason = match mcp_result.stop_reason.as_deref() {
        Some("endTurn") => "stop",
        Some("maxTokens") => "length",
        Some("stopSequence") => "stop",
        _ => "stop",
    }.to_string();
    
    // 5. Create ChatChoice
    let choice = ChatChoice {
        index: 0,
        message,
        finish_reason: Some(finish_reason),
        logprobs: None,
    };
    
    // 6. Generate deterministic ID using content hash
    let mut hasher = DefaultHasher::new();
    text_content.hash(&mut hasher);
    let content_hash = format!("{:016x}", hasher.finish());
    let chat_id = format!("chatcmpl-{}", content_hash);
    
    // 7. Generate session token for conversation continuity
    let session_token = format!("session-{}", uuid::Uuid::new_v4());
    
    // 8. Build final OpenAI-compatible response
    Ok(ChatResponse {
        id: chat_id,
        object: "chat.completion".to_string(),
        created: chrono::Utc::now().timestamp() as u64,
        model: mcp_result.model,
        choices: vec![choice],
        usage: mcp_result.usage, // MCP usage is already in OpenAI-compatible format
        system_fingerprint: Some(format!("mcp-{}", env!("CARGO_PKG_VERSION"))),
        session_token,
    })
}
