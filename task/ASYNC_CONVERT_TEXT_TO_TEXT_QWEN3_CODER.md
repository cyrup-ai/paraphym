# Task: Convert Qwen3 Coder to Use CandleModel::huggingface_file Trait Method

## Location
`packages/candle/src/capability/text_to_text/qwen3_coder.rs`

## Dependencies
- ✅ **UNBLOCKED**: Base trait is now async (ASYNC_FIX_HUGGINGFACE.md complete)

## Problem
**ARCHITECTURAL VIOLATION**: Qwen3 Coder bypasses the `CandleModel::huggingface_file()` trait method and uses `hf_hub::api::tokio::Api` directly. This violates the abstraction and creates inconsistency across the codebase.

### Current Implementation (WRONG)
```rust
use hf_hub::api::tokio::Api;

let api = Api::new()?;
let repo = api.repo(Repo::model(model_id.clone()));
let gguf_path = repo.get(&gguf_filename).await.map_err(|e| {
    Box::<dyn std::error::Error + Send + Sync>::from(format!(
        "Failed to download GGUF file: {}",
        e
    ))
})?;
let _tokenizer_path = repo.get("tokenizer.json").await.map_err(|e| {
    Box::<dyn std::error::Error + Send + Sync>::from(format!(
        "Failed to download tokenizer: {}",
        e
    ))
})?;
```

## Solution
Convert to use the trait method like ALL other models:

```rust
// Remove direct hf_hub imports
// use hf_hub::api::tokio::Api; // ❌ DELETE

// Use trait method instead
let gguf_path = self.huggingface_file(
    &model_id,  // or self.info().registry_key
    &gguf_filename,
).await.map_err(|e| {
    Box::<dyn std::error::Error + Send + Sync>::from(format!(
        "Failed to download GGUF file: {}",
        e
    ))
})?;

let tokenizer_path = self.huggingface_file(
    &model_id,  // or self.info().registry_key
    "tokenizer.json",
).await.map_err(|e| {
    Box::<dyn std::error::Error + Send + Sync>::from(format!(
        "Failed to download tokenizer: {}",
        e
    ))
})?;
```

## Why This Matters
1. **Consistency**: ALL models must use the same abstraction
2. **Maintainability**: Changes to download logic happen in one place (trait method)
3. **Testability**: Can mock `huggingface_file()` for testing
4. **Future-proofing**: Cache strategies, retry logic, etc. can be added to trait method

## Steps
1. Remove direct `use hf_hub::api::tokio::Api` import
2. Replace `Api::new()` and `repo.get()` calls with `self.huggingface_file()`
3. Ensure the model implements `CandleModel` trait properly
4. Update any references to `repo` object
5. Test compilation
6. Verify Qwen3 Coder functionality

## Call Sites to Convert
- GGUF file download (line ~104)
- Tokenizer download (line ~112)

## Priority
🔴 **HIGH** - Architectural violation, breaks abstraction layer

## Status
⏳ TODO
