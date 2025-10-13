//! FLUX.1-schnell provider
//!
//! Fast text-to-image generation using FLUX's 4-step diffusion model with dual text encoding
//! (T5-XXL + CLIP-L) for efficient inference.

use std::path::PathBuf;
use candle_core::{Device, DType, Tensor};
use candle_nn::{VarBuilder, Module};
use candle_transformers::models::{
    flux,
    t5::{T5EncoderModel, Config as T5Config},
    stable_diffusion::clip::{ClipTextTransformer, Config as ClipConfig},
};
use tokenizers::Tokenizer;
use ystream::AsyncStream;
use crate::domain::image_generation::{
    ImageGenerationModel, 
    ImageGenerationConfig, 
    ImageGenerationChunk
};
use crate::domain::model::traits::CandleModel;
use crate::domain::model::CandleModelInfo;

/// FLUX.1-schnell provider for fast 4-step text-to-image generation
#[derive(Clone, Debug)]
pub struct FluxSchnell {}

/// T5-XXL encoder with tokenizer
struct T5WithTokenizer {
    t5: T5EncoderModel,
    tokenizer: Tokenizer,
}

/// CLIP encoder with tokenizer
struct ClipWithTokenizer {
    clip: ClipTextTransformer,
    tokenizer: Tokenizer,
}

impl Default for FluxSchnell {
    fn default() -> Self {
        Self::new()
    }
}

impl FluxSchnell {
    #[inline]
    pub fn new() -> Self {
        Self {}
    }
}

impl ImageGenerationModel for FluxSchnell {
    fn generate(
        &self,
        prompt: &str,
        config: &ImageGenerationConfig,
        device: &Device,
    ) -> AsyncStream<ImageGenerationChunk> {
        let prompt = prompt.to_string();
        let config = config.clone();
        let device = device.clone();
        
        AsyncStream::with_channel(move |sender| {
            // === 1. GET CONFIGURATION VALUES ===
            
            // From ImageGenerationConfig (required fields, not optional)
            let width = config.width;
            let height = config.height;
            let steps = config.steps;
            let guidance_scale = config.guidance_scale;  // Note: FLUX schnell uses 0.0
            
            // From ModelInfo static (single source of truth)
            let use_bf16 = FLUX_SCHNELL_MODEL_INFO.use_bf16;  // true
            
            // === 2. SET RANDOM SEED (if provided) ===
            if let Some(seed) = config.seed
                && let Err(e) = device.set_seed(seed)
            {
                let _ = sender.send(ImageGenerationChunk::Error(
                    format!("Seed setting failed: {}", e)
                ));
                return;
            }
            
            // === 3. DETERMINE DTYPE ===
            let dtype = if use_bf16 {
                device.bf16_default_to_f32()  // Prefer BF16, fallback to F32
            } else {
                DType::F32
            };
            
            // === 4. LAZY LOAD ALL MODEL FILES USING HF_HUB API ===
            use hf_hub::api::sync::Api;
            
            let api = match Api::new() {
                Ok(api) => api,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("Failed to initialize HF API: {}", e)
                    ));
                    return;
                }
            };
            
