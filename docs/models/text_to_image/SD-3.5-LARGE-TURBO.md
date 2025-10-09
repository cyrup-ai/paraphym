# Stable Diffusion 3.5 Large Turbo

**Registry Key**: `stabilityai/stable-diffusion-3.5-large-turbo`
**HuggingFace**: https://huggingface.co/stabilityai/stable-diffusion-3.5-large-turbo

## Overview

Stable Diffusion 3.5 Large Turbo is a multimodal Diffusion Transformer (MMDiT) text-to-image generative model developed by Stability AI. It uses Adversarial Diffusion Distillation (ADD) for improved performance, enabling high-quality image generation in just 4 steps.

## Architecture

- **Type**: Multimodal Diffusion Transformer (MMDiT)
- **Parameters**: 8B
- **Main Model**: 16.5 GB (sd3.5_large_turbo.safetensors)
- **Text Encoders**:
  - OpenCLIP-ViT/G (CLIP-G)
  - CLIP-ViT/L (CLIP-L)
  - T5-xxl
- **VAE**: 16-channel VAE decoder
- **Features**: QK normalization
- **Training**: Adversarial Diffusion Distillation
- **Quantization**: None (FP16, supports 4-bit quantization)

## Memory Requirements

**CRITICAL FOR POOL IMPLEMENTATION**

- **Inference (FP16)**: 22.3 GB (model + text encoders + VAE)
- **Estimated Total Memory**: 25.6 GB (with inference overhead)
- **Memory Allocation**: **25600 MB**

## Performance

- Improved image quality over previous SD versions
- Better typography and text rendering
- Enhanced complex prompt understanding
- Resource-efficient with 4-step inference
- High-quality generation at fast speeds

## Usage Notes

- **Recommended Inference Steps**: 4
- **Recommended Guidance Scale**: 0.0 (classifier-free guidance not needed)
- Requires latest diffusers library
- Supports CUDA with bfloat16 precision
- Supports 4-bit quantization for low VRAM GPUs
- Can be used with ComfyUI or programmatically
- Licensing: Community license for organizations <$1M annual revenue
- Enterprise licensing available for larger organizations
- Multiple text encoders for better prompt understanding
- Optimized for fast, high-quality generation
