# Task: Convert Stella Embedding Model to Use Async huggingface_file

## Dependencies
- ⏳ **BLOCKED BY**: [ASYNC_FIX_HUGGINGFACE.md](./ASYNC_FIX_HUGGINGFACE.md) - CandleModel trait must be async first

## Location
`packages/candle/src/capability/text_embedding/stella.rs`

## Call Sites to Convert

### 1. `embed()` method (TextEmbeddingCapable trait) - Lines 198, 201, 207

**Line 198:**
```rust
let base_weights = self.huggingface_file(self.info().registry_key, "model.safetensors")?;
```

**Line 201:**
```rust
let projection_head = self.huggingface_file(
    self.info().registry_key,
    &format!("2_Dense_{}/model.safetensors", dimension),
)?;
```

**Line 207:**
```rust
let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json")?;
```

### 2. `batch_embed()` method (TextEmbeddingCapable trait) - Lines 374, 375, 379

**Line 374:**
```rust
let base_weights = self.huggingface_file(self.info().registry_key, "model.safetensors")?;
```

**Line 375:**
```rust
let projection_head = self.huggingface_file(
    self.info().registry_key,
    &format!("2_Dense_{}/model.safetensors", dimension),
)?;
```

**Line 379:**
```rust
let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json")?;
```

### 3. `LoadedStellaModel::load()` - Lines 581, 582, 586

**Line 581:**
```rust
let base_weights = base_model.huggingface_file(base_model.info().registry_key, "model.safetensors")?;
```

**Line 582:**
```rust
let projection_head = base_model.huggingface_file(
    base_model.info().registry_key,
    &format!("2_Dense_{}/model.safetensors", dimension),
)?;
```

**Line 586:**
```rust
let tokenizer_path = base_model.huggingface_file(base_model.info().registry_key, "tokenizer.json")?;
```

## Problem
All calls to `huggingface_file()` are synchronous, blocking the async runtime during:
- Base model weights download (large safetensors files)
- MRL projection head downloads (dimension-specific)
- Tokenizer configuration downloads

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
// Line 198
let base_weights = self.huggingface_file(self.info().registry_key, "model.safetensors").await?;

// Line 201
let projection_head = self.huggingface_file(
    self.info().registry_key,
    &format!("2_Dense_{}/model.safetensors", dimension),
).await?;

// Line 207
let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json").await?;
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
// Line 374
let base_weights = self.huggingface_file(self.info().registry_key, "model.safetensors").await?;

// Line 375
let projection_head = self.huggingface_file(
    self.info().registry_key,
    &format!("2_Dense_{}/model.safetensors", dimension),
).await?;

// Line 379
let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json").await?;
```

### Step 5: Update `LoadedStellaModel::load()` signature
```rust
// Before
pub fn load(
    base_model: &StellaEmbeddingModel,
) -> Result<Self, Box<dyn std::error::Error + Send + Sync>>

// After - add async
pub async fn load(
    base_model: &StellaEmbeddingModel,
) -> Result<Self, Box<dyn std::error::Error + Send + Sync>>
```

### Step 6: Add `.await` to load() huggingface_file calls
```rust
// Line 581
let base_weights = base_model.huggingface_file(base_model.info().registry_key, "model.safetensors").await?;

// Line 582
let projection_head = base_model.huggingface_file(
    base_model.info().registry_key,
    &format!("2_Dense_{}/model.safetensors", dimension),
).await?;

// Line 586
let tokenizer_path = base_model.huggingface_file(base_model.info().registry_key, "tokenizer.json").await?;
```

### Step 7: Update TextEmbeddingCapable trait definition
The trait in `capability/traits.rs` needs its method signatures updated to async.

### Step 8: Update all call sites
Search for any code calling:
- `StellaEmbeddingModel.embed()` → Add `.await`
- `StellaEmbeddingModel.batch_embed()` → Add `.await`
- `LoadedStellaModel::load()` → Add `.await`

## Testing
1. Test embedding generation with actual HuggingFace downloads
2. Test all MRL dimensions (256, 768, 1024, 2048, 4096, 6144, 8192)
3. Verify batch embedding works correctly
4. Test both Large (1.5B) and Small (400M) variants
5. Ensure no blocking during model/tokenizer downloads

## Priority
🔴 **HIGH** - Multiple large file downloads will severely impact runtime

## Status
⏳ BLOCKED - Waiting for CandleModel trait to be async
