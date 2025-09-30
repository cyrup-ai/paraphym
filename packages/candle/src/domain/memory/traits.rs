//! CandleMemory trait definition - mirrors paraphym-domain Memory trait exactly
//!
//! This trait provides the core memory interface for Candle-backed memory implementations,
//! enabling trait composition, testability, and 'room to move' architecture benefits.

use ystream::AsyncStream;
use serde::{Deserialize, Serialize};

use super::primitives::MemoryNode;

/// Wrapper for memory lookup results that implements `MessageChunk`
#[derive(Debug, Clone, Default)]
pub struct MemoryLookupResult {
    /// The memory node if found
    pub memory: Option<MemoryNode>,
}

impl MemoryLookupResult {
    /// Create a result with a found memory node
    pub fn found(memory: MemoryNode) -> Self {
        Self { memory: Some(memory) }
    }
    
    /// Create a result with no memory node found
    pub fn not_found() -> Self {
        Self { memory: None }
    }
}

impl cyrup_sugars::prelude::MessageChunk for MemoryLookupResult {
    fn bad_chunk(_error: String) -> Self {
        Self::default()
    }

    fn error(&self) -> Option<&str> {
        None
    }
}

/// `CandleMemory` trait - mirrors `paraphym-domain::Memory` exactly with Candle prefix
///
/// This trait enables:
/// - Trait composition for flexible memory architectures  
/// - Testability with in-memory implementations
/// - Zero-cost abstractions via static dispatch
/// - 'Room to move' for future memory backend changes
pub trait CandleMemory: Send + Sync + 'static {
    /// Store a memory node in the memory system
    ///
    /// # Arguments
    /// * `memory_node` - The memory node to store
    ///
    /// # Returns
    /// `AsyncStream` that completes when the memory is stored
    fn store_memory(&self, memory_node: &MemoryNode) -> AsyncStream<crate::domain::context::chunk::CandleUnit>;

    /// Retrieve memory nodes by similarity search
    ///
    /// # Arguments
    /// * `query` - The query text to search for
    /// * `limit` - Maximum number of results to return
    ///
    /// # Returns
    /// `AsyncStream` of memory nodes ranked by similarity
    fn search_memory(&self, query: &str, limit: usize) -> AsyncStream<MemoryNode>;

    /// Get memory node by ID
    ///
    /// # Arguments
    /// * `id` - The unique ID of the memory node
    ///
    /// # Returns
    /// `AsyncStream` containing the memory lookup result
    fn get_memory(&self, id: &str) -> AsyncStream<MemoryLookupResult>;

    /// Delete memory node by ID
    ///
    /// # Arguments
    /// * `id` - The unique ID of the memory node to delete
    ///
    /// # Returns
    /// `AsyncStream` that completes when the memory is deleted
    fn delete_memory(&self, id: &str) -> AsyncStream<crate::domain::context::chunk::CandleMemoryOperationResult>;

    /// Get memory statistics
    ///
    /// # Returns
    /// `AsyncStream` containing memory system statistics
    fn get_stats(&self) -> AsyncStream<CandleMemoryStats>;
}

/// Memory statistics for monitoring and debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleMemoryStats {
    /// Total number of memory nodes stored
    pub total_memories: u64,
    /// Total size of memory data in bytes
    pub total_size_bytes: u64,
    /// Average embedding dimension
    pub avg_embedding_dimension: f32,
    /// Memory system uptime in seconds
    pub uptime_seconds: u64,
    /// Cache hit ratio (0.0 to 1.0)
    pub cache_hit_ratio: f32,
}

impl Default for CandleMemoryStats {
    fn default() -> Self {
        Self {
            total_memories: 0,
            total_size_bytes: 0,
            avg_embedding_dimension: 768.0,
            uptime_seconds: 0,
            cache_hit_ratio: 0.0,
        }
    }
}

impl cyrup_sugars::prelude::MessageChunk for CandleMemoryStats {
    fn bad_chunk(_error: String) -> Self {
        Self {
            total_memories: 0,
            total_size_bytes: 0,
            avg_embedding_dimension: 0.0,
            uptime_seconds: 0,
            cache_hit_ratio: 0.0,
        }
    }

    fn error(&self) -> Option<&str> {
        None // CandleMemoryStats doesn't have an error field
    }
}

// Bridge implementation removed - calling code now uses SurrealDBMemoryManager directly
