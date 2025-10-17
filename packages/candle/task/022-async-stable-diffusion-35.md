# Task 022: Complete async conversion for Stable Diffusion 3.5

## Scope
Full async conversion for Stable Diffusion 3.5 Turbo

## Files
- `src/capability/text_to_image/stable_diffusion_35_turbo/mod.rs`
- `src/capability/text_to_image/stable_diffusion_35_turbo/clip_g_tokenizer.rs`
- `src/capability/text_to_image/stable_diffusion_35_turbo/clip_l_tokenizer.rs`
- `src/capability/text_to_image/stable_diffusion_35_turbo/t5_tokenizer.rs`
- `src/capability/text_to_image/stable_diffusion_35_turbo/t5_config.rs`

## Current Issues
1. Model forward passes are sync (multiple encoders)
2. CLIP-G tokenizer is sync
3. CLIP-L tokenizer is sync
4. T5-XXL tokenizer is sync
5. Diffusion sampling is sync
6. Image encode/decode is sync

## Changes Needed
1. Make all model.forward() calls async (via Task 001)
2. Wrap all tokenizer operations in spawn_blocking:
   - CLIP-G tokenizer
   - CLIP-L tokenizer
   - T5-XXL tokenizer
3. Wrap diffusion sampling in spawn_blocking
4. Wrap image operations in spawn_blocking
5. Make entire generation pipeline async
6. Update TextToImageCapable trait implementation

## Model Architecture
- Triple text encoder (CLIP-G + CLIP-L + T5-XXL)
- MMDiT backbone
- VAE decoder
- Multi-step diffusion

## Dependencies
- Task 001 (async model.forward)

## Estimated Effort
7 hours (most complex - 3 encoders + diffusion)
