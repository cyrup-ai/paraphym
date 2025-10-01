// src/lib.rs
//! Cognitive Memory System - A revolutionary memory architecture with quantum-inspired routing,
//! emergent agent evolution, and self-modifying capabilities.

pub mod api;
pub mod cognitive;
pub mod constants;
pub mod core;
pub mod graph;
pub mod migration;
pub mod monitoring;
pub mod query;
pub mod schema;
pub mod transaction;
pub mod utils;
pub mod vector;

// Re-export main types for convenience
// Conditional re-exports if API feature is enabled
#[cfg(feature = "api")]
pub use api::APIServer;

// Re-export core memory submodules for backward compatibility
pub use self::core::{
    MemoryMetadata, MemoryNode, SurrealDBMemoryManager, MemoryRelationship,
    manager::{MemoryManager, coordinator::MemoryCoordinator},
    primitives, filter, repository, storage, ops,
};
pub use self::core::SurrealDBMemoryManager as SurrealMemoryManager;

// Re-export manager module for compatibility
pub use self::core::manager;
// Re-export MemoryType from memory primitives (the actual implementation)
pub use self::core::primitives::types::MemoryType;
#[cfg(feature = "api")]
pub use utils::config::APIConfig;
pub use utils::config::MemoryConfig;
pub use utils::error::Error;



/// Initialize the traditional memory system with SurrealDB using a configuration object.
/// This is a more robust approach than just a DB URL.
pub async fn initialize(config: &MemoryConfig) -> Result<SurrealMemoryManager, Error> {
    use surrealdb::engine::any::connect;
    // use surrealdb::opt::auth::Root; // Root auth might not always be needed or desired, depends on config

    // Connect to the database using details from config
    let db = connect(&config.database.connection_string)
        .await
        .map_err(|e| Error::Config(format!("Failed to connect to database: {e}")))?;

    // Use namespace and database from config
    db.use_ns(&config.database.namespace)
        .use_db(&config.database.database)
        .await
        .map_err(|e| {
            Error::Config(format!(
                "Failed to use namespace '{}' and database '{}': {}",
                config.database.namespace, config.database.database, e
            ))
        })?;

    // Handle authentication if username and password are provided
    if let (Some(user), Some(pass)) = (&config.database.username, &config.database.password) {
        db.signin(surrealdb::opt::auth::Root {
            username: user.as_str(),
            password: pass.as_str(),
        })
        .await
        .map_err(|e| Error::Config(format!("Database sign-in failed: {e}")))?;
    }

    // Create the memory manager
    let manager = SurrealMemoryManager::new(db.clone());

    // Initialize the manager (e.g., create tables/schemas if they don_t exist)
    manager.initialize().await?;

    // Run schema migrations to ensure database schema is up-to-date
    tracing::info!("Running schema migrations...");
    manager.run_migrations().await?;
    tracing::info!("Schema migrations completed");

    Ok(manager)
}
