//! Entity model for graph-based memory
//!
//! This module provides entity abstractions for mapping domain objects
//! to graph nodes with support for validation, attributes, and persistence.
//! All operations are thread-safe and use async patterns for maximum performance.

mod futures;
mod types;
mod repository;
mod surreal;

// Re-export all public types to preserve API compatibility
pub use futures::{
    PendingEntity,
    PendingEntityOption,
    PendingEntityList,
    PendingEntityCount,
    PendingUnit,
};

pub use types::{
    Entity,
    EntityValidatorFn,
    BaseEntity,
};

pub use repository::EntityRepository;

pub use surreal::SurrealEntityRepository;
