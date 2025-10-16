# Task: Convert Pool Spawn Worker Functions to Accept Async Closures

## Status
⏳ **TODO** | Priority: 🔥 **CRITICAL**

## Core Objective
Eliminate blocking async calls (`block_in_place` + `block_on`) by converting all pool spawn worker functions to accept async closures. This unifies the API and prevents thread exhaustion in the Tokio runtime.

## Problem Analysis

### Root Cause
Spawn worker functions currently expect **sync closures** (`FnOnce() -> Result<T, PoolError>`), but model loaders have **mixed patterns**:
- Some models have **async** load methods (e.g., `LoadedPhi4ReasoningModel::load().await`)
- Some models have **sync** load methods (e.g., `LoadedKimiK2Model::load()`)
- Some models are pre-loaded and just cloned (e.g., image_embedding, text_to_image)

This type mismatch forces async loads to use blocking wrappers:
```rust
move || {
    tokio::task::block_in_place(|| {  // ❌ DEFEATS ASYNC PURPOSE
        tokio::runtime::Handle::current().block_on(async {
            LoadedPhi4ReasoningModel::load(&m_clone).await
                .map_err(|e| PoolError::SpawnFailed(e.to_string()))
        })
    })
}
```

### Why This is Critical
Using `block_in_place` defeats the purpose of async and can cause:
- Thread pool exhaustion under load
- Increased latency during model initialization
- Resource contention in concurrent worker spawning
- Violation of async runtime best practices

## Solution Design

### Unified Async Closure Signature
Convert all spawn functions to accept async closures with consistent signature:

```rust
pub fn spawn_X_worker<T, F, Fut>(
    &self,
    registry_key: &str,
    model_loader: F,  // ✅ Now accepts async closure
    per_worker_mb: usize,
    allocation_guard: AllocationGuard,
) -> Result<(), PoolError>
where
    T: XCapable + Send + 'static,
    F: FnOnce() -> Fut + Send + 'static,  // ✅ Returns Future
    Fut: std::future::Future<Output = Result<T, PoolError>> + Send + 'static,
```

### Three Call Patterns in Registry

**Pattern 1: Async Load Methods**
```rust
// BEFORE (with blocking)
move || {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            LoadedPhi4ReasoningModel::load(&m_clone).await
                .map_err(|e| PoolError::SpawnFailed(e.to_string()))
        })
    })
}

// AFTER (clean async)
move || async move {
    LoadedPhi4ReasoningModel::load(&m_clone)
        .await
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}
```

**Pattern 2: Sync Load Methods**
```rust
// BEFORE (sync closure)
move || {
    LoadedKimiK2Model::load(&m_clone)
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}

// AFTER (wrapped in async)
move || async move {
    LoadedKimiK2Model::load(&m_clone)
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}
```

**Pattern 3: Pre-loaded Models (Clone Pattern)**
```rust
// BEFORE (sync closure)
let model = (*m_clone).clone();
move || Ok(model)

// AFTER (wrapped in async)
let model = (*m_clone).clone();
move || async move { Ok(model) }
```

## File-by-File Implementation Guide

### 1. Text-to-Text Worker
**File**: [`packages/candle/src/pool/capabilities/text_to_text.rs`](../packages/candle/src/pool/capabilities/text_to_text.rs)

**Status**: ✅ Signature already converted (lines 162-171), ❌ Missing `.await` at line 226

**Changes Required**:
```rust
// Line 226 - Add .await to model_loader call
let model = match model_loader() {  // ❌ CURRENT
let model = match model_loader().await {  // ✅ FIXED
```

**Registry Call Sites** (3 models):
- Line 247: `LoadedKimiK2Model` (sync) - wrap in async block
- Line 279: `LoadedQwen3CoderModel` (sync) - wrap in async block  
- Line 313: `LoadedPhi4ReasoningModel` (async) - remove block_in_place/block_on

### 2. Text Embedding Worker
**File**: [`packages/candle/src/pool/capabilities/text_embedding.rs`](../packages/candle/src/pool/capabilities/text_embedding.rs)

**Status**: ❌ Needs signature change at line ~189 AND `.await` at model_loader call

