//! High-level memory management functionality

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use tokio::sync::{Mutex, RwLock, oneshot};

use crate::memory::{
    MemoryMetadata, MemoryRelationship, MemoryType, filter::MemoryFilter,
    repository::MemoryRepository, storage::MemoryStorage,
};
use crate::domain::memory::primitives::{
    types::MemoryTypeEnum,
    node::MemoryNode,
};
use crate::memory::utils::{Error, Result};
use crate::memory::vector::VectorStore;

/// High-level memory manager that coordinates between different memory components
pub struct MemoryCoordinator<S, V>
where
    S: MemoryStorage,
    V: VectorStore,
{
    storage: Arc<S>,
    vector_store: Arc<Mutex<V>>,
    repository: Arc<RwLock<MemoryRepository>>,
}

impl<S, V> MemoryCoordinator<S, V>
where
    S: MemoryStorage + Send + Sync,
    V: VectorStore + Send + Sync,
{
    /// Create a new memory coordinator
    pub fn new(storage: Arc<S>, vector_store: V) -> Self {
        Self {
            storage,
            vector_store: Arc::new(Mutex::new(vector_store)),
            repository: Arc::new(RwLock::new(MemoryRepository::new())),
        }
    }

    /// Add a new memory
    pub async fn add_memory(
        &self,
        content: String,
        memory_type: MemoryTypeEnum,
        metadata: MemoryMetadata,
    ) -> Result<MemoryNode> {
        // Create memory node
        let content_struct = crate::domain::memory::primitives::types::MemoryContent::text(content);
        let mut memory = MemoryNode::new(memory_type, content_struct);
        // Convert MemoryMetadata to MemoryNodeMetadata
        let node_metadata = crate::domain::memory::primitives::node::MemoryNodeMetadata {
            importance: metadata.importance,
            keywords: metadata.keywords.into_iter().map(|k| k.into()).collect(),
            tags: metadata.tags.into_iter().map(|t| t.into()).collect(),
            custom: std::collections::HashMap::new(),
            version: 1,
        };
        memory.metadata = std::sync::Arc::new(crossbeam_utils::CachePadded::new(node_metadata));

        // Generate embedding for the content
        let content_text = memory.content().to_string();
        let embedding_vec = self.generate_embedding(&content_text).await?;
        memory.embedding = Some(crate::domain::memory::primitives::node::AlignedEmbedding::new(embedding_vec.clone()));

        // Add to vector store
        {
            let vector_store = self.vector_store.lock().await;
            (*vector_store)
                .add(memory.id().to_string(), embedding_vec.clone(), None)
                .await?;
        }

        // Convert domain MemoryNode to memory MemoryNode for storage
        let memory_for_storage = self.convert_domain_to_memory_node(&memory);

        // Store in persistent storage
        self.storage.store(memory_for_storage.clone()).await?;

        // Add to repository
        self.repository.write().await.add(memory_for_storage);

        Ok(memory)
    }

    /// Update an existing memory
    pub async fn update_memory(
        &self,
        id: &str,
        content: Option<String>,
        metadata: Option<MemoryMetadata>,
    ) -> Result<MemoryNode> {
        let mut memory = self.storage.retrieve(id.to_string()).await?;

        // Update content if provided
        if let Some(new_content) = content {
            memory.content = super::super::primitives::types::MemoryContent::new(&new_content);

            // Re-generate embedding for updated content
            let embedding = self.generate_embedding(&memory.content.text).await?;
            memory.embedding = Some(embedding.clone());

            // Update in vector store
            {
                let vector_store = self.vector_store.lock().await;
                (*vector_store)
                    .update(memory.id.clone(), embedding.clone(), None)
                    .await?;
            }
        }

        // Update metadata if provided
        if let Some(new_metadata) = metadata {
            memory.metadata = new_metadata;
        }

        // Update in storage
        self.storage.update(memory.clone()).await?;

        // Update in repository
        self.repository.write().await.update(memory.clone());

        // Convert to domain MemoryNode for return
        Ok(self.convert_memory_to_domain_node(&memory))
    }

    /// Delete a memory
    pub async fn delete_memory(&self, id: &str) -> Result<()> {
        // Remove from vector store
        {
            let vector_store = self.vector_store.lock().await;
            (*vector_store).delete(id.to_string()).await?;
        }

        // Remove from storage
        self.storage.delete(id.to_string()).await?;

        // Remove from repository
        self.repository.write().await.delete(id);

        Ok(())
    }

    /// Search for memories
    pub async fn search_memories(
        &self,
        query: &str,
        filter: Option<MemoryFilter>,
        top_k: usize,
    ) -> Result<Vec<MemoryNode>> {
        // Generate query embedding
        let query_embedding = {
            let vector_store = self.vector_store.lock().await;
            (*vector_store).embed(query.to_string()).await?
        };

        // Search in vector store
        let results = {
            let vector_store = self.vector_store.lock().await;
            (*vector_store)
                .search(query_embedding.clone(), top_k, filter)
                .await?
        };

        // Retrieve full memory nodes and convert to domain nodes
        let mut memories = Vec::new();
        for result in results {
            if let Ok(memory) = self.storage.retrieve(result.id.clone()).await {
                memories.push(self.convert_memory_to_domain_node(&memory));
            }
        }

        Ok(memories)
    }

    /// Get memories by filter
    pub async fn get_memories(&self, filter: MemoryFilter) -> Result<Vec<MemoryNode>> {
        let memories = self.repository.read().await.filter(&filter);
        Ok(memories
            .into_iter()
            .map(|arc_memory| self.convert_memory_to_domain_node(&*arc_memory))
            .collect())
    }

    /// Add a relationship between memories
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

        // Store relationship
        self.storage
            .store_relationship(relationship.clone())
            .await?;

        Ok(relationship)
    }

    /// Get relationships for a memory
    pub async fn get_relationships(&self, memory_id: &str) -> Result<Vec<MemoryRelationship>> {
        self.storage.get_relationships(memory_id.to_string()).await
    }

    /// Convert domain MemoryNode to memory MemoryNode for storage compatibility
    fn convert_domain_to_memory_node(&self, domain_node: &crate::domain::memory::primitives::node::MemoryNode) -> crate::memory::memory::primitives::node::MemoryNode {
        use chrono::Utc;
        use crate::memory::memory::primitives::{
            node::MemoryNode as MemoryMemoryNode,
            metadata::MemoryMetadata as MemoryMemoryMetadata,
            types::MemoryContent as MemoryMemoryContent,
            types::MemoryTypeEnum as MemoryMemoryTypeEnum,
        };

        let embedding_vec = domain_node.embedding().map(|aligned_emb| aligned_emb.to_vec());
        
        // Create memory system metadata from domain metadata
        let memory_metadata = MemoryMemoryMetadata {
            user_id: None,
            agent_id: None,
            context: "default".to_string(),
            importance: domain_node.metadata.importance,
            keywords: domain_node.metadata.keywords.iter().map(|k| k.to_string()).collect(),
            tags: domain_node.metadata.tags.iter().map(|t| t.to_string()).collect(),
            category: "general".to_string(),
            source: None,
            created_at: chrono::Utc::now(),
            last_accessed_at: None,
            embedding: embedding_vec.clone(),
            custom: serde_json::Value::Object(serde_json::Map::new()),
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
            created_at: Utc::now(), // Use current time as approximation
            updated_at: Utc::now(),
            embedding: embedding_vec,
            metadata: memory_metadata,
        }
    }

    /// Convert memory MemoryNode to domain MemoryNode for API compatibility
    fn convert_memory_to_domain_node(&self, memory_node: &crate::memory::memory::primitives::node::MemoryNode) -> crate::domain::memory::primitives::node::MemoryNode {
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

        // Parse UUID from string ID
        let uuid = Uuid::parse_str(&memory_node.id).unwrap_or_else(|_| Uuid::new_v4());

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
            custom: std::collections::HashMap::new(),
            version: 1,
        };
        domain_node.metadata = std::sync::Arc::new(crossbeam_utils::CachePadded::new(domain_metadata));

        domain_node
    }

    /// Generate embedding for text content
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        // Use a simple hash-based embedding for demonstration
        // In production, this would call an actual embedding service like OpenAI, Cohere, etc.
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash = hasher.finish();

        // Convert hash to a simple 384-dimensional embedding
        let mut embedding = Vec::with_capacity(384);
        let mut current_hash = hash;

        for _ in 0..384 {
            // Use different parts of the hash to generate diverse values
            current_hash = current_hash.wrapping_mul(1103515245).wrapping_add(12345);
            let normalized = (current_hash % 10000) as f32 / 10000.0 - 0.5; // Range: -0.5 to 0.5
            embedding.push(normalized);
        }

        // Normalize the embedding vector
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for value in &mut embedding {
                *value /= magnitude;
            }
        }

        Ok(embedding)
    }
}

