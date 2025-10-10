//! # Capability Model Registry
//!
//! **THE ONLY MODEL REGISTRY IN THE ENTIRE CODEBASE**
//!
//! This is the single source of truth for all model registrations.
//! Every model in the system is registered here, keyed by its `registry_key`.
//!
//! ## Architecture
//!
//! Uses enum-based storage to eliminate type erasure while maintaining a single registry:
//! - Each capability has its own enum (TextToTextModel, TextEmbeddingModel, etc.)
//! - Each enum wraps Arc<ConcreteModel> for cheap cloning
//! - Enums implement CandleModel + their capability trait via match delegation
//! - Returns `impl Trait` instead of trait objects for zero-cost abstraction
//!
//! ## Pool Integration
//!
//! TextEmbedding models route through pool for performance:
//! - First request: Spawn 2 workers (0→2 cold start)
//! - Subsequent requests: Route to least-busy worker
//! - Workers keep models loaded in memory (no disk reload)
//!
//! TextToText, Vision, ImageEmbedding, TextToImage models call directly:
//! - These models already store state in structs
//! - No reload-per-call performance problem
//! - Pool integration not needed (yet)
//!
//! ## User Transparency
//!
//! Users call:
//! ```rust
//! let model = registry::get<TextEmbeddingModel>("registry_key")?;
//! let embedding = model.embed("text", None)?;  // Pool intercepts here
//! ```
//!
//! Pool integration is invisible - user code unchanged.
//!
//! ## To Add a New Model:
//!
//! 1. Implement `CandleModel` trait with static `MODEL_INFO`
//! 2. Implement capability trait(s): `TextToTextCapable`, `TextEmbeddingCapable`, etc.
//! 3. Add variant to the appropriate enum below
//! 4. Add model to the appropriate `register_*` function
//!
//! ## Thread Safety
//!
//! Uses `LazyLock` for one-time initialization of static HashMap registries.
//! Runtime registries use `OnceLock<RwLock<HashMap>>` for thread-safe mutation.
//!
//! ## Runtime Registration
//!
//! Some models (ClipVision, FluxSchnell, StableDiffusion35Turbo) require explicit
//! configuration or weight downloads and cannot be statically initialized. Use:
//! - `register_image_embedding()` for ClipVision models
//! - `register_text_to_image()` for Flux/StableDiffusion models
//! - `get_image_embedding_runtime()` / `get_text_to_image_runtime()` to retrieve them

use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock, OnceLock};

use crate::domain::model::traits::CandleModel;
use crate::domain::model::CandleModelInfo;

// Import capability traits
use crate::capability::traits::{
    TextToTextCapable, TextEmbeddingCapable, ImageEmbeddingCapable,
    VisionCapable, TextToImageCapable,
};

// Import all model types
use crate::capability::text_to_text::{
    CandleKimiK2Model, 
    CandleQwen3CoderModel,
    CandlePhi4ReasoningModel,
};
use crate::capability::text_embedding::{
    StellaEmbeddingModel, CandleBertEmbeddingModel, CandleGteQwenEmbeddingModel,
    CandleJinaBertEmbeddingModel, CandleNvEmbedEmbeddingModel,
};
use crate::capability::image_embedding::ClipVisionEmbeddingModel;
use crate::capability::text_to_image::{FluxSchnell, StableDiffusion35Turbo};
use crate::capability::vision::LLaVAModel;

// Import types needed for capability trait implementations
use crate::domain::prompt::CandlePrompt;
use crate::domain::completion::types::CandleCompletionParams;
use crate::domain::completion::CandleCompletionChunk;
use crate::domain::image_generation::{ImageGenerationConfig, ImageGenerationChunk};
use crate::domain::context::chunk::CandleStringChunk;
use candle_core::Device;
use ystream::AsyncStream;

// Pool imports
use crate::pool::capabilities::text_embedding_pool;
use crate::pool::core::PoolError;

// LoadedModel imports
use crate::capability::text_embedding::{
    gte_qwen::LoadedGteQwenModel,
    jina_bert::LoadedJinaBertModel,
    nvembed::LoadedNvEmbedModel,
    stella::LoadedStellaModel,
    bert::LoadedBertModel,
};

