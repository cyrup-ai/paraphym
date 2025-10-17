# Task 020: Complete async conversion for LLaVA vision model

## Scope
Full async conversion for LLaVAModel and LoadedLLaVAModel

## Current Issues
1. Uses channels for thread communication (may be sync blocking)
2. Model forward pass is sync
3. Image processing is sync
4. Text generation is sync

## Files
- `src/capability/vision/llava.rs`

## Changes Needed
1. Review channel-based architecture for async compatibility
2. Make model.forward() async (via Task 001)
3. Wrap image processing in spawn_blocking
4. Make text generation fully async
5. Update VisionCapable trait implementation
6. Ensure thread spawning is async-compatible

## Model Architecture
- Multi-modal vision-language model
- Combines vision encoder + language model
- Uses channels for request/response

## Dependencies
- Task 001 (async model.forward)
- Task 008 (async generation loop, for text output)

## Estimated Effort
6 hours (highest complexity - multi-modal + threading)
