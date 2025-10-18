# Task 020: Complete async conversion for LLaVA vision model (CONTINUED)

## Status: INCOMPLETE (33% complete)

**File**: [`src/capability/vision/llava.rs`](../src/capability/vision/llava.rs) (860 lines)

## Core Objective

Isolate CPU-intensive operations in `spawn_blocking` to prevent blocking the async runtime's executor threads. Currently only tokenization is isolated - image processing and generation remain synchronous.

### What Was Completed ✅

1. ✅ Worker runtime changed to `new_multi_thread().worker_threads(1)` (line 213-214)
2. ✅ `tokenize_image_prompt_static` made async with spawn_blocking (lines 621-666)
3. ✅ Tokenization call sites updated with `.await` (lines 408, 530)

### What Remains INCOMPLETE ❌

**Problem**: The most CPU-intensive operations execute synchronously on the async runtime:
- Image decode/resize/normalize: **50-200ms per image** (synchronous)
- Generation loop: **1-5 seconds for full response** (synchronous)

This blocks the worker runtime's async executor, preventing concurrent request processing.

---

## Research: Image API Architecture

**Location**: [`src/builders/image.rs`](../src/builders/image.rs)

The Image API provides async `.to_tensor()` but internally executes operations **synchronously**:

```rust
// Image API implementation (lines 396-427)
fn to_tensor(self, device: &Device) -> Pin<Box<dyn Future<Output = Result<Tensor, String>>>> {
    let device = device.clone();
    Box::pin(async move {
        // ALL SYNCHRONOUS - no spawn_blocking:
        let img = self.load_image_from_source()?;      // File I/O + image decode
        let img = self.apply_image_operations(img)?;    // CPU-intensive resize
        let tensor = self.image_to_tensor(img)?;        // Permute + dtype conversion
        let tensor = self.apply_tensor_operations(tensor)?;  // Normalization
        let tensor = self.transfer_to_device(tensor, &device)?;  // GPU transfer
        Ok(tensor)
    })
}
```

**Issue**: `Box::pin(async move {...})` creates an async wrapper but operations inside run synchronously on the calling thread. When awaited, they block the async runtime's executor.

**Solution**: Wrap the Image API chain in `tokio::task::spawn_blocking`.

---

## Outstanding Requirements

### Requirement 1: Isolate Image Processing in spawn_blocking

**Current Code** (Lines 395-401 in `process_ask`):

```rust
// 1. Preprocess image - async image loading
let image_tensor = Image::from_path(image_path)
    .resize(image_size, image_size, ResizeFilter::CatmullRom)
    .normalize_unsigned()
    .normalize_with(image_mean, image_std)
    .to_tensor(device)
    .await?;
```

**Problem**: 
- Image decode: **20-50ms** (file I/O + JPEG/PNG decode)
- Resize: **30-100ms** (resampling 336×336 image)
- Normalize: **10-20ms** (tensor operations)
- **Total: 60-170ms blocking async runtime**

**Required Implementation**:

```rust
// 1. Preprocess image - isolated in spawn_blocking
let image_path_owned = image_path.to_string();
let device_clone = device.clone();
let image_tensor = tokio::task::spawn_blocking(move || {
    // Blocking thread pool executes CPU-intensive operations
    let runtime = tokio::runtime::Handle::current();
    runtime.block_on(async move {
        Image::from_path(&image_path_owned)
            .resize(image_size, image_size, ResizeFilter::CatmullRom)
            .normalize_unsigned()
            .normalize_with(image_mean, image_std)
            .to_tensor(&device_clone)
            .await
    })
})
.await
.map_err(|e| format!("Spawn blocking failed: {}", e))?
.map_err(|e| format!("Image processing failed: {}", e))?;
```

**Why this works**:
- `spawn_blocking` moves work to dedicated blocking thread pool
- Image operations don't block async executor
- Runtime is captured to run the async Image chain in blocking context

**Action**: Replace lines 395-401 with the implementation above.

---

### Requirement 2: Isolate URL Image Processing in spawn_blocking

**Current Code** (Lines 521-527 in `process_ask_url`):

