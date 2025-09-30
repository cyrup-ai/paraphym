//! High-level memory management functionality

use std::sync::Arc;
use std::time::SystemTime;

use tokio::sync::RwLock;
use chrono;

use crate::providers::bert_embedding::CandleBertEmbeddingProvider;
use crate::memory::vector::embedding_model::EmbeddingModel;

use crate::memory::{
    MemoryMetadata, MemoryRelationship, 
    repository::MemoryRepository,
};
use crate::memory::memory::ops::filter::MemoryFilter;
use crate::domain::memory::primitives::{
    types::MemoryTypeEnum,
    node::MemoryNode,
};
use crate::memory::utils::{Error, Result};
use crate::memory::memory::manager::surreal::{SurrealDBMemoryManager, MemoryManager};
use futures_util::StreamExt;

/// High-level memory manager that uses SurrealDB's native capabilities directly
pub struct MemoryCoordinator {
    surreal_manager: Arc<SurrealDBMemoryManager>,
    repository: Arc<RwLock<MemoryRepository>>,
    embedding_model: Arc<dyn EmbeddingModel>,
}

impl MemoryCoordinator {
    /// Create a new memory coordinator with SurrealDB
    pub async fn new(surreal_manager: Arc<SurrealDBMemoryManager>) -> Result<Self> {
        // Initialize BERT embedding model
        let embedding_model = Arc::new(CandleBertEmbeddingProvider::new().await?) as Arc<dyn EmbeddingModel>;

        Ok(Self {
            surreal_manager,
            repository: Arc::new(RwLock::new(MemoryRepository::new())),
            embedding_model,
        })
    }

    /// Add a new memory using SurrealDB's native capabilities
    pub async fn add_memory(
        &self,
        content: String,
        memory_type: MemoryTypeEnum,
        metadata: MemoryMetadata,
    ) -> Result<MemoryNode> {
        // Create domain memory node first
        let content_struct = crate::domain::memory::primitives::types::MemoryContent::text(content.clone());
        let mut domain_memory = MemoryNode::new(memory_type, content_struct);
        domain_memory.set_importance(metadata.importance)
            .map_err(|e| Error::Internal(format!("Failed to set importance: {}", e)))?;

        // Apply metadata keywords and tags
        for keyword in &metadata.keywords {
            domain_memory.set_custom_metadata(
                format!("keyword_{}", keyword),
                serde_json::Value::String(keyword.clone())
            );
        }

        for tag in &metadata.tags {
            domain_memory.set_custom_metadata(
                format!("tag_{}", tag),
                serde_json::Value::String(tag.clone())
            );
        }

        // Generate embedding for the content
        let embedding_vec = self.generate_embedding(&content).await?;
        domain_memory.embedding = Some(crate::domain::memory::primitives::node::AlignedEmbedding::new(embedding_vec.clone()));

        // Convert to memory system format for SurrealDB storage
        let memory_for_storage = self.convert_domain_to_memory_node(&domain_memory);

        // Store in SurrealDB - it handles embedding indexing natively
        let stored_memory = self.surreal_manager.create_memory(memory_for_storage.clone()).await?;

        // Add to repository cache
        self.repository.write().await.add(memory_for_storage);

        // Convert stored memory back to domain format for return
        Ok(self.convert_memory_to_domain_node(&stored_memory)?)
    }

    /// Update an existing memory using SurrealDB's native capabilities
    pub async fn update_memory(
        &self,
        id: &str,
        content: Option<String>,
        metadata: Option<MemoryMetadata>,
    ) -> Result<MemoryNode> {
        // Get existing memory from SurrealDB
        let existing_memory = self.surreal_manager.get_memory(id).await?.ok_or_else(||
            Error::NotFound(format!("Memory with id {} not found", id)))?;

        let mut updated_memory = existing_memory;

        // Update content if provided
        if let Some(new_content) = content {
            updated_memory.content = crate::memory::memory::primitives::types::MemoryContent::new(&new_content);

            // Re-generate embedding for updated content
            let embedding = self.generate_embedding(&new_content).await?;
            updated_memory.embedding = Some(embedding);
        }

        // Update metadata if provided
        if let Some(new_metadata) = metadata {
            updated_memory.metadata = new_metadata;
        }

        // Update in SurrealDB - it handles embedding indexing natively
        let stored_memory = self.surreal_manager.update_memory(updated_memory.clone()).await?;

        // Update in repository cache
        self.repository.write().await.update(updated_memory);

        // Convert to domain MemoryNode for return
        Ok(self.convert_memory_to_domain_node(&stored_memory)?)
    }

