# MEMFIX_3: Refactor memory_adapter.rs to Use SurrealDBMemoryManager Directly

## OBJECTIVE:
Remove MemoryCoordinator dependency and update memory_adapter.rs to use SurrealDBMemoryManager directly with database queries instead of cache operations.

## PREREQUISITE:
- MEMFIX_1 and MEMFIX_2 must be completed (SurrealDBMemoryManager has embedding capability)

## SUBTASK1: Update imports
**Location:** `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/context/memory_adapter.rs` (lines 1-20)

Remove MemoryCoordinator and add SurrealDBMemoryManager:
```rust
// REMOVE:
// use paraphym_candle::memory::core::manager::coordinator::MemoryCoordinator;

// ADD:
use paraphym_candle::memory::core::manager::surreal::SurrealDBMemoryManager;
use std::collections::HashMap;
use futures::StreamExt;
use serde_json::json;
```

## SUBTASK2: Update struct field
**Location:** `memory_adapter.rs` (line ~21)

Change from coordinator to manager:
```rust
#[derive(Clone)]
pub struct MemoryContextAdapter {
    // CHANGE FROM:
    // memory_coordinator: Arc<MemoryCoordinator>,
    // TO:
    memory_manager: Arc<SurrealDBMemoryManager>,
    subscriptions: Arc<RwLock<Vec<String>>>,
}
```

## SUBTASK3: Fix initialization in new()
**Location:** `memory_adapter.rs` (lines 80-95)

Initialize SurrealDBMemoryManager directly:
```rust
pub async fn new() -> Result<Self> {
    let config = MemoryConfig { 
        // ... existing config ...
    };
    
    // Initialize with SurrealDB
    let db = surrealdb::Surreal::new::<surrealdb::engine::any::Any>(
        "surrealkv://./data/context_memory.db"
    ).await?;
    
    // Use namespace and database
    db.use_ns("context").use_db("mcp").await?;
    
    // Create SurrealDBMemoryManager with embeddings
    let manager = SurrealDBMemoryManager::with_embeddings(db).await?;
    
    Ok(Self {
        memory_manager: Arc::new(manager),
        subscriptions: Arc::new(RwLock::new(Vec::new())),
    })
}
```

## SUBTASK4: Update store_context() method
**Location:** `memory_adapter.rs` (lines 99-137)

Use manager methods directly:
```rust
pub async fn store_context(&self, key: String, value: Value) -> Result<()> {
    let json_str = serde_json::to_string(&value)?;
    
    // Check if context exists using database query
    let existing = self.find_context_memory(&key).await?;
    
    if let Some(existing_memory) = existing {
        // Update existing
        let mut updated = existing_memory.clone();
        updated.content = paraphym_candle::memory::core::primitives::types::MemoryContent::new(&json_str);
        
        self.memory_manager
            .update_memory(updated)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to update: {}", e))?;
    } else {
        // Create new with metadata
        let mut memory = paraphym_candle::domain::memory::primitives::node::MemoryNode::new(
            paraphym_candle::domain::memory::primitives::types::MemoryTypeEnum::Semantic,
            paraphym_candle::domain::memory::primitives::types::MemoryContent::text(json_str)
        );
        
        memory.set_custom_metadata("type", json!("context"));
        memory.set_custom_metadata("key", json!(key));
        
        self.memory_manager
            .create_memory(memory)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create: {}", e))?;
    }
    
    Ok(())
}
```

## SUBTASK5: Fix find_context_memory() to query database
**Location:** `memory_adapter.rs` (lines 150-163)

Query database directly:
```rust
async fn find_context_memory(&self, key: &str) -> Result<Option<paraphym_candle::domain::memory::primitives::node::MemoryNode>> {
    // Build metadata filters
    let mut filters = HashMap::new();
    filters.insert("type".to_string(), json!("context"));
    filters.insert("key".to_string(), json!(key));
    
    // Query database
    let mut stream = self.memory_manager
        .query_by_metadata(filters)
        .await
        .map_err(|e| anyhow::anyhow!("Query failed: {}", e))?;
    
    // Get first result
    if let Some(result) = stream.next().await {
        Ok(Some(result?))
    } else {
        Ok(None)
    }
}
```

