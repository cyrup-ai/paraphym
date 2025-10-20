//! Vector search functionality - THREAD-SAFE SYNCHRONOUS OPERATIONS
//!
//! This module provides comprehensive vector search capabilities using:
//! - Synchronous vector similarity search with SIMD acceleration
//! - Thread-safe embedding generation and caching
//! - Hybrid search combining vector and keyword approaches
//! - Zero-allocation search result processing
//! - Advanced filtering and ranking algorithms

mod helpers;
mod types;
mod options;
mod cognitive;
mod core;
mod hybrid;

// Re-export public API (maintains backward compatibility)
pub use types::{SearchResult, KeywordSearchFn};
pub use options::SearchOptions;
pub use core::VectorSearch;
pub use hybrid::HybridSearch;

// Internal helpers available to other vector modules if needed
pub(crate) use cognitive::{CognitiveSearchState, process_deferred_results};
pub(crate) use helpers::task_string;
