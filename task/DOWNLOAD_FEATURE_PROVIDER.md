# Download Feature Provider Implementation - MIGRATION GUIDE

## Objective
Migrate ALL embedding providers to use the new `DownloadProviderFactory` abstraction layer, enabling seamless switching between HuggingFace Hub (hf-hub) and ProgressHub download backends through feature flags.

## Current Status

### ✅ Completed
- **Core abstraction layer**: `packages/candle/src/domain/model/download/`
  - `mod.rs` - Trait definition and exports
  - `factory.rs` - Provider selection logic (no .expect() issues found)
  - `hf_hub_provider.rs` - HfHub implementation
  - `progresshub_provider.rs` - ProgressHub wrapper
- **Migrated providers (ALL COMPLETE)**: 
  - `kimi_k2.rs` - Successfully uses DownloadProviderFactory
  - `bert_embedding.rs` - Migrated to use DownloadProviderFactory
  - `gte_qwen_embedding.rs` - Migrated to use DownloadProviderFactory  
  - `jina_bert_embedding.rs` - Migrated to use DownloadProviderFactory
  - `nvembed_embedding.rs` - Migrated to use DownloadProviderFactory
  - `stella_embedding.rs` - Migrated to use DownloadProviderFactory
  - `qwen3_coder.rs` - Migrated to use DownloadProviderFactory
- **Feature flags**: Configured in Cargo.toml with both backends enabled by default
- **Compilation tested**: Verified no download-related errors with different feature flag combinations

## Migration Pattern

### Before (Direct ProgressHub Usage)
```rust
#[cfg(feature = "download-progresshub")]
use progresshub::{ProgressHub, types::ZeroOneOrMany as ProgressHubZeroOneOrMany};

pub async fn with_config(config: SomeConfig) -> Result<Self> {
    #[cfg(feature = "download-progresshub")]
    {
        let results = ProgressHub::builder()
            .model("model-name")
            .with_cli_progress()
            .download()
            .await?;
            
        let model_cache_dir = if let Some(result) = results.into_iter().next() {
            match &result.models {
                ProgressHubZeroOneOrMany::One(model) => {
                    model.model_cache_path.display().to_string()
                }
                _ => return Err(...),
            }
        } else {
            return Err(...);
        };
        
        Self::with_config_and_path(config, model_cache_dir).await
    }
    
    #[cfg(not(feature = "download-progresshub"))]
    {
        Err(...)
    }
}
```

### After (Using DownloadProviderFactory)
```rust
use crate::domain::model::download::DownloadProviderFactory;

pub async fn with_config(config: SomeConfig) -> Result<Self> {
    // Use factory to get download provider (works with both backends)
    let downloader = DownloadProviderFactory::create_default().await?;
    
    // Download model files
    let result = downloader.download_model(
        "model-name",
        vec!["*.safetensors".to_string(), "tokenizer.json".to_string()],
        None, // or Some("Q4_K_M".to_string()) for quantization
    ).await?;
    
    // Use result.cache_dir for model path
    Self::with_config_and_path(
        config, 
        result.cache_dir.to_str()
            .ok_or_else(|| MemoryError::ModelError("Invalid cache directory".to_string()))?
            .to_string()
    ).await
}
```

## Detailed Migration Instructions

### 1. bert_embedding.rs
**Location**: Lines 90-128
**Model**: `sentence-transformers/all-MiniLM-L6-v2`
**Files needed**: `model.safetensors`, `tokenizer.json`, `config.json`

```rust
pub async fn with_config(config: CandleBertConfig) -> Result<Self> {
    use crate::domain::model::download::DownloadProviderFactory;
    
    let downloader = DownloadProviderFactory::create_default().await?;
    let result = downloader.download_model(
        "sentence-transformers/all-MiniLM-L6-v2",
        vec!["model.safetensors".to_string(), "tokenizer.json".to_string(), "config.json".to_string()],
        None,
    ).await?;
    
    Self::with_config_and_path(
        config,
        result.cache_dir.to_str()
            .ok_or_else(|| MemoryError::ModelError("Invalid cache directory".to_string()))?
            .to_string()
    ).await
}
```

