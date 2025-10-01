# MEMFIX_3: Refactor memory_adapter.rs to Use SurrealDBMemoryManager Directly

## OBJECTIVE:
Remove MemoryCoordinator dependency and update memory_adapter.rs to use SurrealDBMemoryManager directly with database queries instead of cache operations. This ensures all memory operations are persistent and directly interact with the SurrealDB database.

## PREREQUISITE:
- MEMFIX_1 and MEMFIX_2 must be completed (SurrealDBMemoryManager must have embedding capability with methods: `with_embeddings`, `search_by_text`, `query_by_metadata`)

## Core Architecture Change:
The existing code uses a layered approach:
```
memory_adapter.rs → MemoryCoordinator → MemoryManager → SurrealDB
                          (cache layer)
```

After refactoring:
```
memory_adapter.rs → SurrealDBMemoryManager → SurrealDB
                    (direct database access)
```

## Implementation Details:

### SUBTASK1: Update imports
**Location:** `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/context/memory_adapter.rs` (lines 1-20)

**Current imports to remove:**
```rust
use paraphym_candle::memory::core::manager::coordinator::MemoryCoordinator;
```

**New imports to add:**
```rust
use paraphym_candle::memory::core::manager::surreal::SurrealDBMemoryManager;
use std::collections::HashMap;
use futures::StreamExt; // For stream handling from async methods
use serde_json::json;
```

### SUBTASK2: Update struct field
**Location:** `memory_adapter.rs` (line ~21)

**Current structure:**
```rust
pub struct MemoryContextAdapter {
    memory_coordinator: Arc<MemoryCoordinator>,
    subscriptions: Arc<RwLock<Vec<String>>>,
}
```

**New structure:**
```rust
#[derive(Clone)]
pub struct MemoryContextAdapter {
    memory_manager: Arc<SurrealDBMemoryManager>,
    subscriptions: Arc<RwLock<Vec<String>>>,
}
```

### SUBTASK3: Fix initialization in new()
**Location:** `memory_adapter.rs` (lines 80-95)

**Key changes:**
- Initialize SurrealDB directly
- Create SurrealDBMemoryManager with embeddings support
- Remove MemoryCoordinator wrapping

**Implementation pattern based on [./src/memory/core/manager/surreal.rs](../packages/candle/src/memory/core/manager/surreal.rs):
```rust
pub async fn new() -> Result<Self> {
    // Initialize SurrealDB with SurrealKV backend
    let db = surrealdb::Surreal::new::<surrealdb::engine::any::Any>(
        "surrealkv://./data/context_memory.db"
    ).await
    .map_err(|e| anyhow::anyhow!("Failed to connect to SurrealDB: {}", e))?;
    
    // Use namespace and database
    db.use_ns("context").use_db("mcp").await
        .map_err(|e| anyhow::anyhow!("Failed to select namespace/database: {}", e))?;
    
    // Create SurrealDBMemoryManager with embeddings (assumes MEMFIX_1/2 completed)
    let manager = SurrealDBMemoryManager::with_embeddings(db).await
        .map_err(|e| anyhow::anyhow!("Failed to create manager with embeddings: {}", e))?;
    
    // Initialize schema and indexes
    manager.initialize().await
        .map_err(|e| anyhow::anyhow!("Failed to initialize schema: {}", e))?;
    
    Ok(Self {
        memory_manager: Arc::new(manager),
        subscriptions: Arc::new(RwLock::new(Vec::new())),
    })
}
```

### SUBTASK4: Update store_context() method
**Location:** `memory_adapter.rs` (lines 99-137)

**Method mapping:**
- `MemoryCoordinator::add_memory()` → `SurrealDBMemoryManager::create_memory()`
- `MemoryCoordinator::update_memory()` → `SurrealDBMemoryManager::update_memory()`

**Implementation using actual SurrealDBMemoryManager API:**
```rust
pub async fn store_context(&self, key: String, value: Value) -> Result<()> {
    let json_str = serde_json::to_string(&value)?;
    
    // Check if context exists using direct database query
    let existing = self.find_context_memory(&key).await?;
    
    if let Some(mut existing_memory) = existing {
        // Update existing memory node
        existing_memory.content = paraphym_candle::memory::core::primitives::types::MemoryContent::new(&json_str);
        existing_memory.metadata.custom.insert(
            "updated_at".to_string(), 
            json!(chrono::Utc::now().timestamp_millis())
        );
        
        // Use update_memory which returns PendingMemory (Future)
        self.memory_manager
            .update_memory(existing_memory)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to update context: {}", e))?;
    } else {
        // Create new memory node with metadata
        let mut memory = paraphym_candle::domain::memory::primitives::node::MemoryNode::new(
            paraphym_candle::domain::memory::primitives::types::MemoryTypeEnum::Semantic,
            paraphym_candle::domain::memory::primitives::types::MemoryContent::text(json_str)
        );
        
        // Set custom metadata for context identification
        memory.metadata.custom.insert("type".to_string(), json!("context"));
        memory.metadata.custom.insert("key".to_string(), json!(key));
        memory.metadata.importance = 1.0;
        
        // Use create_memory which returns PendingMemory (Future)
        self.memory_manager
            .create_memory(memory)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create context: {}", e))?;
    }
    
    Ok(())
}
```

### SUBTASK5: Fix find_context_memory() to query database
**Location:** `memory_adapter.rs` (lines 150-163)

**Current issue:** Uses MemoryCoordinator::get_memories() which only checks cache
**Solution:** Use SurrealDBMemoryManager::query_by_metadata() to directly query database

