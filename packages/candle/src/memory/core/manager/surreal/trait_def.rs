//! MemoryManager trait definition for memory operations.
//! 
//! This module defines the core trait for memory management systems,
//! providing a unified interface for CRUD operations, search, and
//! quantum entanglement features.

use crate::domain::memory::cognitive::types::{CognitiveState, EntanglementType};
use crate::memory::primitives::{MemoryNode, MemoryRelationship};

use super::futures::{
    MemoryQuery, MemoryStream, PendingDeletion, PendingEntanglementEdge, PendingMemory,
    PendingQuantumSignature, PendingQuantumUpdate, PendingRelationship, RelationshipStream,
};

/// Core memory management trait defining operations for storing, retrieving, and managing memory nodes
pub trait MemoryManager: Send + Sync {
    // === Core Memory CRUD Operations ===

    /// Create a new memory node
    fn create_memory(&self, memory: MemoryNode) -> PendingMemory;

    /// Get a memory node by ID
    fn get_memory(&self, id: &str) -> MemoryQuery;

    /// Update an existing memory node
    fn update_memory(&self, memory: MemoryNode) -> PendingMemory;

    /// Delete a memory node by ID
    fn delete_memory(&self, id: &str) -> PendingDeletion;

    // === Search and Query Operations ===

    /// Search memories by embedding vector similarity
    fn search_by_vector(&self, vector: Vec<f32>, limit: usize) -> MemoryStream;

    /// List all memories with pagination support
    fn list_all_memories(&self, limit: usize, offset: usize) -> MemoryStream;

    // === Relationship Operations ===

    /// Create a relationship between two memories
    fn create_relationship(&self, relationship: MemoryRelationship) -> PendingRelationship;

    /// Get all relationships for a memory node
    fn get_relationships(&self, memory_id: &str) -> RelationshipStream;

    /// Delete a relationship by ID
    fn delete_relationship(&self, id: &str) -> PendingDeletion;

    // === Quantum Memory Operations ===

    /// Update quantum signature for a memory node
    fn update_quantum_signature(
        &self,
        memory_id: &str,
        signature: CognitiveState,
    ) -> PendingQuantumUpdate;

    /// Get quantum signature for a memory node
    fn get_quantum_signature(&self, memory_id: &str) -> PendingQuantumSignature;

    // === Entanglement Operations ===

    /// Create an entanglement edge between two memories
    fn create_entanglement_edge(
        &self,
        source_id: &str,
        target_id: &str,
        entanglement_type: EntanglementType,
        strength: f32,
    ) -> PendingEntanglementEdge;

    /// Get all entangled memories for a given memory ID
    fn get_entangled_memories(&self, memory_id: &str) -> MemoryStream;

    /// Get entangled memories filtered by entanglement type
    fn get_entangled_by_type(
        &self,
        memory_id: &str,
        entanglement_type: EntanglementType,
    ) -> MemoryStream;

    /// Traverse entanglement graph from a starting memory
    fn traverse_entanglement_graph(
        &self,
        start_memory_id: &str,
        max_depth: usize,
        min_strength: f32,
    ) -> MemoryStream;

    /// Expand search results via entanglement relationships
    fn expand_via_entanglement(
        &self,
        seed_memory_ids: Vec<String>,
        expansion_factor: usize,
        min_strength: f32,
    ) -> MemoryStream;
}
