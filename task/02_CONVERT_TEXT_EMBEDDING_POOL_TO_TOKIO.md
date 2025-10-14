# Convert Text Embedding Pool from Crossbeam to Tokio Channels - REMAINING ISSUES

## Status: INCOMPLETE - Does NOT Compile (25 errors)

## Core Conversion: ✅ COMPLETE
`packages/candle/src/pool/capabilities/text_embedding.rs` - Fully converted and working

## Remaining Compilation Errors

### 1. Fix traits.rs - chunked_batch_embed ❌
**Location**: `packages/candle/src/capability/traits.rs:195`

**Problem**: 
```rust
fn chunked_batch_embed(...) -> Result<Vec<Vec<f32>>, Box<...>> {
    let chunk_embeddings = self.batch_embed(chunk, task.clone())?;
    //                     ^^^^^^^^^^^^^^ returns Future, not Result
}
```

**Fix Required**:
```rust
// Change function signature to async and return Pin<Box<dyn Future>>
fn chunked_batch_embed(
    &self,
    texts: &[String],
    task: Option<String>,
) -> std::pin::Pin<
    Box<
        dyn std::future::Future<
                Output = std::result::Result<
                    Vec<Vec<f32>>,
                    Box<dyn std::error::Error + Send + Sync>,
                >,
            > + Send
            + '_,
    >,
>;

// In implementation, wrap in Box::pin(async move { ... }) and add .await
let chunk_embeddings = self.batch_embed(chunk, task.clone()).await?;
```

### 2. Fix stella.rs - Type Mismatches ❌
**Locations**: 
- `packages/candle/src/capability/text_embedding/stella.rs:292`
- `packages/candle/src/capability/text_embedding/stella.rs:716`

**Problem**:
```rust
let formatted_text = self.format_with_instruction(&[text], task.as_deref())[0].clone();
//                                                  ^^^^ text is String, need &str
```

**Fix Required**:
```rust
// Change both locations from:
&[text]
// To:
&[&text]
```

### 3. Fix surreal.rs - Future Not Awaited ❌
**Location**: `packages/candle/src/memory/core/manager/surreal.rs:863-869`

**Problem**:
```rust
match model.embed(&content_text, Some("search".to_string())) {
    Ok(embedding_vec) => { ... }
    //  ^^^^^^^^^^^^^^ trying to match on Future, not Result
}
```

**Fix Required**:
```rust
// Add .await before the match
match model.embed(&content_text, Some("search".to_string())).await {
    Ok(embedding_vec) => { ... }
    Err(e) => { ... }
}
```

### 4. Fix multimodal_service.rs - Missing .await ❌
**Locations**:
- `packages/candle/src/memory/vector/multimodal_service.rs:52`
- `packages/candle/src/memory/vector/multimodal_service.rs:73`

**Problem**:
```rust
let result = text_model.embed(&text, task).map_err(|e| { ... })?;
//           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returns Future, needs .await
```

**Fix Required**:
```rust
// Add .await before .map_err
let result = text_model.embed(&text, task).await.map_err(|e| { ... })?;
let result = text_model.batch_embed(&texts, task).await.map_err(|e| { ... })?;
```

### 5. Fix vector_search.rs - Sync Functions Calling Async ❌
**Locations**:
- `packages/candle/src/memory/vector/vector_search.rs:354-365` (search_by_text)
- `packages/candle/src/memory/vector/vector_search.rs:567-579` (batch_search_by_text)

**Problem**:
```rust
pub fn search_by_text(...) -> Result<Vec<SearchResult>> {
    let embedding = self.embedding_model.embed(text, task_string(SEARCH_TASK))?;
    //              ^^^^^^^^^^^^^^^^^^^^^^^ returns Future, cannot use ? in sync fn
}
```

**Fix Required**:
```rust
// Make both functions async
pub async fn search_by_text(
    &self,
    text: &str,
    options: Option<SearchOptions>,
) -> Result<Vec<SearchResult>> {
    let embedding = self.embedding_model.embed(text, task_string(SEARCH_TASK)).await?;
    // ... rest of function
}

pub async fn batch_search_by_text(
    &self,
    texts: &[String],
    options: Option<SearchOptions>,
) -> Result<Vec<Vec<SearchResult>>> {
    let embeddings = self
        .embedding_model
        .batch_embed(texts, task_string(SEARCH_TASK))
        .await?;
    // ... rest of function
}
```

## Verification Commands

After fixes, run:
```bash
cargo check --package paraphym_candle
```

Must show:
```
Finished `dev` profile [unoptimized + debuginfo] target(s) in X.XXs
```

## Success Criteria
- ✅ Zero compilation errors
- ✅ All async trait methods called with .await
- ✅ All sync wrappers converted to async where needed
- ✅ cargo check passes cleanly
