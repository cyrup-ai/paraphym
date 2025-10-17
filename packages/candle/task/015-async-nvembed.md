# Task 015: Complete async conversion for NVEmbed v2

## Scope
Full async conversion for CandleNvEmbedEmbeddingModel and LoadedNvEmbedModel

## Current Issues
1. Model uses Mutex for interior mutability (likely sync operations inside)
2. Forward pass is sync
3. Tokenizer operations are sync

## Files
- `src/capability/text_embedding/nvembed.rs`

## Changes Needed
1. Replace Mutex with async-compatible synchronization if needed
2. Make model.forward() async (via Task 001)
3. Wrap tokenizer operations in spawn_blocking
4. Wrap embedding extraction in spawn_blocking
5. Update TextEmbeddingCapable trait implementation

## Model Architecture
- Uses NvEmbedModel (custom architecture)
- High-performance embeddings

## Dependencies
- Task 001 (async model.forward)

## Estimated Effort
4 hours (higher due to Mutex refactoring)
