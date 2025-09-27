//! Common cognitive types and utilities
//!
//! Shared types for local cognitive operations.

pub mod models;
pub mod types;

// Re-export key types
pub use models::{LocalModel, LocalModelType, CompletionCoreError};
pub use types::{CommitteeConfig, EvaluationRubric, ImpactFactors};