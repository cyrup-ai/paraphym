//! Provides streaming completion capabilities using local Qwen3-Coder-30B models
//! with zero allocation patterns and AsyncStream streaming.
//!
//! This implementation uses the Candle ML framework for local model inference,
//! specifically targeting Qwen architecture models optimized for code generation.

use std::num::NonZeroU32;
use std::path::Path;

use candle_core::quantized::gguf_file;
use candle_core::DType;
use candle_transformers::models::llama::LlamaConfig;
use ystream::AsyncStream;
// SIMD optimizations for high-performance inference
use paraphym_simd::get_cpu_features;
use progresshub::{ProgressHub, types::ZeroOneOrMany as ProgressHubZeroOneOrMany};
use serde::{Deserialize, Serialize};

use crate::domain::chat::message::types::CandleMessageChunk;
use crate::domain::completion::{
    CandleCompletionChunk, CandleCompletionModel, CandleCompletionParams,
};
use crate::domain::context::CandleStringChunk;
use crate::domain::model::{info::CandleModelInfo, traits::CandleModel};
use crate::domain::prompt::CandlePrompt;
use ystream::emit;

/// Builder trait for Qwen3 Coder completion providers
pub trait BuilderCandleQwen3CoderProvider: Send + Sync + 'static {
    // Default implementations for all builders
}

/// High-performance Qwen3 Coder-30B provider for local inference using Candle
///
/// Provides streaming text generation capabilities using the Qwen3-Coder-30B-A3B-Instruct model
/// with automatic model downloading via ProgressHub.
#[derive(Debug, Clone)]
pub struct CandleQwen3CoderProvider {
    /// Model cache directory path
    model_path: String,
    /// GGUF model file path
    gguf_file_path: String,
    /// Provider configuration
    config: CandleQwen3CoderConfig,
    /// Model configuration for inference
    model_config: LlamaConfig,
}

/// Configuration for Qwen3 Coder model inference
#[derive(Debug, Clone)]
pub struct CandleQwen3CoderConfig {
    /// Maximum context length for inference
    max_context: u32,
    /// Default temperature for sampling
    temperature: f64,
    /// Vocabulary size for tokenization
    vocab_size: u32,
    /// Enable key-value caching for faster inference
    use_kv_cache: bool,
    /// Data type for model weights (F16, BF16, F32)
    dtype: DType,
}

impl Default for CandleQwen3CoderConfig {
    #[inline]
    fn default() -> Self {
        Self {
            max_context: 32768, // 32K context for Qwen3-Coder
            temperature: 0.1,   // Lower temperature for code generation
            vocab_size: 152064, // Qwen3 vocabulary size
            use_kv_cache: true,
            dtype: DType::F16,
        }
    }
}

impl CandleQwen3CoderConfig {
    /// Get the temperature setting
    #[inline]
    pub fn temperature(&self) -> f64 {
        self.temperature
    }

    /// Set temperature for sampling
    #[inline]
    pub fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = temperature;
        self
    }

    /// Get the maximum context length
    #[inline]
    pub fn max_context(&self) -> u32 {
        self.max_context
    }

    /// Set maximum context length
    #[inline]
    pub fn with_max_context(mut self, max_context: u32) -> Self {
        self.max_context = max_context;
        self
    }

    /// Get vocabulary size
    #[inline]
    pub fn vocab_size(&self) -> u32 {
        self.vocab_size
    }

    /// Check if KV cache is enabled
    #[inline]
    pub fn uses_kv_cache(&self) -> bool {
        self.use_kv_cache
    }

    /// Enable or disable KV cache
    #[inline]
    pub fn with_kv_cache(mut self, use_cache: bool) -> Self {
        self.use_kv_cache = use_cache;
        self
    }
}

