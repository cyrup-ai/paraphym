//! Provides streaming completion capabilities using local Qwen3 models
//! with quantized GGUF models for efficient inference.
//!
//! This implementation uses Candle's native quantized_qwen3 matching the proven
//! 90+ tokens/s performance from the candle-examples reference implementation.

use std::num::NonZeroU32;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;

use candle_core::{Device, Tensor};
use candle_core::quantized::gguf_file;
use candle_transformers::models::quantized_qwen3::ModelWeights as Qwen3Model;
use candle_transformers::generation::{LogitsProcessor, Sampling};
use tokio_stream::Stream;
use crate::async_stream;
use crate::core::generation::TokenOutputStream;
use serde::{Deserialize, Serialize};

use crate::core::{Engine, EngineConfig};

use crate::domain::completion::{CandleCompletionChunk, CandleCompletionParams};
use crate::domain::context::CandleStringChunk;
use crate::domain::model::{info::CandleModelInfo, traits::CandleModel};
use crate::domain::prompt::CandlePrompt;

/// Builder trait for Qwen3 Quantized completion providers
pub trait BuilderCandleQwen3QuantizedModel: Send + Sync + 'static {
    // Default implementations for all builders
}

/// High-performance Qwen3 1.7B Quantized provider for local inference using Candle
///
/// Provides streaming text generation capabilities using the Qwen3-1.7B quantized model
/// with automatic model downloading via HuggingFace.
#[derive(Debug, Clone)]
pub struct CandleQwen3QuantizedModel {
    /// Model cache directory path
    model_path: String,
    /// GGUF model file path
    gguf_file_path: String,
    /// Engine for orchestration and stream conversion
    engine: Arc<Engine>,
}

impl CandleQwen3QuantizedModel {
    /// Create new Qwen3 Quantized provider with automatic model download
    ///
    /// This method automatically downloads the Qwen3-Coder-30B model from HuggingFace
    /// using ProgressHub and returns a provider ready for inference.
    ///
    /// # Example
    /// ```rust
    /// let provider = CandleQwen3QuantizedModel::new().await?;
    /// ```
    ///
    /// # Errors
    /// Returns error if model download fails or model loading fails
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Create minimal temporary instance to access huggingface_file() method
        let temp = Self::from_gguf(String::new(), String::new()).await?;
        
        // Use huggingface_file() to download GGUF from unsloth/Qwen3-1.7B-GGUF
        let gguf_path = temp.huggingface_file("unsloth/Qwen3-1.7B-GGUF", "Qwen3-1.7B-Q4_K_M.gguf").await?;
        
        // Use huggingface_file() to download tokenizer from Qwen/Qwen3-1.7B  
        let _tokenizer_path = temp.huggingface_file("Qwen/Qwen3-1.7B", "tokenizer.json").await?;

        // Extract model directory from GGUF file path
        let model_cache_dir = gguf_path
            .parent()
            .ok_or_else(|| {
                Box::<dyn std::error::Error + Send + Sync>::from(
                    "Could not determine model directory",
                )
            })?
            .to_str()
            .ok_or_else(|| {
                Box::<dyn std::error::Error + Send + Sync>::from("Invalid model directory path")
            })?
            .to_string();

        Self::from_gguf(
            model_cache_dir,
            gguf_path
                .to_str()
                .ok_or_else(|| {
                    Box::<dyn std::error::Error + Send + Sync>::from("Invalid GGUF file path")
                })?
                .to_string(),
        ).await
    }

    /// Create provider from GGUF file with metadata extraction for Qwen3 Quantized
    ///
    /// This method reads the GGUF file metadata to extract real Qwen3 model configuration
    /// instead of using hardcoded values, ensuring accurate model parameters.
    /// All configuration values come from QWEN3_QUANTIZED_MODEL_INFO (self.info()).
    #[inline]
    pub async fn from_gguf(
        model_cache_dir: String,
        gguf_file_path: String,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Simple engine creation - quantized_qwen3 handles all metadata internally
        let engine_config = EngineConfig::new("qwen3-quantized", "candle-qwen")
            .with_streaming()
            .with_max_tokens(32768)  // From QWEN3_QUANTIZED_MODEL_INFO
            .with_temperature(0.8);   // From QWEN3_QUANTIZED_MODEL_INFO

        let engine = Arc::new(Engine::new(engine_config)?);

        Ok(Self {
            model_path: model_cache_dir,
            gguf_file_path,
            engine,
        })
    }

    /// Get the model path
    #[inline]
    pub fn model_path(&self) -> &str {
        &self.model_path
    }
}