            // FLUX files from main repository
            let flux_repo = api.model(FLUX_SCHNELL_MODEL_INFO.registry_key.to_string());
            let flux_path = match flux_repo.get("flux1-schnell.safetensors") {
                Ok(p) => p,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("Failed to download FLUX model: {}", e)
                    ));
                    return;
                }
            };
            let vae_path = match flux_repo.get("ae.safetensors") {
                Ok(p) => p,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("Failed to download VAE: {}", e)
                    ));
                    return;
                }
            };
            
            // T5 files from google repository
            let t5_repo = api.model("google/t5-v1_1-xxl".to_string());
            let t5_model_path = match t5_repo.get("model.safetensors") {
                Ok(p) => p,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("Failed to download T5 model: {}", e)
                    ));
                    return;
                }
            };
            let t5_config_path = match t5_repo.get("config.json") {
                Ok(p) => p,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("Failed to download T5 config: {}", e)
                    ));
                    return;
                }
            };
            
            // T5 tokenizer from separate repository
            let t5_tok_repo = api.model("lmz/mt5-tokenizers".to_string());
            let t5_tokenizer_path = match t5_tok_repo.get("t5-v1_1-xxl.tokenizer.json") {
                Ok(p) => p,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("Failed to download T5 tokenizer: {}", e)
                    ));
                    return;
                }
            };
            
            // CLIP files from openai repository
            let clip_repo = api.model("openai/clip-vit-large-patch14".to_string());
            let clip_model_path = match clip_repo.get("model.safetensors") {
                Ok(p) => p,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("Failed to download CLIP model: {}", e)
                    ));
                    return;
                }
            };
            let clip_tokenizer_path = match clip_repo.get("tokenizer.json") {
                Ok(p) => p,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("Failed to download CLIP tokenizer: {}", e)
                    ));
                    return;
                }
            };
            
            // === 5. LOAD TEXT ENCODERS ===
            let mut t5_encoder = match T5WithTokenizer::load(
                &t5_model_path,
                &t5_config_path,
                &t5_tokenizer_path,  // Now passed as parameter
                dtype,
                &device,
            ) {
                Ok(encoder) => encoder,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("T5 encoder load failed: {}", e)
                    ));
                    return;
                }
            };
            
            let clip_encoder = match ClipWithTokenizer::load(
                &clip_model_path,
                &clip_tokenizer_path,  // Now passed as parameter
                dtype,
                &device,
            ) {
                Ok(encoder) => encoder,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("CLIP encoder load failed: {}", e)
                    ));
                    return;
                }
            };
            
            // === 6. ENCODE TEXT PROMPT ===
            let t5_emb = match t5_encoder.encode(&prompt, &device) {
                Ok(emb) => emb,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("T5 encoding failed: {}", e)
                    ));
                    return;
                }
            };
            
            let clip_emb = match clip_encoder.encode(&prompt, &device) {
                Ok(emb) => emb,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("CLIP encoding failed: {}", e)
                    ));
                    return;
                }
            };
            
            // === 7. LOAD FLUX TRANSFORMER ===
            let vb_flux = match unsafe {
                VarBuilder::from_mmaped_safetensors(
                    std::slice::from_ref(&flux_path), 
                    dtype, 
                    &device
                )
            } {
                Ok(vb) => vb,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("FLUX VarBuilder creation failed: {}", e)
                    ));
                    return;
                }
            };
            
            let flux_transformer = match flux::model::Flux::new(
                &flux::model::Config::schnell(),
                vb_flux,
            ) {
                Ok(model) => model,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("FLUX model creation failed: {}", e)
                    ));
                    return;
                }
            };
            
            // === 8. LOAD VAE ===
            let vb_vae = match unsafe {
                VarBuilder::from_mmaped_safetensors(
                    std::slice::from_ref(&vae_path), 
                    dtype, 
                    &device
                )
            } {
                Ok(vb) => vb,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("VAE VarBuilder creation failed: {}", e)
                    ));
                    return;
                }
            };
            
            let vae = match flux::autoencoder::AutoEncoder::new(
                &flux::autoencoder::Config::schnell(),
                vb_vae,
            ) {
                Ok(model) => model,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("VAE creation failed: {}", e)
                    ));
                    return;
                }
            };
            
            // === 9. RUN GENERATION ===
            // Note: FLUX schnell uses guidance_scale = 0.0 (no CFG)
            // We use the value from config but it should be 0.0 for schnell
            let generation_config = ImageGenerationConfig {
                width,
                height,
                steps,
                guidance_scale,  // For schnell, this should be 0.0
                negative_prompt: config.negative_prompt,
                seed: config.seed,
                use_flash_attn: config.use_flash_attn,
            };
            
            let image = match generate_flux_image(
                &flux_transformer,
                &vae,
                &t5_emb,
                &clip_emb,
                &generation_config,
                &device,
                &sender,
            ) {
                Ok(result) => result,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("Image generation failed: {}", e)
                    ));
                    return;
                }
            };
            
            let _ = sender.send(ImageGenerationChunk::Complete { image });
        })
    }
    
    fn registry_key(&self) -> &str {
        "flux.1-schnell"
    }
    
    fn default_steps(&self) -> usize {
        4
    }
}

