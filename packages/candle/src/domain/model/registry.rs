//! Model registry for dynamic model discovery and lookup

use std::any::{Any, TypeId};
use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::sync::Arc;

use ahash::RandomState;
use dashmap::{DashMap, DashSet};
use std::sync::LazyLock;

use crate::domain::model::error::{CandleModelError, CandleResult};
use crate::domain::model::traits::CandleModel;
use crate::domain::model::CandleModelInfo;

/// A type-erased model reference
struct CandleModelHandle {
    model: Arc<dyn Any + Send + Sync>,
    info: &'static CandleModelInfo,
    type_name: &'static str,
}

impl CandleModelHandle {
    fn new<M: CandleModel + 'static>(model: M) -> Self {
        let info = model.info();
        Self {
            model: Arc::new(model),
            info,
            type_name: std::any::type_name::<M>(),
        }
    }

    fn as_any(&self) -> &dyn Any {
        &*self.model
    }

    fn as_model<M: CandleModel + 'static>(&self) -> Option<&M> {
        self.model.downcast_ref::<M>()
    }

    /// Attempt to downcast the model handle to a concrete Arc<T>
    fn as_arc<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        // Clone the Arc and attempt downcast
        Arc::clone(&self.model)
            .downcast::<T>()
            .ok()
    }

    fn info(&self) -> &'static CandleModelInfo {
        self.info
    }

    fn type_name(&self) -> &'static str {
        self.type_name
    }
}

/// The global model registry
struct CandleModelRegistryInner {
    // Maps provider name to model name to model handle
    models: DashMap<
        &'static str,
        DashMap<&'static str, Arc<CandleModelHandle>, RandomState>,
        RandomState,
    >,

    // Maps model type to provider+name
    type_registry: DashMap<TypeId, DashSet<(&'static str, &'static str), RandomState>, RandomState>,
}

impl Default for CandleModelRegistryInner {
    fn default() -> Self {
        Self {
            models: DashMap::with_hasher(RandomState::default()),
            type_registry: DashMap::with_hasher(RandomState::default()),
        }
    }
}

/// The global model registry
static GLOBAL_REGISTRY: LazyLock<CandleModelRegistryInner> = LazyLock::new(Default::default);

/// A registry for managing model instances
///
/// This provides a thread-safe way to register, look up, and manage model instances.
/// It supports dynamic model loading and type-safe retrieval.
#[derive(Clone, Default)]
pub struct CandleModelRegistry;

impl CandleModelRegistry {
    /// Create a new model registry
    pub fn new() -> Self {
        Self
    }

