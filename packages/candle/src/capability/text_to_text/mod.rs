//! Text-to-Text Generation Capability
//!
//! Models capable of generating text completions from text prompts.

pub mod kimi_k2;
pub mod phi4_reasoning;
pub mod qwen3_quantized;

// Re-exports for convenience
pub(crate) use kimi_k2::CandleKimiK2Model;
pub(crate) use phi4_reasoning::CandlePhi4ReasoningModel;
pub(crate) use qwen3_quantized::CandleQwen3QuantizedModel;
