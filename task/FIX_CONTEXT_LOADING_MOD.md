# FIX_CONTEXT_LOADING_MOD: Complete Context Loading in mod.rs

## OBJECTIVE
Complete the context loading implementation in builders/agent_role/chat/mod.rs by:
1. Removing underscore prefixes from context variables
2. Calling load_context_into_memory after memory initialization
3. Converting context sources to SessionDocument for session executor

## STATUS
❌ **INCOMPLETE** - Context loading stubbed, not implemented

## DEPENDENCIES
⚠️ **REQUIRES**: FIX_MEMORY_OPS_API.md must be completed first (load_context_into_memory must work)

## CURRENT STATE

### Issue 1: Context Variables Unused (Lines 138-143)
```rust
// CURRENT - variables prefixed with underscore (unused)
let _context_file = self.context_file;
let _context_files = self.context_files;
let _context_directory = self.context_directory;
let _context_github = self.context_github;
```

### Issue 2: load_context_into_memory Not Called (After Line 156)
After memory initialization succeeds, the context is never loaded into memory:
```rust
let memory = if let Some(ref emb_model) = embedding_model {
    match memory_ops::initialize_memory_coordinator(emb_model).await {
        Ok(mgr) => mgr,
        Err(e) => {
            let _ = sender.send(CandleMessageChunk::Error(e));
            return;
        }
    }
} else {
    let _ = sender.send(CandleMessageChunk::Error(
        "Embedding model required for memory system".to_string()
    ));
    return;
};

// MISSING: Should call load_context_into_memory here!
```

### Issue 3: context_documents Stubbed (Line 162)
```rust
// CURRENT - empty stub
let context_documents = Vec::new();

// REQUIRED - Should collect from context sources and convert to SessionDocument
```

## IMPLEMENTATION PLAN

### Fix 1: Remove Underscore Prefixes (Lines 138-143)

**CHANGE FROM**:
```rust
let _context_file = self.context_file;
let _context_files = self.context_files;
let _context_directory = self.context_directory;
let _context_github = self.context_github;
```

**CHANGE TO**:
```rust
let context_file = self.context_file;
let context_files = self.context_files;
let context_directory = self.context_directory;
let context_github = self.context_github;
```

### Fix 2: Call load_context_into_memory (After Line 156)

**INSERT AFTER** memory initialization block:
```rust
// Load context into memory
if let Err(e) = memory_ops::load_context_into_memory(
    &memory,
    context_file.clone(),
    context_files.clone(),
    context_directory.clone(),
    context_github.clone(),
).await {
    let _ = sender.send(CandleMessageChunk::Error(e));
    return;
}
```

### Fix 3: Populate context_documents (Replace Line 162)

**REPLACE**:
```rust
let context_documents = Vec::new();
```

**WITH**:
```rust
// Convert context sources to SessionDocument
let mut context_documents = Vec::new();

// Helper to collect documents from stream
async fn collect_docs(
    ctx_stream: std::pin::Pin<Box<dyn tokio_stream::Stream<Item = crate::domain::context::CandleDocument> + Send>>,
    source_label: &str,
) -> Vec<crate::domain::chat::session::SessionDocument> {
    use tokio_stream::StreamExt;
    let mut docs = Vec::new();
    let mut stream = ctx_stream;
    
    while let Some(doc) = stream.next().await {
        let source = doc.additional_props.get("source")
            .and_then(|v| v.as_str())
            .unwrap_or(source_label)
            .to_string();
            
        let tags = doc.additional_props.get("tags")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect())
            .unwrap_or_default();
            
        docs.push(crate::domain::chat::session::SessionDocument {
            content: doc.data,
            source,
            tags,
        });
    }
    docs
}

// Collect from all context sources
if let Some(ctx) = context_file {
    context_documents.extend(collect_docs(ctx.load(), "context_file").await);
}
if let Some(ctx) = context_files {
    context_documents.extend(collect_docs(ctx.load(), "context_files").await);
}
if let Some(ctx) = context_directory {
    context_documents.extend(collect_docs(ctx.load(), "context_directory").await);
}
if let Some(ctx) = context_github {
    context_documents.extend(collect_docs(ctx.load(), "context_github").await);
}
```

## DATA FLOW

```
CandleContext<T> sources (file/files/directory/github)
  ↓
  .load() → Stream<Item = CandleDocument>
  ↓
  collect via StreamExt::next()
  ↓
  CandleDocument { data, additional_props, ... }
  ↓
  map to SessionDocument { content, source, tags }
  ↓
  Vec<SessionDocument> → passed to execute_chat_session
  ↓
  session.rs loads into memory and uses in prompt
```

## SessionDocument STRUCTURE

From domain/chat/session.rs:
```rust
pub struct SessionDocument {
    pub content: String,  // from CandleDocument.data
    pub source: String,   // from additional_props["source"] or default
    pub tags: Vec<String>, // from additional_props["tags"] or empty
}
```

## VERIFICATION

After fixes:
```bash
# Should compile without errors in mod.rs
cargo check -p paraphym_candle 2>&1 | grep "builders/agent_role/chat/mod.rs"

# Test that context loads
cargo run --bin candle-chat -- --help
```

## DEFINITION OF DONE

- [ ] Context variables no longer prefixed with underscore
- [ ] load_context_into_memory called after memory initialization
- [ ] context_documents populated from context sources using streaming API
- [ ] CandleDocument properly converted to SessionDocument
- [ ] Source extracted from additional_props or defaults to source type
- [ ] Tags extracted from additional_props or defaults to empty vec
- [ ] Code compiles: `cargo check -p paraphym_candle`
- [ ] No unused variable warnings for context sources
