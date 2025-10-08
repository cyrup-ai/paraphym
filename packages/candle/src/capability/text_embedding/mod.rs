//! Text Embedding Capability
//!
//! Providers that implement text embedding using EmbeddingModel trait.

pub mod bert;
pub mod gte_qwen;
pub mod jina_bert;
pub mod nvembed;
pub mod stella;

// Re-exports for convenience
pub use bert::{CandleBertConfig, CandleBertEmbeddingModel};
pub use gte_qwen::{CandleGteQwenConfig, CandleGteQwenEmbeddingModel};
pub use jina_bert::{CandleJinaBertConfig, CandleJinaBertEmbeddingModel};
pub use nvembed::{CandleNvEmbedConfig, CandleNvEmbedEmbeddingModel};
pub use stella::{StellaConfig, StellaEmbeddingModel};