### 2. gte_qwen_embedding.rs  
**Location**: Lines 90-125
**Model**: `Alibaba-NLP/gte-Qwen2-1.5B-instruct`
**Files needed**: Multiple safetensors (uses index.json), `tokenizer.json`, `config.json`

```rust
pub async fn with_config(config: CandleGteQwenConfig) -> Result<Self> {
    use crate::domain::model::download::DownloadProviderFactory;
    
    CandleGteQwenConfig::validate_dimension(config.dimension)?;
    
    let downloader = DownloadProviderFactory::create_default().await?;
    let result = downloader.download_model(
        "Alibaba-NLP/gte-Qwen2-1.5B-instruct",
        vec!["*.safetensors".to_string(), "*.json".to_string()],
        None,
    ).await?;
    
    Self::with_config_and_path(
        config,
        result.cache_dir.to_str()
            .ok_or_else(|| MemoryError::ModelError("Invalid cache directory".to_string()))?
            .to_string()
    ).await
}
```

### 3. jina_bert_embedding.rs
**Location**: Lines 52-82
**Model**: `jinaai/jina-embeddings-v2-base-en`  
**Files needed**: `model.safetensors`, `tokenizer.json`, `config.json`

```rust
pub async fn with_config(config: CandleJinaBertConfig) -> Result<Self> {
    use crate::domain::model::download::DownloadProviderFactory;
    
    let downloader = DownloadProviderFactory::create_default().await?;
    let result = downloader.download_model(
        "jinaai/jina-embeddings-v2-base-en",
        vec!["model.safetensors".to_string(), "tokenizer.json".to_string(), "config.json".to_string()],
        None,
    ).await?;
    
    Self::with_config_and_path(
        config,
        result.cache_dir.to_str()
            .ok_or_else(|| MemoryError::ModelError("Invalid cache directory".to_string()))?
            .to_string()
    ).await
}
```

### 4. nvembed_embedding.rs
**Model**: `nvidia/NV-Embed-v2`
**Files needed**: Model weights and tokenizer files

```rust
pub async fn with_config(config: CandleNvEmbedConfig) -> Result<Self> {
    use crate::domain::model::download::DownloadProviderFactory;
    
    let downloader = DownloadProviderFactory::create_default().await?;
    let result = downloader.download_model(
        "nvidia/NV-Embed-v2",
        vec!["*.safetensors".to_string(), "tokenizer.json".to_string(), "config.json".to_string()],
        None,
    ).await?;
    
    Self::with_config_and_path(
        config,
        result.cache_dir.to_str()
            .ok_or_else(|| MemoryError::ModelError("Invalid cache directory".to_string()))?
            .to_string()
    ).await
}
```

### 5. stella_embedding.rs
**Location**: Lines 90-130
**Models**: `dunzhang/stella_en_400M_v5` or `dunzhang/stella_en_1.5B_v5`
**Files needed**: Model files plus dimension-specific embedding heads

```rust
pub async fn with_config(config: StellaConfig) -> Result<Self> {
    use crate::domain::model::download::DownloadProviderFactory;
    
    StellaConfig::validate_dimension(config.dimension)?;
    
    let downloader = DownloadProviderFactory::create_default().await?;
    let result = downloader.download_model(
        config.repo_name(),
        vec![
            "*.safetensors".to_string(), 
            "tokenizer.json".to_string(), 
            "config.json".to_string(),
            format!("{}/*".to_string(), config.embed_head_dir()), // MRL projection heads
        ],
        None,
    ).await?;
    
    Self::with_config_and_path(
        config,
        result.cache_dir.to_str()
            .ok_or_else(|| MemoryError::ModelError("Invalid cache directory".to_string()))?
            .to_string()
    ).await
}
```

### 6. qwen3_coder.rs
**Location**: Lines 147-190
**Model**: `Qwen/Qwen2.5-Coder-32B-Instruct-GGUF`
**Files needed**: GGUF model file and tokenizer

