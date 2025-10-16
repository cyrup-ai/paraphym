# Synchronous Traits Requiring Async Conversion in `./packages/candle`

This document catalogs all synchronous traits in the Candle package that need to be converted to async patterns.

## Executive Summary

**Total Traits Analyzed**: 35+
**Already Async (No conversion needed)**: 25+
**Need Async Conversion**: 10

---

## ✅ Already Async - No Conversion Needed

These traits are already using async patterns (Stream/Future-based) and do NOT need conversion:

### Stream-Based Traits (Already Async)
1. **`CandleWorkflowStep<In, Out>`** (`workflow/core.rs`)
   - Uses `Pin<Box<dyn Stream<Item = Out> + Send>>`
   - ✅ Fully async

2. **`Op<In, Out>`** and **`DynOp<In, Out>`** (`workflow/ops.rs`)
   - Uses `Pin<Box<dyn Stream<Item = Out> + Send>>`
   - ✅ Fully async

3. **`EmbeddingService`** (`domain/embedding/service.rs`)
   - Methods return `Pin<Box<dyn Stream<Item = ...> + Send>>`
   - ✅ Fully async

4. **`CandleMemory`** (`domain/memory/traits.rs`)
   - All methods return `Pin<Box<dyn Stream<Item = ...> + Send>>`
   - ✅ Fully async

5. **`TextToTextCapable`** (`capability/traits.rs`)
   - `prompt()` returns `Pin<Box<dyn Stream<Item = CandleCompletionChunk> + Send>>`
   - ✅ Fully async

6. **`VisionCapable`** (`capability/traits.rs`)
   - `describe_image()` and `describe_url()` return `Pin<Box<dyn Stream<Item = CandleStringChunk> + Send>>`
   - ✅ Fully async

7. **`TextToImageCapable`** (`capability/traits.rs`)
   - `generate_image()` returns `Pin<Box<dyn Stream<Item = ImageGenerationChunk> + Send>>`
   - ✅ Fully async

8. **`ImageGenerationModel`** (`domain/image_generation/mod.rs`)
   - `generate()` returns `Pin<Box<dyn Stream<Item = ImageGenerationChunk> + Send>>`
   - ✅ Fully async

9. **`VoiceService`** (`domain/voice/mod.rs`)
   - Uses `Pin<Box<dyn Future<Output = ...> + Send + '_>>`
   - ✅ Fully async (Future-based)
   - Note: Marked as unused but provides correct async interface

10. **`DomainCommandExecutor`** (`domain/chat/commands/types/mod.rs`)
    - `execute()` returns `Pin<Box<dyn Stream<Item = CommandExecutionResult> + Send>>`
    - ✅ Fully async

11. **`CandleContext`** (`domain/context/traits.rs`)
    - `load_context()` returns `Pin<Box<dyn Stream<Item = ...> + Send>>`
    - ✅ Fully async

12. **`CandleImmutableEmbeddingModel`** (`domain/context/provider.rs`)
    - `embed()` returns `Pin<Box<dyn Stream<Item = Vec<f32>> + Send>>`
    - ✅ Fully async

13. **`CandleImmutableMemoryManager`** (`domain/context/provider.rs`)
    - `create_memory()` returns `Pin<Box<dyn Stream<Item = ()> + Send>>`
    - ✅ Fully async

### Future-Based Traits (Already Async)
14. **`TextEmbeddingCapable`** (`capability/traits.rs`)
    - `embed()` and `batch_embed()` return `Pin<Box<dyn Future<Output = ...> + Send + '_>>`
    - ✅ Fully async

15. **`ImageEmbeddingCapable`** (`capability/traits.rs`)
    - All methods return `Pin<Box<dyn Future<Output = ...> + Send + '_>>`
    - ✅ Fully async

16. **`Loader<T>`** (`domain/context/loader.rs`)
    - `load_all()` returns `tokio::task::JoinHandle<...>`
    - `stream_files()` returns `impl Stream<Item = ...>`
    - ✅ Fully async

17. **`Extractor<T>`** (`domain/context/extraction/extractor.rs`)
    - Likely async (needs verification of methods)
    - ✅ Likely async

---

## ⚠️ Synchronous Traits That NEED Async Conversion

### 1. **`CandleModel`** (`domain/model/traits.rs`) - CRITICAL
**Location**: Lines 12-108

