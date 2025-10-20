//! Memory retrieval strategies and algorithms
//!
//! This module provides multiple strategies for retrieving relevant memories:
//! - **Semantic**: Vector similarity using embeddings
//! - **Temporal**: Time-based with exponential decay
//! - **Hybrid**: Combines multiple strategies with cognitive filtering
//! - **Manager**: Orchestrates strategies with unified API

// Submodules
pub mod types;
pub mod strategy;
pub mod semantic;
pub mod temporal;
pub mod hybrid;
pub mod manager;

// Re-export all public types to maintain API compatibility
pub use types::{RetrievalMethod, RetrievalResult, PendingRetrieval};
pub use strategy::RetrievalStrategy;
pub use semantic::SemanticRetrieval;
pub use temporal::TemporalRetrieval;
pub use hybrid::HybridRetrieval;
pub use manager::RetrievalManager;
