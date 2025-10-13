# Remove block_on from progresshub.rs:80, 92 (LOW)

**Locations:**
- `src/domain/model/progresshub.rs:80` - Comment claiming "APPROVED"
- `src/domain/model/progresshub.rs:92` - default_for_builder trait method

**Priority:** LOW - Builder pattern, but should use AsyncStream

## Current Code

```rust
/// Trait for models that can be initialized from HuggingFace with progress tracking
pub trait ProgressHubModelProvider: Sized {
    /// Create a new instance by downloading from HuggingFace with progress
    async fn new() -> Result<Self, Box<dyn Error + Send + Sync>>;
    
    /// Synchronous initialization for use in builder patterns.
    ///
    /// Uses shared runtime to await async initialization in sync contexts.
    ///
    /// BLOCKING CODE APPROVED: 2025-01-20 by @maintainer
    /// Rationale: Builder patterns require synchronous initialization. Using
    /// `shared_runtime().block_on()` is the correct pattern for bridging async
    /// operations in sync builder contexts. This is intentional production code.
    ///
    /// # Errors
    /// Returns error if download or initialization fails
    fn default_for_builder() -> Result<Self, Box<dyn Error + Send + Sync>>
    where
        Self: Sized,
    {
        // BLOCKING CODE APPROVED: See trait-level documentation
        crate::runtime::shared_runtime()
            .ok_or_else(|| Box::<dyn Error + Send + Sync>::from("Runtime unavailable"))?
            .block_on(Self::new())  // ← Line 92: NOT APPROVED!
    }
}
```

## Problem: FALSE "APPROVED" Comment

The code claims blocking is "APPROVED" for builder patterns. This is WRONG:
1. Builders should use AsyncStream with async blocks
2. No blocking needed in builders
3. The "approval" comment is false justification
4. Move async initialization inside AsyncStream

## Solution: Delete This Method, Use AsyncStream in Builders

### Step 1: Delete default_for_builder method entirely

Remove the entire `default_for_builder` method from the trait.

### Step 2: Update builders to use AsyncStream

Builders that call `default_for_builder()` should instead use AsyncStream:

```rust
// ANTIPATTERN (current)
let model = ModelType::default_for_builder()?;
AsyncStream::with_channel(|sender| {
    // use model
})

// CORRECT (fix)
AsyncStream::with_channel(|sender| async move {
    let model = ModelType::new().await?;
    // use model
})
```

### Step 3: Remove "APPROVED" comments

Delete all false claims about blocking being approved.

**Pattern Explanation:**
- **ANTIPATTERN (current):** Builder eagerly blocks: `let m = default_for_builder()?; return stream`
- **CORRECT (fix):** Builder returns stream with lazy init: `AsyncStream::with_channel(|s| async move { let m = new().await?; ... })`

## Implementation Notes

1. Delete `default_for_builder` method from ProgressHubModelProvider trait
2. Search for all call sites of `default_for_builder()`
3. Replace with AsyncStream pattern: `AsyncStream::with_channel(|s| async move { T::new().await })`
4. Remove all "BLOCKING CODE APPROVED" comments - they're false

## Files to Modify

- `src/domain/model/progresshub.rs` - Delete default_for_builder method
- Find all call sites and update to AsyncStream pattern

## Note

The "APPROVED" comment is actively harmful - it suggests blocking in builders is acceptable when the correct pattern is AsyncStream with async blocks. This comment should be completely removed.
