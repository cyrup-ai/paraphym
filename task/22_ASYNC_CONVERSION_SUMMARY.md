# Async Conversion - Remaining Tasks

After 3-day async conversion, these synchronous methods remain and need fixing:

## 🔴 Critical - Must Fix
1. **`CandleModel::huggingface_file`** → `ASYNC_FIX_HUGGINGFACE.md`
   - Uses blocking HuggingFace API for network downloads
   - Blocks tokio runtime during I/O

2. **`SpawnLock::wait_for_workers`** → `ASYNC_FIX_SPAWN_LOCK.md`
   - Method name contains "wait" with timeout parameter
   - Likely blocks runtime during wait

## 🟡 Medium - Type Errors
3. **`VoiceService` (voice/mod.rs)** → `ASYNC_FIX_VOICE_SERVICE.md`
   - Uses undefined `crate::AsyncTask` type
   - Duplicate of correct async version in domain/voice/mod.rs

## 🟢 Investigate
4. **`TemplateStore`** → `ASYNC_VERIFY_TEMPLATE_STORE.md`
   - Need to verify if implementations do I/O
   - Convert if necessary

## ✅ Verified Sync (OK to keep)
These are legitimately synchronous and should NOT be converted:

### Field Accessors
- `CandleModel::{name, provider, max_input_tokens, max_output_tokens, ...}`
- `CandleAgentRole::{name, temperature, max_tokens, ...}`
- `PoolWorkerHandle::{core, core_mut, registry_key}`

### Simple Checks (if no I/O)
- `HasWorkers::has_workers` (if just checking a bool/counter)
- `WorkerMetrics::{worker_count, busy_worker_count}` (if atomic reads)
- `SpawnLock::try_acquire_spawn_lock` (non-blocking try)

### Pure Computation
- `TemplateEngine::render` (if just string interpolation)
- Constructor methods like `CandleAgentRole::new`

## Philosophy
**After async conversion, the only valid synchronous methods are:**
1. Pure field accessors (returning `&str`, numbers from memory)
2. Pure constructors (just creating structs)
3. Pure computation (no I/O, no waiting, just CPU)

**Everything else should be async.**

## Next Steps
1. Fix critical blocking I/O (tasks 1-2)
2. Fix type errors (task 3)
3. Investigate potential I/O (task 4)
4. Verify all remaining sync methods are legitimately pure
