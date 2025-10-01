# MEMFIX_2: Implement Embedding Methods in SurrealDBMemoryManager

## OBJECTIVE
Add auto-embedding generation to create_memory() and implement new search_by_text() and query_by_metadata() methods for database-backed operations in the SurrealDBMemoryManager.

## PREREQUISITE: MEMFIX_1 Implementation Required
**CRITICAL**: Before implementing this task, the SurrealDBMemoryManager struct must be modified to include the embedding_model field:

```rust
// In /Volumes/samsung_t9/paraphym/packages/candle/src/memory/core/manager/surreal.rs
// Around line 264, modify the struct:

use std::sync::Arc;
use crate::memory::vector::embedding_model::EmbeddingModel;

#[derive(Debug)]
pub struct SurrealDBMemoryManager {
    db: Surreal<Any>,
    embedding_model: Option<Arc<dyn EmbeddingModel>>,  // ADD THIS FIELD
}
```

And update the constructor (around line 276):
```rust
impl SurrealDBMemoryManager {
    /// Create a new SurrealDB memory manager
    pub fn new(db: Surreal<Any>) -> Self {
        Self { 
            db,
            embedding_model: None,  // Initialize as None
        }
    }
    
    /// Add method to set embedding model
    pub fn with_embedding_model(mut self, model: Arc<dyn EmbeddingModel>) -> Self {
        self.embedding_model = Some(model);
        self
    }
```

## SUBTASK1: Modify create_memory() for Auto-Embedding
**Location:** `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/core/manager/surreal.rs`  
**Line Range:** Approximately 540-570 (inside impl MemoryManager for SurrealDBMemoryManager)

### Current Implementation Analysis
The current `create_memory()` method at line 540 creates a MemoryNodeCreateContent from the memory and stores it without generating embeddings. The embedding field is passed through as-is from the input.

### Required Changes
Replace the existing `create_memory` method with:

```rust
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
```

## SUBTASK2: Implement search_by_text() Method  
**Location:** Add after the existing `search_by_vector` implementation (around line 864)

### Implementation Pattern from Existing Code
Based on the existing async pattern and the coordinator.rs example (lines 120-140), implement:

```rust
impl SurrealDBMemoryManager {
    /// Search memories by text with auto-embedding generation
    pub async fn search_by_text(
        &self,
        text: &str,
        limit: usize
    ) -> Result<Pin<Box<dyn Stream<Item = Result<MemoryNode>> + Send>>> {
        // Generate embedding from text
        if let Some(ref embedding_model) = self.embedding_model {
            // Generate embedding synchronously
            let embedding = embedding_model.embed(
                text,
                Some("search".to_string())
            )?;
            
            // Delegate to existing search_by_vector
            let stream = self.search_by_vector(embedding, limit);
            Ok(Box::pin(stream))
        } else {
            Err(Error::Configuration(
                "No embedding model configured for text search".to_string()
            ))
        }
    }
}
```

## SUBTASK3: Implement query_by_metadata() Method
**Location:** Add after the `query_by_type` method (around line 800)

### Implementation Using SurrealDB's Query Capabilities
Based on the existing query patterns in the codebase:

```rust
impl SurrealDBMemoryManager {
    /// Query memories by metadata filters
    pub async fn query_by_metadata(
        &self,
        metadata_filters: HashMap<String, serde_json::Value>
    ) -> Result<Pin<Box<dyn Stream<Item = Result<MemoryNode>> + Send>>> {
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
        
        Ok(Box::pin(MemoryStream::new(rx)))
    }
}
```

## SUBTASK4: Helper Method for Batch Memory Fetching
**Location:** Add as a private method in the impl block (around line 850)

### Implementation for Efficient Batch Operations
```rust
impl SurrealDBMemoryManager {
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
```

## SUBTASK5: Update Module Exports  
**Location:** `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/core/manager/mod.rs`

### Current State (lines 1-8)
```rust
//! Memory management, coordination, and specific implementations

pub mod coordinator;
pub mod surreal;

pub use coordinator::*;
pub use surreal::*;
```

**No changes needed** - The module already exports everything from surreal using `pub use surreal::*;`. The new methods will be automatically available.

## Implementation Dependencies

### Required Imports
Add these imports at the top of surreal.rs if not present:
```rust
use std::collections::HashMap;
use futures_util::Stream;
use std::pin::Pin;
use tracing;  // For logging in create_memory
```

### Existing Code References
- **EmbeddingModel trait**: [./packages/candle/src/memory/vector/embedding_model.rs](./packages/candle/src/memory/vector/embedding_model.rs) (lines 23-387)
- **MemoryCoordinator pattern**: [./packages/candle/src/memory/core/manager/coordinator.rs](./packages/candle/src/memory/core/manager/coordinator.rs) (lines 50-140)
- **MemoryNode structure**: [./packages/candle/src/memory/core/primitives/node.rs](./packages/candle/src/memory/core/primitives/node.rs)
- **Error types**: [./packages/candle/src/memory/utils/error.rs](./packages/candle/src/memory/utils/error.rs)

## Key Implementation Notes

1. **Embedding Generation**: The `EmbeddingModel::embed()` method is synchronous but should be called within the async spawn block for create_memory.

2. **Stream Return Types**: All query methods return `Pin<Box<dyn Stream<Item = Result<MemoryNode>> + Send>>` for consistency with existing patterns.

3. **Error Handling**: Use proper Result types, never unwrap() or expect(). Log warnings for non-critical failures like embedding generation.

4. **SurrealDB Native Features**: The implementation leverages SurrealDB's native vector similarity search via `vector::similarity::cosine()` function (already implemented in search_by_vector at line 862).

5. **Parameter Binding**: Always use parameterized queries with bind() to prevent SQL injection in query_by_metadata.

## Definition of Done

- [x] SurrealDBMemoryManager struct has embedding_model field (MEMFIX_1 prerequisite)
- [ ] create_memory() auto-generates embeddings when not provided and embedding_model is available
- [ ] search_by_text() generates embedding from text and delegates to search_by_vector
- [ ] query_by_metadata() queries database with proper parameter binding
- [ ] get_memories_by_ids() helper efficiently fetches multiple memories
- [ ] All methods use proper error handling without panic
- [ ] Code compiles with `cargo build -p paraphym_candle`
- [ ] No test files, benchmarks, or documentation files created