//! Domain initialization and configuration

pub mod globals;

use std::sync::Arc;
use paraphym_memory::memory::manager::surreal::SurrealDBMemoryManager;
use paraphym_memory::utils::config::MemoryConfig;
use surrealdb::{Surreal, engine::any::{connect, Any}};
use crate::core::DomainInitError;

/// Initialize memory service with default configuration
///
/// Returns a properly configured memory manager service (not streamed)
/// Following the universal pattern: managers are services, not data to be streamed
pub async fn initialize_memory_service() -> Result<SurrealDBMemoryManager, DomainInitError> {
    let config = get_default_memory_config();
    initialize_memory_service_with_config(config).await
}

/// Initialize memory service with custom configuration
///
/// # Arguments
/// * `config` - Memory configuration with database settings
///
/// # Returns
/// Properly initialized SurrealDBMemoryManager ready for use
///
/// # Performance
/// Production-quality SurrealDB connection with cognitive memory features
pub async fn initialize_memory_service_with_config(
    config: MemoryConfig,
) -> Result<SurrealDBMemoryManager, DomainInitError> {
    // Connect to SurrealDB using the real SDK
    let db: Surreal<Any> = connect(&config.database.connection_string)
        .await
        .map_err(|e| DomainInitError::DatabaseConnectionFailed(e.to_string()))?;

    // Use namespace and database
    db.use_ns(&config.database.namespace)
        .use_db(&config.database.database_name)
        .await
        .map_err(|e| DomainInitError::DatabaseInitializationFailed(e.to_string()))?;

    // Create the real memory manager with SurrealDB connection
    let manager = SurrealDBMemoryManager::new(db);

    // Initialize the memory schema and indexes
    manager
        .initialize()
        .await
        .map_err(|e| DomainInitError::MemoryInitializationFailed(e.to_string()))?;

    Ok(manager)
}

/// Get default memory configuration
///
/// # Returns
/// Default configuration for local SurrealDB with cognitive features
pub fn get_default_memory_config() -> MemoryConfig {
    MemoryConfig::default()
}

/// Memory service connection pool for efficient resource management
///
/// Uses Arc<SurrealDBMemoryManager> for shared ownership across threads
static MEMORY_SERVICE_POOL: once_cell::sync::Lazy<crossbeam_queue::SegQueue<Arc<SurrealDBMemoryManager>>> =
    once_cell::sync::Lazy::new(|| crossbeam_queue::SegQueue::new());

/// Get a memory manager from the connection pool
///
/// # Returns
/// Shared reference to memory manager service, or None if pool is empty
pub fn get_from_pool() -> Option<Arc<SurrealDBMemoryManager>> {
    MEMORY_SERVICE_POOL.pop()
}

/// Return a memory manager to the connection pool
///
/// # Arguments
/// * `memory` - Memory manager service to return to pool
pub fn return_to_pool(memory: Arc<SurrealDBMemoryManager>) {
    MEMORY_SERVICE_POOL.push(memory);
}

/// Get the current connection pool size
///
/// # Returns
/// Number of available managers in the pool
pub fn pool_size() -> usize {
    MEMORY_SERVICE_POOL.len()
}

/// Initialize the connection pool with pre-configured managers
///
/// # Arguments
/// * `pool_size` - Number of managers to pre-allocate
/// * `config` - Configuration for each manager
///
/// # Performance
/// Async initialization with proper error handling
pub async fn initialize_pool(pool_size: usize, config: MemoryConfig) -> Result<(), DomainInitError> {
    for _ in 0..pool_size {
        let manager = initialize_memory_service_with_config(config.clone()).await?;
        return_to_pool(Arc::new(manager));
    }
    Ok(())
}

// Candle-prefixed function aliases for domain compatibility
pub use initialize_memory_service as candle_initialize_memory_service;
pub use initialize_memory_service_with_config as candle_initialize_memory_service_with_config;
pub use get_default_memory_config as candle_get_default_memory_config;
pub use get_from_pool as candle_get_from_pool;
pub use return_to_pool as candle_return_to_pool;
pub use pool_size as candle_pool_size;
