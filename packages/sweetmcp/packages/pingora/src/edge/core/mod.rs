//! Edge service core module
//!
//! This module provides comprehensive EdgeService functionality including
//! service initialization, operations, and builder pattern with zero allocation
//! patterns and blazing-fast performance.


pub mod builder;
pub mod operations;
pub mod proxy_impl;
pub mod service;

// Re-export key types and functions for ergonomic usage
pub use builder::EdgeServiceBuilder;
pub use service::{EdgeService, EdgeServiceError};
