# Task 021: Complete async conversion for FLUX.1-schnell text-to-image

## Core Objective

Wrap CPU-intensive synchronous operations in `spawn_blocking` to prevent blocking the async runtime. The main `generate()` method already returns a Stream, but internal operations (T5 encoding, CLIP encoding, diffusion sampling, VAE decode) execute synchronously and block the runtime.

**File**: [`src/capability/text_to_image/flux_schnell.rs`](../src/capability/text_to_image/flux_schnell.rs) (580 lines)

## Current State Analysis

### What's Already Async ✅

1. **Main generate() method** (Line 56) - Returns `Pin<Box<dyn Stream>>` ✅
2. **T5WithTokenizer::load()** (Line 431) - `async fn` with `tokio::fs::read_to_string` ✅
3. **spawn_stream wrapper** (Line 64) - Uses `crate::async_stream::spawn_stream` ✅

### What's Synchronous (Needs spawn_blocking) ❌

**1. T5 Text Encoding** - Line 459
```rust
fn encode(&mut self, text: &str, device: &Device) -> Result<Tensor, String> {
    // Tokenize (fast)
    let mut tokens = self.tokenizer.encode(text, true)?.get_ids().to_vec();
    tokens.resize(256, 0);
    
    // Model forward pass (CPU-intensive) - SYNC
    let tokens_tensor = Tensor::new(&tokens[..], device)?.unsqueeze(0)?;
    self.t5.forward(&tokens_tensor)  // ❌ BLOCKS RUNTIME (Line 476)
        .map_err(|e| format!("T5 forward failed: {}", e))
}
```
**Issue**: `self.t5.forward()` is CPU-intensive and synchronous
**Impact**: **200-500ms** per encoding (T5-XXL is large model)

**2. CLIP Text Encoding** - Line 517
```rust
fn encode(&self, text: &str, device: &Device) -> Result<Tensor, String> {
    // Tokenize (fast)
    let tokens = self.tokenizer.encode(text, true)?.get_ids().to_vec();
    
    // Model forward pass (CPU-intensive) - SYNC
    let tokens_tensor = Tensor::new(&tokens[..], device)?.unsqueeze(0)?;
    self.clip.forward(&tokens_tensor)  // ❌ BLOCKS RUNTIME (Line 527)
        .map_err(|e| format!("CLIP forward failed: {}", e))
}
```
**Issue**: `self.clip.forward()` is CPU-intensive and synchronous
**Impact**: **50-100ms** per encoding

**3. Image Generation Pipeline** - Line 355 (entire `generate_flux_image` function)
```rust
fn generate_flux_image(...) -> Result<Tensor, String> {
    // 1. Initialize noise (fast)
    let img = flux::sampling::get_noise(...)?.to_dtype(...)?;
    
    // 2. Prepare state (fast)
    let state = flux::sampling::State::new(t5_emb, clip_emb, &img)?;
    
    // 3. Get timesteps (fast)
    let timesteps = flux::sampling::get_schedule(4, None);
    
    // 4. Progress tracking (fast)
    for (step, _window) in timesteps.windows(2).enumerate() {
        let _ = sender.send(ImageGenerationChunk::Step { ... });
    }
    
    // 5. Run denoising (CPU-INTENSIVE) - SYNC
    let denoised = flux::sampling::denoise(  // ❌ BLOCKS RUNTIME (Line 384-392)
        flux_transformer,  // 4-step diffusion loop
        &state.img,
        &state.img_ids,
        &state.txt,
        &state.txt_ids,
        &state.vec,
        &timesteps,
        guidance,
    )?;
    
    // 6. Unpack (moderate)
    let unpacked = flux::sampling::unpack(&denoised, height, width)?;
    
    // 7. VAE decode (CPU-INTENSIVE) - SYNC
    let decoded = vae.decode(&unpacked)?;  // ❌ BLOCKS RUNTIME (Line 400)
    
    // 8. Post-process (fast)
    let image = ((decoded.clamp(-1f32, 1f32)? + 1.0)? * 0.5)?;
    
    Ok(image)
}
```
**Issue**: Entire function runs synchronously with multiple CPU-intensive operations
**Impact**: **2-5 seconds** total for 4-step generation (blocks entire runtime)

