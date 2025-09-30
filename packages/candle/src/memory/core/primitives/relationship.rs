//! Memory relationships implementation
//! This module provides functionality for connecting memory nodes with relationships

use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use cyrup_sugars::prelude::*;

/// Represents the direction of a relationship
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RelationshipDirection {
    /// One-way relationship from source to target
    OneWay,
    /// Two-way relationship between source and target
    TwoWay,
}

impl fmt::Display for RelationshipDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelationshipDirection::OneWay => write!(f, "one_way"),
            RelationshipDirection::TwoWay => write!(f, "two_way"),
        }
    }
}

/// A relationship between two memory nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRelationship {
    /// Unique identifier for the relationship
    pub id: String,
    /// ID of the source memory node
    pub source_id: String,
    /// ID of the target memory node
    pub target_id: String,
    /// Type of relationship
    pub relationship_type: String,
    /// Additional metadata
    pub metadata: Option<Value>,
}

impl MemoryRelationship {
    /// Create a new memory relationship
    pub fn new(source_id: String, target_id: String, relationship_type: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            source_id,
            target_id,
            relationship_type,
            metadata: None,
        }
    }

    /// Create a new relationship with a specific ID
    pub fn with_id(
        id: String,
        source_id: String,
        target_id: String,
        relationship_type: String,
    ) -> Self {
        Self {
            id,
            source_id,
            target_id,
            relationship_type,
            metadata: None,
        }
    }

    /// Add metadata to the relationship
    pub fn with_metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl Default for MemoryRelationship {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            source_id: String::new(),
            target_id: String::new(),
            relationship_type: "unknown".to_string(),
            metadata: None,
        }
    }
}

impl MessageChunk for MemoryRelationship {
    fn bad_chunk(error: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            source_id: "error".to_string(),
            target_id: "error".to_string(),
            relationship_type: "error".to_string(),
            metadata: Some(serde_json::json!({"error": error})),
        }
    }

    fn error(&self) -> Option<&str> {
        self.metadata
            .as_ref()
            .and_then(|m| m.get("error"))
            .and_then(|e| e.as_str())
    }
}
