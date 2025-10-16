# Task: Convert Pool Spawn Worker Functions and Model Load Methods to Async

## Status
⏳ **TODO** | Priority: 🔥 **CRITICAL**

## Core Objective
1. Convert ALL model `load()` methods from sync to async
2. Convert all pool spawn worker functions to accept async closures
3. Eliminate blocking async calls (`block_in_place` + `block_on`) throughout the registry

This unifies the API and prevents thread exhaustion in the Tokio runtime.

## Problem Analysis

### Root Cause
**Dual problem requiring coordinated fix:**

1. **Model load methods are inconsistent**: Some are async, some are sync (but call async methods incorrectly)
2. **Spawn functions expect sync closures**: Forces use of `block_in_place` + `block_on` wrappers

### Current Broken Pattern
Many "sync" load methods actually call `huggingface_file()` which is **async** but don't await it properly:

```rust
// ❌ BROKEN: LoadedKimiK2Model::load() - defined as sync but calls async method
pub fn load(base: &CandleKimiK2Model) -> Result<Self, ...> {
    let gguf_file_path = base
        .huggingface_file(base.info().registry_key, "*.gguf")  // ❌ Missing .await!
        .map_err(|e| ...)?;
    // ...
}
```

The `huggingface_file` trait method ([traits.rs:92](../packages/candle/src/domain/model/traits.rs#L92)) is defined as:
```rust
async fn huggingface_file(&self, repo_key: &str, filename: &str) -> Result<PathBuf, ...>
```

This creates a compile error or undefined behavior when called without `.await`.

### Registry Workaround Pattern
To handle async loads with sync spawn signatures, registry uses blocking wrappers:

```rust
pool.spawn_text_to_text_worker(
    registry_key,
    move || {
        tokio::task::block_in_place(|| {  // ❌ DEFEATS ASYNC PURPOSE
            tokio::runtime::Handle::current().block_on(async {
                LoadedPhi4ReasoningModel::load(&m_clone).await
                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
            })
        })
    },
    per_worker_mb,
    allocation_guard,
)
```

### Why This is Critical
- Thread pool exhaustion under load
- Increased latency during model initialization
- Resource contention in concurrent worker spawning
- Violation of async runtime best practices
- **Incorrect async usage** (calling async methods without .await)

## Solution Design

### Two-Part Solution

**Part 1: Make ALL Model Load Methods Async**
Convert every `pub fn load()` to `pub async fn load()` and add `.await` to async calls:

```rust
// BEFORE: Sync signature, broken async call
pub fn load(base: &CandleKimiK2Model) -> Result<Self, ...> {
    let gguf_file_path = base
        .huggingface_file(base.info().registry_key, "*.gguf")  // ❌ Missing .await
        .map_err(|e| ...)?;
}

// AFTER: Async signature, proper await
pub async fn load(base: &CandleKimiK2Model) -> Result<Self, ...> {
    let gguf_file_path = base
        .huggingface_file(base.info().registry_key, "*.gguf")
        .await  // ✅ Properly awaited
        .map_err(|e| ...)?;
}
```

**Part 2: Make ALL Spawn Functions Accept Async Closures**

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
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<T, PoolError>> + Send + 'static,
```

### Unified Registry Pattern
After both changes, ALL registry calls use the same clean async pattern:

```rust
// ✅ UNIFIED: All models use same pattern
pool.spawn_text_to_text_worker(
    registry_key,
    move || async move {
        LoadedXYZModel::load(&m_clone)
            .await  // ✅ All models now async
            .map_err(|e| PoolError::SpawnFailed(e.to_string()))
    },
    per_worker_mb,
    allocation_guard,
)
```

## Part 1: Convert Model Load Methods to Async

### Models Requiring Async Conversion

#### Text-to-Text Models

**1. LoadedKimiK2Model**
- **File**: [`packages/candle/src/capability/text_to_text/kimi_k2.rs`](../packages/candle/src/capability/text_to_text/kimi_k2.rs)
- **Location**: Line ~403
- **Changes**:
  ```rust
  // Line 403: Change signature
  pub async fn load(  // Add async
      base: &CandleKimiK2Model,
  ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
      
      // Line 407: Add .await
      let gguf_file_path = base
          .huggingface_file(base.info().registry_key, "*.gguf")
          .await  // ✅ Add this
          .map_err(|e| ...)?;
      
      // Line 414: Add .await
      let tokenizer_path = base
          .huggingface_file(base.info().registry_key, "tokenizer.json")
          .await  // ✅ Add this
          .map_err(|e| ...)?;
      
      // Rest of method unchanged
  }
  ```

**2. LoadedQwen3CoderModel**
- **File**: [`packages/candle/src/capability/text_to_text/qwen3_coder.rs`](../packages/candle/src/capability/text_to_text/qwen3_coder.rs)
- **Pattern**: Same as Kimi K2 - add `async` to signature, add `.await` to `huggingface_file` calls

**3. LoadedPhi4ReasoningModel**
- **File**: [`packages/candle/src/capability/text_to_text/phi4_reasoning.rs`](../packages/candle/src/capability/text_to_text/phi4_reasoning.rs)
- **Status**: ✅ Already async (line 344) - no changes needed

#### Text Embedding Models

**4. LoadedBertModel**
- **File**: [`packages/candle/src/capability/text_embedding/bert.rs`](../packages/candle/src/capability/text_embedding/bert.rs)
- **Pattern**: Add `async` to `load()` signature, add `.await` to `huggingface_file` calls

**5. LoadedGteQwenModel**
- **File**: [`packages/candle/src/capability/text_embedding/gte_qwen.rs`](../packages/candle/src/capability/text_embedding/gte_qwen.rs)
- **Status**: Verify if already async or needs conversion

**6. LoadedJinaBertModel**
- **File**: [`packages/candle/src/capability/text_embedding/jina_bert.rs`](../packages/candle/src/capability/text_embedding/jina_bert.rs)
- **Pattern**: Add `async` to `load()` signature, add `.await` to `huggingface_file` calls

**7. LoadedNvEmbedModel**
- **File**: [`packages/candle/src/capability/text_embedding/nvembed.rs`](../packages/candle/src/capability/text_embedding/nvembed.rs)
- **Pattern**: Add `async` to `load()` signature, add `.await` to `huggingface_file` calls

**8. LoadedStellaModel**
- **File**: [`packages/candle/src/capability/text_embedding/stella.rs`](../packages/candle/src/capability/text_embedding/stella.rs)
- **Status**: Verify if already async or needs conversion

#### Vision Models

**9. LoadedLLaVAModel**
- **File**: [`packages/candle/src/capability/vision/llava.rs`](../packages/candle/src/capability/vision/llava.rs)
- **Pattern**: Add `async` to `load()` signature, add `.await` to `huggingface_file` calls

### Pre-loaded Models (No Changes Needed)
These models are loaded in registry.rs and cloned for workers:
- ClipVisionEmbedding models (image_embedding)
- StableDiffusion/Flux models (text_to_image)

## Part 2: Convert Pool Spawn Function Signatures

### 1. Text-to-Text Worker
**File**: [`packages/candle/src/pool/capabilities/text_to_text.rs`](../packages/candle/src/pool/capabilities/text_to_text.rs)

**Status**: ✅ Signature already converted (lines 162-171), ❌ Missing `.await` at line 226

**Changes Required**:
```rust
// Line 226 - Add .await to model_loader call
let model = match model_loader().await {  // ✅ Add .await
    Ok(m) => {
        log::info!("TextToText worker {} ready", worker_id);
        state_clone.store(WorkerState::Ready as u32, Ordering::Release);
        m
    }
    Err(e) => {
        log::error!("TextToText worker {} failed: {}", worker_id, e);
        state_clone.store(WorkerState::Failed as u32, Ordering::Release);
        text_to_text_pool().remove_memory(per_worker_mb_clone);
        return;
    }
};
```

### 2. Text Embedding Worker
**File**: [`packages/candle/src/pool/capabilities/text_embedding.rs`](../packages/candle/src/pool/capabilities/text_embedding.rs)

**Changes Required**:

**Signature** (line ~189):
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
let model = match model_loader().await {  // ✅ Add .await
```

### 3. Image Embedding Worker
**File**: [`packages/candle/src/pool/capabilities/image_embedding.rs`](../packages/candle/src/pool/capabilities/image_embedding.rs)

**Signature** (line ~239):
```rust
pub fn spawn_image_embedding_worker<T, F, Fut>(
    &self,
    registry_key: &str,
    model_loader: F,
    per_worker_mb: usize,
    allocation_guard: AllocationGuard,
) -> Result<(), PoolError>
where
    T: ImageEmbeddingCapable + Send + 'static,
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<T, PoolError>> + Send + 'static,
```

**Model Loader Call** (line ~280):
```rust
let model = match model_loader().await {  // ✅ Add .await
```

### 4. Text-to-Image Worker
**File**: [`packages/candle/src/pool/capabilities/text_to_image.rs`](../packages/candle/src/pool/capabilities/text_to_image.rs)

**Signature** (line ~137):
```rust
pub fn spawn_text_to_image_worker<T, F, Fut>(
    &self,
    registry_key: &str,
    model_loader: F,
    per_worker_mb: usize,
    allocation_guard: AllocationGuard,
) -> Result<(), PoolError>
where
    T: TextToImageCapable + Send + 'static,
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<T, PoolError>> + Send + 'static,
```

**Model Loader Call** (line ~177):
```rust
let model = match model_loader().await {  // ✅ Add .await
```

### 5. Vision Worker
**File**: [`packages/candle/src/pool/capabilities/vision.rs`](../packages/candle/src/pool/capabilities/vision.rs)

**Signature** (line ~175):
```rust
pub fn spawn_vision_worker<T, F, Fut>(
    &self,
    registry_key: &str,
    model_loader: F,
    per_worker_mb: usize,
    allocation_guard: AllocationGuard,
) -> Result<(), PoolError>
where
    T: VisionCapable + Send + 'static,
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<T, PoolError>> + Send + 'static,
```

**Model Loader Call** (line ~215):
```rust
let model = match model_loader().await {  // ✅ Add .await
```

## Part 3: Update Registry Call Sites

**File**: [`packages/candle/src/capability/registry.rs`](../packages/candle/src/capability/registry.rs)

### Unified Pattern for ALL Models

After Parts 1 & 2 are complete, ALL registry calls use this pattern:

```rust
pool.spawn_X_worker(
    registry_key,
    move || async move {
        LoadedXYZModel::load(&m_clone)
            .await
            .map_err(|e| PoolError::SpawnFailed(e.to_string()))
    },
    per_worker_mb,
    allocation_guard,
)
```

### Text-to-Text Models (3 changes)

**Line 247-252: LoadedKimiK2Model**
```rust
// BEFORE
move || {
    LoadedKimiK2Model::load(&m_clone)
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}

// AFTER
move || async move {
    LoadedKimiK2Model::load(&m_clone)
        .await
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}
```

**Line 279-284: LoadedQwen3CoderModel**
```rust
move || async move {
    LoadedQwen3CoderModel::load(&m_clone)
        .await
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}
```

**Line 313-320: LoadedPhi4ReasoningModel**
```rust
// BEFORE (remove block_in_place/block_on)
move || {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            LoadedPhi4ReasoningModel::load(&m_clone).await
                .map_err(|e| PoolError::SpawnFailed(e.to_string()))
        })
    })
}

// AFTER
move || async move {
    LoadedPhi4ReasoningModel::load(&m_clone)
        .await
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}
```

### Text Embedding Models (9 changes)

Apply the unified pattern to:
- Line 370-379: LoadedGteQwenModel (remove block_in_place/block_on)
- Line 406-411: LoadedJinaBertModel
- Line 437-442: LoadedNvEmbedModel
- Line 468-477: LoadedStellaModel (remove block_in_place/block_on)
- Line 503-508: LoadedBertModel
- Line 555-564: LoadedGteQwenModel (remove block_in_place/block_on)
- Line 619-624: LoadedJinaBertModel
- Line 652-661: LoadedStellaModel (remove block_in_place/block_on)
- Line 683-688: LoadedBertModel

### Image Embedding Models (4 changes)

Pre-loaded models use slightly different pattern:

```rust
// Lines 749, 801, 853, 905
let model = (*m_clone).clone();
pool.spawn_image_embedding_worker(
    registry_key,
    move || async move { Ok(model) },  // ✅ Wrap in async
    per_worker_mb,
    allocation_guard,
)
```

### Vision Models (2 changes)

**Lines 947-952, 984-989: LoadedLLaVAModel**
```rust
move || async move {
    LoadedLLaVAModel::load(&m_clone)
        .await
        .map_err(|e| PoolError::SpawnFailed(e.to_string()))
}
```

### Text-to-Image Models (2 changes)

**Lines 1028-1033, 1057-1062**
```rust
let m_inner = (**m).clone();
pool.spawn_text_to_image_worker(
    registry_key,
    move || async move { Ok(m_inner) },  // ✅ Wrap in async
    per_worker_mb,
    allocation_guard,
)
```

## Implementation Strategy

### Phase 1: Convert Model Load Methods to Async (9 files)
1. kimi_k2.rs - Add `async` to signature, `.await` to huggingface_file calls
2. qwen3_coder.rs - Add `async` to signature, `.await` to huggingface_file calls
3. bert.rs - Add `async` to signature, `.await` to huggingface_file calls
4. gte_qwen.rs - Verify/add `async` conversion
5. jina_bert.rs - Add `async` to signature, `.await` to huggingface_file calls
6. nvembed.rs - Add `async` to signature, `.await` to huggingface_file calls
7. stella.rs - Verify/add `async` conversion
8. llava.rs - Add `async` to signature, `.await` to huggingface_file calls
9. phi4_reasoning.rs - ✅ Already async (verify only)

### Phase 2: Update Spawn Function Signatures (4 files)
1. text_embedding.rs - Add `<Fut>` generic and `Future` bound
2. image_embedding.rs - Add `<Fut>` generic and `Future` bound
3. text_to_image.rs - Add `<Fut>` generic and `Future` bound
4. vision.rs - Add `<Fut>` generic and `Future` bound

### Phase 3: Add `.await` to Model Loader Calls (5 files)
1. text_to_text.rs - Line 226: `model_loader().await`
2. text_embedding.rs - Line ~235: `model_loader().await`
3. image_embedding.rs - Line ~280: `model_loader().await`
4. text_to_image.rs - Line ~177: `model_loader().await`
5. vision.rs - Line ~215: `model_loader().await`

### Phase 4: Update Registry Call Sites
1. Remove all `tokio::task::block_in_place` wrappers (5 instances)
2. Remove all `tokio::runtime::Handle::current().block_on` wrappers (5 instances)
3. Add `async move` wrapper to ALL model load calls
4. Add `.await` to ALL model load calls

## Code Pattern: Complete Before/After

### Model Load Method Conversion

```rust
// BEFORE: packages/candle/src/capability/text_to_text/kimi_k2.rs
impl LoadedKimiK2Model {
    pub fn load(  // ❌ Sync signature
        base: &CandleKimiK2Model,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let gguf_file_path = base
            .huggingface_file(base.info().registry_key, "*.gguf")  // ❌ Missing .await
            .map_err(|e| ...)?;
        
        let tokenizer_path = base
            .huggingface_file(base.info().registry_key, "tokenizer.json")  // ❌ Missing .await
            .map_err(|e| ...)?;
        
        // ... rest of method
    }
}

// AFTER: Proper async implementation
impl LoadedKimiK2Model {
    pub async fn load(  // ✅ Async signature
        base: &CandleKimiK2Model,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let gguf_file_path = base
            .huggingface_file(base.info().registry_key, "*.gguf")
            .await  // ✅ Properly awaited
            .map_err(|e| ...)?;
        
        let tokenizer_path = base
            .huggingface_file(base.info().registry_key, "tokenizer.json")
            .await  // ✅ Properly awaited
            .map_err(|e| ...)?;
        
        // ... rest of method unchanged
    }
}
```

### Registry Call Pattern

```rust
// BEFORE: packages/candle/src/capability/registry.rs
pool.spawn_text_to_text_worker(
    registry_key,
    move || {  // ❌ Sync closure
        LoadedKimiK2Model::load(&m_clone)  // ❌ No .await
            .map_err(|e| PoolError::SpawnFailed(e.to_string()))
    },
    per_worker_mb,
    allocation_guard,
)

// AFTER: Unified async pattern
pool.spawn_text_to_text_worker(
    registry_key,
    move || async move {  // ✅ Async closure
        LoadedKimiK2Model::load(&m_clone)
            .await  // ✅ Properly awaited
            .map_err(|e| PoolError::SpawnFailed(e.to_string()))
    },
    per_worker_mb,
    allocation_guard,
)
```

## Definition of Done

### Part 1: Model Load Methods
1. ✅ All 9 model `load()` methods have `async` keyword
2. ✅ All `huggingface_file` calls have `.await`
3. ✅ All model load methods compile without warnings

### Part 2: Spawn Functions
1. ✅ All 5 spawn worker functions accept async closures (`F: FnOnce() -> Fut`)
2. ✅ All 5 spawn worker functions call `model_loader().await`
3. ✅ All spawn functions compile without errors

### Part 3: Registry
1. ✅ Zero instances of `tokio::task::block_in_place` in registry.rs
2. ✅ Zero instances of `tokio::runtime::Handle::current().block_on` in registry.rs
3. ✅ All registry spawn calls use `async move` closure syntax
4. ✅ All registry spawn calls include `.await` on model load

### Verification
1. ✅ Code compiles without errors or warnings
2. ✅ Worker spawning functions correctly for all model types
3. ✅ No blocking async patterns in codebase

## Verification Commands

```bash
# Verify no blocking patterns remain in registry
rg "block_in_place" packages/candle/src/capability/registry.rs
rg "block_on" packages/candle/src/capability/registry.rs
# Expected: No matches found

# Verify all model load methods are async
rg "pub async fn load" packages/candle/src/capability/text_to_text/
rg "pub async fn load" packages/candle/src/capability/text_embedding/
rg "pub async fn load" packages/candle/src/capability/vision/
# Expected: 9 total matches

# Verify huggingface_file calls use .await
rg "huggingface_file.*\.await" packages/candle/src/capability/
# Expected: Multiple matches in load methods

# Verify spawn function async signatures
rg "pub fn spawn_.*_worker.*Fut" packages/candle/src/pool/capabilities/
# Expected: 5 matches (one per capability file)

# Verify model_loader().await in spawn functions
rg "model_loader\(\)\.await" packages/candle/src/pool/capabilities/
# Expected: 5 matches (one per capability file)
```

## Impact Summary

**Total Changes**:
- 9 model load method async conversions
- ~18+ `.await` additions in model load methods
- 4 spawn function signature changes (text_to_text already done)
- 5 `.await` additions in spawn functions
- ~23 closure pattern updates in registry.rs
- 5 `block_in_place`/`block_on` removals

**Risk Level**: Medium - Changes affect model loading code paths but are mechanical type system updates

**Testing Focus**: Verify worker spawning under various conditions (cold start, concurrent requests, model loading failures, network issues during HuggingFace downloads)
