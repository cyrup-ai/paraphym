# Task 013: Complete async conversion for Qwen3-Coder model

## Scope
Full async conversion for CandleQwen3CoderModel and LoadedQwen3CoderModel

## Current Issues
1. LoadedQwen3CoderModel stores `gguf_file_path: String` instead of cached model
2. Likely reloads model on every request
3. All tokenizer/model operations are sync

## Files
- `src/capability/text_to_text/qwen3_coder.rs`

## Changes Needed
1. Change LoadedQwen3CoderModel to cache actual model:
   ```rust
   pub struct LoadedQwen3CoderModel {
       model: Arc<CandleQuantizedLlamaModel>,  // Or appropriate model type
       tokenizer: tokenizers::Tokenizer,
       device: Device,
       engine: Engine,
   }
   ```
2. Load model ONCE in LoadedQwen3CoderModel::load()
3. Use cached model in prompt() method
4. Wrap all tokenizer ops in spawn_blocking
5. Make forward() calls async (depends on Task 001)

## Dependencies
- Task 001 (async model.forward)
- Task 007 (as template)

## Estimated Effort
4 hours
