# Task 019: Complete async conversion for CLIP Vision

## Scope
Full async conversion for ClipVisionModel and ClipVisionEmbeddingModel

## Current Issues
1. Model forward pass is sync
2. Image preprocessing is sync
3. Embedding extraction is sync
4. Uses lazy loading pattern that may be sync

## Files
- `src/capability/image_embedding/clip_vision.rs`
- `src/capability/image_embedding/clip_vision_embedding.rs`

## Changes Needed
1. Make model.forward() async (via Task 001)
2. Wrap image preprocessing in spawn_blocking
3. Wrap embedding extraction in spawn_blocking
4. Make lazy loading async (huggingface_file already async)
5. Update ImageEmbeddingCapable trait implementation

## Model Architecture
- Supports ViT-Base-Patch32 (224×224, 512-dim)
- Supports ViT-Large-Patch14 (336×336, 768-dim)

## Dependencies
- Task 001 (async model.forward)

## Estimated Effort
4 hours
