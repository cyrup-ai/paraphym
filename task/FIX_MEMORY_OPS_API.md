# FIX_MEMORY_OPS_API: Fix CandleContext API Mismatches in memory_ops.rs

## OBJECTIVE
Fix memory_ops.rs to use the correct streaming CandleContext API instead of the non-existent get_documents() method.

## STATUS
❌ **BLOCKED** - 6 compilation errors preventing build

## ERRORS TO FIX

### Error 1-4: CandleContext doesn't have get_documents() method
**Locations**: 
- Line 68: `ctx.get_documents().await` (context_file)
- Line 88: `ctx.get_documents().await` (context_files)  
- Line 108: `ctx.get_documents().await` (context_directory)
- Line 128: `ctx.get_documents().await` (context_github)

**Error Message**:
```
error[E0599]: no method named `get_documents` found for struct `context_impl::CandleContext<T>`
```

**Root Cause**: 
- Code expects: `ctx.get_documents().await -> Result<Vec<Document>, Error>`
- Actual API: `ctx.load() -> Pin<Box<dyn Stream<Item = CandleDocument> + Send>>`

### Error 5-6: SurrealDBMemoryManager::with_embedding_model returns Self, not Result
**Location**: Lines 39-42

**Current Code**:
```rust
let surreal_manager = match SurrealDBMemoryManager::with_embedding_model(db, emb_model.clone()) {
    Ok(mgr) => mgr,
    Err(e) => return Err(format!("Failed to create memory manager: {}", e)),
};
```

**Error Message**:
```
error[E0308]: mismatched types
  --> packages/candle/src/builders/agent_role/chat/memory_ops.rs:40:9
   |
39 |     let surreal_manager = match SurrealDBMemoryManager::with_embedding_model(...) {
   |                                 ------------------------------------------------------------------- this expression has type `SurrealDBMemoryManager`
40 |         Ok(mgr) => mgr,
   |         ^^^^^^ expected `SurrealDBMemoryManager`, found `Result<_, _>`
```

## IMPLEMENTATION PLAN

### Step 1: Fix SurrealDBMemoryManager Instantiation (Lines 39-42)

**CHANGE FROM**:
```rust
let surreal_manager = match SurrealDBMemoryManager::with_embedding_model(db, emb_model.clone()) {
    Ok(mgr) => mgr,
    Err(e) => return Err(format!("Failed to create memory manager: {}", e)),
};
```

**CHANGE TO**:
```rust
let surreal_manager = SurrealDBMemoryManager::with_embedding_model(db, emb_model.clone());
```

### Step 2: Rewrite load_context_into_memory Function (Lines 59-147)

Replace entire function with streaming API implementation:

```rust
pub(super) async fn load_context_into_memory(
    memory: &Arc<dyn crate::memory::core::manager::surreal::MemoryManager>,
    context_file: Option<CandleContext<CandleFile>>,
    context_files: Option<CandleContext<CandleFiles>>,
    context_directory: Option<CandleContext<CandleDirectory>>,
    context_github: Option<CandleContext<CandleGithub>>,
) -> Result<(), String> {
    use tokio_stream::StreamExt;

    // Helper to process document stream into memory
    async fn process_stream(
        memory: &Arc<dyn crate::memory::core::manager::surreal::MemoryManager>,
        mut doc_stream: std::pin::Pin<Box<dyn tokio_stream::Stream<Item = crate::domain::context::CandleDocument> + Send>>,
        source_label: &str,
    ) {
        while let Some(doc) = doc_stream.next().await {
            let content = MemoryContent::new(&doc.data);
            let mut node = MemoryNode::new(MemoryTypeEnum::Semantic, content);
            
            // Extract source from additional_props or use default
            let source = doc.additional_props.get("source")
                .and_then(|v| v.as_str())
                .unwrap_or(source_label)
                .to_string();
            node.metadata.source = Some(source);
            
            // Extract tags from additional_props if available
            if let Some(tags_value) = doc.additional_props.get("tags") {
                if let Some(tags_array) = tags_value.as_array() {
                    node.metadata.tags = tags_array
                        .iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect();
                }
            }
            
            node.metadata.importance = 0.5;
            
            let pending = memory.create_memory(node);
            if let Err(e) = pending.await {
                log::warn!("Failed to ingest document into memory: {:?}", e);
            }
        }
    }

    // Load from context_file
    if let Some(ctx) = context_file {
        let stream = ctx.load();
        process_stream(memory, stream, "context_file").await;
    }

    // Load from context_files
    if let Some(ctx) = context_files {
        let stream = ctx.load();
        process_stream(memory, stream, "context_files").await;
    }

    // Load from context_directory
    if let Some(ctx) = context_directory {
        let stream = ctx.load();
        process_stream(memory, stream, "context_directory").await;
    }

    // Load from context_github
    if let Some(ctx) = context_github {
        let stream = ctx.load();
        process_stream(memory, stream, "context_github").await;
    }

    Ok(())
}
```

## API DIFFERENCES

### Old (Non-existent) API:
```rust
// This doesn't exist!
let docs = ctx.get_documents().await?;
// Returns: Result<Vec<DocumentWithContentSourceTags>, Error>
```

### New (Actual) API:
```rust
// This is the real API
let stream = ctx.load();
// Returns: Pin<Box<dyn Stream<Item = CandleDocument> + Send>>

// CandleDocument structure:
pub struct CandleDocument {
    pub data: String,  // NOT "content"!
    pub format: Option<CandleContentFormat>,
    pub media_type: Option<CandleDocumentMediaType>,
    pub additional_props: HashMap<String, Value>,  // source/tags go here
}
```

## VERIFICATION

After fixes:
```bash
cargo check -p paraphym_candle 2>&1 | grep "memory_ops.rs"
```

Should show ZERO errors in memory_ops.rs.

## DEFINITION OF DONE

- [ ] SurrealDBMemoryManager instantiation fixed (no Result pattern match)
- [ ] load_context_into_memory uses ctx.load() streaming API
- [ ] Documents collected from streams using StreamExt::next()
- [ ] CandleDocument.data mapped to MemoryContent
- [ ] Source/tags extracted from additional_props HashMap
- [ ] All 6 compilation errors in memory_ops.rs resolved
- [ ] Code compiles: `cargo check -p paraphym_candle`
