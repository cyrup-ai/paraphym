# Task: Convert CandleModel::huggingface_file to Async

## Location
`packages/candle/src/domain/model/traits.rs` lines 92-107

## Problem
```rust
fn huggingface_file(
    &self,
    repo_key: &str,
    filename: &str,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>>
{
    use hf_hub::api::sync::Api;  // ❌ BLOCKING API
    
    let api = Api::new()?;
    let repo = api.model(repo_key.to_string());
    let path = repo.get(filename)?;  // ❌ BLOCKING NETWORK I/O
    
    Ok(path)
}
```

**Issue**: Downloads files from HuggingFace using blocking sync API - this will block the entire tokio runtime!

## Solution
Convert to async using tokio API:

```rust
async fn huggingface_file(
    &self,
    repo_key: &str,
    filename: &str,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>>
{
    use hf_hub::api::tokio::Api;  // ✅ ASYNC API
    
    let api = Api::new().await?;
    let repo = api.model(repo_key.to_string());
    let path = repo.get(filename).await?;  // ✅ ASYNC NETWORK I/O
    
    Ok(path)
}
```

## Steps
1. Change method signature to `async fn`
2. Replace `hf_hub::api::sync::Api` with `hf_hub::api::tokio::Api`
3. Add `.await` to `Api::new()`
4. Add `.await` to `repo.get(filename)`
5. Find all call sites and update to await the call
6. Test with actual HuggingFace downloads

## Priority
🔴 **CRITICAL** - This blocks the async runtime during network I/O

## Status
⏳ TODO
