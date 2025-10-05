//! Image Embedding Capability
//!
//! Providers that implement image embedding using EmbeddingModel trait.

pub mod clip_vision_embedding;

// Re-exports for convenience
pub use clip_vision_embedding::ClipVisionEmbeddingProvider;
