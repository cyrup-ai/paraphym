# Stella EN 1.5B v5

**Registry Key**: `dunzhang/stella_en_1.5B_v5`
**HuggingFace**: https://huggingface.co/dunzhang/stella_en_1.5B_v5

## Overview

Stella EN 1.5B v5 is a sentence embedding model optimized for semantic similarity and retrieval tasks. Based on "Alibaba-NLP/gte-large-en-v1.5" and "Alibaba-NLP/gte-Qwen2-1.5B-instruct", this model supports multiple embedding dimensions ranging from 512 to 8192 dimensions, with 1024 dimensions recommended for optimal performance.

## Architecture

- **Type**: Transformer-based sentence embedding model
- **Parameters**: 1.54B
- **Recommended Sequence Length**: 512 tokens
- **Embedding Dimensions**: 512-8192 (recommended: 1024)
- **Quantization**: None (FP32)

## Memory Requirements

**CRITICAL FOR POOL IMPLEMENTATION**

- **Inference (FP32)**: 6.17 GB (model weights)
- **Estimated Total Memory**: 7.2 GB (with inference overhead)
- **Memory Allocation**: **7200 MB**

## Performance

High accuracy on MTEB benchmarks:
- Amazon Counterfactual Classification: 92.866%
- Amazon Polarity Classification: 97.165%
- Optimized for both search/retrieval and semantic similarity tasks

## Usage Notes

- Two primary query prompts:
  - `s2p_query`: For search/retrieval tasks
  - `s2s_query`: For semantic similarity tasks
- Supports SentenceTransformers, Transformers, and Infinity inference server
- Recommended to use 1024-dimensional embeddings for optimal performance
- Requires CUDA/GPU for efficient inference