**Problem Method**:
```rust
fn huggingface_file(
    &self,
    repo_key: &str,
    filename: &str,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>>
```

**Issue**: Uses **blocking** `hf_hub::api::sync::Api` for HuggingFace downloads
- Line 100: `let api = Api::new()?;` - BLOCKING
- Line 102: `let path = repo.get(filename)?;` - BLOCKING network I/O

**Required Change**: Convert to async using `hf_hub::api::tokio::Api`
```rust
async fn huggingface_file(
    &self,
    repo_key: &str,
    filename: &str,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>>
{
    use hf_hub::api::tokio::Api;  // Use tokio async API
    
    let api = Api::new().await?;
    let repo = api.model(repo_key.to_string());
    let path = repo.get(filename).await?;
    Ok(path)
}
```

**Impact**: HIGH - Core trait used by all models

---

### 2. **`VoiceService`** (`voice/mod.rs`) - UNKNOWN TYPE
**Location**: Lines 37-49

**Problem**: Uses `crate::AsyncTask` type which is not defined anywhere
```rust
fn transcribe(&self, request: TranscriptionRequest) 
    -> crate::AsyncTask<Result<TranscriptionResponse<()>>>;

fn synthesize(&self, text: &str, voice_id: &str) 
    -> crate::AsyncTask<Result<Vec<u8>>>;

fn list_voices(&self) 
    -> crate::AsyncTask<Result<Vec<VoiceInfo>>>;
```

**Issue**: `crate::AsyncTask` type is not defined in the codebase

**Required Action**: 
1. **Option A**: Define `AsyncTask` as a type alias
   ```rust
   pub type AsyncTask<T> = Pin<Box<dyn Future<Output = T> + Send>>;
   ```

2. **Option B**: Replace with standard async pattern
   ```rust
   fn transcribe(&self, request: TranscriptionRequest) 
       -> Pin<Box<dyn Future<Output = Result<TranscriptionResponse<()>>> + Send + '_>>;
   ```

3. **Option C**: Use the pattern from `domain/voice/mod.rs` (which is already correct)

**Impact**: MEDIUM - Trait is documented as unused but blocks compilation

---

### 3. **`TemplateEngine`** (`domain/chat/templates/engines.rs`)
**Location**: Lines 12-29

**Synchronous Methods**:
```rust
fn render(&self, template: &CandleChatTemplate, context: &CandleTemplateContext) 
    -> CandleTemplateResult<String>;

fn supports(&self, template: &CandleChatTemplate) -> bool;

fn name(&self) -> &'static str;
```

**Issue**: Template rendering can involve I/O (file loading, network fetches)

**Required Change**: Make `render` async
```rust
async fn render(&self, template: &CandleChatTemplate, context: &CandleTemplateContext) 
    -> CandleTemplateResult<String>;
```

**Impact**: MEDIUM - Template operations could benefit from async I/O

---

### 4. **`TemplateStore`** (`domain/chat/templates/cache/store.rs`)
**Location**: Lines 10-45

**All Synchronous Methods**:
```rust
fn store(&self, template: &ChatTemplate) -> TemplateResult<()>;
fn get(&self, name: &str) -> TemplateResult<Option<ChatTemplate>>;
fn delete(&self, name: &str) -> TemplateResult<bool>;
fn list(&self) -> TemplateResult<Vec<String>>;
fn exists(&self, name: &str) -> TemplateResult<bool>;
```

**Issue**: All methods involve I/O (database, filesystem, cache access)

**Required Change**: Convert all methods to async
```rust
async fn store(&self, template: &ChatTemplate) -> TemplateResult<()>;
async fn get(&self, name: &str) -> TemplateResult<Option<ChatTemplate>>;
async fn delete(&self, name: &str) -> TemplateResult<bool>;
async fn list(&self) -> TemplateResult<Vec<String>>;
async fn exists(&self, name: &str) -> TemplateResult<bool>;
```

**Impact**: HIGH - Storage operations should always be async

---

### 5. **`HasWorkers`** (`pool/core/spawn.rs`)
**Location**: Lines 227-229

**Synchronous Method**:
```rust
fn has_workers(&self, registry_key: &str) -> bool;
```

**Issue**: Simple check, but may involve lock contention

**Required Change**: Consider async if worker registry access is expensive
```rust
async fn has_workers(&self, registry_key: &str) -> bool;
```

**Impact**: LOW - Simple boolean check, async conversion optional

