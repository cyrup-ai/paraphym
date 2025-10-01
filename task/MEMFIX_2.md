# MEMFIX_2: Implement Embedding Methods in SurrealDBMemoryManager

## OBJECTIVE:
Add auto-embedding generation to create_memory() and implement new search_by_text() and query_by_metadata() methods for database-backed operations.

## PREREQUISITE:
- MEMFIX_1 must be completed (VectorSearch fields added to struct)

## SUBTASK1: Modify create_memory() for auto-embedding
**Location:** `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/core/manager/surreal.rs` (lines ~540-570)

Update the create_memory() method to auto-generate embeddings when missing:
```rust
pub async fn create_memory(&self, mut memory: MemoryNode) -> Result<MemoryNode> {
    // Auto-generate embedding if missing
    if memory.embedding.is_none() {
        if let Some(ref embedding_model) = self.embedding_model {
            let content_text = memory.content.to_string();
            let embedding_vec = embedding_model.embed(
                &content_text,
                Some("search".to_string())
            )?;
            memory.embedding = Some(embedding_vec);
        }
    }
    
    // Continue with existing storage logic...
    let memory_schema = MemoryNodeSchema::from_memory_node(&memory)?;
    let created: MemoryNodeSchema = self
        .db
        .create("memory")
        .content(memory_schema)
        .await
        .map_err(|e| Error::Database(format!("{:?}", e)))?;
    
    Ok(Self::from_schema(created))
}
```

## SUBTASK2: Implement search_by_text() method
**Location:** `surreal.rs` (add after search_by_vector method around line 870)

Add semantic search with auto-embedding:
```rust
pub async fn search_by_text(
    &self,
    text: &str,
    limit: usize
) -> Result<MemoryStream> {
    // Generate embedding from text
    if let Some(ref embedding_model) = self.embedding_model {
        let embedding = embedding_model.embed(
            text,
            Some("search".to_string())
        )?;
        // Reuse existing search_by_vector
        self.search_by_vector(embedding, limit).await
    } else {
        Err(Error::Configuration(
            "No embedding model available for text search".to_string()
        ))
    }
}
```

## SUBTASK3: Implement query_by_metadata() method
**Location:** `surreal.rs` (add after query_by_type method around line 800)

Add metadata-based database query:
```rust
pub async fn query_by_metadata(
    &self,
    metadata_filters: HashMap<String, serde_json::Value>
) -> Result<MemoryStream> {
    // Build parameterized query to prevent SQL injection
    let mut query_parts = Vec::new();
    let mut bind_values = Vec::new();
    
    for (idx, (key, value)) in metadata_filters.iter().enumerate() {
        query_parts.push(format!("metadata.custom.{} = ${}", key, idx));
        bind_values.push((idx.to_string(), value.clone()));
    }
    
    let where_clause = if query_parts.is_empty() {
        String::new()
    } else {
        format!(" WHERE {}", query_parts.join(" AND "))
    };
    
    let query_str = format!("SELECT * FROM memory{}", where_clause);
    
    // Execute query with bindings
    let mut query = self.db.query(&query_str);
    for (key, value) in bind_values {
        query = query.bind((key, value));
    }
    
    let result: Vec<MemoryNodeSchema> = query
        .await
        .map_err(|e| Error::Database(format!("{:?}", e)))?
        .take(0)?;
    
    // Convert to stream
    let memories: Vec<Result<MemoryNode>> = result
        .into_iter()
        .map(|schema| Ok(Self::from_schema(schema)))
        .collect();
    
    Ok(Box::pin(futures::stream::iter(memories)))
}
```

## SUBTASK4: Add helper method for batch ID fetching
**Location:** `surreal.rs` (add as private method)

Add helper for fetching multiple memories by IDs:
```rust
async fn get_memories_by_ids(&self, ids: Vec<String>) -> Result<Vec<Result<MemoryNode>>> {
    let mut memories = Vec::new();
    for id in ids {
        let memory_result = self.get_memory(&id).await;
        memories.push(memory_result.and_then(|opt| 
            opt.ok_or_else(|| Error::NotFound(format!("Memory {} not found", id)))
        ));
    }
    Ok(memories)
}
```

## SUBTASK5: Update module exports
**Location:** `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/core/manager/mod.rs`

Export the new methods:
```rust
pub use surreal::{
    SurrealDBMemoryManager,
    // ADD:
    search_by_text,
    query_by_metadata,
};
```

## Definition of Done:
- create_memory() auto-generates embeddings when not provided
- search_by_text() performs semantic search using embedding model
- query_by_metadata() queries database with metadata filters
- All methods handle errors properly without panic
- Module exports are updated
- Code compiles successfully

## Research Notes:
- MemoryNode embedding field is `Option<Vec<f32>>` type
- EmbeddingModel trait's `embed()` method signature: `fn embed(&self, text: &str, task: Option<String>) -> Result<Vec<f32>>`
- MemoryStream type is `Pin<Box<dyn Stream<Item = Result<MemoryNode>>>>`
- MemoryCoordinator at `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/core/manager/coordinator.rs` shows the embedding pattern

## Requirements:
- DO NOT write any unit tests
- DO NOT write any benchmarks
- DO NOT create any documentation files
- Use proper Result error handling, no unwrap() or expect()
- Maintain backward compatibility with existing methods