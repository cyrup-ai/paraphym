//! Global context management
//!
//! This module provides global context initialization and management
//! functionality with zero allocation patterns and blazing-fast performance.

use std::sync::Arc;
use tokio::sync::RwLock;

use super::types::{ApplicationContext, SamplingContext};

/// Global application context initialized at runtime
/// Wrapped in RwLock to support reinitialization
pub static APPLICATION_CONTEXT: RwLock<Option<ApplicationContext>> = RwLock::const_new(None);

/// Global sampling context initialized at runtime
/// Wrapped in RwLock to support reinitialization
pub static SAMPLING_CONTEXT: RwLock<Option<SamplingContext>> = RwLock::const_new(None);

/// Global context manager for initialization and access
pub struct GlobalContextManager;

impl GlobalContextManager {
    /// Initialize the global application context
    /// Must be called once during application startup
    pub async fn initialize_global_context(
        config_path: &std::path::Path,
        log_level: &str,
        plugin_configs: &[crate::config::PluginConfig],
    ) -> Result<(), anyhow::Error> {
        let config_path = config_path.to_path_buf();
        let log_level = log_level.to_string();
        let plugin_configs = plugin_configs.to_vec();

        // Check if already initialized
        {
            let app_lock = APPLICATION_CONTEXT.read().await;
            if app_lock.is_some() {
                return Err(anyhow::anyhow!("Global application context already initialized. Use reinitialize_contexts() for hot-reload."));
            }
        }

        // Initialize the application context
        let app_context = ApplicationContext::initialize(&config_path, &log_level, &plugin_configs)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Failed to initialize application context"))?;

        // Set the global instance
        {
            let mut app_lock = APPLICATION_CONTEXT.write().await;
            *app_lock = Some(app_context);
        }

        // Get the app context after it's been initialized
        let app_context = {
            let app_lock = APPLICATION_CONTEXT.read().await;
            app_lock
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Application context not initialized"))?
                .clone()
        };

        // Initialize the sampling context
        let sampling_context = SamplingContext::new(app_context);
        {
            let mut sampling_lock = SAMPLING_CONTEXT.write().await;
            *sampling_lock = Some(sampling_context);
        }

        log::info!("Global contexts initialized successfully");
        Ok(())
    }

    /// Get the global application context
    pub async fn get_application_context() -> Option<ApplicationContext> {
        let lock = APPLICATION_CONTEXT.read().await;
        lock.clone()
    }

    /// Get the global sampling context
    pub async fn get_sampling_context() -> Option<SamplingContext> {
        let lock = SAMPLING_CONTEXT.read().await;
        lock.clone()
    }

    /// Check if global contexts are initialized
    pub async fn are_contexts_initialized() -> bool {
        let app_lock = APPLICATION_CONTEXT.read().await;
        let sampling_lock = SAMPLING_CONTEXT.read().await;
        app_lock.is_some() && sampling_lock.is_some()
    }

    /// Check if application context is initialized
    pub async fn is_application_context_initialized() -> bool {
        let lock = APPLICATION_CONTEXT.read().await;
        lock.is_some()
    }

    /// Check if sampling context is initialized
    pub async fn is_sampling_context_initialized() -> bool {
        let lock = SAMPLING_CONTEXT.read().await;
        lock.is_some()
    }

    /// Get application context with error handling
    pub async fn require_application_context() -> Result<ApplicationContext, anyhow::Error> {
        let lock = APPLICATION_CONTEXT.read().await;
        lock.clone()
            .ok_or_else(|| anyhow::anyhow!("Application context not initialized"))
    }

    /// Get sampling context with error handling
    pub async fn require_sampling_context() -> Result<SamplingContext, anyhow::Error> {
        let lock = SAMPLING_CONTEXT.read().await;
        lock.clone()
            .ok_or_else(|| anyhow::anyhow!("Sampling context not initialized"))
    }