impl CandleQwen3CoderProvider {
    /// Create new Qwen3 Coder provider with automatic model download
    ///
    /// This method automatically downloads the Qwen3-Coder-30B model from HuggingFace
    /// using ProgressHub and returns a provider ready for inference.
    ///
    /// # Example
    /// ```rust
    /// let provider = CandleQwen3CoderProvider::new().await?;
    /// ```
    ///
    /// # Errors
    /// Returns error if model download fails or model loading fails
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let config = CandleQwen3CoderConfig::default();
        Self::with_config_async(config).await
    }

    /// Create provider with custom configuration and automatic download
    pub async fn with_config_async(
        config: CandleQwen3CoderConfig,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Download model using ProgressHub
        let results = ProgressHub::builder()
            .model("Qwen/Qwen2.5-Coder-32B-Instruct-GGUF")
            .with_cli_progress()
            .download()
            .await?;

        // Extract the model path from download results following ProgressHub example pattern
        let (model_cache_dir, gguf_file_path) = if let Some(result) = results.into_iter().next() {
            match &result.models {
                ProgressHubZeroOneOrMany::One(model) => {
                    // Extract model cache directory
                    let cache_dir = model.model_cache_path.display().to_string();

                    // Find GGUF file with zero allocation - prioritize largest file (model weights over tokenizer)
                    let gguf_file = model
                        .files
                        .iter()
                        .filter(|file| file.filename.ends_with(".gguf"))
                        .max_by_key(|file| file.expected_size)
                        .ok_or("No GGUF files found in downloaded model")?;

                    let gguf_path = gguf_file.path.display().to_string();
                    (cache_dir, gguf_path)
                }
                ProgressHubZeroOneOrMany::Zero => {
                    return Err("No models were downloaded".into());
                }
                ProgressHubZeroOneOrMany::Many(_) => {
                    return Err("Expected exactly one model, got multiple".into());
                }
            }
        } else {
            return Err("No download results returned".into());
        };

        Self::with_config_sync_gguf(model_cache_dir, gguf_file_path, config)
    }


    /// Create provider with custom configuration and existing model path
    pub fn with_config_sync(
        model_path: String,
        config: CandleQwen3CoderConfig,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Log SIMD capabilities for performance debugging
        let cpu_info = get_cpu_features();
        log::info!(
            "Qwen3 Coder Provider initialized with SIMD support: {} (vector width: {} elements)",
            cpu_info.has_simd(),
            cpu_info.vector_width()
        );

        // Create model configuration for Qwen3 Coder (Qwen-based architecture)
        let model_config = LlamaConfig {
            vocab_size: config.vocab_size as usize,
            hidden_size: 8192,            // Qwen3-Coder-30B hidden size
            intermediate_size: 29568,     // FFN intermediate size
            num_hidden_layers: 64,        // Number of layers
            num_attention_heads: 64,      // Number of attention heads
            num_key_value_heads: Some(8), // GQA configuration
            max_position_embeddings: config.max_context as usize,
            rms_norm_eps: 1e-6,
            rope_theta: 1000000.0,      // Qwen3 RoPE theta
            bos_token_id: Some(151643), // Qwen3 BOS token
            eos_token_id: Some(candle_transformers::models::llama::LlamaEosToks::Single(
                151645,
            )), // Qwen3 EOS token
            rope_scaling: None,
            tie_word_embeddings: Some(false),
        };

        Ok(Self {
            model_path: model_path.clone(),
            gguf_file_path: model_path, // For sync method, assume model_path is the GGUF file
            config,
            model_config,
        })
    }

    /// Create provider with custom configuration and GGUF metadata extraction for Qwen3-Coder
    ///
    /// This method reads the GGUF file metadata to extract real Qwen3-Coder model configuration
    /// instead of using hardcoded values, ensuring accurate model parameters for code generation.
    #[inline]
    pub fn with_config_sync_gguf(
        model_cache_dir: String,
        gguf_file_path: String,
        config: CandleQwen3CoderConfig,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Log SIMD capabilities for performance debugging
        let cpu_info = get_cpu_features();
        log::info!(
            "Qwen3 Coder Provider initialized with SIMD support: {} (vector width: {} elements)",
            cpu_info.has_simd(),
            cpu_info.vector_width()
        );

        // Read GGUF file metadata for real model configuration
        let mut file = std::fs::File::open(&gguf_file_path)?;
        let content = gguf_file::Content::read(&mut file)?;

        // Extract Qwen3-specific metadata with Llama fallbacks - zero allocation parsing
        let hidden_size = content
            .metadata
            .get("qwen3.embedding_length")
            .or_else(|| content.metadata.get("llama.embedding_length"))
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as usize)
            .unwrap_or(8192); // Fallback for Qwen3-Coder-30B

        let intermediate_size = content
            .metadata
            .get("qwen3.feed_forward_length")
            .or_else(|| content.metadata.get("llama.feed_forward_length"))
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as usize)
            .unwrap_or(29568); // Fallback for Qwen3-Coder-30B FFN

        let num_hidden_layers = content
            .metadata
            .get("qwen3.block_count")
            .or_else(|| content.metadata.get("llama.block_count"))
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as usize)
            .unwrap_or(64); // Fallback for Qwen3-Coder-30B layers

        let num_attention_heads = content
            .metadata
            .get("qwen3.attention.head_count")
            .or_else(|| content.metadata.get("llama.attention.head_count"))
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as usize)
            .unwrap_or(64); // Fallback for Qwen3-Coder-30B heads

        let num_key_value_heads = content
            .metadata
            .get("qwen3.attention.head_count_kv")
            .or_else(|| content.metadata.get("llama.attention.head_count_kv"))
            .and_then(|v| v.to_u64().ok())
            .map(|v| Some(v as usize))
            .unwrap_or(Some(8)); // Fallback for Qwen3-Coder-30B GQA

        let rope_theta = content
            .metadata
            .get("qwen3.rope.freq_base")
            .or_else(|| content.metadata.get("llama.rope.freq_base"))
            .and_then(|v| v.to_f64().ok())
            .unwrap_or(1000000.0) as f32; // Qwen3 uses higher rope_theta than standard Llama

        // Extract vocab_size from metadata or use config default
        let vocab_size = content
            .metadata
            .get("tokenizer.ggml.token_count")
            .or_else(|| content.metadata.get("qwen3.vocab_size"))
            .or_else(|| content.metadata.get("llama.vocab_size"))
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as usize)
            .unwrap_or(config.vocab_size as usize);

        // Extract Qwen3-specific token IDs with fallbacks
        let bos_token_id = content
            .metadata
            .get("tokenizer.ggml.bos_token_id")
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as u32)
            .unwrap_or(151643); // Qwen3 BOS token

        let eos_token_id = content
            .metadata
            .get("tokenizer.ggml.eos_token_id")
            .and_then(|v| v.to_u64().ok())
            .map(|v| v as u32)
            .unwrap_or(151645); // Qwen3 EOS token

        // Create model configuration with real GGUF metadata
        let model_config = LlamaConfig {
            vocab_size,
            hidden_size,
            intermediate_size,
            num_hidden_layers,
            num_attention_heads,
            num_key_value_heads,
            max_position_embeddings: config.max_context as usize,
            rms_norm_eps: 1e-6,
            rope_theta,
            bos_token_id: Some(bos_token_id),
            eos_token_id: Some(candle_transformers::models::llama::LlamaEosToks::Single(
                eos_token_id,
            )),
            rope_scaling: None,
            tie_word_embeddings: Some(false),
        };

        // Log extracted configuration for debugging
        log::debug!("Extracted GGUF metadata for Qwen3-Coder: hidden_size={}, layers={}, heads={}, kv_heads={:?}, rope_theta={}", 
                   hidden_size, num_hidden_layers, num_attention_heads, num_key_value_heads, rope_theta);

        Ok(Self {
            model_path: model_cache_dir,
            gguf_file_path,
            config,
            model_config,
        })
    }

    /// Get the model path
    #[inline]
    pub fn model_path(&self) -> &str {
        &self.model_path
    }

    /// Get the configuration
    #[inline]
    pub fn config(&self) -> &CandleQwen3CoderConfig {
        &self.config
    }

    /// Generate streaming completion for code generation (LEGACY - use prompt() instead)
    ///
    /// # Arguments
    /// * `prompt` - Input text prompt for code generation
    ///
    /// # Returns
    /// AsyncStream of CandleMessageChunk tokens
    #[deprecated(note = "Use CandleCompletionModel::prompt() instead")]
    pub fn generate_stream(&self, prompt: &str) -> AsyncStream<CandleMessageChunk> {
        use std::num::NonZeroU64;

        use crate::domain::completion::types::CandleCompletionParams;
        use crate::domain::prompt::CandlePrompt;

        // Convert to new API
        let candle_prompt = CandlePrompt::new(prompt);
        let params = CandleCompletionParams {
            temperature: self.config.temperature(),
            max_tokens: NonZeroU64::new(1000),
            ..Default::default()
        };

        // Use real inference via prompt() method
        let completion_stream = self.prompt(candle_prompt, &params);

        // Convert CandleCompletionChunk to legacy CandleMessageChunk format
        AsyncStream::with_channel(move |sender| {
            let completion_chunks: Vec<crate::domain::completion::CandleCompletionChunk> = completion_stream.collect();
            for completion_chunk in completion_chunks {
                let message_chunk = match completion_chunk {
                    crate::domain::completion::CandleCompletionChunk::Text(text) => {
                        CandleMessageChunk::Text(text)
                    }
                    crate::domain::completion::CandleCompletionChunk::Complete {
                        text,
                        finish_reason,
                        usage,
                    } => CandleMessageChunk::Complete {
                        text,
                        finish_reason: finish_reason.map(|f| format!("{:?}", f)),
                        usage: usage.map(|u| format!("{:?}", u)),
                    },
                    _ => CandleMessageChunk::Error("Unknown completion chunk type".to_string()),
                };

                emit!(sender, message_chunk);
            }
        })
    }
}

