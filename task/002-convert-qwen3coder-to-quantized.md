# Task: Convert Qwen3Coder to QuantizedQwen3

## Objective
Convert the existing Qwen3Coder model to use Candle's `quantized_qwen3` implementation. This is a **conversion**, not adding a new model - we keep the same struct names and registry integration.

## Files to Modify

### `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/text_to_text/qwen3_coder.rs`

**Key Changes:**
1. Replace model import:
   ```rust
   // BEFORE:
   use candle_transformers::models::qwen3::ModelWeights;
   
   // AFTER:
   use candle_transformers::models::quantized_qwen3::ModelWeights;
   use candle::quantized::gguf_file;
   ```

2. Update model loading:
   ```rust
   // BEFORE: Complex full-precision loading
   
   // AFTER: Simple GGUF loading (from Candle example)
   let model_path = // get from HF hub: unsloth/Qwen3-1.7B-GGUF
   let mut file = std::fs::File::open(&model_path)?;
   let content = gguf_file::Content::read(&mut file)?;
   let model = ModelWeights::from_gguf(content, &mut file, &device)?;
   ```

3. Remove spawn_blocking:
   ```rust
   // BEFORE:
   let model = tokio::task::spawn_blocking(move || {
       // complex loading
   }).await?;
   
   // AFTER:
   let model = ModelWeights::from_gguf(content, &mut file, &device)?;
   ```

4. Extract EOS token from GGUF:
   ```rust
   // Read from GGUF metadata
   let eos_token_id = content.metadata
       .get("tokenizer.ggml.eos_token_id")
       .and_then(|v| v.to_u32());
   
   // Or use vocab lookup for "<|im_end|>"
   let eos_token = *tokenizer.get_vocab(true).get("<|im_end|>").unwrap();
   ```

5. Simplify forward pass:
   ```rust
   // BEFORE: Wrapped in spawn_blocking
   
   // AFTER: Direct call (from Candle example)
   let input = Tensor::new(tokens, &device)?.unsqueeze(0)?;
   let logits = model.forward(&input, position)?;
   let logits = logits.squeeze(0)?;
   ```

## Implementation Pattern (from Candle)

Reference: `/Volumes/samsung_t9/paraphym/tmp/candle/candle-examples/examples/quantized-qwen3/main.rs`

```rust
// 1. Load GGUF file
let model_path = args.model()?;
let mut file = std::fs::File::open(&model_path)?;
let content = gguf_file::Content::read(&mut file)?;

// 2. Create model - SIMPLE!
let model = Qwen3::from_gguf(content, &mut file, &device)?;

// 3. Load tokenizer
let tokenizer = Tokenizer::from_file(tokenizer_path)?;

// 4. Generation loop - DIRECT!
for index in 0..to_sample {
    let input = Tensor::new(&[next_token], &device)?.unsqueeze(0)?;
    let logits = model.forward(&input, tokens.len() + index)?;
    let logits = logits.squeeze(0)?;
    next_token = logits_processor.sample(&logits)?;
    
    if let Some(t) = token_output_stream.next_token(next_token)? {
        // Send token
    }
}
```

## Model Configuration

- **Registry Key**: Keep existing or update to `unsloth/qwen3-1.7b`
- **HF Repo**: `unsloth/Qwen3-1.7B-GGUF`
- **Model File**: `Qwen3-1.7B-Q4_K_M.gguf`
- **Tokenizer Repo**: `Qwen/Qwen3-1.7B`
- **Tokenizer File**: `tokenizer.json`
- **Size**: ~1.1GB (310 tensors)
- **EOS Token**: `<|im_end|>` (lookup from vocab or GGUF metadata)
- **Chat Template**: `<|im_start|>user\n{prompt}<|im_end|>\n<|im_start|>assistant\n`

## What NOT to Change

1. **Struct names**: Keep `CandleQwen3CoderModel` and `LoadedQwen3CoderModel`
2. **Enum variant**: Keep `TextToTextModel::Qwen3Coder`
3. **Registry integration**: Minimal changes
4. **Public interface**: Keep same methods and signatures

## Testing

After conversion, test with:
```bash
cd /Volumes/samsung_t9/paraphym
cargo run --example fluent_builder --release -- \
  --query "What is 2+2?" \
  --max-tokens 50
```

## Success Criteria
- ✅ Model loads in < 1 second (baseline: 0.36s)
- ✅ Generates at 70+ tokens/s (baseline: 93.87 tokens/s)
- ✅ No hangs or deadlocks
- ✅ Minimal code changes
- ✅ Existing tests still pass
- ✅ No spawn_blocking in hot path
