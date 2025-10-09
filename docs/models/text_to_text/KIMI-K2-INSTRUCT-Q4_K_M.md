# Kimi K2 Instruct Q4_K_M

**Registry Key**: `unsloth/Kimi-K2-Instruct-GGUF`
**HuggingFace**: https://huggingface.co/unsloth/Kimi-K2-Instruct-GGUF

## Overview

Kimi K2 is a state-of-the-art mixture-of-experts (MoE) language model designed for agentic intelligence, tool use, and reasoning tasks. With 1 trillion total parameters and 32 billion activated parameters per token, it delivers exceptional performance across coding, math, and general reasoning tasks.

## Architecture

- **Type**: Mixture-of-Experts (MoE) decoder-only Transformer
- **Total Parameters**: 1.026T (1 trillion)
- **Active Parameters**: 32B per token
- **Layers**: 61 total layers (1 dense layer)
- **Experts**: 384 experts, 8 selected per token
- **Context Length**: 128K tokens
- **Vocabulary Size**: 160K
- **Quantization**: Q4_K_M (4-bit)

## Memory Requirements

**CRITICAL FOR POOL IMPLEMENTATION**

- **Inference (Q4_K_M)**: 621 GB (quantized model weights)
- **Estimated Total Memory**: 715 GB (with inference overhead)
- **Memory Allocation**: **715000 MB**

**Note**: This is an extremely large model requiring significant resources (128GB+ RAM recommended for practical use).

## Performance

- **LiveCodeBench**: 53.7% Pass@1 (top performer in coding)
- **MMLU**: 89.5%
- Strong tool use capabilities
- Excellent reasoning and agentic intelligence performance
- High scores on math and general tasks

## Usage Notes

- Recommended temperature: 0.6
- Requires significant memory (minimum 128GB RAM, preferably 256GB+ for Q4_K_M)
- Supports tool calling and chat completion
- OpenAI/Anthropic-compatible API available
- GGUF format allows flexible deployment
- Best suited for high-memory systems or distributed inference
- MoE architecture: only 32B params active per token despite 1T total
