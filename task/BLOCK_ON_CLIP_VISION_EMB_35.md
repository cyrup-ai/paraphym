# Remove block_on from clip_vision_embedding.rs:35 (LOW)

**Location:** `src/capability/image_embedding/clip_vision_embedding.rs:35`

**Priority:** LOW - Default trait, should be removed

## Current Code

```rust
impl Default for ClipVisionEmbeddingModel {
    fn default() -> Self {
        crate::runtime::shared_runtime()
            .unwrap_or_else(|| panic!("Shared runtime unavailable"))
            .block_on(Self::new())
            .unwrap_or_else(|e| panic!("Failed to initialize ClipVisionEmbeddingModel: {}", e))
    }
}
```

## Problem: Default Trait Forces Blocking

Default trait forces sync, causing blocking initialization with panics.

## Solution: Remove Default Trait

Delete the Default impl entirely:

```rust
// DELETE this entire impl block
```

Find all uses of `ClipVisionEmbeddingModel::default()` and replace with proper initialization inside AsyncStream:

```rust
// ANTIPATTERN
let model = ClipVisionEmbeddingModel::default();
AsyncStream::with_channel(|sender| {
    // use model
})

// CORRECT
AsyncStream::with_channel(|sender| async move {
    let model = ClipVisionEmbeddingModel::new().await?;
    // use model
})
```

**Pattern:** Move async initialization INSIDE AsyncStream, delete Default impl.
