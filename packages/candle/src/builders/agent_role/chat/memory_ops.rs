//! Memory operations for chat functionality

use super::super::*;
use surrealdb::engine::any::connect;
use tokio_stream::StreamExt;
use crate::memory::core::manager::surreal::SurrealDBMemoryManager;
use crate::memory::core::manager::coordinator::MemoryCoordinator;
use crate::memory::primitives::node::MemoryNode;
use crate::memory::primitives::types::{MemoryContent, MemoryTypeEnum};

pub(super) async fn initialize_memory_coordinator(
    emb_model: &TextEmbeddingModel,
) -> Result<Arc<MemoryCoordinator>, String> {
    let db_path = dirs::cache_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("cyrup")
        .join("agent.db");

    // Ensure database directory exists
    if let Some(parent) = db_path.parent() {
        if let Err(e) = tokio::fs::create_dir_all(parent).await {
            return Err(format!("Failed to create database directory: {}", e));
        }
    }

    let db_url = format!("surrealkv://{}", db_path.display());

    // Connect to database
    let db = match connect(&db_url).await {
        Ok(db) => db,
        Err(e) => return Err(format!("Failed to connect to database: {}", e)),
    };

    // Initialize database namespace
    if let Err(e) = db.use_ns("candle").use_db("agent").await {
        return Err(format!("Failed to initialize database namespace: {}", e));
    }

    // Create SurrealDBMemoryManager
    let surreal_manager = SurrealDBMemoryManager::with_embedding_model(db, emb_model.clone());

    if let Err(e) = surreal_manager.initialize().await {
        return Err(format!("Failed to initialize memory tables: {}", e));
    }

    let surreal_arc = Arc::new(surreal_manager);

    // Create MemoryCoordinator
    let coordinator = match MemoryCoordinator::new(surreal_arc, emb_model.clone()).await {
        Ok(coord) => coord,
        Err(e) => return Err(format!("Failed to create memory coordinator: {:?}", e)),
    };

    Ok(Arc::new(coordinator))
}

pub(super) async fn load_context_into_memory(
    memory: &Arc<dyn crate::memory::core::manager::surreal::MemoryManager>,
    context_file: Option<CandleContext<CandleFile>>,
    context_files: Option<CandleContext<CandleFiles>>,
    context_directory: Option<CandleContext<CandleDirectory>>,
    context_github: Option<CandleContext<CandleGithub>>,
) -> Result<(), String> {
    // Load from context_file
    if let Some(ctx) = context_file {
        let stream = ctx.load();
        tokio::pin!(stream);
        while let Some(doc) = stream.next().await {
            let content = MemoryContent::new(&doc.data);
            let mut node = MemoryNode::new(MemoryTypeEnum::Semantic, content);
            node.metadata.source = doc.additional_props.get("path")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            node.metadata.importance = 0.5;

            let pending = memory.create_memory(node);
            if let Err(e) = pending.await {
                log::warn!("Failed to ingest document into memory: {:?}", e);
            }
        }
    }

    // Load from context_files
    if let Some(ctx) = context_files {
        let stream = ctx.load();
        tokio::pin!(stream);
        while let Some(doc) = stream.next().await {
            let content = MemoryContent::new(&doc.data);
            let mut node = MemoryNode::new(MemoryTypeEnum::Semantic, content);
            node.metadata.source = doc.additional_props.get("path")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            node.metadata.importance = 0.5;

            let pending = memory.create_memory(node);
            if let Err(e) = pending.await {
                log::warn!("Failed to ingest document into memory: {:?}", e);
            }
        }
    }

    // Load from context_directory
    if let Some(ctx) = context_directory {
        let stream = ctx.load();
        tokio::pin!(stream);
        while let Some(doc) = stream.next().await {
            let content = MemoryContent::new(&doc.data);
            let mut node = MemoryNode::new(MemoryTypeEnum::Semantic, content);
            node.metadata.source = doc.additional_props.get("path")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            node.metadata.importance = 0.5;

            let pending = memory.create_memory(node);
            if let Err(e) = pending.await {
                log::warn!("Failed to ingest document into memory: {:?}", e);
            }
        }
    }

    // Load from context_github
    if let Some(ctx) = context_github {
        let stream = ctx.load();
        tokio::pin!(stream);
        while let Some(doc) = stream.next().await {
            let content = MemoryContent::new(&doc.data);
            let mut node = MemoryNode::new(MemoryTypeEnum::Semantic, content);
            node.metadata.source = doc.additional_props.get("path")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            node.metadata.importance = 0.5;

            let pending = memory.create_memory(node);
            if let Err(e) = pending.await {
                log::warn!("Failed to ingest document into memory: {:?}", e);
            }
        }
    }

    Ok(())
}

pub(super) fn create_user_memory(
    message: &str,
    metadata: &std::collections::HashMap<String, String>,
) -> MemoryNode {
    let user_content = MemoryContent::new(message);
    let mut user_memory = MemoryNode::new(MemoryTypeEnum::Episodic, user_content);
    user_memory.metadata.tags.push("user_message".to_string());
    user_memory.metadata.source = Some("chat".to_string());
    user_memory.metadata.importance = 0.8;

    // Merge builder metadata into memory metadata
    for (key, value) in metadata {
        if let Err(e) = user_memory.metadata.set_custom(key, value) {
            log::warn!("Failed to set custom metadata '{}' on user memory: {:?}", key, e);
        }
    }

    user_memory
}

pub(super) fn create_assistant_memory(
    message: &str,
    metadata: &std::collections::HashMap<String, String>,
) -> MemoryNode {
    let assistant_content = MemoryContent::new(message);
    let mut assistant_memory = MemoryNode::new(MemoryTypeEnum::Episodic, assistant_content);
    assistant_memory.metadata.tags.push("assistant_response".to_string());
    assistant_memory.metadata.source = Some("chat".to_string());
    assistant_memory.metadata.importance = 0.8;

    // Merge builder metadata into memory metadata
    for (key, value) in metadata {
        if let Err(e) = assistant_memory.metadata.set_custom(key, value) {
            log::warn!("Failed to set custom metadata '{}' on assistant memory: {:?}", key, e);
        }
    }

    assistant_memory
}
