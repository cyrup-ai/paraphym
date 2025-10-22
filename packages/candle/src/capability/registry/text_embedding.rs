//! TextEmbeddingCapable trait implementation for TextEmbeddingModel

use super::pool::capabilities::text_embedding_pool;
use super::pool::core::{PoolError, ensure_workers_spawned_adaptive};
use crate::capability::traits::TextEmbeddingCapable;
use crate::domain::model::traits::CandleModel;
use std::sync::Arc;

// LoadedModel imports
use crate::capability::text_embedding::{
    bert::LoadedBertModel, gte_qwen::LoadedGteQwenModel, jina_bert::LoadedJinaBertModel,
    nvembed::LoadedNvEmbedModel, stella::LoadedStellaModel,
};

use super::enums::TextEmbeddingModel;

impl TextEmbeddingCapable for TextEmbeddingModel {
    fn embed(
        &self,
        text: &str,
        task: Option<String>,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = std::result::Result<
                        Vec<f32>,
                        Box<dyn std::error::Error + Send + Sync>,
                    >,
                > + Send
                + '_,
        >,
    > {
        let text = text.to_string();
        Box::pin(async move {
            match self {
                Self::GteQwen(m) => spawn_embed_gte_qwen(m, &text, task).await,
                Self::JinaBert(m) => spawn_embed_jina_bert(m, &text, task).await,
                Self::NvEmbed(m) => spawn_embed_nvembed(m, &text, task).await,
                Self::Stella(m) => spawn_embed_stella(m, &text, task).await,
                Self::Bert(m) => spawn_embed_bert(m, &text, task).await,
            }
        })
    }

    fn batch_embed(
        &self,
        texts: &[String],
        task: Option<String>,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = std::result::Result<
                        Vec<Vec<f32>>,
                        Box<dyn std::error::Error + Send + Sync>,
                    >,
                > + Send
                + '_,
        >,
    > {
        let texts = texts.to_vec();
        Box::pin(async move {
            match self {
                Self::GteQwen(m) => spawn_batch_embed_gte_qwen(m, &texts, task).await,
                Self::JinaBert(m) => spawn_batch_embed_jina_bert(m, &texts, task).await,
                Self::NvEmbed(m) => spawn_batch_embed_nvembed(m, &texts, task).await,
                Self::Stella(m) => spawn_batch_embed_stella(m, &texts, task).await,
                Self::Bert(m) => spawn_batch_embed_bert(m, &texts, task).await,
            }
        })
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

// Helper macro to eliminate duplication in worker spawning
macro_rules! impl_text_embedding_spawn {
    ($fn_name:ident, $batch_fn_name:ident, $model_ty:ty, $loaded_ty:ty) => {
        async fn $fn_name(
            model: &Arc<$model_ty>,
            text: &str,
            task: Option<String>,
        ) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
            let registry_key = model.info().registry_key;
            let per_worker_mb = model.info().est_memory_allocation_mb;
            let pool = text_embedding_pool();

            log::info!(">>> About to ensure workers for {}", registry_key);
            ensure_workers_spawned_adaptive(
                pool,
                registry_key,
                per_worker_mb,
                pool.config().max_workers_per_model,
                |_, allocation_guard| {
                    let m_clone = model.clone();
                    pool.spawn_text_embedding_worker(
                        registry_key,
                        move || async move {
                            <$loaded_ty>::load(&m_clone)
                                .await
                                .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                        },
                        per_worker_mb,
                        allocation_guard,
                    )
                },
            )
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

            log::info!(">>> Workers ready, calling embed_text for {}", registry_key);
            let result = pool
                .embed_text(registry_key, text, task)
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>);
            log::info!(">>> embed_text returned for {}", registry_key);
            result
        }

        async fn $batch_fn_name(
            model: &Arc<$model_ty>,
            texts: &[String],
            task: Option<String>,
        ) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> {
            let registry_key = model.info().registry_key;
            let per_worker_mb = model.info().est_memory_allocation_mb;
            let pool = text_embedding_pool();

            ensure_workers_spawned_adaptive(
                pool,
                registry_key,
                per_worker_mb,
                pool.config().max_workers_per_model,
                |_, allocation_guard| {
                    let m_clone = model.clone();
                    pool.spawn_text_embedding_worker(
                        registry_key,
                        move || async move {
                            <$loaded_ty>::load(&m_clone)
                                .await
                                .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                        },
                        per_worker_mb,
                        allocation_guard,
                    )
                },
            )
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

            pool.batch_embed_text(registry_key, texts, task)
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        }
    };
}

// Generate functions for each model type
impl_text_embedding_spawn!(
    spawn_embed_gte_qwen,
    spawn_batch_embed_gte_qwen,
    crate::capability::text_embedding::gte_qwen::CandleGteQwenEmbeddingModel,
    LoadedGteQwenModel
);
impl_text_embedding_spawn!(
    spawn_embed_jina_bert,
    spawn_batch_embed_jina_bert,
    crate::capability::text_embedding::jina_bert::CandleJinaBertEmbeddingModel,
    LoadedJinaBertModel
);
impl_text_embedding_spawn!(
    spawn_embed_nvembed,
    spawn_batch_embed_nvembed,
    crate::capability::text_embedding::nvembed::CandleNvEmbedEmbeddingModel,
    LoadedNvEmbedModel
);
impl_text_embedding_spawn!(
    spawn_embed_stella,
    spawn_batch_embed_stella,
    crate::capability::text_embedding::stella::StellaEmbeddingModel,
    LoadedStellaModel
);
impl_text_embedding_spawn!(
    spawn_embed_bert,
    spawn_batch_embed_bert,
    crate::capability::text_embedding::bert::CandleBertEmbeddingModel,
    LoadedBertModel
);
