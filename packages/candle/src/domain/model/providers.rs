//! Domain model types for agent configuration
//!
//! This module provides domain-specific model type abstractions for use
//! in the fluent API and agent configuration, organized by capability.

use serde::{Deserialize, Serialize};

/// Text-to-text model variants (text generation)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextToTextModel {
    KimiK2,
    Qwen3Coder,
    Phi4Reasoning,
}

/// Text embedding model variants
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextEmbeddingModel {
    BERT,
    GTEQwen,
    JinaBERT,
    NvEmbed,
    Stella,
}

/// Image embedding model variants
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImageEmbeddingModel {
    ClipVision,
}

/// Vision model variants (multimodal)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VisionModel {
    ClipVision,
    LLaVA,
}

/// Text-to-image model variants (image generation)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextToImageModel {
    FluxSchnell,
    StableDiffusion35Turbo,
}

impl TextToTextModel {
    /// Get model name
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            TextToTextModel::KimiK2 => "kimi-k2",
            TextToTextModel::Qwen3Coder => "qwen3-coder",
            TextToTextModel::Phi4Reasoning => "phi-4-reasoning",
        }
    }
}

impl TextEmbeddingModel {
    /// Get model name
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            TextEmbeddingModel::BERT => "bert",
            TextEmbeddingModel::GTEQwen => "gte-qwen",
            TextEmbeddingModel::JinaBERT => "jina-bert",
            TextEmbeddingModel::NvEmbed => "nvembed",
            TextEmbeddingModel::Stella => "stella",
        }
    }
}

impl ImageEmbeddingModel {
    /// Get model name
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            ImageEmbeddingModel::ClipVision => "clip-vision",
        }
    }
}

impl VisionModel {
    /// Get model name
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            VisionModel::ClipVision => "clip-vision",
            VisionModel::LLaVA => "llava",
        }
    }
}

impl TextToImageModel {
    /// Get model name
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            TextToImageModel::FluxSchnell => "flux-schnell",
            TextToImageModel::StableDiffusion35Turbo => "stable-diffusion-3.5-turbo",
        }
    }
}

/// Domain model configuration
#[derive(Debug, Clone)]
pub struct CandleDomainModel {
    model_type: DomainModelType,
}

/// Domain model types organized by capability
#[derive(Debug, Clone)]
pub enum DomainModelType {
    TextToText(TextToTextModel),
    TextEmbedding(TextEmbeddingModel),
    ImageEmbedding(ImageEmbeddingModel),
    Vision(VisionModel),
    TextToImage(TextToImageModel),
}

impl CandleDomainModel {
    /// Create text-to-text model
    #[must_use]
    pub fn text_to_text(model: TextToTextModel) -> Self {
        Self {
            model_type: DomainModelType::TextToText(model),
        }
    }

    /// Create text embedding model
    #[must_use]
    pub fn text_embedding(model: TextEmbeddingModel) -> Self {
        Self {
            model_type: DomainModelType::TextEmbedding(model),
        }
    }

    /// Create image embedding model
    #[must_use]
    pub fn image_embedding(model: ImageEmbeddingModel) -> Self {
        Self {
            model_type: DomainModelType::ImageEmbedding(model),
        }
    }

    /// Create vision model
    #[must_use]
    pub fn vision(model: VisionModel) -> Self {
        Self {
            model_type: DomainModelType::Vision(model),
        }
    }

    /// Create text-to-image model
    #[must_use]
    pub fn text_to_image(model: TextToImageModel) -> Self {
        Self {
            model_type: DomainModelType::TextToImage(model),
        }
    }

    /// Check if this is a text-to-text model
    #[must_use]
    pub fn is_text_to_text(&self) -> bool {
        matches!(self.model_type, DomainModelType::TextToText(_))
    }

    /// Check if this is a text embedding model
    #[must_use]
    pub fn is_text_embedding(&self) -> bool {
        matches!(self.model_type, DomainModelType::TextEmbedding(_))
    }

    /// Check if this is an image embedding model
    #[must_use]
    pub fn is_image_embedding(&self) -> bool {
        matches!(self.model_type, DomainModelType::ImageEmbedding(_))
    }

    /// Check if this is a vision model
    #[must_use]
    pub fn is_vision(&self) -> bool {
        matches!(self.model_type, DomainModelType::Vision(_))
    }

    /// Check if this is a text-to-image model
    #[must_use]
    pub fn is_text_to_image(&self) -> bool {
        matches!(self.model_type, DomainModelType::TextToImage(_))
    }

    /// Get the model name
    #[must_use]
    pub fn name(&self) -> &'static str {
        match &self.model_type {
            DomainModelType::TextToText(model) => model.name(),
            DomainModelType::TextEmbedding(model) => model.name(),
            DomainModelType::ImageEmbedding(model) => model.name(),
            DomainModelType::Vision(model) => model.name(),
            DomainModelType::TextToImage(model) => model.name(),
        }
    }
}
