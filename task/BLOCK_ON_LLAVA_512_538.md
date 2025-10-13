# Remove block_on from llava.rs:512, 538 (HIGH)

**Locations:**
- `src/capability/vision/llava.rs:512` - preprocess_image_sync
- `src/capability/vision/llava.rs:538` - preprocess_image_url_sync

**Priority:** HIGH - Sync wrappers blocking inside stream contexts

## Current Code Pattern

```rust
/// Preprocess image from file path (sync version for thread)
fn preprocess_image_sync(
    device: &Device,
    image_path: &str,
    image_size: usize,
    image_mean: [f32; 3],
    image_std: [f32; 3],
) -> Result<Tensor, String> {
    let runtime = crate::runtime::shared_runtime()
        .ok_or_else(|| "Shared runtime unavailable".to_string())?;
    
    runtime.block_on(async {
        Image::from_path(image_path)
            .resize(image_size, image_size, ResizeFilter::CatmullRom)
            .normalize_unsigned()
            .normalize_with(image_mean, image_std)
            .to_tensor(device)
            .await
    })
}
```

## Problem: Eagerly Blocking Before Stream Execution

These "_sync" methods use `runtime.block_on()` to eagerly compute results. This is wrong:
1. Should do async work INSIDE AsyncStream where it belongs
2. Blocking before stream creation violates lazy evaluation
3. Forces use of shared_runtime.block_on()

## Solution: Delete Sync Methods, Use AsyncStream in Callers

### Step 1: Find call sites of _sync methods

Search for:
- `preprocess_image_sync(`
- `preprocess_image_url_sync(`

### Step 2: Move async work INSIDE AsyncStream

Change from this ANTIPATTERN:
```rust
// BAD - eagerly blocking before stream
let tensor = Self::preprocess_image_sync(&device, path, size, mean, std)?;
AsyncStream::with_channel(|sender| {
    // use tensor
})
```

To this CORRECT pattern:
```rust
// GOOD - lazy async inside stream
AsyncStream::with_channel(|sender| async move {
    let tensor = Image::from_path(path)
        .resize(size, size, ResizeFilter::CatmullRom)
        .normalize_unsigned()
        .normalize_with(mean, std)
        .to_tensor(&device)
        .await;
    
    // use tensor
})
```

### Step 3: Delete the _sync methods entirely

Remove both preprocess_image_sync and preprocess_image_url_sync.

**Pattern Explanation:**
- **ANTIPATTERN (current):** `let x = sync_wrapper_with_block_on(); stream { use x }`
- **CORRECT (fix):** `AsyncStream::with_channel(|sender| async move { let x = async_op().await; use x })`

## Implementation Notes

1. Find ALL call sites of these _sync methods
2. Check if callers are using ystream or other sync patterns
3. Replace with AsyncStream::with_channel with async blocks
4. Move the async image preprocessing INSIDE the async block
5. Delete the _sync methods completely

## No Excuses

- "But the caller is in a sync closure" → Use AsyncStream with async block instead
- "But it's simpler to block" → No, it's wrong. Move async work inside streams.
- "But performance" → Lazy evaluation is faster than eager blocking.

Delete these sync wrappers and move async work into AsyncStream.
