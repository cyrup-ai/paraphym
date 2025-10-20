//! Generic wrapper chunk types for common operations
//!
//! This module provides generic wrapper types that implement MessageChunk
//! for common data types including:
//! - Unit operations (success/error)
//! - String values
//! - JSON values
//! - Collections

use cyrup_sugars::prelude::MessageChunk;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A simple unit chunk type that implements `MessageChunk` for operations that don't return data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum CandleUnitChunk {
    /// Operation completed successfully
    #[default]
    Success,
    /// Operation failed with error
    Error(String),
}

impl MessageChunk for CandleUnitChunk {
    fn bad_chunk(error: String) -> Self {
        CandleUnitChunk::Error(error)
    }

    fn error(&self) -> Option<&str> {
        match self {
            CandleUnitChunk::Error(err) => Some(err),
            CandleUnitChunk::Success => None,
        }
    }
}

/// Simple wrapper for String to implement `MessageChunk`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CandleStringChunk(pub String);

impl MessageChunk for CandleStringChunk {
    fn bad_chunk(error: String) -> Self {
        CandleStringChunk(format!("Error: {error}"))
    }

    fn error(&self) -> Option<&str> {
        if self.0.starts_with("Error: ") {
            Some(&self.0)
        } else {
            None
        }
    }
}

/// Wrapper for JSON Value to implement `MessageChunk`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CandleJsonChunk(pub Value);

impl MessageChunk for CandleJsonChunk {
    fn bad_chunk(error: String) -> Self {
        CandleJsonChunk(Value::String(format!("Error: {error}")))
    }

    fn error(&self) -> Option<&str> {
        match &self.0 {
            Value::String(s) if s.starts_with("Error: ") => Some(s),
            _ => None,
        }
    }
}

/// Generic wrapper for collections to implement `MessageChunk`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleCollectionChunk<T> {
    pub items: T,
    pub error_message: Option<String>,
}

impl<T> Default for CandleCollectionChunk<T>
where
    T: Default,
{
    fn default() -> Self {
        CandleCollectionChunk {
            items: T::default(),
            error_message: None,
        }
    }
}

impl<T> MessageChunk for CandleCollectionChunk<T>
where
    T: Default,
{
    fn bad_chunk(error: String) -> Self {
        CandleCollectionChunk {
            items: T::default(),
            error_message: Some(error),
        }
    }

    fn error(&self) -> Option<&str> {
        self.error_message.as_deref()
    }
}

/// Chunk of embedding data for streaming embeddings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingChunk {
    /// The embedding vector
    pub embeddings: cyrup_sugars::ZeroOneOrMany<f32>,

    /// Index in the batch
    pub index: usize,

    /// Additional metadata
    #[serde(flatten)]
    pub metadata: std::collections::HashMap<String, Value>,
}

impl Default for EmbeddingChunk {
    fn default() -> Self {
        Self {
            embeddings: cyrup_sugars::ZeroOneOrMany::None,
            index: 0,
            metadata: std::collections::HashMap::new(),
        }
    }
}

impl MessageChunk for EmbeddingChunk {
    fn bad_chunk(error: String) -> Self {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("error".to_string(), Value::String(error));
        Self {
            embeddings: cyrup_sugars::ZeroOneOrMany::None,
            index: 0,
            metadata,
        }
    }

    fn error(&self) -> Option<&str> {
        if let Some(Value::String(error)) = self.metadata.get("error") {
            Some(error)
        } else {
            None
        }
    }
}

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
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
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
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            ),
            error_message: None,
        }
    }
}