    /// Validate global context integrity
    pub async fn validate_global_contexts() -> Result<Vec<String>, anyhow::Error> {
        let mut errors = Vec::new();

        // Validate application context
        let app_lock = APPLICATION_CONTEXT.read().await;
        if let Some(app_context) = app_lock.as_ref() {
            let app_errors = app_context.validate().await;
            errors.extend(app_errors);
        } else {
            errors.push("Application context not initialized".to_string());
        }
        drop(app_lock);

        // Validate sampling context
        let sampling_lock = SAMPLING_CONTEXT.read().await;
        if sampling_lock.is_none() {
            errors.push("Sampling context not initialized".to_string());
        }
        drop(sampling_lock);

        Ok(errors)
    }

    /// Get global context statistics
    pub async fn get_global_context_stats() -> Result<GlobalContextStats, anyhow::Error> {
        let app_context = Self::require_application_context().await?;
        let sampling_context = Self::require_sampling_context().await?;

        let app_stats = app_context.get_stats().await;
        let sampling_stats = sampling_context.get_stats().await;

        Ok(GlobalContextStats {
            application_initialized: true,
            sampling_initialized: true,
            app_stats,
            sampling_stats,
        })
    }

    /// Reinitialize contexts with graceful shutdown of existing contexts
    ///
    /// This performs a complete reinitialization by:
    /// 1. Acquiring write locks on both contexts (blocks all access)
    /// 2. Shutting down existing contexts gracefully
    /// 3. Initializing new contexts with provided configuration
    ///
    /// **Warning:** This operation blocks all context access during reinitialization.
    /// In-flight requests may fail if they attempt to access context during this window.
    ///
    /// # Arguments
    /// * `config_path` - Path to new configuration file
    /// * `log_level` - New log level setting
    /// * `plugin_configs` - New plugin configurations
    ///
    /// # Returns
    /// * `Ok(())` if reinitialization succeeds
    /// * `Err` if shutdown or initialization fails
    pub async fn reinitialize_contexts(
        config_path: &std::path::Path,
        log_level: &str,
        plugin_configs: &[crate::config::PluginConfig],
    ) -> Result<(), anyhow::Error> {
        log::info!("Beginning context reinitialization...");

        // Acquire write locks for both contexts to prevent concurrent access
        // This blocks all reads until reinitialization completes
        let mut app_lock = APPLICATION_CONTEXT.write().await;
        let mut sampling_lock = SAMPLING_CONTEXT.write().await;

        // Shutdown existing application context if present
        if let Some(existing_app) = app_lock.take() {
            log::info!("Shutting down existing application context...");

            // Cleanup plugin manager
            if let Err(e) = existing_app.plugin_manager().shutdown().await {
                log::warn!("Plugin manager shutdown error during reinit: {}", e);
                // Continue despite errors - best effort cleanup
            }

            // Cleanup memory adapter
            if let Err(e) = existing_app.memory_adapter().shutdown().await {
                log::warn!("Memory adapter shutdown error during reinit: {}", e);
                // Continue despite errors - best effort cleanup
            }

            log::info!("Existing application context shut down successfully");
        }

        // Shutdown existing sampling context if present
        if let Some(existing_sampling) = sampling_lock.take() {
            log::info!("Shutting down existing sampling context...");

            // Clear active sessions
            existing_sampling.clear_active_sessions().await;

            // Clear sampling configuration
            existing_sampling.clear_sampling_config().await;

            log::info!("Existing sampling context shut down successfully");
        }

        // Initialize new application context
        log::info!("Initializing new application context...");
        let new_app_context = ApplicationContext::initialize(
            config_path,
            log_level,
            plugin_configs,
        )
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to initialize new application context"))?;

        // Initialize new sampling context
        log::info!("Initializing new sampling context...");
        let new_sampling_context = SamplingContext::new(new_app_context.clone());

        // Set new contexts (still holding write locks)
        *app_lock = Some(new_app_context);
        *sampling_lock = Some(new_sampling_context);

        // Locks automatically released here
        log::info!("Context reinitialization completed successfully");
        Ok(())
    }

