//! Domain model types for agent configuration
//!
//! This module provides domain-specific model abstractions for use
//! in the fluent API and agent builders.

use crate::model::providers::{LocalTextProvider, LocalEmbeddingProvider};

/// Domain model wrapper
#[derive(Debug, Clone)]
pub struct CandleDomainModel {
    model_type: DomainModelType,
}

/// Domain model types
#[derive(Debug, Clone)]
pub enum DomainModelType {
    TextGeneration(LocalTextProvider),
    Embedding(LocalEmbeddingProvider),
}

impl CandleDomainModel {
    /// Create text generation model
    #[must_use]
    pub fn text_generation(model: LocalTextProvider) -> Self {
        Self {
            model_type: DomainModelType::TextGeneration(model),
        }
    }

    /// Create embedding model
    #[must_use]
    pub fn embedding(model: LocalEmbeddingProvider) -> Self {
        Self {
            model_type: DomainModelType::Embedding(model),
        }
    }

    /// Check if this is a text generation model
    #[must_use]
    pub fn is_text_generation(&self) -> bool {
        matches!(self.model_type, DomainModelType::TextGeneration(_))
    }

    /// Check if this is an embedding model
    #[must_use]
    pub fn is_embedding(&self) -> bool {
        matches!(self.model_type, DomainModelType::Embedding(_))
    }

    /// Get model name
    #[must_use]
    pub fn name(&self) -> &'static str {
        match &self.model_type {
            DomainModelType::TextGeneration(model) => model.name(),
            DomainModelType::Embedding(model) => match model {
                LocalEmbeddingProvider::BERT => "bert-embedding",
                LocalEmbeddingProvider::GTEQwen => "gte-qwen-embedding",
                LocalEmbeddingProvider::JinaBERT => "jina-bert-embedding",
                LocalEmbeddingProvider::NvEmbed => "nvembed-embedding",
                LocalEmbeddingProvider::Stella => "stella-embedding",
                LocalEmbeddingProvider::ClipVision => "clip-vision-embedding",
            },
        }
    }
}