# Task: Automatic Multi-Modal Image Embedding Processing

**Status**: Ready for Execution
**Priority**: High
**Complexity**: Low

## Overview

Restore automatic image embedding processing that was disabled. When metadata contains `image_path`, the system will automatically query the registry for an image embedding model, generate the image embedding, and process it through the cognitive state - all transparently without any user configuration.

## Objective

Enable automatic multi-modal cognitive processing by adding code blocks to existing methods that:
1. Detect image paths in metadata
2. Query the global registry for image embedding model
3. Generate image embeddings automatically
4. Process through existing cognitive_state field
5. Maintain zero API changes (happens "automagically")

## Key Principle: NO NEW FIELDS OR CONSTRUCTORS

This follows the same pattern as activation processing (operations.rs:171-181):
- Check IF condition is met (metadata has image_path)
- Use EXISTING fields (cognitive_state already exists)
- Query registry DYNAMICALLY when needed
- NO new MemoryCoordinator fields
- NO with_*() constructors
- NO user API changes

## Background: The Registry System

From capability/registry/mod.rs:
- Global unified registries for all model capabilities
- `registry::get::<ImageEmbeddingModel>(key)` - query any registered model
- Models are available GLOBALLY without fields
- No initialization needed - registry is always accessible

Example usage (from api.rs:63):
```rust
use crate::capability::registry;

let model = registry::get::<ImageEmbeddingModel>("clip-vision")?;
let embedding = model.embed_image(&path).await?;
```

## Technical Details

### File: packages/candle/src/memory/core/manager/coordinator/operations.rs

**Location 1: After Text Embedding Generation (after line 125)**

Current code (lines 120-125):
```rust
// Generate embedding using BERT
let embedding = self.generate_embedding(&content).await?;
domain_memory.embedding =
    Some(crate::domain::memory::primitives::node::AlignedEmbedding::new(
        embedding,
    ));
```

**Add after line 125:**
```rust
// Automatic image embedding if metadata contains image_path
if let Some(metadata) = &metadata {
    if let Some(image_path_value) = metadata.custom.get("image_path") {
        if let Some(image_path) = image_path_value.as_str() {
            // Query registry for image embedding model (no field needed!)
            use crate::capability::registry;
            use crate::capability::registry::ImageEmbeddingModel;
            use crate::capability::traits::ImageEmbeddingCapable;

            if let Some(vision_model) = registry::get::<ImageEmbeddingModel>("clip-vision") {
                match vision_model.embed_image(image_path).await {
                    Ok(image_embedding) => {
                        // Process image embedding through cognitive state (same as text)
                        match self.cognitive_state.write().await
                            .update_activation_from_stimulus(image_embedding) {
                            Ok(()) => {
                                log::trace!("Updated cognitive activation from image: {}", image_path);
                            }
                            Err(e) => {
                                log::warn!("Failed to update cognitive activation from image: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!("Failed to generate image embedding for {}: {}", image_path, e);
                    }
                }
            } else {
                log::debug!("Image embedding model not available in registry, skipping image processing");
            }
        }
    }
}
```

**Location 2: After Memory Retrieval Activation (after line 181)**

Current code (lines 171-181):
```rust
// Generate stimulus from memory embedding and update cognitive state
if let Some(ref embedding) = domain_memory.embedding {
    let stimulus = embedding.data.clone();
    match self.cognitive_state.write().await.update_activation_from_stimulus(stimulus) {
        Ok(()) => {
            log::trace!("Updated cognitive activation from memory retrieval: {}", memory_id);
        }
        Err(e) => {
            log::warn!("Failed to update cognitive activation from memory retrieval: {}", e);
        }
    }
}
```

