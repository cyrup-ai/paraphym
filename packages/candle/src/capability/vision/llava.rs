//! LLaVA (Large Language and Vision Assistant) provider
//!
//! This module wraps Candle's unified LLaVA model for vision-language understanding.
//! Supports visual question answering, image description, and multi-turn conversations.

use std::num::NonZeroU32;
use std::sync::mpsc;
use std::thread;

use candle_core::{Device, IndexOp, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::generation::{LogitsProcessor, Sampling};
use candle_transformers::models::llama::Cache;
use candle_transformers::models::llava::{LLaVA, config::LLaVAConfig as CandleLLaVAConfig};
use tokenizers::Tokenizer;
use ystream::AsyncStream;

use crate::builders::image::{ImageBuilder, ResizeFilter};
use crate::core::Engine;
use crate::domain::context::CandleStringChunk;
use crate::domain::image::Image;
use crate::domain::model::{CandleModelInfo, CandleProvider, traits::CandleModel};
use crate::domain::model::download::DownloadProviderFactory;

/// Request types for LLaVA model thread communication
enum LLaVARequest {
    Ask {
        image_path: String,
        question: String,
        response_tx: mpsc::Sender<Result<String, String>>,
    },
    AskUrl {
        image_url: String,
        question: String,
        response_tx: mpsc::Sender<Result<String, String>>,
    },
    Shutdown,
}

/// LLaVA model configuration
///
/// Default settings match LLaVA 1.5 with 336×336 images
/// and ImageNet normalization parameters.
#[derive(Clone, Debug)]
pub struct LLaVAConfig {
    /// Image input size (336 for LLaVA 1.5)
    pub image_size: usize,
    /// ImageNet mean for normalization
    pub image_mean: [f32; 3],
    /// ImageNet std for normalization
    pub image_std: [f32; 3],
    /// Temperature for sampling
    pub temperature: f64,
    /// Maximum new tokens to generate
    pub max_new_tokens: usize,
    /// Enable KV cache for faster inference
    pub use_kv_cache: bool,
}

impl Default for LLaVAConfig {
    fn default() -> Self {
        Self {
            image_size: 336,
            image_mean: [0.48145466, 0.4578275, 0.40821073],
            image_std: [0.26862954, 0.2613026, 0.2757771],
            temperature: 0.2,
            max_new_tokens: 512,
            use_kv_cache: true,
        }
    }
}

/// LLaVA vision-language provider
///
/// Wraps Candle's unified LLaVA model (CLIP vision + LLaMA language)
/// for image understanding and visual question answering.
///
/// The actual LLaVA model runs on a dedicated thread to avoid Send/Sync issues
/// with Candle's Module trait. Communication happens via channels.
#[derive(Debug)]
pub struct LLaVAModel {
    request_tx: mpsc::Sender<LLaVARequest>,
    config: LLaVAConfig,
    llava_config: CandleLLaVAConfig,
    model_path: String,
    _engine: Engine,
}

// Static model info for LLaVA 1.5
pub static LLAVA_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: CandleProvider::LLaVAHF,
    name: "llava-1.5-7b-hf",
    registry_key: "llava-hf/llava-1.5-7b-hf",
    max_input_tokens: NonZeroU32::new(4096),
    max_output_tokens: NonZeroU32::new(512),
    input_price: None,  // Local model - no pricing
    output_price: None,
    supports_vision: true,
    supports_function_calling: false,
    supports_streaming: true,
    supports_embeddings: false,
    requires_max_tokens: false,
    supports_thinking: false,
    optimal_thinking_budget: None,
    system_prompt_prefix: None,
    real_name: None,
    model_type: None,
    model_id: "llava",
    quantization: "F16",
    patch: None,
};

