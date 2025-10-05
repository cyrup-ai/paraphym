//! Text-to-Image Generation Capability
//!
//! Providers that implement image generation using ImageGenerationModel trait.

pub mod flux_schnell;
pub mod stable_diffusion_35_turbo;

// Re-exports for convenience
pub use flux_schnell::{FluxConfig, FluxSchnell};
pub use stable_diffusion_35_turbo::{SD35TurboConfig, StableDiffusion35Turbo};
