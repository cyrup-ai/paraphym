pub mod client;
pub mod config;
pub mod dao;
pub mod error;
pub mod group;
pub mod result;
pub mod role;
pub mod user;

// Re-export main components
pub use client::{DatabaseClient, connect_database};
pub use config::{DatabaseConfig, StorageEngine};
pub use dao::{BaseDao, Dao, Entity};
pub use error::{SurrealdbError, SurrealdbErrorContext};
// Export common SurrealDB types for convenience
pub use group::Group;
pub use role::Role;
pub use surrealdb::Surreal;
// Use types from surrealdb SDK, not core
pub use surrealdb::{RecordId as Thing, Value, Object};
// Array is in val module, not sql
pub use surrealdb_core::val::Array;
pub use user::User;
