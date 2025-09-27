//! CandleMemory trait definition - mirrors paraphym-domain Memory trait exactly
//!
//! This trait provides the core memory interface for Candle-backed memory implementations,
//! enabling trait composition, testability, and 'room to move' architecture benefits.

use ystream::AsyncStream;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::primitives::MemoryNode;

/// Wrapper for memory lookup results that implements MessageChunk
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

/// CandleMemory trait - mirrors paraphym-domain::Memory exactly with Candle prefix
///
/// This trait enables:
/// - Trait composition for flexible memory architectures  
/// - Testability with mock implementations
/// - Zero-cost abstractions via static dispatch
/// - 'Room to move' for future memory backend changes
pub trait CandleMemory: Send + Sync + 'static {
    /// Store a memory node in the memory system
    ///
    /// # Arguments
    /// * `memory_node` - The memory node to store
    ///
    /// # Returns
    /// AsyncStream that completes when the memory is stored
    fn store_memory(&self, memory_node: &MemoryNode) -> AsyncStream<crate::domain::context::chunk::CandleUnit>;

    /// Retrieve memory nodes by similarity search
    ///
    /// # Arguments
    /// * `query` - The query text to search for
    /// * `limit` - Maximum number of results to return
    ///
    /// # Returns
    /// AsyncStream of memory nodes ranked by similarity
    fn search_memory(&self, query: &str, limit: usize) -> AsyncStream<MemoryNode>;

    /// Get memory node by ID
    ///
    /// # Arguments
    /// * `id` - The unique ID of the memory node
    ///
    /// # Returns
    /// AsyncStream containing the memory lookup result
    fn get_memory(&self, id: &str) -> AsyncStream<MemoryLookupResult>;

    /// Delete memory node by ID
    ///
    /// # Arguments
    /// * `id` - The unique ID of the memory node to delete
    ///
    /// # Returns
    /// AsyncStream that completes when the memory is deleted
    fn delete_memory(&self, id: &str) -> AsyncStream<crate::domain::context::chunk::CandleMemoryOperationResult>;

    /// Get memory statistics
    ///
    /// # Returns
    /// AsyncStream containing memory system statistics
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

/// Mock memory implementation for testing
#[derive(Debug, Clone, Default)]
pub struct MockCandleMemory {
    /// Stored memory nodes for testing
    pub stored_nodes: std::sync::Arc<std::sync::Mutex<Vec<MemoryNode>>>,
}

impl MockCandleMemory {
    /// Create new mock memory instance
    pub fn new() -> Self {
        Self::default()
    }
}

impl CandleMemory for MockCandleMemory {
    fn store_memory(&self, memory_node: &MemoryNode) -> AsyncStream<crate::domain::context::chunk::CandleUnit> {
        let node = memory_node.clone();
        let stored = self.stored_nodes.clone();

        AsyncStream::with_channel(move |sender| {
            if let Ok(mut nodes) = stored.lock() {
                nodes.push(node);
            }
            let _ = sender.send(crate::domain::context::chunk::CandleUnit::default());
        })
    }

    fn search_memory(&self, _query: &str, limit: usize) -> AsyncStream<MemoryNode> {
        let stored = self.stored_nodes.clone();

        AsyncStream::with_channel(move |sender| {
            if let Ok(nodes) = stored.lock() {
                for node in nodes.iter().take(limit) {
                    let _ = sender.send(node.clone());
                }
            }
        })
    }

    fn get_memory(&self, id: &str) -> AsyncStream<MemoryLookupResult> {
        let search_uuid = match Uuid::parse_str(id) {
            Ok(uuid) => uuid,
            Err(_) => {
                return AsyncStream::with_channel(move |sender| {
                    let _ = sender.send(MemoryLookupResult::not_found());
                });
            }
        };
        let stored = self.stored_nodes.clone();

        AsyncStream::with_channel(move |sender| {
            if let Ok(nodes) = stored.lock() {
                let found = nodes
                    .iter()
                    .find(|node| node.base_memory.id == search_uuid)
                    .cloned();
                let result = match found {
                    Some(memory) => MemoryLookupResult::found(memory),
                    None => MemoryLookupResult::not_found(),
                };
                let _ = sender.send(result);
            } else {
                let _ = sender.send(MemoryLookupResult::not_found());
            }
        })
    }

    fn delete_memory(&self, id: &str) -> AsyncStream<crate::domain::context::chunk::CandleMemoryOperationResult> {
        let search_uuid = match Uuid::parse_str(id) {
            Ok(uuid) => uuid,
            Err(_) => {
                return AsyncStream::with_channel(move |sender| {
                    let _ = sender.send(crate::domain::context::chunk::CandleMemoryOperationResult::failure_with_type(
                        "Invalid UUID format",
                        "delete"
                    ));
                });
            }
        };
        let stored = self.stored_nodes.clone();

        AsyncStream::with_channel(move |sender| {
            if let Ok(mut nodes) = stored.lock() {
                let initial_len = nodes.len();
                nodes.retain(|node| node.base_memory.id != search_uuid);
                let deleted = nodes.len() < initial_len;
                let result = if deleted {
                    crate::domain::context::chunk::CandleMemoryOperationResult::success_with_type("delete")
                } else {
                    crate::domain::context::chunk::CandleMemoryOperationResult::failure_with_type(
                        "Memory node not found",
                        "delete"
                    )
                };
                let _ = sender.send(result);
            } else {
                let _ = sender.send(crate::domain::context::chunk::CandleMemoryOperationResult::failure_with_type(
                    "Failed to acquire memory lock",
                    "delete"
                ));
            }
        })
    }

    fn get_stats(&self) -> AsyncStream<CandleMemoryStats> {
        let stored = self.stored_nodes.clone();

        AsyncStream::with_channel(move |sender| {
            let stats = if let Ok(nodes) = stored.lock() {
                CandleMemoryStats {
                    total_memories: nodes.len() as u64,
                    total_size_bytes: nodes
                        .iter()
                        .map(|n| n.base_memory.content.to_string().len() as u64)
                        .sum(),
                    avg_embedding_dimension: 768.0,
                    uptime_seconds: 0,
                    cache_hit_ratio: 1.0,
                }
            } else {
                CandleMemoryStats::default()
            };
            let _ = sender.send(stats);
        })
    }
}
