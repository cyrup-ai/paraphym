# Task 017: Complete async conversion for Stella embedding

## Scope
Full async conversion for StellaEmbeddingModel and LoadedStellaModel

## Current Issues
1. Model uses Mutex for interior mutability (likely sync operations inside)
2. Forward pass is sync
3. Tokenizer operations are sync

## Files
- `src/capability/text_embedding/stella.rs`

## Changes Needed
1. Replace Mutex with async-compatible synchronization if needed
2. Make model.forward() async (via Task 001)
3. Wrap tokenizer operations in spawn_blocking
4. Wrap embedding extraction in spawn_blocking
5. Update TextEmbeddingCapable trait implementation

## Model Architecture
- Uses custom EmbeddingModel with lm_head projection
- Configurable output dimensions

## Dependencies
- Task 001 (async model.forward)

## Estimated Effort
4 hours (higher due to Mutex refactoring)
