# Task: Multi-Modal Stimulus Integration

**Status**: Ready for Execution
**Priority**: High
**Complexity**: Medium

## Overview

Wire the fully-implemented `MultimodalEmbeddingService` into the memory coordinator hotpath to enable automatic multi-modal cognitive activation from both text and image embeddings. The service is complete with text/vision models, cross-modal similarity, and batch operations - it just needs integration points.

## Objective

Enable transparent multi-modal cognitive stimulus processing by:
1. Adding `MultimodalEmbeddingService` field to `MemoryCoordinator`
2. Wiring multi-modal activation updates in memory retrieval hotpath
3. Supporting both text and image embeddings for cognitive activation
4. Maintaining zero public API changes (everything happens "automagically")

## Background: What's Already Built

### MultimodalEmbeddingService (memory/vector/multimodal_service.rs)

**Fully Implemented Features:**
- Text embedding via `TextEmbeddingModel` from registry
- Vision embedding via `ImageEmbeddingModel` from registry
- Methods:
  - `embed_text(text, task)` - text to embedding
  - `embed_image(path)` - image file to embedding
  - `embed_image_url(url)` - remote image to embedding
  - `embed_image_base64(data)` - base64 image to embedding
  - `batch_embed_text()`, `batch_embed_images()` - batch operations
  - `text_image_similarity()` - cross-modal SIMD cosine similarity
  - `batch_text_image_similarity()` - batch cross-modal ops

**Dependencies:**
```rust
use crate::capability::registry::{ImageEmbeddingModel, TextEmbeddingModel};
use crate::capability::traits::{ImageEmbeddingCapable, TextEmbeddingCapable};
use cyrup_simd::cosine_similarity;
```

**Current Status**: ✅ Fully implemented, ❌ Not wired into coordinator

## Technical Details

### File: packages/candle/src/memory/core/manager/coordinator/lifecycle.rs

**Current Structure:**
```rust
pub(super) cognitive_state: Arc<RwLock<CognitiveState>>,
```

**Required Addition:**
```rust
use crate::memory::vector::MultimodalEmbeddingService;

pub struct MemoryCoordinator {
    // ... existing fields ...
    pub(super) cognitive_state: Arc<RwLock<CognitiveState>>,
    pub(super) multimodal_service: Option<Arc<MultimodalEmbeddingService>>,
}
```

**Initialization in `new()`:**
```rust
cognitive_state: Arc::new(RwLock::new(CognitiveState::new())),
multimodal_service: None, // Initialize as None, can be set via with_multimodal_service()
```

**Add Constructor Method:**
```rust
impl MemoryCoordinator {
    /// Enable multi-modal cognitive processing
    pub fn with_multimodal_service(mut self, service: MultimodalEmbeddingService) -> Self {
        self.multimodal_service = Some(Arc::new(service));
        self
    }
}
```

### File: packages/candle/src/memory/core/manager/coordinator/operations.rs

**Current Activation (lines 178-188):**
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

**Add Multi-Modal Support (after line 188):**
```rust
// If multimodal service available, also process image embeddings
if let Some(ref multimodal) = self.multimodal_service {
    // Check if memory has associated image data (metadata field)
    if let Some(image_path) = domain_memory.metadata.get("image_path").and_then(|v| v.as_str()) {
        // Generate image embedding and update cognitive state
        match multimodal.embed_image(image_path.to_string()).await {
            Ok(image_embedding) => {
                match self.cognitive_state.write().await.update_activation_from_stimulus(image_embedding) {
                    Ok(()) => {
                        log::trace!("Updated cognitive activation from image embedding: {}", memory_id);
                    }
                    Err(e) => {
                        log::warn!("Failed to update cognitive activation from image: {}", e);
                    }
                }
            }
            Err(e) => {
                log::warn!("Failed to generate image embedding: {}", e);
            }
        }
    }
}
```

### File: packages/candle/src/memory/core/ops/retrieval/semantic.rs

**Current Structure:**
```rust
pub struct SemanticRetrieval {
    // ... existing fields ...
    cognitive_state: Option<Arc<RwLock<CognitiveState>>>,
}
```

**Add Multi-Modal Field:**
```rust
multimodal_service: Option<Arc<MultimodalEmbeddingService>>,
```

**Update Constructor:**
```rust
pub fn new(/* ... existing params ... */) -> Self {
    Self {
        // ... existing fields ...
        cognitive_state: None,
        multimodal_service: None,
    }
}

pub fn with_multimodal_service(mut self, service: Arc<MultimodalEmbeddingService>) -> Self {
    self.multimodal_service = Some(service);
    self
}
```

