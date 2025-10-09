# BERT MiniLM L6 v2

**Registry Key**: `sentence-transformers/all-MiniLM-L6-v2`
**HuggingFace**: https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2

## Overview

Sentence-transformers model that maps sentences and paragraphs to a 384-dimensional dense vector space. Designed for tasks like clustering, semantic search, and sentence similarity. Trained on over 1 billion sentence pairs from multiple datasets.

## Architecture

- **Type**: Transformer-based sentence embedding (BERT-based)
- **Parameters**: 22.7M
- **Hidden Size**: 384
- **Layers**: 6 (MiniLM-L6)
- **Embedding Dimension**: 384
- **Max Sequence Length**: 256 word pieces
- **Quantization**: None (FP32)

## Memory Requirements

**CRITICAL FOR POOL IMPLEMENTATION**

- **Inference (FP32)**: 90.9 MB (model weights)
- **Estimated Total Memory**: 100 MB (with inference overhead)
- **Memory Allocation**: **100 MB**

## Performance

- Trained on 1B+ sentence pairs from multiple datasets
- Optimized for semantic similarity and information retrieval
- Efficient inference due to small size

## Usage Notes

- Input text longer than 256 word pieces is truncated
- Can be used with Sentence-Transformers or HuggingFace Transformers
- Supports sentence and short paragraph encoding
- Excellent for applications requiring fast embedding generation
- Very lightweight model suitable for resource-constrained environments
