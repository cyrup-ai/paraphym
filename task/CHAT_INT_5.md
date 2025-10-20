# CHAT_INT_5: Parallelize Context Document Loading

## OBJECTIVE
Eliminate serial bottleneck in context loading by parallelizing document streams from multiple context sources. Currently all context sources are loaded sequentially, causing severe performance degradation when loading large directories or multiple file patterns.

## SEVERITY: CRITICAL PERFORMANCE DEFECT

**Current Behavior**: Context sources are loaded serially in [`session.rs`](../packages/candle/src/domain/chat/session.rs):
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

## EXISTING PARALLEL PATTERNS IN CODEBASE

The codebase already uses `tokio::spawn` for parallel execution in multiple places:

### Pattern 1: Parallel Workflow Execution
**File**: [`workflow/parallel.rs:220-250`](../packages/candle/src/workflow/parallel.rs)
```rust
// Spawn each operation in separate async task
for (op_index, operation) in operations.into_iter().enumerate() {
    let input_clone = input.clone();
    let result_tx_clone = result_tx.clone();

    tokio::spawn(async move {
        // Execute operation and stream all results
        let op_stream = operation.call(input_clone);
        tokio::pin!(op_stream);

        // Stream all results from this operation with index tracking
        while let Some(result) = op_stream.next().await {
            let parallel_result = ParallelResult::new(op_index, result);

            // Send result with operation index for correlation
            if result_tx_clone.send(parallel_result).is_err() {
                break;
            }
        }
    });
}
```

### Pattern 2: Fire-and-Forget Background Tasks
**File**: [`session.rs:510, 522`](../packages/candle/src/domain/chat/session.rs)
```rust
// Background memory storage (already in use)
let memory_clone = memory.clone();
let user_msg = user_message.clone();
tokio::spawn(async move {
    if let Err(e) = memory_clone.add_memory(
        user_msg,
        MemoryTypeEnum::Episodic,
        Some(user_meta)
    ).await {
        log::error!("Failed to store user memory: {:?}", e);
    }
});
```

## IMPLEMENTATION PLAN

### Step 1: Create Helper Function

**File**: `packages/candle/src/domain/chat/session.rs`  
**Location**: After line 76 (after `format_memory_context` function)

Add new helper function to eliminate code duplication:

```rust
/// Load documents from a context stream into memory
async fn load_context_stream(
    stream: Pin<Box<dyn Stream<Item = crate::domain::context::CandleDocument> + Send>>,
    memory: Arc<MemoryCoordinator>,
    metadata: HashMap<String, String>,
    context_tag: &str,
) {
    tokio::pin!(stream);
    while let Some(doc) = stream.next().await {
        let doc_meta = MemoryMetadata {
            user_id: metadata.get("user_id").cloned(),
            agent_id: metadata.get("agent_id").cloned(),
            context: "session_context".to_string(),
            importance: 0.5,
            keywords: vec![],
            tags: vec![context_tag.to_string()],
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
            log::warn!("Failed to load context document from {}: {:?}", context_tag, e);
        }
    }
}
```

### Step 2: Replace Serial Loading with Parallel Execution

**File**: `packages/candle/src/domain/chat/session.rs`  
**Lines to Replace**: 111-239 (entire serial loading block)

**Replace with**:

```rust
// Load context documents from all sources in parallel using tokio::spawn
let mut load_tasks = Vec::new();

// Spawn task for context_file
if let Some(ctx) = context_file {
    let mem = memory.clone();
    let meta = metadata.clone();
    let task = tokio::spawn(async move {
        load_context_stream(ctx.load(), mem, meta, "context_file").await
    });
    load_tasks.push(task);
}

// Spawn task for context_files
if let Some(ctx) = context_files {
    let mem = memory.clone();
    let meta = metadata.clone();
    let task = tokio::spawn(async move {
        load_context_stream(ctx.load(), mem, meta, "context_files").await
    });
    load_tasks.push(task);
}

// Spawn task for context_directory
if let Some(ctx) = context_directory {
    let mem = memory.clone();
    let meta = metadata.clone();
    let task = tokio::spawn(async move {
        load_context_stream(ctx.load(), mem, meta, "context_directory").await
    });
    load_tasks.push(task);
}

// Spawn task for context_github
if let Some(ctx) = context_github {
    let mem = memory.clone();
    let meta = metadata.clone();
    let task = tokio::spawn(async move {
        load_context_stream(ctx.load(), mem, meta, "context_github").await
    });
    load_tasks.push(task);
}

// Wait for all context loading tasks to complete
for task in load_tasks {
    if let Err(e) = task.await {
        log::warn!("Context loading task panicked: {:?}", e);
    }
}
```

