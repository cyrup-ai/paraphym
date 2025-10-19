//! Public API for model registry access

use std::collections::HashMap;

use super::enums::*;
use super::storage::*;
use crate::capability::traits::{
    ImageEmbeddingCapable, TextEmbeddingCapable, TextToImageCapable, TextToTextCapable,
    VisionCapable,
};
use crate::domain::model::traits::CandleModel;

/// Generic getter that returns concrete enum types
///
/// This is the PRIMARY API for getting models from the registry.
/// Returns the actual concrete enum type (TextToTextModel, TextEmbeddingModel, etc.)
/// instead of an opaque `impl Trait`.
///
/// After registry unification, this now works for BOTH static and runtime-registered models!
///
/// # Type Parameter
/// * `T` - The concrete enum type to return (TextToTextModel, TextEmbeddingModel, etc.)
///
/// # Example
/// ```rust
/// use paraphym_candle::capability::registry::{self, TextToTextModel, TextEmbeddingModel};
///
/// // Works for static models
/// let text_model: TextToTextModel = registry::get("unsloth/Kimi-K2-Instruct-GGUF").unwrap();
/// let embed_model: TextEmbeddingModel = registry::get("dunzhang/stella_en_1.5B_v5").unwrap();
///
/// // Now also works for runtime-registered models!
/// let runtime_model: TextToTextModel = registry::get("runtime-registered-key").unwrap();
/// ```
pub fn get<T>(registry_key: &str) -> Option<T>
where
    T: FromRegistry,
{
    T::from_registry(registry_key)
}

/// Trait for types that can be retrieved from the registry
pub trait FromRegistry: Sized {
    fn from_registry(registry_key: &str) -> Option<Self>;
}

impl FromRegistry for TextToTextModel {
    fn from_registry(registry_key: &str) -> Option<Self> {
        // Acquire read lock - sync, fast, no contention
        let registry = TEXT_TO_TEXT_UNIFIED.read();
        registry.get(registry_key).cloned()
    }
}

impl FromRegistry for TextEmbeddingModel {
    fn from_registry(registry_key: &str) -> Option<Self> {
        let registry = TEXT_EMBEDDING_UNIFIED.read();
        registry.get(registry_key).cloned()
    }
}

impl FromRegistry for ImageEmbeddingModel {
    fn from_registry(registry_key: &str) -> Option<Self> {
        let registry = IMAGE_EMBEDDING_UNIFIED.read();
        registry.get(registry_key).cloned()
    }
}

impl FromRegistry for TextToImageModel {
    fn from_registry(registry_key: &str) -> Option<Self> {
        let registry = TEXT_TO_IMAGE_UNIFIED.read();
        registry.get(registry_key).cloned()
    }
}

impl FromRegistry for VisionModel {
    fn from_registry(registry_key: &str) -> Option<Self> {
        let registry = VISION_UNIFIED.read();
        registry.get(registry_key).cloned()
    }
}

impl FromRegistry for AnyModel {
    fn from_registry(registry_key: &str) -> Option<Self> {
        // Lazy view pattern: Check each capability registry in order
        // This avoids maintaining a separate synchronized MODEL_UNIFIED registry
        
        if let Some(model) = TEXT_TO_TEXT_UNIFIED.read().get(registry_key) {
            return Some(AnyModel::TextToText(model.clone()));
        }
        
        if let Some(model) = TEXT_EMBEDDING_UNIFIED.read().get(registry_key) {
            return Some(AnyModel::TextEmbedding(model.clone()));
        }
        
        if let Some(model) = IMAGE_EMBEDDING_UNIFIED.read().get(registry_key) {
            return Some(AnyModel::ImageEmbedding(model.clone()));
        }
        
        if let Some(model) = TEXT_TO_IMAGE_UNIFIED.read().get(registry_key) {
            return Some(AnyModel::TextToImage(model.clone()));
        }
        
        if let Some(model) = VISION_UNIFIED.read().get(registry_key) {
            return Some(AnyModel::Vision(model.clone()));
        }
        
        None
    }
}

