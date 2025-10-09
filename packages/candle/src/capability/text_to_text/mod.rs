//! Text-to-Text Generation Capability
//!
//! Models capable of generating text completions from text prompts.

pub mod kimi_k2;
pub mod phi4_reasoning;
pub mod qwen3_coder;

// Re-exports for convenience
pub use kimi_k2::CandleKimiK2Model;
pub use phi4_reasoning::CandlePhi4ReasoningModel;
pub use qwen3_coder::CandleQwen3CoderModel;
