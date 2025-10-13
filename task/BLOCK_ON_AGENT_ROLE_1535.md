# Remove block_on from agent_role.rs:1535 (CRITICAL)

**Location:** `src/builders/agent_role.rs:1535`

**Priority:** CRITICAL - Uses Handle::current().block_on() inside ystream closure

## Current Code

```rust
let memory_context: Option<String> = if let Some(ref mem_manager) = memory {
    let memory_stream = mem_manager.search_by_content(&user_message);
    let timeout_duration = std::time::Duration::from_millis(memory_read_timeout);

    // Use tokio::task::block_in_place pattern with timeout protection
    let results: Vec<MemoryResult<MemoryNode>> =
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                match tokio::time::timeout(timeout_duration, async {
                    use futures_util::StreamExt;
                    memory_stream.take(5).collect().await
                }).await {
                    Ok(results) => results,
                    Err(_) => {
                        log::warn!("Memory search timed out after {}ms, proceeding with empty context", memory_read_timeout);
                        Vec::new()
                    }
                }
            })
        });

    let memories: Vec<MemoryNode> =
        results.into_iter().filter_map(|r| r.ok()).collect();

    if !memories.is_empty() {
        Some(format_memory_context(&memories, 1000))
    } else {
        None
    }
} else {
    None
};
```

Context: Inside ystream::spawn_stream at line 1438, searching memory before building prompt.

## Problem: Nested Runtime Error via Handle::current()

This code uses **`Handle::current().block_on()`** which tries to block on the current runtime from within itself - the classic nested runtime error! This:
1. Will cause "Cannot start a runtime from within a runtime" error
2. Uses block_in_place + Handle::current().block_on() antipattern
3. Eagerly blocks to search memory before building prompt
4. Inside ystream sync closure instead of AsyncStream async block

## Solution: Use .await Inside AsyncStream

When refactoring line 1514 to use `AsyncStream::with_channel(|sender| async move { ... })`, this memory search should use `.await`:

```rust
// Inside AsyncStream::with_channel(|sender| async move { ... })

let memory_context: Option<String> = if let Some(ref mem_manager) = memory {
    let memory_stream = mem_manager.search_by_content(&user_message);
    let timeout_duration = std::time::Duration::from_millis(memory_read_timeout);

    // Use .await with timeout instead of block_on
    let results: Vec<MemoryResult<MemoryNode>> = match tokio::time::timeout(
        timeout_duration,
        async {
            use futures_util::StreamExt;
            memory_stream.take(5).collect().await
        }
    ).await {
        Ok(results) => results,
        Err(_) => {
            log::warn!("Memory search timed out after {}ms, proceeding with empty context", memory_read_timeout);
            Vec::new()
        }
    };

    let memories: Vec<MemoryNode> =
        results.into_iter().filter_map(|r| r.ok()).collect();

    if !memories.is_empty() {
        Some(format_memory_context(&memories, 1000))
    } else {
        None
    }
} else {
    None
};
```

**Pattern Explanation:**
- **ANTIPATTERN (current):** `block_in_place(|| Handle::current().block_on(async { stream.collect().await }))`
- **CORRECT (fix):** `timeout(dur, async { stream.collect().await }).await` (inside async block)

## Implementation Notes

1. **Part of same refactoring** as BLOCK_ON_AGENT_ROLE_1514.md
2. Remove `tokio::task::block_in_place` wrapper entirely
3. Remove `Handle::current().block_on` wrapper entirely
4. Use direct `.await` on timeout future
5. The timeout pattern is correct, just needs to be in async context
6. This eliminates the nested runtime error

## Dependencies

- Must be fixed with BLOCK_ON_AGENT_ROLE_1514.md
- Part of ystream → AsyncStream refactoring at line 1438
- Related to BLOCK_ON_AGENT_ROLE_1586.md and BLOCK_ON_AGENT_ROLE_1673.md
## Reference Example

See `src/agent/builder.rs:345` for working example of `AsyncStream::with_channel` with `async move` block.