## Required Changes

### Change 1: Make T5WithTokenizer::encode() Async

**Current** (Line 459):
```rust
fn encode(&mut self, text: &str, device: &Device) -> Result<Tensor, String> {
    // Sync tokenization + forward pass
}
```

**Required**:
```rust
async fn encode(&mut self, text: &str, device: &Device) -> Result<Tensor, String> {
    // Tokenize (fast, keep sync)
    let mut tokens = self
        .tokenizer
        .encode(text, true)
        .map_err(|e| format!("T5 tokenization failed: {}", e))?
        .get_ids()
        .to_vec();
    tokens.resize(256, 0);
    
    // Wrap CPU-intensive operations in spawn_blocking
    let tokens_clone = tokens.clone();
    let device_clone = device.clone();
    
    tokio::task::spawn_blocking(move || {
        // Create tensor (CPU-intensive)
        let tokens_tensor = Tensor::new(&tokens_clone[..], &device_clone)
            .map_err(|e| format!("T5 token tensor creation failed: {}", e))?
            .unsqueeze(0)
            .map_err(|e| format!("T5 unsqueeze failed: {}", e))?;
        
        // Forward pass (CPU-intensive) - Note: t5 is !Send, needs special handling
        // This will require capturing t5 with raw pointer or refactoring
        // See "Model !Send Constraint" section below
        
        Ok::<Tensor, String>(result)
    })
    .await
    .map_err(|e| format!("Spawn blocking failed: {}", e))?
}
```

**Challenge**: T5EncoderModel is likely `!Send`, similar to LLaVA model. See "Model !Send Constraint" section below for solution.

### Change 2: Make ClipWithTokenizer::encode() Async

**Current** (Line 517):
```rust
fn encode(&self, text: &str, device: &Device) -> Result<Tensor, String> {
    // Sync tokenization + forward pass
}
```

**Required**: Same pattern as T5WithTokenizer::encode() - wrap tensor creation and forward pass in spawn_blocking.

### Change 3: Make generate_flux_image() Async

**Current** (Line 355):
```rust
fn generate_flux_image(
    flux_transformer: &flux::model::Flux,
    vae: &flux::autoencoder::AutoEncoder,
    t5_emb: &Tensor,
    clip_emb: &Tensor,
    config: &ImageGenerationConfig,
    device: &Device,
    sender: &tokio::sync::mpsc::UnboundedSender<ImageGenerationChunk>,
) -> Result<Tensor, String> {
    // Entirely synchronous
}
```

