# Remove block_on from agent_role.rs:1318 (MEDIUM)

**Location:** `src/builders/agent_role.rs:1318`

**Priority:** MEDIUM - Part of same memory initialization refactoring as lines 1287, 1297

## Current Code

```rust
// Ingest documents from context fields into memory
if let Some(ref mem_mgr) = memory {
    use crate::memory::primitives::node::MemoryNode;
    use crate::memory::primitives::types::{MemoryContent, MemoryTypeEnum};
    use chrono::Utc;
    
    let Some(runtime) = crate::runtime::shared_runtime() else {
        return Err(AgentError::MemoryInit("Failed to access shared runtime for document ingestion".into()));
    };
    
    runtime.block_on(async {
        // Load from context_file
        if let Some(ctx) = context_file {
            let docs = ctx.load().collect();
            for doc in docs {
                let content_hash = crate::domain::memory::serialization::content_hash(&doc.data);
                let memory = MemoryNode {
                    id: format!("doc_{}", content_hash),
                    content: MemoryContent::new(&doc.data),
                    content_hash,
                    memory_type: MemoryTypeEnum::Semantic,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    embedding: None,
                    evaluation_status: crate::memory::monitoring::operations::OperationStatus::Pending,
                    metadata: crate::memory::primitives::metadata::MemoryMetadata::new(),
                    relevance_score: None,
                };
                if let Err(e) = mem_mgr.create_memory(memory).await {
                    log::error!("Failed to ingest document: {:?}", e);
                }
            }
        }
        // ... similar for context_files, context_directory, context_github ...
    });
}
```

Context: After memory manager initialization, still in sync `chat()` builder method.

## Problem: Eager Document Ingestion During Build

The code **eagerly blocks** to ingest all context documents during builder setup. This:
1. Forces synchronous document loading before stream consumption
2. Loads and processes all documents even if stream is never consumed
3. Uses shared_runtime().block_on() risking nested runtime errors
4. Can be slow for large document sets, blocking the builder unnecessarily

## Solution: Move Document Ingestion Inside AsyncStream

This is **part of the same refactoring** as lines 1287 and 1297. Move ALL document ingestion inside the AsyncStream:

```rust
Ok(AsyncStream::with_channel(move |sender| async move {
    // Database connection and memory manager initialization (from previous fixes)
    // ... db connection code ...
    // ... memory manager creation code ...
    
    let memory_manager = Arc::new(manager);
    
    // Document ingestion - use .await instead of block_on
    use crate::memory::primitives::node::MemoryNode;
    use crate::memory::primitives::types::{MemoryContent, MemoryTypeEnum};
    use chrono::Utc;
    
    // Load from context_file
    if let Some(ctx) = context_file {
        let docs = ctx.load().collect();
        for doc in docs {
            let content_hash = crate::domain::memory::serialization::content_hash(&doc.data);
            let memory = MemoryNode {
                id: format!("doc_{}", content_hash),
                content: MemoryContent::new(&doc.data),
                content_hash,
                memory_type: MemoryTypeEnum::Semantic,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                embedding: None,
                evaluation_status: crate::memory::monitoring::operations::OperationStatus::Pending,
                metadata: crate::memory::primitives::metadata::MemoryMetadata::new(),
                relevance_score: None,
            };
            if let Err(e) = memory_manager.create_memory(memory).await {
                log::error!("Failed to ingest document from context_file: {:?}", e);
            }
        }
    }
    
    // Similar for context_files, context_directory, context_github...
    
    // Now proceed with actual agent logic...
}))
```

**Pattern Explanation:**
- **ANTIPATTERN (current):** `runtime.block_on(async { for doc in docs { mgr.create(doc).await } });` before stream creation
- **CORRECT (fix):** Inside AsyncStream: `for doc in docs { mgr.create(doc).await; }`

## Implementation Notes

1. **Must be fixed together** with BLOCK_ON_AGENT_ROLE_1287.md and BLOCK_ON_AGENT_ROLE_1297.md
2. Remove runtime.block_on() wrapper entirely
3. Use direct `.await` on `create_memory()` calls
4. Keep error logging with log::error - don't fail stream on individual document errors
5. This makes document loading truly lazy - only happens when stream is consumed
6. Consider adding progress messages via sender for long-running document ingestion

## Dependencies

- Part of unified memory initialization refactoring
- Lines 1287, 1297, 1318 must all move inside AsyncStream together
- The entire memory initialization (1268-1377) should become part of the AsyncStream

## Benefits

1. Lazy initialization - documents only load when stream is consumed
2. Proper async without blocking
3. No nested runtime risk
4. Progress can be reported via stream chunks during ingestion
