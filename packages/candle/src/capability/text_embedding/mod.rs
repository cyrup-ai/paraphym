//! Text Embedding Capability
//!
//! Providers that implement text embedding using EmbeddingModel trait.

pub mod bert;
pub mod gte_qwen;
pub mod jina_bert;
pub mod nvembed;
pub mod stella;

// Re-exports for convenience
pub use bert::CandleBertEmbeddingModel;
pub use gte_qwen::CandleGteQwenEmbeddingModel;
pub use jina_bert::CandleJinaBertEmbeddingModel;
pub use nvembed::CandleNvEmbedEmbeddingModel;
pub use stella::StellaEmbeddingModel;