**Signature Change** (line ~189):
```rust
// BEFORE
pub fn spawn_text_embedding_worker<T, F>(
    &self,
    registry_key: &str,
    model_loader: F,
    per_worker_mb: usize,
    allocation_guard: AllocationGuard,
) -> Result<(), PoolError>
where
    T: TextEmbeddingCapable + Send + 'static,
    F: FnOnce() -> Result<T, PoolError> + Send + 'static,

// AFTER
pub fn spawn_text_embedding_worker<T, F, Fut>(
    &self,
    registry_key: &str,
    model_loader: F,
    per_worker_mb: usize,
    allocation_guard: AllocationGuard,
) -> Result<(), PoolError>
where
    T: TextEmbeddingCapable + Send + 'static,
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<T, PoolError>> + Send + 'static,
```

**Model Loader Call** (line ~235):
```rust
let model = match model_loader() {  // ❌ CURRENT
let model = match model_loader().await {  // ✅ FIXED
```

**Registry Call Sites** (9 instances):
- Line 370: `LoadedGteQwenModel` (async) - remove block_in_place/block_on
- Line 406: `LoadedJinaBertModel` (sync) - wrap in async block
- Line 437: `LoadedNvEmbedModel` (sync) - wrap in async block
- Line 468: `LoadedStellaModel` (async) - remove block_in_place/block_on
- Line 503: `LoadedBertModel` (sync) - wrap in async block
- Line 555: `LoadedGteQwenModel` (async) - remove block_in_place/block_on
- Line 619: `LoadedJinaBertModel` (sync) - wrap in async block
- Line 652: `LoadedStellaModel` (async) - remove block_in_place/block_on
- Line 683: `LoadedBertModel` (sync) - wrap in async block

### 3. Image Embedding Worker
**File**: [`packages/candle/src/pool/capabilities/image_embedding.rs`](../packages/candle/src/pool/capabilities/image_embedding.rs)

**Status**: ❌ Needs signature change at line ~239 AND `.await` at model_loader call

**Signature Change** (line ~239):
```rust
// Add Fut generic parameter and Future bound (same pattern as text_embedding)
pub fn spawn_image_embedding_worker<T, F, Fut>(
    ...
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<T, PoolError>> + Send + 'static,
```

**Model Loader Call** (line ~280):
```rust
let model = match model_loader().await {  // Add .await
```

**Registry Call Sites** (4 instances - all use pre-loaded clone pattern):
- Lines 749, 801, 853, 905: Change `move || Ok(model)` to `move || async move { Ok(model) }`

### 4. Text-to-Image Worker
**File**: [`packages/candle/src/pool/capabilities/text_to_image.rs`](../packages/candle/src/pool/capabilities/text_to_image.rs)

**Status**: ❌ Needs signature change at line ~137 AND `.await` at model_loader call

**Signature Change** (line ~137):
```rust
pub fn spawn_text_to_image_worker<T, F, Fut>(
    ...
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<T, PoolError>> + Send + 'static,
```

**Model Loader Call** (line ~177):
```rust
let model = match model_loader().await {  // Add .await
```

**Registry Call Sites** (2 instances - both use pre-loaded clone pattern):
- Lines 1028, 1057: Change `move || Ok(m_inner)` to `move || async move { Ok(m_inner) }`

### 5. Vision Worker
**File**: [`packages/candle/src/pool/capabilities/vision.rs`](../packages/candle/src/pool/capabilities/vision.rs)

**Status**: ❌ Needs signature change at line ~175 AND `.await` at model_loader call

**Signature Change** (line ~175):
```rust
pub fn spawn_vision_worker<T, F, Fut>(
    ...
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<T, PoolError>> + Send + 'static,
```

**Model Loader Call** (line ~215):
```rust
let model = match model_loader().await {  // Add .await
```

**Registry Call Sites** (2 instances):
- Lines 947, 984: `LoadedLLaVAModel` (sync) - wrap in async block

## Registry.rs Complete Change List

**File**: [`packages/candle/src/capability/registry.rs`](../packages/candle/src/capability/registry.rs)