**Required**:
```rust
async fn generate_flux_image(
    flux_transformer: &flux::model::Flux,
    vae: &flux::autoencoder::AutoEncoder,
    t5_emb: &Tensor,
    clip_emb: &Tensor,
    config: &ImageGenerationConfig,
    device: &Device,
    sender: &tokio::sync::mpsc::UnboundedSender<ImageGenerationChunk>,
) -> Result<Tensor, String> {
    // Fast operations (keep sync)
    let img = flux::sampling::get_noise(1, config.height, config.width, device)?
        .to_dtype(t5_emb.dtype())?;
    let state = flux::sampling::State::new(t5_emb, clip_emb, &img)?;
    let timesteps = flux::sampling::get_schedule(4, None);
    
    // Progress tracking (fast, keep sync)
    let total_steps = timesteps.len().saturating_sub(1);
    for (step, _window) in timesteps.windows(2).enumerate() {
        let _ = sender.send(ImageGenerationChunk::Step {
            step,
            total: total_steps,
            latent: state.img.clone(),
        });
    }
    
    // Wrap CPU-intensive denoising in spawn_blocking
    let guidance = 0.0;
    let state_clone = state.clone();  // May need to adjust based on Clone impl
    let timesteps_clone = timesteps.clone();
    
    let denoised = tokio::task::spawn_blocking(move || {
        flux::sampling::denoise(
            flux_transformer,  // Note: May be !Send - see constraint section
            &state_clone.img,
            &state_clone.img_ids,
            &state_clone.txt,
            &state_clone.txt_ids,
            &state_clone.vec,
            &timesteps_clone,
            guidance,
        )
    })
    .await
    .map_err(|e| format!("Spawn blocking failed: {}", e))?
    .map_err(|e| format!("Denoising failed: {}", e))?;
    
    // Unpack (moderate, keep in spawn_blocking)
    let height = config.height;
    let width = config.width;
    let unpacked = tokio::task::spawn_blocking(move || {
        flux::sampling::unpack(&denoised, height, width)
    })
    .await
    .map_err(|e| format!("Spawn blocking failed: {}", e))?
    .map_err(|e| format!("Unpack failed: {}", e))?;
    
    // VAE decode (CPU-intensive, wrap in spawn_blocking)
    let decoded = tokio::task::spawn_blocking(move || {
        vae.decode(&unpacked)
    })
    .await
    .map_err(|e| format!("Spawn blocking failed: {}", e))?
    .map_err(|e| format!("VAE decode failed: {}", e))?;
    
    // Post-process (fast, can stay sync)
    let image = ((decoded
        .clamp(-1f32, 1f32)?
        + 1.0)?
        * 0.5)?;
    
    Ok(image)
}
```

### Change 4: Update Call Sites

**Main generate() method** (Line 235) - Update calls:

**Current**:
```rust
let t5_emb = match t5_encoder.encode(&prompt, &device) {  // Line 235
    Ok(emb) => emb,
    Err(e) => { /* error */ }
};

let clip_emb = match clip_encoder.encode(&prompt, &device) {  // Line 243
    Ok(emb) => emb,
    Err(e) => { /* error */ }
};

let image = match generate_flux_image(  // Line 320
    &flux_transformer,
    &vae,
    &t5_emb,
    &clip_emb,
    &generation_config,
    &device,
    &tx,
) {
    Ok(result) => result,
    Err(e) => { /* error */ }
};
```

**Required**:
```rust
let t5_emb = match t5_encoder.encode(&prompt, &device).await {  // Add .await
    Ok(emb) => emb,
    Err(e) => { /* error */ }
};

let clip_emb = match clip_encoder.encode(&prompt, &device).await {  // Add .await
    Ok(emb) => emb,
    Err(e) => { /* error */ }
};

let image = match generate_flux_image(  // Add .await
    &flux_transformer,
    &vae,
    &t5_emb,
    &clip_emb,
    &generation_config,
    &device,
    &tx,
).await {
    Ok(result) => result,
    Err(e) => { /* error */ }
};
```

## Model !Send Constraint

**Problem**: FLUX models (T5EncoderModel, ClipTextTransformer, flux::model::Flux, flux::autoencoder::AutoEncoder) are likely `!Send` because they contain Candle tensors and model state.

**Solution Options**:

### Option A: Keep Models Synchronous (Recommended for Initial Implementation)

Since the entire `generate()` method runs in `spawn_stream` which already isolates it from the main runtime, we can keep the model operations synchronous and only wrap the *outer* generate call:

**Current** (Line 56-348):
```rust
fn generate(...) -> Pin<Box<dyn Stream<Item = ImageGenerationChunk> + Send>> {
    Box::pin(crate::async_stream::spawn_stream(move |tx| async move {
        // All model loading and generation happens here (sync)
    }))
}
```

**This is already correct!** The `spawn_stream` uses `tokio::spawn` which moves the entire closure to a separate task. The issue is that *within* this task, we have long-running synchronous operations that should yield.