/// Generate image using 4-step FLUX denoising
fn generate_flux_image(
    flux_transformer: &flux::model::Flux,
    vae: &flux::autoencoder::AutoEncoder,
    t5_emb: &Tensor,
    clip_emb: &Tensor,
    config: &ImageGenerationConfig,
    device: &Device,
    sender: &ystream::AsyncStreamSender<ImageGenerationChunk>,
) -> Result<Tensor, String> {
    // 1. Initialize noise
    let img = flux::sampling::get_noise(1, config.height, config.width, device)
        .map_err(|e| format!("Noise generation failed: {}", e))?
        .to_dtype(t5_emb.dtype())
        .map_err(|e| format!("Noise dtype conversion failed: {}", e))?;
    
    // 2. Prepare State (packs embeddings + image for diffusion)
    let state = flux::sampling::State::new(t5_emb, clip_emb, &img)
        .map_err(|e| format!("State preparation failed: {}", e))?;
    
    // 3. Get timestep schedule (4 steps for schnell, no shift)
    let timesteps = flux::sampling::get_schedule(4, None);
    
    // 4. Track progress through denoising steps
    let total_steps = timesteps.len().saturating_sub(1);
    for (step, _window) in timesteps.windows(2).enumerate() {
        let _ = sender.send(ImageGenerationChunk::Step {
            step,
            total: total_steps,
            latent: state.img.clone(),
        });
    }
    
    // 5. Run full denoise (guidance = 0.0 for schnell)
    let guidance = 0.0;
    let denoised = flux::sampling::denoise(
        flux_transformer,
        &state.img,
        &state.img_ids,
        &state.txt,
        &state.txt_ids,
        &state.vec,
        &timesteps,
        guidance,
    ).map_err(|e| format!("Denoising failed: {}", e))?;
    
    // 6. Unpack latent patches back to spatial dimensions
    let unpacked = flux::sampling::unpack(&denoised, config.height, config.width)
        .map_err(|e| format!("Unpack failed: {}", e))?;
    
    // 7. VAE decode to pixel space
    let decoded = vae.decode(&unpacked)
        .map_err(|e| format!("VAE decode failed: {}", e))?;
    
    // 8. Post-process: scale from [-1, 1] to [0, 1]
    let image = ((decoded.clamp(-1f32, 1f32)
        .map_err(|e| format!("Clamp failed: {}", e))? + 1.0)
        .map_err(|e| format!("Add failed: {}", e))? * 0.5)
        .map_err(|e| format!("Scale failed: {}", e))?;
    
    Ok(image)
}

/// T5-XXL encoder with tokenizer
impl T5WithTokenizer {
    fn load(
        model_file: &PathBuf,
        config_file: &PathBuf,
        tokenizer_file: &PathBuf,  // NEW: Accept as parameter instead of downloading
        dtype: DType,
        device: &Device,
    ) -> Result<Self, String> {
        // Load T5 config
        let config_str = std::fs::read_to_string(config_file)
            .map_err(|e| format!("T5 config read failed: {}", e))?;
        let config: T5Config = serde_json::from_str(&config_str)
            .map_err(|e| format!("T5 config parse failed: {}", e))?;
        
        // Load T5 model
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[model_file], dtype, device)
                .map_err(|e| format!("T5 VarBuilder failed: {}", e))?
        };
        let t5 = T5EncoderModel::load(vb, &config)
            .map_err(|e| format!("T5 encoder load failed: {}", e))?;
        
        // Load T5 tokenizer from provided path
        let tokenizer = Tokenizer::from_file(tokenizer_file)
            .map_err(|e| format!("T5 tokenizer load failed: {}", e))?;
        
        Ok(Self { t5, tokenizer })
    }
    
    fn encode(&mut self, text: &str, device: &Device) -> Result<Tensor, String> {
        // Tokenize and resize to exactly 256 tokens (FLUX requirement)
        let mut tokens = self.tokenizer
            .encode(text, true)
            .map_err(|e| format!("T5 tokenization failed: {}", e))?
            .get_ids()
            .to_vec();
        
        tokens.resize(256, 0);
        
        let tokens_tensor = Tensor::new(&tokens[..], device)
            .map_err(|e| format!("T5 token tensor creation failed: {}", e))?
            .unsqueeze(0)
            .map_err(|e| format!("T5 unsqueeze failed: {}", e))?;
        
        self.t5.forward(&tokens_tensor)
            .map_err(|e| format!("T5 forward failed: {}", e))
    }
}

