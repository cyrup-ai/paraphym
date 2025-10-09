# NV-Embed v2

**Registry Key**: `nvidia/NV-Embed-v2`
**HuggingFace**: https://huggingface.co/nvidia/NV-Embed-v2

## Overview

A generalist embedding model that ranks No. 1 on the Massive Text Embedding Benchmark with a score of 72.31 across 56 text embedding tasks. Built on Mistral-7B-v0.1 with latent-attention pooling for high-quality embeddings.

## Architecture

- **Type**: Transformer-based generalist embedding model (Mistral-7B-based)
- **Parameters**: 7.85B
- **Base Model**: Mistral-7B-v0.1
- **Embedding Dimension**: 4096
- **Pooling Type**: Latent-Attention
- **Tensor Type**: FP16
- **Quantization**: None (FP16)

## Memory Requirements

**CRITICAL FOR POOL IMPLEMENTATION**

- **Inference (FP16)**: 15.7 GB (model weights)
- **Estimated Total Memory**: 18.1 GB (with inference overhead)
- **Memory Allocation**: **18100 MB**

## Performance

- **MTEB Leaderboard Score**: 72.31 (Rank #1)
- **Retrieval Sub-category Score**: 62.65
- **Amazon Polarity Classification**: 97.742% accuracy
- Highest performance across 56 diverse text embedding tasks
- State-of-the-art generalist embedding model

## Usage Notes

- Supports both HuggingFace Transformers and Sentence-Transformers
- Requires torch 2.2.0 and transformers 4.42.4
- Supports multi-GPU encoding for large-scale applications
- Non-commercial license (CC-BY-NC-4.0)
- Designed for various embedding tasks:
  - Retrieval
  - Classification
  - Semantic similarity
  - Clustering
  - Information retrieval