//==============================================================================
// CAPABILITY ENUMS
//==============================================================================

/// Enum for all text-to-text models
#[derive(Clone, Debug)]
pub enum TextToTextModel {
    KimiK2(Arc<CandleKimiK2Model>),
    Qwen3Coder(Arc<CandleQwen3CoderModel>),
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
    fn info(&self) -> &'static CandleModelInfo {
        match self {
            Self::KimiK2(m) => m.info(),
            Self::Qwen3Coder(m) => m.info(),
            Self::Phi4Reasoning(m) => m.info(),
        }
    }
}

impl CandleModel for TextEmbeddingModel {
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
    fn info(&self) -> &'static CandleModelInfo {
        match self {
            Self::ClipVision(m) => m.info(),
        }
    }
}

impl CandleModel for TextToImageModel {
    fn info(&self) -> &'static CandleModelInfo {
        match self {
            Self::FluxSchnell(m) => m.info(),
            Self::StableDiffusion35Turbo(m) => m.info(),
        }
    }
}

impl CandleModel for VisionModel {
    fn info(&self) -> &'static CandleModelInfo {
        match self {
            Self::LLaVA(m) => m.info(),
        }
    }
}

impl CandleModel for AnyModel {
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

//==============================================================================
// CAPABILITY TRAIT IMPLEMENTATIONS
//==============================================================================

impl TextToTextCapable for TextToTextModel {
    fn prompt(
        &self,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> AsyncStream<CandleCompletionChunk> {
        match self {
            Self::KimiK2(m) => m.prompt(prompt, params),
            Self::Qwen3Coder(m) => m.prompt(prompt, params),
            Self::Phi4Reasoning(m) => m.prompt(prompt, params),
        }
    }
}

impl TextEmbeddingCapable for TextEmbeddingModel {
    fn embed(&self, text: &str, task: Option<String>)
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::GteQwen(m) => {
                let registry_key = m.info().registry_key;
                let pool = text_embedding_pool();

                // Check if workers exist for this model
                if !pool.has_workers(registry_key) {
                    // Cold start: Spawn 2 workers if memory allows
                    let per_worker_mb = m.info().est_memory_allocation_mb;
                    let current_mb = pool.total_memory_mb();
                    let total_system_mb = query_system_memory_mb();
                    let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;
                    
                    // Try spawning 2 workers (cold start policy)
                    let workers_to_spawn = if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
                        2  // Spawn 2 workers
                    } else if current_mb + per_worker_mb <= memory_limit_mb {
                        1  // Degraded: only 1 worker fits
                    } else {
                        return Err(Box::new(PoolError::MemoryExhausted(format!(
                            "Cannot spawn workers for {}. Need {} MB, only {} MB available (80% limit)",
                            registry_key, per_worker_mb, memory_limit_mb.saturating_sub(current_mb)
                        ))) as Box<dyn std::error::Error + Send + Sync>);
                    };
                    
                    // Spawn workers
                    for _ in 0..workers_to_spawn {
                        let m_clone = m.clone();
                        pool.spawn_text_embedding_worker(
                            registry_key,
                            move || {
                                LoadedGteQwenModel::load(&m_clone)
                                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                            },
                            per_worker_mb,
                        ).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                    }
                }

                // Route through pool
                pool.embed_text(registry_key, text, task)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
            
            Self::JinaBert(m) => {
                let registry_key = m.info().registry_key;
                let pool = text_embedding_pool();
                
                if !pool.has_workers(registry_key) {
                    let per_worker_mb = m.info().est_memory_allocation_mb;
                    let current_mb = pool.total_memory_mb();
                    let total_system_mb = query_system_memory_mb();
                    let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;
                    
                    let workers_to_spawn = if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
                        2
                    } else if current_mb + per_worker_mb <= memory_limit_mb {
                        1
                    } else {
                        return Err(Box::new(PoolError::MemoryExhausted(format!(
                            "Cannot spawn workers for {}", registry_key
                        ))) as Box<dyn std::error::Error + Send + Sync>);
                    };
                    
                    for _ in 0..workers_to_spawn {
                        let m_clone = m.clone();
                        pool.spawn_text_embedding_worker(
                            registry_key,
                            move || {
                                LoadedJinaBertModel::load(&m_clone)
                                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                            },
                            per_worker_mb,
                        ).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                    }
                }
                
