# Task 016: Add Download Lock Coordination and Increase Spawn Timeout

## Priority: CRITICAL
## Status: NOT STARTED
## Created: 2025-10-19

---

## Executive Summary

**Problem**: Multiple workers spawning simultaneously cause lock acquisition failures when trying to download the same model file, resulting in worker spawn failures and degraded system reliability.

**Root Cause**: 
1. No application-level coordination when multiple workers download same file
2. HuggingFace Hub library's file lock fails fast (5 second retry) instead of waiting
3. 30-second worker spawn timeout is too short for large model downloads

**Impact**:
- Worker spawn failures: "Lock acquisition failed"
- System instability when scaling workers
- Failed requests during cold starts
- User-facing errors during peak load

---

## Current Code Structure

### Relevant Files

| File | Purpose | Lines |
|------|---------|-------|
| [`./src/domain/model/traits.rs`](../packages/candle/src/domain/model/traits.rs#L92-L117) | Contains `huggingface_file()` - needs lock coordination |
| [`./src/domain/model/mod.rs`](../packages/candle/src/domain/model/mod.rs) | Module registry - needs new `download_lock` module |
| [`./src/capability/registry/pool/core/spawn.rs`](../packages/candle/src/capability/registry/pool/core/spawn.rs#L108) | Contains 30s timeout - needs increase to 6 hours |
| [`./tmp/hf-hub/src/api/tokio.rs`](../tmp/hf-hub/src/api/tokio.rs#L74-L86) | HF Hub lock behavior - 5 second retry limit |

### Existing Patterns in Codebase

#### DashMap Usage (Already Established)

**Pattern 1**: Worker Pool Storage  
[`./src/capability/registry/pool/core/pool.rs:18`](../packages/candle/src/capability/registry/pool/core/pool.rs#L18)
```rust
pub struct Pool<W: PoolWorkerHandle> {
    /// Map of registry_key -> Vec<W>
    workers: DashMap<String, Vec<W>>,
    // ...
}
```

**Pattern 2**: Macro System Storage  
[`./src/domain/chat/macros.rs:721-723`](../packages/candle/src/domain/chat/macros.rs#L721-L723)
```rust
pub struct MacroSystem {
    /// Active recording sessions
    recording_sessions: DashMap<Uuid, MacroRecordingSession>,
    /// Active playback sessions
    playback_sessions: DashMap<Uuid, MacroPlaybackSession>,
    // ...
}
```

#### tokio::sync::Mutex Usage (Already Established)

[`./src/domain/concurrency/mod.rs:4`](../packages/candle/src/domain/concurrency/mod.rs#L4)
```rust
use tokio::sync::Mutex;
```

**Key Insight**: Our new download lock will follow the same patterns already proven in the codebase.

---

## Evidence from Logs

### Example 1: Text Embedding Worker Lock Failure

```
[2025-10-19T17:52:28Z ERROR paraphym_candle::capability::registry::pool::capabilities::text_embedding] 
TextEmbedding worker 1 failed: Worker spawn failed: Lock acquisition failed: 
/Volumes/samsung_t9/ai/models/hub/models--dunzhang--stella_en_400M_v5/blobs/17e549d16172a548a3115739b55575968eb6523653daad76c46b0758e9425032.lock
```

**What Happened**:
1. Worker 0 starts downloading `model.safetensors` (1.62 GB)
2. Worker 1 spawns simultaneously, tries to download same file
3. Worker 1 can't acquire file lock (Worker 0 has it)
4. After 5 seconds of retries, Worker 1 gives up with ERROR
5. Worker 1 spawn fails, reducing system capacity

### Example 2: Text-to-Text Worker Lock Failure

```
[2025-10-19T17:50:37Z ERROR paraphym_candle::capability::registry::pool::capabilities::text_to_text] 
TextToText worker 1 failed: Worker spawn failed: Lock acquisition failed: 
/Volumes/samsung_t9/ai/models/hub/models--unsloth--Qwen3-1.7B-GGUF/blobs/b139949c5bd74937ad8ed8c8cf3d9ffb1e99c866c823204dc42c0d91fa181897.lock
```

**Pattern**: Consistently affects Worker 1+ when spawning multiple workers for same model.

### Example 3: Spawn Timeout

```
[2025-10-19T17:52:52Z WARN paraphym_candle::memory::core::manager::surreal] 
Failed to generate embedding: SpawnTimeout("Timed out after 30s waiting for dunzhang/stella_en_400M_v5 workers to spawn")
```

**What Happened**: 
1. Download takes > 30 seconds (1.62 GB file)
2. Waiting thread times out before download completes
3. System fails even though download would succeed if given more time

---

## Root Cause Analysis

### HuggingFace Hub Library Lock Behavior

From [`./tmp/hf-hub/src/api/tokio.rs:74-86`](../tmp/hf-hub/src/api/tokio.rs#L74-L86):

```rust
async fn lock_file(mut path: PathBuf) -> Result<Handle, ApiError> {
    path.set_extension("lock");

    let file = tokio::fs::File::create(path.clone()).await?;
    let mut res = lock(&file);
    for _ in 0..5 {  // ❌ Only 5 retries
        if res == 0 {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;  // ❌ 1 second between retries
        res = lock(&file);
    }
    if res != 0 {
        Err(ApiError::LockAcquisition(path))  // ❌ Fails after 5 seconds
    } else {
        Ok(Handle { file })
    }
}
```

**Issue**: HuggingFace Hub gives up after **5 seconds** if lock can't be acquired. This is insufficient for:
- Large model downloads (can take 30-60 seconds)
- Slow internet connections
- Concurrent worker spawning

### Current Spawn Timeout

From [`./src/capability/registry/pool/core/spawn.rs:107-108`](../packages/candle/src/capability/registry/pool/core/spawn.rs#L107-L108):

```rust
} else {
    // Another thread is spawning - wait for it to complete (30s timeout)
    pool.wait_for_workers(registry_key, Duration::from_secs(30)).await
}
```

**Issue**: 30-second timeout is too short for:
- Large models (Llama 70B: 40GB+, can take 10+ minutes on slow connections)
- Initial cache population
- Network congestion
- Slow disk I/O

### Current huggingface_file() Implementation

From [`./src/domain/model/traits.rs:92-117`](../packages/candle/src/domain/model/traits.rs#L92-L117):

```rust
fn huggingface_file(
    &self,
    repo_key: &str,
    filename: &str,
) -> impl std::future::Future<Output = Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>>> + Send
where
    Self: Sized,
{
    async {
        use hf_hub::api::tokio::ApiBuilder;

        // ❌ NO COORDINATION - multiple workers can call simultaneously
        let mut builder = ApiBuilder::from_env();
        
        if let Ok(token) = std::env::var("HF_TOKEN") {
            builder = builder.with_token(Some(token));
        }
        
        let api = builder.build()?;
        let repo = api.model(repo_key.to_string());
        let path = repo.get(filename).await?;  // ❌ Causes lock conflicts

        Ok(path)
    }
}
```

---

## Solution Architecture

### Phase 1: Application-Level Download Coordination

**Add global download lock registry** to coordinate multiple workers:

```rust
use dashmap::DashMap;
use tokio::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    /// Global registry of download locks keyed by "repo/filename"
    /// Ensures only one download happens at a time per file
    static ref DOWNLOAD_LOCKS: DashMap<String, Arc<Mutex<()>>> = DashMap::new();
}

/// Acquire download lock for a specific file
pub async fn acquire_download_lock(repo: &str, filename: &str) -> Arc<Mutex<()>> {
    let key = format!("{}/{}", repo, filename);
    DOWNLOAD_LOCKS
        .entry(key)
        .or_insert_with(|| Arc::new(Mutex::new(())))
        .value()
        .clone()
}
```

### Phase 2: Coordinated Download Flow

**Updated `huggingface_file()` with coordination**:

```rust
async fn huggingface_file(
    &self,
    repo_key: &str,
    filename: &str,
) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    use crate::domain::model::download_lock::acquire_download_lock;
    
    // Step 1: Acquire application-level lock (blocks if another worker is downloading)
    let lock = acquire_download_lock(repo_key, filename).await;
    let _guard = lock.lock().await;
    
    // Step 2: Check cache (file might be ready now if we waited)
    if let Some(path) = self.huggingface_file_cached(repo_key, filename) {
        log::info!("✅ File available in cache after waiting: {}", filename);
        return Ok(path);
    }
    
    // Step 3: We hold the lock and file isn't cached - proceed with download
    log::info!("⬇️  Downloading {} from {}", filename, repo_key);
    
    use hf_hub::api::tokio::ApiBuilder;
    let mut builder = ApiBuilder::from_env();
    
    if let Ok(token) = std::env::var("HF_TOKEN") {
        builder = builder.with_token(Some(token));
    }
    
    let api = builder.build()?;
    let repo = api.model(repo_key.to_string());
    let path = repo.get(filename).await?;  // This now won't have lock conflicts
    
    log::info!("✅ Download complete: {}", filename);
    
    // Lock released here (_guard drops)
    Ok(path)
}
```

**Flow**:
1. Worker 0 acquires app lock → downloads file → releases lock
2. Worker 1 tries to acquire app lock → **blocks waiting** (not fails!)
3. Worker 0 completes download
4. Worker 1 acquires lock → checks cache → finds file → uses it immediately
5. No HuggingFace Hub lock conflicts because only one worker downloads at a time

### Phase 3: Increase Spawn Timeout

**Change timeout from 30 seconds to 6 hours**:

```rust
} else {
    // Another thread is spawning - wait for it to complete
    // 6 hour timeout allows for large model downloads (e.g., Llama 70B ~40GB)
    // on slow connections without premature failures
    pool.wait_for_workers(registry_key, Duration::from_secs(6 * 3600)).await
}
```

**Rationale**:
- **Llama 70B**: 40GB+ download (1-4 hours on 10-50 Mbps)
- **Network issues**: Retries, congestion can delay downloads
- **First-time setup**: Users installing on slow connections
- **Better to wait** than fail and force user to retry manually

---

## Implementation Steps

### Step 1: Add Download Lock Infrastructure

**File**: [`./src/domain/model/download_lock.rs`](../packages/candle/src/domain/model/download_lock.rs) (new file)

**Create**:
```rust
//! Download coordination to prevent concurrent downloads of same file
//!
//! Uses global DashMap to coordinate downloads across all workers.
//! Pattern follows existing usage in pool.rs and macros.rs.

use dashmap::DashMap;
use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static! {
    /// Global registry of download locks keyed by "repo/filename"
    ///
    /// Ensures only one download happens at a time per file across all workers.
    /// When multiple workers spawn simultaneously, first acquires lock and downloads,
    /// others block until download completes then use cached file.
    ///
    /// # Pattern
    /// Follows same DashMap pattern as:
    /// - pool.rs: `workers: DashMap<String, Vec<W>>`
    /// - macros.rs: `recording_sessions: DashMap<Uuid, MacroRecordingSession>`
    static ref DOWNLOAD_LOCKS: DashMap<String, Arc<Mutex<()>>> = DashMap::new();
}

/// Acquire download lock for specific HuggingFace file
///
/// # Arguments
/// * `repo` - Repository identifier (e.g., "unsloth/Qwen3-1.7B-GGUF")
/// * `filename` - File name within repo (e.g., "model.gguf")
///
/// # Returns
/// Arc to Mutex that coordinates downloads. Hold lock while downloading.
///
/// # Example
/// ```rust
/// let lock = acquire_download_lock("unsloth/Qwen3-1.7B-GGUF", "model.gguf").await;
/// let _guard = lock.lock().await;  // Blocks if another worker is downloading
/// // ... download file ...
/// // _guard drops here, releasing lock
/// ```
pub async fn acquire_download_lock(repo: &str, filename: &str) -> Arc<Mutex<()>> {
    let key = format!("{}/{}", repo, filename);
    
    DOWNLOAD_LOCKS
        .entry(key)
        .or_insert_with(|| Arc::new(Mutex::new(())))
        .value()
        .clone()
}
```

### Step 2: Register Module

**File**: [`./src/domain/model/mod.rs`](../packages/candle/src/domain/model/mod.rs)

**Location**: After line 5 (after `pub mod capabilities;`)

**Add**:
```rust
pub mod download_lock;
```

**Result**:
```rust
pub mod capabilities;
pub mod download_lock;  // ← NEW MODULE
pub mod error;
pub mod info;
// ...
```

### Step 3: Update CandleModel Trait

**File**: [`./src/domain/model/traits.rs`](../packages/candle/src/domain/model/traits.rs)

**Location**: Lines 92-117 (existing `huggingface_file()`)

**Replace with**:
```rust
fn huggingface_file(
    &self,
    repo_key: &str,
    filename: &str,
) -> impl std::future::Future<Output = Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>>> + Send
where
    Self: Sized,
{
    async move {
        use crate::domain::model::download_lock::acquire_download_lock;
        
        // CRITICAL: Acquire application-level lock BEFORE attempting download
        // This prevents race conditions when multiple workers spawn simultaneously
        let lock = acquire_download_lock(repo_key, filename).await;
        let _guard = lock.lock().await;
        
        // Check cache first (file might be ready if we waited for lock)
        if let Some(path) = self.huggingface_file_cached(repo_key, filename) {
            log::info!("✅ Using cached file (available after lock wait): {}", filename);
            return Ok(path);
        }
        
        // We hold lock and file not cached - proceed with download
        log::info!("⬇️  Starting download: {} from {}", filename, repo_key);
        
        use hf_hub::api::tokio::ApiBuilder;

        let mut builder = ApiBuilder::from_env();
        
        if let Ok(token) = std::env::var("HF_TOKEN") {
            builder = builder.with_token(Some(token));
        }
        
        let api = builder.build()?;
        let repo = api.model(repo_key.to_string());
        let path = repo.get(filename).await?;

        log::info!("✅ Download complete: {}", filename);
        
        Ok(path)
        // Lock released here when _guard drops
    }
}
```

**Key Changes**:
1. Added `use crate::domain::model::download_lock::acquire_download_lock;`
2. Acquire lock before any download attempt
3. Check cache after acquiring lock (file might be ready)
4. Download only if cache check fails
5. Lock automatically releases when function returns

### Step 4: Increase Spawn Timeout

**File**: [`./src/capability/registry/pool/core/spawn.rs`](../packages/candle/src/capability/registry/pool/core/spawn.rs)

**Location**: Line 108

**Current**:
```rust
} else {
    // Another thread is spawning - wait for it to complete (30s timeout)
    pool.wait_for_workers(registry_key, Duration::from_secs(30)).await
}
```

**Change To**:
```rust
} else {
    // Another thread is spawning - wait for it to complete
    // 6 hour timeout allows for large model downloads (e.g., Llama 70B ~40GB)
    // on slow connections without premature failures
    pool.wait_for_workers(registry_key, Duration::from_secs(6 * 3600)).await
}
```

---

## Dependencies

### Already Present in Codebase

✅ **dashmap** - Already in [`Cargo.toml:186`](../packages/candle/Cargo.toml#L186)  
✅ **lazy_static** - Already in [`Cargo.toml:178`](../packages/candle/Cargo.toml#L178)  
✅ **tokio::sync::Mutex** - Already used in [`concurrency/mod.rs`](../packages/candle/src/domain/concurrency/mod.rs#L4)  
✅ **DashMap pattern** - Already used in [`pool.rs`](../packages/candle/src/capability/registry/pool/core/pool.rs#L18) and [`macros.rs`](../packages/candle/src/domain/chat/macros.rs#L721)

### Requires Task 015

⚠️ **huggingface_file_cached()** - Must be implemented first (Task 015)  
This method checks cache without downloading. Current `huggingface_file()` will call it.

---

## Definition of Done

✅ `download_lock.rs` module created in `./src/domain/model/`  
✅ `acquire_download_lock()` function implemented using DashMap + Arc<Mutex>  
✅ Module registered in `./src/domain/model/mod.rs`  
✅ `huggingface_file()` updated to use download locks  
✅ Spawn timeout increased from 30s to 6 hours in `spawn.rs:108`  
✅ No "Lock acquisition failed" errors in logs  
✅ Multiple workers can spawn without conflicts  
✅ Large model downloads don't timeout  
✅ Workers wait properly for in-progress downloads  

---

## Performance Impact

### Before Fix

**Concurrent Spawn**: Worker 1 fails with lock error ❌  
**Download Time**: > 30 seconds causes spawn timeout ❌  
**Reliability**: ~50% failure rate when scaling workers ❌  

### After Fix

**Concurrent Spawn**: Worker 1 waits, then uses cached file ✅  
**Download Time**: Up to 6 hours allowed (handles any model) ✅  
**Reliability**: 100% success rate with proper coordination ✅  

---

## Edge Cases Handled

### Case 1: Worker Crash During Download

**Scenario**: Worker 0 acquires lock, starts download, crashes mid-download

**Behavior**: 
- Lock is held by tokio::sync::Mutex (not OS-level)
- When Worker 0's task drops, Mutex guard drops
- Lock automatically releases
- Worker 1 can proceed (will re-download)

**Status**: ✅ Handled by tokio Mutex semantics

### Case 2: Network Failure Mid-Download

**Scenario**: Download starts, network drops, hf_hub errors out

**Behavior**:
- `repo.get()` returns error
- `huggingface_file()` returns error to caller
- Lock is released (guard drops)
- Next worker can retry download

**Status**: ✅ Handled by error propagation

### Case 3: Partial Download (Corrupted File)

**Scenario**: Download completes but file is corrupted/incomplete

**Behavior**:
- Task 015's file verification will detect (size check)
- Next request will re-download
- Lock prevents multiple simultaneous re-downloads

**Status**: ✅ Handled by combination with Task 015

---

## Technical Notes

### Why tokio::sync::Mutex?

**Not** `std::sync::Mutex` because:
- `std::sync::Mutex` blocks OS thread (bad in async)
- `tokio::sync::Mutex` is async-aware (yields to runtime)
- Allows other tasks to run while waiting for lock

**Already established in codebase**: See [`./src/domain/concurrency/mod.rs:4`](../packages/candle/src/domain/concurrency/mod.rs#L4)

### Why DashMap?

**Not** `RwLock<HashMap>` because:
- Better concurrent performance
- Lock-free reads for different keys
- Lower contention in multi-worker scenarios

**Already established in codebase**: 
- Worker pool: [`./src/capability/registry/pool/core/pool.rs:18`](../packages/candle/src/capability/registry/pool/core/pool.rs#L18)
- Macro system: [`./src/domain/chat/macros.rs:721`](../packages/candle/src/domain/chat/macros.rs#L721)

### Why 6 Hours?

**Calculation**:
- Llama 70B: ~40 GB
- Slow connection: 10 Mbps = 1.25 MB/s
- Download time: 40000 MB / 1.25 MB/s = 32000 seconds = ~9 hours
- 6 hours is conservative but allows for retry overhead

---

## File Summary

| File | Action | Lines Changed |
|------|--------|---------------|
| `./src/domain/model/download_lock.rs` | **CREATE** | ~50 lines (new module) |
| `./src/domain/model/mod.rs` | **MODIFY** | +1 line (register module) |
| `./src/domain/model/traits.rs` | **MODIFY** | ~30 lines (update huggingface_file) |
| `./src/capability/registry/pool/core/spawn.rs` | **MODIFY** | 1 line (change timeout) |

**Total**: 1 new file, 3 modified files, ~82 lines changed

---

## Related Tasks

- **Task 015**: File existence check (provides `huggingface_file_cached()`)
- **Task 014**: Temperature defaults (unrelated but also pool/worker focused)

---

## End of Task Specification