**Add after line 181:**
```rust
// Automatic image embedding processing on retrieval
if let Some(image_path_value) = domain_memory.metadata.custom.get("image_path") {
    if let Some(image_path) = image_path_value.as_ref().as_str() {
        use crate::capability::registry;
        use crate::capability::registry::ImageEmbeddingModel;
        use crate::capability::traits::ImageEmbeddingCapable;

        if let Some(vision_model) = registry::get::<ImageEmbeddingModel>("clip-vision") {
            match vision_model.embed_image(image_path).await {
                Ok(image_embedding) => {
                    match self.cognitive_state.write().await
                        .update_activation_from_stimulus(image_embedding) {
                        Ok(()) => {
                            log::trace!("Updated cognitive activation from retrieved image: {}", image_path);
                        }
                        Err(e) => {
                            log::warn!("Failed to update cognitive activation from retrieved image: {}", e);
                        }
                    }
                }
                Err(e) => {
                    log::warn!("Failed to generate image embedding on retrieval: {}", e);
                }
            }
        }
    }
}
```

## Architecture Flow

```
User adds memory with metadata.custom["image_path"] = "/path/to/image.jpg"
         │
         ▼
MemoryCoordinator.add_memory()
         │
         ├──> Generate text embedding (existing)
         │
         └──> [NEW] Check metadata for image_path
                    │
                    ├──> Query registry::get::<ImageEmbeddingModel>("clip-vision")
                    │
                    ├──> Model found? Generate image embedding
                    │
                    └──> Process through cognitive_state.update_activation_from_stimulus()

User retrieves memory with image_path
         │
         ▼
MemoryCoordinator.get_memory()
         │
         ├──> Process text embedding through cognitive state (existing)
         │
         └──> [NEW] Check metadata for image_path
                    │
                    ├──> Query registry for vision model
                    │
                    ├──> Generate image embedding
                    │
                    └──> Process through cognitive_state
```

## Implementation Checklist

### Phase 1: Add Imports (if needed)
- [ ] Verify registry imports at top of operations.rs
- [ ] Check if ImageEmbeddingModel import exists
- [ ] Check if ImageEmbeddingCapable trait import exists

### Phase 2: Add Image Processing to add_memory()
- [ ] Find line 125 (after text embedding generation)
- [ ] Add image metadata check
- [ ] Query registry::get::<ImageEmbeddingModel>("clip-vision")
- [ ] Generate image embedding if model available
- [ ] Process through cognitive_state.update_activation_from_stimulus()
- [ ] Add appropriate log::trace and log::warn messages

### Phase 3: Add Image Processing to get_memory()
- [ ] Find line 181 (after text embedding processing)
- [ ] Add image metadata check
- [ ] Query registry for vision model
- [ ] Generate image embedding
- [ ] Process through cognitive_state
- [ ] Add logging

### Phase 4: Verification
- [ ] Run `cargo check -p paraphym_candle --color=never`
- [ ] Verify no new fields added to MemoryCoordinator
- [ ] Verify no new constructors added
- [ ] Confirm image processing is fully automatic
- [ ] Test with metadata containing image_path

## Success Criteria

✅ Image embeddings processed automatically when metadata contains image_path
✅ NO new fields added to MemoryCoordinator
✅ NO with_*() constructors added
✅ Uses registry::get() to access models dynamically
✅ Graceful degradation if vision model not in registry
✅ Zero public API changes
✅ Proper error handling (no unwrap/expect)
✅ Follows same pattern as text embedding activation

## Non-Goals (Out of Scope)

❌ Adding tests or benchmarks
❌ Writing documentation
❌ Registering vision models (that's separate)
❌ Changing public API
❌ Adding new fields to MemoryCoordinator
❌ Creating MultimodalEmbeddingService instances

## Notes

- Vision model must be registered in registry separately (via runtime or static registration)
- If "clip-vision" key not found, feature silently skips (graceful degradation)
- Image path is read from `metadata.custom["image_path"]` string value
- Follows EXACT same pattern as activation processing from STUB_2
- Registry access is GLOBAL - no fields or initialization needed
- The metadata.custom field is already HashMap<Arc<str>, Arc<serde_json::Value>>
- This is the same metadata that users already provide when adding memories