```rust
pub async fn with_config_async(config: CandleQwen3CoderConfig) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
    use crate::domain::model::download::DownloadProviderFactory;
    
    let downloader = DownloadProviderFactory::create_default().await?;
    let result = downloader.download_model(
        "Qwen/Qwen2.5-Coder-32B-Instruct-GGUF",
        vec!["*.gguf".to_string(), "tokenizer.json".to_string()],
        Some("Q4_K_M".to_string()), // Default quantization for GGUF
    ).await?;
    
    // Find GGUF file from results
    let gguf_file = result.files.iter()
        .find(|f| f.extension().and_then(|s| s.to_str()) == Some("gguf"))
        .ok_or_else(|| Box::<dyn std::error::Error + Send + Sync>::from("GGUF file not found"))?;
    
    Self::with_config_sync_gguf(
        result.cache_dir.to_str()
            .ok_or_else(|| Box::<dyn std::error::Error + Send + Sync>::from("Invalid cache directory"))?
            .to_string(),
        gguf_file.to_str()
            .ok_or_else(|| Box::<dyn std::error::Error + Send + Sync>::from("Invalid GGUF file path"))?
            .to_string(),
        config,
    )
}
```

## Implementation Checklist

### Step 1: Remove Direct ProgressHub Dependencies
For each provider file:
- [ ] Remove `#[cfg(feature = "download-progresshub")]` imports
- [ ] Remove `use progresshub::{ProgressHub, types::ZeroOneOrMany as ProgressHubZeroOneOrMany};`
- [ ] Remove all references to `ProgressHubZeroOneOrMany`
- [ ] Remove conditional compilation blocks for progresshub feature

### Step 2: Add Factory Import
For each provider file:
- [ ] Add `use crate::domain::model::download::DownloadProviderFactory;`

### Step 3: Update with_config Methods
For each provider:
- [ ] Replace ProgressHub::builder() pattern with DownloadProviderFactory
- [ ] Use downloader.download_model() with appropriate parameters
- [ ] Extract cache_dir from result instead of complex model path extraction
- [ ] Remove feature-specific conditional compilation

### Step 4: Testing
- [ ] Compile with `--features download-hf-hub` only
- [ ] Compile with `--features download-progresshub` only  
- [ ] Compile with both features (default)
- [ ] Run tests for each provider

## Key Differences Between Backends

### ProgressHub
- Downloads to `~/.cache/huggingface/hub/`
- Provides CLI progress bar
- Supports quantization filtering for GGUF models
- Returns detailed file metadata

### HF-Hub
- Downloads to `~/.cache/huggingface/hub/` (same location)
- No built-in CLI progress (could be added)
- Requires explicit file patterns
- More flexible API for custom configurations

## Common Pitfalls to Avoid

1. **Don't forget error conversion**: Use `.map_err()` to convert errors to appropriate types
2. **Path handling**: Always check `to_str()` returns Some before unwrapping
3. **File patterns**: Be specific about which files to download (*.safetensors vs model.safetensors)
4. **Quantization**: Only applicable for GGUF models (Qwen3Coder, KimiK2)
5. **Multiple safetensors**: Some models use index.json with multiple weight files (GTE-Qwen)

## Success Criteria

✅ ALL providers use `DownloadProviderFactory`
✅ ZERO direct ProgressHub imports in provider files  
✅ Code compiles with `--features download-hf-hub` alone
✅ Code compiles with `--features download-progresshub` alone
✅ Code compiles with both features enabled (default)
✅ No .expect() or .unwrap() in implementation
✅ Proper error handling throughout

## Testing Commands

```bash
# Test with HF-Hub only
cargo check -p paraphym_candle --no-default-features --features download-hf-hub

# Test with ProgressHub only  
cargo check -p paraphym_candle --no-default-features --features download-progresshub

# Test with both (default)
cargo check -p paraphym_candle

# Run specific provider tests
cargo test -p paraphym_candle bert_embedding
cargo test -p paraphym_candle gte_qwen_embedding
# ... etc
```

## Notes

- The factory already returns Result properly (no .expect() issue found)
- Both backends download to the same cache location for compatibility
- The abstraction allows for future backend additions (e.g., local cache, S3, etc.)
- Consider adding progress callback support to the trait for UI integration