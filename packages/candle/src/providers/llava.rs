//! LLaVA (Large Language and Vision Assistant) provider
//!
//! This module wraps Candle's unified LLaVA model for vision-language understanding.
//! Supports visual question answering, image description, and multi-turn conversations.

use candle_core::{Device, IndexOp, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::llava::{LLaVA, config::LLaVAConfig};
use candle_transformers::models::llama::Cache;
use candle_transformers::generation::{LogitsProcessor, Sampling};
use tokenizers::Tokenizer;

use crate::builders::image::{ImageBuilder, ResizeFilter};
use crate::domain::image::Image;
use crate::core::Engine;
use crate::domain::model::download::DownloadProviderFactory;
use crate::domain::context::CandleStringChunk;
use ystream::AsyncStream;

/// LLaVA model configuration
/// 
/// Default settings match LLaVA 1.5 with 336×336 images
/// and ImageNet normalization parameters.
#[derive(Clone, Debug)]
pub struct LLaVAProviderConfig {
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

impl Default for LLaVAProviderConfig {
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
pub struct LLaVAProvider {
    model: LLaVA,
    tokenizer: Tokenizer,
    config: LLaVAProviderConfig,
    llava_config: LLaVAConfig,
    device: Device,
    _engine: Engine,
}

impl LLaVAProvider {
    /// Load LLaVA model from local path
    /// 
    /// Expects safetensors files and tokenizer.json in model_path.
    /// 
    /// # Arguments
    /// * `model_path` - Directory containing model weights and tokenizer
    /// * `device` - Target device (CPU/CUDA/Metal)
    /// * `config` - Provider configuration
    pub fn from_pretrained(
        model_path: &str, 
        device: Device,
        config: LLaVAProviderConfig
    ) -> Result<Self, String> {
        // Load LLaVA config
        let llava_config_path = format!("{}/config.json", model_path);
        let llava_config: LLaVAConfig = serde_json::from_slice(
            &std::fs::read(&llava_config_path)
                .map_err(|e| format!("Failed to read config: {}", e))?
        ).map_err(|e| format!("Failed to parse config: {}", e))?;
        
        // Load tokenizer
        let tokenizer_path = format!("{}/tokenizer.json", model_path);
        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;
        
        // Load model weights
        let dtype = candle_core::DType::F16;
        let weight_files: Vec<std::path::PathBuf> = vec![format!("{}/model.safetensors", model_path).into()];
        
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&weight_files, dtype, &device)
        }.map_err(|e| format!("Failed to load weights: {}", e))?;
        
        // Load LLaVA model (unified vision + language)
        let model = LLaVA::load(vb, &llava_config, None)
            .map_err(|e| format!("Failed to create LLaVA model: {}", e))?;
        
        // Create engine for stream orchestration
        let engine = Engine::new(crate::core::EngineConfig::default())
            .map_err(|e| format!("Failed to create engine: {}", e))?;
        
        Ok(Self {
            model,
            tokenizer,
            config,
            llava_config,
            device,
            _engine: engine,
        })
    }
    
    /// Create LLaVA provider with automatic model download
    /// 
    /// Downloads LLaVA model from HuggingFace and initializes provider.
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let config = LLaVAProviderConfig::default();
        Self::with_config(config).await
    }
    
    /// Create provider with config and auto-download
    pub async fn with_config(
        config: LLaVAProviderConfig
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Use factory for download provider
        let downloader = DownloadProviderFactory::create_default()?;
        
        // Download LLaVA model
        let result = downloader.download_model(
            "llava-hf/llava-1.5-7b-hf",
            vec!["*.safetensors".to_string(), "config.json".to_string(), "tokenizer.json".to_string()],
            None,
        ).await?;
        
        let model_path = result.cache_dir.to_str()
            .ok_or("Invalid cache directory")?;
        
        Self::from_pretrained(
            model_path,
            Device::cuda_if_available(0).unwrap_or(Device::Cpu),
            config
        ).map_err(|e| Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Preprocess image from file path using LLaVA normalization
    /// 
    /// LLaVA uses two-stage normalization (CRITICAL - different from CLIP):
    /// 1. [0,255] → [0,1] via normalize_unsigned()
    /// 2. (x - mean) / std via normalize_with()
    async fn preprocess_image(&self, image_path: &str) -> Result<Tensor, String> {
        Image::from_path(image_path)
            .resize(
                self.config.image_size,
                self.config.image_size,
                ResizeFilter::CatmullRom
            )
            .normalize_unsigned()
            .normalize_with(
                self.config.image_mean,
                self.config.image_std
            )
            .to_tensor(&self.device)
            .await
    }

    /// Preprocess image from URL
    async fn preprocess_image_url(&self, url: &str) -> Result<Tensor, String> {
        Image::from_url(url)
            .resize(self.config.image_size, self.config.image_size, ResizeFilter::CatmullRom)
            .normalize_unsigned()
            .normalize_with(self.config.image_mean, self.config.image_std)
            .to_tensor(&self.device)
            .await
    }

    /// Preprocess image from base64
    async fn _preprocess_image_base64(&self, base64: &str) -> Result<Tensor, String> {
        Image::from_base64(base64)
            .resize(self.config.image_size, self.config.image_size, ResizeFilter::CatmullRom)
            .normalize_unsigned()
            .normalize_with(self.config.image_mean, self.config.image_std)
            .to_tensor(&self.device)
            .await
    }

    /// Tokenize prompt with image tokens
    /// 
    /// Handles <image> placeholder insertion for multimodal embeddings
    fn tokenize_image_prompt(&self, prompt: &str) -> Result<Tensor, String> {
        let image_token_index = self.llava_config.image_token_index as i64;
        
        // Split by <image> and tokenize chunks (avoid unwrap in map)
        let mut chunks: Vec<Vec<i64>> = Vec::new();
        for s in prompt.split("<image>") {
            let encoding = self.tokenizer.encode(s, true)
                .map_err(|e| format!("Tokenization failed: {}", e))?;
            chunks.push(
                encoding.get_ids()
                    .iter()
                    .map(|x| *x as i64)
                    .collect()
            );
        }
        
        // Interleave text tokens with image tokens
        let mut input_ids = Vec::new();
        let mut offset = 0;
        
        if !chunks.is_empty() && !chunks[0].is_empty() 
            && chunks[0][0] == self.llava_config.bos_token_id as i64 
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
        Tensor::from_vec(input_ids, (1, input_len), &self.device)
            .map_err(|e| format!("Tokenization tensor failed: {}", e))
    }

    /// Sample next token from logits using LogitsProcessor
    fn sample_token(&self, logits: &Tensor) -> Result<u32, String> {
        let logits = logits.squeeze(0)
            .map_err(|e| format!("Logits squeeze failed: {}", e))?;
        
        // Use LogitsProcessor for sampling
        let sampling = if self.config.temperature <= 0.0 {
            Sampling::ArgMax
        } else {
            Sampling::All { temperature: self.config.temperature }
        };
        
        let mut processor = LogitsProcessor::from_sampling(299792458, sampling);
        processor.sample(&logits)
            .map_err(|e| format!("Sampling failed: {}", e))
    }

    /// Answer question about an image
    /// 
    /// Uses multimodal embedding fusion and autoregressive generation
    pub async fn ask(
        &self,
        image_path: &str,
        question: &str
    ) -> Result<String, String> {
        // 1. Preprocess image
        let image_tensor = self.preprocess_image(image_path).await?;
        let image_size = (self.config.image_size as u32, self.config.image_size as u32);
        
        // 2. Format prompt with image token
        let prompt = format!("USER: <image>\n{}\nASSISTANT:", question);
        
        // 3. Tokenize prompt (handles <image> token)
        let input_ids = self.tokenize_image_prompt(&prompt)?;
        
        // 4. Prepare multimodal embeddings (vision + text fusion)
        let image_batch = image_tensor.unsqueeze(0)
            .map_err(|e| format!("Image batch failed: {}", e))?;
        
        let input_embeds = self.model
            .prepare_inputs_labels_for_multimodal(
                &input_ids, 
                &[image_batch], 
                &[image_size]
            )
            .map_err(|e| format!("Embedding preparation failed: {}", e))?;
        
        // 5. Create KV cache
        let llama_config = self.llava_config.to_llama_config();
        let mut cache = Cache::new(
            self.config.use_kv_cache,
            candle_core::DType::F16,
            &llama_config,
            &self.device
        ).map_err(|e| format!("Cache creation failed: {}", e))?;
        
        // 6. Generate response (autoregressive loop)
        let mut generated_text = String::new();
        let mut current_embeds = input_embeds;
        
        let mut index_pos = 0;
        for index in 0..self.config.max_new_tokens {
            // Get current embedding dimensions
            let (_, input_embeds_len, _) = current_embeds.dims3()
                .map_err(|e| format!("Failed to get embed dims: {}", e))?;
            
            // Determine context size and index based on cache state
            let (context_size, context_index) = if self.config.use_kv_cache && index > 0 {
                (1, index_pos)  // Only new embedding after first iteration
            } else {
                (input_embeds_len, 0)  // All embeddings on first iteration
            };
            
            // Slice embeddings to pass only relevant portion
            let input = current_embeds.i((.., input_embeds_len.saturating_sub(context_size).., ..))
                .map_err(|e| format!("Failed to slice embeddings: {}", e))?;
            
            // Forward pass with sliced input
            let logits = self.model.forward(&input, context_index, &mut cache)
                .map_err(|e| format!("Generation failed: {}", e))?;
            
            // Update position tracking
            let (_, input_len, _) = input.dims3()
                .map_err(|e| format!("Failed to get input dims: {}", e))?;
            index_pos += input_len;
            
            // Sample next token
            let next_token = self.sample_token(&logits)?;
            
            // Check EOS
            if next_token == self.llava_config.eos_token_id as u32 {
                break;
            }
            
            // Decode token
            if let Ok(text) = self.tokenizer.decode(&[next_token], false) {
                generated_text.push_str(&text);
            }
            
            // Embed next token and append
            let next_token_tensor = Tensor::new(&[next_token], &self.device)
                .map_err(|e| format!("Token tensor failed: {}", e))?;
            let next_embeds = self.model.llama.embed(&next_token_tensor)
                .map_err(|e| format!("Embedding failed: {}", e))?
                .unsqueeze(0)
                .map_err(|e| format!("Unsqueeze failed: {}", e))?;
            
            current_embeds = Tensor::cat(&[current_embeds, next_embeds], 1)
                .map_err(|e| format!("Embedding concat failed: {}", e))?;
        }
        
        Ok(generated_text)
    }

    /// Answer question about image from URL
    /// 
    /// Same as ask() but loads image from URL
    pub async fn ask_url(
        &self,
        image_url: &str,
        question: &str
    ) -> Result<String, String> {
        // 1. Preprocess image from URL
        let image_tensor = self.preprocess_image_url(image_url).await?;
        let image_size = (self.config.image_size as u32, self.config.image_size as u32);
        
        // 2. Format prompt with image token
        let prompt = format!("USER: <image>\n{}\nASSISTANT:", question);
        
        // 3. Tokenize prompt (handles <image> token)
        let input_ids = self.tokenize_image_prompt(&prompt)?;
        
        // 4. Prepare multimodal embeddings (vision + text fusion)
        let image_batch = image_tensor.unsqueeze(0)
            .map_err(|e| format!("Image batch failed: {}", e))?;
        
        let input_embeds = self.model
            .prepare_inputs_labels_for_multimodal(
                &input_ids, 
                &[image_batch], 
                &[image_size]
            )
            .map_err(|e| format!("Embedding preparation failed: {}", e))?;
        
        // 5. Create KV cache
        let llama_config = self.llava_config.to_llama_config();
        let mut cache = Cache::new(
            self.config.use_kv_cache,
            candle_core::DType::F16,
            &llama_config,
            &self.device
        ).map_err(|e| format!("Cache creation failed: {}", e))?;
        
        // 6. Generate response (autoregressive loop)
        let mut generated_text = String::new();
        let mut current_embeds = input_embeds;
        
        let mut index_pos = 0;
        for index in 0..self.config.max_new_tokens {
            // Get current embedding dimensions
            let (_, input_embeds_len, _) = current_embeds.dims3()
                .map_err(|e| format!("Failed to get embed dims: {}", e))?;
            
            // Determine context size and index based on cache state
            let (context_size, context_index) = if self.config.use_kv_cache && index > 0 {
                (1, index_pos)  // Only new embedding after first iteration
            } else {
                (input_embeds_len, 0)  // All embeddings on first iteration
            };
            
            // Slice embeddings to pass only relevant portion
            let input = current_embeds.i((.., input_embeds_len.saturating_sub(context_size).., ..))
                .map_err(|e| format!("Failed to slice embeddings: {}", e))?;
            
            // Forward pass with sliced input
            let logits = self.model.forward(&input, context_index, &mut cache)
                .map_err(|e| format!("Generation failed: {}", e))?;
            
            // Update position tracking
            let (_, input_len, _) = input.dims3()
                .map_err(|e| format!("Failed to get input dims: {}", e))?;
            index_pos += input_len;
            
            // Sample next token
            let next_token = self.sample_token(&logits)?;
            
            // Check EOS
            if next_token == self.llava_config.eos_token_id as u32 {
                break;
            }
            
            // Decode token
            if let Ok(text) = self.tokenizer.decode(&[next_token], false) {
                generated_text.push_str(&text);
            }
            
            // Embed next token and append
            let next_token_tensor = Tensor::new(&[next_token], &self.device)
                .map_err(|e| format!("Token tensor failed: {}", e))?;
            let next_embeds = self.model.llama.embed(&next_token_tensor)
                .map_err(|e| format!("Embedding failed: {}", e))?
                .unsqueeze(0)
                .map_err(|e| format!("Unsqueeze failed: {}", e))?;
            
            current_embeds = Tensor::cat(&[current_embeds, next_embeds], 1)
                .map_err(|e| format!("Embedding concat failed: {}", e))?;
        }
        
        Ok(generated_text)
    }
    
    /// Stream chat responses token by token
    /// 
    /// **NOTE**: Due to Candle's LLaVA containing non-Send trait objects,
    /// streaming happens after full generation. Returns buffered AsyncStream.
    /// 
    /// For true streaming, await the entire response then iterate the stream.
    pub async fn stream_chat(
        &self,
        image_path: &str,
        question: &str
    ) -> AsyncStream<CandleStringChunk> {
        // Generate all tokens first (unavoidable due to Send constraints)
        let result = self.ask(image_path, question).await;
        
        match result {
            Ok(text) => {
                // Split into chunks and stream
                let chunks: Vec<CandleStringChunk> = text
                    .chars()
                    .map(|c| CandleStringChunk(c.to_string()))
                    .collect();
                
                AsyncStream::with_channel(move |sender| {
                    for chunk in chunks {
                        if sender.send(chunk).is_err() {
                            break;
                        }
                    }
                })
            }
            Err(e) => {
                AsyncStream::with_channel(move |sender| {
                    let _ = sender.send(CandleStringChunk(format!("Error: {}", e)));
                })
            }
        }
    }
}