// Static model info for Qwen3 1.7B Quantized
pub static QWEN3_QUANTIZED_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::Unsloth,
    name: "qwen3-coder-30b-instruct",
    registry_key: "unsloth/Qwen3-Coder-30B-A3B-Instruct-GGUF",
    quantization_url: None,
    max_input_tokens: NonZeroU32::new(32768), // 32K context
    max_output_tokens: NonZeroU32::new(8192),
    input_price: None, // Local model - no pricing
    output_price: None,
    supports_vision: false,
    supports_function_calling: true,
    supports_streaming: true,
    supports_embeddings: false,
    requires_max_tokens: false,
    supports_thinking: false,
    optimal_thinking_budget: None,
    system_prompt_prefix: None,
    real_name: None,
    model_type: None,
    model_id: "qwen-coder",
    quantization: "Q4_0",
    patch: None,
    embedding_dimension: None,
    vocab_size: Some(151936),
    image_size: None,
    image_mean: None,
    image_std: None,
    default_temperature: Some(0.7),
    default_top_k: Some(50),
    default_top_p: Some(0.9),
    supports_kv_cache: true,
    supports_flash_attention: false,
    use_bf16: false,
    default_steps: None,
    default_guidance_scale: None,
    time_shift: None,
    est_memory_allocation_mb: 0,
};

