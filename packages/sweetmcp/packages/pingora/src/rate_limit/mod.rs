//! Rate limiting module decomposition
//!
//! This module provides the decomposed rate limiting functionality split into
//! logical modules for better maintainability and adherence to the 300-line limit.

#![allow(dead_code)]

pub mod algorithms;
pub mod distributed;
pub mod limiter;

// Re-export key types and functions for backward compatibility
pub use limiter::AdvancedRateLimitManager;
