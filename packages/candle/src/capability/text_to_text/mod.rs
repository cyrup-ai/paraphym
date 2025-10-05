//! Text-to-Text Generation Capability
//!
//! Providers that implement text generation (completion) using CandleCompletionModel trait.

pub mod kimi_k2;
pub mod phi4_reasoning;
pub mod qwen3_coder;

// Re-exports for convenience
pub use kimi_k2::{CandleKimiK2Config, CandleKimiK2Provider};
pub use phi4_reasoning::{CandlePhi4ReasoningConfig, CandlePhi4ReasoningProvider};
pub use qwen3_coder::{CandleQwen3CoderConfig, CandleQwen3CoderProvider};
