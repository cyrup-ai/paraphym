# Remove block_on from qwen3_coder.rs:51 (LOW)

**Location:** `src/capability/text_to_text/qwen3_coder.rs:51`

**Priority:** LOW - Default trait, should be removed

## Current Code

```rust
impl Default for CandleQwen3CoderModel {
    fn default() -> Self {
        crate::runtime::shared_runtime()
            .unwrap_or_else(|| panic!("Shared runtime unavailable"))
            .block_on(Self::new())
            .unwrap_or_else(|e| panic!("Failed to initialize CandleQwen3CoderModel: {}", e))
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

Find all uses of `CandleQwen3CoderModel::default()` and replace with proper initialization inside AsyncStream:

```rust
// ANTIPATTERN
let model = CandleQwen3CoderModel::default();
AsyncStream::with_channel(|sender| {
    // use model
})

// CORRECT
AsyncStream::with_channel(|sender| async move {
    let model = CandleQwen3CoderModel::new().await?;
    // use model
})
```

**Pattern:** Move async initialization INSIDE AsyncStream, delete Default impl.
