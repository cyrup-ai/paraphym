# Task: Convert Phi-4 Reasoning Model to Use Async huggingface_file

## Dependencies
- ⏳ **BLOCKED BY**: [ASYNC_FIX_HUGGINGFACE.md](./ASYNC_FIX_HUGGINGFACE.md) - CandleModel trait must be async first

## Location
`packages/candle/src/capability/text_to_text/phi4_reasoning.rs`

## Call Sites to Convert

### 1. `completion()` method - Line 118
```rust
let gguf_path = match self.huggingface_file(
    self.info().quantization_url.unwrap(),
    "phi-4-reasoning-Q4_K_M.gguf",
) {
    Ok(path) => path,
    Err(e) => { /* ... */ }
};
```

### 2. `completion()` method - Line 133
```rust
let tokenizer_path = match self.huggingface_file(self.info().registry_key, "tokenizer.json") {
    Ok(path) => path,
    Err(e) => { /* ... */ }
};
```

### 3. `CandlePhi4Generator::load()` - Line 349
```rust
let gguf_file_path = base
    .huggingface_file(
        base.info().quantization_url.unwrap(),
        "phi-4-reasoning-Q4_K_M.gguf",
    )
    .map_err(|e| { /* ... */ })?;
```

### 4. `CandlePhi4Generator::load()` - Line 359
```rust
let tokenizer_path = base
    .huggingface_file(base.info().registry_key, "tokenizer.json")
    .map_err(|e| { /* ... */ })?;
```

## Problem
All calls to `huggingface_file()` are synchronous, blocking the async runtime during:
- Large GGUF quantized model downloads (multi-GB files)
- Tokenizer configuration downloads

## Solution Steps

### Step 1: Update `completion()` method signature
```rust
// Before
fn completion(
    &self,
    prompt: CandlePrompt,
    params: &CandleCompletionParams,
) -> Pin<Box<dyn Stream<Item = CandleCompletionChunk> + Send>>

// After - add async
async fn completion(
    &self,
    prompt: CandlePrompt,
    params: &CandleCompletionParams,
) -> Pin<Box<dyn Stream<Item = CandleCompletionChunk> + Send>>
```

### Step 2: Add `.await` to completion() huggingface_file calls
```rust
// Line 118
let gguf_path = match self.huggingface_file(
    self.info().quantization_url.unwrap(),
    "phi-4-reasoning-Q4_K_M.gguf",
).await {
    // ...
};

// Line 133
let tokenizer_path = match self.huggingface_file(self.info().registry_key, "tokenizer.json").await {
    // ...
};
```

### Step 3: Update `CandlePhi4Generator::load()` signature
```rust
// Before
pub fn load(
    base: &CandlePhi4ReasoningModel,
) -> Result<Self, Box<dyn std::error::Error + Send + Sync>>

// After - add async
pub async fn load(
    base: &CandlePhi4ReasoningModel,
) -> Result<Self, Box<dyn std::error::Error + Send + Sync>>
```

### Step 4: Add `.await` to load() method calls
```rust
// Line 349
let gguf_file_path = base
    .huggingface_file(
        base.info().quantization_url.unwrap(),
        "phi-4-reasoning-Q4_K_M.gguf",
    ).await
    .map_err(|e| { /* ... */ })?;

// Line 359
let tokenizer_path = base
    .huggingface_file(base.info().registry_key, "tokenizer.json").await
    .map_err(|e| { /* ... */ })?;
```

### Step 5: Update all call sites of these methods
Search for any code calling:
- `CandlePhi4ReasoningModel.completion()` → Add `.await`
- `CandlePhi4Generator::load()` → Add `.await`

## Testing
1. Test with actual HuggingFace quantized model downloads
2. Verify completion streaming functions correctly
3. Test error handling paths
4. Confirm no runtime blocking during downloads

## Priority
🔴 **HIGH** - Large quantized model downloads will severely block runtime

## Status
⏳ BLOCKED - Waiting for CandleModel trait to be async