    /// Delete a memory using SurrealDB's native capabilities
    pub async fn delete_memory(&self, id: &str) -> Result<()> {
        // Delete from SurrealDB - it handles vector index removal natively
        self.surreal_manager.delete_memory(id).await?;

        // Remove from repository cache
        self.repository.write().await.delete(id);

        Ok(())
    }

    /// Search for memories using native SurrealDB vector search
    pub async fn search_memories(
        &self,
        query: &str,
        filter: Option<MemoryFilter>,
        top_k: usize,
    ) -> Result<Vec<MemoryNode>> {
        // Generate embedding for the query using the same method as add_memory
        let query_embedding = self.generate_embedding(query).await?;

        // Use SurrealDB's native vector similarity search directly
        let memory_stream = self.surreal_manager.search_by_vector(query_embedding, top_k);

        // Collect results using StreamExt::collect()
        let memories: Vec<_> = memory_stream.collect().await;

        // Convert to domain MemoryNodes with proper error handling
        let mut result_memories = Vec::new();
        for memory_result in memories {
            match memory_result {
                Ok(memory) => {
                    let domain_memory = self.convert_memory_to_domain_node(&memory)?;
                    result_memories.push(domain_memory);
                }
                Err(e) => return Err(Error::Internal(format!("Search failed: {}", e))),
            }
        }

        // Apply filter if provided
        let filtered_memories = if let Some(filter) = filter {
            result_memories.into_iter()
                .filter(|memory| {
                    // Apply memory type filter
                    if let Some(memory_types) = &filter.memory_types {
                        let domain_memory_type = memory.memory_type();
                        let converted_type = match domain_memory_type {
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Semantic => crate::memory::memory::primitives::types::MemoryTypeEnum::Semantic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Episodic => crate::memory::memory::primitives::types::MemoryTypeEnum::Episodic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Procedural => crate::memory::memory::primitives::types::MemoryTypeEnum::Procedural,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Working => crate::memory::memory::primitives::types::MemoryTypeEnum::Working,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::LongTerm => crate::memory::memory::primitives::types::MemoryTypeEnum::LongTerm,
                            // Map additional domain variants to closest memory system equivalents
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Fact => crate::memory::memory::primitives::types::MemoryTypeEnum::Semantic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Episode => crate::memory::memory::primitives::types::MemoryTypeEnum::Episodic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Declarative => crate::memory::memory::primitives::types::MemoryTypeEnum::Semantic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Implicit => crate::memory::memory::primitives::types::MemoryTypeEnum::Procedural,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Explicit => crate::memory::memory::primitives::types::MemoryTypeEnum::Semantic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Contextual => crate::memory::memory::primitives::types::MemoryTypeEnum::Semantic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Temporal => crate::memory::memory::primitives::types::MemoryTypeEnum::Episodic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Spatial => crate::memory::memory::primitives::types::MemoryTypeEnum::Episodic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Associative => crate::memory::memory::primitives::types::MemoryTypeEnum::Semantic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Emotional => crate::memory::memory::primitives::types::MemoryTypeEnum::Episodic,
                        };
                        if !memory_types.contains(&converted_type) {
                            return false;
                        }
                    }

                    // Apply importance range filter
                    if let Some((min_importance, max_importance)) = filter.importance_range {
                        let importance = memory.importance();
                        if importance < min_importance || importance > max_importance {
                            return false;
                        }
                    }

                    // Apply time range filter
                    if let Some(time_range) = &filter.time_range {
                        if let Some(start) = time_range.start {
                            let start_system_time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(start.timestamp() as u64);
                            if memory.base_memory.created_at < start_system_time {
                                return false;
                            }
                        }
                        if let Some(end) = time_range.end {
                            let end_system_time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(end.timestamp() as u64);
                            if memory.base_memory.created_at >= end_system_time {
                                return false;
                            }
                        }
                    }

                    true
                })
                .collect()
        } else {
            result_memories
        };

        Ok(filtered_memories)
    }

    /// Get memories by filter
    pub async fn get_memories(&self, filter: MemoryFilter) -> Result<Vec<MemoryNode>> {
        let memories = self.repository.read().await.filter(&filter);
        let mut result_memories = Vec::new();
        for arc_memory in memories {
            let domain_memory = self.convert_memory_to_domain_node(&*arc_memory)?;
            result_memories.push(domain_memory);
        }
        Ok(result_memories)
    }

