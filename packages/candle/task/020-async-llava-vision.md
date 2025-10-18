# Task 020: Complete async conversion for LLaVA vision model

## Core Objective

Convert LLaVA vision-language model's CPU-intensive operations to async using `spawn_blocking`, enabling non-blocking image processing and text generation while maintaining the existing channel-based architecture for the !Send model.

## Current State Analysis

### File Location
- **Primary file**: [`src/capability/vision/llava.rs`](../src/capability/vision/llava.rs) (850 lines)

### Architecture Overview

LLaVA uses a **worker thread pattern** to handle Candle's `!Send` model constraint:

```rust
// Line 186: Worker thread spawned with current_thread runtime
tokio::task::spawn_blocking(move || {
    let rt = tokio::runtime::Builder::new_current_thread()  // ← ISSUE: No spawn_blocking support
        .enable_all()
        .build()
        .expect("Failed to create worker runtime");
    
    let local = tokio::task::LocalSet::new();
    rt.block_on(local.run_until(async move {
        // Model operations happen here synchronously
    }));
});
```

**Current flow**:
1. Main runtime receives request via `describe_image()` / `describe_url()`
2. Request sent through async channel (`tokio::sync::mpsc::unbounded_channel`)
3. Worker thread receives request and processes **synchronously**
4. Response sent back through async channel
5. Main runtime streams response to caller

### What Works (No Changes Needed)

✅ **Channel architecture** - Already async-compatible:
- Uses `tokio::sync::mpsc::unbounded_channel` (lines 169-170)
- Request/response communication is non-blocking
- Thread spawning uses `tokio::task::spawn_blocking` (line 186)

✅ **Streaming interface** - Already returns `Stream<Item = CandleStringChunk>`

### What Needs Async Conversion

❌ **Worker runtime** - Currently `current_thread` which doesn't support `spawn_blocking`
- Line 187: `new_current_thread()` → Change to `new_multi_thread().worker_threads(1)`

❌ **Image processing** - Synchronous and CPU-intensive:
- Line 449: `ImageBuilder::load()` - File I/O + decode
- Line 450: `.resize()` - Image resizing
- Line 460: Pixel normalization - CPU-intensive tensor ops
- **Needs**: Wrap entire `process_image_static()` in `spawn_blocking`

❌ **Tokenization** - Synchronous and CPU-intensive:
- Line 694: `tokenizer.encode()` - CPU-intensive
- **Needs**: Wrap in `spawn_blocking`

❌ **Model forward passes** - Synchronous and CPU-intensive:
- Line 568: `model.vision_encoder.forward()` - Vision encoding
- Line 572: `model.llama.embed()` - Text embedding
- Line 613: `model.llama.forward()` - Language model forward
- Line 638: `model.llama.embed()` - Token embedding in generation loop
- **Needs**: Wrap each forward pass in `spawn_blocking`

❌ **Generation loop** - Synchronous tensor operations:
- Lines 604-648: Autoregressive generation loop
- Tensor operations: `unsqueeze`, `cat`, `squeeze`, `argmax`
- **Needs**: Wrap generation logic in `spawn_blocking`

## Pattern Reference: GTE-Qwen Async Conversion

See [`src/capability/text_embedding/gte_qwen.rs`](../src/capability/text_embedding/gte_qwen.rs) for reference pattern:

```rust
// Pattern 1: Async forward_pass signature
async fn forward_pass_with_task(
    tokenizer: Tokenizer,
    model: Model,
    device: Device,
    texts: Vec<String>,
    task: Option<String>,
) -> Result<(Model, Vec<Vec<f32>>)> {
    
    // Pattern 2: Wrap tokenization in spawn_blocking
    let tokenizer_clone = tokenizer.clone();
    let tokens = tokio::task::spawn_blocking(move || {
        tokenizer_clone
            .encode_batch(formatted_texts, true)
            .map_err(|e| MemoryError::ModelError(format!("Tokenization failed: {}", e)))
    })
    .await
    .map_err(|e| MemoryError::ModelError(format!("Spawn blocking failed: {}", e)))??;

    // Pattern 3: Wrap model.forward in spawn_blocking
    let (returned_model, logits) = tokio::task::spawn_blocking(move || {
        let mut model_mut = model;
        let result = model_mut
            .forward(&token_ids_clone, 0, None)
            .map_err(|e| MemoryError::ModelError(format!("Forward pass failed: {}", e)));
        (model_mut, result)
    })
    .await
    .map_err(|e| MemoryError::ModelError(format!("Spawn blocking failed: {}", e)))?;

    // Pattern 4: Wrap embedding extraction in spawn_blocking
    let embeddings_data = tokio::task::spawn_blocking(move || {
        // CPU-intensive tensor operations
        pooled_embeddings
            .to_vec2::<f32>()
            .map_err(|e| MemoryError::ModelError(format!("Failed to convert embeddings: {}", e)))
    })
    .await
    .map_err(|e| MemoryError::ModelError(format!("Spawn blocking failed: {}", e)))??;

    Ok((returned_model, embeddings_data))
}
```

