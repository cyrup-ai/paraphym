//! Memory module that provides the core memory functionality

// New hierarchical module structure
pub mod manager;
pub mod ops;
pub mod primitives;
pub mod schema;
pub mod systems;

#[cfg(test)]
pub mod tests;

// Re-export main types to maintain backward compatibility
// Using a hybrid approach: explicit exports for conflicts, module re-exports for compatibility

// Module re-exports for backward compatibility (keeping internal imports working)
// Manager types (explicit imports to avoid conflicts)
pub use manager::coordinator::MemoryCoordinator;
pub use manager::surreal::MemoryQuery as SurrealMemoryQuery; // Rename conflicting type
pub use manager::surreal::{
    MemoryManager, MemoryStream, PendingDeletion, PendingMemory, PendingRelationship,
    RelationshipStream, SurrealDBMemoryManager,
};
pub use ops::filter;
pub use ops::filter::{MemoryFilter, MemoryFilterBuilder, TimeRange}; /* Keep ops versions as primary */
// Main operations types - explicit to avoid conflicts
pub use ops::query::{MemoryQuery, MemoryQueryExecutor, MemoryQueryResult, SortOrder}; /* Keep ops::MemoryQuery as primary */
pub use ops::repository;
pub use ops::storage;
pub use primitives::metadata::MemoryMetadata;
// Alias the conflicting primitives types
pub use primitives::metadata::{
    MemoryFilter as PrimitivesMemoryFilter, TimeRange as PrimitivesTimeRange,
};
// Primitives types
pub use primitives::node::MemoryNode;
pub use primitives::relationship::MemoryRelationship;
pub use primitives::types::{
    BaseMemory, MemoryContent, MemoryType, MemoryTypeEnum, RelationshipType,
};
// Schema and systems
pub use schema::*;
pub use systems::*;
