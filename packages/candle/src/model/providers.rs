//! Local Candle provider definitions
//!
//! This module provides provider enumeration for local Candle models.

use serde::{Deserialize, Serialize};

/// Local Candle provider types - LOCAL ONLY
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CandleProvider {
    /// Local text generation models
    LocalTextGeneration(LocalTextProvider),
    /// Local embedding models
    LocalEmbedding(LocalEmbeddingProvider),
}

/// Local text generation provider variants
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LocalTextProvider {
    KimiK2,
    Qwen3Coder,
}

/// Local embedding provider variants
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LocalEmbeddingProvider {
    BERT,
    GTEQwen,
    JinaBERT,
    NvEmbed,
    Stella,
    ClipVision,
}

impl LocalTextProvider {
    /// Get provider name
    pub fn name(&self) -> &'static str {
        match self {
            LocalTextProvider::KimiK2 => "kimi-k2",
            LocalTextProvider::Qwen3Coder => "qwen3-coder",
        }
    }
}

impl LocalEmbeddingProvider {
    /// Get provider name
    pub fn name(&self) -> &'static str {
        match self {
            LocalEmbeddingProvider::BERT => "bert",
            LocalEmbeddingProvider::GTEQwen => "gte-qwen",
            LocalEmbeddingProvider::JinaBERT => "jina-bert",
            LocalEmbeddingProvider::NvEmbed => "nvembed",
            LocalEmbeddingProvider::Stella => "stella",
            LocalEmbeddingProvider::ClipVision => "clip-vision",
        }
    }
}

impl CandleProvider {
    pub fn name(&self) -> &str {
        match self {
            CandleProvider::LocalTextGeneration(provider) => provider.name(),
            CandleProvider::LocalEmbedding(provider) => provider.name(),
        }
    }

    pub fn supports_streaming(&self) -> bool {
        match self {
            CandleProvider::LocalTextGeneration(_) => true,
            CandleProvider::LocalEmbedding(_) => false,
        }
    }
}