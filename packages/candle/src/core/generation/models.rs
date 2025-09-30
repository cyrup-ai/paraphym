//! Model integration and wrapper functionality  
//!
//! This module provides model wrappers and integration points for Candle models,
//! consolidating duplicate CandleLlamaModel definitions and providing a unified
//! interface for model operations.

use std::sync::Arc;
use std::path::{Path, PathBuf};
use std::collections::HashSet;

use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::llama::{Cache, Llama};
use candle_transformers::models::quantized_llama;
use candle_core::quantized::gguf_file;

use super::types::CandleResult;
use crate::core::ModelConfig as CandleConfig;
use crate::core::model_config::ModelArchitecture;
use crate::domain::model::error::CandleModelError;

/// Trait for Candle model implementations
///
/// Defines the interface that all Candle models must implement for
/// integration with the text generation system.
pub trait CandleModel: Send + Sync {
    /// Perform a forward pass through the model
    fn forward(&mut self, input: &Tensor, position: usize) -> CandleResult<Tensor>;

    /// Get the model's device
    fn device(&self) -> &Device;

    /// Get the model's vocabulary size
    fn vocab_size(&self) -> usize;

    /// Get model configuration if available
    fn config(&self) -> Option<&CandleConfig> {
        None
    }
}

/// Llama model wrapper for Candle integration
///
/// Consolidates duplicate CandleLlamaModel definitions into a single,
/// well-defined implementation with proper error handling and configuration.
#[derive(Debug)]
pub struct CandleLlamaModel {
    /// The underlying Llama model
    model: Llama,

    /// KV cache for the model
    cache: Cache,

    /// Device the model is loaded on
    device: Device,

    /// Model configuration
    config: Arc<CandleConfig>,

    /// Model vocabulary size
    vocab_size: usize,
}
impl CandleLlamaModel {
    /// Create new CandleLlamaModel
    pub fn new(
        model: Llama,
        cache: Cache,
        device: Device,
        config: Arc<CandleConfig>,
        vocab_size: usize,
    ) -> Self {
        Self {
            model,
            cache,
            device,
            config,
            vocab_size,
        }
    }

    /// Load a Llama model from the specified path
    pub fn from_path<P: AsRef<std::path::Path>>(
        model_path: P,
        device: Device,
        config: Arc<CandleConfig>,
    ) -> CandleResult<Self> {
        let model_path = model_path.as_ref();
        
        // Extract the LlamaConfig from ModelConfig's architecture
        let llama_config = match &config.architecture {
            ModelArchitecture::Llama(llama_cfg) => llama_cfg,
            _ => return Err(CandleModelError::InvalidConfiguration(
                "Expected Llama architecture in config".into()
            )),
        };
        
        // Determine if single or multi-file model
        let safetensors_files = if model_path.is_file() {
            // Single file provided directly
            vec![model_path.to_path_buf()]
        } else if model_path.is_dir() {
            // Check for index file first (multi-file model)
            let index_path = model_path.join("model.safetensors.index.json");
            if index_path.exists() {
                discover_multi_file_model(&index_path)?
            } else {
                // Single file in directory
                let single_file = model_path.join("model.safetensors");
                if single_file.exists() {
                    vec![single_file]
                } else {
                    return Err(CandleModelError::InvalidConfiguration(
                        format!("No model files found in {}", model_path.display()).into()
                    ));
                }
            }
        } else {
            return Err(CandleModelError::InvalidConfiguration(
                format!("Invalid model path: {}", model_path.display()).into()
            ));
        };
        
        // Load model weights using memory-mapped safetensors
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&safetensors_files, config.dtype, &device)
                .map_err(|e| CandleModelError::InvalidConfiguration(
                    format!("Failed to load model weights: {}", e).into()
                ))?
        };
        
        // Create KV cache for efficient inference
        let cache = Cache::new(true, config.dtype, llama_config, &device)
            .map_err(|e| CandleModelError::InvalidConfiguration(
                format!("Failed to create cache: {}", e).into()
            ))?;
        
        // Load the Llama model from weights
        let model = Llama::load(vb, llama_config)
            .map_err(|e| CandleModelError::InvalidConfiguration(
                format!("Failed to load Llama model: {}", e).into()
            ))?;
        
        let vocab_size = config.vocab_size;
        
        Ok(Self::new(model, cache, device, config, vocab_size))
    }

    /// Get the underlying Llama model
    pub fn inner(&self) -> &Llama {
        &self.model
    }

    /// Get the underlying Llama model mutably
    pub fn inner_mut(&mut self) -> &mut Llama {
        &mut self.model
    }

    /// Get model configuration
    pub fn config(&self) -> &CandleConfig {
        &self.config
    }
}
impl CandleModel for CandleLlamaModel {
    fn forward(&mut self, input: &Tensor, position: usize) -> CandleResult<Tensor> {
        self.model
            .forward(input, position, &mut self.cache)
            .map_err(Into::into)
    }

    fn device(&self) -> &Device {
        &self.device
    }

    fn vocab_size(&self) -> usize {
        self.vocab_size
    }

    fn config(&self) -> Option<&CandleConfig> {
        Some(&self.config)
    }
}

/// Quantized Llama model wrapper for GGUF models
///
/// This wrapper handles quantized models loaded from GGUF files using
/// the candle_transformers::models::quantized_llama module.
#[derive(Debug)]
pub struct CandleQuantizedLlamaModel {
    /// The underlying quantized model weights
    model_weights: quantized_llama::ModelWeights,