### Text-to-Text Models (3 changes)
```rust
// Line 249-252: LoadedKimiK2Model (SYNC)
move || async move {
    LoadedKimiK2Model::load(&m_clone)
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}

// Line 281-284: LoadedQwen3CoderModel (SYNC)
move || async move {
    LoadedQwen3CoderModel::load(&m_clone)
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}

// Line 313-320: LoadedPhi4ReasoningModel (ASYNC)
move || async move {
    LoadedPhi4ReasoningModel::load(&m_clone)
        .await
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}
```

### Text Embedding Models (9 changes)
```rust
// Line 372-379: LoadedGteQwenModel (ASYNC)
move || async move {
    LoadedGteQwenModel::load(&m_clone)
        .await
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}

// Line 408-411: LoadedJinaBertModel (SYNC)
move || async move {
    LoadedJinaBertModel::load(&m_clone)
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}

// Line 439-442: LoadedNvEmbedModel (SYNC)
move || async move {
    LoadedNvEmbedModel::load(&m_clone)
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}

// Line 470-477: LoadedStellaModel (ASYNC)
move || async move {
    LoadedStellaModel::load(&m_clone)
        .await
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}

// Line 505-508: LoadedBertModel (SYNC)
move || async move {
    LoadedBertModel::load(&m_clone)
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}

// Line 557-564: LoadedGteQwenModel (ASYNC) - duplicate registration
move || async move {
    LoadedGteQwenModel::load(&m_clone)
        .await
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}

// Line 621-624: LoadedJinaBertModel (SYNC) - duplicate registration
move || async move {
    LoadedJinaBertModel::load(&m_clone)
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}

// Line 654-661: LoadedStellaModel (ASYNC) - duplicate registration
move || async move {
    LoadedStellaModel::load(&m_clone)
        .await
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}

// Line 685-688: LoadedBertModel (SYNC) - duplicate registration
move || async move {
    LoadedBertModel::load(&m_clone)
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}
```

### Image Embedding Models (4 changes)
```rust
// Lines 749-753: ClipVisionEmbedding (pre-loaded)
let model = (*m_clone).clone();
pool.spawn_image_embedding_worker(
    registry_key,
    move || async move { Ok(model) },  // Add async wrapper
    per_worker_mb,
    allocation_guard,
)

// Repeat for lines 801, 853, 905 (same pattern)
```

### Vision Models (2 changes)
```rust
// Line 949-952: LoadedLLaVAModel (SYNC)
move || async move {
    LoadedLLaVAModel::load(&m_clone)
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}

// Line 986-989: LoadedLLaVAModel (SYNC) - duplicate registration
move || async move {
    LoadedLLaVAModel::load(&m_clone)
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}
```

### Text-to-Image Models (2 changes)
```rust
// Lines 1028-1033: StableDiffusion (pre-loaded)
let m_inner = (**m).clone();
pool.spawn_text_to_image_worker(
    registry_key,
    move || async move { Ok(m_inner) },  // Add async wrapper
    per_worker_mb,
    allocation_guard,
)

// Line 1057-1062: Flux (same pattern)
```

## Implementation Strategy

### Phase 1: Update Spawn Function Signatures (4 files)
1. text_embedding.rs - Add `<Fut>` generic and `Future` bound
2. image_embedding.rs - Add `<Fut>` generic and `Future` bound
3. text_to_image.rs - Add `<Fut>` generic and `Future` bound
4. vision.rs - Add `<Fut>` generic and `Future` bound

### Phase 2: Add `.await` to Model Loader Calls (5 files)
1. text_to_text.rs - Line 226: `model_loader().await`
2. text_embedding.rs - Line ~235: `model_loader().await`
3. image_embedding.rs - Line ~280: `model_loader().await`
4. text_to_image.rs - Line ~177: `model_loader().await`
5. vision.rs - Line ~215: `model_loader().await`

### Phase 3: Update Registry Call Sites
1. Remove all `tokio::task::block_in_place` wrappers (5 instances)
2. Remove all `tokio::runtime::Handle::current().block_on` wrappers (5 instances)
3. Wrap sync load calls in `async move` blocks (remaining instances)
4. Add `async move` wrapper to pre-loaded model clones

