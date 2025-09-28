//! Utility modules for domain types and operations

pub mod json_util;
pub mod notnan;

// Re-export commonly used types
pub use notnan::{NotNan, NotNanError};
