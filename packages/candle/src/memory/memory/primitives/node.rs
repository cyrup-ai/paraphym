// src/memory/primitives/node.rs
//! Memory node implementation for the memory system.
//! This module defines the core data structures for memory nodes.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use cyrup_sugars::prelude::*;

use super::metadata::MemoryMetadata;
use super::types::{MemoryTypeEnum, MemoryContent};

/// A memory node in the memory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryNode {
    /// Unique identifier for the memory
    pub id: String,
    /// Content of the memory
    pub content: MemoryContent,
    /// Type of memory
    pub memory_type: MemoryTypeEnum,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Embedding vector
    pub embedding: Option<Vec<f32>>,
    /// Metadata associated with the memory
    pub metadata: MemoryMetadata,
}

impl MemoryNode {
    /// Create a new memory node
    pub fn new(memory_type: MemoryTypeEnum, content: MemoryContent) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        Self {
            id,
            content,
            memory_type,
            created_at: now,
            updated_at: now,
            embedding: None,
            metadata: MemoryMetadata::with_memory_type(memory_type),
        }
    }

    /// Create a new memory node with a specific ID
    pub fn with_id(id: String, memory_type: MemoryTypeEnum, content: MemoryContent) -> Self {
        let now = Utc::now();

        Self {
            id,
            content,
            memory_type,
            created_at: now,
            updated_at: now,
            embedding: None,
            metadata: MemoryMetadata::with_memory_type(memory_type),
        }
    }

    /// Set the embedding for this memory
    pub fn with_embedding(mut self, embedding: Vec<f32>) -> Self {
        self.embedding = Some(embedding.clone());
        self
    }

    /// Set the importance for this memory
    pub fn with_importance(mut self, importance: f32) -> Self {
        self.metadata.importance = importance;
        self
    }

    /// Add custom metadata to this memory
    pub fn with_custom_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        if let Err(_) = self.metadata.set_custom(&key, value.clone()) {
            // If setting custom fails, create a new object and insert
            if let serde_json::Value::Object(ref mut map) = self.metadata.custom {
                map.insert(key, value);
            } else {
                let mut map = serde_json::Map::new();
                map.insert(key, value);
                self.metadata.custom = serde_json::Value::Object(map);
            }
        }
        self
    }

    /// Update the last accessed time
    pub fn update_last_accessed(&mut self) {
        self.metadata.last_accessed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Get the importance value from metadata
    pub fn importance(&self) -> f32 {
        self.metadata.importance
    }

    /// Get the last accessed timestamp
    pub fn last_accessed(&self) -> Option<DateTime<Utc>> {
        self.metadata.last_accessed_at
    }

    /// Get base memory representation - returns a reference to self for now
    pub fn base_memory(&self) -> &Self {
        self
    }
}

impl Default for MemoryNode {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            content: MemoryContent::default(),
            memory_type: MemoryTypeEnum::default(),
            created_at: now,
            updated_at: now,
            embedding: None,
            metadata: MemoryMetadata::default(),
        }
    }
}

impl MessageChunk for MemoryNode {
    fn bad_chunk(error: String) -> Self {
        let mut node = Self::default();
        node.content = MemoryContent::new(&format!("ERROR: {}", error));
        node
    }

    fn error(&self) -> Option<&str> {
        if self.content.text.starts_with("ERROR: ") {
            Some(&self.content.text)
        } else {
            None
        }
    }
}