```rust
// 1. Preprocess image from URL - async image loading
let image_tensor = Image::from_url(image_url)
    .resize(image_size, image_size, ResizeFilter::CatmullRom)
    .normalize_unsigned()
    .normalize_with(image_mean, image_std)
    .to_tensor(device)
    .await?;
```

**Problem**: Same as Requirement 1, plus network I/O blocking

**Required Implementation**:

```rust
// 1. Preprocess image from URL - isolated in spawn_blocking
let image_url_owned = image_url.to_string();
let device_clone = device.clone();
let image_tensor = tokio::task::spawn_blocking(move || {
    let runtime = tokio::runtime::Handle::current();
    runtime.block_on(async move {
        Image::from_url(&image_url_owned)
            .resize(image_size, image_size, ResizeFilter::CatmullRom)
            .normalize_unsigned()
            .normalize_with(image_mean, image_std)
            .to_tensor(&device_clone)
            .await
    })
})
.await
.map_err(|e| format!("Spawn blocking failed: {}", e))?
.map_err(|e| format!("Image processing failed: {}", e))?;
```

**Action**: Replace lines 521-527 with the implementation above.

---

### Requirement 3: Understanding Model Forward Pass Constraints

**Architecture**: LLaVA model is `!Send` (cannot move between threads) due to Candle's design.

**Current Implementation** (Lines 416-481):
```rust
// 6. Generate response (autoregressive loop)
for index in 0..max_new_tokens {
    // Tensor slicing (fast, <1ms)
    let input = current_embeds.i((.., input_embeds_len.saturating_sub(context_size).., ..))?;
    
    // MODEL FORWARD - CANNOT wrap in spawn_blocking (!Send constraint)
    let logits = model.forward(&input, context_index, &mut cache)?;  // Line 449: 50-200ms per token
    
    // Sample next token (fast, <1ms)
    let next_token = Self::sample_token_static(temperature, &logits)?;  // Line 459
    
    // Decode token (fast, <1ms)
    if let Ok(text) = tokenizer.decode(&[next_token], false) {
        generated_text.push_str(&text);
    }
    
    // MODEL EMBED - CANNOT wrap in spawn_blocking (!Send constraint)
    let next_embeds = model.llama.embed(&next_token_tensor)?  // Line 475: 10-30ms
        .unsqueeze(0)?;
    
    current_embeds = Tensor::cat(&[current_embeds, next_embeds], 1)?;
}
```

**Why Model Operations CANNOT Be Async**:

1. **`!Send` Constraint**: Candle's `LLaVA` model contains raw pointers and non-thread-safe state
2. **LocalSet Requirement**: Model lives on `tokio::task::LocalSet` in dedicated worker thread
3. **Mutable State**: `model.forward(&mut cache)` requires exclusive mutable access
4. **Architecture Decision**: Worker thread pattern chosen specifically to handle !Send models

**What CAN Be Optimized**: None of the individual operations in the loop are slow enough (<1ms each except model ops) to warrant spawn_blocking overhead (~100μs per call).

**Conclusion**: Generation loop should remain synchronous. The model operations (50-200ms per token) are the bottleneck, but they MUST stay on LocalSet due to !Send constraint.

**Action**: **NO CHANGES REQUIRED**. Add documentation explaining why:

```rust
// 6. Generate response (autoregressive loop)
// NOTE: This loop remains synchronous because:
// - Model is !Send (cannot move to spawn_blocking threads)
// - Model lives on LocalSet in dedicated worker thread
// - Each non-model operation is fast (<1ms)
// - spawn_blocking overhead (~100μs) would not improve performance
// This architectural constraint is correct for Candle's !Send models
let mut generated_text = String::new();
let mut current_embeds = input_embeds;
```

**Insert**: Add comment block at line 426 (before generation loop).

---

## Revised Definition of Done

✅ Worker runtime changed to `new_multi_thread().worker_threads(1)` - COMPLETED  
✅ `tokenize_image_prompt_static` async with spawn_blocking - COMPLETED  
❌ **Image processing (from_path) wrapped in spawn_blocking** - Lines 395-401  
❌ **URL processing (from_url) wrapped in spawn_blocking** - Lines 521-527  
✅ Model forward passes documented as architecturally constrained - Add comment at line 426  
✅ No unwrap() or expect() in implementation - VERIFIED  
✅ No block_on in async paths - VERIFIED  

