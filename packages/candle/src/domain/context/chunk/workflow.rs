//! Workflow Data Chunk Types
//!
//! Chunk types for workflow-specific data streaming.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use cyrup_sugars::prelude::MessageChunk;
use std::time::{SystemTime, UNIX_EPOCH};

/// Workflow data chunk for streaming JSON values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDataChunk {
    /// The JSON data
    pub data: Value,
    /// Step name that produced this data
    pub step_name: Option<String>,
    /// Processing timestamp
    pub timestamp: Option<u64>,
    /// Error message if this represents an error
    pub error_message: Option<String>,
}

impl MessageChunk for WorkflowDataChunk {
    fn bad_chunk(error: String) -> Self {
        Self {
            data: Value::String(format!("Error: {error}")),
            step_name: Some("error".to_string()),
            timestamp: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            ),
            error_message: Some(error),
        }
    }

    fn error(&self) -> Option<&str> {
        self.error_message.as_deref()
    }
}

impl Default for WorkflowDataChunk {
    fn default() -> Self {
        Self {
            data: Value::Null,
            step_name: None,
            timestamp: None,
            error_message: None,
        }
    }
}

impl From<Value> for WorkflowDataChunk {
    fn from(data: Value) -> Self {
        Self {
            data,
            step_name: None,
            timestamp: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            ),
            error_message: None,
        }
    }
}
