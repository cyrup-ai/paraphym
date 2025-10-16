# Task: Convert GTE-Qwen Embedding Model to Use Async huggingface_file

## Dependencies
- ⏳ **BLOCKED BY**: [ASYNC_FIX_HUGGINGFACE.md](./ASYNC_FIX_HUGGINGFACE.md) - CandleModel trait must be async first

## Location
`packages/candle/src/capability/text_embedding/gte_qwen.rs`

## Call Sites to Convert

### 1. `embed()` method (TextEmbeddingCapable trait) - Lines 216, 217, 218

**Line 216:**
```rust
let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json")?;
```

**Line 217:**
```rust
let config_path = self.huggingface_file(self.info().registry_key, "config.json")?;
```

**Line 218:**
```rust
let index_path = self.huggingface_file(self.info().registry_key, "model.safetensors.index.json")?;
```

### 2. `batch_embed()` method (TextEmbeddingCapable trait) - Lines 343, 344, 345, 346

**Line 343:**
```rust
let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json")?;
```

**Line 344:**
```rust
let config_path = self.huggingface_file(self.info().registry_key, "config.json")?;
```

**Line 345:**
```rust
let index_path = self.huggingface_file(self.info().registry_key, "model.safetensors.index.json")?;
```

### 3. `LoadedGteQwenModel::load()` - Lines 495, 497, 499, 500

**Line 495:**
```rust
let tokenizer_path = base_model.huggingface_file(base_model.info().registry_key, "tokenizer.json")?;
```

**Line 497:**
```rust
let config_path = base_model.huggingface_file(base_model.info().registry_key, "config.json")?;
```

**Line 499:**
```rust
let index_path = base_model.huggingface_file(base_model.info().registry_key, "model.safetensors.index.json")?;
```

**Line 500 (continuation):**
Additional huggingface_file call for model weights after parsing index.

## Problem
All calls to `huggingface_file()` are synchronous, blocking the async runtime during:
- Tokenizer configuration downloads
- Model configuration downloads
- Safetensors index file downloads
- Individual model shard downloads (multi-file sharded models)

## Solution Steps

### Step 1: Update `embed()` method signature
```rust
// Before
fn embed(
    &self,
    text: &str,
    task: Option<String>,
    dimension: Option<u32>,
) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>>

// After - add async
async fn embed(
    &self,
    text: &str,
    task: Option<String>,
    dimension: Option<u32>,
) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>>
```

### Step 2: Add `.await` to embed() huggingface_file calls
```rust
// Line 216
let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json").await?;

// Line 217
let config_path = self.huggingface_file(self.info().registry_key, "config.json").await?;

// Line 218
let index_path = self.huggingface_file(self.info().registry_key, "model.safetensors.index.json").await?;
```

### Step 3: Update `batch_embed()` method signature
```rust
// Before
fn batch_embed(
    &self,
    texts: &[String],
    task: Option<String>,
    dimension: Option<u32>,
) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>>

// After - add async
async fn batch_embed(
    &self,
    texts: &[String],
    task: Option<String>,
    dimension: Option<u32>,
) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>>
```

### Step 4: Add `.await` to batch_embed() huggingface_file calls
```rust
// Line 343
let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json").await?;

// Line 344
let config_path = self.huggingface_file(self.info().registry_key, "config.json").await?;

// Line 345
let index_path = self.huggingface_file(self.info().registry_key, "model.safetensors.index.json").await?;
```

### Step 5: Update `LoadedGteQwenModel::load()` signature
```rust
// Before
pub fn load(
    base_model: &CandleGteQwenEmbeddingModel,
) -> Result<Self, Box<dyn std::error::Error + Send + Sync>>

// After - add async
pub async fn load(
    base_model: &CandleGteQwenEmbeddingModel,
) -> Result<Self, Box<dyn std::error::Error + Send + Sync>>
```

### Step 6: Add `.await` to load() huggingface_file calls
```rust
// Line 495
let tokenizer_path = base_model.huggingface_file(base_model.info().registry_key, "tokenizer.json").await?;

// Line 497
let config_path = base_model.huggingface_file(base_model.info().registry_key, "config.json").await?;

// Line 499
let index_path = base_model.huggingface_file(base_model.info().registry_key, "model.safetensors.index.json").await?;

// Any additional huggingface_file calls in the weight loading loop
```

### Step 7: Update TextEmbeddingCapable trait definition
The trait in `capability/traits.rs` needs its method signatures updated to async.

### Step 8: Update all call sites
Search for any code calling:
- `CandleGteQwenEmbeddingModel.embed()` → Add `.await`
- `CandleGteQwenEmbeddingModel.batch_embed()` → Add `.await`
- `LoadedGteQwenModel::load()` → Add `.await`

## Testing
1. Test embedding generation with actual HuggingFace downloads
2. Verify sharded model loading (multi-file safetensors)
3. Test batch embedding functionality
4. Verify tokenizer configuration loading
5. Ensure no blocking during downloads

## Priority
🔴 **HIGH** - Sharded model downloads involve multiple files blocking runtime

## Status
⏳ BLOCKED - Waiting for CandleModel trait to be async