                pool.embed_text(registry_key, text, task)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
            
            Self::NvEmbed(m) => {
                let registry_key = m.info().registry_key;
                let pool = text_embedding_pool();
                
                if !pool.has_workers(registry_key) {
                    let per_worker_mb = m.info().est_memory_allocation_mb;
                    let current_mb = pool.total_memory_mb();
                    let total_system_mb = query_system_memory_mb();
                    let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;
                    
                    let workers_to_spawn = if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
                        2
                    } else if current_mb + per_worker_mb <= memory_limit_mb {
                        1
                    } else {
                        return Err(Box::new(PoolError::MemoryExhausted(format!(
                            "Cannot spawn workers for {}", registry_key
                        ))) as Box<dyn std::error::Error + Send + Sync>);
                    };
                    
                    for _ in 0..workers_to_spawn {
                        let m_clone = m.clone();
                        pool.spawn_text_embedding_worker(
                            registry_key,
                            move || {
                                LoadedNvEmbedModel::load(&m_clone)
                                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                            },
                            per_worker_mb,
                        ).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                    }
                }
                
                pool.embed_text(registry_key, text, task)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
            
            Self::Stella(m) => {
                let registry_key = m.info().registry_key;
                let pool = text_embedding_pool();
                
                if !pool.has_workers(registry_key) {
                    let per_worker_mb = m.info().est_memory_allocation_mb;
                    let current_mb = pool.total_memory_mb();
                    let total_system_mb = query_system_memory_mb();
                    let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;
                    
                    let workers_to_spawn = if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
                        2
                    } else if current_mb + per_worker_mb <= memory_limit_mb {
                        1
                    } else {
                        return Err(Box::new(PoolError::MemoryExhausted(format!(
                            "Cannot spawn workers for {}", registry_key
                        ))) as Box<dyn std::error::Error + Send + Sync>);
                    };
                    
                    for _ in 0..workers_to_spawn {
                        let m_clone = m.clone();
                        pool.spawn_text_embedding_worker(
                            registry_key,
                            move || {
                                LoadedStellaModel::load(&m_clone)
                                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                            },
                            per_worker_mb,
                        ).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                    }
                }
                
                pool.embed_text(registry_key, text, task)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
            
            Self::Bert(m) => {
                let registry_key = m.info().registry_key;
                let pool = text_embedding_pool();
                
                if !pool.has_workers(registry_key) {
                    let per_worker_mb = m.info().est_memory_allocation_mb;
                    let current_mb = pool.total_memory_mb();
                    let total_system_mb = query_system_memory_mb();
                    let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;
                    
                    let workers_to_spawn = if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
                        2
                    } else if current_mb + per_worker_mb <= memory_limit_mb {
                        1
                    } else {
                        return Err(Box::new(PoolError::MemoryExhausted(format!(
                            "Cannot spawn workers for {}", registry_key
                        ))) as Box<dyn std::error::Error + Send + Sync>);
                    };
                    
                    for _ in 0..workers_to_spawn {
                        let m_clone = m.clone();
                        pool.spawn_text_embedding_worker(
                            registry_key,
                            move || {
                                LoadedBertModel::load(&m_clone)
                                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                            },
                            per_worker_mb,
                        ).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                    }
                }
                
                pool.embed_text(registry_key, text, task)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
        }
    }

    fn batch_embed(&self, texts: &[String], task: Option<String>)
        -> std::result::Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::GteQwen(m) => {
                let registry_key = m.info().registry_key;
                let pool = text_embedding_pool();
                
                // Check if workers exist for this model
                if !pool.has_workers(registry_key) {
                    // Cold start: Spawn 2 workers if memory allows
                    let per_worker_mb = m.info().est_memory_allocation_mb;
                    let current_mb = pool.total_memory_mb();
                    let total_system_mb = query_system_memory_mb();
                    let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;
                    
                    let workers_to_spawn = if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
                        2
                    } else if current_mb + per_worker_mb <= memory_limit_mb {
                        1
                    } else {
                        return Err(Box::new(PoolError::MemoryExhausted(format!(
                            "Cannot spawn workers for {}", registry_key
                        ))) as Box<dyn std::error::Error + Send + Sync>);
                    };
                    
                    for _ in 0..workers_to_spawn {
                        let m_clone = m.clone();
                        pool.spawn_text_embedding_worker(
                            registry_key,
                            move || {
                                LoadedGteQwenModel::load(&m_clone)
                                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                            },
                            per_worker_mb,
                        ).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                    }
                }
                