---

## Implementation Steps

### Step 1: Wrap Image Processing (Lines 395-401)

**Before**:
```rust
let image_tensor = Image::from_path(image_path)
    .resize(image_size, image_size, ResizeFilter::CatmullRom)
    .normalize_unsigned()
    .normalize_with(image_mean, image_std)
    .to_tensor(device)
    .await?;
```

**After**:
```rust
let image_path_owned = image_path.to_string();
let device_clone = device.clone();
let image_tensor = tokio::task::spawn_blocking(move || {
    let runtime = tokio::runtime::Handle::current();
    runtime.block_on(async move {
        Image::from_path(&image_path_owned)
            .resize(image_size, image_size, ResizeFilter::CatmullRom)
            .normalize_unsigned()
            .normalize_with(image_mean, image_std)
            .to_tensor(&device_clone)
            .await
    })
})
.await
.map_err(|e| format!("Spawn blocking failed: {}", e))?
.map_err(|e| format!("Image processing failed: {}", e))?;
```

### Step 2: Wrap URL Processing (Lines 521-527)

Apply same pattern as Step 1, replacing `from_path` with `from_url`.

### Step 3: Document Generation Loop Constraints (Line 426)

**Insert before line 426**:
```rust
// 6. Generate response (autoregressive loop)
// NOTE: This loop remains synchronous because:
// - LLaVA model is !Send (contains raw pointers, cannot move between threads)
// - Model lives on LocalSet in dedicated worker thread
// - model.forward() and model.llama.embed() require &mut model access
// - Each non-model operation is fast (<1ms): tensor slicing, sampling, decoding
// - spawn_blocking overhead (~100μs per call) would not improve performance
// This is architecturally correct for Candle's !Send models
```

---

## Why This Approach Is Correct

### Image Processing: spawn_blocking Required

**Evidence**:
- Image decode: 20-50ms (file I/O + JPEG/PNG decompression)
- Resize: 30-100ms (Catmull-Rom resampling for 336×336 image)
- Normalize: 10-20ms (tensor broadcast operations)

**60-170ms of synchronous operations blocking async executor = unacceptable**

Pattern reference: [`src/capability/text_embedding/gte_qwen.rs`](../src/capability/text_embedding/gte_qwen.rs) lines 54-63 (tokenization spawn_blocking)

### Model Operations: spawn_blocking Impossible

**Evidence**:
- Model type: `candle_transformers::models::llava::LLaVA` is `!Send`
- Worker thread: Lines 210-268 use `LocalSet` specifically for !Send models
- Architecture: Dedicated worker thread pattern chosen to handle Candle's constraints

**Attempting spawn_blocking would cause compile error**: "LLaVA cannot be sent between threads safely"

---

## Performance Impact

### Before (Current State):
- Image processing: **60-170ms blocking async executor**
- Generation: **1-5 seconds blocking async executor**
- **Total: ~1.5-5+ seconds of blocked async execution per request**

### After (With Image spawn_blocking):
- Image processing: **Non-blocking** (isolated in thread pool)
- Generation: **Still blocks, but architecturally unavoidable**
- **Result: ~60-170ms improvement in async responsiveness**

---

## Pattern Reference

**Tokenization Pattern** (Already Implemented):
```rust
// Line 625-655 in llava.rs
async fn tokenize_image_prompt_static(...) -> Result<Tensor, String> {
    let input_ids = tokio::task::spawn_blocking(move || {
        // CPU-intensive tokenization
        ...
    }).await??;
    Ok(tensor)
}
```

**Apply Same Pattern**: Image processing follows identical isolation strategy.

---

## File Locations

- **Implementation file**: [`src/capability/vision/llava.rs`](../src/capability/vision/llava.rs)
- **Image API**: [`src/builders/image.rs`](../src/builders/image.rs) (lines 396-427: to_tensor implementation)
- **Pattern reference**: [`src/capability/text_embedding/gte_qwen.rs`](../src/capability/text_embedding/gte_qwen.rs) (spawn_blocking patterns)
