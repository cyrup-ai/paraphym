//! LLaVA (Large Language and Vision Assistant) provider
//!
//! This module wraps Candle's unified LLaVA model for vision-language understanding.
//! Supports visual question answering, image description, and multi-turn conversations.

use std::num::NonZeroU32;
use std::sync::{mpsc, Arc, Mutex};
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
    #[allow(dead_code)] // Reserved for graceful shutdown implementation
    Shutdown,
}



/// LLaVA vision-language provider
///
/// Wraps Candle's unified LLaVA model (CLIP vision + LLaMA language)
/// for image understanding and visual question answering.
///
/// The actual LLaVA model runs on a dedicated thread to avoid Send/Sync issues
/// with Candle's Module trait. Communication happens via channels.
/// Thread spawns lazily on first use.
#[derive(Debug)]
pub struct LLaVAModel {
    request_tx: Arc<Mutex<Option<mpsc::Sender<LLaVARequest>>>>,
    _engine: Engine,
}

// Static model info for LLaVA 1.5
pub static LLAVA_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: CandleProvider::LLaVAHF,
    name: "llava-1.5-7b-hf",
    registry_key: "llava-hf/llava-1.5-7b-hf",
    quantization_url: None,
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
    embedding_dimension: None,
    vocab_size: None,
    image_size: Some(336),
    image_mean: Some([0.48145466, 0.4578275, 0.40821073]),
    image_std: Some([0.26862954, 0.2613026, 0.2757771]),
    default_temperature: Some(0.2),
    default_top_k: None,
    default_top_p: None,
    supports_kv_cache: true,
    supports_flash_attention: false,
    use_bf16: false,
    default_steps: None,
    default_guidance_scale: None,
    time_shift: None,
    est_memory_allocation_mb: 0,
};

impl LLaVAModel {
    /// Create new LLaVA model (lazy initialization)
    /// 
    /// Thread spawns on first describe_image() or describe_url() call.
    /// All configuration comes from LLAVA_MODEL_INFO.
    pub fn new() -> Self {
        Self {
            request_tx: Arc::new(Mutex::new(None)),
            _engine: Engine::default(),
        }
    }