---

### 6. **`MemoryGovernorAccess`** (`pool/core/spawn.rs`)
**Location**: Lines 232-234

**Synchronous Method**:
```rust
fn memory_governor(&self) -> Arc<MemoryGovernor>;
```

**Issue**: Returns Arc, likely cheap to clone

**Required Change**: Probably OK as-is (returning Arc is cheap)

**Impact**: VERY LOW - Arc cloning is fast, no conversion needed

---

### 7. **`SpawnLock`** (`pool/core/spawn.rs`)
**Location**: Lines 237-240

**Synchronous Methods**:
```rust
fn try_acquire_spawn_lock(&self, registry_key: &str) -> Option<SpawnGuard>;
fn wait_for_workers(&self, registry_key: &str, timeout: Duration) -> Result<(), PoolError>;
```

**Issue**: `wait_for_workers` uses **blocking wait** with timeout

**Required Change**: Convert `wait_for_workers` to async
```rust
async fn wait_for_workers(&self, registry_key: &str, timeout: Duration) 
    -> Result<(), PoolError>;
```

**Impact**: HIGH - Blocking wait should be async

---

### 8. **`WorkerMetrics`** (`pool/core/spawn.rs`)
**Location**: Lines 243-246

**Synchronous Methods**:
```rust
fn worker_count(&self, registry_key: &str) -> usize;
fn busy_worker_count(&self, registry_key: &str) -> usize;
```

**Issue**: Simple counter reads, likely using atomic operations

**Required Change**: Probably OK as-is

**Impact**: VERY LOW - Simple reads, no conversion needed

---

### 9. **`PoolWorkerHandle`** (`pool/core/types.rs`)
**Location**: Lines 473-482

**Synchronous Methods**:
```rust
fn core(&self) -> &WorkerHandle;
fn core_mut(&mut self) -> &mut WorkerHandle;
fn registry_key(&self) -> &str;
```

**Issue**: Simple field access, no I/O

**Required Change**: None needed

**Impact**: NONE - Field access doesn't need async

---

### 10. **`CandleAgentRole`** (`domain/agent/role.rs`)
**Location**: Lines 82-100

**All Synchronous Methods**:
```rust
fn name(&self) -> &str;
fn temperature(&self) -> f64;
fn max_tokens(&self) -> Option<u64>;
fn memory_read_timeout(&self) -> Option<u64>;
fn system_prompt(&self) -> Option<&str>;
fn new(name: impl Into<String>) -> Self;
```

**Issue**: Simple field accessors and constructors

**Required Change**: None needed

**Impact**: NONE - Field access doesn't need async

---

## Priority Matrix

### 🔴 CRITICAL (Must Convert)
1. **`CandleModel::huggingface_file`** - Blocking network I/O
2. **`TemplateStore`** (all methods) - Blocking storage I/O
3. **`SpawnLock::wait_for_workers`** - Blocking wait with timeout

### 🟡 MEDIUM (Should Convert)
4. **`VoiceService`** - Fix AsyncTask type definition
5. **`TemplateEngine::render`** - May involve I/O

### 🟢 LOW (Optional/Not Needed)
6. **`HasWorkers`** - Simple check, optional
7. **`MemoryGovernorAccess`** - Arc cloning, no change needed
8. **`WorkerMetrics`** - Simple counters, no change needed
9. **`PoolWorkerHandle`** - Field access, no change needed
10. **`CandleAgentRole`** - Field access, no change needed

---

## Migration Strategy

### Phase 1: Critical Blocking I/O
1. Convert `CandleModel::huggingface_file` to use `hf_hub::api::tokio::Api`
2. Convert all `TemplateStore` methods to async
3. Convert `SpawnLock::wait_for_workers` to async

### Phase 2: Type Fixes
4. Define `AsyncTask` type or replace with standard pattern in `VoiceService`

### Phase 3: Optional Improvements
5. Consider async conversion for `TemplateEngine::render`
6. Evaluate `HasWorkers` based on actual implementation complexity

---

## Notes

- All traits already using `Pin<Box<dyn Stream<...> + Send>>` or `Pin<Box<dyn Future<...> + Send>>` are correctly async
- Simple field accessors and Arc returns do NOT need async conversion
- Focus on methods that involve I/O, network, or blocking waits

---

**Last Updated**: 2025-10-15
**Status**: Initial analysis complete, ready for implementation
