//! Domain provider types for agent configuration
//!
//! This module provides domain-specific provider abstractions for use
//! in the fluent API and agent configuration.

use crate::model::providers::{LocalTextProvider, LocalEmbeddingProvider};

/// Domain provider configuration
#[derive(Debug, Clone)]
pub struct CandleDomainProvider {
    provider_type: DomainProviderType,
}

/// Domain provider types
#[derive(Debug, Clone)]
pub enum DomainProviderType {
    TextGeneration(LocalTextProvider),
    Embedding(LocalEmbeddingProvider),
}

impl CandleDomainProvider {
    /// Create text generation provider
    #[must_use]
    pub fn text_generation(provider: LocalTextProvider) -> Self {
        Self {
            provider_type: DomainProviderType::TextGeneration(provider),
        }
    }

    /// Create embedding provider  
    #[must_use]
    pub fn embedding(provider: LocalEmbeddingProvider) -> Self {
        Self {
            provider_type: DomainProviderType::Embedding(provider),
        }
    }

    /// Check if this is a text generation provider
    #[must_use]
    pub fn is_text_generation(&self) -> bool {
        matches!(self.provider_type, DomainProviderType::TextGeneration(_))
    }

    /// Check if this is an embedding provider
    #[must_use]
    pub fn is_embedding(&self) -> bool {
        matches!(self.provider_type, DomainProviderType::Embedding(_))
    }
}