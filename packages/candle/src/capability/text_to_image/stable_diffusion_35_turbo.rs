//! Stable Diffusion 3.5 Large Turbo provider
//!
//! Text-to-image generation using SD3.5's MMDiT diffusion model with triple CLIP encoding
//! (CLIP-L + CLIP-G + T5-XXL) for 4-step turbo inference.

use std::path::PathBuf;
use candle_core::{Device, DType, Tensor, D, IndexOp};
use candle_nn::{VarBuilder, Module};
use candle_transformers::models::{
    mmdit::model::{Config as MMDiTConfig, MMDiT},
    stable_diffusion::{
        vae::AutoEncoderKL,
        clip::{ClipTextTransformer, Config as ClipConfig},
    },
    t5::{T5EncoderModel, Config as T5Config},
    flux,
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

/// Stable Diffusion 3.5 Large Turbo provider
/// Uses lazy loading via huggingface_file() - no stored paths
#[derive(Clone, Debug)]
pub struct StableDiffusion35Turbo { }

/// Triple CLIP encoder wrapper
struct TripleClipEncoder {
    clip_l: ClipWithTokenizer,
    clip_g: ClipWithTokenizer,
    clip_g_projection: candle_nn::Linear,
    t5: T5WithTokenizer,
}

/// CLIP encoder with tokenizer
struct ClipWithTokenizer {
    clip: ClipTextTransformer,
    config: ClipConfig,
    tokenizer: Tokenizer,
    max_tokens: usize,
}

/// T5 encoder with tokenizer
struct T5WithTokenizer {
    t5: T5EncoderModel,
    tokenizer: Tokenizer,
    max_tokens: usize,
}

impl StableDiffusion35Turbo {
    pub fn new() -> Self {
        Self { }
    }
}

impl Default for StableDiffusion35Turbo {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageGenerationModel for StableDiffusion35Turbo {
    fn generate(
        &self,
        prompt: &str,
        config: &ImageGenerationConfig,
        device: &Device,
    ) -> AsyncStream<ImageGenerationChunk> {
        let prompt = prompt.to_string();
        let config = config.clone();
        let device = device.clone();
        let model_self = self.clone();
        
        AsyncStream::with_channel(move |sender| {
            // Get model-specific config from ModelInfo
            let time_shift = match model_self.info().time_shift {
                Some(ts) => ts,
                None => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        "time_shift not configured in ModelInfo".to_string()
                    ));
                    return;
                }
            };
            let use_flash_attn = model_self.info().supports_flash_attention;
            
            // Lazy load model files
            let clip_g_path = match model_self.huggingface_file("text_encoders/clip_g.safetensors") {
                Ok(p) => p,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("CLIP-G download failed: {}", e)
                    ));
                    return;
                }
            };
            
            let clip_l_path = match model_self.huggingface_file("text_encoders/clip_l.safetensors") {
                Ok(p) => p,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("CLIP-L download failed: {}", e)
                    ));
                    return;
                }
            };
            
            let t5xxl_path = match model_self.huggingface_file("text_encoders/t5xxl_fp16.safetensors") {
                Ok(p) => p,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("T5-XXL download failed: {}", e)
                    ));
                    return;
                }
            };
            
            let mmdit_path = match model_self.huggingface_file("sd3.5_large_turbo.safetensors") {
                Ok(p) => p,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("MMDiT download failed: {}", e)
                    ));
                    return;
                }
            };
            
            // Download tokenizers and configs using HF Hub API
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
            
            // CLIP-L tokenizer
            let clip_l_repo = api.model("openai/clip-vit-large-patch14".to_string());
            let clip_l_tokenizer_path = match clip_l_repo.get("tokenizer.json") {
                Ok(p) => p,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("Failed to download CLIP-L tokenizer: {}", e)
                    ));
                    return;
                }
            };
            
            // CLIP-G tokenizer
            let clip_g_repo = api.model("laion/CLIP-ViT-bigG-14-laion2B-39B-b160k".to_string());
            let clip_g_tokenizer_path = match clip_g_repo.get("tokenizer.json") {
                Ok(p) => p,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("Failed to download CLIP-G tokenizer: {}", e)
                    ));
                    return;
                }
            };
            
            // T5 config and tokenizer
            let t5_repo = api.model("google/t5-v1_1-xxl".to_string());
            let t5_config_path = match t5_repo.get("config.json") {
                Ok(p) => p,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("Failed to download T5 config: {}", e)
                    ));
                    return;
                }
            };
            
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
            
            // Set random seed
            if let Some(seed) = config.seed
                && let Err(e) = device.set_seed(seed)
            {
                let _ = sender.send(ImageGenerationChunk::Error(
                    format!("Seed setting failed: {e}")
                ));
                return;
            }
            
            // Load triple CLIP encoders
            let mut triple_clip = match TripleClipEncoder::load(
                &clip_g_path,
                &clip_l_path,
                &t5xxl_path,
                &clip_l_tokenizer_path,
                &clip_g_tokenizer_path,
                &t5_config_path,
                &t5_tokenizer_path,
                &device,
            ) {
                Ok(encoder) => encoder,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("Triple CLIP load failed: {}", e)
                    ));
                    return;
                }
            };
            
            // Encode prompts
            let (context, y) = match triple_clip.encode_prompt(&prompt, &device) {
                Ok(result) => result,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("Text encoding failed: {}", e)
                    ));
                    return;
                }
            };
            
            // Handle negative prompt
            let (context, y) = if let Some(neg_prompt) = &config.negative_prompt {
                match triple_clip.encode_prompt(neg_prompt, &device) {
                    Ok((ctx_uncond, y_uncond)) => {
                        match (
                            Tensor::cat(&[&context, &ctx_uncond], 0),
                            Tensor::cat(&[&y, &y_uncond], 0),
                        ) {
                            (Ok(ctx), Ok(y_cat)) => (ctx, y_cat),
                            (Err(e), _) | (_, Err(e)) => {
                                let _ = sender.send(ImageGenerationChunk::Error(
                                    format!("Context concatenation failed: {}", e)
                                ));
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        let _ = sender.send(ImageGenerationChunk::Error(
                            format!("Negative prompt encoding failed: {}", e)
                        ));
                        return;
                    }
                }
            } else {
                match (
                    Tensor::cat(&[&context, &context], 0),
                    Tensor::cat(&[&y, &y], 0),
                ) {
                    (Ok(ctx), Ok(y_cat)) => (ctx, y_cat),
                    (Err(e), _) | (_, Err(e)) => {
                        let _ = sender.send(ImageGenerationChunk::Error(
                            format!("Context duplication failed: {}", e)
                        ));
                        return;
                    }
                }
            };
            
            // Load MMDiT model
            let vb = match unsafe {
                VarBuilder::from_mmaped_safetensors(&[&mmdit_path], DType::F16, &device)
            } {
                Ok(vb) => vb,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("VarBuilder creation failed: {}", e)
                    ));
                    return;
                }
            };
            
            let mmdit = match MMDiT::new(
                &MMDiTConfig::sd3_5_large(),
                use_flash_attn,
                vb.pp("model.diffusion_model"),
            ) {
                Ok(model) => model,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("MMDiT creation failed: {}", e)
                    ));
                    return;
                }
            };
            
            // Load VAE
            let vae_config = candle_transformers::models::stable_diffusion::vae::AutoEncoderKLConfig {
                block_out_channels: vec![128, 256, 512, 512],
                layers_per_block: 2,
                latent_channels: 16,
                norm_num_groups: 32,
                use_quant_conv: false,
                use_post_quant_conv: false,
            };
            
            let vae = match AutoEncoderKL::new(vb.pp("first_stage_model"), 3, 3, vae_config) {
                Ok(model) => model,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("VAE creation failed: {}", e)
                    ));
                    return;
                }
            };
            
            // Euler sampling
            let latent = match euler_sample(
                &mmdit,
                &y,
                &context,
                &config,
                time_shift,
                &device,
                &sender,
            ) {
                Ok(result) => result,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("Sampling failed: {}", e)
                    ));
                    return;
                }
            };
            
            // VAE decode
            let image = match decode_latent(&vae, &latent) {
                Ok(result) => result,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("VAE decode failed: {}", e)
                    ));
                    return;
                }
            };
            
            let _ = sender.send(ImageGenerationChunk::Complete { image });
        })
    }
    
    fn model_name(&self) -> &str {
        "stable-diffusion-3.5-large-turbo"
    }
    
    fn default_steps(&self) -> usize {
        4
    }
}