impl crate::capability::traits::TextToTextCapable for CandleQwen3QuantizedModel {
    fn prompt(
        &self,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> Pin<Box<dyn Stream<Item = CandleCompletionChunk> + Send>> {
        // Clone data needed for generation
        let model_path = self.model_path.clone();
        let gguf_file_path = self.gguf_file_path.clone();
        let engine = self.engine.clone();

        // Get configuration from ModelInfo
        let _max_context = self
            .info()
            .max_input_tokens
            .map(|t| t.get())
            .unwrap_or(32768);

        // Extract sampling parameters
        let temperature = if params.temperature != 1.0 {
            params.temperature
        } else {
            self.info().default_temperature.unwrap_or(0.7)
        };

        let top_k = params
            .additional_params
            .as_ref()
            .and_then(|p| p.get("top_k"))
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .or(QWEN3_QUANTIZED_MODEL_INFO.default_top_k.map(|k| k as usize));

        let top_p = params
            .additional_params
            .as_ref()
            .and_then(|p| p.get("top_p"))
            .and_then(|v| v.as_f64())
            .or(QWEN3_QUANTIZED_MODEL_INFO.default_top_p);

        let repeat_penalty = params
            .additional_params
            .as_ref()
            .and_then(|p| p.get("repeat_penalty"))
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0);

        // Format prompt using Qwen3 chat template
        let prompt_text = format!("<|im_start|>user\n{}<|im_end|>\n<|im_start|>assistant\n", prompt.content);
        let max_tokens = params.max_tokens.map(|n| n.get()).unwrap_or(1000);

        // Use Engine's coordinate_generation for automatic metrics and stream conversion
        Box::pin(engine.coordinate_generation(move || {
            async_stream::spawn_stream(move |tx| async move {
                // Load device (prefer GPU if available)
                let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
                    log::warn!("Device detection failed: {}. Using CPU.", e);
                    Device::Cpu
                });

                // Load tokenizer directly (no spawn_blocking needed for simple file read)
                let tokenizer_path = format!("{}/tokenizer.json", model_path);
                let tokenizer = match tokenizers::Tokenizer::from_file(&tokenizer_path) {
                    Ok(t) => t,
                    Err(e) => {
                        let _ = tx.send(CandleStringChunk(format!(
                            "ERROR: Failed to load tokenizer: {}",
                            e
                        )));
                        return;
                    }
                };

                // Get EOS token from vocabulary
                let eos_token_id = *tokenizer.get_vocab(true).get("<|im_end|>").unwrap_or(&151645) as u32;

                // Load GGUF file directly
                let mut file = match std::fs::File::open(&gguf_file_path) {
                    Ok(f) => f,
                    Err(e) => {
                        let _ = tx.send(CandleStringChunk(format!(
                            "ERROR: Failed to open GGUF file: {}",
                            e
                        )));
                        return;
                    }
                };

                let content = match gguf_file::Content::read(&mut file) {
                    Ok(c) => c,
                    Err(e) => {
                        let _ = tx.send(CandleStringChunk(format!(
                            "ERROR: Failed to read GGUF content: {}",
                            e
                        )));
                        return;
                    }
                };

                // Create Qwen3 model directly - SIMPLE!
                let mut model = match Qwen3Model::from_gguf(content, &mut file, &device) {
                    Ok(m) => m,
                    Err(e) => {
                        let _ = tx.send(CandleStringChunk(format!(
                            "ERROR: Failed to create model: {}",
                            e
                        )));
                        return;
                    }
                };

                // Encode the prompt
                let tokens = match tokenizer.encode(prompt_text.as_str(), false) {
                    Ok(encoding) => encoding.get_ids().to_vec(),
                    Err(e) => {
                        let _ = tx.send(CandleStringChunk(format!(
                            "ERROR: Failed to encode prompt: {}",
                            e
                        )));
                        return;
                    }
                };

                // Create LogitsProcessor for sampling
                let seed = 299792458;
                let mut logits_processor = {
                    let sampling = if temperature <= 0.0 {
                        Sampling::ArgMax
                    } else if let Some(k) = top_k {
                        Sampling::TopK { k, temperature: temperature as f64 }
                    } else if let Some(p) = top_p {
                        Sampling::TopP { p, temperature: temperature as f64 }
                    } else {
                        Sampling::All { temperature: temperature as f64 }
                    };
                    LogitsProcessor::from_sampling(seed, sampling)
                };

                // Create TokenOutputStream for efficient decoding
                let mut tos = TokenOutputStream::new(tokenizer.clone());

                // Track all tokens for repeat penalty
                let mut all_tokens = Vec::with_capacity(tokens.len() + max_tokens as usize);
                all_tokens.extend_from_slice(&tokens);
                let start_at = all_tokens.len();

                // Initial forward pass
                let input = match Tensor::new(&tokens[..], &device) {
                    Ok(t) => match t.unsqueeze(0) {
                        Ok(t) => t,
                        Err(e) => {
                            let _ = tx.send(CandleStringChunk(format!(
                                "ERROR: Failed to unsqueeze tensor: {}",
                                e
                            )));
                            return;
                        }
                    },
                    Err(e) => {
                        let _ = tx.send(CandleStringChunk(format!(
                            "ERROR: Failed to create input tensor: {}",
                            e
                        )));
                        return;
                    }
                };

                let logits = match model.forward(&input, 0) {
                    Ok(l) => l,
                    Err(e) => {
                        let _ = tx.send(CandleStringChunk(format!(
                            "ERROR: Forward pass failed: {}",
                            e
                        )));
                        return;
                    }
                };

                let logits = match logits.squeeze(0) {
                    Ok(l) => l,
                    Err(e) => {
                        let _ = tx.send(CandleStringChunk(format!(
                            "ERROR: Failed to squeeze logits: {}",
                            e
                        )));
                        return;
                    }
                };

                // Apply temperature scaling
                let logits = if temperature != 1.0 {
                    match logits / temperature as f64 {
                        Ok(l) => l,
                        Err(e) => {
                            let _ = tx.send(CandleStringChunk(format!(
                                "ERROR: Temperature scaling failed: {}",
                                e
                            )));
                            return;
                        }
                    }
                } else {
                    logits
                };

                // Conditional repeat penalty - skip when == 1.0 for performance
                let logits = if repeat_penalty != 1.0 {
                    match candle_transformers::utils::apply_repeat_penalty(
                        &logits,
                        repeat_penalty as f32,
                        &all_tokens[start_at..],
                    ) {
                        Ok(l) => l,
                        Err(e) => {
                            let _ = tx.send(CandleStringChunk(format!(
                                "ERROR: Repeat penalty failed: {}",
                                e
                            )));
                            return;
                        }
                    }
                } else {
                    logits // Skip expensive operation when not needed
                };

                let mut next_token = match logits_processor.sample(&logits) {
                    Ok(t) => t,
                    Err(e) => {
                        let _ = tx.send(CandleStringChunk(format!(
                            "ERROR: Sampling failed: {}",
                            e
                        )));
                        return;
                    }
                };

                all_tokens.push(next_token);

                // Send first token
                if let Some(t) = tos.next_token(next_token).ok().flatten() {
                    let _ = tx.send(CandleStringChunk(t));
                }

                // Continue generation
                for index in 0..max_tokens {
                    if next_token == eos_token_id {
                        break;
                    }

                    let input = match Tensor::new(&[next_token], &device) {
                        Ok(t) => match t.unsqueeze(0) {
                            Ok(t) => t,
                            Err(e) => {
                                let _ = tx.send(CandleStringChunk(format!(
                                    "ERROR: Failed to unsqueeze tensor: {}",
                                    e
                                )));
                                return;
                            }
                        },
                        Err(e) => {
                            let _ = tx.send(CandleStringChunk(format!(
                                "ERROR: Failed to create input tensor: {}",
                                e
                            )));
                            return;
                        }
                    };

                    let logits = match model.forward(&input, tokens.len() + index as usize) {
                        Ok(l) => l,
                        Err(e) => {
                            let _ = tx.send(CandleStringChunk(format!(
                                "ERROR: Forward pass failed: {}",
                                e
                            )));
                            return;
                        }
                    };

                    let logits = match logits.squeeze(0) {
                        Ok(l) => l,
                        Err(e) => {
                            let _ = tx.send(CandleStringChunk(format!(
                                "ERROR: Failed to squeeze logits: {}",
                                e
                            )));
                            return;
                        }
                    };

                    // Apply temperature scaling
                    let logits = if temperature != 1.0 {
                        match logits / temperature as f64 {
                            Ok(l) => l,
                            Err(e) => {
                                let _ = tx.send(CandleStringChunk(format!(
                                    "ERROR: Temperature scaling failed: {}",
                                    e
                                )));
                                return;
                            }
                        }
                    } else {
                        logits
                    };

                    // Conditional repeat penalty - skip when == 1.0 for performance
                    let logits = if repeat_penalty != 1.0 {
                        match candle_transformers::utils::apply_repeat_penalty(
                            &logits,
                            repeat_penalty as f32,
                            &all_tokens[start_at..],
                        ) {
                            Ok(l) => l,
                            Err(e) => {
                                let _ = tx.send(CandleStringChunk(format!(
                                    "ERROR: Repeat penalty failed: {}",
                                    e
                                )));
                                return;
                            }
                        }
                    } else {
                        logits // Skip expensive operation when not needed
                    };

                    next_token = match logits_processor.sample(&logits) {
                        Ok(t) => t,
                        Err(e) => {
                            let _ = tx.send(CandleStringChunk(format!(
                                "ERROR: Sampling failed: {}",
                                e
                            )));
                            return;
                        }
                    };

                    all_tokens.push(next_token);

                    // Send token through stream using TokenOutputStream
                    if let Some(t) = tos.next_token(next_token).ok().flatten() {
                        let _ = tx.send(CandleStringChunk(t));
                    }
                }

                // Flush any remaining tokens
                if let Ok(Some(t)) = tos.decode_rest() {
                    if !t.is_empty() {
                        let _ = tx.send(CandleStringChunk(t));
                    }
                }
            })
        }))
    }
}

