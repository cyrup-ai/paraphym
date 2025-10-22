//! NVEmbed v2 embedding provider for local inference using Candle ML framework
//!
//! This provider uses nvidia/NV-Embed-v2 model for generating
//! 4096-dimensional embeddings with Mistral decoder and latent attention.

mod base;
mod config;
mod instruction;
mod loaded;

// Public API - maintain exact same exports as original nvembed.rs
pub use base::CandleNvEmbedEmbeddingModel;
pub use loaded::LoadedNvEmbedModel;
