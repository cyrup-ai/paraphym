# Task: Convert Pool Spawn Worker Functions to Accept Async Closures

## Locations
- `packages/candle/src/pool/capabilities/text_to_text.rs:162` - `spawn_text_to_text_worker`
- `packages/candle/src/pool/capabilities/text_embedding.rs` - `spawn_text_embedding_worker`
- `packages/candle/src/pool/capabilities/image_embedding.rs` - `spawn_image_embedding_worker`
- `packages/candle/src/pool/capabilities/text_to_image.rs` - `spawn_text_to_image_worker`
- `packages/candle/src/pool/capabilities/vision.rs` - `spawn_vision_worker`

## Problem
**BLOCKING ASYNC CALLS WITH block_in_place + block_on**

Current pattern in registry (line 313-318):
```rust
pool.spawn_text_to_text_worker(
    registry_key,
    move || {
        tokio::task::block_in_place(|| {  // ❌ BLOCKING
            tokio::runtime::Handle::current().block_on(async {  // ❌ BLOCKING
                LoadedPhi4ReasoningModel::load(&m_clone).await
                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
            })
        })
    },
    per_worker_mb,
    allocation_guard,
)
```

**Root cause**: Spawn functions expect SYNC closures:
```rust
pub fn spawn_text_to_text_worker<T, F>(
    &self,
    registry_key: &str,
    model_loader: F,  // ❌ SYNC closure
    per_worker_mb: usize,
    allocation_guard: AllocationGuard,
) -> Result<(), PoolError>
where
    T: TextToTextCapable + Send + 'static,
    F: FnOnce() -> Result<T, PoolError> + Send + 'static,
    // ^^^^^^^ SYNC - no async in signature
```

But model loaders are ALL async:
- `LoadedPhi4ReasoningModel::load().await`
- `LoadedKimiK2Model::load().await`  
- `LoadedBertModel::load().await`
- `LoadedStellaModel::load().await`
- etc.

## Solution

Change spawn functions to accept async closures:

```rust
pub fn spawn_text_to_text_worker<T, F, Fut>(
    &self,
    registry_key: &str,
    model_loader: F,  // ✅ ASYNC closure
    per_worker_mb: usize,
    allocation_guard: AllocationGuard,
) -> Result<(), PoolError>
where
    T: TextToTextCapable + Send + 'static,
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = Result<T, PoolError>> + Send + 'static,
    // ^^^ ASYNC - returns Future
```

Then registry calls become clean:
```rust
pool.spawn_text_to_text_worker(
    registry_key,
    move || async move {  // ✅ Clean async closure
        LoadedPhi4ReasoningModel::load(&m_clone)
            .await
            .map_err(|e| PoolError::SpawnFailed(e.to_string()))
    },
    per_worker_mb,
    allocation_guard,
)
```

## Files Requiring Changes

### 1. Pool Spawn Functions (5 files)
Update signatures to accept `async FnOnce`:
- `text_to_text.rs` - spawn_text_to_text_worker
- `text_embedding.rs` - spawn_text_embedding_worker  
- `image_embedding.rs` - spawn_image_embedding_worker
- `text_to_image.rs` - spawn_text_to_image_worker
- `vision.rs` - spawn_vision_worker

### 2. Registry Call Sites (registry.rs)
Remove `block_in_place` + `block_on` wrappers at:
- Line ~313 (Phi4)
- Line ~373 (Kimi-K2)
- Line ~471 (Qwen3Coder)
- Line ~558 (BERT embedding)
- Line ~652 (GTE-Qwen embedding)
- And all other model spawn locations

### 3. Inside Spawn Functions
Update model_loader invocation from:
```rust
let model = model_loader()?;  // Sync call
```

To:
```rust
let model = model_loader().await?;  // Async call
```

## Impact Analysis

**Every model spawn is affected** because ALL LoadedModel::load() methods are async:
- LoadedPhi4ReasoningModel::load()
- LoadedKimiK2Model::load()
- LoadedBertModel::load()
- LoadedStellaModel::load()
- LoadedGteQwenModel::load()
- LoadedJinaBertModel::load()
- LoadedNvEmbedModel::load()
- And any future models

## Steps
1. Update spawn_text_to_text_worker signature to accept async closure
2. Update spawn_text_embedding_worker signature
3. Update spawn_image_embedding_worker signature  
4. Update spawn_text_to_image_worker signature
5. Update spawn_vision_worker signature
6. Remove all `block_in_place` + `block_on` from registry.rs
7. Update model_loader calls inside spawn functions to use `.await`
8. Test worker spawning for each model type

## Testing
- Verify cold start (0→2 workers) works
- Verify concurrent requests spawn additional workers
- Verify no runtime blocking during model loading
- Check logs for any spawn failures

## Priority
🔥 **CRITICAL** - Using `block_in_place` defeats the purpose of async and can cause thread exhaustion

## Status
⏳ TODO