impl LLaVAModel {
    /// Load LLaVA model from local path
    ///
    /// Expects safetensors files and tokenizer.json in model_path.
    /// Spawns a dedicated thread for the model to avoid Send/Sync issues.
    ///
    /// # Arguments
    /// * `model_path` - Directory containing model weights and tokenizer
    /// * `device` - Target device (CPU/CUDA/Metal)
    /// * `config` - Provider configuration
    pub fn from_pretrained(
        model_path: &str,
        device: Device,
        config: LLaVAConfig,
    ) -> Result<Self, String> {
        // Load LLaVA config
        let llava_config_path = format!("{}/config.json", model_path);
        let llava_config: CandleLLaVAConfig = serde_json::from_slice(
            &std::fs::read(&llava_config_path)
                .map_err(|e| format!("Failed to read config: {}", e))?,
        )
        .map_err(|e| format!("Failed to parse config: {}", e))?;

        // Create channel for communication with model thread
        let (request_tx, request_rx) = mpsc::channel::<LLaVARequest>();
        
        // Create initialization channel to signal success/failure
        let (init_tx, init_rx) = mpsc::channel::<Result<(), String>>();

        // Clone data needed for thread (all Send types)
        let model_path_clone = model_path.to_string();
        let config_clone = config.clone();
        let llava_config_clone = llava_config.clone();

        // Spawn dedicated thread that creates and owns the model
        // This avoids moving non-Send types across thread boundaries
        thread::spawn(move || {
            // Load tokenizer inside thread
            let tokenizer_path = format!("{}/tokenizer.json", model_path_clone);
            let tokenizer = match Tokenizer::from_file(&tokenizer_path) {
                Ok(t) => t,
                Err(e) => {
                    let err_msg = format!("Failed to load tokenizer: {}", e);
                    let _ = init_tx.send(Err(err_msg));
                    return;
                }
            };

            // Load model weights inside thread
            let dtype = candle_core::DType::F16;
            let weight_files: Vec<std::path::PathBuf> =
                vec![format!("{}/model.safetensors", model_path_clone).into()];

            let vb = match unsafe { VarBuilder::from_mmaped_safetensors(&weight_files, dtype, &device) } {
                Ok(vb) => vb,
                Err(e) => {
                    let err_msg = format!("Failed to load weights: {}", e);
                    let _ = init_tx.send(Err(err_msg));
                    return;
                }
            };

            // Load LLaVA model (unified vision + language) inside thread
            let model = match LLaVA::load(vb, &llava_config_clone, None) {
                Ok(m) => m,
                Err(e) => {
                    let err_msg = format!("Failed to create LLaVA model: {}", e);
                    let _ = init_tx.send(Err(err_msg));
                    return;
                }
            };

            // Signal successful initialization
            let _ = init_tx.send(Ok(()));
            
            // Run model thread - model never leaves this thread
            Self::model_thread(model, tokenizer, config_clone, llava_config_clone, device, model_path_clone, request_rx)
        });

        // Wait for initialization to complete or fail
        match init_rx.recv() {
            Ok(Ok(())) => {
                // Initialization successful, continue
            }
            Ok(Err(e)) => {
                return Err(e);
            }
            Err(_) => {
                return Err("Model thread panicked during initialization".to_string());
            }
        }

        // Create engine for stream orchestration
        let engine = Engine::new(crate::core::EngineConfig::default())
            .map_err(|e| format!("Failed to create engine: {}", e))?;

        Ok(Self {
            request_tx,
            config,
            llava_config,
            model_path: model_path.to_string(),
            _engine: engine,
        })
    }

    /// Dedicated thread that owns the LLaVA model and processes requests
    fn model_thread(
        model: LLaVA,
        tokenizer: Tokenizer,
        config: LLaVAConfig,
        llava_config: CandleLLaVAConfig,
        device: Device,
        model_path: String,
        request_rx: mpsc::Receiver<LLaVARequest>,
    ) {
        while let Ok(request) = request_rx.recv() {
            match request {
                LLaVARequest::Ask { image_path, question, response_tx } => {
                    let result = Self::process_ask_sync(
                        &model,
                        &tokenizer,
                        &config,
                        &llava_config,
                        &device,
                        &model_path,
                        &image_path,
                        &question,
                    );
                    let _ = response_tx.send(result);
                }
                LLaVARequest::AskUrl { image_url, question, response_tx } => {
                    let result = Self::process_ask_url_sync(
                        &model,
                        &tokenizer,
                        &config,
                        &llava_config,
                        &device,
                        &model_path,
                        &image_url,
                        &question,
                    );
                    let _ = response_tx.send(result);
                }
                LLaVARequest::Shutdown => break,
            }
        }
    }

