# Task 019: Fix CLIP Vision Async Architecture - to_tensor_sync()

## Core Objective

Fix the architectural flaw in `LoadedClipVisionModel` where `tokio::runtime::Handle::current().block_on()` is used inside `spawn_blocking` contexts. This violates the async architecture principles established in Tasks 014, 015, and 017.

**The Problem:** Blocking a blocking thread defeats the purpose of `spawn_blocking`.

**The Solution:** Create `to_tensor_sync()` method that executes synchronously without async wrapping.

## Architectural Context

### Why This Matters

The codebase has established a clear pattern for async/sync boundaries:

1. **Tasks 014, 015, 017:** Fixed NVEmbed and Stella to use `std::sync::Mutex` inside `spawn_blocking` (not `tokio::sync::Mutex` with `block_on`)
2. **BERT Pattern ([src/capability/text_embedding/bert.rs:554-617](../src/capability/text_embedding/bert.rs)):** Uses fully synchronous `forward_pass()` inside `spawn_blocking`
3. **Current CLIP Issue:** Uses `block_on()` inside `spawn_blocking`, blocking the blocking thread

### BERT Pattern (Correct) vs CLIP Pattern (Incorrect)

**✅ BERT Pattern:**
```rust
// src/capability/text_embedding/bert.rs:581-617
tokio::task::spawn_blocking(move || {
    CandleBertEmbeddingModel::forward_pass(
        &tokenizer,
        &model,
        &device,
        &[&text],
    )
})
```

Where `forward_pass()` is **fully synchronous**:
```rust
// Lines 112-180
fn forward_pass(
    tokenizer: &Tokenizer,
    model: &BertModel,
    device: &Device,
    texts: &[&str],
) -> Result<Vec<Vec<f32>>> {
    // Tokenize (sync)
    // Create tensors (sync)
    // Model inference (sync)
    // Extract embeddings (sync)
}
```

**❌ Current CLIP Pattern:**
```rust
// src/capability/image_embedding/clip_vision.rs:704-708
tokio::task::spawn_blocking(move || {
    let rt = tokio::runtime::Handle::current();  // ❌ Getting async runtime
    let image_tensor = rt
        .block_on(image_builder.to_tensor(&device))  // ❌ Blocking blocking thread
        .map_err(|e| format!("Image preprocessing failed: {}", e))?;
    // ...
})
```

## Current Implementation Status

### ✅ Already Complete

1. **LoadedClipVisionModel** created with `Arc<ClipModel>` (BERT pattern)
2. **ImageEmbeddingCapable** trait implemented with `spawn_blocking`
3. **ClipVisionEmbeddingModel** updated with loaded model support
4. **Backward compatibility** maintained
5. **Error handling** production-grade (no unwrap/expect)
6. **Code compiles** with zero errors/warnings

### ❌ Outstanding Issue

**Missing `to_tensor_sync()` method** - Required to replace `block_on()` calls

## Implementation Plan

### File 1: Add to_tensor_sync() Method

**File:** [`src/builders/image.rs`](../src/builders/image.rs)  
**Location:** After `to_tensor()` method (around line 430)  
**Section:** `impl<F1, F2> ImageBuilderImpl<F1, F2>`