## SUBTASK6: Update search_contexts() method
**Location:** `memory_adapter.rs` (lines 186-218)

Use search_by_text:
```rust
pub async fn search_contexts(&self, pattern: &str, limit: Option<usize>) -> Result<Vec<(String, Value)>> {
    let search_limit = limit.unwrap_or(10).min(100);
    
    // Use semantic search
    let mut stream = self.memory_manager
        .search_by_text(pattern, search_limit)
        .await
        .map_err(|e| anyhow::anyhow!("Search failed: {}", e))?;
    
    let mut results = Vec::new();
    
    while let Some(memory_result) = stream.next().await {
        let memory = memory_result?;
        
        // Check if context entry
        if let Some(type_val) = memory.metadata.custom.get("type") {
            if type_val.as_str() == Some("context") {
                if let Some(key_val) = memory.metadata.custom.get("key") {
                    if let Some(key) = key_val.as_str() {
                        let value: Value = serde_json::from_str(&memory.content.to_string())?;
                        results.push((key.to_string(), value));
                    }
                }
            }
        }
    }
    
    Ok(results)
}
```

## SUBTASK7: Fix get_stats() to query database
**Location:** `memory_adapter.rs` (lines 254-283)

Query database for real stats:
```rust
pub async fn get_stats(&self) -> MemoryStats {
    let mut filters = HashMap::new();
    filters.insert("type".to_string(), json!("context"));
    
    match self.memory_manager.query_by_metadata(filters).await {
        Ok(mut stream) => {
            let mut total_nodes = 0;
            let mut total_relationships = 0;
            
            while let Some(result) = stream.next().await {
                if let Ok(memory) = result {
                    total_nodes += 1;
                    
                    // Count relationships
                    if let Ok(rels) = self.memory_manager
                        .get_relationships(&memory.id)
                        .await 
                    {
                        total_relationships += rels.len();
                    }
                }
            }
            
            MemoryStats {
                total_nodes,
                total_relationships,
            }
        }
        Err(e) => {
            log::warn!("Failed to get stats: {}", e);
            MemoryStats {
                total_nodes: 0,
                total_relationships: 0,
            }
        }
    }
}
```

## SUBTASK8: Update validate() method
**Location:** `memory_adapter.rs` (around line 240)

Update to use manager directly:
```rust
pub async fn validate(&self) -> Result<()> {
    // Test search functionality
    match tokio::time::timeout(
        std::time::Duration::from_secs(5),
        self.memory_manager.search_by_text("__health_check__", 1)
    ).await {
        Ok(Ok(_)) => {
            log::debug!("Memory system validation passed");
            Ok(())
        }
        Ok(Err(e)) => {
            Err(anyhow::anyhow!("Validation failed: {}", e))
        }
        Err(_) => {
            Err(anyhow::anyhow!("Validation timeout"))
        }
    }
}
```

## Definition of Done:
- MemoryCoordinator is completely removed
- memory_adapter uses SurrealDBMemoryManager directly
- All methods query the database, not cache
- Data persists across restarts
- Code compiles without errors
- All anyhow::Result error handling is proper

## Research Notes:
- Current implementation at: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/context/memory_adapter.rs`
- MemoryCoordinator usage to be removed from lines: 15, 80-95
- get_memories() cache-only bug is at lines 150-163
- SurrealDBMemoryManager methods from MEMFIX_2: search_by_text(), query_by_metadata()

## Requirements:
- DO NOT write any unit tests
- DO NOT write any benchmarks
- DO NOT create any documentation files
- Ensure all data operations hit the database directly
- Use proper error handling with anyhow::Result