    /// Process ask request synchronously on model thread
    fn process_ask_sync(
        model: &LLaVA,
        tokenizer: &Tokenizer,
        config: &LLaVAConfig,
        llava_config: &CandleLLaVAConfig,
        device: &Device,
        _model_path: &str,
        image_path: &str,
        question: &str,
    ) -> Result<String, String> {
        // 1. Preprocess image
        let image_tensor = Self::preprocess_image_sync(config, device, image_path)?;
        let image_size = (config.image_size as u32, config.image_size as u32);

        // 2. Format prompt with image token
        let prompt = format!("USER: <image>\n{}\nASSISTANT:", question);

        // 3. Tokenize prompt (handles <image> token)
        let input_ids = Self::tokenize_image_prompt_static(tokenizer, llava_config, device, &prompt)?;

        // 4. Prepare multimodal embeddings (vision + text fusion)
        let image_batch = image_tensor
            .unsqueeze(0)
            .map_err(|e| format!("Image batch failed: {}", e))?;

        let input_embeds = model
            .prepare_inputs_labels_for_multimodal(&input_ids, &[image_batch], &[image_size])
            .map_err(|e| format!("Embedding preparation failed: {}", e))?;

        // 5. Create KV cache
        let llama_config = llava_config.to_llama_config();
        let mut cache = Cache::new(
            config.use_kv_cache,
            candle_core::DType::F16,
            &llama_config,
            device,
        )
        .map_err(|e| format!("Cache creation failed: {}", e))?;

        // 6. Generate response (autoregressive loop)
        let mut generated_text = String::new();
        let mut current_embeds = input_embeds;

        let mut index_pos = 0;
        for index in 0..config.max_new_tokens {
            // Get current embedding dimensions
            let (_, input_embeds_len, _) = current_embeds
                .dims3()
                .map_err(|e| format!("Failed to get embed dims: {}", e))?;

            // Determine context size and index based on cache state
            let (context_size, context_index) = if config.use_kv_cache && index > 0 {
                (1, index_pos) // Only new embedding after first iteration
            } else {
                (input_embeds_len, 0) // All embeddings on first iteration
            };

            // Slice embeddings to pass only relevant portion
            let input = current_embeds
                .i((.., input_embeds_len.saturating_sub(context_size).., ..))
                .map_err(|e| format!("Failed to slice embeddings: {}", e))?;

            // Forward pass with sliced input
            let logits = model
                .forward(&input, context_index, &mut cache)
                .map_err(|e| format!("Generation failed: {}", e))?;

            // Update position tracking
            let (_, input_len, _) = input
                .dims3()
                .map_err(|e| format!("Failed to get input dims: {}", e))?;
            index_pos += input_len;

            // Sample next token
            let next_token = Self::sample_token_static(config, &logits)?;

            // Check EOS
            if next_token == llava_config.eos_token_id as u32 {
                break;
            }

            // Decode token
            if let Ok(text) = tokenizer.decode(&[next_token], false) {
                generated_text.push_str(&text);
            }

            // Embed next token and append
            let next_token_tensor = Tensor::new(&[next_token], device)
                .map_err(|e| format!("Token tensor failed: {}", e))?;
            let next_embeds = model
                .llama
                .embed(&next_token_tensor)
                .map_err(|e| format!("Embedding failed: {}", e))?
                .unsqueeze(0)
                .map_err(|e| format!("Unsqueeze failed: {}", e))?;

            current_embeds = Tensor::cat(&[current_embeds, next_embeds], 1)
                .map_err(|e| format!("Embedding concat failed: {}", e))?;
        }

        Ok(generated_text)
    }

