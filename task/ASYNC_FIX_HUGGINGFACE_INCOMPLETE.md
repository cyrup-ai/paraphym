# INCOMPLETE: CandleModel::huggingface_file Cannot Be Converted to Async (Yet)

## Task Status: ❌ BLOCKED

The original task in `ASYNC_FIX_HUGGINGFACE.md` is NOT completable as written because converting `huggingface_file()` to async requires converting **53+ caller methods** to async, cascading throughout the codebase.

## What I Discovered

### The Trait Method (Easy Part)
```rust
// IN: domain/model/traits.rs
async fn huggingface_file(&self, repo_key: &str, filename: &str) 
    -> Result<PathBuf, Box<dyn Error + Send + Sync>>
{
    use hf_hub::api::tokio::Api;
    let api = Api::new()?;  // Already sync
    let path = repo.get(filename).await?;  // Only this needs await
    Ok(path)
}
```

### The Problem (Hard Part)
**53+ call sites** across the codebase, many in non-async contexts:

1. **Sync constructors**: `LoadedKimiK2Model::load()` (lines 481-527 in kimi_k2.rs)
2. **Sync trait methods**: Various model initialization methods
3. **Nested calls**: Methods calling methods calling huggingface_file

### Files That Would Need Changes
```
Modified files after partial attempt:
- src/domain/model/traits.rs (trait method)
- src/capability/text_to_image/stable_diffusion_35_turbo/mod.rs (7 call sites)
- src/capability/text_to_text/kimi_k2.rs (4 call sites + make load() async)
- src/capability/image_embedding/clip_vision.rs (3 methods)
- src/capability/registry.rs (pool spawning logic)
- ... and 48+ more errors after these changes
```

## Why This Is Blocked

The task says:
> 5. Find all call sites and update to await the call

But **many call sites are in sync contexts** that would also need to become async, which would cascade to THEIR callers, creating a massive refactor.

### Example Cascade:
```
huggingface_file (async)
  ├─> LoadedKimiK2Model::load() (needs to be async)
  │     └─> Pool::spawn_worker() callback (needs to be async)  
  │           └─> ensure_workers_spawned() (needs to be async)
  │                 └─> TextToTextCapable::prompt() (already returns Stream, complex refactor)
  └─> StableDiffusion35Turbo::generate() (inside async block, can be fixed)
```

## Two Possible Solutions

### Option A: Complete Async Cascade (Massive)
1. Make `huggingface_file()` async
2. Make ALL 53+ callers async
3. Propagate async through call chains
4. Update pool worker spawning to handle async initialization
5. Test entire codebase

**Estimated effort**: 2-3 days of careful refactoring

### Option B: Keep It Sync (Pragmatic)
The current code works because:
1. HuggingFace file downloads happen during **initialization**, not hot path
2. Files are cached after first download
3. Models are loaded once per worker, not per request
4. The blocking is acceptable in initialization phase

**If we must make it async**: Use `tokio::task::spawn_blocking()` to run blocking code in async context:

```rust
fn huggingface_file(&self, repo_key: &str, filename: &str) 
    -> Result<PathBuf, Box<dyn Error + Send + Sync>>
{
    use hf_hub::api::sync::Api;  // Keep sync API
    
    let api = Api::new()?;
    let repo = api.model(repo_key.to_string());
    let path = repo.get(filename)?;
    Ok(path)
}

// Callers wrap it:
tokio::task::spawn_blocking(move || {
    model.huggingface_file(key, file)
}).await??
```

## Recommendation

**DO NOT** attempt Option A without dedicated time for the full cascade.

**EITHER**:
1. Accept that initialization code can be blocking (current state)
2. Use `spawn_blocking` wrapper at call sites that need async
3. Make a separate task for the full async cascade refactor

## What Was NOT in Original Task

The original task said:
> 5. Find all call sites and update to await the call

But it did NOT mention:
- Making 50+ caller methods async
- Refactoring pool worker initialization
- Cascading async through deep call chains
- Updating trait designs for async initialization

This is a **multi-day refactor**, not a simple trait method conversion.

## Current Status

- ✅ Identified the blocking I/O (hf_hub::api::sync::Api)
- ✅ Found correct async API (hf_hub::api::tokio::Api)
- ❌ Cannot complete without massive refactor
- ✅ Reverted all changes (code compiles)

## Next Actions

User must decide:
1. Accept blocking initialization code as-is
2. Create new comprehensive task for full async cascade
3. Use spawn_blocking as intermediate solution