## Model Load Method Reference

### Async Load Methods (require `.await`)
- `LoadedPhi4ReasoningModel::load()` - [phi4_reasoning.rs:344](../packages/candle/src/capability/text_to_text/phi4_reasoning.rs#L344)
- `LoadedGteQwenModel::load()` - Uses async huggingface_file calls
- `LoadedStellaModel::load()` - Uses async huggingface_file calls

### Sync Load Methods (no `.await` needed)
- `LoadedKimiK2Model::load()` - [kimi_k2.rs:403](../packages/candle/src/capability/text_to_text/kimi_k2.rs#L403)
- `LoadedQwen3CoderModel::load()`
- `LoadedJinaBertModel::load()`
- `LoadedNvEmbedModel::load()`
- `LoadedBertModel::load()`
- `LoadedLLaVAModel::load()`

### Pre-loaded Models (clone pattern)
- ClipVisionEmbedding models (4 instances)
- StableDiffusion/Flux models (2 instances)

## Code Pattern Examples

### Complete Before/After: Text-to-Text Spawn
```rust
// BEFORE: text_to_text.rs line 226
let model = match model_loader() {
    Ok(m) => {
        log::info!("TextToText worker {} ready", worker_id);
        state_clone.store(
            WorkerState::Ready as u32,
            std::sync::atomic::Ordering::Release,
        );
        m
    }
    Err(e) => {
        log::error!("TextToText worker {} failed: {}", worker_id, e);
        state_clone.store(
            WorkerState::Failed as u32,
            std::sync::atomic::Ordering::Release,
        );
        text_to_text_pool().remove_memory(per_worker_mb_clone);
        return;
    }
};

// AFTER: Add .await
let model = match model_loader().await {  // ✅ Only change needed
    Ok(m) => {
        // ... rest unchanged
    }
    Err(e) => {
        // ... rest unchanged
    }
};
```

### Complete Before/After: Registry Async Load
```rust
// BEFORE: registry.rs line 313-320
pool.spawn_text_to_text_worker(
    registry_key,
    move || {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                LoadedPhi4ReasoningModel::load(&m_clone).await
                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
            })
        })
    },
    per_worker_mb,
    allocation_guard,
)

// AFTER: Clean async closure
pool.spawn_text_to_text_worker(
    registry_key,
    move || async move {
        LoadedPhi4ReasoningModel::load(&m_clone)
            .await
            .map_err(|e| PoolError::SpawnFailed(e.to_string()))
    },
    per_worker_mb,
    allocation_guard,
)
```

## Definition of Done

1. ✅ All 5 spawn worker functions accept async closures (`F: FnOnce() -> Fut`)
2. ✅ All 5 spawn worker functions call `model_loader().await`
3. ✅ Zero instances of `tokio::task::block_in_place` in registry.rs
4. ✅ Zero instances of `tokio::runtime::Handle::current().block_on` in registry.rs
5. ✅ All registry spawn calls use async closure syntax
6. ✅ Code compiles without errors
7. ✅ Worker spawning functions correctly for all model types

## Verification Commands

```bash
# Verify no blocking patterns remain
rg "block_in_place" packages/candle/src/capability/registry.rs
rg "block_on" packages/candle/src/capability/registry.rs

# Expected: No matches found

# Verify async signatures
rg "pub fn spawn_.*_worker.*Fut" packages/candle/src/pool/capabilities/

# Expected: 5 matches (one per capability file)

# Verify .await on model_loader
rg "model_loader\(\)\.await" packages/candle/src/pool/capabilities/

# Expected: 5 matches (one per capability file)
```

## Impact Summary

**Total Changes**:
- 5 function signature changes
- 5 `.await` additions in spawn functions
- ~23 closure pattern updates in registry.rs
- 0 changes to model loading logic
- 0 changes to worker loop implementations

**Risk Level**: Low - Changes are mechanical type system updates with no logic changes

**Testing Focus**: Verify worker spawning under various load conditions (cold start, concurrent requests, model loading failures)