    /// Process ask_url request synchronously on model thread
    fn process_ask_url_sync(
        model: &LLaVA,
        tokenizer: &Tokenizer,
        config: &LLaVAConfig,
        llava_config: &CandleLLaVAConfig,
        device: &Device,
        _model_path: &str,
        image_url: &str,
        question: &str,
    ) -> Result<String, String> {
        // 1. Preprocess image from URL
        let image_tensor = Self::preprocess_image_url_sync(config, device, image_url)?;
        let image_size = (config.image_size as u32, config.image_size as u32);

        // 2. Format prompt with image token
        let prompt = format!("USER: <image>\n{}\nASSISTANT:", question);

        // 3. Tokenize prompt (handles <image> token)
        let input_ids = Self::tokenize_image_prompt_static(tokenizer, llava_config, device, &prompt)?;

        // 4. Prepare multimodal embeddings (vision + text fusion)
        let image_batch = image_tensor
            .unsqueeze(0)
            .map_err(|e| format!("Image batch failed: {}", e))?;

        let input_embeds = model
            .prepare_inputs_labels_for_multimodal(&input_ids, &[image_batch], &[image_size])
            .map_err(|e| format!("Embedding preparation failed: {}", e))?;

        // 5. Create KV cache
        let llama_config = llava_config.to_llama_config();
        let mut cache = Cache::new(
            config.use_kv_cache,
            candle_core::DType::F16,
            &llama_config,
            device,
        )
        .map_err(|e| format!("Cache creation failed: {}", e))?;

        // 6. Generate response (autoregressive loop)
        let mut generated_text = String::new();
        let mut current_embeds = input_embeds;

        let mut index_pos = 0;
        for index in 0..config.max_new_tokens {
            // Get current embedding dimensions
            let (_, input_embeds_len, _) = current_embeds
                .dims3()
                .map_err(|e| format!("Failed to get embed dims: {}", e))?;

            // Determine context size and index based on cache state
            let (context_size, context_index) = if config.use_kv_cache && index > 0 {
                (1, index_pos) // Only new embedding after first iteration
            } else {
                (input_embeds_len, 0) // All embeddings on first iteration
            };

            // Slice embeddings to pass only relevant portion
            let input = current_embeds
                .i((.., input_embeds_len.saturating_sub(context_size).., ..))
                .map_err(|e| format!("Failed to slice embeddings: {}", e))?;

            // Forward pass with sliced input
            let logits = model
                .forward(&input, context_index, &mut cache)
                .map_err(|e| format!("Generation failed: {}", e))?;

            // Update position tracking
            let (_, input_len, _) = input
                .dims3()
                .map_err(|e| format!("Failed to get input dims: {}", e))?;
            index_pos += input_len;

            // Sample next token
            let next_token = Self::sample_token_static(config, &logits)?;

            // Check EOS
            if next_token == llava_config.eos_token_id as u32 {
                break;
            }

            // Decode token
            if let Ok(text) = tokenizer.decode(&[next_token], false) {
                generated_text.push_str(&text);
            }

            // Embed next token and append
            let next_token_tensor = Tensor::new(&[next_token], device)
                .map_err(|e| format!("Token tensor failed: {}", e))?;
            let next_embeds = model
                .llama
                .embed(&next_token_tensor)
                .map_err(|e| format!("Embedding failed: {}", e))?
                .unsqueeze(0)
                .map_err(|e| format!("Unsqueeze failed: {}", e))?;

            current_embeds = Tensor::cat(&[current_embeds, next_embeds], 1)
                .map_err(|e| format!("Embedding concat failed: {}", e))?;
        }

        Ok(generated_text)
    }

