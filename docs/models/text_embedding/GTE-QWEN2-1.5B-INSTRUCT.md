# GTE-Qwen2 1.5B Instruct

**Registry Key**: `Alibaba-NLP/gte-Qwen2-1.5B-instruct`
**HuggingFace**: https://huggingface.co/Alibaba-NLP/gte-Qwen2-1.5B-instruct

## Overview

GTE-Qwen2-1.5B-instruct is a multilingual text embedding model built on Qwen2-1.5B, designed for sentence similarity and semantic search tasks. It features bidirectional attention mechanisms and instruction tuning, trained on a multilingual text corpus to support cross-lingual applications.

## Architecture

- **Type**: Transformer-based multilingual embedding model (Qwen2-based)
- **Parameters**: 1.78B
- **Embedding Dimension**: 1,536
- **Max Input Tokens**: 32,000
- **Tensor Type**: FP32
- **Quantization**: None (FP32)

## Memory Requirements

**CRITICAL FOR POOL IMPLEMENTATION**

- **Inference (FP32)**: 6.62 GB (model weights)
- **Estimated Total Memory**: 7.6 GB (with inference overhead)
- **Memory Allocation**: **7600 MB**

## Performance

- **MTEB (English) Score**: 67.16
- **C-MTEB (Chinese) Score**: 67.65
- Multilingual capabilities across multiple languages
- Optimized for semantic search and text embedding tasks

## Usage Notes

- Compatible with Sentence Transformers and Transformers libraries
- Requires transformers>=4.39.2 and flash_attn>=2.5.6
- Supports custom prompts for task-specific optimization
- Bidirectional attention for improved context understanding
- Instruction tuning enables better semantic understanding
- Excellent for multilingual applications