                pool.batch_embed_text(registry_key, texts, task)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
            Self::JinaBert(m) => {
                let registry_key = m.info().registry_key;
                let pool = text_embedding_pool();
                
                if !pool.has_workers(registry_key) {
                    let per_worker_mb = m.info().est_memory_allocation_mb;
                    let current_mb = pool.total_memory_mb();
                    let total_system_mb = query_system_memory_mb();
                    let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;
                    
                    let workers_to_spawn = if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
                        2
                    } else if current_mb + per_worker_mb <= memory_limit_mb {
                        1
                    } else {
                        return Err(Box::new(PoolError::MemoryExhausted(format!(
                            "Cannot spawn workers for {}", registry_key
                        ))) as Box<dyn std::error::Error + Send + Sync>);
                    };
                    
                    for _ in 0..workers_to_spawn {
                        let m_clone = m.clone();
                        pool.spawn_text_embedding_worker(
                            registry_key,
                            move || {
                                LoadedJinaBertModel::load(&m_clone)
                                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                            },
                            per_worker_mb,
                        ).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                    }
                }
                
                pool.batch_embed_text(registry_key, texts, task)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
            Self::NvEmbed(m) => {
                let registry_key = m.info().registry_key;
                let pool = text_embedding_pool();
                
                if !pool.has_workers(registry_key) {
                    let per_worker_mb = m.info().est_memory_allocation_mb;
                    let current_mb = pool.total_memory_mb();
                    let total_system_mb = query_system_memory_mb();
                    let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;
                    
                    let workers_to_spawn = if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
                        2
                    } else if current_mb + per_worker_mb <= memory_limit_mb {
                        1
                    } else {
                        return Err(Box::new(PoolError::MemoryExhausted(format!(
                            "Cannot spawn workers for {}", registry_key
                        ))) as Box<dyn std::error::Error + Send + Sync>);
                    };
                    
                    for _ in 0..workers_to_spawn {
                        let m_clone = m.clone();
                        pool.spawn_text_embedding_worker(
                            registry_key,
                            move || {
                                LoadedNvEmbedModel::load(&m_clone)
                                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                            },
                            per_worker_mb,
                        ).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                    }
                }
                
                pool.batch_embed_text(registry_key, texts, task)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
            Self::Stella(m) => {
                let registry_key = m.info().registry_key;
                let pool = text_embedding_pool();
                
                if !pool.has_workers(registry_key) {
                    let per_worker_mb = m.info().est_memory_allocation_mb;
                    let current_mb = pool.total_memory_mb();
                    let total_system_mb = query_system_memory_mb();
                    let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;
                    
                    let workers_to_spawn = if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
                        2
                    } else if current_mb + per_worker_mb <= memory_limit_mb {
                        1
                    } else {
                        return Err(Box::new(PoolError::MemoryExhausted(format!(
                            "Cannot spawn workers for {}", registry_key
                        ))) as Box<dyn std::error::Error + Send + Sync>);
                    };
                    
                    for _ in 0..workers_to_spawn {
                        let m_clone = m.clone();
                        pool.spawn_text_embedding_worker(
                            registry_key,
                            move || {
                                LoadedStellaModel::load(&m_clone)
                                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                            },
                            per_worker_mb,
                        ).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                    }
                }
                
                pool.batch_embed_text(registry_key, texts, task)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
            Self::Bert(m) => {
                let registry_key = m.info().registry_key;
                let pool = text_embedding_pool();
                
                if !pool.has_workers(registry_key) {
                    let per_worker_mb = m.info().est_memory_allocation_mb;
                    let current_mb = pool.total_memory_mb();
                    let total_system_mb = query_system_memory_mb();
                    let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;
                    
                    let workers_to_spawn = if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
                        2
                    } else if current_mb + per_worker_mb <= memory_limit_mb {
                        1
                    } else {
                        return Err(Box::new(PoolError::MemoryExhausted(format!(
                            "Cannot spawn workers for {}", registry_key
                        ))) as Box<dyn std::error::Error + Send + Sync>);
                    };
                    
                    for _ in 0..workers_to_spawn {
                        let m_clone = m.clone();
                        pool.spawn_text_embedding_worker(
                            registry_key,
                            move || {
                                LoadedBertModel::load(&m_clone)
                                    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                            },
                            per_worker_mb,
                        ).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                    }
                }
                