    /// Add a relationship between memories using SurrealDB's native capabilities
    pub async fn add_relationship(
        &self,
        source_id: &str,
        target_id: &str,
        relationship_type: String,
        metadata: Option<serde_json::Value>,
    ) -> Result<MemoryRelationship> {
        let mut relationship = MemoryRelationship::new(
            source_id.to_string(),
            target_id.to_string(),
            relationship_type,
        );

        if let Some(metadata) = metadata {
            relationship = relationship.with_metadata(metadata);
        }

        // Store relationship in SurrealDB
        let stored_relationship = self.surreal_manager
            .create_relationship(relationship)
            .await?;

        Ok(stored_relationship)
    }

    /// Get relationships for a memory using SurrealDB's native capabilities
    pub async fn get_relationships(&self, memory_id: &str) -> Result<Vec<MemoryRelationship>> {
        // Use SurrealDB's native relationship retrieval directly
        let relationship_stream = self.surreal_manager.get_relationships(memory_id);

        // Collect results using StreamExt::collect()
        let relationships: Vec<_> = relationship_stream.collect().await;

        // Convert to MemoryRelationships with proper error handling
        let mut result_relationships = Vec::new();
        for relationship_result in relationships {
            match relationship_result {
                Ok(relationship) => result_relationships.push(relationship),
                Err(e) => return Err(Error::Internal(format!("Failed to retrieve relationships: {}", e))),
            }
        }

        Ok(result_relationships)
    }

