# Task: Convert CLIP Vision Image Embedding to Use Async huggingface_file

## Location
`packages/candle/src/capability/image_embedding/clip_vision.rs`

## Dependencies
- ✅ **UNBLOCKED**: Base trait is now async (ASYNC_FIX_HUGGINGFACE.md complete)

## Overview
Convert all `huggingface_file()` calls in CLIP Vision image embedding model to use async/await pattern.

## Call Sites to Convert

Total: **4 call sites** missing `.await`

### Compilation Errors
```
error[E0599]: no method named `map_err` found for opaque type `impl Future<Output = Result<PathBuf, Box<dyn Error + Send + Sync>>>`
   --> packages/candle/src/capability/image_embedding/clip_vision.rs:114:14
   --> packages/candle/src/capability/image_embedding/clip_vision.rs:174:14
   --> packages/candle/src/capability/image_embedding/clip_vision.rs:234:14
   --> packages/candle/src/capability/image_embedding/clip_vision.rs:294:14
```

## Solution Pattern

**Current (broken)**:
```rust
let model_path = self
    .huggingface_file(self.info().registry_key, "model.safetensors")
    .map_err(|e| format!("Failed to get model file: {}", e))?;
```

**Fixed**:
```rust
let model_path = self
    .huggingface_file(self.info().registry_key, "model.safetensors")
    .await
    .map_err(|e| format!("Failed to get model file: {}", e))?;
```

## Steps
1. Add `.await` after each `huggingface_file()` call before `.map_err()`
2. Ensure parent method/function is `async` or uses async closure
3. Test compilation
4. Verify image embedding functionality

## Priority
🔴 **HIGH** - Blocking compilation of image embedding capability

## Status
⏳ TODO
