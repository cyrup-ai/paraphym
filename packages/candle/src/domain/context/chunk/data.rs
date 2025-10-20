//! Document and Embedding Chunk Types
//!
//! Chunk types for document content and vector embeddings.

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use cyrup_sugars::{ZeroOneOrMany, prelude::MessageChunk};

/// Candle chunk of document content for streaming file operations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CandleDocumentChunk {
    /// Optional path to the source file
    pub path: Option<PathBuf>,

    /// The content of this chunk
    pub content: String,

    /// Byte range in the original file
    pub byte_range: Option<(usize, usize)>,

    /// Additional metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

// Convenience constructors
impl CandleDocumentChunk {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            path: None,
            content: content.into(),
            byte_range: None,
            metadata: HashMap::new(),
        }
    }

    #[must_use]
    pub fn with_range(mut self, start: usize, end: usize) -> Self {
        self.byte_range = Some((start, end));
        self
    }
}

impl MessageChunk for CandleDocumentChunk {
    fn bad_chunk(error: String) -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("error".to_string(), Value::String(error));
        Self {
            path: None,
            content: String::new(),
            byte_range: None,
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

/// Chunk of embedding data for streaming embeddings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingChunk {
    /// The embedding vector
    pub embeddings: ZeroOneOrMany<f32>,

    /// Index in the batch
    pub index: usize,

    /// Additional metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

impl Default for EmbeddingChunk {
    fn default() -> Self {
        Self {
            embeddings: ZeroOneOrMany::None,
            index: 0,
            metadata: HashMap::new(),
        }
    }
}

impl MessageChunk for EmbeddingChunk {
    fn bad_chunk(error: String) -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("error".to_string(), Value::String(error));
        Self {
            embeddings: ZeroOneOrMany::None,
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