    /// Create LLaVA provider with automatic model download
    ///
    /// Downloads LLaVA model from HuggingFace and initializes provider.
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let config = LLaVAConfig::default();
        Self::with_config(config).await
    }

    /// Create provider with config and auto-download
    pub async fn with_config(
        config: LLaVAConfig,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Use factory for download provider
        let downloader = DownloadProviderFactory::create_default()?;

        // Download LLaVA model
        let result = downloader
            .download_model(
                "llava-hf/llava-1.5-7b-hf",
                vec![
                    "*.safetensors".to_string(),
                    "config.json".to_string(),
                    "tokenizer.json".to_string(),
                ],
                None,
            )
            .collect()
            .map_err(|e| {
                Box::<dyn std::error::Error + Send + Sync>::from(format!(
                    "Download task failed: {}",
                    e
                ))
            })??;

        let model_path = result.cache_dir.to_str().ok_or("Invalid cache directory")?;

        Self::from_pretrained(
            model_path,
            Device::cuda_if_available(0).unwrap_or(Device::Cpu),
            config,
        )
        .map_err(|e| Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Preprocess image from file path using LLaVA normalization (sync version for thread)
    ///
    /// LLaVA uses two-stage normalization (CRITICAL - different from CLIP):
    /// 1. [0,255] → [0,1] via normalize_unsigned()
    /// 2. (x - mean) / std via normalize_with()
    fn preprocess_image_sync(config: &LLaVAConfig, device: &Device, image_path: &str) -> Result<Tensor, String> {
        // Use blocking runtime for sync context
        let runtime = tokio::runtime::Runtime::new().map_err(|e| format!("Runtime creation failed: {}", e))?;
        runtime.block_on(async {
            Image::from_path(image_path)
                .resize(
                    config.image_size,
                    config.image_size,
                    ResizeFilter::CatmullRom,
                )
                .normalize_unsigned()
                .normalize_with(config.image_mean, config.image_std)
                .to_tensor(device)
                .await
        })
    }

    /// Preprocess image from URL (sync version for thread)
    fn preprocess_image_url_sync(config: &LLaVAConfig, device: &Device, url: &str) -> Result<Tensor, String> {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| format!("Runtime creation failed: {}", e))?;
        runtime.block_on(async {
            Image::from_url(url)
                .resize(
                    config.image_size,
                    config.image_size,
                    ResizeFilter::CatmullRom,
                )
                .normalize_unsigned()
                .normalize_with(config.image_mean, config.image_std)
                .to_tensor(device)
                .await
        })
    }

    /// Tokenize prompt with image tokens (static version for thread)
    ///
    /// Handles <image> placeholder insertion for multimodal embeddings
    fn tokenize_image_prompt_static(
        tokenizer: &Tokenizer,
        llava_config: &CandleLLaVAConfig,
        device: &Device,
        prompt: &str,
    ) -> Result<Tensor, String> {
        let image_token_index = llava_config.image_token_index as i64;

        // Split by <image> and tokenize chunks (avoid unwrap in map)
        let mut chunks: Vec<Vec<i64>> = Vec::new();
        for s in prompt.split("<image>") {
            let encoding = tokenizer
                .encode(s, true)
                .map_err(|e| format!("Tokenization failed: {}", e))?;
            chunks.push(encoding.get_ids().iter().map(|x| *x as i64).collect());
        }

        // Interleave text tokens with image tokens
        let mut input_ids = Vec::new();
        let mut offset = 0;

        if !chunks.is_empty()
            && !chunks[0].is_empty()
            && chunks[0][0] == llava_config.bos_token_id as i64
        {
            offset = 1;
            input_ids.push(chunks[0][0]);
        }

        for (i, chunk) in chunks.iter().enumerate() {
            if i > 0 {
                input_ids.push(image_token_index);
            }
            input_ids.extend(&chunk[offset..]);
            offset = 0;
        }

        let input_len = input_ids.len();
        Tensor::from_vec(input_ids, (1, input_len), device)
            .map_err(|e| format!("Tokenization tensor failed: {}", e))
    }

    /// Sample next token from logits using LogitsProcessor (static version for thread)
    fn sample_token_static(config: &LLaVAConfig, logits: &Tensor) -> Result<u32, String> {
        let logits = logits
            .squeeze(0)
            .map_err(|e| format!("Logits squeeze failed: {}", e))?;

        // Use LogitsProcessor for sampling
        let sampling = if config.temperature <= 0.0 {
            Sampling::ArgMax
        } else {
            Sampling::All {
                temperature: config.temperature,
            }
        };

        let mut processor = LogitsProcessor::from_sampling(299792458, sampling);
        processor
            .sample(&logits)
            .map_err(|e| format!("Sampling failed: {}", e))
    }

    /// Describe an image with a text query (sends request to model thread)
    ///
    /// Uses multimodal embedding fusion and autoregressive generation
    pub fn describe_image(&self, image_path: &str, query: &str) -> AsyncStream<CandleStringChunk> {
        let (response_tx, response_rx) = mpsc::channel();
        
        if let Err(e) = self.request_tx.send(LLaVARequest::Ask {
            image_path: image_path.to_string(),
            question: query.to_string(),
            response_tx,
        }) {
            return AsyncStream::with_channel(move |sender| {
                let _ = sender.send(CandleStringChunk(format!("Error: Failed to send request: {}", e)));
            });
        }
        
        match response_rx.recv() {
            Ok(Ok(text)) => {
                // Emit full text as single chunk for efficiency
                AsyncStream::with_channel(move |sender| {
                    let _ = sender.send(CandleStringChunk(text));
                })
            }
            Ok(Err(e)) => AsyncStream::with_channel(move |sender| {
                let _ = sender.send(CandleStringChunk(format!("Error: {}", e)));
            }),
            Err(e) => AsyncStream::with_channel(move |sender| {
                let _ = sender.send(CandleStringChunk(format!("Error: Failed to receive response: {}", e)));
            }),
        }
    }

    /// Describe an image from URL with a text query (sends request to model thread)
    ///
    /// Same as describe_image() but loads image from URL
    pub fn describe_url(&self, image_url: &str, query: &str) -> AsyncStream<CandleStringChunk> {
        let (response_tx, response_rx) = mpsc::channel();
        
        if let Err(e) = self.request_tx.send(LLaVARequest::AskUrl {
            image_url: image_url.to_string(),
            question: query.to_string(),
            response_tx,
        }) {
            return AsyncStream::with_channel(move |sender| {
                let _ = sender.send(CandleStringChunk(format!("Error: Failed to send request: {}", e)));
            });
        }
        
        match response_rx.recv() {
            Ok(Ok(text)) => {
                // Emit full text as single chunk for efficiency
                AsyncStream::with_channel(move |sender| {
                    let _ = sender.send(CandleStringChunk(text));
                })
            }
            Ok(Err(e)) => AsyncStream::with_channel(move |sender| {
                let _ = sender.send(CandleStringChunk(format!("Error: {}", e)));
            }),
            Err(e) => AsyncStream::with_channel(move |sender| {
                let _ = sender.send(CandleStringChunk(format!("Error: Failed to receive response: {}", e)));
            }),
        }
    }

    /// Stream chat responses token by token
    ///
    /// **NOTE**: Due to Candle's LLaVA containing non-Send trait objects,
    /// streaming happens after full generation. Returns buffered AsyncStream.
    ///
    /// For true streaming, await the entire response then iterate the stream.
    pub fn stream_chat(
        &self,
        image_path: &str,
        question: &str,
    ) -> AsyncStream<CandleStringChunk> {
        // Use describe_image which handles channel communication and streaming
        self.describe_image(image_path, question)
    }
}