**Refined Solution**: Add `tokio::task::yield_now()` calls between CPU-intensive operations:

```rust
// After T5 encoding
let t5_emb = t5_encoder.encode(&prompt, &device)?;
tokio::task::yield_now().await;  // Allow other tasks to run

// After CLIP encoding  
let clip_emb = clip_encoder.encode(&prompt, &device)?;
tokio::task::yield_now().await;

// Between diffusion steps (modify denoise to be async with yields)
```

### Option B: Use Raw Pointer Pattern (Advanced)

Similar to LLaVA (Task 020), use raw pointers to pass `!Send` models into spawn_blocking:

```rust
let model_ptr = flux_transformer as *const flux::model::Flux;
tokio::task::spawn_blocking(move || {
    let model = unsafe { &*model_ptr };
    flux::sampling::denoise(model, ...)
})
.await?
```

**Note**: This is unsafe and requires careful lifetime management.

## Recommended Implementation Strategy

Given the complexity of FLUX with multiple models and the !Send constraint, the recommended approach is:

### Phase 1: Add Yielding (Low Risk)

1. Keep all operations synchronous
2. Add `tokio::task::yield_now().await` between major operations
3. This allows the async runtime to remain responsive

**Changes**:
- After T5 encoding (line 241)
- After CLIP encoding (line 249)
- After FLUX model loading (line 303)
- After VAE loading (line 319)
- Between diffusion steps in generate_flux_image

### Phase 2: Full Async Conversion (Higher Risk)

Only if Phase 1 is insufficient:
1. Make encode methods async
2. Make generate_flux_image async
3. Wrap CPU-intensive operations in spawn_blocking
4. Handle !Send constraint with raw pointers or refactoring

## Performance Impact

**Current State**:
- T5 encoding: **200-500ms** blocking
- CLIP encoding: **50-100ms** blocking
- Diffusion (4 steps): **1.5-3 seconds** blocking
- VAE decode: **200-400ms** blocking
- **Total: ~2-5 seconds** of blocked async runtime per image

**After Phase 1 (Yielding)**:
- Same wall-clock time but runtime remains responsive
- Other tasks can interleave execution

**After Phase 2 (Full Async)**:
- CPU-intensive ops isolated in thread pool
- Async runtime never blocks
- True concurrent request processing

## Definition of Done

### Phase 1 (Minimum)
✅ Add `tokio::task::yield_now().await` after each major operation
✅ Runtime remains responsive during generation
✅ Code compiles with `cargo check`

### Phase 2 (Full)
✅ `T5WithTokenizer::encode()` is async
✅ `ClipWithTokenizer::encode()` is async
✅ `generate_flux_image()` is async
✅ CPU-intensive operations wrapped in spawn_blocking or yielding
✅ All call sites updated with `.await`
✅ No unwrap() or expect() in implementation
✅ Code compiles with `cargo check`

## File Locations

- **Implementation**: [`src/capability/text_to_image/flux_schnell.rs`](../src/capability/text_to_image/flux_schnell.rs)
- **Pattern reference**: [`src/capability/image_embedding/clip_vision.rs`](../src/capability/image_embedding/clip_vision.rs) (spawn_blocking pattern)
- **Pattern reference**: [`src/capability/vision/llava.rs`](../src/capability/vision/llava.rs) (!Send model handling)

## Notes

- FLUX.1-schnell uses 4-step diffusion (faster than standard diffusion)
- Guidance scale is 0.0 (no classifier-free guidance)
- Dual text encoding: T5-XXL (256 tokens) + CLIP-L (77 tokens)
- Models are likely `!Send` due to Candle's architecture
- The current `spawn_stream` wrapper already provides some isolation

## Estimated Effort

**Phase 1**: 1-2 hours (add yielding)
**Phase 2**: 4-6 hours (full async conversion with !Send handling)
