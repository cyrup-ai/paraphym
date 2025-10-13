# Remove block_on from agent_role.rs:1297 (MEDIUM)

**Location:** `src/builders/agent_role.rs:1297`

**Priority:** MEDIUM - Part of same memory initialization refactoring as line 1287

## Current Code

```rust
let manager = runtime.block_on(async {
    let mgr = SurrealDBMemoryManager::with_embedding_model(db, embedding_model.clone()).await
        .map_err(|e| AgentError::MemoryInit(format!("Failed to create memory manager: {}", e)))?;
    mgr.initialize().await
        .map_err(|e| AgentError::MemoryInit(format!("Failed to initialize memory tables: {}", e)))?;
    Ok::<_, AgentError>(mgr)
})?;
```

Context: Immediately after line 1287's database connection, still in the sync `chat()` builder method.

## Problem: Eager Memory Manager Initialization

The code **eagerly blocks** to create and initialize the memory manager during builder setup. This:
1. Forces synchronous execution before stream consumption
2. Initializes resources that may never be used
3. Uses shared_runtime().block_on() risking nested runtime errors

## Solution: Move Inside AsyncStream with Database Connection

This is **part of the same refactoring** as BLOCK_ON_AGENT_ROLE_1287.md. When moving database connection inside AsyncStream, also move memory manager creation:

```rust
Ok(AsyncStream::with_channel(move |sender| async move {
    // Database connection (from line 1287 fix)
    let db = match connect(&db_url).await {
        Ok(db) => db,
        Err(e) => {
            let _ = sender.send(CandleMessageChunk::Error(
                format!("Failed to connect to database: {}", e)
            ));
            return;
        }
    };
    
    if let Err(e) = db.use_ns("candle").use_db("agent").await {
        let _ = sender.send(CandleMessageChunk::Error(
            format!("Failed to initialize database namespace: {}", e)
        ));
        return;
    }
    
    // Memory manager initialization - use .await instead of block_on
    let manager = match SurrealDBMemoryManager::with_embedding_model(db, embedding_model.clone()).await {
        Ok(mgr) => mgr,
        Err(e) => {
            let _ = sender.send(CandleMessageChunk::Error(
                format!("Failed to create memory manager: {}", e)
            ));
            return;
        }
    };
    
    if let Err(e) = manager.initialize().await {
        let _ = sender.send(CandleMessageChunk::Error(
            format!("Failed to initialize memory tables: {}", e)
        ));
        return;
    }
    
    // Continue with document ingestion and stream execution...
}))
```

**Pattern Explanation:**
- **ANTIPATTERN (current):** `let mgr = runtime.block_on(async { create_and_init().await })?;` before stream creation
- **CORRECT (fix):** Inside AsyncStream: `let mgr = create().await?; mgr.init().await?;`

## Implementation Notes

1. **Must be fixed together** with BLOCK_ON_AGENT_ROLE_1287.md and BLOCK_ON_AGENT_ROLE_1318.md
2. Remove runtime.block_on() wrapper entirely
3. Use direct `.await` on async operations
4. Convert Result handling from `?` to match for better error messages via sender
5. Memory manager becomes Arc<> after this point, wrap appropriately

## Dependencies

- Part of unified memory initialization refactoring
- Lines 1287, 1297, 1318 must all move inside AsyncStream together