**Implementation pattern (assumes MEMFIX_2 adds query_by_metadata):**
```rust
async fn find_context_memory(&self, key: &str) -> Result<Option<paraphym_candle::domain::memory::primitives::node::MemoryNode>> {
    // Build metadata filters for query
    let mut filters = HashMap::new();
    filters.insert("type".to_string(), json!("context"));
    filters.insert("key".to_string(), json!(key));
    
    // Query database directly using metadata filters
    // query_by_metadata returns a MemoryStream
    let mut stream = self.memory_manager
        .query_by_metadata(filters)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to query memory: {}", e))?;
    
    // Get first matching result from stream
    if let Some(result) = stream.next().await {
        match result {
            Ok(memory) => Ok(Some(memory)),
            Err(e) => Err(anyhow::anyhow!("Failed to read memory: {}", e))
        }
    } else {
        Ok(None)
    }
}
```

### SUBTASK6: Update search_contexts() method
**Location:** `memory_adapter.rs` (lines 186-218)

**Method mapping:**
- `MemoryCoordinator::search_memories()` → `SurrealDBMemoryManager::search_by_text()`

**Implementation pattern (assumes MEMFIX_1 adds search_by_text with embeddings):**
```rust
pub async fn search_contexts(&self, pattern: &str, limit: Option<usize>) -> Result<Vec<(String, Value)>> {
    let search_limit = limit.unwrap_or(10).min(100);
    
    // Use semantic search with embeddings (added in MEMFIX_1)
    // search_by_text returns a MemoryStream
    let mut stream = self.memory_manager
        .search_by_text(pattern, search_limit)
        .await
        .map_err(|e| anyhow::anyhow!("Search failed: {}", e))?;
    
    let mut results = Vec::new();
    
    // Process stream of search results
    while let Some(memory_result) = stream.next().await {
        match memory_result {
            Ok(memory) => {
                // Check if this is a context entry
                if let Some(type_val) = memory.metadata.custom.get("type") {
                    if type_val.as_str() == Some("context") {
                        if let Some(key_val) = memory.metadata.custom.get("key") {
                            if let Some(key) = key_val.as_str() {
                                // Parse content back to JSON Value
                                let value: Value = serde_json::from_str(&memory.content.to_string())
                                    .unwrap_or_else(|_| json!(null));
                                results.push((key.to_string(), value));
                            }
                        }
                    }
                }
            }
            Err(e) => {
                log::warn!("Error processing search result: {}", e);
            }
        }
    }
    
    Ok(results)
}
```

### SUBTASK7: Fix get_stats() to query database
**Location:** `memory_adapter.rs` (lines 254-283)

**Implementation using direct database queries:**
```rust
pub async fn get_stats(&self) -> MemoryStats {
    // Build filter for context entries
    let mut filters = HashMap::new();
    filters.insert("type".to_string(), json!("context"));
    
    // Query all context memories (assumes query_by_metadata from MEMFIX_2)
    match self.memory_manager.query_by_metadata(filters).await {
        Ok(mut stream) => {
            let mut total_nodes = 0;
            let mut total_relationships = 0;
            
            // Count nodes and their relationships
            while let Some(result) = stream.next().await {
                if let Ok(memory) = result {
                    total_nodes += 1;
                    
                    // Get relationships for this node
                    // get_relationships returns RelationshipStream
                    let mut rel_stream = self.memory_manager
                        .get_relationships(&memory.id);
                    
                    // Count relationships
                    while let Some(rel_result) = rel_stream.next().await {
                        if rel_result.is_ok() {
                            total_relationships += 1;
                        }
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

### SUBTASK8: Update validate() method
**Location:** `memory_adapter.rs` (around line 240)

**Implementation using direct manager methods:**
```rust
pub async fn validate(&self) -> Result<()> {
    // Test search functionality with timeout
    // search_by_text should be available after MEMFIX_1
    match tokio::time::timeout(
        std::time::Duration::from_secs(5),
        async {
            let mut stream = self.memory_manager
                .search_by_text("__health_check__", 1)
                .await?;
            // Just check if we can execute the search
            stream.next().await;
            Ok::<(), anyhow::Error>(())
        }
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

## Key Implementation Notes:

### Method Mappings (based on actual SurrealDBMemoryManager):
1. **Creating memory**: `create_memory()` returns `PendingMemory` (a Future)
2. **Updating memory**: `update_memory()` returns `PendingMemory` (a Future)  
3. **Getting memory**: `get_memory()` returns `MemoryQuery` (a Future)
4. **Deleting memory**: `delete_memory()` returns `PendingDeletion` (a Future)
5. **Querying by type**: `query_by_type()` returns `MemoryStream`
6. **Getting relationships**: `get_relationships()` returns `RelationshipStream`

### Stream Handling:
All query methods return streams that need to be processed with `StreamExt`:
- Use `stream.next().await` to get individual items
- Handle `Result<T>` for each item from the stream
- Streams are async iterators, perfect for processing large result sets

### Direct Database Access:
- No caching layer - all operations hit SurrealDB directly
- Data persists across restarts
- Real-time consistency without cache invalidation issues

### Error Handling Pattern:
```rust
.map_err(|e| anyhow::anyhow!("Context-specific error: {}", e))?
```

## Definition of Done:
- ✅ MemoryCoordinator is completely removed from imports and struct
- ✅ memory_adapter uses SurrealDBMemoryManager directly  
- ✅ All methods query the database, not cache
- ✅ Data persists across restarts (stored in `./data/context_memory.db`)
- ✅ Code compiles without errors
- ✅ All anyhow::Result error handling is proper
- ✅ Streams are properly handled with `StreamExt`

## Dependencies:
- MEMFIX_1: Must add `with_embeddings()` constructor to SurrealDBMemoryManager
- MEMFIX_2: Must add `search_by_text()` and `query_by_metadata()` methods