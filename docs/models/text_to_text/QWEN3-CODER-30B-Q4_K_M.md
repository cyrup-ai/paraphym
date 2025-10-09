# Qwen3-Coder 30B Q4_K_M

**Registry Key**: `unsloth/Qwen3-Coder-30B-A3B-Instruct-GGUF`
**HuggingFace**: https://huggingface.co/unsloth/Qwen3-Coder-30B-A3B-Instruct-GGUF

## Overview

Qwen3-Coder-30B-A3B-Instruct is a coding-focused language model with strong agentic coding capabilities and exceptional long-context support. Designed specifically for complex coding tasks, tool use, and function invocation with context windows up to 262K tokens.

## Architecture

- **Type**: Mixture-of-Experts (MoE) decoder-only Transformer
- **Total Parameters**: 30.5B
- **Active Parameters**: 3.3B per token
- **Layers**: 48
- **Attention Heads**: 32 (Query), 4 (Key-Value)
- **Experts**: 128 experts, 8 activated per token
- **Context Length**: 262,144 tokens (extendable to 1M with Yarn)
- **Quantization**: Q4_K_M (4-bit)

## Memory Requirements

**CRITICAL FOR POOL IMPLEMENTATION**

- **Inference (Q4_K_M)**: 18.6 GB (quantized model weights)
- **Estimated Total Memory**: 21.4 GB (with inference overhead)
- **Memory Allocation**: **21400 MB**

## Performance

- Significant performance on agentic coding tasks
- Supports long-context understanding (up to 256K tokens)
- Optimized for code generation, completion, and analysis
- Excellent tool calling and function invocation capabilities
- Efficient MoE architecture (only 3.3B active params despite 30.5B total)

## Usage Notes

- Recommended temperature: 0.7
- Recommended top_p: 0.8
- Suggested maximum output length: 65,536 tokens
- Requires transformers >=4.51.0
- Supports tool calling and function invocation
- GGUF format for flexible deployment
- MoE architecture provides efficiency with high capacity
- Ideal for complex coding tasks requiring long context
