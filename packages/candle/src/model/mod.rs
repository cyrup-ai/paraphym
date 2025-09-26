//! Model system core module
//!
//! This module provides the core abstractions and types for AI model management,
//! including traits, information, registry, and error handling.

pub mod capabilities;
// Error types are defined in domain/model/error.rs
pub mod info;
pub mod registry;
pub mod resolver;
pub mod traits;
pub mod usage;
pub mod validation;

// Generated modules - created by build system
pub mod providers;
pub mod models;

// Re-export commonly used types
pub use capabilities::*;
pub use crate::domain::model::error::{CandleModelError as ModelError, CandleResult as Result};
pub use info::ModelInfo;
pub use registry::ModelRegistry;
pub use resolver::*;
pub use traits::*;
pub use usage::Usage;
pub use validation::*;

// Re-export generated types (commented out until build system generates actual content)
// pub use providers::*;
// pub use models::*;
