# Task 014: Complete async conversion for BERT embedding

## Scope
Full async conversion for CandleBertEmbeddingModel and LoadedBertModel

## Current Issues
1. Model forward pass is sync
2. Tokenizer operations are sync
3. Embedding extraction is sync

## Files
- `src/capability/text_embedding/bert.rs`

## Changes Needed
1. Make model.forward() async (via Task 001)
2. Wrap tokenizer operations in spawn_blocking:
   - tokenizer.encode()
3. Wrap embedding extraction in spawn_blocking if CPU-intensive
4. Update TextEmbeddingCapable trait implementation to be fully async

## Model Architecture
- Uses BertModel from candle-transformers
- Loads from sentence-transformers/all-MiniLM-L6-v2
- Returns 384-dimensional embeddings

## Dependencies
- Task 001 (async model.forward)

## Estimated Effort
3 hours
