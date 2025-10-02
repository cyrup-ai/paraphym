//! TLS Builder Interface
//!
//! This module provides a fluent, secure-by-default certificate management API.
//! All internal complexity is hidden behind the builder interface.


// Internal modules - not exposed publicly
pub(crate) mod bootstrap_client;
pub mod certificate;
pub(crate) mod crl_cache;
pub mod errors;
pub(crate) mod key_encryption;
pub(crate) mod ocsp;

pub mod tls_manager;
pub mod types;

// Public builder interface - the only public API
pub mod builder;
pub use builder::CertificateAuthority;

// Public TLS manager for enterprise connections

// Public error types for TLS operations

// Public certificate types and utilities