    /// Convert domain MemoryNode to memory MemoryNode for storage compatibility
    fn convert_domain_to_memory_node(&self, domain_node: &crate::domain::memory::primitives::node::MemoryNode) -> crate::memory::memory::primitives::node::MemoryNode {
        use crate::memory::memory::primitives::{
            node::MemoryNode as MemoryMemoryNode,
            metadata::MemoryMetadata as MemoryMemoryMetadata,
            types::MemoryContent as MemoryMemoryContent,
            types::MemoryTypeEnum as MemoryMemoryTypeEnum,
        };

        let embedding_vec = domain_node.embedding().map(|aligned_emb| aligned_emb.to_vec());
        
        // Create memory system metadata preserving domain metadata
        let memory_metadata = MemoryMemoryMetadata {
            user_id: domain_node.metadata.custom.get("user_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            agent_id: domain_node.metadata.custom.get("agent_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            context: domain_node.metadata.custom.get("context")
                .and_then(|v| v.as_str())
                .unwrap_or("default")
                .to_string(),
            importance: domain_node.importance(),
            keywords: domain_node.metadata.keywords.iter().map(|k| k.to_string()).collect(),
            tags: domain_node.metadata.tags.iter().map(|t| t.to_string()).collect(),
            category: domain_node.metadata.custom.get("category")
                .and_then(|v| v.as_str())
                .unwrap_or("general")
                .to_string(),
            source: domain_node.metadata.custom.get("source")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            created_at: domain_node.base_memory.created_at.into(),
            last_accessed_at: Some(chrono::DateTime::<chrono::Utc>::from(domain_node.last_accessed())),
            embedding: embedding_vec.clone(),
            custom: serde_json::to_value(&domain_node.metadata.custom)
                .unwrap_or_else(|_| serde_json::Value::Object(serde_json::Map::new())),
        };

        // Create memory content
        let memory_content = MemoryMemoryContent::new(&domain_node.content().to_string());

        // Convert memory type - map to closest equivalent
        let memory_type = match domain_node.memory_type() {
            crate::domain::memory::primitives::types::MemoryTypeEnum::Semantic => MemoryMemoryTypeEnum::Semantic,
            crate::domain::memory::primitives::types::MemoryTypeEnum::Episodic => MemoryMemoryTypeEnum::Episodic,
            crate::domain::memory::primitives::types::MemoryTypeEnum::Procedural => MemoryMemoryTypeEnum::Procedural,
            crate::domain::memory::primitives::types::MemoryTypeEnum::Working => MemoryMemoryTypeEnum::Working,
            crate::domain::memory::primitives::types::MemoryTypeEnum::LongTerm => MemoryMemoryTypeEnum::LongTerm,
            // Map additional domain variants to closest memory system equivalents
            crate::domain::memory::primitives::types::MemoryTypeEnum::Fact => MemoryMemoryTypeEnum::Semantic,
            crate::domain::memory::primitives::types::MemoryTypeEnum::Episode => MemoryMemoryTypeEnum::Episodic,
            crate::domain::memory::primitives::types::MemoryTypeEnum::Declarative => MemoryMemoryTypeEnum::Semantic,
            crate::domain::memory::primitives::types::MemoryTypeEnum::Implicit => MemoryMemoryTypeEnum::Procedural,
            crate::domain::memory::primitives::types::MemoryTypeEnum::Explicit => MemoryMemoryTypeEnum::Semantic,
            crate::domain::memory::primitives::types::MemoryTypeEnum::Contextual => MemoryMemoryTypeEnum::Semantic,
            crate::domain::memory::primitives::types::MemoryTypeEnum::Temporal => MemoryMemoryTypeEnum::Episodic,
            crate::domain::memory::primitives::types::MemoryTypeEnum::Spatial => MemoryMemoryTypeEnum::Episodic,
            crate::domain::memory::primitives::types::MemoryTypeEnum::Associative => MemoryMemoryTypeEnum::Semantic,
            crate::domain::memory::primitives::types::MemoryTypeEnum::Emotional => MemoryMemoryTypeEnum::Episodic,
        };

        MemoryMemoryNode {
            id: domain_node.id().to_string(),
            content: memory_content,
            memory_type,
            created_at: domain_node.base_memory.created_at.into(),
            updated_at: domain_node.base_memory.updated_at.into(),
            embedding: embedding_vec,
            metadata: memory_metadata,
        }
    }

    /// Convert memory MemoryNode to domain MemoryNode for API compatibility
    fn convert_memory_to_domain_node(&self, memory_node: &crate::memory::memory::primitives::node::MemoryNode) -> Result<crate::domain::memory::primitives::node::MemoryNode> {
        use uuid::Uuid;
        use crate::domain::memory::primitives::{
            node::{MemoryNode as DomainMemoryNode, MemoryNodeMetadata, AlignedEmbedding},
            types::{MemoryContent as DomainMemoryContent, MemoryTypeEnum as DomainMemoryTypeEnum},
        };

        // Convert memory type - map to closest equivalent
        let domain_memory_type = match memory_node.memory_type {
            crate::memory::memory::primitives::types::MemoryTypeEnum::Semantic => DomainMemoryTypeEnum::Semantic,
            crate::memory::memory::primitives::types::MemoryTypeEnum::Episodic => DomainMemoryTypeEnum::Episodic,
            crate::memory::memory::primitives::types::MemoryTypeEnum::Procedural => DomainMemoryTypeEnum::Procedural,
            crate::memory::memory::primitives::types::MemoryTypeEnum::Working => DomainMemoryTypeEnum::Working,
            crate::memory::memory::primitives::types::MemoryTypeEnum::LongTerm => DomainMemoryTypeEnum::LongTerm,
        };

        // Create domain content
        let domain_content = DomainMemoryContent::text(&memory_node.content.text);

        // Parse UUID from string ID - fail fast on corruption
        let uuid = Uuid::parse_str(&memory_node.id)
            .map_err(|e| Error::Internal(format!("Invalid UUID in memory node {}: {}", memory_node.id, e)))?;

        // Create domain node
        let mut domain_node = DomainMemoryNode::with_id(uuid, domain_memory_type, domain_content);

        // Convert embedding if present
        if let Some(embedding_vec) = &memory_node.embedding {
            domain_node.embedding = Some(AlignedEmbedding::new(embedding_vec.clone()));
        }

        // Convert metadata
        let domain_metadata = MemoryNodeMetadata {
            importance: memory_node.metadata.importance,
            keywords: memory_node.metadata.keywords.iter().map(|k| k.clone().into()).collect(),
            tags: memory_node.metadata.tags.iter().map(|t| t.clone().into()).collect(),
            custom: memory_node.metadata.custom
                .as_object()
                .map(|obj| obj.iter()
                    .map(|(k, v)| (Arc::from(k.as_str()), Arc::new(v.clone())))
                    .collect())
                .unwrap_or_default(),
            version: 1,
        };
        domain_node.metadata = std::sync::Arc::new(crossbeam_utils::CachePadded::new(domain_metadata));

        Ok(domain_node)
    }

    /// Generate embedding for text content using BERT model
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        // Use existing BERT embedding provider
        let embedding = self.embedding_model.embed(text, None)
            .map_err(|e| Error::Internal(format!("BERT embedding failed: {}", e)))?;
        Ok(embedding)
    }
}