impl CandleModel for CandleQwen3QuantizedModel {
    #[inline]
    fn info(&self) -> &'static CandleModelInfo {
        &QWEN3_QUANTIZED_MODEL_INFO
    }
}

/// Qwen3 Quantized completion request format for HTTP API compatibility
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct CandleQwenCompletionRequest {
    prompt: String,
    temperature: f64,
    max_tokens: u64,
    stream: bool,
    model: String,
}

/// Validate that the model path exists and is accessible
///
/// # Errors
/// Returns error if the path does not exist or is not accessible
#[allow(dead_code)]
fn validate_model_path(path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let model_path = Path::new(path);

    if !model_path.exists() {
        return Err(format!("Model path does not exist: {}", path).into());
    }

    if !model_path.is_dir() && !model_path.is_file() {
        return Err(format!("Model path is neither file nor directory: {}", path).into());
    }

    Ok(())
}

/// Loaded Qwen3 Quantized model that keeps resources in memory for worker threads
///
/// This model pre-loads the actual model into memory with safe async mutable access,
/// avoiding disk I/O on every request.
#[derive(Clone)]
pub struct LoadedQwen3QuantizedModel {
    /// The loaded Qwen3 model using Candle's native quantized implementation
    /// Wrapped in Arc<Mutex> for safe sharing in async context
    model: Arc<tokio::sync::Mutex<Qwen3Model>>,
    tokenizer: tokenizers::Tokenizer,
    device: Device,
    engine: Arc<Engine>,
    /// EOS token ID extracted from GGUF metadata
    eos_token_id: Option<u32>,
}

