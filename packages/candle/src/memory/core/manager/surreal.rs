// src/memory/memory_manager.rs
//! Memory manager implementation for SurrealDB.
//! This module provides the core functionality for managing memory nodes and relationships.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::any::Any;

// Remove imports that conflict with local definitions
use crate::domain::memory::cognitive::types::{CognitiveState, EntanglementType};
use crate::memory::migration::{
    BuiltinMigrations, DataExporter, DataImporter, ExportFormat, ImportFormat, MigrationManager,
};
use crate::memory::primitives::types::MemoryTypeEnum;
use crate::memory::primitives::{MemoryNode, MemoryRelationship};
use crate::memory::schema::memory_schema::{MemoryMetadataSchema, MemoryNodeSchema};
use crate::memory::schema::quantum_schema::QuantumSignatureSchema;
use crate::memory::schema::relationship_schema::Relationship;
use crate::memory::utils::error::Error;
use std::path::Path;

// Vector search and embedding imports
use crate::capability::registry::TextEmbeddingModel;
use crate::capability::traits::TextEmbeddingCapable;
use log; // For logging in create_memory

/// Content structure for creating/updating memory nodes (without ID)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MemoryNodeCreateContent {
    pub content: String,
    pub content_hash: u64,
    pub memory_type: MemoryTypeEnum,
    pub metadata: MemoryMetadataSchema,
}

impl From<&MemoryNode> for MemoryNodeCreateContent {
    fn from(memory: &MemoryNode) -> Self {
        Self {
            content: memory.content.text.clone(),
            content_hash: memory.content_hash,
            memory_type: memory.memory_type,
            metadata: MemoryMetadataSchema {
                created_at: memory.metadata.created_at,
                last_accessed_at: memory
                    .metadata
                    .last_accessed_at
                    .unwrap_or(memory.metadata.created_at),
                importance: memory.metadata.importance,
                embedding: memory.metadata.embedding.clone(),
                custom: memory.metadata.custom.clone(),
            },
        }
    }
}

/// Content structure for creating relationships (without ID)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RelationshipCreateContent {
    pub source_id: String,
    pub target_id: String,
    pub relationship_type: String,
    pub metadata: serde_json::Value,
    pub created_at: u64,
    pub updated_at: u64,
    pub strength: f32,
}

impl From<&MemoryRelationship> for RelationshipCreateContent {
    fn from(relationship: &MemoryRelationship) -> Self {
        // Preserve timestamps if present, generate if absent
        let now = crate::memory::utils::current_timestamp_ms();

        Self {
            source_id: relationship.source_id.clone(),
            target_id: relationship.target_id.clone(),
            relationship_type: relationship.relationship_type.clone(),
            metadata: relationship
                .metadata
                .clone()
                .unwrap_or_else(|| serde_json::Value::Object(serde_json::Map::new())),
            created_at: relationship.created_at.unwrap_or(now),
            updated_at: relationship.updated_at.unwrap_or(now),
            strength: relationship.strength.unwrap_or(1.0),
        }
    }
}

use crate::memory::primitives::metadata::MemoryMetadata;

/// Result type for memory operations
pub type Result<T> = std::result::Result<T, Error>;

/// A pending memory operation that resolves to a MemoryNode
pub struct PendingMemory {
    rx: tokio::sync::oneshot::Receiver<Result<MemoryNode>>,
}

impl PendingMemory {
    pub fn new(rx: tokio::sync::oneshot::Receiver<Result<MemoryNode>>) -> Self {
        Self { rx }
    }
}