                pool.batch_embed_text(registry_key, texts, task)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
        }
    }
    
    fn embedding_dimension(&self) -> usize {
        match self {
            Self::Stella(m) => m.embedding_dimension(),
            Self::Bert(m) => m.embedding_dimension(),
            Self::GteQwen(m) => m.embedding_dimension(),
            Self::JinaBert(m) => m.embedding_dimension(),
            Self::NvEmbed(m) => m.embedding_dimension(),
        }
    }
}

impl ImageEmbeddingCapable for ImageEmbeddingModel {
    fn embed_image(&self, image_path: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::ClipVision(m) => m.embed_image(image_path)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        }
    }
    
    fn embed_image_url(&self, url: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::ClipVision(m) => m.embed_image_url(url)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        }
    }
    
    fn embed_image_base64(&self, base64_data: &str) 
        -> std::result::Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::ClipVision(m) => m.embed_image_base64(base64_data)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        }
    }
    
    fn batch_embed_images(&self, image_paths: Vec<&str>) 
        -> std::result::Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::ClipVision(m) => m.batch_embed_images(image_paths)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        }
    }
    
    fn embedding_dimension(&self) -> usize {
        match self {
            Self::ClipVision(m) => m.embedding_dimension(),
        }
    }
}

impl VisionCapable for VisionModel {
    fn describe_image(&self, image_path: &str, query: &str)
        -> AsyncStream<CandleStringChunk> {
        match self {
            Self::LLaVA(m) => m.describe_image(image_path, query),
        }
    }

    fn describe_url(&self, url: &str, query: &str)
        -> AsyncStream<CandleStringChunk> {
        match self {
            Self::LLaVA(m) => m.describe_url(url, query),
        }
    }
}

impl TextToImageCapable for TextToImageModel {
    fn generate_image(
        &self,
        prompt: &str,
        config: &ImageGenerationConfig,
        device: &Device,
    ) -> AsyncStream<ImageGenerationChunk> {
        match self {
            Self::FluxSchnell(m) => m.generate_image(prompt, config, device),
            Self::StableDiffusion35Turbo(m) => m.generate_image(prompt, config, device),
        }
    }
    
    fn registry_key(&self) -> &str {
        match self {
            Self::FluxSchnell(m) => m.registry_key(),
            Self::StableDiffusion35Turbo(m) => m.registry_key(),
        }
    }
}

//==============================================================================
// REGISTRY STORAGE
//==============================================================================

static TEXT_TO_TEXT_REGISTRY: LazyLock<HashMap<&'static str, TextToTextModel>> = 
    LazyLock::new(|| {
        let mut map = HashMap::new();
        
        let model = Arc::new(CandleKimiK2Model::default());
        let key = model.info().registry_key;
        map.insert(key, TextToTextModel::KimiK2(model));
        
        let model = Arc::new(CandleQwen3CoderModel::default());
        let key = model.info().registry_key;
        map.insert(key, TextToTextModel::Qwen3Coder(model));
        
        let model = Arc::new(CandlePhi4ReasoningModel::default());
        let key = model.info().registry_key;
        map.insert(key, TextToTextModel::Phi4Reasoning(model));
        
        map
    });

static TEXT_EMBEDDING_REGISTRY: LazyLock<HashMap<&'static str, TextEmbeddingModel>> = 
    LazyLock::new(|| {
        let mut map = HashMap::new();
        
        let model = Arc::new(StellaEmbeddingModel::default());
        let key = model.info().registry_key;
        map.insert(key, TextEmbeddingModel::Stella(model));
        
        let model = Arc::new(CandleBertEmbeddingModel::default());
        let key = model.info().registry_key;
        map.insert(key, TextEmbeddingModel::Bert(model));
        
        let model = Arc::new(CandleGteQwenEmbeddingModel::default());
        let key = model.info().registry_key;
        map.insert(key, TextEmbeddingModel::GteQwen(model));
        
        let model = Arc::new(CandleJinaBertEmbeddingModel::default());
        let key = model.info().registry_key;
        map.insert(key, TextEmbeddingModel::JinaBert(model));
        
        let model = Arc::new(CandleNvEmbedEmbeddingModel::default());
        let key = model.info().registry_key;
        map.insert(key, TextEmbeddingModel::NvEmbed(model));
        
        map
    });

