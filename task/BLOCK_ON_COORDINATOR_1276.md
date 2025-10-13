# Remove block_on from coordinator.rs:1276 (HIGH)

**Location:** `src/memory/core/manager/coordinator.rs:1276`

**Priority:** HIGH - Handle::current().block_on() eagerly before stream execution

## Current Code

```rust
fn search_by_content(&self, query: &str) -> MemoryStream {
    // ALWAYS try vector search first for semantic similarity
    // Generate embedding for the query
    let embedding = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async { self.generate_embedding(query).await })  // ← Line 1276
    });

    match embedding {
        Ok(emb) => {
            // Successfully generated embedding - use vector search with cosine similarity
            self.surreal_manager.search_by_vector(emb, 10)
        }
        Err(e) => {
            // Only fall back to substring search if embedding generation fails
            log::warn!(
                "Embedding generation failed, falling back to substring search: {}",
                e
            );
            self.surreal_manager.search_by_content(query, 10)
        }
    }
}
```

## Problem: Eagerly Generating Embedding Before Stream

This method:
1. Returns `MemoryStream` (correct - sync method returning stream)
2. Uses `Handle::current().block_on()` to eagerly generate embedding
3. Then creates and returns the stream

This is wrong: **eagerly blocking before stream creation** instead of **lazy generation inside stream**.

## Solution: Move Embedding Generation Inside Stream

The embedding generation should happen lazily when the stream is consumed:

```rust
fn search_by_content(&self, query: &str) -> MemoryStream {
    let query = query.to_string();
    let self_clone = self.clone(); // or capture what's needed
    
    AsyncStream::with_channel(move |sender| async move {
        // Generate embedding lazily when stream is consumed
        match self_clone.generate_embedding(&query).await {
            Ok(emb) => {
                // Use vector search with cosine similarity
                let mut stream = self_clone.surreal_manager.search_by_vector(emb, 10);
                
                // Forward results through sender
                use futures_util::StreamExt;
                while let Some(result) = stream.next().await {
                    if sender.send(result).is_err() {
                        break;
                    }
                }
            }
            Err(e) => {
                // Fall back to substring search
                log::warn!(
                    "Embedding generation failed, falling back to substring search: {}",
                    e
                );
                
                let mut stream = self_clone.surreal_manager.search_by_content(&query, 10);
                
                // Forward results through sender
                use futures_util::StreamExt;
                while let Some(result) = stream.next().await {
                    if sender.send(result).is_err() {
                        break;
                    }
                }
            }
        }
    })
}
```

**Pattern Explanation:**
- **ANTIPATTERN (current):** `let x = block_on(async_op()); return stream_using(x)`
- **CORRECT (fix):** `AsyncStream::with_channel(|s| async move { let x = async_op().await; ... })`

## Implementation Notes

1. Remove `block_in_place` + `Handle::current().block_on()` wrappers
2. Capture necessary data for closure
3. Create AsyncStream that generates embedding when consumed
4. Use `.await` on `generate_embedding()`
5. Forward search results through sender

## Files to Modify

- `src/memory/core/manager/coordinator.rs` - Update search_by_content method (line 1271)

## Benefits

1. Lazy evaluation - embedding only generated if stream is consumed
2. No Handle::current().block_on() - eliminates nested runtime risk
3. Proper async pattern
4. Stream creation is instant, work happens on consumption