/// CLIP encoder with tokenizer
impl ClipWithTokenizer {
    fn load(
        model_file: &PathBuf,
        tokenizer_file: &PathBuf,  // NEW: Accept as parameter instead of downloading
        dtype: DType,
        device: &Device,
    ) -> Result<Self, String> {
        // Load CLIP model
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[model_file], dtype, device)
                .map_err(|e| format!("CLIP VarBuilder failed: {}", e))?
        };
        
        let clip_config = ClipConfig::sdxl();
        
        let clip = ClipTextTransformer::new(vb.pp("text_model"), &clip_config)
            .map_err(|e| format!("CLIP encoder creation failed: {}", e))?;
        
        // Load CLIP tokenizer from provided path
        let tokenizer = Tokenizer::from_file(tokenizer_file)
            .map_err(|e| format!("CLIP tokenizer load failed: {}", e))?;
        
        Ok(Self { clip, tokenizer })
    }
    
    fn encode(&self, text: &str, device: &Device) -> Result<Tensor, String> {
        // Tokenize with CLIP tokenizer (natural max 77 tokens)
        let tokens = self.tokenizer
            .encode(text, true)
            .map_err(|e| format!("CLIP tokenization failed: {}", e))?
            .get_ids()
            .to_vec();
        
        let tokens_tensor = Tensor::new(&tokens[..], device)
            .map_err(|e| format!("CLIP token tensor creation failed: {}", e))?
            .unsqueeze(0)
            .map_err(|e| format!("CLIP unsqueeze failed: {}", e))?;
        
        self.clip.forward(&tokens_tensor)
            .map_err(|e| format!("CLIP forward failed: {}", e))
    }
}

// Static model info for FLUX Schnell
static FLUX_SCHNELL_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::BlackForestLabs,
    name: "FLUX.1-schnell",
    registry_key: "black-forest-labs/FLUX.1-schnell",
    quantization_url: None,
    max_input_tokens: None,
    max_output_tokens: None,
    input_price: None,
    output_price: None,
    supports_vision: false,
    supports_function_calling: false,
    supports_streaming: true,
    supports_embeddings: false,
    requires_max_tokens: false,
    supports_thinking: false,
    optimal_thinking_budget: None,
    system_prompt_prefix: None,
    real_name: None,
    model_type: None,
    model_id: "flux-schnell",
    quantization: "bf16",
    patch: None,
    embedding_dimension: None,
    vocab_size: None,
    image_size: None,
    image_mean: None,
    image_std: None,
    default_temperature: None,
    default_top_k: None,
    default_top_p: None,
    supports_kv_cache: false,
    supports_flash_attention: false,
    use_bf16: true,
    default_steps: Some(4),
    default_guidance_scale: Some(0.0),
    time_shift: None,
    est_memory_allocation_mb: 0,
};

impl CandleModel for FluxSchnell {
    fn info(&self) -> &'static CandleModelInfo {
        &FLUX_SCHNELL_MODEL_INFO
    }
}

impl crate::capability::traits::TextToImageCapable for FluxSchnell {
    fn generate_image(
        &self,
        prompt: &str,
        config: &crate::domain::image_generation::ImageGenerationConfig,
        device: &candle_core::Device,
    ) -> AsyncStream<crate::domain::image_generation::ImageGenerationChunk> {
        // Delegate to ImageGenerationModel trait
        <Self as ImageGenerationModel>::generate(self, prompt, config, device)
    }
    
    fn registry_key(&self) -> &str {
        <Self as ImageGenerationModel>::registry_key(self)
    }
    
    fn default_steps(&self) -> usize {
        <Self as ImageGenerationModel>::default_steps(self)
    }
}
