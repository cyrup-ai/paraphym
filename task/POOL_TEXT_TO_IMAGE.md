# POOL_TEXT_TO_IMAGE: Integrate TextToImage Pool into Registry

## OBJECTIVE

Integrate the TextToImage pool into `capability/registry.rs` so that image generation models (FluxSchnell, StableDiffusion35Turbo) route through the pool with automatic worker spawning and memory management.

## SCOPE

**2 Models:**
- FluxSchnell
- StableDiffusion35Turbo

**1 Method:**
- `generate_image()` - prompt + config → image stream

## CURRENT STATE (Direct Call)

**File**: `packages/candle/src/capability/registry.rs` lines ~540-560

```rust
impl TextToImageCapable for TextToImageModel {
    fn generate_image(
        &self,
        prompt: &str,
        config: &ImageGenerationConfig,
        device: &Device,
    ) -> AsyncStream<ImageGenerationChunk> {
        match self {
            Self::FluxSchnell(m) => m.generate_image(prompt, config, device),
            Self::StableDiffusion35Turbo(m) => m.generate_image(prompt, config, device),
        }
    }

    fn registry_key(&self) -> &str {
        match self {
            Self::FluxSchnell(m) => m.registry_key(),
            Self::StableDiffusion35Turbo(m) => m.registry_key(),
        }
    }
}
```

## REQUIRED STATE (Pool Integration)

Add cold start logic and pool routing for both models:

```rust
impl TextToImageCapable for TextToImageModel {
    fn generate_image(
        &self,
        prompt: &str,
        config: &ImageGenerationConfig,
        device: &Device,
    ) -> AsyncStream<ImageGenerationChunk> {
        match self {
            Self::FluxSchnell(m) => {
                let registry_key = m.info().registry_key;
                let pool = text_to_image_pool();

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
                        if let Err(e) = pool.spawn_text_to_image_worker(
                            registry_key,
                            move || {
                                LoadedFluxSchnellModel::load(&m_clone)
                                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                            },
                            per_worker_mb,
                        ) {
                            return spawn_stream(async move { Err(e) });
                        }
                    }
                }

                // Route through pool
                pool.generate_image(registry_key, prompt, config.clone(), device.clone())
            }
            Self::StableDiffusion35Turbo(m) => {
                let registry_key = m.info().registry_key;
                let pool = text_to_image_pool();

                // Same cold start logic
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
                        if let Err(e) = pool.spawn_text_to_image_worker(
                            registry_key,
                            move || {
                                LoadedStableDiffusion35TurboModel::load(&m_clone)
                                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                            },
                            per_worker_mb,
                        ) {
                            return spawn_stream(async move { Err(e) });
                        }
                    }
                }

                // Route through pool
                pool.generate_image(registry_key, prompt, config.clone(), device.clone())
            }
        }
    }

    fn registry_key(&self) -> &str {
        // No pool routing needed - this is just metadata
        match self {
            Self::FluxSchnell(m) => m.registry_key(),
            Self::StableDiffusion35Turbo(m) => m.registry_key(),
        }
    }
}
```

## REQUIRED IMPORTS

Add to top of registry.rs:

```rust
use crate::pool::capabilities::text_to_image::text_to_image_pool;
```

LoadedModel types (verify these exist):
- LoadedFluxSchnellModel
- LoadedStableDiffusion35TurboModel

## IMPLEMENTATION STEPS

### 1. Add Pool Import
Add `text_to_image_pool` to imports at top of registry.rs

### 2. Update FluxSchnell Integration
Add cold start logic and pool routing

### 3. Update StableDiffusion35Turbo Integration
Same pattern as FluxSchnell

### 4. Leave registry_key() Unchanged
This is metadata-only, no pool routing needed

### 5. Verify LoadedModel Types Exist
Search for LoadedFluxSchnellModel and LoadedStableDiffusion35TurboModel definitions

### 6. Add LoadedModel Imports
Import both LoadedModel types if they exist

## POOL INFRASTRUCTURE (Already Complete)

✅ `text_to_image_pool()` - Global pool accessor
✅ `pool.has_workers(registry_key)` - Check if workers exist
✅ `pool.spawn_text_to_image_worker()` - Spawn worker with model loader
✅ `pool.generate_image()` - Route prompt+config+device to worker, returns AsyncStream
✅ `pool.total_memory_mb()` - Current memory usage
✅ Worker loop in `pool/capabilities/text_to_image.rs`
✅ Maintenance thread coordinates this pool

## ERROR HANDLING

Since `generate_image()` returns AsyncStream, errors must be returned as streams:

```rust
// Memory error:
return spawn_stream(async move {
    Err(PoolError::MemoryExhausted(msg))
});

// Spawn error:
if let Err(e) = pool.spawn_text_to_image_worker(...) {
    return spawn_stream(async move { Err(e) });
}
```

## IMPORTANT NOTES

### Device Parameter
The `generate_image()` method takes `&Device` parameter. You may need to `.clone()` this when passing to the pool if Device implements Clone. Check the pool method signature.

### Config Cloning
`ImageGenerationConfig` will likely need to be cloned when passed to pool. Verify it implements Clone.

## VERIFICATION

### Compile Check
```bash
cargo check -p paraphym_candle
```

### Test Both Models
```rust
let model = registry::get_text_to_image_runtime("flux-schnell")?;
let config = ImageGenerationConfig::default();
let device = Device::cuda_if_available(0)?;

// Test FluxSchnell (should spawn 2 workers)
let stream = model.generate_image("A beautiful sunset", &config, &device);

// Test StableDiffusion35Turbo
let model2 = registry::get_text_to_image_runtime("sd3.5-turbo")?;
let stream2 = model2.generate_image("A cat on a skateboard", &config, &device);
```

### Verify Worker Count
After first request, check logs for "Spawned worker 1" and "Spawned worker 2"

## DEFINITION OF DONE

- [ ] `text_to_image_pool` imported in registry.rs
- [ ] FluxSchnell has cold start logic and routes through pool
- [ ] StableDiffusion35Turbo has cold start logic and routes through pool
- [ ] `registry_key()` unchanged (metadata only)
- [ ] Both LoadedModel types imported
- [ ] `cargo check -p paraphym_candle` passes
- [ ] No unwrap() or expect() in implementation
- [ ] Error handling returns error streams correctly

## ESTIMATED TIME

15 minutes (2 models, streaming pattern)
