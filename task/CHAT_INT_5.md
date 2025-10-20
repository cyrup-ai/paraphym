# CHAT_INT_5: Parallelize Context Document Loading

## OBJECTIVE
Eliminate serial bottleneck in context loading by parallelizing document streams from multiple context sources. Currently all context sources are loaded sequentially, causing severe performance degradation when loading large directories or multiple file patterns.

## SEVERITY: CRITICAL PERFORMANCE DEFECT

**Current Behavior**: Context sources are loaded serially in session.rs:
```rust
// Lines 113-143: Load context_file (BLOCKS)
// Lines 145-175: Load context_files (WAITS for context_file to finish)
// Lines 177-207: Load context_directory (WAITS for context_files to finish)
// Lines 209-239: Load context_github (WAITS for context_directory to finish)
```

**Impact**:
- Loading 4 context sources with 100ms each = 400ms total (serial)
- Should be ~100ms total with parallel loading
- **4x performance penalty** for every chat session initialization
- Scales linearly with number of sources (unacceptable)
- User-facing latency before first response

## CURRENT STATUS

**File**: `packages/candle/src/domain/chat/session.rs`
**Lines**: 113-239

**Issues**:
1. ❌ Context sources loaded one after another
2. ❌ No concurrent processing of independent streams
3. ❌ Blocking behavior prevents session from starting until ALL context loaded
4. ❌ Large directories can block session for seconds

## REQUIRED IMPLEMENTATION

### Parallel Context Loading Pattern

Replace serial loading (lines 113-239) with parallel execution:

```rust
// Spawn parallel tasks for each context source
let mut load_tasks = Vec::new();

// Load context_file in parallel
if let Some(ctx) = context_file {
    let memory = memory.clone();
    let metadata = metadata.clone();
    let task = tokio::spawn(async move {
        let stream = ctx.load();
        tokio::pin!(stream);
        while let Some(doc) = stream.next().await {
            let doc_meta = MemoryMetadata {
                user_id: metadata.get("user_id").cloned(),
                agent_id: metadata.get("agent_id").cloned(),
                context: "session_context".to_string(),
                importance: 0.5,
                keywords: vec![],
                tags: vec!["context_file".to_string()],
                category: "context".to_string(),
                source: doc.additional_props.get("path")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                created_at: chrono::Utc::now(),
                last_accessed_at: None,
                embedding: None,
                custom: serde_json::Value::Object(serde_json::Map::new()),
            };

            if let Err(e) = memory.add_memory(
                doc.data,
                MemoryTypeEnum::Semantic,
                Some(doc_meta)
            ).await {
                log::warn!("Failed to load context document: {:?}", e);
            }
        }
    });
    load_tasks.push(task);
}

// Repeat for context_files, context_directory, context_github
// (same pattern as above)

// Wait for all context loading tasks to complete
for task in load_tasks {
    if let Err(e) = task.await {
        log::warn!("Context loading task failed: {:?}", e);
    }
}
```

### Helper Function to Reduce Duplication

Create helper function to eliminate code duplication (128 lines → ~40 lines):

```rust
async fn load_context_source<T>(
    ctx: CandleContext<T>,
    memory: Arc<MemoryCoordinator>,
    metadata: HashMap<String, String>,
    context_type_tag: &str,
) where
    T: Send + 'static,
{
    let stream = ctx.load();
    tokio::pin!(stream);
    while let Some(doc) = stream.next().await {
        let doc_meta = MemoryMetadata {
            user_id: metadata.get("user_id").cloned(),
            agent_id: metadata.get("agent_id").cloned(),
            context: "session_context".to_string(),
            importance: 0.5,
            keywords: vec![],
            tags: vec![context_type_tag.to_string()],
            category: "context".to_string(),
            source: doc.additional_props.get("path")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            created_at: chrono::Utc::now(),
            last_accessed_at: None,
            embedding: None,
            custom: serde_json::Value::Object(serde_json::Map::new()),
        };

        if let Err(e) = memory.add_memory(
            doc.data,
            MemoryTypeEnum::Semantic,
            Some(doc_meta)
        ).await {
            log::warn!("Failed to load context document: {:?}", e);
        }
    }
}

// Usage:
let mut load_tasks = Vec::new();

if let Some(ctx) = context_file {
    let memory = memory.clone();
    let metadata = metadata.clone();
    load_tasks.push(tokio::spawn(load_context_source(
        ctx, memory, metadata, "context_file"
    )));
}

// Repeat for other sources...
```

## ADDITIONAL FIXES INCLUDED

### Fix 1: Add Context Type Tags
Currently all context documents have empty tags. Add distinguishing tags:
- `"context_file"` for single file contexts
- `"context_files"` for glob pattern contexts
- `"context_directory"` for directory contexts
- `"context_github"` for GitHub repository contexts

### Fix 2: Remove Code Duplication
Replace 128 lines of duplicated code with helper function (see above).

### Fix 3: Remove Dead Code
Remove unused `SessionDocument` struct (lines 46-52).

## PERFORMANCE TARGETS

**Before**:
- 4 context sources @ 100ms each = 400ms serial loading
- Large directory (1000 files) = several seconds blocking

**After**:
- 4 context sources @ 100ms each = ~100ms parallel loading
- Large directory loading in background while session starts
- **4x speedup** for typical use case
- **10-100x better UX** for large context loading

## TESTING

Test with multiple context sources:
```rust
let agent = CandleFluentAi::agent_role("test")
    .model(model)
    .embedding_model(embedding)
    .context(
        CandleContext::of("file1.txt"),
        CandleContext::glob("*.rs"),
        CandleContext::of("./docs"),
        CandleContext::glob("github:owner/repo/**/*.md"),
    )
    .chat(|_| async { CandleChatLoop::UserPrompt("test".to_string()) });
```

Verify:
- ✅ All context sources loaded
- ✅ Loading happens in parallel (measure timing)
- ✅ Session starts without waiting for all context
- ✅ No race conditions in memory storage
- ✅ Error in one source doesn't block others

## DEFINITION OF DONE

- [ ] Context loading parallelized with tokio::spawn
- [ ] Helper function eliminates code duplication
- [ ] Context type tags added for filtering
- [ ] SessionDocument dead code removed
- [ ] Performance test shows 4x speedup
- [ ] No regression in functionality
- [ ] Code compiles: `cargo check -p cyrup_candle`
- [ ] All context sources properly loaded in parallel

## CRITICAL NOTES

**This is NOT an optimization** - it's a critical performance defect. Serial loading:
1. Blocks session initialization unnecessarily
2. Scales poorly with number of sources
3. Prevents responsive user experience
4. Violates async Rust best practices

**Zero tolerance for serial blocking in async code when parallel execution is possible.**
