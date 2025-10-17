# Task 018: Complete async conversion for GTE-Qwen2

## Scope
Full async conversion for CandleGteQwenEmbeddingModel and LoadedGteQwenModel

## Current Issues
1. Model uses Mutex for interior mutability (KV cache management)
2. Forward pass is sync
3. Tokenizer operations are sync

## Files
- `src/capability/text_embedding/gte_qwen.rs`

## Changes Needed
1. Replace Mutex with async-compatible synchronization
2. Make model.forward() async (via Task 001)
3. Wrap tokenizer operations in spawn_blocking
4. Handle Qwen2 KV cache updates asynchronously
5. Update TextEmbeddingCapable trait implementation

## Model Architecture
- Uses Qwen2 model with KV cache
- Requires &mut Model for forward pass

## Dependencies
- Task 001 (async model.forward)

## Estimated Effort
5 hours (highest complexity due to KV cache + Mutex)