// IMAGE_EMBEDDING_REGISTRY: Empty because ClipVision requires local model files, not HF downloads
// Use runtime registration after downloading weights manually
static IMAGE_EMBEDDING_REGISTRY: LazyLock<HashMap<&'static str, ImageEmbeddingModel>> = 
    LazyLock::new(HashMap::new);

static IMAGE_EMBEDDING_RUNTIME: OnceLock<RwLock<HashMap<String, ImageEmbeddingModel>>> = OnceLock::new();

// TEXT_TO_IMAGE_REGISTRY: Empty because Flux/SD require local model files, not HF downloads  
// Use runtime registration after downloading weights manually
static TEXT_TO_IMAGE_REGISTRY: LazyLock<HashMap<&'static str, TextToImageModel>> = 
    LazyLock::new(HashMap::new);

static TEXT_TO_IMAGE_RUNTIME: OnceLock<RwLock<HashMap<String, TextToImageModel>>> = OnceLock::new();

static VISION_REGISTRY: LazyLock<HashMap<&'static str, VisionModel>> = 
    LazyLock::new(|| {
        let mut map = HashMap::new();
        
        let model = Arc::new(LLaVAModel::default());
        let key = model.info().registry_key;
        map.insert(key, VisionModel::LLaVA(model));
        
        map
    });

static MODEL_REGISTRY: LazyLock<HashMap<&'static str, AnyModel>> = 
    LazyLock::new(|| {
        let mut map = HashMap::new();
        
        // Populate from all capability registries
        for (key, model) in TEXT_TO_TEXT_REGISTRY.iter() {
            map.insert(*key, AnyModel::TextToText(model.clone()));
        }
        
        for (key, model) in TEXT_EMBEDDING_REGISTRY.iter() {
            map.insert(*key, AnyModel::TextEmbedding(model.clone()));
        }
        
        for (key, model) in IMAGE_EMBEDDING_REGISTRY.iter() {
            map.insert(*key, AnyModel::ImageEmbedding(model.clone()));
        }
        
        for (key, model) in TEXT_TO_IMAGE_REGISTRY.iter() {
            map.insert(*key, AnyModel::TextToImage(model.clone()));
        }
        
        for (key, model) in VISION_REGISTRY.iter() {
            map.insert(*key, AnyModel::Vision(model.clone()));
        }
        
        map
    });

//==============================================================================
// PUBLIC API
//==============================================================================

/// Generic getter that returns concrete enum types
///
/// This is the PRIMARY API for getting models from the registry.
/// Returns the actual concrete enum type (TextToTextModel, TextEmbeddingModel, etc.)
/// instead of an opaque `impl Trait`.
///
/// # Type Parameter
/// * `T` - The concrete enum type to return (TextToTextModel, TextEmbeddingModel, etc.)
///
/// # Example
/// ```rust
/// use paraphym_candle::capability::registry::{self, TextToTextModel, TextEmbeddingModel};
///
/// let text_model: TextToTextModel = registry::get("unsloth/Kimi-K2-Instruct-GGUF").unwrap();
/// let embed_model: TextEmbeddingModel = registry::get("dunzhang/stella_en_1.5B_v5").unwrap();
/// ```
pub fn get<T>(registry_key: &str) -> Option<T>
where
    T: FromRegistry,
{
    T::from_registry(registry_key)
}

/// Trait for types that can be retrieved from the registry
pub trait FromRegistry: Sized {
    fn from_registry(registry_key: &str) -> Option<Self>;
}

impl FromRegistry for TextToTextModel {
    fn from_registry(registry_key: &str) -> Option<Self> {
        TEXT_TO_TEXT_REGISTRY.get(registry_key).cloned()
    }
}

