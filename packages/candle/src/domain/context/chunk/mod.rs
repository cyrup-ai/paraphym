//! Chunk Types for Streaming Operations
//!
//! These types represent partial data that flows through `AsyncStream<T>`
//! and are designed to work with the `NotResult` constraint.
//!
//! Originally consolidated from a monolithic chunk.rs file, now organized into
//! focused modules for better maintainability:
//!
//! - [`media`] - Image, audio, voice, and speech chunks
//! - [`completion`] - LLM completion and chat message chunks
//! - [`data`] - Document and embedding chunks
//! - [`results`] - Result wrapper types for async operations
//! - [`wrappers`] - Generic and primitive type wrappers
//! - [`workflow`] - Workflow-specific data chunks

pub mod media;
pub mod completion;
pub mod data;
pub mod results;
pub mod wrappers;
pub mod workflow;

// Re-export all public types to maintain flat namespace
pub use media::*;
pub use completion::*;
pub use data::*;
pub use results::*;
pub use wrappers::*;
pub use workflow::*;
