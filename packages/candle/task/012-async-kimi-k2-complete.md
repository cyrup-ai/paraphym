# Task 012: Complete async conversion for Kimi-K2 model

## Scope
Full async conversion for CandleKimiK2Model and LoadedKimiK2Model

## Current Issues
1. LoadedKimiK2Model stores `gguf_file_path: String` instead of cached model
2. Likely reloads 7.8GB+ model on every request
3. All tokenizer/model operations are sync

## Files
- `src/capability/text_to_text/kimi_k2.rs`

## Changes Needed
1. Change LoadedKimiK2Model to cache actual model:
   ```rust
   pub struct LoadedKimiK2Model {
       model: Arc<CandleQuantizedLlamaModel>,
       tokenizer: tokenizers::Tokenizer,
       device: Device,
       engine: Engine,
   }
   ```
2. Load model ONCE in LoadedKimiK2Model::load()
3. Use cached model in prompt() method
4. Wrap all tokenizer ops in spawn_blocking
5. Make forward() calls async (depends on Task 001)

## Dependencies
- Task 001 (async model.forward)
- Task 007 (as template)

## Estimated Effort
4 hours