/// Euler sampling with CFG
fn euler_sample(
    mmdit: &MMDiT,
    y: &Tensor,
    context: &Tensor,
    config: &ImageGenerationConfig,
    time_shift: f64,
    device: &Device,
    sender: &ystream::AsyncStreamSender<ImageGenerationChunk>,
) -> Result<Tensor, String> {
    let mut x = flux::sampling::get_noise(1, config.height, config.width, device)
        .map_err(|e| format!("Noise generation failed: {}", e))?
        .to_dtype(DType::F16)
        .map_err(|e| format!("Dtype conversion failed: {}", e))?;
    
    let sigmas: Vec<f64> = (0..=config.steps)
        .map(|i| i as f64 / config.steps as f64)
        .rev()
        .map(|t| time_snr_shift(time_shift, t))
        .collect();
    
    for (step, window) in sigmas.windows(2).enumerate() {
        let (s_curr, s_prev) = (window[0], window[1]);
        let timestep = s_curr * 1000.0;
        
        let noise_pred = mmdit.forward(
            &Tensor::cat(&[&x, &x], 0)
                .map_err(|e| format!("Latent concat failed at step {}: {}", step, e))?,
            &Tensor::full(timestep as f32, (2,), device)
                .map_err(|e| format!("Timestep tensor failed: {}", e))?
                .contiguous()
                .map_err(|e| format!("Contiguous failed: {}", e))?,
            y,
            context,
            None,
        ).map_err(|e| format!("MMDiT forward failed at step {}: {}", step, e))?;
        
        let guidance = apply_cfg(config.guidance_scale, &noise_pred)?;
        
        let step_delta = (guidance * (s_prev - s_curr))
            .map_err(|e| format!("Guidance scaling failed: {}", e))?;
        x = (x + step_delta)
            .map_err(|e| format!("Latent update failed at step {}: {}", step, e))?;
        
        let _ = sender.send(ImageGenerationChunk::Step {
            step,
            total: config.steps,
            latent: x.clone(),
        });
    }
    
    Ok(x)
}

