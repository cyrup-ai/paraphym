# Jina Embeddings v2 Base EN

**Registry Key**: `jinaai/jina-embeddings-v2-base-en`
**HuggingFace**: https://huggingface.co/jinaai/jina-embeddings-v2-base-en

## Overview

English monolingual embedding model based on BERT architecture with ALiBi (Attention with Linear Biases) variant. Supports 8192 sequence length and is optimized for long document processing. Trained on over 400 million sentence pairs for semantic similarity and retrieval tasks.

## Architecture

- **Type**: Transformer-based embedding model (JinaBERT with ALiBi)
- **Parameters**: 137M
- **Max Sequence Length**: 8192 tokens
- **Embedding Dimension**: 768 (standard BERT base)
- **Tensor Type**: FP16
- **Quantization**: None (FP16)

## Memory Requirements

**CRITICAL FOR POOL IMPLEMENTATION**

- **Inference (FP16)**: 275 MB (model weights)
- **Estimated Total Memory**: 400 MB (with inference overhead)
- **Memory Allocation**: **400 MB**

## Performance

- Amazon Counterfactual Classification: 74.73% accuracy
- Amazon Polarity Classification: 88.54% accuracy
- ArguAna Retrieval: 35.49% map@10
- Optimized for long document processing (up to 8192 tokens)

## Usage Notes

- Apply mean pooling when integrating with applications
- Supports symmetric bidirectional ALiBi for long sequences
- Compatible with transformers and sentence-transformers libraries
- Recommended for single GPU inference
- Ideal use cases:
  - RAG (Retrieval Augmented Generation)
  - Semantic similarity
  - Text reranking
  - Document retrieval
  - Long document processing