    /// Ensure model thread is spawned (lazy initialization)
    ///
    /// Returns sender for communication with model thread.
    /// Thread spawns on first call, subsequent calls return cached sender.
    fn ensure_thread_spawned(&self) -> Result<mpsc::Sender<LLaVARequest>, Box<dyn std::error::Error + Send + Sync>> {
        // Lock the Option<Sender> - prevents race conditions
        let mut tx_guard = self.request_tx.lock()
            .map_err(|e| format!("Lock poisoned: {}", e))?;

        // Check if thread already spawned
        if let Some(sender) = tx_guard.as_ref() {
            return Ok(sender.clone());
        }
        
        // === FIRST USE: Initialize thread ===
        
        // Step 1: Get model files via huggingface_file() BEFORE spawning
        // This downloads files if needed and returns cached paths
        let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json")?;
        let weights_path = self.huggingface_file(self.info().registry_key, "model.safetensors")?;
        let config_path = self.huggingface_file(self.info().registry_key, "config.json")?;
        
        // Step 2: Load LLaVA config (CandleLLaVAConfig, not our deleted LLaVAConfig!)
        let llava_config: CandleLLaVAConfig = serde_json::from_slice(
            &std::fs::read(&config_path)
                .map_err(|e| format!("Failed to read config: {}", e))?
        ).map_err(|e| format!("Failed to parse config: {}", e))?;
        
        // Step 3: Create channels for request/response
        let (request_tx, request_rx) = mpsc::channel();
        let (init_tx, init_rx) = mpsc::channel();
        
        // Step 4: Determine device
        let device = Device::cuda_if_available(0).unwrap_or(Device::Cpu);
        
        // Step 5: Extract ALL config from ModelInfo (self.info())
        // These will be passed to thread as parameters
        let image_size = self.info().image_size
            .ok_or("image_size not in ModelInfo")? as usize;
        let image_mean = self.info().image_mean
            .ok_or("image_mean not in ModelInfo")?;
        let image_std = self.info().image_std
            .ok_or("image_std not in ModelInfo")?;
        let temperature = self.info().default_temperature
            .ok_or("default_temperature not in ModelInfo")?;
        let max_new_tokens = self.info().max_output_tokens
            .ok_or("max_output_tokens not in ModelInfo")?.get() as usize;
        let use_kv_cache = self.info().supports_kv_cache;
        
        // Step 6: Spawn dedicated thread with async runtime
        thread::spawn(move || {
            // Create tokio runtime for async image operations
            let rt = match tokio::runtime::Runtime::new() {
                Ok(rt) => rt,
                Err(e) => {
                    let _ = init_tx.send(Err(format!("Runtime creation failed: {}", e)));
                    return;
                }
            };

            // Load tokenizer INSIDE thread
            let tokenizer = match Tokenizer::from_file(&tokenizer_path) {
                Ok(t) => t,
                Err(e) => {
                    let _ = init_tx.send(Err(format!("Tokenizer load failed: {}", e)));
                    return;
                }
            };

            // Load model weights INSIDE thread
            let vb = match unsafe {
                VarBuilder::from_mmaped_safetensors(&[weights_path], candle_core::DType::F16, &device)
            } {
                Ok(vb) => vb,
                Err(e) => {
                    let _ = init_tx.send(Err(format!("VarBuilder failed: {}", e)));
                    return;
                }
            };

            // Load LLaVA model INSIDE thread (this is the non-Send part!)
            let model = match LLaVA::load(vb, &llava_config, None) {
                Ok(m) => m,
                Err(e) => {
                    let _ = init_tx.send(Err(format!("Model load failed: {}", e)));
                    return;
                }
            };

            // Signal successful initialization
            let _ = init_tx.send(Ok(()));

            // Run model thread with config parameters and runtime
            Self::model_thread_with_config(
                model,
                tokenizer,
                llava_config,
                device,
                request_rx,
                image_size,
                image_mean,
                image_std,
                temperature,
                max_new_tokens,
                use_kv_cache,
                rt,
            );
        });
        
        // Step 7: Wait for initialization to complete
        match init_rx.recv() {
            Ok(Ok(())) => {},
            Ok(Err(e)) => return Err(e.into()),
            Err(e) => return Err(format!("Init channel failed: {}", e).into()),
        };
        
        // Step 8: Store sender for future calls
        *tx_guard = Some(request_tx.clone());
        Ok(request_tx)
    }

    /// Model thread that processes requests (runs forever until shutdown)
    ///
    /// All config values passed as parameters (from ModelInfo via ensure_thread_spawned)
    fn model_thread_with_config(
        model: LLaVA,
        tokenizer: Tokenizer,
        llava_config: CandleLLaVAConfig,
        device: Device,
        request_rx: mpsc::Receiver<LLaVARequest>,
        image_size: usize,
        image_mean: [f32; 3],
        image_std: [f32; 3],
        temperature: f64,
        max_new_tokens: usize,
        use_kv_cache: bool,
        rt: tokio::runtime::Runtime,
    ) {
        while let Ok(request) = request_rx.recv() {
            match request {
                LLaVARequest::Ask { image_path, question, response_tx } => {
                    let result = rt.block_on(async {
                        Self::process_ask(
                            &model,
                            &tokenizer,
                            &llava_config,
                            &device,
                            &image_path,
                            &question,
                            image_size,
                            image_mean,
                            image_std,
                            temperature,
                            max_new_tokens,
                            use_kv_cache,
                        ).await
                    });
                    let _ = response_tx.send(result);
                }
                LLaVARequest::AskUrl { image_url, question, response_tx } => {
                    let result = rt.block_on(async {
                        Self::process_ask_url(
                            &model,
                            &tokenizer,
                            &llava_config,
                            &device,
                            &image_url,
                            &question,
                            image_size,
                            image_mean,
                            image_std,
                            temperature,
                            max_new_tokens,
                            use_kv_cache,
                        ).await
                    });
                    let _ = response_tx.send(result);
                }
                LLaVARequest::Shutdown => break,
            }
        }
    }

