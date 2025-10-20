//! Completion Streaming Chunk Types
//!
//! Chunk types for LLM completion streaming including text generation,
//! tool calls, and chat messages.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use cyrup_sugars::prelude::MessageChunk;
use crate::domain::model::CandleUsage;
use crate::domain::chat::message::types::CandleMessageRole;

/// Chunk of chat message for streaming responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageChunk {
    /// Partial message content
    pub content: String,

    /// Role of the message sender
    pub role: CandleMessageRole,

    /// Whether this is the final chunk
    pub is_final: bool,

    /// Additional metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

/// Reason why a completion finished
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FinishReason {
    /// Completion finished naturally at a stopping point
    Stop,
    /// Completion reached maximum token length limit
    Length,
    /// Completion was filtered due to content policy
    ContentFilter,
    /// Completion finished to execute tool calls
    ToolCalls,
    /// Completion failed due to an error
    Error,
}

/// Comprehensive completion chunk supporting all streaming features - EXACT REPLICA of domain
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CandleCompletionChunk {
    /// Text content chunk
    Text(String),

    /// Tool call started
    ToolCallStart { id: String, name: String },

    /// Partial tool call with streaming input
    ToolCall {
        id: String,
        name: String,
        partial_input: String,
    },

    /// Tool call completed
    ToolCallComplete {
        id: String,
        name: String,
        input: String,
    },

    /// Completion finished with final information
    Complete {
        text: String,
        finish_reason: Option<FinishReason>,
        usage: Option<CandleUsage>,
    },

    /// Error occurred during streaming
    Error(String),
}

impl Default for CandleCompletionChunk {
    fn default() -> Self {
        CandleCompletionChunk::Text(String::new())
    }
}

impl MessageChunk for CandleCompletionChunk {
    fn bad_chunk(error: String) -> Self {
        CandleCompletionChunk::Error(error)
    }

    fn error(&self) -> Option<&str> {
        match self {
            CandleCompletionChunk::Error(err) => Some(err),
            _ => None,
        }
    }
}