/// Future type for memory operations
pub struct MemoryFuture<T> {
    rx: oneshot::Receiver<Result<T>>,
}

impl<T> MemoryFuture<T> {
    pub fn new(rx: oneshot::Receiver<Result<T>>) -> Self {
        Self { rx }
    }
}

impl<T> Future for MemoryFuture<T> {
    type Output = Result<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.rx).poll(cx) {
            Poll::Ready(Ok(result)) => Poll::Ready(result),
            Poll::Ready(Err(_)) => Poll::Ready(Err(Error::Internal(
                "Memory operation task failed".to_string(),
            ))),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Trait for memory management operations
pub trait MemoryManagement: Send + Sync {
    /// Add a new memory
    fn add(
        &self,
        content: String,
        memory_type: &dyn MemoryType,
        metadata: MemoryMetadata,
    ) -> MemoryFuture<MemoryNode>;

    /// Update an existing memory
    fn update(
        &self,
        id: &str,
        content: Option<String>,
        metadata: Option<MemoryMetadata>,
    ) -> MemoryFuture<MemoryNode>;

    /// Delete a memory
    fn delete(&self, id: &str) -> MemoryFuture<()>;

    /// Search for memories
    fn search(&self, query: &str, top_k: usize) -> MemoryFuture<Vec<MemoryNode>>;

    /// Get memories by filter
    fn filter(&self, filter: MemoryFilter) -> MemoryFuture<Vec<MemoryNode>>;
}