## Required Changes

### Change 1: Upgrade Worker Runtime (Line 187)

**Current**:
```rust
let rt = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .expect("Failed to create worker runtime");
```

**Required**:
```rust
let rt = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(1)  // Single worker thread, but enables spawn_blocking
    .enable_all()
    .build()
    .expect("Failed to create worker runtime");
```

**Why**: `current_thread` runtime doesn't support `spawn_blocking`. Multi-threaded runtime with 1 worker enables spawn_blocking while keeping single-threaded execution.

### Change 2: Make `process_image_static` Async (Lines 434-485)

**Current signature**:
```rust
fn process_image_static(
    image_path: &str,
    configs: &ImageProcessingConfig,
    device: &Device,
) -> Result<Tensor, String> {
    // Synchronous image loading and processing
}
```

**Required signature**:
```rust
async fn process_image_static(
    image_path: String,
    image_size: usize,
    image_mean: [f32; 3],
    image_std: [f32; 3],
    device: Device,
) -> Result<Tensor, String> {
    // Wrap CPU-intensive operations in spawn_blocking
    
    // Step 1: Load and resize image (CPU-intensive I/O + image decode)
    let image = tokio::task::spawn_blocking(move || {
        ImageBuilder::load(&image_path)
            .map_err(|e| format!("Image load failed: {}", e))?
            .resize(image_size, image_size, ResizeFilter::Triangle)
            .map_err(|e| format!("Image resize failed: {}", e))?
            .build()
            .map_err(|e| format!("Image build failed: {}", e))
    })
    .await
    .map_err(|e| format!("Spawn blocking failed: {}", e))??;

    // Step 2: Convert to tensor and normalize (CPU-intensive)
    let pixel_values = tokio::task::spawn_blocking(move || {
        let img = image.to_rgb8();
        let data = img.as_raw();
        
        Tensor::from_vec(
            data.clone(),
            (image_size, image_size, 3),
            &device,
        )
        .map_err(|e| format!("Tensor creation failed: {}", e))?
        .permute((2, 0, 1))
        .map_err(|e| format!("Permute failed: {}", e))?
        .to_dtype(candle_core::DType::F32)
        .map_err(|e| format!("To dtype failed: {}", e))?
        .affine(1. / 255., 0.)
        .map_err(|e| format!("Affine scale failed: {}", e))?
        // Normalize with mean/std
        // ... normalization code ...
    })
    .await
    .map_err(|e| format!("Spawn blocking failed: {}", e))??;
    
    Ok(pixel_values)
}
```

### Change 3: Make `process_url_static` Async (Lines 487-538)

Same pattern as `process_image_static`:
- Change signature to `async fn`
- Take owned parameters instead of references
- Wrap image download in `spawn_blocking`
- Wrap image processing in `spawn_blocking`

### Change 4: Make `generate_text_static` Async (Lines 588-652)

**Current**: Synchronous generation loop with tensor operations

**Required**: Wrap each generation step in `spawn_blocking`:

```rust
async fn generate_text_static(
    model: &LLaVA,
    tokenizer: &Tokenizer,
    // ... other params
) -> Result<String, String> {
    let mut generated_text = String::new();
    let mut index_pos = 0;
    let mut current_embeds = image_embeds.clone();

    // Process prompt (wrap in spawn_blocking)
    let model_ref = model as *const LLaVA;  // Raw pointer for !Send
    let logits = tokio::task::spawn_blocking(move || {
        let model = unsafe { &*model_ref };
        model.llama.forward(&prompt_embeds, index_pos, &mut kv_cache)
    })
    .await
    .map_err(|e| format!("Spawn blocking failed: {}", e))??;

    // Generation loop
    for _step in 0..max_new_tokens {
        // Wrap each generation step in spawn_blocking
        let (next_token, next_embeds) = tokio::task::spawn_blocking(move || {
            // Forward pass
            let logits = model.llama.forward(&current_embeds, index_pos, &mut kv_cache)?;
            
            // Sample token
            let next_token = Self::sample_token_static(temperature, &logits)?;
            
            // Embed next token
            let next_embeds = model.llama.embed(&next_token_tensor)?;
            
            Ok::<_, String>((next_token, next_embeds))
        })
        .await
        .map_err(|e| format!("Spawn blocking failed: {}", e))??;

        // Check EOS and decode (fast, keep outside spawn_blocking)
        if next_token == eos_token_id {
            break;
        }
        
        if let Ok(text) = tokenizer.decode(&[next_token], false) {
            generated_text.push_str(&text);
        }
        
        current_embeds = next_embeds;
        index_pos += 1;
    }

    Ok(generated_text)
}
```

