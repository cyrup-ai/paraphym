//! Certificate Authority domain object and builders
//!
//! This module has been decomposed into logical components for better maintainability:
//! - `types`: Core type definitions and data structures
//! - `utils`: Helper functions for formatting and utilities
//! - `builders`: Base builder functionality and entry points
//! - `filesystem`: Filesystem-based certificate authority operations
//! - `keychain`: System keychain-based operations  
//! - `remote`: Remote URL-based certificate authority loading

pub mod builders;
pub mod filesystem;
pub mod keychain;
pub mod remote;
pub mod types;
pub mod utils;

// Import the responses module that this module depends on
pub mod responses {
    pub use crate::tls::builder::responses::*;
}

// Re-export all public types and builders for backward compatibility
pub use builders::AuthorityBuilder;
pub use types::*;
