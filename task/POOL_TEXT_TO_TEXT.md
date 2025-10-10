# POOL_TEXT_TO_TEXT: Integrate TextToText Pool into Registry

## OBJECTIVE

Integrate the TextToText pool into `capability/registry.rs` so that all text generation models (KimiK2, Qwen3Coder, Phi4Reasoning) route through the pool with automatic worker spawning and memory management.

## SCOPE

**3 Models:**
- KimiK2
- Qwen3Coder
- Phi4Reasoning

**1 Method:**
- `prompt()` - returns AsyncStream<CandleCompletionChunk>

## CURRENT STATE (Direct Call)

**File**: `packages/candle/src/capability/registry.rs` lines ~223-232

```rust
impl TextToTextCapable for TextToTextModel {
    fn prompt(
        &self,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> AsyncStream<CandleCompletionChunk> {
        match self {
            Self::KimiK2(m) => m.prompt(prompt, params),
            Self::Qwen3Coder(m) => m.prompt(prompt, params),
            Self::Phi4Reasoning(m) => m.prompt(prompt, params),
        }
    }
}
```

## REQUIRED STATE (Pool Integration)

Add cold start logic and pool routing for all 3 models:

```rust
impl TextToTextCapable for TextToTextModel {
    fn prompt(
        &self,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> AsyncStream<CandleCompletionChunk> {
        match self {
            Self::KimiK2(m) => {
                let registry_key = m.info().registry_key;
                let pool = text_to_text_pool();

                // Cold start: spawn workers if needed
                if !pool.has_workers(registry_key) {
                    let per_worker_mb = m.info().est_memory_allocation_mb;
                    let current_mb = pool.total_memory_mb();
                    let total_system_mb = query_system_memory_mb();
                    let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;

                    let workers_to_spawn = if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
                        2
                    } else if current_mb + per_worker_mb <= memory_limit_mb {
                        1
                    } else {
                        // Return error stream
                        return spawn_stream(async move {
                            Err(PoolError::MemoryExhausted(format!(
                                "Cannot spawn workers for {}. Need {} MB, only {} MB available",
                                registry_key, per_worker_mb, memory_limit_mb.saturating_sub(current_mb)
                            )))
                        });
                    };

                    for _ in 0..workers_to_spawn {
                        let m_clone = m.clone();
                        if let Err(e) = pool.spawn_text_to_text_worker(
                            registry_key,
                            move || {
                                LoadedKimiK2Model::load(&m_clone)
                                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                            },
                            per_worker_mb,
                        ) {
                            return spawn_stream(async move { Err(e) });
                        }
                    }
                }

                // Route through pool
                pool.prompt(registry_key, prompt, params.clone())
            }
            Self::Qwen3Coder(m) => {
                // Same pattern with LoadedQwen3CoderModel
            }
            Self::Phi4Reasoning(m) => {
                // Same pattern with LoadedPhi4ReasoningModel
            }
        }
    }
}
```

## REQUIRED IMPORTS

Add to top of registry.rs:

```rust
use crate::pool::capabilities::text_to_text::text_to_text_pool;
```

LoadedModel types (verify these exist):
- LoadedKimiK2Model
- LoadedQwen3CoderModel
- LoadedPhi4ReasoningModel

## IMPLEMENTATION STEPS

### 1. Add Pool Import
Add `text_to_text_pool` to imports at top of registry.rs

### 2. Update KimiK2 Integration
Replace direct call with pool routing + cold start logic

### 3. Update Qwen3Coder Integration
Same pattern as KimiK2

### 4. Update Phi4Reasoning Integration
Same pattern as KimiK2

### 5. Verify LoadedModel Types Exist
Search for LoadedKimiK2Model, LoadedQwen3CoderModel, LoadedPhi4ReasoningModel definitions

### 6. Add LoadedModel Imports
Import all 3 LoadedModel types if they exist

## POOL INFRASTRUCTURE (Already Complete)

✅ `text_to_text_pool()` - Global pool accessor
✅ `pool.has_workers(registry_key)` - Check if workers exist
✅ `pool.spawn_text_to_text_worker()` - Spawn worker with model loader
✅ `pool.prompt()` - Route request to worker, returns AsyncStream
✅ `pool.total_memory_mb()` - Current memory usage
✅ Worker loop in `pool/capabilities/text_to_text.rs`
✅ Maintenance thread coordinates this pool

## ERROR HANDLING

Since `prompt()` returns AsyncStream, errors must be returned as streams:

```rust
// Memory error:
return spawn_stream(async move {
    Err(PoolError::MemoryExhausted(msg))
});

// Spawn error:
if let Err(e) = pool.spawn_text_to_text_worker(...) {
    return spawn_stream(async move { Err(e) });
}
```

## VERIFICATION

### Compile Check
```bash
cargo check -p paraphym_candle
```

### Test Cold Start
```rust
let model = registry::get::<TextToTextModel>("moonshot/kimi-k2")?;
let stream = model.prompt(prompt, &params);  // Should spawn 2 workers
```

### Verify Worker Count
After first request, check logs for "Spawned worker 1" and "Spawned worker 2"

## DEFINITION OF DONE

- [ ] `text_to_text_pool` imported in registry.rs
- [ ] KimiK2 has cold start logic and routes through pool
- [ ] Qwen3Coder has cold start logic and routes through pool
- [ ] Phi4Reasoning has cold start logic and routes through pool
- [ ] All 3 LoadedModel types imported
- [ ] `cargo check -p paraphym_candle` passes
- [ ] No unwrap() or expect() in implementation
- [ ] Error handling returns error streams correctly

## ESTIMATED TIME

15 minutes
