//! Extended search and query operations for SurrealDB.
//!
//! This module provides specialized query methods beyond the base MemoryManager trait,
//! including hybrid search, text search, metadata querying, and content-based deduplication.

use crate::capability::traits::TextEmbeddingCapable;
use crate::memory::primitives::MemoryNode;
use crate::memory::schema::memory_schema::MemoryNodeSchema;
use crate::memory::utils::error::Error;

use super::futures::MemoryStream;
use super::manager::SurrealDBMemoryManager;
use super::trait_def::MemoryManager;
use super::Result;

impl SurrealDBMemoryManager {
    /// Advanced hybrid search combining vector similarity and graph expansion
    ///
    /// This method performs a two-phase search:
    /// 1. MTREE-based vector similarity search to find initial "seed" memories
    /// 2. Multi-hop graph traversal via entanglement edges to expand results
    ///
    /// # Arguments
    /// * `query_vector` - The embedding vector to search for
    /// * `limit` - Maximum number of results to return
    /// * `expansion_depth` - Number of graph hops (0 = pure vector search, 1-5 = hybrid)
    ///
    /// # Returns
    /// A stream of memories ordered by relevance (vector score + graph proximity)
    pub fn search_with_entanglement(
        &self,
        query_vector: Vec<f32>,
        limit: usize,
        expansion_depth: usize,
    ) -> MemoryStream {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let db = self.db.clone();

        tokio::spawn(async move {
            let vector_json = serde_json::to_string(&query_vector).unwrap_or_default();
            let safe_depth = expansion_depth.min(5);

            let initial_limit = (limit as f32 * 0.3).ceil() as usize;

            let sql = if safe_depth > 0 {
                let mut depth_queries = Vec::new();
                for depth in 1..=safe_depth {
                    let mut chain = String::from("->entangled");
                    for _ in 1..depth {
                        chain.push_str("->memory->entangled");
                    }

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
        if let Some(ref embedding_model) = self.embedding_model {
            let embedding = embedding_model.embed(text, Some("search".to_string())).await?;
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
            let mut conditions = Vec::new();
            let mut bindings = Vec::new();

            for (idx, (key, value)) in metadata_filters.iter().enumerate() {
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
    #[allow(dead_code)]
    async fn get_memories_by_ids(&self, ids: Vec<String>) -> Result<Vec<MemoryNode>> {
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
    /// * `hash` - The i64 content hash to search for
    ///
    /// # Returns
    /// * `Ok(true)` - A memory with this content hash exists
    /// * `Ok(false)` - No memory with this content hash exists
    /// * `Err(Error)` - Database query failed
    pub async fn document_exists_by_hash(&self, hash: i64) -> Result<bool> {
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
    /// * `hash` - The i64 content hash to search for
    ///
    /// # Returns
    /// * `Ok(Some(MemoryNode))` - Found memory with this hash
    /// * `Ok(None)` - No memory with this hash exists
    /// * `Err(Error)` - Database query failed
    pub async fn find_document_by_hash(&self, hash: i64) -> Result<Option<MemoryNode>> {
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
    /// * `hash` - The i64 content hash to search for
    /// * `timestamp` - The new timestamp (DateTime<Utc>)
    ///
    /// # Returns
    /// * `Ok(true)` - Successfully updated timestamp
    /// * `Ok(false)` - No memory with this hash exists
    /// * `Err(Error)` - Database update failed
    pub async fn update_document_age_by_hash(
        &self,
        hash: i64,
        timestamp: chrono::DateTime<chrono::Utc>,
    ) -> Result<bool> {
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
