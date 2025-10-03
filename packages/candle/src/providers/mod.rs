//! Candle Model Providers
//!
//! This module provides model providers for the Candle ML framework integration.
//! All providers implement the CandleCompletionModel trait for consistent streaming inference.

pub mod bert_embedding;
pub mod clip_vision;
pub mod gte_qwen_embedding;
pub mod jina_bert_embedding;
pub mod kimi_k2;
pub mod nvembed_embedding;
pub mod qwen3_coder;
pub mod stable_diffusion_35_turbo;
pub mod flux_schnell;
pub mod stella_embedding;
pub mod tokenizer; // tokenizers dependency is available in Cargo.toml

// Re-export primary provider types
pub use bert_embedding::{CandleBertConfig, CandleBertEmbeddingProvider};
pub use clip_vision::ClipVisionProvider;
pub use gte_qwen_embedding::{CandleGteQwenConfig, CandleGteQwenEmbeddingProvider};
pub use jina_bert_embedding::{CandleJinaBertConfig, CandleJinaBertEmbeddingProvider};
pub use kimi_k2::{CandleKimiK2Config, CandleKimiK2Provider};
pub use nvembed_embedding::{CandleNvEmbedConfig, CandleNvEmbedEmbeddingProvider};
pub use qwen3_coder::{CandleQwen3CoderConfig, CandleQwen3CoderProvider};
pub use stable_diffusion_35_turbo::{SD35TurboConfig, StableDiffusion35Turbo};
pub use flux_schnell::{FluxConfig, FluxSchnell};
pub use stella_embedding::{StellaConfig, StellaEmbeddingProvider};
pub use tokenizer::{CandleTokenizer, CandleTokenizerConfig};
