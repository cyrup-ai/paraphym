//! Vision (Multimodal) Capability
//!
//! Providers that implement vision/multimodal capabilities.

pub mod clip_vision;
pub mod llava;

// Re-exports for convenience
pub use clip_vision::ClipVisionProvider;
pub use llava::{LLaVAProvider, LLaVAProviderConfig};