    /// Shutdown contexts gracefully (cleanup before application exit)
    ///
    /// This performs cleanup but does not clear the global contexts.
    /// For reinitialization, use `reinitialize_contexts()` instead.
    pub async fn shutdown_contexts() -> Result<(), anyhow::Error> {
        log::info!("Beginning context shutdown...");

        // Perform cleanup operations before shutdown
        let app_lock = APPLICATION_CONTEXT.read().await;
        if let Some(app_context) = app_lock.as_ref() {
            // Cleanup plugin manager
            if let Err(e) = app_context.plugin_manager().shutdown().await {
                log::warn!("Plugin manager shutdown error: {}", e);
            }

            // Cleanup memory adapter
            if let Err(e) = app_context.memory_adapter().shutdown().await {
                log::warn!("Memory adapter shutdown error: {}", e);
            }

            log::info!("Application context shutdown completed");
        }
        drop(app_lock);

        let sampling_lock = SAMPLING_CONTEXT.read().await;
        if let Some(sampling_context) = sampling_lock.as_ref() {
            // Clear active sessions
            sampling_context.clear_active_sessions().await;

            // Clear sampling configuration
            sampling_context.clear_sampling_config().await;

            log::info!("Sampling context shutdown completed");
        }
        drop(sampling_lock);

        Ok(())
    }
}

/// Convenience functions for global context access
///
/// All functions are now async due to RwLock requirement.
pub mod context_access {
    use super::*;

    /// Get application context (panics if not initialized)
    ///
    /// **Note:** Now returns owned value, not reference
    pub async fn app_context() -> ApplicationContext {
        let lock = APPLICATION_CONTEXT.read().await;
        lock.clone()
            .expect("Application context not initialized")
    }

    /// Get sampling context (panics if not initialized)
    ///
    /// **Note:** Now returns owned value, not reference
    pub async fn sampling_context() -> SamplingContext {
        let lock = SAMPLING_CONTEXT.read().await;
        lock.clone()
            .expect("Sampling context not initialized")
    }

    /// Try to get application context (returns None if not initialized)
    ///
    /// **Note:** Now returns owned value, not reference
    pub async fn try_app_context() -> Option<ApplicationContext> {
        let lock = APPLICATION_CONTEXT.read().await;
        lock.clone()
    }

    /// Try to get sampling context (returns None if not initialized)
    ///
    /// **Note:** Now returns owned value, not reference
    pub async fn try_sampling_context() -> Option<SamplingContext> {
        let lock = SAMPLING_CONTEXT.read().await;
        lock.clone()
    }

    /// Get logger from application context
    pub async fn logger() -> Arc<crate::context::logger::ConsoleLogger> {
        app_context().await.logger().clone()
    }

    /// Get plugin manager from application context
    pub async fn plugin_manager() -> Arc<crate::plugin::PluginManager> {
        app_context().await.plugin_manager().clone()
    }

    /// Get memory adapter from application context
    pub async fn memory_adapter() -> Arc<crate::context::MemoryContextAdapter> {
        app_context().await.memory_adapter().clone()
    }

    /// Check if database is available
    pub async fn is_database_available() -> bool {
        try_app_context()
            .await
            .map(|ctx| ctx.is_database_initialized())
            .unwrap_or(false)
    }
}

/// Global context statistics
#[derive(Debug, Clone)]
pub struct GlobalContextStats {
    /// Whether application context is initialized
    pub application_initialized: bool,
    /// Whether sampling context is initialized
    pub sampling_initialized: bool,
    /// Application context statistics
    pub app_stats: super::types::ContextStats,
    /// Sampling context statistics
    pub sampling_stats: super::types::SamplingStats,
}

/// Initialize global context (convenience function)
pub async fn initialize_global_context(
    config_path: &std::path::Path,
    log_level: &str,
    plugin_configs: &[crate::config::PluginConfig],
) -> Result<(), anyhow::Error> {
    GlobalContextManager::initialize_global_context(config_path, log_level, plugin_configs).await
}