    /// Device the model is loaded on
    device: Device,

    /// Model configuration
    config: Arc<CandleConfig>,

    /// Model vocabulary size
    vocab_size: usize,
}

impl CandleQuantizedLlamaModel {
    /// Create new CandleQuantizedLlamaModel
    pub fn new(
        model_weights: quantized_llama::ModelWeights,
        device: Device,
        config: Arc<CandleConfig>,
        vocab_size: usize,
    ) -> Self {
        Self {
            model_weights,
            device,
            config,
            vocab_size,
        }
    }

    /// Load a quantized Llama model from a GGUF file
    pub fn from_gguf_path<P: AsRef<std::path::Path>>(
        model_path: P,
        device: Device,
        config: Arc<CandleConfig>,
    ) -> CandleResult<Self> {
        let mut file = std::fs::File::open(&model_path)
            .map_err(|e| crate::domain::model::error::CandleModelError::InvalidConfiguration(
                format!("Failed to open GGUF file: {}", e).into()
            ))?;
        
        let gguf_content = gguf_file::Content::read(&mut file)
            .map_err(|e| crate::domain::model::error::CandleModelError::InvalidConfiguration(
                format!("Failed to read GGUF file: {}", e).into()
            ))?;
        
        let model_weights = quantized_llama::ModelWeights::from_gguf(gguf_content, &mut file, &device)
            .map_err(|e| crate::domain::model::error::CandleModelError::InvalidConfiguration(
                format!("Failed to load model weights from GGUF: {}", e).into()
            ))?;
        
        // Extract vocab size from config or use default
        let vocab_size = config.vocab_size;
        
        Ok(Self::new(model_weights, device, config, vocab_size))
    }

    /// Get the underlying model weights
    pub fn model_weights(&self) -> &quantized_llama::ModelWeights {
        &self.model_weights
    }

    /// Get the underlying model weights mutably
    pub fn model_weights_mut(&mut self) -> &mut quantized_llama::ModelWeights {
        &mut self.model_weights
    }
}

impl CandleModel for CandleQuantizedLlamaModel {
    fn forward(&mut self, input: &Tensor, position: usize) -> CandleResult<Tensor> {
        self.model_weights
            .forward(input, position)
            .map_err(Into::into)
    }

    fn device(&self) -> &Device {
        &self.device
    }

    fn vocab_size(&self) -> usize {
        self.vocab_size
    }

    fn config(&self) -> Option<&CandleConfig> {
        Some(&self.config)
    }
}

/// Model factory for creating different model types
pub struct ModelFactory;

impl ModelFactory {
    /// Create a quantized Llama model from GGUF file path
    pub fn create_quantized_llama<P: AsRef<std::path::Path>>(
        model_path: P,
        config: Arc<CandleConfig>,
        device: Device,
    ) -> CandleResult<CandleQuantizedLlamaModel> {
        CandleQuantizedLlamaModel::from_gguf_path(model_path, device, config)
    }

    /// Create a Llama model from configuration (placeholder for regular models)
    pub fn create_llama(
        _config: Arc<CandleConfig>,
        _device: Device,
    ) -> CandleResult<CandleLlamaModel> {
        Err(crate::domain::model::error::CandleModelError::OperationNotSupported(
            "Regular Llama model loading not implemented - use create_quantized_llama for GGUF models".into()
        ))
    }

    /// Create a model from a model type string
    pub fn create_from_type(
        model_type: &str,
        config: Arc<CandleConfig>,
        device: Device,
    ) -> CandleResult<Box<dyn CandleModel>> {
        match model_type.to_lowercase().as_str() {
            "llama" => {
                let model = Self::create_llama(config, device)?;
                Ok(Box::new(model))
            }
            _ => Err(
                crate::domain::model::error::CandleModelError::OperationNotSupported(
                    format!("Unsupported model type: {}", model_type).into(),
                ),
            ),
        }
    }
}

/// Helper function to discover model files from a multi-file model index
///
/// Parses a `model.safetensors.index.json` file to find all weight files
/// referenced in the weight_map, enabling loading of large models split
/// across multiple safetensors files.
fn discover_multi_file_model(index_path: &Path) -> CandleResult<Vec<PathBuf>> {
    let file = std::fs::File::open(index_path)
        .map_err(|e| CandleModelError::InvalidConfiguration(
            format!("Failed to open index file: {}", e).into()
        ))?;
    
    let json: serde_json::Value = serde_json::from_reader(&file)
        .map_err(|e| CandleModelError::InvalidConfiguration(
            format!("Failed to parse index JSON: {}", e).into()
        ))?;
    
    let weight_map = json.get("weight_map")
        .and_then(|v| v.as_object())
        .ok_or_else(|| CandleModelError::InvalidConfiguration(
            "No weight_map in index file".into()
        ))?;
    
    let base_dir = index_path.parent()
        .ok_or_else(|| CandleModelError::InvalidConfiguration(
            format!("Invalid index path: {}", index_path.display()).into()
        ))?;
    
    let mut files = HashSet::new();
    
    for value in weight_map.values() {
        if let Some(filename) = value.as_str() {
            files.insert(base_dir.join(filename));
        }
    }
    
    Ok(files.into_iter().collect())
}