fn time_snr_shift(alpha: f64, t: f64) -> f64 {
    alpha * t / (1.0 + (alpha - 1.0) * t)
}

fn apply_cfg(scale: f64, noise_pred: &Tensor) -> Result<Tensor, String> {
    let cond = noise_pred.narrow(0, 0, 1)
        .map_err(|e| format!("Cond narrow failed: {}", e))?;
    let uncond = noise_pred.narrow(0, 1, 1)
        .map_err(|e| format!("Uncond narrow failed: {}", e))?;
    
    ((scale * cond).map_err(|e| format!("Cond scale failed: {}", e))?
        - ((scale - 1.0) * uncond).map_err(|e| format!("Uncond scale failed: {}", e))?)
        .map_err(|e| format!("CFG subtraction failed: {}", e))
}

fn decode_latent(vae: &AutoEncoderKL, latent: &Tensor) -> Result<Tensor, String> {
    let latent_scaled = ((latent / 1.5305)
        .map_err(|e| format!("Latent division failed: {}", e))? + 0.0609)
        .map_err(|e| format!("Latent offset failed: {}", e))?;
    
    let img = vae.decode(&latent_scaled)
        .map_err(|e| format!("VAE decode failed: {}", e))?;
    
    let img = ((img.clamp(-1f32, 1f32)
        .map_err(|e| format!("Clamp failed: {}", e))? + 1.0)
        .map_err(|e| format!("Add failed: {}", e))? * 0.5)
        .map_err(|e| format!("Scale failed: {}", e))?;
    
    Ok(img)
}

