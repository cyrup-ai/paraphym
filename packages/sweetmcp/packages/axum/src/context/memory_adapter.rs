//! Memory system adapter for context API
//! This module provides integration between MCP context API and the sophisticated memory system

use std::sync::Arc;

use anyhow::Result;
use serde_json::Value;
use tokio::sync::RwLock;

// Production memory system from paraphym_candle
use paraphym_candle::memory::{
    MemoryConfig, initialize, MemoryMetadata,
};
use paraphym_candle::memory::core::primitives::types::MemoryTypeEnum as CoreMemoryTypeEnum;
use paraphym_candle::memory::core::manager::coordinator::MemoryCoordinator;
use paraphym_candle::domain::memory::primitives::types::MemoryTypeEnum as DomainMemoryTypeEnum;

/// Production memory adapter using SurrealDB with automatic BERT embedding generation
#[derive(Clone)]
pub struct MemoryContextAdapter {
    memory_coordinator: Arc<MemoryCoordinator>,
    subscriptions: Arc<RwLock<Vec<String>>>,
}

impl MemoryContextAdapter {
    /// Create a new memory context adapter with production memory system
    pub async fn new() -> Result<Self> {
        // Use production MemoryConfig from paraphym_candle
        let config = MemoryConfig {
            database: paraphym_candle::memory::utils::config::DatabaseConfig {
                db_type: paraphym_candle::memory::utils::config::DatabaseType::SurrealDB,
                connection_string: "surrealkv://./data/context_memory.db".to_string(),
                namespace: "context".to_string(),
                database: "mcp".to_string(),
                username: None,
                password: None,
                pool_size: Some(5),
                options: None,
            },
            // Note: MemoryCoordinator generates BERT embeddings (384D) internally
            // No external vector_store config needed
            vector_store: paraphym_candle::memory::utils::config::VectorStoreConfig {
                store_type: paraphym_candle::memory::utils::config::VectorStoreType::SurrealDB,
                embedding_model: paraphym_candle::memory::utils::config::EmbeddingModelConfig {
                    model_type: paraphym_candle::memory::utils::config::EmbeddingModelType::Custom,
                    model_name: "bert-base-uncased".to_string(),
                    api_key: None,
                    api_base: None,
                    options: None,
                },
                dimension: 384, // Actual BERT embedding dimension used by coordinator
                connection_string: None,
                api_key: None,
                options: None,
            },
            completion: paraphym_candle::memory::utils::config::CompletionConfig {
                provider: paraphym_candle::memory::utils::config::CompletionProvider::Custom,
                model_name: "default".to_string(),
                api_key: None,
                api_base: None,
                temperature: Some(0.7),
                max_tokens: Some(2048),
                options: None,
            },
            api: None,
            cache: paraphym_candle::memory::utils::config::CacheConfig {
                enabled: true,
                cache_type: paraphym_candle::memory::utils::config::CacheType::Memory,
                size: Some(1000),
                ttl: Some(3600),
                options: None,
            },
            logging: paraphym_candle::memory::utils::config::LoggingConfig {
                level: paraphym_candle::memory::utils::config::LogLevel::Info,
                file: Some("./logs/context_memory.log".to_string()),
                console: true,
                options: None,
            },
        };

        // Initialize production memory system with coordinator for automatic embeddings
        let manager = initialize(&config)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to initialize memory system: {}", e))?;

        // Wrap with MemoryCoordinator to enable automatic BERT embedding generation
        let coordinator = MemoryCoordinator::new(Arc::new(manager))
            .await
            .map_err(|e| anyhow::anyhow!("Failed to initialize memory coordinator: {}", e))?;

        Ok(Self {
            memory_coordinator: Arc::new(coordinator),
            subscriptions: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Store context value using production memory system with automatic BERT embeddings
    /// Updates existing context if key already exists, creates new one otherwise
    pub async fn store_context(&self, key: String, value: Value) -> Result<()> {
        let json_str = serde_json::to_string(&value)?;

        // Check if context with this key already exists
        let existing = self.find_context_memory(&key).await?;

        if let Some(existing_memory) = existing {
            // Update existing memory with new content
            let existing_id = existing_memory.id().to_string();
            
            // Preserve existing metadata, just update the content
            let mut updated_metadata = paraphym_candle::memory::MemoryMetadata::new();
            updated_metadata.importance = existing_memory.metadata.importance;
            updated_metadata.custom = existing_memory.metadata.custom
                .iter()
                .map(|(k, v)| (k.to_string(), (**v).clone()))
                .collect();
            
            self.memory_coordinator
                .update_memory(&existing_id, Some(json_str), Some(updated_metadata))
                .await
                .map_err(|e| anyhow::anyhow!("Failed to update context: {}", e))?;
        } else {
            // Create new memory with metadata
            let mut metadata = MemoryMetadata::new();
            metadata.importance = 1.0;
            metadata.custom = serde_json::json!({
                "key": key,
                "type": "context"
            });

            self.memory_coordinator
                .add_memory(json_str, DomainMemoryTypeEnum::Semantic, metadata)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to store context: {}", e))?;
        }

        Ok(())
    }

    /// Retrieve context value from production memory using metadata filtering
    pub async fn get_context(&self, key: &str) -> Result<Option<Value>> {
        if let Some(memory) = self.find_context_memory(key).await? {
            let value: Value = serde_json::from_str(memory.content().to_string().as_str())?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    /// Helper method to find a context memory by key
    async fn find_context_memory(&self, key: &str) -> Result<Option<paraphym_candle::domain::memory::primitives::node::MemoryNode>> {
        // Filter by memory type, context type, and specific key in metadata
        let filter = paraphym_candle::memory::core::ops::filter::MemoryFilter::default()
            .with_memory_types(vec![CoreMemoryTypeEnum::Semantic])
            .with_metadata("type", serde_json::Value::String("context".to_string()))
            .with_metadata("key", serde_json::Value::String(key.to_string()));

        let memories = self.memory_coordinator
            .get_memories(filter)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to find context: {}", e))?;

        Ok(memories.into_iter().next())
    }

    /// Add a subscription
    pub async fn add_subscription(&self, uri: String) -> Result<()> {
        let mut subs = self.subscriptions.write().await;
        if !subs.contains(&uri) {
            subs.push(uri);
        }
        Ok(())
    }

    /// Remove a subscription
    pub async fn remove_subscription(&self, uri: &str) -> Result<()> {
        let mut subs = self.subscriptions.write().await;
        subs.retain(|s| s != uri);
        Ok(())
    }

    /// Get all active subscriptions
    pub async fn get_subscriptions(&self) -> Vec<String> {
        self.subscriptions.read().await.clone()
    }

    /// Search using production semantic search with BERT embeddings
    /// 
    /// # Arguments
    /// * `pattern` - The search pattern/query
    /// * `limit` - Maximum number of results to return (default: 10, max: 100)
    pub async fn search_contexts(&self, pattern: &str, limit: Option<usize>) -> Result<Vec<(String, Value)>> {
        let mut results = Vec::new();

        // Enforce reasonable limits: default 10, max 100
        let search_limit = limit.unwrap_or(10).min(100);

        // Use coordinator's semantic search which leverages BERT embeddings
        let search_results = self.memory_coordinator
            .search_memories(pattern, None, search_limit)
            .await
            .map_err(|e| anyhow::anyhow!("Search failed: {}", e))?;

        for memory in search_results {
            // Check if this is a context entry by looking at metadata
            if let Some(custom) = memory.metadata.custom.get("type") {
                if custom.as_str() == Some("context") {
                    if let Some(key_value) = memory.metadata.custom.get("key") {
                        if let Some(key) = key_value.as_str() {
                            if let Ok(value) = serde_json::from_str(memory.content().to_string().as_str()) {
                                results.push((key.to_string(), value));
                            }
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// Graceful shutdown with proper resource cleanup
    pub async fn shutdown(&self) -> Result<()> {
        log::info!("Shutting down MemoryContextAdapter...");
        
        // Clear all subscriptions
        let mut subs = self.subscriptions.write().await;
        let sub_count = subs.len();
        subs.clear();
        drop(subs); // Explicitly drop write lock

        log::info!("Cleared {} subscriptions", sub_count);

        // Note: MemoryCoordinator and SurrealDB connections are cleaned up
        // automatically when the Arc is dropped. The Surreal<Any> connection
        // has a proper Drop implementation that closes the connection.
        log::info!("MemoryContextAdapter shutdown complete. Database connections will be closed on Arc drop.");
        
        Ok(())
    }

    /// Validate memory system
    pub async fn validate(&self) -> Result<()> {
        // Test search functionality with proper timeout wrapping the async operation
        match tokio::time::timeout(
            std::time::Duration::from_secs(5),
            self.memory_coordinator.search_memories("__health_check__", None, 1)
        ).await {
            Ok(Ok(_)) => {
                log::debug!("Memory system validation passed: Coordinator and embeddings functional");
                Ok(())
            }
            Ok(Err(e)) => {
                Err(anyhow::anyhow!("Memory system validation failed: {}", e))
            }
            Err(_) => {
                Err(anyhow::anyhow!("Memory system validation failed: timeout"))
            }
        }
    }

    /// Get real statistics from memory system using efficient metadata filtering
    pub async fn get_stats(&self) -> MemoryStats {
        let mut total_nodes = 0;
        let mut total_relationships = 0;

        // Filter for context entries directly using metadata
        let filter = paraphym_candle::memory::core::ops::filter::MemoryFilter::default()
            .with_memory_types(vec![CoreMemoryTypeEnum::Semantic])
            .with_metadata("type", serde_json::Value::String("context".to_string()));

        match self.memory_coordinator.get_memories(filter).await {
            Ok(memories) => {
                total_nodes = memories.len();
                
                // Count relationships for context nodes
                for memory in memories {
                    let node_id = memory.id().to_string();
                    if let Ok(relationships) = self.memory_coordinator.get_relationships(&node_id).await {
                        total_relationships += relationships.len();
                    }
                }
            }
            Err(e) => {
                log::warn!("Failed to get memory statistics: {}", e);
            }
        }

        MemoryStats {
            total_nodes,
            total_relationships,
        }
    }
}

/// Memory adapter statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_nodes: usize,
    pub total_relationships: usize,
}
