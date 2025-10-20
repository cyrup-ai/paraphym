//! Zero-Allocation Error Handling System
//!
//! This module provides comprehensive error handling with zero heap allocation,
//! circuit breaker patterns, and lock-free error aggregation for blazing-fast performance.

// Module declarations
mod types;
mod circuit_breaker;
mod core;
mod breaker;
mod stats;
mod conversions;

// Re-export fundamental types
pub use types::{
    MAX_ERROR_MESSAGE_LEN,
    ErrorCategory,
    ErrorSeverity,
    ErrorRecoverability,
    ZeroAllocMessage,
    ErrorMessage,
};

// Re-export circuit breaker
pub use circuit_breaker::{
    CircuitBreakerState,
    CircuitBreaker,
    CircuitBreakerError,
};

// Re-export core error type
pub use core::{
    ZeroAllocError,
    ZeroAllocResult,
};

// Re-export error circuit breaker and counter
pub use breaker::{
    ErrorCircuitBreaker,
    ErrorCounter,
};

// Re-export statistics
pub use stats::{
    ErrorAggregator,
    record_error,
    error_stats,
    error_breaker,
    total_errors,
    reset_error_stats,
};

// Re-export conversion traits
pub use conversions::{
    IntoZeroAllocError,
    ZeroAllocResultExt,
};

// Convenience macros for creating errors with location

/// Convenience macro for creating errors with location
#[macro_export]
macro_rules! error {
    ($category:expr, $severity:expr, $recoverability:expr, $message:expr, $code:expr) => {
        $crate::domain::error::ZeroAllocError::new($category, $severity, $recoverability, $message, $code)
            .with_location(file!(), line!())
    };
}

/// Convenience macro for creating retriable errors
#[macro_export]
macro_rules! retriable_error {
    ($category:expr, $message:expr, $code:expr) => {
        $crate::error!(
            $category,
            $crate::domain::error::ErrorSeverity::Error,
            $crate::domain::error::ErrorRecoverability::Retriable,
            $message,
            $code
        )
    };
}

/// Convenience macro for creating permanent errors
#[macro_export]
macro_rules! permanent_error {
    ($category:expr, $message:expr, $code:expr) => {
        $crate::error!(
            $category,
            $crate::domain::error::ErrorSeverity::Error,
            $crate::domain::error::ErrorRecoverability::Permanent,
            $message,
            $code
        )
    };
}

/// Convenience macro for creating critical errors
#[macro_export]
macro_rules! critical_error {
    ($category:expr, $message:expr, $code:expr) => {
        $crate::error!(
            $category,
            $crate::domain::error::ErrorSeverity::Critical,
            $crate::domain::error::ErrorRecoverability::Manual,
            $message,
            $code
        )
    };
}