impl TripleClipEncoder {
    fn load(
        clip_g_file: &PathBuf,
        clip_l_file: &PathBuf,
        t5xxl_file: &PathBuf,
        clip_l_tokenizer_path: &PathBuf,
        clip_g_tokenizer_path: &PathBuf,
        t5_config_path: &PathBuf,
        t5_tokenizer_path: &PathBuf,
        device: &Device,
    ) -> Result<Self, String> {
        let vb_clip_l = unsafe {
            VarBuilder::from_mmaped_safetensors(&[clip_l_file], DType::F16, device)
                .map_err(|e| format!("CLIP-L VarBuilder failed: {}", e))?
        };
        let clip_l = ClipWithTokenizer::new(
            vb_clip_l,
            ClipConfig::sdxl(),
            clip_l_tokenizer_path,
            77,
        )?;
        
        let vb_clip_g = unsafe {
            VarBuilder::from_mmaped_safetensors(&[clip_g_file], DType::F16, device)
                .map_err(|e| format!("CLIP-G VarBuilder failed: {}", e))?
        };
        let clip_g = ClipWithTokenizer::new(
            vb_clip_g.clone(),
            ClipConfig::sdxl2(),
            clip_g_tokenizer_path,
            77,
        )?;
        
        let clip_g_projection = candle_nn::linear_no_bias(
            1280, 
            1280, 
            vb_clip_g.pp("text_projection")
        ).map_err(|e| format!("Text projection creation failed: {}", e))?;
        
        let vb_t5 = unsafe {
            VarBuilder::from_mmaped_safetensors(&[t5xxl_file], DType::F16, device)
                .map_err(|e| format!("T5 VarBuilder failed: {}", e))?
        };
        let t5 = T5WithTokenizer::new(vb_t5, t5_config_path, t5_tokenizer_path, 77)?;
        
        Ok(Self {
            clip_l,
            clip_g,
            clip_g_projection,
            t5,
        })
    }
    
    fn encode_prompt(
        &mut self,
        prompt: &str,
        device: &Device,
    ) -> Result<(Tensor, Tensor), String> {
        let (clip_l_emb, clip_l_pooled) = self.clip_l.encode(prompt, device)?;
        let (clip_g_emb, clip_g_pooled) = self.clip_g.encode(prompt, device)?;
        
        let clip_g_pooled_proj = self.clip_g_projection
            .forward(&clip_g_pooled.unsqueeze(0)
                .map_err(|e| format!("Unsqueeze failed: {}", e))?)
            .map_err(|e| format!("Projection forward failed: {}", e))?
            .squeeze(0)
            .map_err(|e| format!("Squeeze failed: {}", e))?;
        
        let y = Tensor::cat(&[&clip_l_pooled, &clip_g_pooled_proj], 0)
            .map_err(|e| format!("Y concatenation failed: {}", e))?
            .unsqueeze(0)
            .map_err(|e| format!("Y unsqueeze failed: {}", e))?;
        
        let clip_concat = Tensor::cat(&[&clip_l_emb, &clip_g_emb], D::Minus1)
            .map_err(|e| format!("CLIP concatenation failed: {}", e))?
            .pad_with_zeros(D::Minus1, 0, 2048)
            .map_err(|e| format!("CLIP padding failed: {}", e))?;
        
        let t5_emb = self.t5.encode(prompt, device)?
            .to_dtype(DType::F16)
            .map_err(|e| format!("T5 dtype conversion failed: {}", e))?;
        
        let context = Tensor::cat(&[&clip_concat, &t5_emb], D::Minus2)
            .map_err(|e| format!("Context concatenation failed: {}", e))?;
        
        Ok((context, y))
    }
}

impl ClipWithTokenizer {
    fn new(
        vb: VarBuilder,
        config: ClipConfig,
        tokenizer_path: &PathBuf,
        max_tokens: usize,
    ) -> Result<Self, String> {
        let clip = ClipTextTransformer::new(vb, &config)
            .map_err(|e| format!("CLIP creation failed: {}", e))?;
        
        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| format!("Tokenizer load failed: {}", e))?;
        