    /// Process ask request asynchronously on model thread
    async fn process_ask(
        model: &LLaVA,
        tokenizer: &Tokenizer,
        llava_config: &CandleLLaVAConfig,
        device: &Device,
        image_path: &str,
        question: &str,
        image_size: usize,
        image_mean: [f32; 3],
        image_std: [f32; 3],
        temperature: f64,
        max_new_tokens: usize,
        use_kv_cache: bool,
    ) -> Result<String, String> {
        // 1. Preprocess image - async image loading
        let image_tensor = Image::from_path(image_path)
            .resize(image_size, image_size, ResizeFilter::CatmullRom)
            .normalize_unsigned()
            .normalize_with(image_mean, image_std)
            .to_tensor(device)
            .await?;
        let image_size_tuple = (image_size as u32, image_size as u32);

        // 2. Format prompt with image token
        let prompt = format!("USER: <image>\n{}\nASSISTANT:", question);

        // 3. Tokenize prompt (handles <image> token)
        let input_ids = Self::tokenize_image_prompt_static(tokenizer, llava_config, device, &prompt)?;

        // 4. Prepare multimodal embeddings (vision + text fusion)
        let image_batch = image_tensor
            .unsqueeze(0)
            .map_err(|e| format!("Image batch failed: {}", e))?;

        let input_embeds = model
            .prepare_inputs_labels_for_multimodal(&input_ids, &[image_batch], &[image_size_tuple])
            .map_err(|e| format!("Embedding preparation failed: {}", e))?;

        // 5. Create KV cache
        let llama_config = llava_config.to_llama_config();
        let mut cache = Cache::new(
            use_kv_cache,
            candle_core::DType::F16,
            &llama_config,
            device,
        )
        .map_err(|e| format!("Cache creation failed: {}", e))?;

        // 6. Generate response (autoregressive loop)
        let mut generated_text = String::new();
        let mut current_embeds = input_embeds;

        let mut index_pos = 0;
        for index in 0..max_new_tokens {
            // Get current embedding dimensions
            let (_, input_embeds_len, _) = current_embeds
                .dims3()
                .map_err(|e| format!("Failed to get embed dims: {}", e))?;

            // Determine context size and index based on cache state
            let (context_size, context_index) = if use_kv_cache && index > 0 {
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
            let next_token = Self::sample_token_static(temperature, &logits)?;

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

    /// Process ask_url request asynchronously on model thread
    async fn process_ask_url(
        model: &LLaVA,
        tokenizer: &Tokenizer,
        llava_config: &CandleLLaVAConfig,
        device: &Device,
        image_url: &str,
        question: &str,
        image_size: usize,
        image_mean: [f32; 3],
        image_std: [f32; 3],
        temperature: f64,
        max_new_tokens: usize,
        use_kv_cache: bool,
    ) -> Result<String, String> {
        // 1. Preprocess image from URL - async image loading
        let image_tensor = Image::from_url(image_url)
            .resize(image_size, image_size, ResizeFilter::CatmullRom)
            .normalize_unsigned()
            .normalize_with(image_mean, image_std)
            .to_tensor(device)
            .await?;
        let image_size_tuple = (image_size as u32, image_size as u32);

        // 2. Format prompt with image token
        let prompt = format!("USER: <image>\n{}\nASSISTANT:", question);

        // 3. Tokenize prompt (handles <image> token)
        let input_ids = Self::tokenize_image_prompt_static(tokenizer, llava_config, device, &prompt)?;

        // 4. Prepare multimodal embeddings (vision + text fusion)
        let image_batch = image_tensor
            .unsqueeze(0)
            .map_err(|e| format!("Image batch failed: {}", e))?;

        let input_embeds = model
            .prepare_inputs_labels_for_multimodal(&input_ids, &[image_batch], &[image_size_tuple])
            .map_err(|e| format!("Embedding preparation failed: {}", e))?;

        // 5. Create KV cache
        let llama_config = llava_config.to_llama_config();
        let mut cache = Cache::new(
            use_kv_cache,
            candle_core::DType::F16,
            &llama_config,
            device,
        )
        .map_err(|e| format!("Cache creation failed: {}", e))?;

        // 6. Generate response (autoregressive loop)
        let mut generated_text = String::new();
        let mut current_embeds = input_embeds;

        let mut index_pos = 0;
        for index in 0..max_new_tokens {
            // Get current embedding dimensions
            let (_, input_embeds_len, _) = current_embeds
                .dims3()
                .map_err(|e| format!("Failed to get embed dims: {}", e))?;

            // Determine context size and index based on cache state
            let (context_size, context_index) = if use_kv_cache && index > 0 {
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
            let next_token = Self::sample_token_static(temperature, &logits)?;

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
    fn sample_token_static(temperature: f64, logits: &Tensor) -> Result<u32, String> {
        let logits = logits
            .squeeze(0)
            .map_err(|e| format!("Logits squeeze failed: {}", e))?;

        // Use LogitsProcessor for sampling
        let sampling = if temperature <= 0.0 {
            Sampling::ArgMax
        } else {
            Sampling::All { temperature }
        };

        let mut processor = LogitsProcessor::from_sampling(299792458, sampling);
        processor
            .sample(&logits)
            .map_err(|e| format!("Sampling failed: {}", e))
    }

    /// Describe an image with a text query (sends request to model thread)
    ///
    /// Thread spawns lazily on first call. Uses multimodal embedding fusion
    /// and autoregressive generation.
    pub fn describe_image(&self, image_path: &str, query: &str) -> AsyncStream<CandleStringChunk> {
        // Ensure thread is spawned (lazy initialization)
        let sender = match self.ensure_thread_spawned() {
            Ok(s) => s,
            Err(e) => {
                return AsyncStream::with_channel(move |tx| {
                    let _ = tx.send(CandleStringChunk(format!("Error: {}", e)));
                });
            }
        };

        let (response_tx, response_rx) = mpsc::channel();

        if let Err(e) = sender.send(LLaVARequest::Ask {
            image_path: image_path.to_string(),
            question: query.to_string(),
            response_tx,
        }) {
            return AsyncStream::with_channel(move |tx| {
                let _ = tx.send(CandleStringChunk(format!("Error: {}", e)));
            });
        }

        match response_rx.recv() {
            Ok(Ok(text)) => {
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

    /// Describe an image from URL with a text query
    ///
    /// Same as describe_image() but loads image from URL
    pub fn describe_url(&self, image_url: &str, query: &str) -> AsyncStream<CandleStringChunk> {
        // Ensure thread is spawned (lazy initialization)
        let sender = match self.ensure_thread_spawned() {
            Ok(s) => s,
            Err(e) => {
                return AsyncStream::with_channel(move |tx| {
                    let _ = tx.send(CandleStringChunk(format!("Error: {}", e)));
                });
            }
        };
        
        let (response_tx, response_rx) = mpsc::channel();
        
        if let Err(e) = sender.send(LLaVARequest::AskUrl {
            image_url: image_url.to_string(),
            question: query.to_string(),
            response_tx,
        }) {
            return AsyncStream::with_channel(move |tx| {
                let _ = tx.send(CandleStringChunk(format!("Error: {}", e)));
            });
        }
        
        match response_rx.recv() {
            Ok(Ok(text)) => {
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
        Self::new()
    }
}

/// Loaded LLaVA model for pool workers
///
/// Wrapper around LLaVAModel that can be loaded in pool worker threads.
/// Delegates all VisionCapable trait methods to the wrapped model.
#[derive(Debug)]
pub struct LoadedLLaVAModel {
    model: LLaVAModel,
}

impl LoadedLLaVAModel {
    /// Load model from LLaVAModel configuration
    ///
    /// Creates a new LLaVAModel instance in the pool worker thread.
    pub fn load(config: &LLaVAModel) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Create new instance - LLaVAModel uses lazy initialization
        let _ = config; // Suppress unused warning
        Ok(Self {
            model: LLaVAModel::new(),
        })
    }
}

impl CandleModel for LoadedLLaVAModel {
    fn info(&self) -> &'static CandleModelInfo {
        self.model.info()
    }
}

impl crate::capability::traits::VisionCapable for LoadedLLaVAModel {
    fn describe_image(&self, image_path: &str, query: &str) -> AsyncStream<CandleStringChunk> {
        self.model.describe_image(image_path, query)
    }

    fn describe_url(&self, url: &str, query: &str) -> AsyncStream<CandleStringChunk> {
        self.model.describe_url(url, query)
    }
}
