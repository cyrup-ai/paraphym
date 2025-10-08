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
/// Stores model paths, loads models on-demand in generate()
#[derive(Clone, Debug)]
pub struct FluxSchnell {
    flux_file: PathBuf,
    t5_model_file: PathBuf,
    t5_config_file: PathBuf,
    clip_file: PathBuf,
    vae_file: PathBuf,
    config: FluxConfig,
}

/// FLUX schnell configuration
#[derive(Debug, Clone)]
pub struct FluxConfig {
    /// CFG scale (FLUX schnell uses 0.0 - no guidance)
    pub guidance_scale: f64,
    /// Use BF16 precision when available
    pub use_bf16: bool,
}

impl Default for FluxConfig {
    fn default() -> Self {
        Self {
            guidance_scale: 0.0,  // FLUX schnell doesn't use CFG
            use_bf16: true,
        }
    }
}

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
        Self::from_pretrained()
            .unwrap_or_else(|e| panic!("Failed to initialize FluxSchnell: {}", e))
    }
}

impl FluxSchnell {
    /// Load FLUX schnell model paths from HuggingFace Hub
    pub fn from_pretrained() -> Result<Self, String> {
        let api = hf_hub::api::sync::Api::new()
            .map_err(|e| format!("HF API init failed: {}", e))?;
        
        // Load FLUX transformer
        let flux_repo = api.repo(hf_hub::Repo::model(
            "black-forest-labs/FLUX.1-schnell".to_string()
        ));
        let flux_file = flux_repo.get("flux1-schnell.safetensors")
            .map_err(|e| format!("FLUX model download failed: {}", e))?;
        
        // Load T5-XXL text encoder (with PR #2 revision for compatibility)
        let t5_repo = api.repo(hf_hub::Repo::with_revision(
            "google/t5-v1_1-xxl".to_string(),
            hf_hub::RepoType::Model,
            "refs/pr/2".to_string(),
        ));
        let t5_model_file = t5_repo.get("model.safetensors")
            .map_err(|e| format!("T5 model download failed: {}", e))?;
        let t5_config_file = t5_repo.get("config.json")
            .map_err(|e| format!("T5 config download failed: {}", e))?;
        
        // Load CLIP text encoder
        let clip_repo = api.repo(hf_hub::Repo::model(
            "openai/clip-vit-large-patch14".to_string()
        ));
        let clip_file = clip_repo.get("model.safetensors")
            .map_err(|e| format!("CLIP model download failed: {}", e))?;
        
        // Load VAE
        let vae_file = flux_repo.get("ae.safetensors")
            .map_err(|e| format!("VAE download failed: {}", e))?;
        
        Ok(Self {
            flux_file,
            t5_model_file,
            t5_config_file,
            clip_file,
            vae_file,
            config: FluxConfig::default(),
        })
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
        let flux_file = self.flux_file.clone();
        let t5_model_file = self.t5_model_file.clone();
        let t5_config_file = self.t5_config_file.clone();
        let clip_file = self.clip_file.clone();
        let vae_file = self.vae_file.clone();
        let provider_config = self.config.clone();
        
        AsyncStream::with_channel(move |sender| {
            // Set random seed
            if let Some(seed) = config.seed
                && let Err(e) = device.set_seed(seed)
            {
                let _ = sender.send(ImageGenerationChunk::Error(
                    format!("Seed setting failed: {e}")
                ));
                return;
            }
            
            // Determine dtype (prefer BF16 if available)
            let dtype = if provider_config.use_bf16 {
                device.bf16_default_to_f32()
            } else {
                DType::F32
            };
            
            // Load T5 encoder
            let mut t5_encoder = match T5WithTokenizer::load(
                &t5_model_file,
                &t5_config_file,
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
            
            // Load CLIP encoder
            let clip_encoder = match ClipWithTokenizer::load(
                &clip_file,
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
            
            // Encode text prompt (T5 + CLIP)
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
            
            // Load FLUX transformer
            let vb_flux = match unsafe {
                VarBuilder::from_mmaped_safetensors(std::slice::from_ref(&flux_file), dtype, &device)
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
            
            // Load VAE
            let vb_vae = match unsafe {
                VarBuilder::from_mmaped_safetensors(std::slice::from_ref(&vae_file), dtype, &device)
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
            
            // Run 4-step denoising generation
            let image = match generate_flux_image(
                &flux_transformer,
                &vae,
                &t5_emb,
                &clip_emb,
                &config,
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
    
    fn model_name(&self) -> &str {
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
        
        // Load T5 tokenizer
        let api = hf_hub::api::sync::Api::new()
            .map_err(|e| format!("HF API failed: {}", e))?;
        let tokenizer_file = api.model("lmz/mt5-tokenizers".to_string())
            .get("t5-v1_1-xxl.tokenizer.json")
            .map_err(|e| format!("T5 tokenizer download failed: {}", e))?;
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
        
        // Load CLIP tokenizer
        let api = hf_hub::api::sync::Api::new()
            .map_err(|e| format!("HF API failed: {}", e))?;
        let tokenizer_file = api.model("openai/clip-vit-large-patch14".to_string())
            .get("tokenizer.json")
            .map_err(|e| format!("CLIP tokenizer download failed: {}", e))?;
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
    
    fn model_name(&self) -> &str {
        <Self as ImageGenerationModel>::model_name(self)
    }
    
    fn default_steps(&self) -> usize {
        <Self as ImageGenerationModel>::default_steps(self)
    }
}
