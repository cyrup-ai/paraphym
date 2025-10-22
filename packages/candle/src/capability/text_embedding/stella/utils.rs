//! Shared utilities for Stella embedding model

use candle_core::{DType, Device};
use candle_nn::VarBuilder;
use candle_transformers::models::stella_en_v5::{Config, EmbedDim, ModelVariant};
use std::path::PathBuf;
use tokenizers::{PaddingDirection, PaddingParams, PaddingStrategy, Tokenizer, TruncationParams};

/// Detect best device and dtype
///
/// Returns (Device, DType) where Device is Metal/CUDA/CPU and DType is F16 for CUDA, F32 otherwise
pub(crate) fn detect_device_and_dtype() -> (Device, DType) {
    let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
        log::warn!("Device detection failed: {}. Using CPU.", e);
        Device::Cpu
    });
    let dtype = if device.is_cuda() {
        DType::F16
    } else {
        DType::F32
    };
    (device, dtype)
}

/// Configure tokenizer with variant-specific padding and truncation
///
/// # Padding
/// - Large (1.5B): Left padding with |endoftext|> token
/// - Small (400M): Right padding with default token
pub(crate) fn configure_stella_tokenizer(
    tokenizer: &mut Tokenizer,
    variant: ModelVariant,
    max_length: usize,
) -> Result<(), String> {
    // Variant-specific padding
    match variant {
        ModelVariant::Large => {
            let pad_id = tokenizer
                .token_to_id("<|endoftext|>")
                .ok_or("Tokenizer missing |endoftext|> token")?;
            tokenizer.with_padding(Some(PaddingParams {
                strategy: PaddingStrategy::BatchLongest,
                direction: PaddingDirection::Left,
                pad_to_multiple_of: None,
                pad_id,
                pad_type_id: 0,
                pad_token: "<|endoftext|>".to_string(),
            }));
        }
        ModelVariant::Small => {
            tokenizer.with_padding(Some(PaddingParams {
                strategy: PaddingStrategy::BatchLongest,
                direction: PaddingDirection::Right,
                ..Default::default()
            }));
        }
    }

    // Set truncation if not already set
    if tokenizer.get_truncation().is_none() {
        tokenizer
            .with_truncation(Some(TruncationParams {
                max_length,
                strategy: tokenizers::TruncationStrategy::LongestFirst,
                stride: 0,
                direction: tokenizers::TruncationDirection::Right,
            }))
            .map_err(|e| format!("Failed to set truncation: {}", e))?;
    }

    Ok(())
}

/// Create Stella config based on variant and embedding dimension
pub(crate) fn create_stella_config(variant: ModelVariant, embed_dim: EmbedDim) -> Config {
    match variant {
        ModelVariant::Large => Config::new_1_5_b_v5(embed_dim),
        ModelVariant::Small => Config::new_400_m_v5(embed_dim),
    }
}

/// Load Stella model weights (base + projection head)
///
/// # Safety
/// Uses unsafe mmap - caller must ensure files are valid SafeTensors
pub(crate) fn load_stella_weights(
    base_weights: PathBuf,
    projection_head: PathBuf,
    dtype: DType,
    device: &Device,
) -> Result<(VarBuilder<'static>, VarBuilder<'static>), String> {
    let base_vb = unsafe {
        VarBuilder::from_mmaped_safetensors(&[base_weights], dtype, device)
            .map_err(|e| format!("Failed to load base model weights: {}", e))?
    };

    let embed_vb = unsafe {
        VarBuilder::from_mmaped_safetensors(&[projection_head], DType::F32, device)
            .map_err(|e| format!("Failed to load projection head weights: {}", e))?
    };

    Ok((base_vb, embed_vb))
}
