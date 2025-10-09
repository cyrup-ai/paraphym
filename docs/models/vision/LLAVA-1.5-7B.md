# LLaVA 1.5 7B

**Registry Key**: `llava-hf/llava-1.5-7b-hf`
**HuggingFace**: https://huggingface.co/llava-hf/llava-1.5-7b-hf

## Overview

LLaVA (Large Language and Vision Assistant) 1.5 7B is an open-source multimodal chatbot trained by fine-tuning LLaMA/Vicuna on GPT-generated instruction data. It enables vision-language interactions, allowing the model to understand and respond to queries about images.

## Architecture

- **Type**: Vision-language multimodal model
- **Parameters**: 7.06B
- **Base Architecture**: Transformer-based auto-regressive language model
- **Vision Encoder**: CLIP-based vision encoder
- **Language Model**: LLaMA 2 / Vicuna
- **Quantization**: None (FP16 default, supports 4-bit via bitsandbytes)

## Memory Requirements

**CRITICAL FOR POOL IMPLEMENTATION**

- **Inference (FP16)**: 14.1 GB (model weights)
- **Estimated Total Memory**: 17.0 GB (with inference overhead)
- **Memory Allocation**: **17000 MB**

## Performance

- Trained in September 2023
- Part of LLaVA-v1.5 series
- Supports multi-image and multi-prompt generation
- Effective vision-language understanding
- Compatible with Flash Attention 2 for faster generation

## Usage Notes

- Requires transformers >= 4.35.3
- Recommended to use GPU with float16 precision
- Supports 4-bit quantization via bitsandbytes for reduced memory
- Uses specific chat template with `USER:` and `ASSISTANT:` roles
- Can process images via URL or local path
- Default generation limit: 200 new tokens
- Llama 2 Community License
- Supports both vision and text inputs simultaneously
- Flash Attention 2 compatible for improved speed
