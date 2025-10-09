# FLUX.1 Schnell

**Registry Key**: `black-forest-labs/FLUX.1-schnell`
**HuggingFace**: https://huggingface.co/black-forest-labs/FLUX.1-schnell

## Overview

FLUX.1 [schnell] is a text-to-image generative AI model developed by Black Forest Labs. It uses a rectified flow transformer architecture trained with latent adversarial diffusion distillation, enabling high-quality image generation in just 1-4 steps. "Schnell" means "fast" in German, reflecting its efficiency.

## Architecture

- **Type**: Rectified flow transformer (diffusion model)
- **Parameters**: 12B
- **Components**:
  - Main transformer: 23.8 GB
  - VAE (autoencoder): 335 MB
  - Text encoders: Dual text encoding
- **Training**: Latent adversarial diffusion distillation
- **Quantization**: None (FP16)

## Memory Requirements

**CRITICAL FOR POOL IMPLEMENTATION**

- **Inference (FP16)**: 24.1 GB (model weights + VAE)
- **Estimated Total Memory**: 27.7 GB (with inference overhead)
- **Memory Allocation**: **27700 MB**

## Performance

- Can generate high-quality images in 1-4 steps
- Competitive prompt following
- Matches performance of closed-source alternatives
- Significantly faster than traditional diffusion models
- Optimized for speed without sacrificing quality

## Usage Notes

- **Recommended Guidance Scale**: 0.0 (classifier-free guidance not needed)
- **Recommended Inference Steps**: 4
- Requires PyTorch and Diffusers library
- Supports CPU and GPU inference
- Can offload to CPU to save VRAM
- Apache 2.0 license (open source)
- Supports ComfyUI integration
- Available via multiple API endpoints
- Limitations:
  - May amplify societal biases
  - Prompt following depends on prompting style
  - Not designed for factual information
