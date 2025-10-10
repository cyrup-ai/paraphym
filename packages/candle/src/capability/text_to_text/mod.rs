//! Text-to-Text Generation Capability
//!
//! Models capable of generating text completions from text prompts.

pub mod kimi_k2;
pub mod phi4_reasoning;
pub mod qwen3_coder;

// Re-exports for convenience
pub use kimi_k2::{CandleKimiK2Model, LoadedKimiK2Model, KIMI_K2_MODEL_INFO};
pub use phi4_reasoning::{CandlePhi4ReasoningModel, LoadedPhi4ReasoningModel, PHI4_REASONING_MODEL_INFO};
pub use qwen3_coder::{CandleQwen3CoderModel, LoadedQwen3CoderModel, QWEN3_CODER_MODEL_INFO};
