//! Template storage implementation
//!
//! Provides persistent storage for templates with caching support.

use std::collections::HashMap;

use crate::domain::chat::templates::core::{ChatTemplate, TemplateError, TemplateResult};

/// Template storage interface
pub trait TemplateStore: Send + Sync {
    /// Store a template
    ///
    /// # Errors
    ///
    /// Returns `TemplateError` if template cannot be stored
    fn store(&self, template: &ChatTemplate) -> TemplateResult<()>;

    /// Retrieve a template by name
    ///
    /// # Errors
    ///
    /// Returns `TemplateError` if template retrieval fails
    fn get(&self, name: &str) -> TemplateResult<Option<ChatTemplate>>;

    /// Delete a template
    ///
    /// # Errors
    ///
    /// Returns `TemplateError` if template deletion fails
    fn delete(&self, name: &str) -> TemplateResult<bool>;

    /// List all template names
    ///
    /// # Errors
    ///
    /// Returns `TemplateError` if template listing fails
    fn list(&self) -> TemplateResult<Vec<String>>;

    /// Check if template exists
    ///
    /// # Errors
    ///
    /// Returns `TemplateError` if existence check fails
    fn exists(&self, name: &str) -> TemplateResult<bool>;
}

/// In-memory template store implementation
pub struct MemoryStore {
    templates: std::sync::RwLock<HashMap<String, ChatTemplate>>,
}

impl MemoryStore {
    /// Create a new memory store
    pub fn new() -> Self {
        Self {
            templates: std::sync::RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateStore for MemoryStore {
    fn store(&self, template: &ChatTemplate) -> TemplateResult<()> {
        let mut store = self
            .templates
            .write()
            .map_err(|_| TemplateError::StorageError {
                message: "Failed to acquire write lock".to_string(),
            })?;

        store.insert(template.name().clone(), template.clone());
        Ok(())
    }

    fn get(&self, name: &str) -> TemplateResult<Option<ChatTemplate>> {
        let store = self
            .templates
            .read()
            .map_err(|_| TemplateError::StorageError {
                message: "Failed to acquire read lock".to_string(),
            })?;

        Ok(store.get(name).cloned())
    }

    fn delete(&self, name: &str) -> TemplateResult<bool> {
        let mut store = self
            .templates
            .write()
            .map_err(|_| TemplateError::StorageError {
                message: "Failed to acquire write lock".to_string(),
            })?;

        Ok(store.remove(name).is_some())
    }

    fn list(&self) -> TemplateResult<Vec<String>> {
        let store = self
            .templates
            .read()
            .map_err(|_| TemplateError::StorageError {
                message: "Failed to acquire read lock".to_string(),
            })?;

        Ok(store.keys().cloned().collect())
    }

    fn exists(&self, name: &str) -> TemplateResult<bool> {
        let store = self
            .templates
            .read()
            .map_err(|_| TemplateError::StorageError {
                message: "Failed to acquire read lock".to_string(),
            })?;

        Ok(store.contains_key(name))
    }
}