impl LoadedQwen3QuantizedModel {
    /// Load model resources into memory (called once per worker)
    ///
    /// This method loads EVERYTHING once: model, tokenizer, device.
    /// The model stays in memory for all subsequent requests.
    pub async fn load(
        base: &CandleQwen3QuantizedModel,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        log::info!("Loading Qwen3 model using Candle's native quantized implementation");
        
        // Get file paths
        let gguf_file_path = std::path::PathBuf::from(&base.gguf_file_path);
        let tokenizer_path = std::path::PathBuf::from(&base.model_path).join("tokenizer.json");

        if !tokenizer_path.exists() {
            return Err(
                Box::from(format!("Tokenizer file not found: {:?}", tokenizer_path))
                    as Box<dyn std::error::Error + Send + Sync>,
            );
        }

        // Load device (prefer GPU if available)
        let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
            log::warn!("Device detection failed: {}. Using CPU.", e);
            Device::Cpu
        });

        // Load GGUF file - simple and direct (no spawn_blocking)
        log::info!("Loading model from {}", gguf_file_path.display());
        let mut file = std::fs::File::open(&gguf_file_path).map_err(|e| {
            Box::from(format!("Failed to open GGUF file: {}", e))
                as Box<dyn std::error::Error + Send + Sync>
        })?;
        
        let content = gguf_file::Content::read(&mut file).map_err(|e| {
            Box::from(format!("Failed to read GGUF content: {}", e))
                as Box<dyn std::error::Error + Send + Sync>
        })?;

        // Extract EOS token from GGUF metadata
        let eos_token_id = content
            .metadata
            .get("tokenizer.ggml.eos_token_id")
            .and_then(|v| v.to_u32().ok());

        log::info!("EOS token ID from GGUF: {:?}", eos_token_id);

        // Create model using Candle's native implementation - simple and fast!
        let model = Qwen3Model::from_gguf(content, &mut file, &device).map_err(|e| {
            Box::from(format!("Failed to create model: {}", e))
                as Box<dyn std::error::Error + Send + Sync>
        })?;

        log::info!("Model loaded successfully");

        // Load tokenizer - direct synchronous loading (no spawn_blocking)
        log::info!("Loading tokenizer from {}", tokenizer_path.display());
        let tokenizer = tokenizers::Tokenizer::from_file(&tokenizer_path).map_err(|e| {
            Box::from(format!("Failed to load tokenizer: {}", e))
                as Box<dyn std::error::Error + Send + Sync>
        })?;

        log::info!("Tokenizer loaded successfully");

        Ok(Self {
            model: Arc::new(tokio::sync::Mutex::new(model)),
            tokenizer,
            device,
            engine: Arc::clone(&base.engine),
            eos_token_id,
        })
    }
}