**Challenge**: Model is `!Send`, so can't move across threads. Solutions:
1. **Raw pointer pattern** (shown above) - Use raw pointer to pass !Send reference
2. **Keep model on LocalSet** - Don't use spawn_blocking for model operations, only for tensor math
3. **Refactor architecture** - This may require deeper changes

### Change 5: Update Tokenization (Line 694)

Wrap tokenization in `spawn_blocking`:

```rust
async fn tokenize_image_prompt_static(
    tokenizer: Tokenizer,  // Owned, not reference
    llava_config: &CandleLLaVAConfig,
    device: &Device,
    prompt: String,  // Owned, not reference
) -> Result<Tensor, String> {
    tokio::task::spawn_blocking(move || {
        // Tokenization logic (CPU-intensive)
        // ... existing code ...
    })
    .await
    .map_err(|e| format!("Spawn blocking failed: {}", e))?
}
```

## Implementation Steps

1. **Change worker runtime** (Line 187)
   - `new_current_thread()` → `new_multi_thread().worker_threads(1)`
   - Verify spawn_blocking is available within worker

2. **Convert `process_image_static` to async**
   - Change signature to `async fn` with owned parameters
   - Wrap ImageBuilder operations in spawn_blocking
   - Wrap tensor operations in spawn_blocking
   - Update call sites with `.await`

3. **Convert `process_url_static` to async**
   - Same pattern as `process_image_static`
   - Wrap HTTP download in spawn_blocking
   - Wrap image processing in spawn_blocking

4. **Convert `tokenize_image_prompt_static` to async**
   - Change signature to `async fn` with owned parameters
   - Wrap tokenization in spawn_blocking
   - Update call sites with `.await`

5. **Convert `generate_text_static` to async**
   - This is the most complex change
   - Options:
     a. Use raw pointer pattern for !Send model
     b. Keep model operations sync, wrap tensor math only
     c. Refactor to pass model operations through channels
   - Choose option (b) initially - minimal changes, safe

6. **Update `model_task_with_config` call sites**
   - Ensure all async functions are awaited
   - Verify error handling propagates correctly

7. **Compile and verify**
   - `cargo check --manifest-path ./Cargo.toml`
   - `cargo build --manifest-path ./Cargo.toml`
   - Verify no unwrap() or expect() in new code
   - Verify no block_on in async paths

## Definition of Done

✅ Worker runtime changed to `new_multi_thread().worker_threads(1)`
✅ `process_image_static` is async with spawn_blocking for CPU-intensive ops
✅ `process_url_static` is async with spawn_blocking for CPU-intensive ops  
✅ `tokenize_image_prompt_static` is async with spawn_blocking
✅ `generate_text_static` handles async properly (model operations may remain sync on LocalSet)
✅ All async functions use `.await` at call sites
✅ No unwrap() or expect() in implementation (use map_err with descriptive messages)
✅ No block_on in async code paths
✅ Code compiles with `cargo build`
✅ Channel architecture remains unchanged (already async-compatible)

## Notes on Architecture

The LLaVA architecture is **inherently complex** due to Candle's `!Send` constraint:
- Model cannot move between threads
- Worker thread pattern is correct approach
- Async conversion focuses on making CPU-intensive ops non-blocking WITHIN the worker
- The worker runtime needs spawn_blocking support, hence multi_thread with 1 worker

This task maintains the existing architecture while making operations async where beneficial.

## Dependencies

- Task 001 (async model.forward) - Referenced pattern for spawn_blocking
- Pattern reference: [`src/capability/text_embedding/gte_qwen.rs`](../src/capability/text_embedding/gte_qwen.rs)
- Pattern reference: [`src/capability/text_embedding/jina_bert.rs`](../src/capability/text_embedding/jina_bert.rs)

## Estimated Effort

6-8 hours due to:
- Multi-modal complexity (vision + language)
- !Send model constraint requiring careful threading
- Generation loop async conversion
- Multiple CPU-intensive operation types (image, tensor, text)
