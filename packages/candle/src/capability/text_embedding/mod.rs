//! Text Embedding Capability
//!
//! Providers that implement text embedding using EmbeddingModel trait.

pub mod safetensors_validation;

pub mod bert;
pub mod gte_qwen;
pub mod jina_bert;
pub mod nvembed;
pub mod stella;

// Re-exports for convenience
pub(crate) use bert::CandleBertEmbeddingModel;
pub(crate) use gte_qwen::CandleGteQwenEmbeddingModel;
pub(crate) use jina_bert::CandleJinaBertEmbeddingModel;
pub(crate) use nvembed::CandleNvEmbedEmbeddingModel;
pub(crate) use stella::StellaEmbeddingModel;
