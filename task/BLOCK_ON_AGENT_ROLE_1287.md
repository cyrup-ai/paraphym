# Remove block_on from agent_role.rs:1287 (MEDIUM)

**Location:** `src/builders/agent_role.rs:1287`

**Priority:** MEDIUM - In sync builder method, but should defer initialization to stream execution

## Current Code

```rust
fn chat<F>(self, handler: F) -> Result<AsyncStream<CandleMessageChunk>, AgentError>
where
    F: FnOnce(&CandleAgentConversation) -> CandleChatLoop + Send + 'static,
{
    // ...
    let Some(runtime) = crate::runtime::shared_runtime() else {
        return Err(AgentError::MemoryInit("Failed to access shared runtime for memory creation".into()));
    };
    
    let db_url = format!("surrealkv://{}", db_path.display());
    let db = runtime.block_on(async {
        let db = connect(&db_url).await.map_err(|e| 
            AgentError::MemoryInit(format!("Failed to connect to database: {}", e))
        )?;
        db.use_ns("candle").use_db("agent").await.map_err(|e|
            AgentError::MemoryInit(format!("Failed to initialize database namespace: {}", e))
        )?;
        Ok::<_, AgentError>(db)
    })?;
    // ...
}
```

Context: This is in the `chat()` builder method at line 1246 which returns `Result<AsyncStream>` - it's NOT an async function.

## Problem: Eager Database Connection During Build

The code **eagerly connects** to the database during the builder's `chat()` method, before any stream execution begins. This:
1. Forces synchronous blocking during builder setup
2. Connects to database even if the stream is never consumed
3. Uses shared_runtime().block_on() risking nested runtime errors
4. Violates lazy evaluation - streams should initialize resources when consumed, not during creation

## Solution: Defer Initialization to Stream Execution

Move the database connection inside the AsyncStream so it happens lazily when the stream is actually consumed:

```rust
fn chat<F>(self, handler: F) -> Result<AsyncStream<CandleMessageChunk>, AgentError>
where
    F: FnOnce(&CandleAgentConversation) -> CandleChatLoop + Send + 'static,
{
    // ... extract fields from self ...
    
    // Don't connect to database here - just prepare the config
    let db_path = dirs::cache_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("paraphym")
        .join("agent.db");
    
    if let Some(parent) = db_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    
    let db_url = format!("surrealkv://{}", db_path.display());
    
    // Return stream that connects lazily
    Ok(AsyncStream::with_channel(move |sender| async move {
        // Connect to database when stream is consumed
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
        
        // Continue with memory manager initialization...
        // (This also requires moving lines 1297 and 1318 inside this async block)
    }))
}
```

**Pattern Explanation:**
- **ANTIPATTERN (current):** Builder eagerly blocks: `let db = runtime.block_on(connect()); return Ok(stream)`
- **CORRECT (fix):** Builder returns stream that connects lazily: `Ok(AsyncStream::with_channel(|s| async move { let db = connect().await; ... }))`

## Implementation Notes

1. This affects lines 1287, 1297, and 1318 - ALL memory initialization must move inside AsyncStream
2. Remove all shared_runtime() usage in this method
3. The entire memory initialization block (lines 1268-1306) should move inside AsyncStream
4. Document ingestion (lines 1308-1377) also needs to move inside AsyncStream
5. This is a significant refactoring - the stream creation logic changes substantially
6. Benefits: lazy initialization, proper async, no nested runtime risk

## Related Tasks

- BLOCK_ON_AGENT_ROLE_1297.md - Memory manager initialization (same refactoring)
- BLOCK_ON_AGENT_ROLE_1318.md - Document ingestion (same refactoring)

These three must be fixed together as part of one refactoring effort.
