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
use crate::memory::primitives::types::MemoryTypeEnum;
use crate::memory::primitives::{MemoryNode, MemoryRelationship};
use crate::memory::schema::memory_schema::{MemoryMetadataSchema, MemoryNodeSchema};
use crate::memory::schema::relationship_schema::RelationshipSchema;
use crate::memory::utils::error::Error;
use crate::memory::migration::{DataExporter, ExportFormat, MigrationManager, BuiltinMigrations, DataImporter, ImportFormat};
use std::path::Path;

// Vector search and embedding imports
use crate::memory::vector::vector_search::{VectorSearch, SearchOptions};
use crate::memory::vector::embedding_model::EmbeddingModel;
use crate::providers::bert_embedding::CandleBertEmbeddingProvider;
use tracing;  // For logging in create_memory

/// Content structure for creating/updating memory nodes (without ID)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MemoryNodeCreateContent {
    pub content: String,
    pub memory_type: MemoryTypeEnum,
    pub metadata: MemoryMetadataSchema,
}

impl From<&MemoryNode> for MemoryNodeCreateContent {
    fn from(memory: &MemoryNode) -> Self {
        Self {
            content: memory.content.text.clone(),
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

/// A stream of memory nodes
pub struct MemoryStream {
    rx: tokio::sync::mpsc::Receiver<Result<MemoryNode>>,
}

impl MemoryStream {
    fn new(rx: tokio::sync::mpsc::Receiver<Result<MemoryNode>>) -> Self {
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
}

/// SurrealDB implementation of the memory manager
#[derive(Debug)]
pub struct SurrealDBMemoryManager {
    db: Surreal<Any>,
    #[allow(dead_code)] // Will be used in future VectorStore implementation
    vector_search: Option<Arc<VectorSearch>>,
    #[allow(dead_code)] // Will be used in future VectorStore implementation  
    embedding_model: Option<Arc<dyn EmbeddingModel>>,
}

/// Data structure for memory export containing nodes and relationships
#[derive(Debug, Serialize, Deserialize)]
struct ExportData {
    nodes: Vec<MemoryNode>,
    relationships: Vec<MemoryRelationship>,
}

impl SurrealDBMemoryManager {
    /// Create a new SurrealDB memory manager with embedding support
    ///
    /// This factory method initializes the manager with BERT embedding capabilities.
    /// The VectorSearch field remains None as VectorStore implementation is pending.
    pub async fn with_embeddings(db: Surreal<Any>) -> Result<Self> {
        // Create BERT embedding model using ProgressHub download
        let embedding_model = Arc::new(
            CandleBertEmbeddingProvider::new().await?
        ) as Arc<dyn EmbeddingModel>;

        // VectorSearch requires VectorStore trait implementation which is not yet available
        // When VectorStore is implemented, initialize VectorSearch here

        Ok(Self {
            db,
            vector_search: None, // Will be populated when VectorStore implementation is available
            embedding_model: Some(embedding_model),
        })
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

        let mut metadata = MemoryMetadata::new();
        metadata.created_at = schema.metadata.created_at;
        metadata.last_accessed_at = Some(schema.metadata.last_accessed_at);
        metadata.importance = schema.metadata.importance;
        metadata.embedding = embedding.clone();
        metadata.custom = schema.metadata.custom;

        MemoryNode {
            id,
            content: crate::memory::core::primitives::types::MemoryContent::new(&schema.content),
            memory_type: schema.memory_type,
            created_at: schema.metadata.created_at,
            updated_at: schema.metadata.last_accessed_at,
            embedding,
            metadata,
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
        tracing::info!("Initializing migration manager...");

        // Create migration manager with shared database connection
        let db_arc = Arc::new(self.db.clone());
        let mut manager = MigrationManager::new(db_arc).await
            .map_err(|e| Error::Other(format!("Failed to create migration manager: {}", e)))?;

        // Add all builtin migrations
        for migration in BuiltinMigrations::all() {
            manager.add_migration(migration);
        }

        tracing::info!("Running schema migrations...");

        // Run migrations (now async, not returning PendingMigration)
        manager.migrate().await
            .map_err(|e| Error::Other(format!("Migration execution failed: {}", e)))?;

        tracing::info!("Schema migrations completed successfully");

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
    pub async fn export_memories(
        &self,
        path: &Path,
        format: ExportFormat,
    ) -> Result<usize> {
        tracing::info!("Exporting memories to {:?} in {:?} format", path, format);

        // Query all memory nodes from 'memory' table
        let nodes_query = "SELECT * FROM memory";
        let mut nodes_response = self.db.query(nodes_query).await
            .map_err(|e| Error::Database(format!("Failed to query memory nodes: {}", e)))?;

        // Get as schema first, then convert using established helper
        let node_schemas: Vec<MemoryNodeSchema> = nodes_response.take(0)
            .map_err(|e| Error::Database(format!("Failed to parse memory nodes: {}", e)))?;

        // Use from_schema for consistent conversion (see line 328)
        let nodes: Vec<MemoryNode> = node_schemas
            .into_iter()
            .map(SurrealDBMemoryManager::from_schema)
            .collect();

        // Query all relationships from 'memory_relationship' table
        let rels_query = "SELECT * FROM memory_relationship";
        let mut rels_response = self.db.query(rels_query).await
            .map_err(|e| Error::Database(format!("Failed to query relationships: {}", e)))?;

        // Get as RelationshipSchema first, then convert properly
        let relationship_schemas: Vec<RelationshipSchema> = rels_response.take(0)
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
        exporter.export_to_file(export_slice, path).await
            .map_err(|e| Error::Migration(format!("Export failed: {}", e)))?;

        let total = nodes.len() + relationships.len();
        tracing::info!("Successfully exported {} items to {:?}", total, path);

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
    pub async fn import_memories(
        &self,
        path: &Path,
        format: ImportFormat,
    ) -> Result<usize> {
        tracing::info!("Importing memories from {:?} in {:?} format", path, format);

        // Use DataImporter to read from file
        let importer = DataImporter::new();
        let import_vec: Vec<ExportData> = match format {
            ImportFormat::Json => importer.import_json(path).await
                .map_err(|e| Error::Migration(format!("JSON import failed: {}", e)))?,
            ImportFormat::Csv => importer.import_csv(path).await
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
            let result: Option<MemoryNodeSchema> = self.db
                .create(("memory", node.id.as_str()))
                .content(create_content)
                .await
                .map_err(|e| Error::Database(format!("Failed to insert memory node {}: {}", node.id, e)))?;

            match result {
                Some(_) => inserted_count += 1,
                None => return Err(Error::NotFound(format!("Failed to create memory node {}", node.id))),
            }
        }

        // Insert relationships
        for rel in import_data.relationships {
            let create_content = RelationshipCreateContent::from(&rel);
            let result: Option<RelationshipSchema> = self.db
                .create(("memory_relationship", rel.id.as_str()))
                .content(create_content)
                .await
                .map_err(|e| Error::Database(format!("Failed to insert relationship {}: {}", rel.id, e)))?;

            match result {
                Some(_) => inserted_count += 1,
                None => return Err(Error::NotFound(format!("Failed to create relationship {}", rel.id))),
            }
        }

        tracing::info!("Successfully imported {} items from {:?}", inserted_count, path);

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
            if memory.embedding.is_none() {
                if let Some(ref model) = embedding_model {
                    // Extract text content for embedding
                    let content_text = memory.content.text.clone();
                    
                    // Generate embedding synchronously (EmbeddingModel trait methods are sync)
                    match model.embed(&content_text, Some("search".to_string())) {
                        Ok(embedding_vec) => {
                            memory.embedding = Some(embedding_vec);
                            // Also update metadata.embedding for consistency
                            memory.metadata.embedding = memory.embedding.clone();
                        }
                        Err(e) => {
                            // Log but don't fail - memory can exist without embedding
                            tracing::warn!("Failed to generate embedding: {:?}", e);
                        }
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
            let created: Option<RelationshipSchema> = match db
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
                    let results: Vec<RelationshipSchema> = response.take(0).unwrap_or_default();

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
                .delete::<Option<RelationshipSchema>>(("memory_relationship", &id_str))
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
}

// Additional methods for SurrealDBMemoryManager
impl SurrealDBMemoryManager {
    /// Search memories by text with auto-embedding generation
    pub async fn search_by_text(
        &self,
        text: &str,
        limit: usize
    ) -> Result<MemoryStream> {
        // Generate embedding from text
        if let Some(ref embedding_model) = self.embedding_model {
            // Generate embedding synchronously
            let embedding = embedding_model.embed(
                text,
                Some("search".to_string())
            )?;
            
            // Delegate to existing search_by_vector
            let stream = self.search_by_vector(embedding, limit);
            Ok(stream)
        } else {
            Err(Error::Config(
                "No embedding model configured for text search".to_string()
            ))
        }
    }

    /// Query memories by metadata filters
    pub async fn query_by_metadata(
        &self,
        metadata_filters: std::collections::HashMap<String, serde_json::Value>
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
    async fn get_memories_by_ids(&self, ids: Vec<String>) -> Result<Vec<MemoryNode>> {
        // Use SurrealDB's batch select for efficiency
        let query = "SELECT * FROM memory WHERE id IN $ids";
        
        let mut response = self.db
            .query(query)
            .bind(("ids", ids))
            .await
            .map_err(|e| Error::Database(format!("{:?}", e)))?;
        
        let results: Vec<MemoryNodeSchema> = response
            .take(0)
            .map_err(|e| Error::Database(format!("{:?}", e)))?;
        
        Ok(results
            .into_iter()
            .map(Self::from_schema)
            .collect())
    }
}