impl CandleModel for LLaVAModel {
    fn info(&self) -> &'static CandleModelInfo {
        &LLAVA_MODEL_INFO
    }
}

impl Default for LLaVAModel {
    fn default() -> Self {
        // Create dummy channel for registry stub
        let (request_tx, _) = mpsc::channel();
        
        // Create minimal default LLaVA config (based on LLaVA 1.5-7B structure)
        // This stub is only used for registry initialization and should never be used for actual inference
        let llava_config = CandleLLaVAConfig {
            architectures: vec!["LlavaLlamaForCausalLM".to_string()],
            bos_token_id: 1,
            eos_token_id: 2,
            hidden_size: 4096,
            image_aspect_ratio: "pad".to_string(),
            image_crop_resolution: 224,
            image_grid_pinpoints: vec![],
            image_split_resolution: 224,
            intermediate_size: 11008,
            max_position_embeddings: 4096,
            mm_hidden_size: 1024,
            mm_patch_merge_type: "flat".to_string(),
            mm_projector_type: "mlp2x_gelu".to_string(),
            mm_use_im_start_end: false,
            mm_vision_select_feature: "patch".to_string(),
            mm_vision_select_layer: -2,
            mm_vision_tower: None,
            model_type: "llava".to_string(),
            num_attention_heads: 32,
            num_hidden_layers: 32,
            num_key_value_heads: 32,
            pad_token_id: 0,
            rms_norm_eps: 1e-5,
            rope_theta: 10000.0,
            tokenizer_model_max_length: Some(4096),
            torch_dtype: "float16".to_string(),
            use_cache: true,
            vocab_size: 32000,
            image_token_index: -200,
            hf: false,
            tie_word_embeddings: Some(false),
        };
        
        // Create default engine
        let engine = Engine::default();
        
        Self {
            request_tx,
            config: LLaVAConfig::default(),
            llava_config,
            model_path: String::new(),
            _engine: engine,
        }
    }
}
