//! MemoryManager trait implementation for coordinator

use crate::domain::memory::cognitive::types::{CognitiveState, EntanglementType};
use crate::memory::core::manager::surreal::{
    MemoryManager, MemoryStream, PendingDeletion, PendingEntanglementEdge, PendingMemory,
    PendingQuantumSignature, PendingQuantumUpdate, PendingRelationship, RelationshipStream,
};
use crate::memory::core::primitives::{
    node::MemoryNode as CoreMemoryNode, types::MemoryTypeEnum as CoreMemoryTypeEnum,
};
use crate::memory::MemoryRelationship;

use super::lifecycle::MemoryCoordinator;

impl MemoryManager for MemoryCoordinator {
    // Delegate non-search methods directly to surreal_manager

    fn create_memory(&self, memory: CoreMemoryNode) -> PendingMemory {
        self.surreal_manager.create_memory(memory)
    }

    fn get_memory(&self, id: &str) -> crate::memory::core::manager::surreal::MemoryQuery {
        self.surreal_manager.get_memory(id)
    }

    fn update_memory(&self, memory: CoreMemoryNode) -> PendingMemory {
        self.surreal_manager.update_memory(memory)
    }

    fn delete_memory(&self, id: &str) -> PendingDeletion {
        self.surreal_manager.delete_memory(id)
    }

    fn create_relationship(&self, relationship: MemoryRelationship) -> PendingRelationship {
        self.surreal_manager.create_relationship(relationship)
    }

    fn get_relationships(&self, memory_id: &str) -> RelationshipStream {
        self.surreal_manager.get_relationships(memory_id)
    }

    fn delete_relationship(&self, id: &str) -> PendingDeletion {
        self.surreal_manager.delete_relationship(id)
    }

    fn query_by_type(&self, memory_type: CoreMemoryTypeEnum) -> MemoryStream {
        self.surreal_manager.query_by_type(memory_type)
    }

    // QUANTUM-ROUTED SEARCH METHODS

    fn search_by_content(&self, query: &str) -> MemoryStream {
        let query = query.to_string();
        let self_clone = self.clone();

        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            // Generate embedding lazily when stream is consumed
            match self_clone.generate_embedding(&query).await {
                Ok(emb) => {
                    // Use vector search with cosine similarity
                    let mut stream = self_clone.surreal_manager.search_by_vector(emb, 10);

                    // Forward results through sender
                    use futures_util::StreamExt;
                    while let Some(result) = stream.next().await {
                        if tx.send(result).await.is_err() {
                            break;
                        }
                    }
                }
                Err(e) => {
                    // Fall back to substring search
                    log::warn!(
                        "Embedding generation failed, falling back to substring search: {}",
                        e
                    );

                    let mut stream = self_clone.surreal_manager.search_by_content(&query);

                    // Forward results through sender
                    use futures_util::StreamExt;
                    while let Some(result) = stream.next().await {
                        if tx.send(result).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });

        MemoryStream::new(rx)
    }

    fn search_by_vector(&self, vector: Vec<f32>, limit: usize) -> MemoryStream {
        // Vector search already uses quantum strategy by default
        self.surreal_manager.search_by_vector(vector, limit)
    }

    fn search_by_temporal(&self, query: &str, limit: usize) -> MemoryStream {
        self.surreal_manager.search_by_temporal(query, limit)
    }

    fn search_by_pattern(&self, query: &str, limit: usize) -> MemoryStream {
        self.surreal_manager.search_by_pattern(query, limit)
    }

    fn update_quantum_signature(
        &self,
        memory_id: &str,
        cognitive_state: &CognitiveState,
    ) -> PendingQuantumUpdate {
        self.surreal_manager
            .update_quantum_signature(memory_id, cognitive_state)
    }

    fn get_quantum_signature(&self, memory_id: &str) -> PendingQuantumSignature {
        self.surreal_manager.get_quantum_signature(memory_id)
    }

    fn create_entanglement_edge(
        &self,
        source_id: &str,
        target_id: &str,
        strength: f32,
        bond_type: EntanglementType,
    ) -> PendingEntanglementEdge {
        self.surreal_manager
            .create_entanglement_edge(source_id, target_id, strength, bond_type)
    }

    fn get_entangled_memories(&self, memory_id: &str, min_strength: f32) -> MemoryStream {
        self.surreal_manager
            .get_entangled_memories(memory_id, min_strength)
    }

    fn get_entangled_by_type(&self, memory_id: &str, bond_type: EntanglementType) -> MemoryStream {
        self.surreal_manager
            .get_entangled_by_type(memory_id, bond_type)
    }

    fn traverse_entanglement_graph(&self, memory_id: &str, max_depth: usize) -> MemoryStream {
        self.surreal_manager
            .traverse_entanglement_graph(memory_id, max_depth)
    }

    fn expand_via_entanglement(&self, memory_ids: Vec<String>, min_strength: f32) -> MemoryStream {
        self.surreal_manager
            .expand_via_entanglement(memory_ids, min_strength)
    }
}

// Optional: Add Drop implementation for automatic cleanup
impl Drop for MemoryCoordinator {
    fn drop(&mut self) {
        // Implement graceful shutdown of workers
        self.shutdown_workers();
    }
}
