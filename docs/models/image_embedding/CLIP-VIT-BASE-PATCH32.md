# CLIP ViT Base Patch32

**Registry Key**: `openai/clip-vit-base-patch32`
**HuggingFace**: https://huggingface.co/openai/clip-vit-base-patch32

## Overview

CLIP (Contrastive Language-Image Pre-training) by OpenAI is a multimodal model trained to maximize the similarity of (image, text) pairs via a contrastive loss. Developed in January 2021 for zero-shot image classification research, CLIP connects vision and language understanding.

## Architecture

- **Type**: Vision-Language multimodal model
- **Vision Encoder**: Vision Transformer (ViT-B/32)
- **Text Encoder**: Masked self-attention Transformer
- **Image Size**: 224x224
- **Patch Size**: 32x32 pixels
- **Embedding Dimension**: 512
- **Quantization**: None (FP32/FP16)

## Memory Requirements

**CRITICAL FOR POOL IMPLEMENTATION**

- **Inference (FP32)**: 605 MB (model weights - both encoders)
- **Estimated Total Memory**: 700 MB (with inference overhead)
- **Memory Allocation**: **700 MB**

## Performance

- Evaluated on 30+ datasets including ImageNet, CIFAR, Food101
- Zero-shot classification capabilities
- Performance metrics:
  - Gender classification: >96% across races
  - Racial classification: ~93%
  - Age classification: ~63%
- Strong performance on diverse visual tasks

## Usage Notes

- Primarily intended for research purposes
- Not recommended for deployment without extensive testing
- Works with Hugging Face Transformers library
- Limited to English language text
- Supports zero-shot image classification
- Key limitations:
  - Struggles with fine-grained classification
  - Potential demographic biases in classification
  - Performance varies based on prompt/class design
- Includes both vision and text encoders