        Ok(Self { clip, tokenizer, max_tokens, config })
    }
    
    fn encode(&self, text: &str, device: &Device) -> Result<(Tensor, Tensor), String> {
        let pad_id = match &self.config.pad_with {
            Some(padding) => *self.tokenizer
                .get_vocab(true)
                .get(padding.as_str())
                .ok_or_else(|| "Failed to tokenize CLIP padding".to_string())?,
            None => *self.tokenizer
                .get_vocab(true)
                .get("<|endoftext|>")
                .ok_or_else(|| "Failed to tokenize CLIP end-of-text".to_string())?,
        };
        
        let mut tokens = self.tokenizer
            .encode(text, true)
            .map_err(|e| format!("Tokenization failed: {}", e))?
            .get_ids()
            .to_vec();
        
        let eos_pos = tokens.len() - 1;
        
        while tokens.len() < self.max_tokens {
            tokens.push(pad_id);
        }
        
        let tokens_tensor = Tensor::new(&tokens[..], device)
            .map_err(|e| format!("Token tensor creation failed: {}", e))?
            .unsqueeze(0)
            .map_err(|e| format!("Unsqueeze failed: {}", e))?;
        
        let (emb, emb_penultimate) = self.clip
            .forward_until_encoder_layer(&tokens_tensor, usize::MAX, -2)
            .map_err(|e| format!("CLIP forward failed: {}", e))?;
        
        let pooled = emb.i((0, eos_pos, ..))
            .map_err(|e| format!("Pooled extraction failed: {}", e))?;
        
        Ok((emb_penultimate, pooled))
    }
}

impl T5WithTokenizer {
    fn new(
        vb: VarBuilder,
        config_path: &PathBuf,
        tokenizer_path: &PathBuf,
        max_tokens: usize,
    ) -> Result<Self, String> {
        let config_str = std::fs::read_to_string(config_path)
            .map_err(|e| format!("T5 config read failed: {}", e))?;
        let config: T5Config = serde_json::from_str(&config_str)
            .map_err(|e| format!("T5 config parse failed: {}", e))?;
        
        let t5 = T5EncoderModel::load(vb, &config)
            .map_err(|e| format!("T5 model load failed: {}", e))?;
        
        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| format!("T5 tokenizer load failed: {}", e))?;
        
        Ok(Self { t5, tokenizer, max_tokens })
    }
    
    fn encode(&mut self, text: &str, device: &Device) -> Result<Tensor, String> {
        let mut tokens = self.tokenizer
            .encode(text, true)
            .map_err(|e| format!("T5 tokenization failed: {}", e))?
            .get_ids()
            .to_vec();
        
        tokens.resize(self.max_tokens, 0);
        
        let tokens_tensor = Tensor::new(&tokens[..], device)
            .map_err(|e| format!("T5 token tensor failed: {}", e))?
            .unsqueeze(0)
            .map_err(|e| format!("T5 unsqueeze failed: {}", e))?;
        
        self.t5.forward_dt(&tokens_tensor, Some(DType::F32))
            .map_err(|e| format!("T5 forward failed: {}", e))
    }
}

// Static model info for Stable Diffusion 3.5 Turbo
static SD35_TURBO_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::StabilityAI,
    name: "stable-diffusion-3.5-large-turbo",
    registry_key: "stabilityai/stable-diffusion-3.5-large-turbo",
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
    model_id: "sd35-turbo",
    quantization: "fp16",
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
    supports_flash_attention: true,
    use_bf16: false,
    default_steps: Some(4),
    default_guidance_scale: Some(3.5),
    time_shift: Some(3.0),
};

impl CandleModel for StableDiffusion35Turbo {
    fn info(&self) -> &'static CandleModelInfo {
        &SD35_TURBO_MODEL_INFO
    }
}

impl crate::capability::traits::TextToImageCapable for StableDiffusion35Turbo {
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