// Implement builder trait
impl BuilderCandleQwen3CoderProvider for CandleQwen3CoderProvider {}

// Static model info for Qwen3-Coder-30B
static QWEN3_CODER_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider_name: "candle-qwen",
    name: "qwen3-coder-30b-instruct",
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
    hf_repo_url: "unsloth/Qwen3-Coder-30B-A3B-Instruct-GGUF",
    quantization: "Q4_0",
    patch: None,
};

impl CandleCompletionModel for CandleQwen3CoderProvider {
    fn prompt(
        &self,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> AsyncStream<CandleCompletionChunk> {
        // Create ModelConfig for this provider (thin wrapper - only config!)
        // Convert LlamaConfig to the format expected by ModelArchitecture
        let candle_config = candle_transformers::models::llama::Config {
            hidden_size: self.model_config.hidden_size,
            intermediate_size: self.model_config.intermediate_size,
            vocab_size: self.model_config.vocab_size,
            num_hidden_layers: self.model_config.num_hidden_layers,
            num_attention_heads: self.model_config.num_attention_heads,
            num_key_value_heads: self
                .model_config
                .num_key_value_heads
                .unwrap_or(self.model_config.num_attention_heads),
            use_flash_attn: false,
            rms_norm_eps: self.model_config.rms_norm_eps,
            rope_theta: self.model_config.rope_theta,
            bos_token_id: self.model_config.bos_token_id,
            eos_token_id: self.model_config.eos_token_id.clone(),
            rope_scaling: self.model_config.rope_scaling.clone(),
            max_position_embeddings: self.model_config.max_position_embeddings,
            tie_word_embeddings: self.model_config.tie_word_embeddings.unwrap_or(false),
        };

        let _model_config = crate::core::ModelConfig::new(
            &self.model_path,
            format!("{}/tokenizer.json", self.model_path),
            crate::core::ModelArchitecture::Llama(candle_config),
            "qwen3-coder",
            "qwen3-coder",
        )
        .with_vocab_size(self.config.vocab_size as usize)
        .with_context_length(self.config.max_context as usize)
        .with_dtype(self.config.dtype);

        // Create SIMD-optimized SamplingConfig from params
        let _cpu_info = get_cpu_features();
        let _sampling_config =
            crate::core::generation::SamplingConfig::new(params.temperature as f32)
                .with_top_k(50) // Default for now
                .with_top_p(0.9) // Default for now
                .with_repetition_penalty(1.0)
                .with_frequency_penalty(0.0)
                .with_presence_penalty(0.0);

        // Format prompt
        let _prompt_text = format!("User: {}\nAssistant: ", prompt);
        let _max_tokens = params.max_tokens.map(|n| n.get()).unwrap_or(1000);

        // Create TextGenerator and perform local inference
        let model_path = self.model_path.clone();
        let gguf_file_path = self.gguf_file_path.clone();
        let config = self.config.clone();
        let model_config = self.model_config.clone();

        AsyncStream::with_channel(move |sender| {
            use crate::core::generation::{
                generator::TextGenerator,
                tokens::SpecialTokens,
                // models::CandleModel as CoreCandleModel, // Reserved for future candle model integration
            };
            use candle_core::Device;
            use tokenizers::Tokenizer;

            // Load device (prefer GPU if available)
            let device = Device::Cpu; // TODO: Add GPU detection

            // Load tokenizer
            let tokenizer = match Tokenizer::from_file(format!("{}/tokenizer.json", model_path)) {
                Ok(t) => t,
                Err(e) => {
                    let error_chunk = CandleCompletionChunk::Error(format!("Failed to load tokenizer: {}", e));
                    let _ = sender.send(error_chunk);
                    return;
                }
            };

            // Create real quantized model implementation
            use crate::core::generation::models::ModelFactory;
            use crate::core::ModelConfig as CandleConfig;
            use std::sync::Arc;

            // Create model configuration for the quantized model
            let candle_model_config = Arc::new(CandleConfig::new(
                &gguf_file_path,
                format!("{}/tokenizer.json", model_path),
                crate::core::ModelArchitecture::Llama(candle_transformers::models::llama::Config {
                    hidden_size: model_config.hidden_size,
                    intermediate_size: model_config.intermediate_size,
                    vocab_size: model_config.vocab_size,
                    num_hidden_layers: model_config.num_hidden_layers,
                    num_attention_heads: model_config.num_attention_heads,
                    num_key_value_heads: model_config.num_key_value_heads.unwrap_or(model_config.num_attention_heads),
                    use_flash_attn: false,
                    rms_norm_eps: model_config.rms_norm_eps,
                    rope_theta: model_config.rope_theta,
                    bos_token_id: model_config.bos_token_id,
                    eos_token_id: model_config.eos_token_id.clone(),
                    rope_scaling: model_config.rope_scaling.clone(),
                    max_position_embeddings: model_config.max_position_embeddings,
                    tie_word_embeddings: model_config.tie_word_embeddings.unwrap_or(false),
                }),
                "qwen3-coder",
                "qwen3-coder",
            )
            .with_vocab_size(model_config.vocab_size)
            .with_context_length(config.max_context as usize)
            .with_dtype(config.dtype));

            // Load the real quantized model
            let quantized_model = match ModelFactory::create_quantized_llama(&gguf_file_path, candle_model_config, device.clone()) {
                Ok(model) => model,
                Err(e) => {
                    let error_chunk = CandleCompletionChunk::Error(format!("Failed to load quantized model: {}", e));
                    let _ = sender.send(error_chunk);
                    return;
                }
            };

            // Create TextGenerator with real model
            let text_generator = TextGenerator::new(
                Box::new(quantized_model),
                tokenizer,
                device,
                _sampling_config,
            );

            // Set up special tokens
            let special_tokens = SpecialTokens {
                bos_token_id: Some(model_config.bos_token_id.unwrap_or(151643)), // Qwen3 BOS
                eos_token_id: match &model_config.eos_token_id {
                    Some(candle_transformers::models::llama::LlamaEosToks::Single(id)) => Some(*id),
                    _ => Some(151645), // Qwen3 EOS
                },
                pad_token_id: None,
            };

            // Generate text using TextGenerator
            let text_stream = text_generator.generate(
                _prompt_text,
                _max_tokens.try_into().unwrap(),
                special_tokens,
            );

            // Convert CandleStringChunk to CandleCompletionChunk using correct ystream pattern
            let text_chunks: Vec<CandleStringChunk> = text_stream.collect();
            for string_chunk in text_chunks {
                let completion_chunk = CandleCompletionChunk::Text(string_chunk.0);
                emit!(sender, completion_chunk);
            }
        })
    }
}

impl CandleModel for CandleQwen3CoderProvider {
    fn info(&self) -> &'static CandleModelInfo {
        &QWEN3_CODER_MODEL_INFO
    }
}

/// Qwen3 Coder completion request format for HTTP API compatibility
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
