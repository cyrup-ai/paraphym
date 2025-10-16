# Task: Convert Stable Diffusion 3.5 Turbo to Use Async huggingface_file

## Dependencies
- ⏳ **BLOCKED BY**: [ASYNC_FIX_HUGGINGFACE.md](./ASYNC_FIX_HUGGINGFACE.md) - CandleModel trait must be async first

## Location
`packages/candle/src/capability/text_to_image/stable_diffusion_35_turbo/mod.rs`

## Call Sites to Convert

### 1. Main model files - Lines 116, 130, 144, 158
```rust
// Line 116 - CLIP-G encoder
let clip_g_path = match model_self.huggingface_file(
    model_self.info().registry_key,
    "text_encoders/clip_g.safetensors",
) { /* ... */ };

// Line 130 - CLIP-L encoder
let clip_l_path = match model_self.huggingface_file(
    model_self.info().registry_key,
    "text_encoders/clip_l.safetensors",
) { /* ... */ };

// Line 144 - T5-XXL encoder
let t5xxl_path = match model_self.huggingface_file(
    model_self.info().registry_key,
    "text_encoders/t5xxl_fp16.safetensors",
) { /* ... */ };

// Line 158 - MMDiT main model
let mmdit_path = match model_self.huggingface_file(
    model_self.info().registry_key,
    "sd3.5_large_turbo.safetensors",
) { /* ... */ };
```

### 2. Tokenizer files - Lines 174, 187, 200, 212
```rust
// Line 174 - CLIP-L tokenizer
let clip_l_tokenizer_path = match ClipLTokenizer
    .huggingface_file(ClipLTokenizer.info().registry_key, "tokenizer.json")
{ /* ... */ };

// Line 187 - CLIP-G tokenizer
let clip_g_tokenizer_path = match ClipGTokenizer
    .huggingface_file(ClipGTokenizer.info().registry_key, "tokenizer.json")
{ /* ... */ };

// Line 200 - T5 config
let t5_config_path = match T5ConfigModel
    .huggingface_file(T5ConfigModel.info().registry_key, "config.json")
{ /* ... */ };

// Line 212 - T5 tokenizer
let t5_tokenizer_path = match T5TokenizerModel.huggingface_file(
    T5TokenizerModel.info().registry_key,
    "t5-v1_1-xxl.tokenizer.json",
) { /* ... */ };
```

## Problem
All 8 calls to `huggingface_file()` are synchronous, blocking the async runtime during:
- Large text encoder downloads (CLIP-G, CLIP-L, T5-XXL - multi-GB files)
- Main diffusion model download (sd3.5_large_turbo.safetensors - very large)
- Multiple tokenizer and config downloads

This is particularly critical as Stable Diffusion 3.5 involves downloading **multiple multi-gigabyte files**.

## Solution Steps

### Step 1: Identify the enclosing method
The huggingface_file calls are inside the `generate_image()` method which returns a stream.
This method needs to be made async.

### Step 2: Update method signature
```rust
// Before
fn generate_image(
    &self,
    config: ImageGenerationConfig,
) -> Pin<Box<dyn Stream<Item = ImageGenerationChunk> + Send>>

// After - add async
async fn generate_image(
    &self,
    config: ImageGenerationConfig,
) -> Pin<Box<dyn Stream<Item = ImageGenerationChunk> + Send>>
```

### Step 3: Add `.await` to all huggingface_file calls
```rust
// Main model files
let clip_g_path = match model_self.huggingface_file(
    model_self.info().registry_key,
    "text_encoders/clip_g.safetensors",
).await { /* ... */ };

let clip_l_path = match model_self.huggingface_file(
    model_self.info().registry_key,
    "text_encoders/clip_l.safetensors",
).await { /* ... */ };

let t5xxl_path = match model_self.huggingface_file(
    model_self.info().registry_key,
    "text_encoders/t5xxl_fp16.safetensors",
).await { /* ... */ };

let mmdit_path = match model_self.huggingface_file(
    model_self.info().registry_key,
    "sd3.5_large_turbo.safetensors",
).await { /* ... */ };

// Tokenizer files
let clip_l_tokenizer_path = match ClipLTokenizer
    .huggingface_file(ClipLTokenizer.info().registry_key, "tokenizer.json").await
{ /* ... */ };

let clip_g_tokenizer_path = match ClipGTokenizer
    .huggingface_file(ClipGTokenizer.info().registry_key, "tokenizer.json").await
{ /* ... */ };

let t5_config_path = match T5ConfigModel
    .huggingface_file(T5ConfigModel.info().registry_key, "config.json").await
{ /* ... */ };

let t5_tokenizer_path = match T5TokenizerModel.huggingface_file(
    T5TokenizerModel.info().registry_key,
    "t5-v1_1-xxl.tokenizer.json",
).await { /* ... */ };
```

### Step 4: Update ImageGenerationCapable trait
The trait in `capability/traits.rs` needs `generate_image()` method signature updated to async.

### Step 5: Update all call sites
Search for any code calling:
- `StableDiffusion35Turbo.generate_image()` → Add `.await`

## Testing
1. Test full image generation pipeline with actual downloads
2. Verify all 3 text encoders load correctly (CLIP-G, CLIP-L, T5-XXL)
3. Test main diffusion model loading
4. Verify tokenizers load properly
5. Test image generation produces correct output
6. Ensure no blocking during large file downloads
7. Test progress reporting during downloads

## Priority
🔴 **CRITICAL** - Downloads multiple multi-gigabyte files that will severely block runtime

## Status
⏳ BLOCKED - Waiting for CandleModel trait to be async

## Notes
- This is one of the most critical conversions due to file sizes
- Consider adding progress reporting for these large downloads
- May need to update error handling to preserve download progress info