    /// Register a model with the registry
    ///
    /// # Arguments
    /// * `provider` - The provider name (e.g., "candle-kimi", "candle-qwen")
    /// * `model` - The model instance to register
    ///
    /// # Returns
    /// A result containing the registered model or an error if registration fails
    ///
    /// # Errors
    ///
    /// Returns `CandleModelError::ModelAlreadyRegistered` if model already exists
    pub fn register<M: CandleModel + 'static>(
        &self,
        provider: &'static str,
        model: M,
    ) -> CandleResult<RegisteredModel<M>> {
        let handle = Arc::new(CandleModelHandle::new(model));
        let model_name = handle.info().name();

        // Get or create the provider's model map
        let provider_models = GLOBAL_REGISTRY
            .models
            .entry(provider)
            .or_insert_with(|| DashMap::with_hasher(RandomState::default()));

        // Check for duplicate model
        if provider_models.contains_key(model_name) {
            return Err(CandleModelError::ModelAlreadyExists {
                provider: provider.into(),
                name: model_name.into(),
            });
        }

        // Register the model
        provider_models.insert(model_name, handle.clone());

        // Register the model type
        let type_id = TypeId::of::<M>();
        let type_entries = GLOBAL_REGISTRY
            .type_registry
            .entry(type_id)
            .or_insert_with(|| DashSet::with_hasher(RandomState::default()));

        type_entries.insert((provider, model_name));

        // Return a registered model handle
        Ok(RegisteredModel {
            handle,
            _marker: PhantomData,
        })
    }

    /// Count the number of registered models per provider
    /// 
    /// Returns a vector of (`provider_name`, `model_count`) tuples.
    /// Used by `ModelResolver` for usage-based default provider selection.
    /// 
    /// # Returns
    /// 
    /// A vector where each element is a tuple of (provider name, number of models)
    pub fn count_models_by_provider(&self) -> Vec<(&'static str, usize)> {
        GLOBAL_REGISTRY
            .models
            .iter()
            .map(|entry| (*entry.key(), entry.value().len()))
            .collect()
    }

    /// Get a model by provider and name
    ///
    /// # Arguments
    /// * `provider` - The provider name
    /// * `name` - The model name
    ///
    /// # Returns
    /// A result containing the model if found, or an error if not found or type mismatch
    ///
    /// # Errors
    ///
    /// Returns `CandleModelError::TypeMismatch` if model type doesn't match
    pub fn get<M: CandleModel + 'static>(
        &self,
        provider: &str,
        name: &str,
    ) -> CandleResult<Option<RegisteredModel<M>>> {
        let Some(provider_models) = GLOBAL_REGISTRY.models.get(provider) else {
            return Ok(None);
        };

        let Some(handle) = provider_models.get(name) else {
            return Ok(None);
        };

        // Verify the model type
        if handle.as_any().downcast_ref::<M>().is_none() {
            return Err(CandleModelError::InvalidConfiguration(
                "model type does not match requested type".into(),
            ));
        }

        Ok(Some(RegisteredModel {
            handle: handle.clone(),
            _marker: PhantomData,
        }))
    }

    /// Get a model by provider and name, returning an error if not found
    ///
    /// # Arguments
    /// * `provider` - The provider name
    /// * `name` - The model name
    ///
    /// # Returns
    /// A result containing the model if found, or an error if not found or type mismatch
    ///
    /// # Errors
    ///
    /// Returns `CandleModelError::ModelNotFound` if model doesn't exist or `CandleModelError::TypeMismatch` if type doesn't match
    pub fn get_required<M: CandleModel + 'static>(
        &self,
        provider: &'static str,
        name: &'static str,
    ) -> CandleResult<RegisteredModel<M>> {
        self.get(provider, name)?
            .ok_or_else(|| CandleModelError::ModelNotFound {
                provider: provider.into(),
                name: name.into(),
            })
    }

    /// Find all models of a specific type
    ///
    /// # Returns
    /// A vector of registered models of the specified type
    pub fn find_all<M: CandleModel + 'static>(&self) -> Vec<RegisteredModel<M>> {
        let type_id = TypeId::of::<M>();
        let mut result = Vec::new();

        if let Some(type_entries) = GLOBAL_REGISTRY.type_registry.get(&type_id) {
            for entry in type_entries.iter() {
                let (provider, name) = *entry;
                if let Some(provider_models) = GLOBAL_REGISTRY.models.get(provider)
                    && let Some(handle) = provider_models.get(name)
                    && handle.as_any().downcast_ref::<M>().is_some()
                {
                    result.push(RegisteredModel {
                        handle: handle.clone(),
                        _marker: PhantomData,
                    });
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
    ///
    /// # Errors
    ///
    /// Returns `CandleModelError::TypeMismatch` if model cannot be cast to requested type
    pub fn get_as<T>(
        &self,
        provider: &'static str,
        name: &'static str,
    ) -> CandleResult<Option<Arc<T>>>
    where
        T: Send + Sync + Sized + 'static,
    {
        let Some(provider_models) = GLOBAL_REGISTRY.models.get(provider) else {
            return Ok(None);
        };

        let Some(handle) = provider_models.get(name) else {
            return Ok(None);
        };

        // Use the Arc downcasting helper
        match handle.as_arc::<T>() {
            Some(arc_model) => Ok(Some(arc_model)),
            None => Err(CandleModelError::InvalidConfiguration(
                format!(
                    "Model '{name}' from provider '{provider}' is not of type {}",
                    std::any::type_name::<T>()
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
    ///
    /// # Errors
    ///
    /// Returns `CandleModelError::TypeMismatch` if model cannot be cast to requested type
    pub fn get_boxed<T>(
        &self,
        provider: &'static str,
        name: &'static str,
    ) -> CandleResult<Option<Box<T>>>
    where
        T: Send + Sync + 'static + ?Sized,
    {
        let Some(provider_models) = GLOBAL_REGISTRY.models.get(provider) else {
            return Ok(None);
        };

        let Some(_handle) = provider_models.get(name) else {
            return Ok(None);
        };

        // Attempt to convert the handle to a boxed trait object
        // This is complex for ?Sized types and requires careful implementation
        Err(CandleModelError::InvalidConfiguration(Cow::Owned(format!(
            "Boxed trait object conversion for model '{name}' from provider '{provider}' requires additional implementation for ?Sized types"
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
    ///
    /// # Errors
    ///
    /// Returns `CandleModelError::ModelNotFound` if model doesn't exist or `CandleModelError::TypeMismatch` if type doesn't match
    pub fn get_required_as<T>(
        &self,
        provider: &'static str,
        name: &'static str,
    ) -> CandleResult<Arc<T>>
    where
        T: Send + Sync + Sized + 'static,
    {
        self.get_as(provider, name)?
            .ok_or_else(|| CandleModelError::ModelNotFound {
                provider: provider.into(),
                name: name.into(),
            })
    }

    /// Get a model as a boxed trait object, returning an error if not found
    ///
    /// # Arguments
    /// * `provider` - The provider name
    /// * `name` - The model name
    ///
    /// # Returns
    /// A result containing the model as a boxed trait object
    ///
    /// # Errors
    ///
    /// Returns `CandleModelError::ModelNotFound` if model doesn't exist or `CandleModelError::TypeMismatch` if type doesn't match
    pub fn get_required_boxed<T>(
        &self,
        provider: &'static str,
        name: &'static str,
    ) -> CandleResult<Box<T>>
    where
        T: Send + Sync + Sized + 'static,
    {
        self.get_boxed(provider, name)?
            .ok_or_else(|| CandleModelError::ModelNotFound {
                provider: provider.into(),
                name: name.into(),
            })
    }
}

/// A handle to a registered model
///
/// This provides type-safe access to a registered model and ensures
/// proper cleanup when the last reference is dropped.
pub struct RegisteredModel<M: CandleModel + 'static> {
    handle: Arc<CandleModelHandle>,
    _marker: PhantomData<M>,
}

impl<M: CandleModel + 'static> Clone for RegisteredModel<M> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            _marker: PhantomData,
        }
    }
}

impl<M: CandleModel + 'static> RegisteredModel<M> {
    /// Try to get a reference to the model, returning an error on type mismatch
    ///
    /// This is a fallible alternative to using Deref or `AsRef`, which panic on type mismatch.
    /// Use this when you want to handle type mismatches gracefully.
    ///
    /// # Errors
    ///
    /// Returns `CandleModelError::TypeMismatch` if the stored model is not of type M
    pub fn try_get(&self) -> CandleResult<&M> {
        self.handle
            .as_model::<M>()
            .ok_or_else(|| CandleModelError::TypeMismatch {
                expected: std::any::type_name::<M>(),
                found: self.handle.type_name(),
            })
    }
}

impl<M: CandleModel + 'static> std::ops::Deref for RegisteredModel<M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        self.handle
            .as_model()
            .unwrap_or_else(|| {
                panic!(
                    "Type mismatch in RegisteredModel::deref - expected {}, found {}",
                    std::any::type_name::<M>(),
                    self.handle.type_name()
                )
            })
    }
}

impl<M: CandleModel + 'static> AsRef<M> for RegisteredModel<M> {
    fn as_ref(&self) -> &M {
        self.handle
            .as_model()
            .unwrap_or_else(|| {
                panic!(
                    "Type mismatch in RegisteredModel::as_ref - expected {}, found {}",
                    std::any::type_name::<M>(),
                    self.handle.type_name()
                )
            })
    }
}

impl<M: CandleModel + 'static> std::fmt::Debug for RegisteredModel<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisteredModel")
            .field("provider", &self.info().provider())
            .field("name", &self.info().name())
            .finish()
    }
}

impl<M: CandleModel + 'static> PartialEq for RegisteredModel<M> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.handle, &other.handle)
    }
}

impl<M: CandleModel + 'static> Eq for RegisteredModel<M> {}

impl<M: CandleModel + 'static> Hash for RegisteredModel<M> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.info().provider().hash(state);
        self.handle.info().name().hash(state);
    }
}