impl Future for PendingMemory {
    type Output = Result<MemoryNode>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match Pin::new(&mut self.rx).poll(cx) {
            std::task::Poll::Ready(Ok(result)) => std::task::Poll::Ready(result),
            std::task::Poll::Ready(Err(_)) => {
                std::task::Poll::Ready(Err(Error::Other("Channel closed".to_string())))
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

/// A query for a specific memory
pub struct MemoryQuery {
    rx: tokio::sync::oneshot::Receiver<Result<Option<MemoryNode>>>,
}

impl MemoryQuery {
    pub fn new(rx: tokio::sync::oneshot::Receiver<Result<Option<MemoryNode>>>) -> Self {
        Self { rx }
    }
}

impl Future for MemoryQuery {
    type Output = Result<Option<MemoryNode>>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match Pin::new(&mut self.rx).poll(cx) {
            std::task::Poll::Ready(Ok(result)) => std::task::Poll::Ready(result),
            std::task::Poll::Ready(Err(_)) => {
                std::task::Poll::Ready(Err(Error::Other("Channel closed".to_string())))
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

/// A pending deletion operation
pub struct PendingDeletion {
    rx: tokio::sync::oneshot::Receiver<Result<bool>>,
}

impl PendingDeletion {
    fn new(rx: tokio::sync::oneshot::Receiver<Result<bool>>) -> Self {
        Self { rx }
    }
}

impl Future for PendingDeletion {
    type Output = Result<bool>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match Pin::new(&mut self.rx).poll(cx) {
            std::task::Poll::Ready(Ok(result)) => std::task::Poll::Ready(result),
            std::task::Poll::Ready(Err(_)) => {
                std::task::Poll::Ready(Err(Error::Other("Channel closed".to_string())))
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

/// A pending relationship operation
pub struct PendingRelationship {
    rx: tokio::sync::oneshot::Receiver<Result<MemoryRelationship>>,
}

impl PendingRelationship {
    fn new(rx: tokio::sync::oneshot::Receiver<Result<MemoryRelationship>>) -> Self {
        Self { rx }
    }
}

impl Future for PendingRelationship {
    type Output = Result<MemoryRelationship>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match Pin::new(&mut self.rx).poll(cx) {
            std::task::Poll::Ready(Ok(result)) => std::task::Poll::Ready(result),
            std::task::Poll::Ready(Err(_)) => {
                std::task::Poll::Ready(Err(Error::Other("Channel closed".to_string())))
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

/// Pending quantum signature update operation
pub struct PendingQuantumUpdate {
    rx: tokio::sync::oneshot::Receiver<Result<()>>,
}

impl PendingQuantumUpdate {
    pub fn new(rx: tokio::sync::oneshot::Receiver<Result<()>>) -> Self {
        Self { rx }
    }
}

impl Future for PendingQuantumUpdate {
    type Output = Result<()>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match Pin::new(&mut self.rx).poll(cx) {
            std::task::Poll::Ready(Ok(result)) => std::task::Poll::Ready(result),
            std::task::Poll::Ready(Err(_)) => {
                std::task::Poll::Ready(Err(Error::Other("Channel closed".to_string())))
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

/// Pending quantum signature retrieval operation
pub struct PendingQuantumSignature {
    rx: tokio::sync::oneshot::Receiver<Result<Option<CognitiveState>>>,
}

impl PendingQuantumSignature {
    pub fn new(rx: tokio::sync::oneshot::Receiver<Result<Option<CognitiveState>>>) -> Self {
        Self { rx }
    }
}

impl Future for PendingQuantumSignature {
    type Output = Result<Option<CognitiveState>>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match Pin::new(&mut self.rx).poll(cx) {
            std::task::Poll::Ready(Ok(result)) => std::task::Poll::Ready(result),
            std::task::Poll::Ready(Err(_)) => {
                std::task::Poll::Ready(Err(Error::Other("Channel closed".to_string())))
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

/// Pending entanglement edge creation (RELATE operation)
pub struct PendingEntanglementEdge {
    rx: tokio::sync::oneshot::Receiver<Result<()>>,
}

impl PendingEntanglementEdge {
    pub fn new(rx: tokio::sync::oneshot::Receiver<Result<()>>) -> Self {
        Self { rx }
    }
}

impl Future for PendingEntanglementEdge {
    type Output = Result<()>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match Pin::new(&mut self.rx).poll(cx) {
            std::task::Poll::Ready(Ok(result)) => std::task::Poll::Ready(result),
            std::task::Poll::Ready(Err(_)) => {
                std::task::Poll::Ready(Err(Error::Other("Channel closed".to_string())))
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

/// Pending single embedding generation operation
pub struct PendingEmbedding {
    rx: tokio::sync::oneshot::Receiver<Result<Vec<f32>>>,
}

impl PendingEmbedding {
    pub fn new(rx: tokio::sync::oneshot::Receiver<Result<Vec<f32>>>) -> Self {
        Self { rx }
    }
}

impl Future for PendingEmbedding {
    type Output = Result<Vec<f32>>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match Pin::new(&mut self.rx).poll(cx) {
            std::task::Poll::Ready(Ok(result)) => std::task::Poll::Ready(result),
            std::task::Poll::Ready(Err(_)) => {
                std::task::Poll::Ready(Err(Error::Other("Channel closed".to_string())))
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

/// Pending batch embedding generation operation
pub struct PendingBatchEmbedding {
    rx: tokio::sync::oneshot::Receiver<Result<Vec<Vec<f32>>>>,
}

impl PendingBatchEmbedding {
    pub fn new(rx: tokio::sync::oneshot::Receiver<Result<Vec<Vec<f32>>>>) -> Self {
        Self { rx }
    }
}

impl Future for PendingBatchEmbedding {
    type Output = Result<Vec<Vec<f32>>>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match Pin::new(&mut self.rx).poll(cx) {
            std::task::Poll::Ready(Ok(result)) => std::task::Poll::Ready(result),
            std::task::Poll::Ready(Err(_)) => {
                std::task::Poll::Ready(Err(Error::Other("Channel closed".to_string())))
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

/// A stream of memory nodes
pub struct MemoryStream {
    rx: tokio::sync::mpsc::Receiver<Result<MemoryNode>>,
}

impl MemoryStream {
    pub(crate) fn new(rx: tokio::sync::mpsc::Receiver<Result<MemoryNode>>) -> Self {
        Self { rx }
    }
}

impl futures_util::Stream for MemoryStream {
    type Item = Result<MemoryNode>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}

/// A stream of memory relationships
pub struct RelationshipStream {
    rx: tokio::sync::mpsc::Receiver<Result<MemoryRelationship>>,
}

impl RelationshipStream {
    fn new(rx: tokio::sync::mpsc::Receiver<Result<MemoryRelationship>>) -> Self {
        Self { rx }
    }
}

impl futures_util::Stream for RelationshipStream {
    type Item = Result<MemoryRelationship>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}

/// Memory manager trait - no async methods, returns concrete types
pub trait MemoryManager: Send + Sync + 'static {
    /// Create a new memory node
    fn create_memory(&self, memory: MemoryNode) -> PendingMemory;

    /// Get a memory node by ID
    fn get_memory(&self, id: &str) -> MemoryQuery;

    /// Update a memory node
    fn update_memory(&self, memory: MemoryNode) -> PendingMemory;

    /// Delete a memory node
    fn delete_memory(&self, id: &str) -> PendingDeletion;

    /// Create a relationship between memory nodes
    fn create_relationship(&self, relationship: MemoryRelationship) -> PendingRelationship;

    /// Get relationships for a memory node
    fn get_relationships(&self, memory_id: &str) -> RelationshipStream;

    /// Delete a relationship
    fn delete_relationship(&self, id: &str) -> PendingDeletion;

    /// Query memories by type
    fn query_by_type(&self, memory_type: MemoryTypeEnum) -> MemoryStream;

    /// Search memories by content
    fn search_by_content(&self, query: &str) -> MemoryStream;

    /// Search memories by vector similarity
    fn search_by_vector(&self, vector: Vec<f32>, limit: usize) -> MemoryStream;

    /// Search memories by temporal ordering (recent first)
    fn search_by_temporal(&self, query: &str, limit: usize) -> MemoryStream;

    /// Search memories by pattern matching
    fn search_by_pattern(&self, query: &str, limit: usize) -> MemoryStream;

    /// Update quantum signature for a memory node
    /// Persists the CognitiveState's quantum signature (denormalized cache)
    fn update_quantum_signature(
        &self,
        memory_id: &str,
        cognitive_state: &CognitiveState,
    ) -> PendingQuantumUpdate;

    /// Get quantum signature for a memory node
    /// Returns None if memory has no quantum signature
    fn get_quantum_signature(&self, memory_id: &str) -> PendingQuantumSignature;

    /// Create entanglement edge using RELATE (SurrealDB graph optimized)
    /// This creates an edge in the entangled RELATION table with graph pointers
    fn create_entanglement_edge(
        &self,
        source_id: &str,
        target_id: &str,
        strength: f32,
        bond_type: EntanglementType,
    ) -> PendingEntanglementEdge;

    /// Get memories entangled with a given memory (graph traversal)
    /// Uses SurrealDB's ->entangled syntax with pointer-based O(1) lookups
    /// Filters by minimum strength threshold
    fn get_entangled_memories(&self, memory_id: &str, min_strength: f32) -> MemoryStream;

    /// Get entangled memories filtered by bond type (graph + index)
    /// Types: Semantic, Bell, BellPair, Temporal, Causal, etc.
    /// Uses both graph pointers and bond_type index
    fn get_entangled_by_type(&self, memory_id: &str, bond_type: EntanglementType) -> MemoryStream;

    /// Traverse entanglement graph to specified depth
    /// Returns all memories reachable within max_depth hops
    /// Uses recursive graph traversal with SurrealDB range syntax
    fn traverse_entanglement_graph(&self, memory_id: &str, max_depth: usize) -> MemoryStream;

    /// Expand a set of memory IDs via entanglement
    /// For each input memory, finds entangled neighbors
    /// Returns union of all entangled memories (deduplicated)
    fn expand_via_entanglement(&self, memory_ids: Vec<String>, min_strength: f32) -> MemoryStream;
}

/// SurrealDB implementation of the memory manager
#[derive(Debug, Clone)]
pub struct SurrealDBMemoryManager {
    db: Surreal<Any>,
    embedding_model: Option<TextEmbeddingModel>,
}

/// Data structure for memory export containing nodes and relationships
#[derive(Debug, Serialize, Deserialize)]
struct ExportData {
    nodes: Vec<MemoryNode>,
    relationships: Vec<MemoryRelationship>,
}

// Implementation for SurrealDBMemoryManager
impl SurrealDBMemoryManager {
    /// Create a new SurrealDB memory manager with a custom embedding model
    ///
    /// This factory method initializes the manager with the provided embedding model
    /// for auto-generating embeddings on memory creation and text search.
    ///
    /// # Arguments
    /// * `db` - Connected SurrealDB instance
    /// * `embedding_model` - Custom embedding model to use for vector operations
    pub async fn with_embedding_model(
        db: Surreal<Any>,
        embedding_model: TextEmbeddingModel,
    ) -> Result<Self> {
        Ok(Self {
            db,
            embedding_model: Some(embedding_model),
        })
    }

    /// Create a new SurrealDB memory manager without an embedding model
    pub async fn new(db: Surreal<Any>) -> Result<Self> {
        Ok(Self {
            db,
            embedding_model: None,
        })
    }

    /// Create a new SurrealDB memory manager with default embedding model from registry
    ///
    /// Convenience method that uses Stella 1024 as the default embedding model.
    /// This is the recommended way to create a memory manager with embeddings.
    ///
    /// # Arguments
    /// * `db` - Connected SurrealDB instance
    ///
    /// # Returns
    /// Memory manager with default Stella embedding model from registry
    pub async fn with_embeddings(db: Surreal<Any>) -> Result<Self> {
        use crate::capability::registry;
        let default_model: TextEmbeddingModel = registry::get("dunzhang/stella_en_400M_v5")
            .ok_or_else(|| {
                Error::Config("Default Stella embedding model not found in registry".into())
            })?;
        Self::with_embedding_model(db, default_model).await
    }

    /// Get a reference to the database connection
    pub fn database(&self) -> &Surreal<Any> {
        &self.db
    }

    /// Initialize the manager (create tables, indexes, etc.)
    pub async fn initialize(&self) -> Result<()> {
        // Create memory table - schemaless for flexibility
        self.db
            .query("DEFINE TABLE memory SCHEMALESS")
            .await
            .map_err(|e| Error::Database(format!("{:?}", e)))?;

        // Create relationship table - schemaless for flexibility
        self.db
            .query("DEFINE TABLE memory_relationship SCHEMALESS")
            .await
            .map_err(|e| Error::Database(format!("{:?}", e)))?;

        // Create indexes for efficient querying
        self.db
            .query("DEFINE INDEX memory_type_idx ON TABLE memory COLUMNS memory_type")
            .await
            .map_err(|e| Error::Database(format!("{:?}", e)))?;

        // Define content_hash as required field
        self.db
            .query("DEFINE FIELD content_hash ON TABLE memory TYPE number")
            .await
            .map_err(|e| Error::Database(format!("{:?}", e)))?;

        // Create index for content hash deduplication
        self.db
            .query("DEFINE INDEX memory_content_hash_idx ON TABLE memory COLUMNS content_hash")
            .await
            .map_err(|e| Error::Database(format!("{:?}", e)))?;

        self.db.query("DEFINE INDEX memory_relationship_source_idx ON TABLE memory_relationship COLUMNS source_id")
            .await
            .map_err(|e| Error::Database(format!("{:?}", e)))?;

        self.db.query("DEFINE INDEX memory_relationship_target_idx ON TABLE memory_relationship COLUMNS target_id")
            .await
            .map_err(|e| Error::Database(format!("{:?}", e)))?;

        Ok(())
    }

    /// Convert a database schema to a memory node
    fn from_schema(schema: MemoryNodeSchema) -> MemoryNode {
        let id = schema.id.key().to_string();
        let embedding = schema.metadata.embedding;
        let content = crate::memory::core::primitives::types::MemoryContent::new(&schema.content);
        let content_hash = schema.content_hash;

        let mut metadata = MemoryMetadata::new();
        metadata.created_at = schema.metadata.created_at;
        metadata.last_accessed_at = Some(schema.metadata.last_accessed_at);
        metadata.importance = schema.metadata.importance;
        metadata.embedding = embedding.clone();
        metadata.custom = schema.metadata.custom;

        MemoryNode {
            id,
            content,
            content_hash,
            memory_type: schema.memory_type,
            created_at: schema.metadata.created_at,
            updated_at: schema.metadata.last_accessed_at,
            embedding,
            evaluation_status: crate::memory::monitoring::operations::OperationStatus::Pending,
            metadata,
            relevance_score: None, // Will be populated during search operations
        }
    }

    /// Execute a raw query against the database
    pub async fn execute_query(
        &self,
        query: &str,
    ) -> std::result::Result<surrealdb::Response, Error> {
        self.db
            .query(query)
            .await
            .map_err(|e| Error::Database(format!("{:?}", e)))
    }

    /// Health check method to verify database connectivity
    pub async fn health_check(&self) -> Result<()> {
        // Perform a simple query to check if the database is responsive
        match self.db.query("SELECT 1 as health").await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Database(format!("{:?}", e))),
        }
    }

    /// Run all pending schema migrations
    ///
    /// Executes V1, V2, V3, and any future migrations that haven't been applied yet.
    /// Safe to call multiple times (migrations are idempotent).
    ///
    /// # Returns
    /// Ok(()) if all migrations succeeded, Err if any migration failed
    pub async fn run_migrations(&self) -> Result<()> {
        log::info!("Initializing migration manager...");

        // Create migration manager with shared database connection
        let db_arc = Arc::new(self.db.clone());
        let mut manager = MigrationManager::new(db_arc)
            .await
            .map_err(|e| Error::Other(format!("Failed to create migration manager: {}", e)))?;

        // Add all builtin migrations
        for migration in BuiltinMigrations::all() {
            manager.add_migration(migration);
        }

        log::info!("Running schema migrations...");

        // Run migrations (now async, not returning PendingMigration)
        manager
            .migrate()
            .await
            .map_err(|e| Error::Other(format!("Migration execution failed: {}", e)))?;

        log::info!("Schema migrations completed successfully");

        Ok(())
    }

    /// Export all memories to a file in specified format
    ///
    /// Supports JSON and CSV formats. Binary format is not supported as it requires
    /// bincode::Encode trait which MemoryNode and MemoryRelationship don't implement.
    ///
    /// # Arguments
    /// * `path` - Path to export file
    /// * `format` - Export format (Json or Csv; Binary will return error)
    ///
    /// # Returns
    /// Total count of exported items (nodes + relationships)
    ///
    /// # Errors
    /// Returns Error::Database if database queries fail
    /// Returns Error::Migration if export operation fails
    pub async fn export_memories(&self, path: &Path, format: ExportFormat) -> Result<usize> {
        log::info!("Exporting memories to {:?} in {:?} format", path, format);

        // Query all memory nodes from 'memory' table
        let nodes_query = "SELECT * FROM memory";
        let mut nodes_response = self
            .db
            .query(nodes_query)
            .await
            .map_err(|e| Error::Database(format!("Failed to query memory nodes: {}", e)))?;

        // Get as schema first, then convert using established helper
        let node_schemas: Vec<MemoryNodeSchema> = nodes_response
            .take(0)
            .map_err(|e| Error::Database(format!("Failed to parse memory nodes: {}", e)))?;

        // Use from_schema for consistent conversion (see line 328)
        let nodes: Vec<MemoryNode> = node_schemas
            .into_iter()
            .map(SurrealDBMemoryManager::from_schema)
            .collect();

        // Query all relationships from 'memory_relationship' table
        let rels_query = "SELECT * FROM memory_relationship";
        let mut rels_response = self
            .db
            .query(rels_query)
            .await
            .map_err(|e| Error::Database(format!("Failed to query relationships: {}", e)))?;

        // Get as Relationship first, then convert properly
        let relationship_schemas: Vec<Relationship> = rels_response
            .take(0)
            .map_err(|e| Error::Database(format!("Failed to parse relationships: {}", e)))?;

        // Convert with full field preservation
        let relationships: Vec<MemoryRelationship> = relationship_schemas
            .into_iter()
            .map(|schema| MemoryRelationship {
                id: schema.id.key().to_string(),
                source_id: schema.source_id,
                target_id: schema.target_id,
                relationship_type: schema.relationship_type,
                metadata: Some(schema.metadata),
                created_at: Some(schema.created_at),
                updated_at: Some(schema.updated_at),
                strength: Some(schema.strength),
            })
            .collect();

        // Create export data structure
        let export_data = ExportData {
            nodes: nodes.clone(),
            relationships: relationships.clone(),
        };

        // Convert to slice for DataExporter (it expects &[T])
        let export_slice = std::slice::from_ref(&export_data);

        // Use DataExporter to write to file
        let exporter = DataExporter::new(format);
        exporter
            .export_to_file(export_slice, path)
            .await
            .map_err(|e| Error::Migration(format!("Export failed: {}", e)))?;

        let total = nodes.len() + relationships.len();
        log::info!("Successfully exported {} items to {:?}", total, path);

        Ok(total)
    }

    /// Import memories from a file in specified format
    ///
    /// Imports both memory nodes and relationships from a file created by export_memories().
    /// The file must contain an ExportData structure with nodes and relationships arrays.
    ///
    /// # Arguments
    /// * `path` - Path to import file
    /// * `format` - Import format (Json or Csv)
    ///
    /// # Returns
    /// Total count of imported items (nodes + relationships)
    ///
    /// # Errors
    /// Returns Error::Migration if import or deserialization fails
    /// Returns Error::Database if database insertion fails
    pub async fn import_memories(&self, path: &Path, format: ImportFormat) -> Result<usize> {
        log::info!("Importing memories from {:?} in {:?} format", path, format);

        // Use DataImporter to read from file
        let importer = DataImporter::new();
        let import_vec: Vec<ExportData> = match format {
            ImportFormat::Json => importer
                .import_json(path)
                .await
                .map_err(|e| Error::Migration(format!("JSON import failed: {}", e)))?,
            ImportFormat::Csv => importer
                .import_csv(path)
                .await
                .map_err(|e| Error::Migration(format!("CSV import failed: {}", e)))?,
        };

        // Extract the single ExportData element (export creates array of 1 element)
        let import_data = import_vec
            .into_iter()
            .next()
            .ok_or_else(|| Error::Migration("Import file is empty".to_string()))?;

        let mut inserted_count = 0;

        // Insert memory nodes
        for node in import_data.nodes {
            let create_content = MemoryNodeCreateContent::from(&node);
            let result: Option<MemoryNodeSchema> = self
                .db
                .create(("memory", node.id.as_str()))
                .content(create_content)
                .await
                .map_err(|e| {
                    Error::Database(format!("Failed to insert memory node {}: {}", node.id, e))
                })?;

            match result {
                Some(_) => inserted_count += 1,
                None => {
                    return Err(Error::NotFound(format!(
                        "Failed to create memory node {}",
                        node.id
                    )));
                }
            }
        }

        // Insert relationships
        for rel in import_data.relationships {
            let create_content = RelationshipCreateContent::from(&rel);
            let result: Option<Relationship> = self
                .db
                .create(("memory_relationship", rel.id.as_str()))
                .content(create_content)
                .await
                .map_err(|e| {
                    Error::Database(format!("Failed to insert relationship {}: {}", rel.id, e))
                })?;

            match result {
                Some(_) => inserted_count += 1,
                None => {
                    return Err(Error::NotFound(format!(
                        "Failed to create relationship {}",
                        rel.id
                    )));
                }
            }
        }

        log::info!(
            "Successfully imported {} items from {:?}",
            inserted_count,
            path
        );

        Ok(inserted_count)
    }
}

impl MemoryManager for SurrealDBMemoryManager {
    fn create_memory(&self, mut memory: MemoryNode) -> PendingMemory {
        let db = self.db.clone();
        let embedding_model = self.embedding_model.clone();

        let (tx, rx) = tokio::sync::oneshot::channel();

        tokio::spawn(async move {
            // Auto-generate embedding if missing and model is available
            if memory.embedding.is_none()
                && let Some(ref model) = embedding_model
            {
                // Extract text content for embedding
                let content_text = memory.content.text.clone();

                // Generate embedding asynchronously
                match model.embed(&content_text, Some("search".to_string())).await {
                    Ok(embedding_vec) => {
                        memory.embedding = Some(embedding_vec);
                        // Also update metadata.embedding for consistency
                        memory.metadata.embedding = memory.embedding.clone();
                    }
                    Err(e) => {
                        // Log but don't fail - memory can exist without embedding
                        log::warn!("Failed to generate embedding: {:?}", e);
                    }
                }
            }

            // Continue with existing storage logic
            let memory_content = MemoryNodeCreateContent::from(&memory);

            // Create the memory in SurrealDB with specific ID
            // The embedding is stored directly in the metadata field
            let created: Option<MemoryNodeSchema> = match db
                .create(("memory", memory.id.as_str()))
                .content(memory_content)
                .await
            {
                Ok(created) => created,
                Err(e) => {
                    let _ = tx.send(Err(Error::Database(format!("{:?}", e))));
                    return;
                }
            };

            let result = match created {
                Some(schema) => Ok(SurrealDBMemoryManager::from_schema(schema)),
                None => Err(Error::NotFound("Failed to create memory".to_string())),
            };

            let _ = tx.send(result);
        });

        PendingMemory::new(rx)
    }

    fn get_memory(&self, id: &str) -> MemoryQuery {
        let db = self.db.clone();
        let id = id.to_string();

        let (tx, rx) = tokio::sync::oneshot::channel();

        tokio::spawn(async move {
            let result = match db.select::<Option<MemoryNodeSchema>>(("memory", id)).await {
                Ok(result) => Ok(result.map(SurrealDBMemoryManager::from_schema)),
                Err(e) => Err(Error::Database(format!("{:?}", e))),
            };

            let _ = tx.send(result);
        });

        MemoryQuery::new(rx)
    }

    fn update_memory(&self, memory: MemoryNode) -> PendingMemory {
        let db = self.db.clone();
        let id = memory.id.clone();
        let memory_content = MemoryNodeCreateContent::from(&memory);

        let (tx, rx) = tokio::sync::oneshot::channel();

        tokio::spawn(async move {
            // Update the memory in SurrealDB
            // The embedding is updated directly in the metadata field
            let updated: Option<MemoryNodeSchema> =
                match db.update(("memory", &id)).content(memory_content).await {
                    Ok(updated) => updated,
                    Err(e) => {
                        let _ = tx.send(Err(Error::Database(format!("{:?}", e))));
                        return;
                    }
                };

            let result = match updated {
                Some(schema) => Ok(SurrealDBMemoryManager::from_schema(schema)),
                None => Err(Error::NotFound(format!("Memory with id {id} not found"))),
            };

            let _ = tx.send(result);
        });

        PendingMemory::new(rx)
    }

    fn delete_memory(&self, id: &str) -> PendingDeletion {
        let db = self.db.clone();
        let id_str = id.to_string();

        let (tx, rx) = tokio::sync::oneshot::channel();

        tokio::spawn(async move {
            // Delete from SurrealDB
            let result = match db
                .delete::<Option<MemoryNodeSchema>>(("memory", &id_str))
                .await
            {
                Ok(result) => Ok(result.is_some()),
                Err(e) => Err(Error::Database(format!("{:?}", e))),
            };

            let _ = tx.send(result);
        });

        PendingDeletion::new(rx)
    }

    fn create_relationship(&self, relationship: MemoryRelationship) -> PendingRelationship {
        let db = self.db.clone();

        let content = RelationshipCreateContent::from(&relationship);

        let (tx, rx) = tokio::sync::oneshot::channel();

        tokio::spawn(async move {
            let created: Option<Relationship> = match db
                .create(("memory_relationship", relationship.id.as_str()))
                .content(content)
                .await
            {
                Ok(created) => created,
                Err(e) => {
                    let _ = tx.send(Err(Error::Database(format!("{:?}", e))));
                    return;
                }
            };

            let result = match created {
                Some(schema) => Ok(MemoryRelationship {
                    id: schema.id.key().to_string(),
                    source_id: schema.source_id,
                    target_id: schema.target_id,
                    relationship_type: schema.relationship_type,
                    metadata: Some(schema.metadata),
                    created_at: Some(schema.created_at),
                    updated_at: Some(schema.updated_at),
                    strength: Some(schema.strength),
                }),
                None => Err(Error::NotFound("Failed to create relationship".to_string())),
            };

            let _ = tx.send(result);
        });

        PendingRelationship::new(rx)
    }

    fn get_relationships(&self, memory_id: &str) -> RelationshipStream {
        let db = self.db.clone();
        let memory_id = memory_id.to_string();

        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            let sql_query = "SELECT * FROM memory_relationship WHERE source_id = $memory_id OR target_id = $memory_id";

            match db.query(sql_query).bind(("memory_id", memory_id)).await {
                Ok(mut response) => {
                    let results: Vec<Relationship> = response.take(0).unwrap_or_default();

                    for schema in results {
                        let relationship = MemoryRelationship {
                            id: schema.id.key().to_string(),
                            source_id: schema.source_id,
                            target_id: schema.target_id,
                            relationship_type: schema.relationship_type,
                            metadata: Some(schema.metadata),
                            created_at: Some(schema.created_at),
                            updated_at: Some(schema.updated_at),
                            strength: Some(schema.strength),
                        };

                        if tx.send(Ok(relationship)).await.is_err() {
                            break;
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(Error::Database(format!("{:?}", e)))).await;
                }
            }
        });

        RelationshipStream::new(rx)
    }

    fn delete_relationship(&self, id: &str) -> PendingDeletion {
        let db = self.db.clone();
        let id_str = id.to_string();

        let (tx, rx) = tokio::sync::oneshot::channel();

        tokio::spawn(async move {
            let result = match db
                .delete::<Option<Relationship>>(("memory_relationship", &id_str))
                .await
            {
                Ok(result) => Ok(result.is_some()),
                Err(e) => Err(Error::Database(format!("{:?}", e))),
            };

            let _ = tx.send(result);
        });

        PendingDeletion::new(rx)
    }

    fn query_by_type(&self, memory_type: MemoryTypeEnum) -> MemoryStream {
        let db = self.db.clone();
        // Use the serialized format which is capitalized
        let memory_type_str = match &memory_type {
            MemoryTypeEnum::Episodic => "Episodic".to_string(),
            MemoryTypeEnum::Semantic => "Semantic".to_string(),
            MemoryTypeEnum::Procedural => "Procedural".to_string(),
            MemoryTypeEnum::Working => "Working".to_string(),
            MemoryTypeEnum::LongTerm => "LongTerm".to_string(),
        };

        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            let sql_query = "SELECT * FROM memory WHERE memory_type = $memory_type";

            match db
                .query(sql_query)
                .bind(("memory_type", memory_type_str))
                .await
            {
                Ok(mut response) => {
                    let results: Vec<MemoryNodeSchema> = response.take(0).unwrap_or_default();

                    for schema in results {
                        let memory = SurrealDBMemoryManager::from_schema(schema);
                        if tx.send(Ok(memory)).await.is_err() {
                            break;
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(Error::Database(format!("{:?}", e)))).await;
                }
            }
        });

        MemoryStream::new(rx)
    }

    fn search_by_content(&self, query: &str) -> MemoryStream {
        let db = self.db.clone();
        let query = query.to_string();

        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            let sql_query = "SELECT * FROM memory WHERE content CONTAINS $query";

            match db.query(sql_query).bind(("query", query)).await {
                Ok(mut response) => {
                    let results: Vec<MemoryNodeSchema> = response.take(0).unwrap_or_default();

                    for schema in results {
                        let memory = SurrealDBMemoryManager::from_schema(schema);
                        if tx.send(Ok(memory)).await.is_err() {
                            break;
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(Error::Database(format!("{:?}", e)))).await;
                }
            }
        });

        MemoryStream::new(rx)
    }

    fn search_by_vector(&self, vector: Vec<f32>, limit: usize) -> MemoryStream {
        let db = self.db.clone();

        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            // Convert vector to JSON array format for SurrealDB
            let vector_json = serde_json::to_string(&vector).unwrap_or_else(|_| "[]".to_string());

            // Use SurrealDB's native vector similarity search
            let sql_query = format!(
                "SELECT *, vector::similarity::cosine(metadata.embedding, {vector_json}) AS score
                FROM memory
                WHERE metadata.embedding != NULL
                ORDER BY score DESC
                LIMIT {limit};"
            );

            match db.query(&sql_query).await {
                Ok(mut response) => {
                    // Parse as generic JSON to handle the dynamic score field
                    let results: Vec<serde_json::Value> = response.take(0).unwrap_or_default();

                    for result in results {
                        // Extract the score from the result
                        let score = result
                            .get("score")
                            .and_then(|s| s.as_f64())
                            .map(|s| s as f32);

                        // Parse the rest as a MemoryNodeSchema
                        if let Ok(schema) = serde_json::from_value::<MemoryNodeSchema>(result) {
                            let mut memory = SurrealDBMemoryManager::from_schema(schema);
                            // Set the actual cosine similarity score
                            memory.relevance_score = score;

                            if tx.send(Ok(memory)).await.is_err() {
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(Error::Database(format!("{:?}", e)))).await;
                }
            }
        });

        MemoryStream::new(rx)
    }

    fn search_by_temporal(&self, query: &str, limit: usize) -> MemoryStream {
        let db = self.db.clone();
        let query = query.to_string();
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            // Query memories ordered by created_at descending (recent first)
            let sql = format!(
                "SELECT * FROM memory
                 WHERE content.text CONTAINS $query
                 ORDER BY created_at DESC
                 LIMIT {limit}"
            );

            match db.query(&sql).bind(("query", query)).await {
                Ok(mut response) => {
                    let results: Vec<MemoryNodeSchema> = response.take(0).unwrap_or_default();
                    for schema in results {
                        let memory = SurrealDBMemoryManager::from_schema(schema);
                        if tx.send(Ok(memory)).await.is_err() {
                            break;
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(Error::Database(format!("{:?}", e)))).await;
                }
            }
        });

        MemoryStream::new(rx)
    }

    fn search_by_pattern(&self, query: &str, limit: usize) -> MemoryStream {
        let db = self.db.clone();
        let query = query.to_string();
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            // Pattern search using fuzzy matching and wildcards
            // Uses SurrealDB's text search operators for pattern recognition
            let pattern = format!("%{}%", query); // Wildcard pattern
            let sql = format!(
                "SELECT * FROM memory
                 WHERE content.text LIKE $pattern
                 OR metadata.keywords CONTAINS $query
                 ORDER BY metadata.importance DESC
                 LIMIT {limit}"
            );

            match db
                .query(&sql)
                .bind(("pattern", pattern))
                .bind(("query", query))
                .await
            {
                Ok(mut response) => {
                    let results: Vec<MemoryNodeSchema> = response.take(0).unwrap_or_default();
                    for schema in results {
                        let memory = SurrealDBMemoryManager::from_schema(schema);
                        if tx.send(Ok(memory)).await.is_err() {
                            break;
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(Error::Database(format!("{:?}", e)))).await;
                }
            }
        });

        MemoryStream::new(rx)
    }

    fn create_entanglement_edge(
        &self,
        source_id: &str,
        target_id: &str,
        strength: f32,
        bond_type: EntanglementType,
    ) -> PendingEntanglementEdge {
        let db = self.db.clone();
        let source_id = source_id.to_string();
        let target_id = target_id.to_string();
        let bond_type_str = format!("{:?}", bond_type);

        let (tx, rx) = tokio::sync::oneshot::channel();

        tokio::spawn(async move {
            // Use RELATE statement for graph-optimized edge creation
            // This creates bidirectional IN/OUT pointers automatically
            // SurrealDB stores 4 pointers: source->OUT, edge->IN, edge->OUT, target->IN
            let sql = "
                RELATE $source->entangled->$target
                SET
                    strength = $strength,
                    bond_type = $bond_type,
                    created_at = time::now()
            ";

            let result = match db
                .query(sql)
                .bind(("source", format!("memory:{}", source_id)))
                .bind(("target", format!("memory:{}", target_id)))
                .bind(("strength", strength))
                .bind(("bond_type", bond_type_str.clone()))
                .await
            {
                Ok(_) => {
                    log::debug!(
                        "Created {:?} entanglement edge: {} -> {} (strength: {:.4})",
                        bond_type_str,
                        source_id,
                        target_id,
                        strength
                    );
                    Ok(())
                }
                Err(e) => {
                    log::error!(
                        "Failed to create entanglement edge {} -> {}: {:?}",
                        source_id,
                        target_id,
                        e
                    );
                    Err(Error::Database(format!("RELATE failed: {:?}", e)))
                }
            };

            let _ = tx.send(result);
        });

        PendingEntanglementEdge::new(rx)
    }

    fn get_entangled_memories(&self, memory_id: &str, min_strength: f32) -> MemoryStream {
        let db = self.db.clone();
        let memory_id = memory_id.to_string();
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            // OPTIMIZED: Use SurrealDB graph syntax with inline filtering
            // ->entangled uses graph pointers (O(1) lookup per edge)
            // [WHERE strength >= $min] filters during traversal (uses strength_idx)
            // .out.* fetches target memory records
            let sql = "
                SELECT ->entangled[WHERE strength >= $min_strength].out.*
                FROM $memory_id
            ";

            match db
                .query(sql)
                .bind(("memory_id", format!("memory:{}", memory_id)))
                .bind(("min_strength", min_strength))
                .await
            {
                Ok(mut response) => {
                    // SurrealDB returns array of target memories directly
                    let results: Vec<MemoryNodeSchema> = response.take(0).unwrap_or_default();

                    log::debug!(
                        "Retrieved {} entangled memories for {} (min_strength: {}) via graph pointers",
                        results.len(),
                        memory_id,
                        min_strength
                    );

                    for schema in results {
                        let memory = SurrealDBMemoryManager::from_schema(schema);
                        if tx.send(Ok(memory)).await.is_err() {
                            break;
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to retrieve entangled memories for {} (graph query): {:?}",
                        memory_id,
                        e
                    );
                    let _ = tx.send(Err(Error::Database(format!("{:?}", e)))).await;
                }
            }
        });

        MemoryStream::new(rx)
    }

    fn get_entangled_by_type(&self, memory_id: &str, bond_type: EntanglementType) -> MemoryStream {
        let db = self.db.clone();
        let memory_id = memory_id.to_string();
        let type_str = format!("{:?}", bond_type);
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            // OPTIMIZED: Graph traversal + index lookup
            // ->entangled uses graph pointers
            // [WHERE bond_type = $type] uses bond_type_idx for fast equality check
            // .out.* fetches target memory records
            let sql = "
                SELECT ->entangled[WHERE bond_type = $bond_type].out.*
                FROM $memory_id
            ";

            match db
                .query(sql)
                .bind(("memory_id", format!("memory:{}", memory_id)))
                .bind(("bond_type", type_str.clone()))
                .await
            {
                Ok(mut response) => {
                    let results: Vec<MemoryNodeSchema> = response.take(0).unwrap_or_default();

                    log::debug!(
                        "Retrieved {} {:?} entangled memories for {} via graph + index",
                        results.len(),
                        bond_type,
                        memory_id
                    );

                    for schema in results {
                        let memory = SurrealDBMemoryManager::from_schema(schema);
                        if tx.send(Ok(memory)).await.is_err() {
                            break;
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to retrieve {:?} entangled memories for {}: {:?}",
                        bond_type,
                        memory_id,
                        e
                    );
                    let _ = tx.send(Err(Error::Database(format!("{:?}", e)))).await;
                }
            }
        });

        MemoryStream::new(rx)
    }

    fn traverse_entanglement_graph(&self, memory_id: &str, max_depth: usize) -> MemoryStream {
        let db = self.db.clone();
        let memory_id = memory_id.to_string();
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            // SurrealDB multi-hop graph traversal using 4-pointer structure
            // Since SurrealDB doesn't support ->edge{1..N} syntax for graph traversal,
            // we build UNION query with one subquery per depth level
            //
            // Each hop uses ->entangled.out.* pattern (confirmed from SDK tests):
            // - Depth 1: ->entangled.out.*
            // - Depth 2: ->entangled->memory->entangled.out.*
            // - Depth N: (repeat chain N times)
            //
            // Performance: O(E^D) where E = avg edges/node, D = depth
            // DISTINCT eliminates duplicates across all depths: O(N log N)

            // Validate depth within SurrealDB limits (max 256)
            let safe_depth = max_depth.clamp(1, 256);

            // Build UNION query for each depth level
            let mut depth_queries = Vec::new();

            for depth in 1..=safe_depth {
                // Build chain: ->entangled->memory->entangled->memory...->entangled.out.*
                // Depth 1: ->entangled.out.*
                // Depth 2: ->entangled->memory->entangled.out.*
                // Depth 3: ->entangled->memory->entangled->memory->entangled.out.*
                let mut chain = String::new();
                for i in 0..depth {
                    if i > 0 {
                        chain.push_str("->memory");
                    }
                    chain.push_str("->entangled");
                }
                chain.push_str(".out.*");

                depth_queries.push(format!("SELECT {} FROM $memory_id", chain));
            }

            let sql = format!(
                "SELECT DISTINCT * FROM ({}) ",
                depth_queries.join(" UNION ")
            );

            match db
                .query(&sql)
                .bind(("memory_id", format!("memory:{}", memory_id)))
                .await
            {
                Ok(mut response) => {
                    let results: Vec<MemoryNodeSchema> = response.take(0).unwrap_or_default();

                    log::info!(
                        "Traversed entanglement graph from {} (depth {}): {} memories found",
                        memory_id,
                        safe_depth,
                        results.len()
                    );

                    for schema in results {
                        let memory = SurrealDBMemoryManager::from_schema(schema);
                        if tx.send(Ok(memory)).await.is_err() {
                            break;
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to traverse entanglement graph from {} (depth {}): {:?}",
                        memory_id,
                        safe_depth,
                        e
                    );
                    let _ = tx.send(Err(Error::Database(format!("{:?}", e)))).await;
                }
            }
        });

        MemoryStream::new(rx)
    }

    fn expand_via_entanglement(&self, memory_ids: Vec<String>, min_strength: f32) -> MemoryStream {
        let db = self.db.clone();
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            // Handle empty input gracefully
            if memory_ids.is_empty() {
                return; // Empty stream
            }

            // Convert memory IDs to SurrealDB record format
            let record_ids: Vec<String> = memory_ids
                .iter()
                .map(|id| format!("memory:{}", id))
                .collect();

            // Parallel expansion using graph pointers (based on working QENT_4 pattern)
            // Uses proven ->entangled[WHERE ...].out.* syntax from get_entangled_memories
            //
            // For each seed memory:
            // 1. Fetch OUT pointers (O(1) per seed via graph keys)
            // 2. Get edge IDs for all seeds
            // 3. Filter edges where strength >= min_strength (uses strength_idx: O(log E))
            // 4. Fetch .out.* to get full target records
            // 5. DISTINCT deduplicates overlapping neighbors (O(N log N))
            //
            // Performance: O(S * E * log E) where S = seeds, E = avg edges/seed
            // Without index: O(S * E²) - 100-1000x slower

            let sql = format!(
                "
                SELECT DISTINCT ->entangled[WHERE strength >= $min_strength].out.*
                FROM {}
            ",
                record_ids.join(", ")
            );

            match db.query(&sql).bind(("min_strength", min_strength)).await {
                Ok(mut response) => {
                    let results: Vec<MemoryNodeSchema> = response.take(0).unwrap_or_default();

                    log::info!(
                        "Expanded {} seed memories via entanglement (min_strength {}): {} total memories",
                        memory_ids.len(),
                        min_strength,
                        results.len()
                    );

                    for schema in results {
                        let memory = SurrealDBMemoryManager::from_schema(schema);
                        if tx.send(Ok(memory)).await.is_err() {
                            break;
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to expand via entanglement (seeds: {}): {:?}",
                        memory_ids.len(),
                        e
                    );
                    let _ = tx.send(Err(Error::Database(format!("{:?}", e)))).await;
                }
            }
        });

        MemoryStream::new(rx)
    }

    fn update_quantum_signature(
        &self,
        memory_id: &str,
        cognitive_state: &CognitiveState,
    ) -> PendingQuantumUpdate {
        let db = self.db.clone();
        let memory_id = memory_id.to_string();

        // Convert CognitiveState to schema
        let signature_schema = QuantumSignatureSchema::from_cognitive_state(cognitive_state);

        let (tx, rx) = tokio::sync::oneshot::channel();

        tokio::spawn(async move {
            // Update memory record with quantum signature (denormalized cache)
            // Primary edges are in entangled RELATION table
            let sql = "UPDATE memory SET quantum_signature = $signature WHERE id = $memory_id";

            let result = match db
                .query(sql)
                .bind(("memory_id", format!("memory:{}", memory_id)))
                .bind(("signature", signature_schema.clone()))
                .await
            {
                Ok(_) => {
                    log::debug!(
                        "Persisted quantum signature for {} with {} bonds",
                        memory_id,
                        signature_schema.entanglement_bonds.len()
                    );
                    Ok(())
                }
                Err(e) => {
                    log::error!(
                        "Failed to update quantum signature for {}: {:?}",
                        memory_id,
                        e
                    );
                    Err(Error::Database(format!(
                        "Quantum signature update failed: {:?}",
                        e
                    )))
                }
            };

            let _ = tx.send(result);
        });

        PendingQuantumUpdate::new(rx)
    }

    fn get_quantum_signature(&self, memory_id: &str) -> PendingQuantumSignature {
        let db = self.db.clone();
        let memory_id = memory_id.to_string();

        let (tx, rx) = tokio::sync::oneshot::channel();

        tokio::spawn(async move {
            // Query for quantum signature field (denormalized cache)
            // For live edges, query entangled RELATION table separately
            let sql = "SELECT quantum_signature FROM memory WHERE id = $memory_id";

            let result = match db
                .query(sql)
                .bind(("memory_id", format!("memory:{}", memory_id)))
                .await
            {
                Ok(mut response) => {
                    // Extract quantum_signature field
                    let signatures: Vec<Option<QuantumSignatureSchema>> =
                        response.take(0).unwrap_or_default();

                    if let Some(Some(schema)) = signatures.first() {
                        // Convert schema back to CognitiveState
                        match schema.to_cognitive_state() {
                            Ok(state) => {
                                log::debug!(
                                    "Loaded quantum signature for {} with {} bonds (cached)",
                                    memory_id,
                                    state.quantum_entanglement_bond_count()
                                );
                                Ok(Some(state))
                            }
                            Err(e) => {
                                log::error!(
                                    "Failed to deserialize quantum signature for {}: {}",
                                    memory_id,
                                    e
                                );
                                Err(Error::Other(format!("Deserialization failed: {}", e)))
                            }
                        }
                    } else {
                        // Memory has no quantum signature
                        Ok(None)
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to retrieve quantum signature for {}: {:?}",
                        memory_id,
                        e
                    );
                    Err(Error::Database(format!("Query failed: {:?}", e)))
                }
            };

            let _ = tx.send(result);
        });

        PendingQuantumSignature::new(rx)
    }
}

// Additional methods for SurrealDBMemoryManager
impl SurrealDBMemoryManager {
    /// # Hybrid Search with Entanglement Expansion
    ///
    /// Combines vector similarity search with graph-based context expansion.
    ///
    /// ## How It Works
    ///
    /// 1. **Vector Search**: Find top-N most similar memories by cosine similarity
    /// 2. **Graph Expansion**: Traverse entanglement graph from seed memories
    /// 3. **Merge Results**: Return union of vector matches + entangled neighbors
    ///
    /// ## Parameters
    ///
    /// - `vector`: Query embedding (f32 vector)
    /// - `limit`: Maximum total results to return
    /// - `expand_depth`: Graph traversal depth
    ///   - `0`: No expansion (pure vector search)
    ///   - `1`: Include direct entangled neighbors
    ///   - `2`: Include neighbors of neighbors
    ///   - `3+`: Deeper traversal (use with caution)
    ///
    /// ## Returns
    ///
    /// `MemoryStream` containing:
    /// - Top vector similarity matches (seed memories)
    /// - Memories entangled with seeds (within `expand_depth` hops)
    /// - Deduplicated (DISTINCT)
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// // Search with depth-2 entanglement expansion
    /// let results = manager.search_with_entanglement(query_vec, 20, 2);
    /// let memories: Vec<_> = results.collect().await;
    ///
    /// // Results contain:
    /// // - 10 best vector matches (seeds)
    /// // - ~10-30 entangled memories (neighbors + neighbors-of-neighbors)
    /// ```
    pub fn search_with_entanglement(
        &self,
        vector: Vec<f32>,
        limit: usize,
        expand_depth: usize,
    ) -> MemoryStream {
        let db = self.db.clone();
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            // SURREALDB HYBRID SEARCH OPTIMIZATION
            // ====================================
            // Uses single-query CTE approach for optimal performance
            //
            // Phase 1: MTREE Vector Search (from QENT_1 V2 migration)
            //   - Uses memory_embedding_idx MTREE DIMENSION 384 DIST COSINE TYPE F32
            //   - O(log N) index lookup vs O(N) table scan
            //   - Returns top-K candidates by cosine similarity
            //
            // Phase 2: Graph Expansion (from edges.rs 4-pointer structure)
            //   - Follows OUT pointers from seed memories: O(1) per seed
            //   - Each hop: Fetch OUT pointers -> Get edge IDs -> Fetch target IDs
            //   - Uses strength_idx and type_idx for filtering: O(log E) per edge
            //   - Depth-N: O(S * E^D) where S=seeds, E=avg edges, D=depth
            //
            // Phase 3: Deduplication & Merge
            //   - DISTINCT eliminates overlap between vector results and graph expansion
            //   - O(N log N) HashSet-based deduplication
            //
            // Total complexity: O(log N) + O(S * E^D * log E) + O(N log N)

            // Validate and clamp expand_depth to SurrealDB limits (max 256 levels)
            let safe_depth = if expand_depth > 0 {
                expand_depth.clamp(1, 256)
            } else {
                0
            };

            let initial_limit = if safe_depth > 0 {
                limit / 2 // Reserve half for expansion
            } else {
                limit // No expansion, use full limit
            };

            let vector_json = serde_json::to_string(&vector).unwrap_or_else(|_| "[]".to_string());

            // OPTIMIZED SINGLE-QUERY APPROACH using CTE
            // Combines vector search + graph expansion in one roundtrip
            let sql = if safe_depth > 0 {
                // Build UNION queries for each depth level (1 to safe_depth)
                // Depth 1: ->entangled
                // Depth 2: ->entangled->memory->entangled
                // Depth 3: ->entangled->memory->entangled->memory->entangled
                // Each depth gets its own subquery joined with UNION
                let mut depth_queries = Vec::new();

                for depth in 1..=safe_depth {
                    // Build chain: start with ->entangled, add ->memory->entangled for each additional hop
                    let mut chain = String::from("->entangled");
                    for _ in 1..depth {
                        chain.push_str("->memory->entangled");
                    }

                    // Each depth subquery: traverse chain from seeds, filter by strength
                    depth_queries.push(format!(
                        "SELECT DISTINCT out AS id FROM (SELECT VALUE id FROM $seeds){} WHERE strength >= 0.7",
                        chain
                    ));
                }

                let union_queries = depth_queries.join(" UNION ");

                format!("
                    -- CTE for vector similarity seeds
                    LET $seeds = (
                        SELECT id,
                               vector::similarity::cosine(metadata.embedding, {vector_json}) AS vector_score
                        FROM memory
                        WHERE metadata.embedding != NULL
                        ORDER BY vector_score DESC
                        LIMIT {initial_limit}
                    );

                    -- Hybrid query: seeds + multi-hop graph expansion
                    SELECT DISTINCT m.* FROM memory m
                    WHERE m.id IN (SELECT VALUE id FROM $seeds)  -- Include seed memories
                    OR m.id IN (
                        -- Multi-hop graph expansion using UNION pattern (depths 1..{safe_depth})
                        SELECT DISTINCT VALUE id FROM ({union_queries})
                    )
                    LIMIT {limit};
                ", vector_json = vector_json, initial_limit = initial_limit, limit = limit, safe_depth = safe_depth, union_queries = union_queries)
            } else {
                // No expansion: pure MTREE vector search
                format!("
                    SELECT *,
                           vector::similarity::cosine(metadata.embedding, {vector_json}) AS vector_score
                    FROM memory
                    WHERE metadata.embedding != NULL
                    ORDER BY vector_score DESC
                    LIMIT {limit}
                ", vector_json = vector_json, limit = limit)
            };

            match db.query(&sql).await {
                Ok(mut response) => {
                    let results: Vec<MemoryNodeSchema> = response.take(0).unwrap_or_default();

                    log::info!(
                        "Hybrid search (depth {}): {} total results (limit {})",
                        safe_depth,
                        results.len(),
                        limit
                    );

                    for schema in results {
                        let memory = SurrealDBMemoryManager::from_schema(schema);
                        if tx.send(Ok(memory)).await.is_err() {
                            break;
                        }
                    }
                }
                Err(e) => {
                    log::error!("Hybrid search failed (depth {}): {:?}", safe_depth, e);
                    let _ = tx.send(Err(Error::Database(format!("{:?}", e)))).await;
                }
            }
        });

        MemoryStream::new(rx)
    }

    /// Search memories by text with auto-embedding generation
    pub async fn search_by_text(&self, text: &str, limit: usize) -> Result<MemoryStream> {
        // Generate embedding from text
        if let Some(ref embedding_model) = self.embedding_model {
            // Generate embedding asynchronously
            let embedding = embedding_model.embed(text, Some("search".to_string())).await?;

            // Delegate to existing search_by_vector
            let stream = self.search_by_vector(embedding, limit);
            Ok(stream)
        } else {
            Err(Error::Config(
                "No embedding model configured for text search".to_string(),
            ))
        }
    }

    /// Query memories by metadata filters
    pub async fn query_by_metadata(
        &self,
        metadata_filters: std::collections::HashMap<String, serde_json::Value>,
    ) -> Result<MemoryStream> {
        let db = self.db.clone();

        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            // Build WHERE clause from filters
            let mut conditions = Vec::new();
            let mut bindings = Vec::new();

            for (idx, (key, value)) in metadata_filters.iter().enumerate() {
                // Use parameter binding to prevent injection
                let param_name = format!("param_{}", idx);
                conditions.push(format!("metadata.custom.{} = ${}", key, param_name));
                bindings.push((param_name, value.clone()));
            }

            let where_clause = if conditions.is_empty() {
                String::new()
            } else {
                format!(" WHERE {}", conditions.join(" AND "))
            };

            let query_str = format!("SELECT * FROM memory{}", where_clause);

            // Build and execute query with bindings
            let mut query_builder = db.query(&query_str);
            for (param, value) in bindings {
                query_builder = query_builder.bind((param, value));
            }

            match query_builder.await {
                Ok(mut response) => {
                    let results: Vec<MemoryNodeSchema> = response.take(0).unwrap_or_default();

                    for schema in results {
                        let memory = SurrealDBMemoryManager::from_schema(schema);
                        if tx.send(Ok(memory)).await.is_err() {
                            break;
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(Error::Database(format!("{:?}", e)))).await;
                }
            }
        });

        Ok(MemoryStream::new(rx))
    }

    /// Fetch multiple memories by their IDs efficiently
    #[allow(dead_code)] // Utility method for batch memory retrieval
    async fn get_memories_by_ids(&self, ids: Vec<String>) -> Result<Vec<MemoryNode>> {
        // Use SurrealDB's batch select for efficiency
        let query = "SELECT * FROM memory WHERE id IN $ids";

        let mut response = self
            .db
            .query(query)
            .bind(("ids", ids))
            .await
            .map_err(|e| Error::Database(format!("{:?}", e)))?;

        let results: Vec<MemoryNodeSchema> = response
            .take(0)
            .map_err(|e| Error::Database(format!("{:?}", e)))?;

        Ok(results.into_iter().map(Self::from_schema).collect())
    }

    /// Check if a document exists by content hash
    ///
    /// This method enables content-based deduplication by searching for existing
    /// memories with the same content hash.
    ///
    /// # Arguments
    /// * `hash` - The u64 content hash to search for
    ///
    /// # Returns
    /// * `Ok(true)` - A memory with this content hash exists
    /// * `Ok(false)` - No memory with this content hash exists
    /// * `Err(Error)` - Database query failed
    pub async fn document_exists_by_hash(&self, hash: u64) -> Result<bool> {
        let query = "SELECT id FROM memory WHERE content_hash = $hash LIMIT 1";

        let mut response = self
            .db
            .query(query)
            .bind(("hash", hash))
            .await
            .map_err(|e| Error::Database(format!("Failed to query by content_hash: {:?}", e)))?;

        let results: Vec<serde_json::Value> = response
            .take(0)
            .map_err(|e| Error::Database(format!("Failed to parse hash query results: {:?}", e)))?;

        Ok(!results.is_empty())
    }

    /// Find a document by content hash
    ///
    /// Returns the full memory node if a document with the given hash exists.
    ///
    /// # Arguments
    /// * `hash` - The u64 content hash to search for
    ///
    /// # Returns
    /// * `Ok(Some(MemoryNode))` - Found memory with this hash
    /// * `Ok(None)` - No memory with this hash exists
    /// * `Err(Error)` - Database query failed
    pub async fn find_document_by_hash(&self, hash: u64) -> Result<Option<MemoryNode>> {
        let query = "SELECT * FROM memory WHERE content_hash = $hash LIMIT 1";

        let mut response = self
            .db
            .query(query)
            .bind(("hash", hash))
            .await
            .map_err(|e| Error::Database(format!("Failed to query by content_hash: {:?}", e)))?;

        let results: Vec<MemoryNodeSchema> = response
            .take(0)
            .map_err(|e| Error::Database(format!("Failed to parse hash query results: {:?}", e)))?;

        Ok(results.into_iter().next().map(Self::from_schema))
    }

    /// Update document age/timestamp by content hash
    ///
    /// This method "refreshes" a document by updating its timestamps when identical
    /// content is re-ingested. This ensures frequently referenced documents remain
    /// fresh in the temporal decay model.
    ///
    /// # Arguments
    /// * `hash` - The u64 content hash to search for
    /// * `timestamp` - The new timestamp (DateTime<Utc>)
    ///
    /// # Returns
    /// * `Ok(true)` - Successfully updated timestamp
    /// * `Ok(false)` - No memory with this hash exists
    /// * `Err(Error)` - Database update failed
    pub async fn update_document_age_by_hash(
        &self,
        hash: u64,
        timestamp: chrono::DateTime<chrono::Utc>,
    ) -> Result<bool> {
        // Update both updated_at and last_accessed_at to "freshen" the document
        let query = "UPDATE memory SET updated_at = $timestamp, metadata.last_accessed_at = $timestamp WHERE content_hash = $hash";

        let mut response = self
            .db
            .query(query)
            .bind(("hash", hash))
            .bind(("timestamp", timestamp))
            .await
            .map_err(|e| {
                Error::Database(format!("Failed to update age by content_hash: {:?}", e))
            })?;

        let results: Vec<serde_json::Value> = response
            .take(0)
            .map_err(|e| Error::Database(format!("Failed to parse update results: {:?}", e)))?;

        Ok(!results.is_empty())
    }
}
