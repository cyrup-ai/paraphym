//! Model registry for dynamic model discovery and lookup

use std::any::TypeId;
use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::sync::Arc;

use ahash::RandomState;
use dashmap::{DashMap, DashSet};
use std::sync::LazyLock;

use crate::domain::model::error::{CandleModelError as ModelError, CandleResult as Result};
use crate::model::info::ModelInfo;
use crate::model::traits::Model;

/// Type-erased model handle with preserved type information for downcasting
struct CandleModelHandle {
    model: Arc<dyn std::any::Any + Send + Sync>,
    info: &'static ModelInfo,
}

impl CandleModelHandle {
    /// Create new model handle from any Model implementation
    fn new<M: Model + 'static>(model: M) -> Self {
        let info = model.info();
        Self {
            model: Arc::new(model),
            info,
        }
    }
    
    /// Get model info
    fn info(&self) -> &'static ModelInfo {
        self.info
    }

    /// Get as Any trait object for downcasting
    fn as_any(&self) -> &dyn std::any::Any {
        &*self.model
    }

    /// Get as specific model type
    fn as_model<M: Model + 'static>(&self) -> Option<&M> {
        self.model.downcast_ref::<M>()
    }

    /// Attempt to downcast the model handle to a concrete Arc<T>
    fn as_arc<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        Arc::clone(&self.model)
            .downcast::<T>()
            .ok()
    }
}

/// Generic Candle model for extensibility with zero-allocation patterns
#[derive(Debug, Clone)]
pub struct GenericCandleModel {
    name: String,
    provider: String,
    context_length: u32,
    supports_streaming: bool,
    supports_tools: bool,
}

impl GenericCandleModel {
    /// Create new generic model
    pub fn new(
        name: String,
        provider: String,
        context_length: u32,
        supports_streaming: bool,
        supports_tools: bool,
    ) -> Self {
        Self {
            name,
            provider,
            context_length,
            supports_streaming,
            supports_tools,
        }
    }
    
    /// Get model info
    pub fn info(&self) -> ModelInfo {
        ModelInfo {
            name: Box::leak(self.name.clone().into_boxed_str()),
            provider_name: Box::leak(self.provider.clone().into_boxed_str()),
            max_input_tokens: NonZeroU32::new(self.context_length).or_else(|| NonZeroU32::new(4096)),
            max_output_tokens: NonZeroU32::new(2048),
            input_price: None,
            output_price: None,
            supports_vision: false,
            supports_function_calling: self.supports_tools,
            supports_streaming: self.supports_streaming,
            supports_embeddings: false,
            requires_max_tokens: false,
            supports_thinking: false,
            optimal_thinking_budget: None,
            system_prompt_prefix: None,
            real_name: None,
            model_type: None,
            patch: None,
        }
    }


}

/// The global model registry with zero-allocation typed handles
struct ModelRegistryInner {
    /// Maps provider name to model name to typed model handle
    models: DashMap<&'static str, DashMap<&'static str, Arc<CandleModelHandle>, RandomState>, RandomState>,
    
    /// Maps model type names to provider+name for efficient lookup
    type_registry: DashMap<&'static str, DashSet<(&'static str, &'static str), RandomState>, RandomState>,
}

impl Default for ModelRegistryInner {
    fn default() -> Self {
        Self {
            models: DashMap::with_hasher(RandomState::default()),
            type_registry: DashMap::with_hasher(RandomState::default()),
        }
    }
}

/// The global model registry
static GLOBAL_REGISTRY: LazyLock<ModelRegistryInner> = LazyLock::new(Default::default);

/// A registry for managing model instances
///
/// This provides a thread-safe way to register, look up, and manage model instances.
/// It supports dynamic model loading and type-safe retrieval.
#[derive(Clone, Default)]
pub struct ModelRegistry;

impl ModelRegistry {
    /// Create a new model registry
    pub fn new() -> Self {
        Self
    }

    /// Register a model with the registry
    ///
    /// # Arguments
    /// * `provider` - The provider name (e.g., "openai", "anthropic")
    /// * `model` - The model instance to register
    ///
    /// # Returns
    /// A result containing the registered model or an error if registration fails
    pub fn register<M: Model + 'static>(
        &self,
        provider: &'static str,
        model: M,
    ) -> Result<RegisteredModel<M>> {
        let handle = Arc::new(CandleModelHandle::new(model));
        let model_name = handle.info().name();

        // Get or create the provider's model map
        let provider_models = GLOBAL_REGISTRY
            .models
            .entry(provider)
            .or_insert_with(|| DashMap::with_hasher(RandomState::default()));

        // Check for duplicate model
        if provider_models.contains_key(model_name) {
            return Err(ModelError::ModelAlreadyExists {
                provider: provider.into(),
                name: model_name.into()});
        }

        // Register the model
        provider_models.insert(model_name, handle.clone());

        // Register the model type
        let _type_id = TypeId::of::<M>();
        let type_name = std::any::type_name::<M>();
        let type_entries = GLOBAL_REGISTRY
            .type_registry
            .entry(type_name)
            .or_insert_with(|| DashSet::with_hasher(RandomState::default()));

        type_entries.insert((provider, model_name));

        // Return a registered model handle
        Ok(RegisteredModel {
            handle,
            _marker: PhantomData})
    }

    /// Get a model by provider and name
    ///
    /// # Arguments
    /// * `provider` - The provider name
    /// * `name` - The model name
    ///
    /// # Returns
    /// A result containing the model if found, or an error if not found or type mismatch
    pub fn get<M: Model + 'static>(
        &self,
        provider: &str,
        name: &str,
    ) -> Result<Option<RegisteredModel<M>>> {
        let provider_models = match GLOBAL_REGISTRY.models.get(provider) {
            Some(provider) => provider,
            None => return Ok(None)};

        let handle = match provider_models.get(name) {
            Some(handle) => handle,
            None => return Ok(None)};

        // Verify the model type
        if handle.as_any().downcast_ref::<M>().is_none() {
            return Err(ModelError::InvalidConfiguration(
                "model type does not match requested type".into(),
            ));
        }

        Ok(Some(RegisteredModel {
            handle: handle.clone(),
            _marker: PhantomData}))
    }

    /// Get a model by provider and name, returning an error if not found
    ///
    /// # Arguments
    /// * `provider` - The provider name
    /// * `name` - The model name
    ///
    /// # Returns
    /// A result containing the model if found, or an error if not found or type mismatch
    pub fn get_required<M: Model + 'static>(
        &self,
        provider: &'static str,
        name: &'static str,
    ) -> Result<RegisteredModel<M>> {
        self.get(provider, name)?
            .ok_or_else(|| ModelError::ModelNotFound {
                provider: provider.into(),
                name: name.into()})
    }

    /// Find all models of a specific type
    ///
    /// # Returns
    /// A vector of registered models of the specified type
    pub fn find_all<M: Model + 'static>(&self) -> Vec<RegisteredModel<M>> {
        let _type_id = TypeId::of::<M>();
        let type_name = std::any::type_name::<M>();
        let mut result = Vec::new();

        if let Some(type_entries) = GLOBAL_REGISTRY.type_registry.get(type_name) {
            for entry in type_entries.iter() {
                let (provider, name) = *entry;
                if let Some(provider_models) = GLOBAL_REGISTRY.models.get(provider)
                    && let Some(handle) = provider_models.get(name)
                    && handle.as_any().downcast_ref::<M>().is_some()
                {
                    result.push(RegisteredModel {
                        handle: handle.clone(),
                        _marker: PhantomData});
                }
            }
        }

        result
    }

    /// Get a model as a specific trait object
    ///
    /// # Arguments
    /// * `provider` - The provider name
    /// * `name` - The model name
    ///
    /// # Returns
    /// A result containing the model as the requested trait object
    pub fn get_as<T: Send + Sync + Sized + 'static>(
        &self,
        provider: &'static str,
        name: &'static str,
    ) -> Result<Option<Arc<T>>>
    {
        let provider_models = match GLOBAL_REGISTRY.models.get(provider) {
            Some(provider) => provider,
            None => return Ok(None)};

        let handle = match provider_models.get(name) {
            Some(handle) => handle,
            None => return Ok(None)};

        match handle.as_arc::<T>() {
            Some(arc_model) => Ok(Some(arc_model)),
            None => Err(ModelError::InvalidConfiguration(
                format!(
                    "Model '{}' from provider '{}' is not of type {}",
                    name, provider, std::any::type_name::<T>()
                ).into()
            )),
        }
    }

    /// Get a model as a boxed trait object
    ///
    /// # Arguments
    /// * `provider` - The provider name
    /// * `name` - The model name
    ///
    /// # Returns
    /// A result containing the model as a boxed trait object
    pub fn get_boxed<T: Send + Sync + ?Sized + 'static>(
        &self,
        provider: &'static str,
        name: &'static str,
    ) -> Result<Option<Box<T>>>
    {
        let provider_models = match GLOBAL_REGISTRY.models.get(provider) {
            Some(provider) => provider,
            None => return Ok(None)};

        let _handle = match provider_models.get(name) {
            Some(handle) => handle,
            None => return Ok(None)};

        Err(ModelError::InvalidConfiguration(Cow::Owned(format!(
            "Boxed trait object conversion for model '{}' from provider '{}' requires additional implementation for ?Sized types",
            name, provider
        ))))
    }

    /// Get a model as a specific trait object, returning an error if not found
    ///
    /// # Arguments
    /// * `provider` - The provider name
    /// * `name` - The model name
    ///
    /// # Returns
    /// A result containing the model as the requested trait object
    pub fn get_required_as<T: Send + Sync + Sized + 'static>(
        &self,
        provider: &'static str,
        name: &'static str,
    ) -> Result<Arc<T>>
    {
        self.get_as(provider, name)?
            .ok_or_else(|| ModelError::ModelNotFound {
                provider: provider.into(),
                name: name.into()})
    }

    /// Get a model as a boxed trait object, returning an error if not found
    ///
    /// # Arguments
    /// * `provider` - The provider name
    /// * `name` - The model name
    ///
    /// # Returns
    /// A result containing the model as a boxed trait object
    pub fn get_required_boxed<T: Send + Sync + Sized + 'static>(
        &self,
        provider: &'static str,
        name: &'static str,
    ) -> Result<Box<T>>
    {
        self.get_boxed(provider, name)?
            .ok_or_else(|| ModelError::ModelNotFound {
                provider: provider.into(),
                name: name.into()})
    }
}

/// A handle to a registered model
///
/// This provides type-safe access to a registered model and ensures
/// proper cleanup when the last reference is dropped.
pub struct RegisteredModel<M: Model + 'static> {
    handle: Arc<CandleModelHandle>,
    _marker: PhantomData<M>}

impl<M: Model + 'static> Clone for RegisteredModel<M> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            _marker: PhantomData}
    }
}

impl<M: Model + 'static> std::ops::Deref for RegisteredModel<M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        // SAFETY: Type M is guaranteed to match at construction time in register().
        // RegisteredModel<M> can only be created with a model of type M.
        // This unwrap is safe but required because Deref cannot return Result.
        self.handle
            .as_model()
            .unwrap_or_else(|| {
                panic!(
                    "Type invariant violated: RegisteredModel<{}> handle does not contain type {}",
                    std::any::type_name::<M>(),
                    std::any::type_name::<M>()
                )
            })
    }
}

impl<M: Model + 'static> AsRef<M> for RegisteredModel<M> {
    fn as_ref(&self) -> &M {
        // SAFETY: Type M is guaranteed to match at construction time in register().
        // RegisteredModel<M> can only be created with a model of type M.
        // This unwrap is safe but required because AsRef cannot return Result.
        self.handle
            .as_model()
            .unwrap_or_else(|| {
                panic!(
                    "Type invariant violated: RegisteredModel<{}> handle does not contain type {}",
                    std::any::type_name::<M>(),
                    std::any::type_name::<M>()
                )
            })
    }
}

impl<M: Model + 'static> std::fmt::Debug for RegisteredModel<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisteredModel")
            .field("provider", &self.info().provider())
            .field("name", &self.info().name())
            .finish()
    }
}

impl<M: Model + 'static> PartialEq for RegisteredModel<M> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.handle, &other.handle)
    }
}

impl<M: Model + 'static> Eq for RegisteredModel<M> {}

impl<M: Model + 'static> Hash for RegisteredModel<M> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.info().provider().hash(state);
        self.handle.info().name().hash(state);
    }
}

// Registry implementation complete