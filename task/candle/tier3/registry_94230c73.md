# `packages/candle/src/capability/registry.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: candle
- **File Hash**: 94230c73  
- **Timestamp**: 2025-10-10T02:15:58.141434+00:00  
- **Lines of Code**: 409

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 409 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 3 Evaluations


- Line 424
  - actual
  - 

```rust
///
/// This is the PRIMARY API for getting models from the registry.
/// Returns the actual concrete enum type (TextToTextModel, TextEmbeddingModel, etc.)
/// instead of an opaque `impl Trait`.
///
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 537
  - legacy
  - 

```rust
}

/// Get a model by provider and name (legacy compatibility)
///
/// Searches through all registered models to find one matching provider and name.
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 661
  - Fall back
  - 

```rust
    }
    
    // Fall back to static registry
    IMAGE_EMBEDDING_REGISTRY.get(key).cloned()
}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 678
  - Fall back
  - 

```rust
    }
    
    // Fall back to static registry
    TEXT_TO_IMAGE_REGISTRY.get(key).cloned()
}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Orphaned Methods


### `get_text_embedding()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs` (line 504)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// Returns an enum that implements both CandleModel and TextEmbeddingCapable.
pub fn get_text_embedding(registry_key: &str) -> Option<impl TextEmbeddingCapable + CandleModel> {
    TEXT_EMBEDDING_REGISTRY.get(registry_key).cloned()
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `get_model()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs` (line 533)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Returns the unified AnyModel enum that implements CandleModel.
/// Use this for generic model access when capability doesn't matter.
pub fn get_model(registry_key: &str) -> Option<impl CandleModel> {
    MODEL_REGISTRY.get(registry_key).cloned()
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `get_vision()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs` (line 525)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// Returns an enum that implements both CandleModel and VisionCapable.
pub fn get_vision(registry_key: &str) -> Option<impl VisionCapable + CandleModel> {
    VISION_REGISTRY.get(registry_key).cloned()
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `get_text_to_image_runtime()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs` (line 668)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// Checks runtime registry first, then falls back to static registry.
pub fn get_text_to_image_runtime(key: &str) -> Option<TextToImageModel> {
    // Check runtime registry first
    if let Some(runtime) = TEXT_TO_IMAGE_RUNTIME.get() {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `get_text_to_image()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs` (line 518)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// Returns an enum that implements both CandleModel and TextToImageCapable.
pub fn get_text_to_image(registry_key: &str) -> Option<impl TextToImageCapable + CandleModel> {
    TEXT_TO_IMAGE_REGISTRY.get(registry_key).cloned()
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `get_image_embedding_runtime()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs` (line 651)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// Checks runtime registry first, then falls back to static registry.
pub fn get_image_embedding_runtime(key: &str) -> Option<ImageEmbeddingModel> {
    // Check runtime registry first
    if let Some(runtime) = IMAGE_EMBEDDING_RUNTIME.get() {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `all_registry_keys()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs` (line 586)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// }
/// ```
pub fn all_registry_keys() -> Vec<&'static str> {
    MODEL_REGISTRY.iter().map(|(key, _model)| *key).collect()
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `count_models_by_provider()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs` (line 563)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Returns a vector of (provider_name, model_count) tuples.
/// Useful for determining default provider based on model availability.
pub fn count_models_by_provider() -> Vec<(&'static str, usize)> {
    let mut counts = std::collections::HashMap::new();
    
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `get_image_embedding()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs` (line 511)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// Returns an enum that implements both CandleModel and ImageEmbeddingCapable.
pub fn get_image_embedding(registry_key: &str) -> Option<impl ImageEmbeddingCapable + CandleModel> {
    IMAGE_EMBEDDING_REGISTRY.get(registry_key).cloned()
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `register_text_to_image()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs` (line 641)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// registry::register_text_to_image("flux-schnell", model);
/// ```
pub fn register_text_to_image(key: impl Into<String>, model: TextToImageModel) {
    let runtime = TEXT_TO_IMAGE_RUNTIME.get_or_init(|| RwLock::new(HashMap::new()));
    if let Ok(mut map) = runtime.write() {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `has_model()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs` (line 600)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// }
/// ```
pub fn has_model(registry_key: &str) -> bool {
    MODEL_REGISTRY.contains_key(registry_key)
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `get_text_to_text()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs` (line 497)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// }
/// ```
pub fn get_text_to_text(registry_key: &str) -> Option<impl TextToTextCapable + CandleModel> {
    TEXT_TO_TEXT_REGISTRY.get(registry_key).cloned()
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `register_image_embedding()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs` (line 621)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// registry::register_image_embedding("my-clip-model", model);
/// ```
pub fn register_image_embedding(key: impl Into<String>, model: ImageEmbeddingModel) {
    let runtime = IMAGE_EMBEDDING_RUNTIME.get_or_init(|| RwLock::new(HashMap::new()));
    if let Ok(mut map) = runtime.write() {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `model_count()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs` (line 685)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// Returns the count of all models in the registry across all capabilities.
pub fn model_count() -> usize {
    MODEL_REGISTRY.len()
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `get_by_provider_and_name()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs` (line 550)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// }
/// ```
pub fn get_by_provider_and_name(provider: &str, name: &str) -> Option<AnyModel> {
    MODEL_REGISTRY.iter()
        .find(|(_, model)| {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym