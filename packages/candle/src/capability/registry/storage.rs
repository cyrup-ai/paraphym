//! Registry storage - unified registries for all model types using parking_lot::RwLock

use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use parking_lot::RwLock;

use super::enums::*;
use crate::capability::text_embedding::{
    CandleBertEmbeddingModel, CandleGteQwenEmbeddingModel, CandleJinaBertEmbeddingModel,
    CandleNvEmbedEmbeddingModel, StellaEmbeddingModel,
};
use crate::capability::text_to_text::{
    CandleKimiK2Model, CandlePhi4ReasoningModel,
};
use crate::capability::vision::LLaVAModel;
use crate::domain::model::traits::CandleModel;

//==============================================================================
// UNIFIED REGISTRIES
//==============================================================================
// All registries now use LazyLock<RwLock<HashMap<String, T>>> for unified
// storage that supports both static initialization and runtime registration.
// parking_lot::RwLock provides sync read/write access with excellent performance.

/// Unified text-to-text model registry
///
/// Initialized with static models (Kimi K2, Phi4 Reasoning) and supports
/// runtime registration for models requiring async initialization (e.g., Qwen3Coder).
pub(super) static TEXT_TO_TEXT_UNIFIED: LazyLock<RwLock<HashMap<String, TextToTextModel>>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();

        let model = Arc::new(CandleKimiK2Model::default());
        let key = model.info().registry_key.to_string();
        map.insert(key, TextToTextModel::KimiK2(model));

        let model = Arc::new(CandlePhi4ReasoningModel::default());
        let key = model.info().registry_key.to_string();
        map.insert(key, TextToTextModel::Phi4Reasoning(model));

        RwLock::new(map)
    });

/// Unified text embedding model registry
///
/// Initialized with all static embedding models (Stella, BERT, GteQwen, JinaBert, NvEmbed).
pub(super) static TEXT_EMBEDDING_UNIFIED: LazyLock<RwLock<HashMap<String, TextEmbeddingModel>>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();

        let model = Arc::new(StellaEmbeddingModel::default());
        let key = model.info().registry_key.to_string();
        map.insert(key, TextEmbeddingModel::Stella(model));

        let model = Arc::new(CandleBertEmbeddingModel::default());
        let key = model.info().registry_key.to_string();
        map.insert(key, TextEmbeddingModel::Bert(model));

        let model = Arc::new(CandleGteQwenEmbeddingModel::default());
        let key = model.info().registry_key.to_string();
        map.insert(key, TextEmbeddingModel::GteQwen(model));

        let model = Arc::new(CandleJinaBertEmbeddingModel::default());
        let key = model.info().registry_key.to_string();
        map.insert(key, TextEmbeddingModel::JinaBert(model));

        let model = Arc::new(CandleNvEmbedEmbeddingModel::default());
        let key = model.info().registry_key.to_string();
        map.insert(key, TextEmbeddingModel::NvEmbed(model));

        RwLock::new(map)
    });

/// Unified image embedding model registry
///
/// Starts empty because ClipVision requires local model files, not HF downloads.
/// Use runtime registration after downloading weights manually.
pub(super) static IMAGE_EMBEDDING_UNIFIED: LazyLock<RwLock<HashMap<String, ImageEmbeddingModel>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// Unified text-to-image model registry
///
/// Starts empty because Flux/SD require local model files, not HF downloads.
/// Use runtime registration after downloading weights manually.
pub(super) static TEXT_TO_IMAGE_UNIFIED: LazyLock<RwLock<HashMap<String, TextToImageModel>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// Unified vision model registry
///
/// Initialized with static vision models (LLaVA).
pub(super) static VISION_UNIFIED: LazyLock<RwLock<HashMap<String, VisionModel>>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();

        let model = Arc::new(LLaVAModel::default());
        let key = model.info().registry_key.to_string();
        map.insert(key, VisionModel::LLaVA(model));

        RwLock::new(map)
    });