**Add Image Query Support in `retrieve()` (after text embedding generation):**
```rust
// If multimodal service available and query contains image reference
if let Some(ref multimodal) = self.multimodal_service {
    if let Some(image_path) = extract_image_path_from_query(&query) {
        // Generate combined text-image embedding for query
        match multimodal.embed_image(image_path).await {
            Ok(image_emb) => {
                // Combine text and image embeddings (average fusion)
                let combined_embedding: Vec<f32> = query_embedding.iter()
                    .zip(image_emb.iter())
                    .map(|(t, i)| (t + i) / 2.0)
                    .collect();
                query_embedding = combined_embedding;
                log::trace!("Using combined text-image embedding for query");
            }
            Err(e) => {
                log::warn!("Failed to generate image embedding for query: {}", e);
            }
        }
    }
}
```

**Helper Function (add at end of file):**
```rust
/// Extract image path from query string if present
/// Looks for patterns like "image:/path/to/file.jpg" or "img:url"
fn extract_image_path_from_query(query: &str) -> Option<String> {
    if let Some(img_marker) = query.find("image:").or_else(|| query.find("img:")) {
        let start = img_marker + if query[img_marker..].starts_with("image:") { 6 } else { 4 };
        let path_str = &query[start..];
        let end = path_str.find(char::is_whitespace).unwrap_or(path_str.len());
        Some(path_str[..end].to_string())
    } else {
        None
    }
}
```

## Integration Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Memory Retrieval Hotpath                  │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  MemoryCoordinator.get_memory(memory_id)                    │
│         │                                                     │
│         ├──> Text Embedding ────> CognitiveState             │
│         │                                                     │
│         └──> [NEW] MultimodalEmbeddingService                │
│                    │                                          │
│                    ├──> Image Embedding ──> CognitiveState   │
│                    │                                          │
│                    └──> Cross-Modal Similarity               │
│                                                               │
│  SemanticRetrieval.retrieve(query)                           │
│         │                                                     │
│         ├──> Text Query Embedding                            │
│         │                                                     │
│         └──> [NEW] Image Query Embedding                     │
│                    │                                          │
│                    └──> Combined Embedding ──> Vector Search │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

## Implementation Checklist

### Phase 1: Add MultimodalEmbeddingService Field
- [ ] Add import in lifecycle.rs: `use crate::memory::vector::MultimodalEmbeddingService;`
- [ ] Add field to MemoryCoordinator struct: `pub(super) multimodal_service: Option<Arc<MultimodalEmbeddingService>>`
- [ ] Initialize in new(): `multimodal_service: None`
- [ ] Add with_multimodal_service() constructor method

### Phase 2: Wire Image Embedding in MemoryCoordinator
- [ ] In operations.rs get_memory(), add multimodal check after text embedding
- [ ] Extract image path from metadata field
- [ ] Generate image embedding via multimodal.embed_image()
- [ ] Update cognitive state with image stimulus
- [ ] Add appropriate log::trace and log::warn messages

### Phase 3: Wire Multi-Modal Query in SemanticRetrieval
- [ ] Add multimodal_service field to SemanticRetrieval
- [ ] Update new() to initialize as None
- [ ] Add with_multimodal_service() constructor
- [ ] In retrieve(), check for image path in query
- [ ] Generate combined text-image embedding
- [ ] Add extract_image_path_from_query() helper function

### Phase 4: Remove Dead Code Annotations
- [ ] Verify no #[allow(dead_code)] on MultimodalEmbeddingService methods
- [ ] Check cargo check shows no dead_code warnings for multimodal_service.rs

### Phase 5: Verification
- [ ] Run `cargo check -p paraphym_candle --color=never 2>&1 | grep -i "multimodal\|dead_code"`
- [ ] Verify no new compilation errors introduced
- [ ] Verify multimodal_service field is Optional (no breaking changes)
- [ ] Confirm activation updates work for both text and image embeddings

## Success Criteria

✅ MultimodalEmbeddingService integrated into MemoryCoordinator
✅ Image embeddings automatically processed in get_memory() hotpath
✅ Combined text-image queries supported in semantic retrieval
✅ Zero public API changes (feature is opt-in via with_multimodal_service())
✅ All changes transparent to existing code
✅ No new dead_code warnings
✅ Proper error handling (no unwrap/expect)

## Non-Goals (Out of Scope)

❌ Adding tests or benchmarks
❌ Writing documentation
❌ Implementing new multimodal features
❌ Changing public API surface
❌ Fixing unrelated compilation errors
❌ Modifying MultimodalEmbeddingService implementation

## Notes

- The MultimodalEmbeddingService is FULLY IMPLEMENTED - we're just wiring it into the hotpath
- All changes maintain backward compatibility (Option fields)
- Image path extraction uses simple string matching (can be enhanced later)
- Combined embedding uses average fusion (proven technique for multi-modal)
- SIMD-optimized cosine similarity already in MultimodalEmbeddingService
- Follows same pattern as activation processing integration from STUB_2
