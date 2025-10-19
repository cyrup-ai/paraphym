# Task: Verify Qwen3Coder as Default Model

## Objective
Ensure the converted Qwen3Coder (now using quantized 1.7B) is the default model to prove the architecture works.

## Files to Check/Update

### 1. `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry/builder.rs`

Verify or update default model:

```rust
// Should already be using Qwen3Coder, or update to:
pub const DEFAULT_TEXT_TO_TEXT_MODEL: &str = "unsloth/qwen3-coder";
```

### 2. `/Volumes/samsung_t9/paraphym/packages/candle/examples/fluent_builder.rs`

Verify example uses Qwen3Coder by default:

```rust
// Check help text reflects correct model
println!("Model: qwen3-coder (default)");
```

**Note**: If Phi4 is currently default, temporarily change to Qwen3Coder for testing, then restore Phi4 after it's fixed in Task 007.

## Model Configuration

- **Registry Key**: `unsloth/qwen3-1.7b`
- **GGUF Repo**: `unsloth/Qwen3-1.7B-GGUF`
- **Model File**: `Qwen3-1.7B-Q4_K_M.gguf`
- **Tokenizer**: Qwen/Qwen3-1.7B (tokenizer.json)
- **Size**: ~1.1GB
- **EOS Token**: `<|im_end|>`
- **Chat Template**: `<|im_start|>user\n{prompt}<|im_end|>\n<|im_start|>assistant\n`

## Success Criteria
- fluent_builder runs with Qwen3 by default
- Model loads in < 2 seconds (matching Candle baseline)
- Generates at 80+ tokens/s
- All examples work correctly
