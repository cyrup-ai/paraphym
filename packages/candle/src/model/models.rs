//! Local Candle model definitions
//!
//! This module provides model enumeration for local Candle models.

use std::num::NonZeroU32;
use serde::{Deserialize, Serialize};
use super::providers::{LocalTextProvider, LocalEmbeddingProvider};

/// All available local Candle models - LOCAL ONLY
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CandleModel {
    /// Local text generation models
    LocalTextGeneration(LocalTextProvider),
    /// Local embedding models
    LocalEmbedding(LocalEmbeddingProvider),
}


/// Model metadata for basic info
#[derive(Debug, Clone)]
pub struct ModelMetadata {
    pub name: &'static str,
    pub context_length: Option<NonZeroU32>,
    pub supports_streaming: bool,
    pub supports_function_calling: bool,
}

impl LocalTextProvider {
    /// Get model metadata
    pub fn metadata(&self) -> ModelMetadata {
        match self {
            LocalTextProvider::KimiK2 => ModelMetadata {
                name: "kimi-k2-instruct",
                context_length: NonZeroU32::new(131072),
                supports_streaming: true,
                supports_function_calling: true,
            },
            LocalTextProvider::Qwen3Coder => ModelMetadata {
                name: "qwen3-coder-30b",
                context_length: NonZeroU32::new(32768),
                supports_streaming: true,
                supports_function_calling: true,
            },
        }
    }
}

impl LocalEmbeddingProvider {
    /// Get embedding dimensions
    pub fn dimensions(&self) -> usize {
        match self {
            LocalEmbeddingProvider::BERT => 384,
            LocalEmbeddingProvider::GTEQwen => 1536,
            LocalEmbeddingProvider::JinaBERT => 768,
            LocalEmbeddingProvider::NvEmbed => 4096,
            LocalEmbeddingProvider::Stella => 1024,
            LocalEmbeddingProvider::ClipVision => 512,
        }
    }
}

impl CandleModel {
    pub fn name(&self) -> &str {
        match self {
            CandleModel::LocalTextGeneration(LocalTextProvider::KimiK2) => "kimi-k2",
            CandleModel::LocalTextGeneration(LocalTextProvider::Qwen3Coder) => "qwen3-coder",
            CandleModel::LocalEmbedding(LocalEmbeddingProvider::BERT) => "bert-embedding",
            CandleModel::LocalEmbedding(LocalEmbeddingProvider::GTEQwen) => "gte-qwen-embedding",
            CandleModel::LocalEmbedding(LocalEmbeddingProvider::JinaBERT) => "jina-bert-embedding",
            CandleModel::LocalEmbedding(LocalEmbeddingProvider::NvEmbed) => "nvembed-embedding",
            CandleModel::LocalEmbedding(LocalEmbeddingProvider::Stella) => "stella-embedding",
            CandleModel::LocalEmbedding(LocalEmbeddingProvider::ClipVision) => "clip-vision-embedding",
        }
    }

    pub fn context_length(&self) -> u32 {
        match self {
            CandleModel::LocalTextGeneration(LocalTextProvider::KimiK2) => 131072,
            CandleModel::LocalTextGeneration(LocalTextProvider::Qwen3Coder) => 32768,
            CandleModel::LocalEmbedding(_) => 512,
        }
    }

    pub fn supports_streaming(&self) -> bool {
        match self {
            CandleModel::LocalTextGeneration(_) => true,
            CandleModel::LocalEmbedding(_) => false,
        }
    }
}

pub fn list_available_models() -> Vec<CandleModel> {
    vec![
        CandleModel::LocalTextGeneration(LocalTextProvider::KimiK2),
        CandleModel::LocalTextGeneration(LocalTextProvider::Qwen3Coder),
        CandleModel::LocalEmbedding(LocalEmbeddingProvider::BERT),
        CandleModel::LocalEmbedding(LocalEmbeddingProvider::GTEQwen),
        CandleModel::LocalEmbedding(LocalEmbeddingProvider::JinaBERT),
        CandleModel::LocalEmbedding(LocalEmbeddingProvider::NvEmbed),
        CandleModel::LocalEmbedding(LocalEmbeddingProvider::Stella),
    ]
}