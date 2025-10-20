//! Zero-Allocation Context Provider System
//!
//! Production-ready context management with streaming-only architecture, zero Arc usage,
//! lock-free atomic operations, and immutable messaging patterns. Provides blazing-fast
//! context loading and management with full memory integration.
//!
//! Features: File/Directory/GitHub indexing, vector embeddings, memory storage,
//! parallel processing, real-time event streaming, comprehensive error handling.

pub mod types;
pub mod processor;
pub mod context_impl;

// Re-export all public types to maintain API compatibility
pub use types::*;
pub use processor::*;
pub use context_impl::*;