**Code Reduction**: 128 lines → ~52 lines (60% reduction)

### Step 3: Remove Dead Code

**File**: `packages/candle/src/domain/chat/session.rs`  
**Lines to Delete**: 46-52 (SessionDocument struct)

Remove the unused struct:
```rust
// DELETE THESE LINES:
/// Simple document structure for session context
#[derive(Debug, Clone)]
pub struct SessionDocument {
    pub content: String,
    pub source: String,
    pub tags: Vec<String>,
}
```

This struct was removed from public exports and is no longer used anywhere in the codebase.

## KEY IMPLEMENTATION DETAILS

### Memory Cloning is Safe
`Arc<MemoryCoordinator>` is cloned for each spawned task. This is efficient because:
- Arc is a reference-counted pointer (cheap clone)
- No data duplication occurs
- All tasks share the same underlying MemoryCoordinator instance

### Context Type Tags
Each context source now gets a distinguishing tag:
- `"context_file"` - single file context
- `"context_files"` - glob pattern context  
- `"context_directory"` - directory context
- `"context_github"` - GitHub repository context

These tags enable filtering and searching by context source type.

### Error Handling
- Individual document failures are logged but don't stop the task
- Task panics are caught and logged at the join point
- One source failing doesn't block other sources

## PERFORMANCE IMPROVEMENT

**Before** (Serial):
```
context_file:      [====] 100ms
context_files:            [====] 100ms
context_directory:                   [====] 100ms  
context_github:                             [====] 100ms
Total: 400ms
```

**After** (Parallel):
```
context_file:      [====] 100ms
context_files:     [====] 100ms
context_directory: [====] 100ms
context_github:    [====] 100ms
Total: ~100ms (4x faster)
```

For large directories with 1000+ files, the improvement can be 10-100x better due to background loading.

## FILES TO MODIFY

1. **`packages/candle/src/domain/chat/session.rs`**
   - Add `load_context_stream` helper function (after line 76)
   - Replace lines 111-239 with parallel loading pattern
   - Delete lines 46-52 (SessionDocument dead code)

Total changes: 1 file, ~200 lines affected

## IMPORTS ALREADY PRESENT

All required imports are already in `session.rs`:
- ✅ `std::collections::HashMap`
- ✅ `std::sync::Arc`
- ✅ `tokio_stream::{Stream, StreamExt}`
- ✅ `crate::domain::context::provider::CandleContext`
- ✅ `crate::memory::core::manager::coordinator::MemoryCoordinator`
- ✅ `crate::domain::memory::primitives::types::MemoryTypeEnum`
- ✅ `crate::memory::MemoryMetadata`

No additional imports needed.

## DEFINITION OF DONE

1. ✅ Helper function `load_context_stream` added to session.rs
2. ✅ Lines 111-239 replaced with parallel tokio::spawn pattern
3. ✅ All 4 context sources spawn independent tasks
4. ✅ Tasks are joined with `.await` to ensure completion
5. ✅ Context type tags added ("context_file", "context_files", etc.)
6. ✅ SessionDocument struct deleted (lines 46-52)
7. ✅ Code compiles: `cargo check -p cyrup_candle`
8. ✅ No new warnings introduced

## VERIFICATION

After implementation, verify parallel execution:

```bash
# Should compile without errors
cargo check -p cyrup_candle

# Look for the new helper function
grep -n "load_context_stream" packages/candle/src/domain/chat/session.rs

# Verify tokio::spawn is used for all 4 sources  
grep -n "tokio::spawn" packages/candle/src/domain/chat/session.rs | grep -E "(context_file|context_files|context_directory|context_github)"

# Confirm SessionDocument is removed
grep -n "SessionDocument" packages/candle/src/domain/chat/session.rs
# Should return: no matches
```

## CRITICAL NOTES

**This is NOT an optimization** - it's a critical performance defect. Serial loading:
1. Blocks session initialization unnecessarily
2. Scales poorly with number of sources  
3. Prevents responsive user experience
4. Violates async Rust best practices

**Zero tolerance for serial blocking in async code when parallel execution is possible.**

The codebase already uses `tokio::spawn` extensively for parallel execution (see [`workflow/parallel.rs`](../packages/candle/src/workflow/parallel.rs) and background memory storage in [`session.rs`](../packages/candle/src/domain/chat/session.rs:510)). This fix simply applies the same proven pattern to context loading.
