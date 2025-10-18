//! Image Embedding Capability
//!
//! Providers that implement image embedding using ImageEmbeddingCapable trait.

pub mod clip_vision;
pub mod clip_vision_embedding;

// Re-exports for convenience
pub use clip_vision::{ClipVisionModel, LoadedClipVisionModel};
pub use clip_vision_embedding::ClipVisionEmbeddingModel;