**Why This Works:** All helper methods are already synchronous:
- [`load_image_from_source()`](../src/builders/image.rs#L516) - Line 516
- [`apply_image_operations()`](../src/builders/image.rs#L551) - Line 551  
- [`image_to_tensor()`](../src/builders/image.rs#L586) - Line 586
- [`apply_tensor_operations()`](../src/builders/image.rs#L626) - Line 626
- [`transfer_to_device()`](../src/builders/image.rs#L695) - Line 695

**Add This Method:**

```rust
impl<F1, F2> ImageBuilderImpl<F1, F2>
where
    F1: Fn(String) + Send + Sync + 'static,
    F2: FnMut(ImageChunk) -> ImageChunk + Send + 'static,
{
    // ... existing methods ...

    /// Synchronous tensor creation for spawn_blocking contexts
    ///
    /// This method performs the same operations as `to_tensor()` but without
    /// async wrapping, making it suitable for use inside `spawn_blocking`.
    ///
    /// Executes the complete image processing pipeline:
    /// 1. Load image from source (base64/URL/path)
    /// 2. Apply image-level operations (resize, RGB conversion)
    /// 3. Convert image to tensor (HWC→CHW, u8→f32)
    /// 4. Apply tensor-level operations (normalize, clamp)
    /// 5. Transfer to target device
    ///
    /// # Pattern Reference
    /// Follows BERT's synchronous forward_pass pattern:
    /// - src/capability/text_embedding/bert.rs:112-180
    ///
    /// # Usage
    /// ```rust
    /// tokio::task::spawn_blocking(move || {
    ///     let tensor = image_builder.to_tensor_sync(&device)?;
    ///     // ... rest of sync processing
    /// })
    /// ```
    pub fn to_tensor_sync(self, device: &candle_core::Device) -> Result<candle_core::Tensor, String> {
        // Step 1: Load image from source (base64/URL/path)
        let img = self.load_image_from_source()?;
        
        // Step 2: Apply image-level operations (resize, RGB conversion)
        let img = self.apply_image_operations(img)?;
        
        // Step 3: Convert image to tensor (HWC→CHW, u8→f32)
        let tensor = self.image_to_tensor(img)?;
        
        // Step 4: Apply tensor-level operations (normalize, clamp)
        let tensor = self.apply_tensor_operations(tensor)?;
        
        // Step 5: Transfer to target device
        let tensor = self.transfer_to_device(tensor, device)?;
        
        Ok(tensor)
    }
}
```

### File 2: Replace block_on() Calls

**File:** [`src/capability/image_embedding/clip_vision.rs`](../src/capability/image_embedding/clip_vision.rs)  
**Locations:** Lines 707, 793, 879, 967

**Change 1: embed_image() method (Line 707)**

```rust
// BEFORE (❌):
let rt = tokio::runtime::Handle::current();
let image_tensor = rt
    .block_on(image_builder.to_tensor(&device))
    .map_err(|e| format!("Image preprocessing failed: {}", e))?;

// AFTER (✅):
let image_tensor = image_builder
    .to_tensor_sync(&device)
    .map_err(|e| format!("Image preprocessing failed: {}", e))?;
```

**Change 2: embed_image_url() method (Line 793)**

```rust
// BEFORE (❌):
let rt = tokio::runtime::Handle::current();
let image_tensor = rt
    .block_on(image_builder.to_tensor(&device))
    .map_err(|e| format!("Image URL preprocessing failed: {}", e))?;

// AFTER (✅):
let image_tensor = image_builder
    .to_tensor_sync(&device)
    .map_err(|e| format!("Image URL preprocessing failed: {}", e))?;
```

**Change 3: embed_image_base64() method (Line 879)**

```rust
// BEFORE (❌):
let rt = tokio::runtime::Handle::current();
let image_tensor = rt
    .block_on(image_builder.to_tensor(&device))
    .map_err(|e| format!("Base64 image preprocessing failed: {}", e))?;

// AFTER (✅):
let image_tensor = image_builder
    .to_tensor_sync(&device)
    .map_err(|e| format!("Base64 image preprocessing failed: {}", e))?;
```

**Change 4: batch_embed_images() method (Line 967 - inside loop)**

```rust
// BEFORE (❌):
let tensor = rt
    .block_on(image_builder.to_tensor(&device))
    .map_err(|e| format!("Image preprocessing failed for {}: {}", path, e))?;

// AFTER (✅):
let tensor = image_builder
    .to_tensor_sync(&device)
    .map_err(|e| format!("Image preprocessing failed for {}: {}", path, e))?;
```

**Also remove the `rt` variable declaration** (around line 956):
```rust
// DELETE THIS LINE:
let rt = tokio::runtime::Handle::current();
```

## Complete Example: embed_image() After Fix

```rust
fn embed_image(&self, image_path: &str) -> Pin<Box<dyn Future<...>>> {
    let image_path = image_path.to_string();
    let model = self.model.clone();
    let device = self.device.clone();
    let image_size = self.config.image_size;
    let image_mean = /* ... */;
    let image_std = /* ... */;
    
    Box::pin(async move {
        let image_mean = image_mean?;
        let image_std = image_std?;
        
        // ✅ Proper spawn_blocking pattern (like BERT)
        let embedding = tokio::task::spawn_blocking(move || {
            // Image preprocessing - synchronous
            let image_builder = Image::from_path(&image_path)
                .resize(image_size, image_size, ResizeFilter::Triangle)
                .normalize_unsigned()
                .normalize_with(image_mean, image_std);
            
            // ✅ Synchronous tensor creation (no block_on)
            let image_tensor = image_builder.to_tensor_sync(&device)?;
            
            // Add batch dimension - synchronous
            let batched = image_tensor.unsqueeze(0)?;
            
            // Model inference - synchronous
            let features = model.get_image_features(&batched)?;
            
            // Tensor conversion - synchronous
            features.to_vec1::<f32>()
        })
        .await??;
        
        Ok(embedding)
    })
}
```

## Why Helper Methods Are Already Sync

From [`src/builders/image.rs`](../src/builders/image.rs):

| Method | Line | Signature | Operations |
|--------|------|-----------|------------|
| `load_image_from_source` | 516 | `fn(&self) -> Result<DynamicImage, String>` | File I/O, base64 decode, image decode |
| `apply_image_operations` | 551 | `fn(&self, DynamicImage) -> Result<DynamicImage, String>` | Image resize, format conversion |
| `image_to_tensor` | 586 | `fn(&self, DynamicImage) -> Result<Tensor, String>` | RGB conversion, tensor creation, permute, dtype |
| `apply_tensor_operations` | 626 | `fn(&self, Tensor) -> Result<Tensor, String>` | Normalize, clamp, broadcast ops |
| `transfer_to_device` | 695 | `fn(&self, Tensor, &Device) -> Result<Tensor, String>` | Device transfer |

**All methods are synchronous.** The `to_tensor()` method (line 369) just wraps them in `Box::pin(async move { ... })`.

## Architecture Comparison

| Component | BERT (✅ Correct) | CLIP Before (❌ Wrong) | CLIP After (✅ Fixed) |
|-----------|------------------|----------------------|---------------------|
| **Model Storage** | `Arc<BertModel>` | `Arc<ClipModel>` | `Arc<ClipModel>` |
| **Processing Method** | `forward_pass()` | `to_tensor()` | `to_tensor_sync()` |
| **Method Type** | Synchronous | Async (wrapped) | Synchronous |
| **spawn_blocking** | Direct call | `block_on()` call | Direct call |
| **Blocking Thread** | Yields properly | Blocked | Yields properly |

## Definition of Done

✅ **Code Changes:**
1. `to_tensor_sync()` method added to `src/builders/image.rs` (after line 430)
2. All 4 `block_on()` calls replaced with `to_tensor_sync()` in `src/capability/image_embedding/clip_vision.rs`
3. `rt` variable declarations removed (no longer needed)

✅ **Verification:**
1. Code compiles with `cargo check --lib` (zero errors, zero warnings)
2. No `tokio::runtime::Handle::current()` in `spawn_blocking` contexts
3. Architecture consistent with BERT pattern

✅ **Quality:**
1. No `unwrap()` or `expect()` calls
2. Proper error propagation with `map_err`
3. Clear documentation on `to_tensor_sync()` method

## File Checklist

- [ ] **src/builders/image.rs** - Add `to_tensor_sync()` method after line 430
- [ ] **src/capability/image_embedding/clip_vision.rs** - Replace 4 occurrences:
  - [ ] Line 707 - `embed_image()` method
  - [ ] Line 793 - `embed_image_url()` method  
  - [ ] Line 879 - `embed_image_base64()` method
  - [ ] Line 967 - `batch_embed_images()` method (inside loop)
  - [ ] Line 956 - Remove `rt` variable declaration
- [ ] **Compile check** - `cargo check --lib` passes

## References

### Codebase Citations

- **BERT Pattern:** [src/capability/text_embedding/bert.rs:554-617](../src/capability/text_embedding/bert.rs) - Correct spawn_blocking usage
- **BERT forward_pass:** [src/capability/text_embedding/bert.rs:112-180](../src/capability/text_embedding/bert.rs) - Synchronous processing
- **Image Builder:** [src/builders/image.rs:369-430](../src/builders/image.rs) - Current async to_tensor()
- **Helper Methods:** [src/builders/image.rs:516-695](../src/builders/image.rs) - All synchronous
- **Current CLIP:** [src/capability/image_embedding/clip_vision.rs:652-1026](../src/capability/image_embedding/clip_vision.rs) - LoadedClipVisionModel implementation

### Related Tasks

- **Task 014:** Fixed NVEmbed async/sync boundaries
- **Task 015:** Fixed Stella async/sync boundaries  
- **Task 017:** Established `std::sync::Mutex` pattern in spawn_blocking

## Estimated Effort

**15-20 minutes** - Simple refactor exposing existing synchronous functionality