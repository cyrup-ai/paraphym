# Task: Convert Kimi K2 Model to Use Async huggingface_file

## Dependencies
- ⏳ **BLOCKED BY**: [ASYNC_FIX_HUGGINGFACE.md](./ASYNC_FIX_HUGGINGFACE.md) - CandleModel trait must be async first

## Location
`packages/candle/src/capability/text_to_text/kimi_k2.rs`

## Call Sites to Convert

### 1. `completion()` method - Line 90
```rust
let gguf_file_path = match self.huggingface_file(self.info().registry_key, "*.gguf") {
    Ok(p) => p,
    Err(e) => { /* ... */ }
};
```

### 2. `completion()` method - Line 102
```rust
let tokenizer_path = match self.huggingface_file(self.info().registry_key, "tokenizer.json") {
    Ok(p) => p,
    Err(e) => { /* ... */ }
};
```

### 3. `CandleKimiK2Generator::load()` - Line 408
```rust
let gguf_file_path = base
    .huggingface_file(base.info().registry_key, "*.gguf")
    .map_err(|e| { /* ... */ })?;
```

### 4. `CandleKimiK2Generator::load()` - Line 415
```rust
let tokenizer_path = base
    .huggingface_file(base.info().registry_key, "tokenizer.json")
    .map_err(|e| { /* ... */ })?;
```

## Problem
All calls to `huggingface_file()` are synchronous, which will block the tokio runtime during:
- GGUF model file downloads (potentially GB-sized files)
- Tokenizer file downloads

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

### Step 2: Add `.await` to all `huggingface_file` calls
```rust
// Line 90
let gguf_file_path = match self.huggingface_file(self.info().registry_key, "*.gguf").await {
    // ...
};

// Line 102
let tokenizer_path = match self.huggingface_file(self.info().registry_key, "tokenizer.json").await {
    // ...
};
```

### Step 3: Update `CandleKimiK2Generator::load()` signature
```rust
// Before
pub fn load(
    base: &CandleKimiK2Model,
) -> Result<Self, Box<dyn std::error::Error + Send + Sync>>

// After - add async
pub async fn load(
    base: &CandleKimiK2Model,
) -> Result<Self, Box<dyn std::error::Error + Send + Sync>>
```

### Step 4: Add `.await` to load() method calls
```rust
// Line 408
let gguf_file_path = base
    .huggingface_file(base.info().registry_key, "*.gguf").await
    .map_err(|e| { /* ... */ })?;

// Line 415
let tokenizer_path = base
    .huggingface_file(base.info().registry_key, "tokenizer.json").await
    .map_err(|e| { /* ... */ })?;
```

### Step 5: Update all call sites of these methods
Search for any code calling:
- `CandleKimiK2Model.completion()` → Add `.await`
- `CandleKimiK2Generator::load()` → Add `.await`

## Testing
1. Test model initialization with actual HuggingFace downloads
2. Verify completion streaming works correctly
3. Ensure error handling is preserved
4. Verify no blocking in async runtime

## Priority
🔴 **HIGH** - Blocks async runtime during large file downloads

## Status
⏳ BLOCKED - Waiting for CandleModel trait to be async