/// Get a text-to-text model by registry_key
///
/// Returns an enum that implements both CandleModel and TextToTextCapable.
/// After unification, this now works for both static and runtime-registered models.
///
/// # Example
/// ```rust
/// use paraphym_candle::capability::registry;
///
/// if let Some(model) = registry::get_text_to_text("unsloth/Kimi-K2-Instruct-GGUF") {
///     // model implements TextToTextCapable
/// }
/// ```
pub fn get_text_to_text(registry_key: &str) -> Option<impl TextToTextCapable> {
    TEXT_TO_TEXT_UNIFIED.read().get(registry_key).cloned()
}

/// Get a text embedding model by registry_key
///
/// Returns an enum that implements both CandleModel and TextEmbeddingCapable.
pub fn get_text_embedding(registry_key: &str) -> Option<impl TextEmbeddingCapable> {
    TEXT_EMBEDDING_UNIFIED.read().get(registry_key).cloned()
}

/// Get an image embedding model by registry_key
///
/// Returns an enum that implements both CandleModel and ImageEmbeddingCapable.
pub fn get_image_embedding(registry_key: &str) -> Option<impl ImageEmbeddingCapable> {
    IMAGE_EMBEDDING_UNIFIED.read().get(registry_key).cloned()
}

/// Get a text-to-image model by registry_key
///
/// Returns an enum that implements both CandleModel and TextToImageCapable.
pub fn get_text_to_image(registry_key: &str) -> Option<impl TextToImageCapable> {
    TEXT_TO_IMAGE_UNIFIED.read().get(registry_key).cloned()
}

/// Get a vision model by registry_key
///
/// Returns an enum that implements both CandleModel and VisionCapable.
pub fn get_vision(registry_key: &str) -> Option<impl VisionCapable> {
    VISION_UNIFIED.read().get(registry_key).cloned()
}

/// Get any model by registry_key
///
/// Returns the unified AnyModel enum that implements CandleModel.
/// Use this for generic model access when capability doesn't matter.
pub fn get_model(registry_key: &str) -> Option<impl CandleModel> {
    // Use FromRegistry implementation which does lazy aggregation
    AnyModel::from_registry(registry_key)
}

/// Get a model by provider and name (legacy compatibility)
///
/// Searches through all registered models to find one matching provider and name.
/// This maintains compatibility with the old registry API.
///
/// # Example
/// ```rust
/// use paraphym_candle::capability::registry;
///
/// if let Some(model) = registry::get_by_provider_and_name("candle-kimi", "kimi-k2-instruct") {
///     // model is AnyModel enum
/// }
/// ```
pub fn get_by_provider_and_name(provider: &str, name: &str) -> Option<AnyModel> {
    // Check each unified registry for matching provider and name
    
    // Text-to-text models
    for model in TEXT_TO_TEXT_UNIFIED.read().values() {
        let info = model.info();
        if info.provider_str() == provider && info.name() == name {
            return Some(AnyModel::TextToText(model.clone()));
        }
    }
    
    // Text embedding models
    for model in TEXT_EMBEDDING_UNIFIED.read().values() {
        let info = model.info();
        if info.provider_str() == provider && info.name() == name {
            return Some(AnyModel::TextEmbedding(model.clone()));
        }
    }
    
    // Image embedding models
    for model in IMAGE_EMBEDDING_UNIFIED.read().values() {
        let info = model.info();
        if info.provider_str() == provider && info.name() == name {
            return Some(AnyModel::ImageEmbedding(model.clone()));
        }
    }
    
    // Text-to-image models
    for model in TEXT_TO_IMAGE_UNIFIED.read().values() {
        let info = model.info();
        if info.provider_str() == provider && info.name() == name {
            return Some(AnyModel::TextToImage(model.clone()));
        }
    }
    
    // Vision models
    for model in VISION_UNIFIED.read().values() {
        let info = model.info();
        if info.provider_str() == provider && info.name() == name {
            return Some(AnyModel::Vision(model.clone()));
        }
    }
    
    None
}

