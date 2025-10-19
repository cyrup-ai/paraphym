//! Model enum definitions and CandleModel trait implementations

use std::sync::Arc;
use crate::domain::model::CandleModelInfo;
use crate::domain::model::traits::CandleModel;

// Import all model types
use crate::capability::image_embedding::ClipVisionEmbeddingModel;
use crate::capability::text_embedding::{
    CandleBertEmbeddingModel, CandleGteQwenEmbeddingModel, CandleJinaBertEmbeddingModel,
    CandleNvEmbedEmbeddingModel, StellaEmbeddingModel,
};
use crate::capability::text_to_image::{FluxSchnell, StableDiffusion35Turbo};
use crate::capability::text_to_text::{
    CandleKimiK2Model, CandlePhi4ReasoningModel, CandleQwen3QuantizedModel,
};
use crate::capability::vision::LLaVAModel;

//==============================================================================
// CAPABILITY ENUMS
//==============================================================================

/// Enum for all text-to-text models
#[derive(Clone, Debug)]
pub enum TextToTextModel {
    KimiK2(Arc<CandleKimiK2Model>),
    Qwen3Quantized(Arc<CandleQwen3QuantizedModel>),
    Phi4Reasoning(Arc<CandlePhi4ReasoningModel>),
}

/// Enum for all text embedding models
#[derive(Clone, Debug)]
pub enum TextEmbeddingModel {
    Stella(Arc<StellaEmbeddingModel>),
    Bert(Arc<CandleBertEmbeddingModel>),
    GteQwen(Arc<CandleGteQwenEmbeddingModel>),
    JinaBert(Arc<CandleJinaBertEmbeddingModel>),
    NvEmbed(Arc<CandleNvEmbedEmbeddingModel>),
}

/// Enum for all image embedding models
#[derive(Clone, Debug)]
pub enum ImageEmbeddingModel {
    ClipVision(Arc<ClipVisionEmbeddingModel>),
}

/// Enum for all text-to-image models
#[derive(Clone, Debug)]
pub enum TextToImageModel {
    FluxSchnell(Arc<FluxSchnell>),
    StableDiffusion35Turbo(Arc<StableDiffusion35Turbo>),
}

/// Enum for all vision/multimodal models
#[derive(Clone, Debug)]
pub enum VisionModel {
    LLaVA(Arc<LLaVAModel>),
}

/// Unified enum for cross-capability model access
#[derive(Clone, Debug)]
pub enum AnyModel {
    TextToText(TextToTextModel),
    TextEmbedding(TextEmbeddingModel),
    ImageEmbedding(ImageEmbeddingModel),
    TextToImage(TextToImageModel),
    Vision(VisionModel),
}

//==============================================================================
// CANDLEMODEL TRAIT IMPLEMENTATIONS
//==============================================================================

impl CandleModel for TextToTextModel {
    #[inline]
    fn info(&self) -> &'static CandleModelInfo {
        match self {
            Self::KimiK2(m) => m.info(),
            Self::Qwen3Quantized(m) => m.info(),
            Self::Phi4Reasoning(m) => m.info(),
        }
    }
}

impl CandleModel for TextEmbeddingModel {
    #[inline]
    fn info(&self) -> &'static CandleModelInfo {
        match self {
            Self::Stella(m) => m.info(),
            Self::Bert(m) => m.info(),
            Self::GteQwen(m) => m.info(),
            Self::JinaBert(m) => m.info(),
            Self::NvEmbed(m) => m.info(),
        }
    }
}

impl CandleModel for ImageEmbeddingModel {
    #[inline]
    fn info(&self) -> &'static CandleModelInfo {
        match self {
            Self::ClipVision(m) => m.info(),
        }
    }
}

impl CandleModel for TextToImageModel {
    #[inline]
    fn info(&self) -> &'static CandleModelInfo {
        match self {
            Self::FluxSchnell(m) => m.info(),
            Self::StableDiffusion35Turbo(m) => m.info(),
        }
    }
}

impl CandleModel for VisionModel {
    #[inline]
    fn info(&self) -> &'static CandleModelInfo {
        match self {
            Self::LLaVA(m) => m.info(),
        }
    }
}

impl CandleModel for AnyModel {
    #[inline]
    fn info(&self) -> &'static CandleModelInfo {
        match self {
            Self::TextToText(m) => m.info(),
            Self::TextEmbedding(m) => m.info(),
            Self::ImageEmbedding(m) => m.info(),
            Self::TextToImage(m) => m.info(),
            Self::Vision(m) => m.info(),
        }
    }
}
