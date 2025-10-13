# Replace Runtime::new() with shared_runtime in clip_vision_embedding.rs:296 (HIGH)

**Location:** `src/capability/image_embedding/clip_vision_embedding.rs:296`

**Priority:** HIGH - Creates new runtime instead of using shared_runtime

## Current Code

```rust
impl LoadedClipVisionModel {
    /// Creates a persistent tokio runtime and wraps the ClipVisionEmbeddingModel
    /// for efficient reuse in worker threads.
    pub fn load(base_model: &ClipVisionEmbeddingModel)
        -> std::result::Result<Self, Box<dyn std::error::Error + Send + Sync>>
    {
        // Create persistent runtime for this worker
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        // Clone the model (Arc clone is cheap)
        let model = ClipVisionEmbeddingModel {
            provider: Arc::clone(&base_model.provider),
            dimension: base_model.dimension,
        };
        
        Ok(Self {
            model,
            runtime,
        })
    }
}
```

Context: Creating a LoadedClipVisionModel with persistent runtime for worker threads.

## Problem: Creating New Runtime

The code creates a **new tokio Runtime** with `Runtime::new()`. This:
1. Can fail with nested runtime error if called from async context
2. Creates multiple runtimes instead of reusing the shared one
3. Wastes resources - each runtime has thread pools
4. Should use the shared_runtime pattern instead

## Solution: Use shared_runtime Pattern

### Option 1: Don't Store Runtime (RECOMMENDED)

If this model is used from async contexts, don't store the runtime - just use methods directly:

```rust
// Remove LoadedClipVisionModel entirely if it's only used to hold a runtime

// Instead, use ClipVisionEmbeddingModel methods directly from async code:
impl ClipVisionEmbeddingModel {
    pub async fn embed_image_async(&self, image_path: &str) -> Result<Vec<f32>> {
        let tensor = self.provider.encode_image(image_path).await?;
        // ... convert tensor ...
    }
}
```

### Option 2: Reference shared_runtime (if runtime must be stored)

Store a reference to shared runtime instead of creating new:

```rust
impl LoadedClipVisionModel {
    pub fn load(base_model: &ClipVisionEmbeddingModel)
        -> std::result::Result<Self, Box<dyn std::error::Error + Send + Sync>>
    {
        // Use shared runtime instead of creating new
        let runtime = crate::runtime::shared_runtime()
            .ok_or_else(|| "Shared runtime unavailable")?;
        
        let model = ClipVisionEmbeddingModel {
            provider: Arc::clone(&base_model.provider),
            dimension: base_model.dimension,
        };
        
        Ok(Self {
            model,
            runtime, // Note: This now stores JoinHandle, not Runtime
        })
    }
}
```

But wait - shared_runtime() returns `&'static JoinHandle<()>`, not a Runtime. The pattern needs rethinking.

### Option 3: Make Methods Async (BEST)

The real fix is to make the calling code async instead of storing a runtime:

```rust
// Change LoadedClipVisionModel to not need a runtime at all
pub struct LoadedClipVisionModel {
    model: ClipVisionEmbeddingModel,
    // Remove runtime field
}

impl LoadedClipVisionModel {
    pub fn load(base_model: &ClipVisionEmbeddingModel) -> Self {
        Self {
            model: ClipVisionEmbeddingModel {
                provider: Arc::clone(&base_model.provider),
                dimension: base_model.dimension,
            },
        }
    }
}

impl crate::capability::traits::ImageEmbeddingCapable for LoadedClipVisionModel {
    // Change signature to be async-aware or use shared_runtime properly
    fn embed_image(&self, image_path: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> 
    {
        // Use shared_runtime().block_on() for sync trait requirement
        let runtime = crate::runtime::shared_runtime()
            .ok_or("Shared runtime unavailable")?;
        
        let provider = Arc::clone(&self.model.provider);
        let result = runtime.block_on(async {
            provider.encode_image(image_path).await
        });
        
        // ... convert result ...
    }
}
```

**Pattern Explanation:**
- **ANTIPATTERN (current):** `let runtime = Runtime::new()?;` creates new runtime
- **CORRECT (fix):** Use `shared_runtime()` or make callers async

## Implementation Notes

1. Investigate why LoadedClipVisionModel needs to own a runtime
2. If it's for sync trait requirements, use shared_runtime().block_on()
3. If it's for async code, remove runtime storage and use async methods
4. Check all call sites of LoadedClipVisionModel::load()
5. This may require changes to the ImageEmbeddingCapable trait

## Related Issues

Lines 317, 341, 365, 389 all use this runtime for block_on - those will need updates too.
