# Task 021: Complete async conversion for FLUX.1-schnell

## Scope
Full async conversion for FluxSchnell text-to-image model

## Current Issues
1. Model forward pass is sync (multiple passes for diffusion)
2. T5-XXL encoder is sync
3. Image decoding/encoding is sync
4. 4-step generation loop is sync

## Files
- `src/capability/text_to_image/flux_schnell.rs`

## Changes Needed
1. Make all model.forward() calls async (via Task 001)
2. Wrap T5 encoding in spawn_blocking
3. Wrap diffusion sampling in spawn_blocking (CPU-intensive)
4. Wrap image encode/decode in spawn_blocking
5. Make entire generation pipeline async
6. Update TextToImageCapable trait implementation

## Model Architecture
- FLUX.1-schnell for 4-step generation
- T5-XXL text encoder
- Diffusion model with sampling

## Dependencies
- Task 001 (async model.forward)

## Estimated Effort
5 hours (multiple forward passes + diffusion)