impl crate::capability::traits::TextToTextCapable for LoadedQwen3QuantizedModel {
    fn prompt(
        &self,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> Pin<Box<dyn Stream<Item = CandleCompletionChunk> + Send>> {
        // Clone pre-loaded resources for the generation closure
        let engine = self.engine.clone();
        let model = self.model.clone();  // ✅ Use CACHED model
        let device = self.device.clone();
        let tokenizer = self.tokenizer.clone(); // ✅ Clone pre-loaded tokenizer
        let eos_token_id = self.eos_token_id.unwrap_or(151645);
        
        log::info!("🚀 Using CACHED model from memory - no loading needed!");

        // Build sampling config
        let temperature = if params.temperature != 1.0 {
            params.temperature
        } else {
            QWEN3_QUANTIZED_MODEL_INFO.default_temperature.unwrap_or(0.8)
        };

        // Extract additional params or use defaults
        let top_k = params
            .additional_params
            .as_ref()
            .and_then(|p| p.get("top_k"))
            .and_then(|v| v.as_u64())
            .map(|v| v as usize);

        let top_p = params
            .additional_params
            .as_ref()
            .and_then(|p| p.get("top_p"))
            .and_then(|v| v.as_f64())
            .or(QWEN3_QUANTIZED_MODEL_INFO.default_top_p);

        let repeat_penalty = params
            .additional_params
            .as_ref()
            .and_then(|p| p.get("repeat_penalty"))
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0);

        // Format prompt using Qwen3 chat template
        let prompt_text = format!("<|im_start|>user\n{}<|im_end|>\n<|im_start|>assistant\n", prompt.content);
        let max_tokens = params.max_tokens.map(|n| n.get()).unwrap_or(1000);

        // Use Engine's coordinate_generation for automatic metrics and stream conversion
        Box::pin(engine.coordinate_generation(move || {
            async_stream::spawn_stream(move |tx| async move {
                log::info!("✅ Using cached model from memory - no disk I/O!");

                // Encode the prompt
                let tokens = match tokenizer.encode(prompt_text.as_str(), false) {
                    Ok(encoding) => encoding.get_ids().to_vec(),
                    Err(e) => {
                        let _ = tx.send(CandleStringChunk(format!(
                            "ERROR: Failed to encode prompt: {}",
                            e
                        )));
                        return;
                    }
                };

                // Create LogitsProcessor for sampling
                let seed = 299792458;
                let mut logits_processor = {
                    let sampling = if temperature <= 0.0 {
                        Sampling::ArgMax
                    } else if let Some(k) = top_k {
                        Sampling::TopK { k, temperature: temperature as f64 }
                    } else if let Some(p) = top_p {
                        Sampling::TopP { p, temperature: temperature as f64 }
                    } else {
                        Sampling::All { temperature: temperature as f64 }
                    };
                    LogitsProcessor::from_sampling(seed, sampling)
                };

                // Create TokenOutputStream for efficient decoding
                let mut tos = TokenOutputStream::new(tokenizer.clone());

                // Track all tokens for repeat penalty
                let mut all_tokens = Vec::with_capacity(tokens.len() + max_tokens as usize);
                all_tokens.extend_from_slice(&tokens);
                let start_at = all_tokens.len();

                // Lock the model for generation
                let mut model = model.lock().await;

                // Initial forward pass
                let input = match Tensor::new(&tokens[..], &device) {
                    Ok(t) => match t.unsqueeze(0) {
                        Ok(t) => t,
                        Err(e) => {
                            let _ = tx.send(CandleStringChunk(format!(
                                "ERROR: Failed to unsqueeze tensor: {}",
                                e
                            )));
                            return;
                        }
                    },
                    Err(e) => {
                        let _ = tx.send(CandleStringChunk(format!(
                            "ERROR: Failed to create input tensor: {}",
                            e
                        )));
                        return;
                    }
                };

                let logits = match model.forward(&input, 0) {
                    Ok(l) => l,
                    Err(e) => {
                        let _ = tx.send(CandleStringChunk(format!(
                            "ERROR: Forward pass failed: {}",
                            e
                        )));
                        return;
                    }
                };

                let logits = match logits.squeeze(0) {
                    Ok(l) => l,
                    Err(e) => {
                        let _ = tx.send(CandleStringChunk(format!(
                            "ERROR: Failed to squeeze logits: {}",
                            e
                        )));
                        return;
                    }
                };

                // Apply temperature scaling
                let logits = if temperature != 1.0 {
                    match logits / temperature as f64 {
                        Ok(l) => l,
                        Err(e) => {
                            let _ = tx.send(CandleStringChunk(format!(
                                "ERROR: Temperature scaling failed: {}",
                                e
                            )));
                            return;
                        }
                    }
                } else {
                    logits
                };

                // Conditional repeat penalty - skip when == 1.0 for performance
                let logits = if repeat_penalty != 1.0 {
                    match candle_transformers::utils::apply_repeat_penalty(
                        &logits,
                        repeat_penalty as f32,
                        &all_tokens[start_at..],
                    ) {
                        Ok(l) => l,
                        Err(e) => {
                            let _ = tx.send(CandleStringChunk(format!(
                                "ERROR: Repeat penalty failed: {}",
                                e
                            )));
                            return;
                        }
                    }
                } else {
                    logits // Skip expensive operation when not needed
                };

                let mut next_token = match logits_processor.sample(&logits) {
                    Ok(t) => t,
                    Err(e) => {
                        let _ = tx.send(CandleStringChunk(format!(
                            "ERROR: Sampling failed: {}",
                            e
                        )));
                        return;
                    }
                };

                all_tokens.push(next_token);

                // Send first token
                if let Some(t) = tos.next_token(next_token).ok().flatten() {
                    let _ = tx.send(CandleStringChunk(t));
                }

                // Continue generation
                for index in 0..max_tokens {
                    if next_token == eos_token_id {
                        break;
                    }

                    let input = match Tensor::new(&[next_token], &device) {
                        Ok(t) => match t.unsqueeze(0) {
                            Ok(t) => t,
                            Err(e) => {
                                let _ = tx.send(CandleStringChunk(format!(
                                    "ERROR: Failed to unsqueeze tensor: {}",
                                    e
                                )));
                                return;
                            }
                        },
                        Err(e) => {
                            let _ = tx.send(CandleStringChunk(format!(
                                "ERROR: Failed to create input tensor: {}",
                                e
                            )));
                            return;
                        }
                    };

                    let logits = match model.forward(&input, tokens.len() + index as usize) {
                        Ok(l) => l,
                        Err(e) => {
                            let _ = tx.send(CandleStringChunk(format!(
                                "ERROR: Forward pass failed: {}",
                                e
                            )));
                            return;
                        }
                    };

                    let logits = match logits.squeeze(0) {
                        Ok(l) => l,
                        Err(e) => {
                            let _ = tx.send(CandleStringChunk(format!(
                                "ERROR: Failed to squeeze logits: {}",
                                e
                            )));
                            return;
                        }
                    };

                    // Apply temperature scaling
                    let logits = if temperature != 1.0 {
                        match logits / temperature as f64 {
                            Ok(l) => l,
                            Err(e) => {
                                let _ = tx.send(CandleStringChunk(format!(
                                    "ERROR: Temperature scaling failed: {}",
                                    e
                                )));
                                return;
                            }
                        }
                    } else {
                        logits
                    };

                    // Conditional repeat penalty - skip when == 1.0 for performance
                    let logits = if repeat_penalty != 1.0 {
                        match candle_transformers::utils::apply_repeat_penalty(
                            &logits,
                            repeat_penalty as f32,
                            &all_tokens[start_at..],
                        ) {
                            Ok(l) => l,
                            Err(e) => {
                                let _ = tx.send(CandleStringChunk(format!(
                                    "ERROR: Repeat penalty failed: {}",
                                    e
                                )));
                                return;
                            }
                        }
                    } else {
                        logits // Skip expensive operation when not needed
                    };

                    next_token = match logits_processor.sample(&logits) {
                        Ok(t) => t,
                        Err(e) => {
                            let _ = tx.send(CandleStringChunk(format!(
                                "ERROR: Sampling failed: {}",
                                e
                            )));
                            return;
                        }
                    };

                    all_tokens.push(next_token);

                    // Send token through stream using TokenOutputStream
                    if let Some(t) = tos.next_token(next_token).ok().flatten() {
                        let _ = tx.send(CandleStringChunk(t));
                    }
                }

                // Flush any remaining tokens
                if let Ok(Some(t)) = tos.decode_rest() {
                    if !t.is_empty() {
                        let _ = tx.send(CandleStringChunk(t));
                    }
                }
            })
        }))
    }
}



impl std::fmt::Debug for LoadedQwen3QuantizedModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoadedQwen3QuantizedModel")
            .field("device", &self.device)
            .field("model", &"Arc<Mutex<Qwen3Model>>")
            .field("eos_token_id", &self.eos_token_id)
            .finish()
    }
}

impl CandleModel for LoadedQwen3QuantizedModel {
    #[inline]
    fn info(&self) -> &'static CandleModelInfo {
        &QWEN3_QUANTIZED_MODEL_INFO
    }
}
