# Task 016: Complete async conversion for Jina-BERT

## Scope
Full async conversion for CandleJinaBertEmbeddingModel and LoadedJinaBertModel

## Current Issues
1. Model forward pass is sync
2. Tokenizer operations are sync
3. Embedding extraction is sync

## Files
- `src/capability/text_embedding/jina_bert.rs`

## Changes Needed
1. Make model.forward() async (via Task 001)
2. Wrap tokenizer operations in spawn_blocking
3. Wrap embedding extraction in spawn_blocking
4. Update TextEmbeddingCapable trait implementation

## Model Architecture
- Uses BertModel from candle-transformers
- Jina-specific BERT variant

## Dependencies
- Task 001 (async model.forward)

## Estimated Effort
3 hours
