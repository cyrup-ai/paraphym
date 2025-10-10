# POOL_VISION: Integrate Vision Pool into Registry

## OBJECTIVE

Integrate the Vision pool into `capability/registry.rs` so that LLaVA routes through the pool with automatic worker spawning and memory management for vision-language tasks.

## SCOPE

**1 Model:**
- LLaVA

**2 Methods:**
- `describe_image()` - image + query → text stream
- `describe_url()` - URL + query → text stream

## CURRENT STATE (Direct Call)

**File**: `packages/candle/src/capability/registry.rs` lines ~525-538

```rust
impl VisionCapable for VisionModel {
    fn describe_image(&self, image_path: &str, query: &str)
        -> AsyncStream<CandleStringChunk> {
        match self {
            Self::LLaVA(m) => m.describe_image(image_path, query),
        }
    }

    fn describe_url(&self, url: &str, query: &str)
        -> AsyncStream<CandleStringChunk> {
        match self {
            Self::LLaVA(m) => m.describe_url(url, query),
        }
    }
}
```

## REQUIRED STATE (Pool Integration)

Add cold start logic and pool routing for both methods:

```rust
impl VisionCapable for VisionModel {
    fn describe_image(&self, image_path: &str, query: &str)
        -> AsyncStream<CandleStringChunk> {
        match self {
            Self::LLaVA(m) => {
                let registry_key = m.info().registry_key;
                let pool = vision_pool();

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
                        if let Err(e) = pool.spawn_vision_worker(
                            registry_key,
                            move || {
                                LoadedLLaVAModel::load(&m_clone)
                                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                            },
                            per_worker_mb,
                        ) {
                            return spawn_stream(async move { Err(e) });
                        }
                    }
                }

                // Route through pool
                pool.describe_image(registry_key, image_path, query)
            }
        }
    }

    fn describe_url(&self, url: &str, query: &str)
        -> AsyncStream<CandleStringChunk> {
        match self {
            Self::LLaVA(m) => {
                let registry_key = m.info().registry_key;
                let pool = vision_pool();

                // Same cold start logic as describe_image()
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
                        return spawn_stream(async move {
                            Err(PoolError::MemoryExhausted(format!(
                                "Cannot spawn workers for {}. Need {} MB, only {} MB available",
                                registry_key, per_worker_mb, memory_limit_mb.saturating_sub(current_mb)
                            )))
                        });
                    };

                    for _ in 0..workers_to_spawn {
                        let m_clone = m.clone();
                        if let Err(e) = pool.spawn_vision_worker(
                            registry_key,
                            move || {
                                LoadedLLaVAModel::load(&m_clone)
                                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                            },
                            per_worker_mb,
                        ) {
                            return spawn_stream(async move { Err(e) });
                        }
                    }
                }

                // Route through pool
                pool.describe_url(registry_key, url, query)
            }
        }
    }
}
```

## REQUIRED IMPORTS

Add to top of registry.rs:

```rust
use crate::pool::capabilities::vision::vision_pool;
```

LoadedModel type (verify exists):
- LoadedLLaVAModel

## IMPLEMENTATION STEPS

### 1. Add Pool Import
Add `vision_pool` to imports at top of registry.rs

### 2. Update describe_image()
Add cold start logic and pool routing

### 3. Update describe_url()
Copy cold start logic from describe_image(), route through pool

### 4. Verify LoadedLLaVAModel Exists
Search for LoadedLLaVAModel definition

### 5. Add LoadedModel Import
Import LoadedLLaVAModel if it exists

## POOL INFRASTRUCTURE (Already Complete)

✅ `vision_pool()` - Global pool accessor
✅ `pool.has_workers(registry_key)` - Check if workers exist
✅ `pool.spawn_vision_worker()` - Spawn worker with model loader
✅ `pool.describe_image()` - Route image+query to worker, returns AsyncStream
✅ `pool.describe_url()` - Route URL+query to worker, returns AsyncStream
✅ `pool.total_memory_mb()` - Current memory usage
✅ Worker loop in `pool/capabilities/vision.rs`
✅ Maintenance thread coordinates this pool

## ERROR HANDLING

Since methods return AsyncStream, errors must be returned as streams:

```rust
// Memory error:
return spawn_stream(async move {
    Err(PoolError::MemoryExhausted(msg))
});

// Spawn error:
if let Err(e) = pool.spawn_vision_worker(...) {
    return spawn_stream(async move { Err(e) });
}
```

## VERIFICATION

### Compile Check
```bash
cargo check -p paraphym_candle
```

### Test Both Methods
```rust
let model = registry::get<VisionModel>("llava-1.5-7b")?;

// Test describe_image (should spawn 2 workers)
let stream1 = model.describe_image("path/to/image.jpg", "What is in this image?");

// Test describe_url (workers already exist)
let stream2 = model.describe_url("https://example.com/image.jpg", "Describe this scene");
```

### Verify Worker Count
After first request, check logs for "Spawned worker 1" and "Spawned worker 2"

## DEFINITION OF DONE

- [ ] `vision_pool` imported in registry.rs
- [ ] `describe_image()` has cold start logic and routes through pool
- [ ] `describe_url()` has cold start logic and routes through pool
- [ ] LoadedLLaVAModel imported
- [ ] `cargo check -p paraphym_candle` passes
- [ ] No unwrap() or expect() in implementation
- [ ] Error handling returns error streams correctly

## ESTIMATED TIME

10 minutes (2 methods, streaming pattern)
