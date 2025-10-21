# Task: Fix Registry Lazy Loading for Image Embedding and Text-to-Image Models

**Status**: Ready for Execution
**Priority**: CRITICAL - Blocking MULTIMODAL.md
**Complexity**: Low

## Overview

Two entire registry categories are completely empty, breaking the user's expectation that "calling the registry the model loads". The IMAGE_EMBEDDING_UNIFIED and TEXT_TO_IMAGE_UNIFIED registries start empty and have NO lazy loading mechanism, so `registry::get()` always returns `None`.

## Problem

**File**: `packages/candle/src/capability/registry/storage.rs`

Lines 78-90 show the broken registries:

```rust
/// Unified image embedding model registry
///
/// Starts empty because ClipVision requires local model files, not HF downloads.
/// Use runtime registration after downloading weights manually.
pub(super) static IMAGE_EMBEDDING_UNIFIED: LazyLock<RwLock<HashMap<String, ImageEmbeddingModel>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));  // ← EMPTY!

/// Unified text-to-image model registry
///
/// Starts empty because Flux/SD require local model files, not HF downloads.
/// Use runtime registration after downloading weights manually.
pub(super) static TEXT_TO_IMAGE_UNIFIED: LazyLock<RwLock<HashMap<String, TextToImageModel>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));  // ← EMPTY!
```

**The comment "Use runtime registration after downloading weights manually" is nonsense:**
1. Nothing ever calls `register_image_embedding()` or `register_text_to_image()`
2. "Manually" violates the "automate everything" principle
3. The models DO use HuggingFace downloads via `huggingface_file()`

**File**: `packages/candle/src/capability/registry/api.rs`

Lines 62-73 show the broken lookup:

```rust
impl FromRegistry for ImageEmbeddingModel {
    fn from_registry(registry_key: &str) -> Option<Self> {
        let registry = IMAGE_EMBEDDING_UNIFIED.read();
        registry.get(registry_key).cloned()  // ← HashMap is empty, always returns None
    }
}

impl FromRegistry for TextToImageModel {
    fn from_registry(registry_key: &str) -> Option<Self> {
        let registry = TEXT_TO_IMAGE_UNIFIED.read();
        registry.get(registry_key).cloned()  // ← HashMap is empty, always returns None
    }
}
```

## Current Behavior (Broken)

```rust
// User code
let model = registry::get::<ImageEmbeddingModel>("openai/clip-vit-base-patch32");
// Returns None because IMAGE_EMBEDDING_UNIFIED is empty

// This is expected to work but doesn't:
let model = registry::get::<TextToImageModel>("black-forest-labs/FLUX.1-schnell");
// Returns None because TEXT_TO_IMAGE_UNIFIED is empty
```

## Expected Behavior (Fix)

User expectation: "calling the registry the model loads"

```rust
// First call: lazy loads + registers + returns model
let model = registry::get::<ImageEmbeddingModel>("openai/clip-vit-base-patch32").unwrap();

// Second call: fast HashMap lookup
let model = registry::get::<ImageEmbeddingModel>("openai/clip-vit-base-patch32").unwrap();
```

## Solution

Add lazy loading to `FromRegistry` implementations in `api.rs`.

### Part 1: Fix ImageEmbeddingModel (lines 62-67)

**Replace**:
```rust
impl FromRegistry for ImageEmbeddingModel {
    fn from_registry(registry_key: &str) -> Option<Self> {
        let registry = IMAGE_EMBEDDING_UNIFIED.read();
        registry.get(registry_key).cloned()
    }
}
```

**With**:
```rust
impl FromRegistry for ImageEmbeddingModel {
    fn from_registry(registry_key: &str) -> Option<Self> {
        // First, try normal lookup
        {
            let registry = IMAGE_EMBEDDING_UNIFIED.read();
            if let Some(model) = registry.get(registry_key).cloned() {
                return Some(model);
            }
        }

        // Lazy registration for CLIP Vision models
        match registry_key {
            "openai/clip-vit-base-patch32" => {
                use crate::capability::image_embedding::{ClipVisionModel, ClipVisionEmbeddingModel};
                use std::sync::Arc;

                // Synchronous initialization (ClipVisionModel::new is sync!)
                let clip_model = ClipVisionModel::new(512).ok()?;
                let embedding_model = ClipVisionEmbeddingModel::from_model(clip_model, 512);
                let registry_model = ImageEmbeddingModel::ClipVision(Arc::new(embedding_model));

                // Register for future lookups
                let mut registry = IMAGE_EMBEDDING_UNIFIED.write();
                registry.insert(registry_key.to_string(), registry_model.clone());

                Some(registry_model)
            }
            "openai/clip-vit-large-patch14-336" => {
                use crate::capability::image_embedding::{ClipVisionModel, ClipVisionEmbeddingModel};
                use std::sync::Arc;

                let clip_model = ClipVisionModel::new(768).ok()?;
                let embedding_model = ClipVisionEmbeddingModel::from_model(clip_model, 768);
                let registry_model = ImageEmbeddingModel::ClipVision(Arc::new(embedding_model));

                let mut registry = IMAGE_EMBEDDING_UNIFIED.write();
                registry.insert(registry_key.to_string(), registry_model.clone());

                Some(registry_model)
            }
            _ => None
        }
    }
}
```