/// Count models by provider
///
/// Returns a vector of (provider_name, model_count) tuples.
/// Useful for determining default provider based on model availability.
///
/// Note: This function dynamically aggregates counts from all unified registries,
/// including runtime-registered models.
pub fn count_models_by_provider() -> Vec<(&'static str, usize)> {
    let mut counts = HashMap::new();
    
    // Aggregate from all unified registries
    for model in TEXT_TO_TEXT_UNIFIED.read().values() {
        let provider = model.info().provider_str();
        *counts.entry(provider).or_insert(0) += 1;
    }
    
    for model in TEXT_EMBEDDING_UNIFIED.read().values() {
        let provider = model.info().provider_str();
        *counts.entry(provider).or_insert(0) += 1;
    }
    
    for model in IMAGE_EMBEDDING_UNIFIED.read().values() {
        let provider = model.info().provider_str();
        *counts.entry(provider).or_insert(0) += 1;
    }
    
    for model in TEXT_TO_IMAGE_UNIFIED.read().values() {
        let provider = model.info().provider_str();
        *counts.entry(provider).or_insert(0) += 1;
    }
    
    for model in VISION_UNIFIED.read().values() {
        let provider = model.info().provider_str();
        *counts.entry(provider).or_insert(0) += 1;
    }
    
    counts.into_iter().collect()
}

/// Get all registered model registry keys
///
/// Returns a vector of all `registry_key` values for models in the registry.
/// Dynamically aggregates from all unified registries, including runtime-registered models.
///
/// Keys are deduplicated using a HashSet to ensure each registry_key appears exactly once,
/// even if a key were to exist in multiple capability registries (which registration
/// validation now prevents).
///
/// # Example
/// ```rust
/// use paraphym_candle::capability::registry;
///
/// for key in registry::all_registry_keys() {
///     println!("Registered: {}", key);
/// }
/// ```
pub fn all_registry_keys() -> Vec<String> {
    use std::collections::HashSet;
    
    let mut keys = HashSet::new();
    
    // Aggregate from all unified registries with automatic deduplication
    keys.extend(TEXT_TO_TEXT_UNIFIED.read().keys().cloned());
    keys.extend(TEXT_EMBEDDING_UNIFIED.read().keys().cloned());
    keys.extend(IMAGE_EMBEDDING_UNIFIED.read().keys().cloned());
    keys.extend(TEXT_TO_IMAGE_UNIFIED.read().keys().cloned());
    keys.extend(VISION_UNIFIED.read().keys().cloned());
    
    keys.into_iter().collect()
}

/// Check if a registry_key is registered
///
/// Checks all unified registries, including runtime-registered models.
///
/// # Performance
///
/// Uses short-circuit evaluation with the `||` operator - if the key is found
/// in an early registry (e.g., TEXT_TO_TEXT_UNIFIED), the remaining registry
/// checks are skipped entirely. This makes lookups efficient even when checking
/// across all 5 capability registries.
///
/// # Example
/// ```rust
/// use paraphym_candle::capability::registry;
///
/// if registry::has_model("unsloth/Kimi-K2-Instruct-GGUF") {
///     println!("Kimi K2 is available");
/// }
/// ```
pub fn has_model(registry_key: &str) -> bool {
    // Short-circuit evaluation: stops at first match
    TEXT_TO_TEXT_UNIFIED.read().contains_key(registry_key)
        || TEXT_EMBEDDING_UNIFIED.read().contains_key(registry_key)
        || IMAGE_EMBEDDING_UNIFIED.read().contains_key(registry_key)
        || TEXT_TO_IMAGE_UNIFIED.read().contains_key(registry_key)
        || VISION_UNIFIED.read().contains_key(registry_key)
}

/// Get the total number of registered models
///
/// Returns the count of all models in the registry across all capabilities,
/// including runtime-registered models.
pub fn model_count() -> usize {
    TEXT_TO_TEXT_UNIFIED.read().len()
        + TEXT_EMBEDDING_UNIFIED.read().len()
        + IMAGE_EMBEDDING_UNIFIED.read().len()
        + TEXT_TO_IMAGE_UNIFIED.read().len()
        + VISION_UNIFIED.read().len()
}
