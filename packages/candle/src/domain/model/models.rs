//! Domain model types for agent configuration
//!
//! This module re-exports model types from the providers module for backward compatibility.
//! The canonical definitions are in the providers module.

// Re-export all model types from providers
pub use super::providers::{
    CandleDomainModel,
    DomainModelType,
    ImageEmbeddingModel,
    TextEmbeddingModel,
    TextToImageModel,
    TextToTextModel,
    VisionModel,
};
