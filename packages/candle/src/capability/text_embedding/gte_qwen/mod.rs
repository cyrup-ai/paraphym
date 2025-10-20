//! GTE-Qwen embedding provider for local inference using Candle ML framework
//! GTE-Qwen2 Embedding Provider
//!
//! This provider uses Alibaba-NLP/gte-Qwen2-1.5B-instruct model for generating
//! 1536-dimensional embeddings with lazy-loading via huggingface_file().

mod config;
mod instruction;
mod base;
mod loaded;

// Public API - maintain exact same exports as original gte_qwen.rs
pub use base::CandleGteQwenEmbeddingModel;
pub use loaded::LoadedGteQwenModel;
