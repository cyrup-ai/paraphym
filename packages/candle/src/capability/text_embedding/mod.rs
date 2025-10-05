//! Text Embedding Capability
//!
//! Providers that implement text embedding using EmbeddingModel trait.

pub mod bert;
pub mod gte_qwen;
pub mod jina_bert;
pub mod nvembed;
pub mod stella;

// Re-exports for convenience
pub use bert::{CandleBertConfig, CandleBertEmbeddingProvider};
pub use gte_qwen::{CandleGteQwenConfig, CandleGteQwenEmbeddingProvider};
pub use jina_bert::{CandleJinaBertConfig, CandleJinaBertEmbeddingProvider};
pub use nvembed::{CandleNvEmbedConfig, CandleNvEmbedEmbeddingProvider};
pub use stella::{StellaConfig, StellaEmbeddingProvider};