### Part 2: Fix TextToImageModel (lines 69-74)

**Replace**:
```rust
impl FromRegistry for TextToImageModel {
    fn from_registry(registry_key: &str) -> Option<Self> {
        let registry = TEXT_TO_IMAGE_UNIFIED.read();
        registry.get(registry_key).cloned()
    }
}
```

**With**:
```rust
impl FromRegistry for TextToImageModel {
    fn from_registry(registry_key: &str) -> Option<Self> {
        // First, try normal lookup
        {
            let registry = TEXT_TO_IMAGE_UNIFIED.read();
            if let Some(model) = registry.get(registry_key).cloned() {
                return Some(model);
            }
        }

        // Lazy registration for text-to-image models
        match registry_key {
            "black-forest-labs/FLUX.1-schnell" => {
                use crate::capability::text_to_image::FluxSchnell;
                use std::sync::Arc;

                // FluxSchnell::new() is sync and lightweight (no downloads yet)
                let model = FluxSchnell::new();
                let registry_model = TextToImageModel::FluxSchnell(Arc::new(model));

                // Register for future lookups
                let mut registry = TEXT_TO_IMAGE_UNIFIED.write();
                registry.insert(registry_key.to_string(), registry_model.clone());

                Some(registry_model)
            }
            "stabilityai/stable-diffusion-3.5-large-turbo" => {
                use crate::capability::text_to_image::StableDiffusion35Turbo;
                use std::sync::Arc;

                let model = StableDiffusion35Turbo::new();
                let registry_model = TextToImageModel::StableDiffusion35Turbo(Arc::new(model));

                let mut registry = TEXT_TO_IMAGE_UNIFIED.write();
                registry.insert(registry_key.to_string(), registry_model.clone());

                Some(registry_model)
            }
            _ => None
        }
    }
}
```

### Part 3: Update storage.rs Comments (lines 78-90)

**Replace misleading comments**:
```rust
/// Starts empty because ClipVision requires local model files, not HF downloads.
/// Use runtime registration after downloading weights manually.
```

**With accurate comments**:
```rust
/// Starts empty and uses lazy loading on first registry::get() access.
/// Models are automatically created and registered when requested.
```

Same for TEXT_TO_IMAGE_UNIFIED.

## Why This Works

### ClipVision Lazy Loading:
1. `ClipVisionModel::new(512)` is **sync** (models.rs:41) - no async needed!
2. Just stores dimension, no downloads yet
3. Downloads happen later when `embed_image()` is called
4. Workers spawn via `ensure_workers_spawned_adaptive()` on first embedding

### FluxSchnell/SD3.5 Lazy Loading:
1. `FluxSchnell::new()` is sync (flux_schnell.rs:71-76)
2. Just initializes empty Arc<Mutex<Option<channel>>>
3. Downloads happen later when `generate()` is called
4. Worker thread spawns lazily on first use

### Performance:
- First call: Creates model wrapper + registers (microseconds)
- Subsequent calls: Fast HashMap lookup (nanoseconds)
- Heavy work (downloads, model loading) happens later when methods are called

## Implementation Checklist

- [ ] Add lazy loading to `FromRegistry for ImageEmbeddingModel` in api.rs
- [ ] Add lazy loading to `FromRegistry for TextToImageModel` in api.rs
- [ ] Update misleading comments in storage.rs
- [ ] Add necessary imports to api.rs
- [ ] Run `cargo check -p paraphym_candle --color=never`
- [ ] Verify registry::get() works for all models

## Success Criteria

✅ `registry::get::<ImageEmbeddingModel>("openai/clip-vit-base-patch32")` returns model on first call
✅ `registry::get::<ImageEmbeddingModel>("openai/clip-vit-large-patch14-336")` returns model on first call
✅ `registry::get::<TextToImageModel>("black-forest-labs/FLUX.1-schnell")` returns model on first call
✅ `registry::get::<TextToImageModel>("stabilityai/stable-diffusion-3.5-large-turbo")` returns model on first call
✅ Subsequent calls use fast HashMap lookup
✅ NO "manual registration" needed
✅ Matches user expectation: "calling the registry the model loads"
✅ MULTIMODAL.md can now use `.unwrap()` instead of "if available" checks
✅ `cargo check` passes

## Notes

- This fixes the bug the user identified: "claude fuckery where image embedding models don't properly use the registry"
- The original comment "Use runtime registration after downloading weights manually" was Claude being lazy
- All models use `huggingface_file()` for downloads (verified in audit)
- Lazy loading is the CORRECT pattern - models download on first use, not on registration
- After this fix, the registry will work as the user expects: transparently and automatically