impl FromRegistry for TextEmbeddingModel {
    fn from_registry(registry_key: &str) -> Option<Self> {
        TEXT_EMBEDDING_REGISTRY.get(registry_key).cloned()
    }
}

impl FromRegistry for ImageEmbeddingModel {
    fn from_registry(registry_key: &str) -> Option<Self> {
        IMAGE_EMBEDDING_REGISTRY.get(registry_key).cloned()
    }
}

impl FromRegistry for TextToImageModel {
    fn from_registry(registry_key: &str) -> Option<Self> {
        TEXT_TO_IMAGE_REGISTRY.get(registry_key).cloned()
    }
}

impl FromRegistry for VisionModel {
    fn from_registry(registry_key: &str) -> Option<Self> {
        VISION_REGISTRY.get(registry_key).cloned()
    }
}

impl FromRegistry for AnyModel {
    fn from_registry(registry_key: &str) -> Option<Self> {
        MODEL_REGISTRY.get(registry_key).cloned()
    }
}

/// Get a text-to-text model by registry_key
///
/// Returns an enum that implements both CandleModel and TextToTextCapable.
///
/// # Example
/// ```rust
/// use paraphym_candle::capability::registry;
///
/// if let Some(model) = registry::get_text_to_text("unsloth/Kimi-K2-Instruct-GGUF") {
///     // model implements TextToTextCapable
/// }
/// ```
pub fn get_text_to_text(registry_key: &str) -> Option<impl TextToTextCapable + CandleModel> {
    TEXT_TO_TEXT_REGISTRY.get(registry_key).cloned()
}

/// Get a text embedding model by registry_key
///
/// Returns an enum that implements both CandleModel and TextEmbeddingCapable.
pub fn get_text_embedding(registry_key: &str) -> Option<impl TextEmbeddingCapable + CandleModel> {
    TEXT_EMBEDDING_REGISTRY.get(registry_key).cloned()
}

/// Get an image embedding model by registry_key
///
/// Returns an enum that implements both CandleModel and ImageEmbeddingCapable.
pub fn get_image_embedding(registry_key: &str) -> Option<impl ImageEmbeddingCapable + CandleModel> {
    IMAGE_EMBEDDING_REGISTRY.get(registry_key).cloned()
}

/// Get a text-to-image model by registry_key
///
/// Returns an enum that implements both CandleModel and TextToImageCapable.
pub fn get_text_to_image(registry_key: &str) -> Option<impl TextToImageCapable + CandleModel> {
    TEXT_TO_IMAGE_REGISTRY.get(registry_key).cloned()
}

/// Get a vision model by registry_key
///
/// Returns an enum that implements both CandleModel and VisionCapable.
pub fn get_vision(registry_key: &str) -> Option<impl VisionCapable + CandleModel> {
    VISION_REGISTRY.get(registry_key).cloned()
}

/// Get any model by registry_key
///
/// Returns the unified AnyModel enum that implements CandleModel.
/// Use this for generic model access when capability doesn't matter.
pub fn get_model(registry_key: &str) -> Option<impl CandleModel> {
    MODEL_REGISTRY.get(registry_key).cloned()
}

/// Get a model by provider and name (legacy compatibility)
///
/// Searches through all registered models to find one matching provider and name.
/// This maintains compatibility with the old registry API.
///
/// # Example
/// ```rust
/// use paraphym_candle::capability::registry;
///
/// if let Some(model) = registry::get_by_provider_and_name("candle-kimi", "kimi-k2-instruct") {
///     // model is AnyModel enum
/// }
/// ```
pub fn get_by_provider_and_name(provider: &str, name: &str) -> Option<AnyModel> {
    MODEL_REGISTRY.iter()
        .find(|(_, model)| {
            let info = model.info();
            info.provider_str() == provider && info.name() == name
        })
        .map(|(_, model)| model.clone())
}

/// Count models by provider
///
/// Returns a vector of (provider_name, model_count) tuples.
/// Useful for determining default provider based on model availability.
pub fn count_models_by_provider() -> Vec<(&'static str, usize)> {
    let mut counts = std::collections::HashMap::new();
    
    for (_key, model) in MODEL_REGISTRY.iter() {
        let provider = model.info().provider_str();
        *counts.entry(provider).or_insert(0) += 1;
    }
    
    counts.into_iter().collect()
}

/// Get all registered model registry keys
///
/// Returns a vector of all `registry_key` values for models in the registry.
///
/// # Example
/// ```rust
/// use paraphym_candle::capability::registry;
///
/// for key in registry::all_registry_keys() {
///     println!("Registered: {}", key);
/// }
/// ```
pub fn all_registry_keys() -> Vec<&'static str> {
    MODEL_REGISTRY.iter().map(|(key, _model)| *key).collect()
}

/// Check if a registry_key is registered
///
/// # Example
/// ```rust
/// use paraphym_candle::capability::registry;
///
/// if registry::has_model("unsloth/Kimi-K2-Instruct-GGUF") {
///     println!("Kimi K2 is available");
/// }
/// ```
pub fn has_model(registry_key: &str) -> bool {
    MODEL_REGISTRY.contains_key(registry_key)
}

//==============================================================================
// RUNTIME REGISTRATION
//==============================================================================

/// Register an image embedding model at runtime
///
/// Use this for models that require explicit configuration (e.g., ClipVision)
/// and cannot be statically initialized.
///
/// # Example
/// ```rust
/// use paraphym_candle::capability::registry;
/// use paraphym_candle::capability::image_embedding::ClipVisionEmbeddingModel;
///
/// let model = ClipVisionEmbeddingModel::from_model(clip_model, 512);
/// registry::register_image_embedding("my-clip-model", model);
/// ```
pub fn register_image_embedding(key: impl Into<String>, model: ImageEmbeddingModel) {
    let runtime = IMAGE_EMBEDDING_RUNTIME.get_or_init(|| RwLock::new(HashMap::new()));
    if let Ok(mut map) = runtime.write() {
        map.insert(key.into(), model);
    }
}

/// Register a text-to-image model at runtime
///
/// Use this for models that require weight downloads (e.g., FluxSchnell)
/// and cannot be statically initialized.
///
/// # Example
/// ```rust
/// use paraphym_candle::capability::registry;
/// use paraphym_candle::capability::text_to_image::FluxSchnell;
///
/// let model = FluxSchnell::from_pretrained().unwrap();
/// registry::register_text_to_image("flux-schnell", model);
/// ```
pub fn register_text_to_image(key: impl Into<String>, model: TextToImageModel) {
    let runtime = TEXT_TO_IMAGE_RUNTIME.get_or_init(|| RwLock::new(HashMap::new()));
    if let Ok(mut map) = runtime.write() {
        map.insert(key.into(), model);
    }
}

/// Get an image embedding model from runtime registry
///
/// Checks runtime registry first, then falls back to static registry.
pub fn get_image_embedding_runtime(key: &str) -> Option<ImageEmbeddingModel> {
    // Check runtime registry first
    if let Some(runtime) = IMAGE_EMBEDDING_RUNTIME.get() {
        if let Ok(map) = runtime.read() {
            if let Some(model) = map.get(key) {
                return Some(model.clone());
            }
        }
    }
    
    // Fall back to static registry
    IMAGE_EMBEDDING_REGISTRY.get(key).cloned()
}

/// Get a text-to-image model from runtime registry
///
/// Checks runtime registry first, then falls back to static registry.
pub fn get_text_to_image_runtime(key: &str) -> Option<TextToImageModel> {
    // Check runtime registry first
    if let Some(runtime) = TEXT_TO_IMAGE_RUNTIME.get() {
        if let Ok(map) = runtime.read() {
            if let Some(model) = map.get(key) {
                return Some(model.clone());
            }
        }
    }
    
    // Fall back to static registry
    TEXT_TO_IMAGE_REGISTRY.get(key).cloned()
}

/// Get the total number of registered models
///
/// Returns the count of all models in the registry across all capabilities.
pub fn model_count() -> usize {
    MODEL_REGISTRY.len()
}

//==============================================================================
// HELPER FUNCTIONS
//==============================================================================

/// Query total system memory in MB
fn query_system_memory_mb() -> usize {
    use sysinfo::System;
    let mut sys = System::new_all();
    sys.refresh_memory();
    (sys.total_memory() / 1024 / 1024) as usize
}
