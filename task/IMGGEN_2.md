# IMGGEN_2: Stable Diffusion 3.5 Turbo Provider

## OBJECTIVE
Implement complete Stable Diffusion 3.5 Large Turbo provider for 4-step text-to-image generation following paraphym's provider architecture pattern.

## PRIORITY
🔴 CRITICAL - Primary image generation model

## SUBTASK 1: Create Provider Structure

**What needs to change**:
- Create new provider file following kimi_k2.rs pattern
- Define provider struct with model components

**Where changes happen**:
- NEW: `packages/candle/src/providers/stable_diffusion_35_turbo.rs`

**Implementation**:
```rust
use std::sync::Arc;
use candle_core::{Device, DType, Tensor};
use candle_transformers::models::mmdit::model::{Config as MMDiTConfig, MMDiT};
use ystream::AsyncStream;
use crate::domain::image_generation::{ImageGenerationModel, ImageGenerationConfig, ImageGenerationChunk};

pub struct StableDiffusion35Turbo {
    mmdit: MMDiT,                          // Main diffusion model
    vae: AutoEncoderKL,                    // VAE for latent decoding
    text_encoder_clip_g: ClipTextTransformer,
    text_encoder_clip_l: ClipTextTransformer,  
    text_encoder_t5: T5EncoderModel,       // Triple text encoders
    config: SD35TurboConfig,
}

#[derive(Debug, Clone)]
pub struct SD35TurboConfig {
    pub steps: usize,              // Default: 4 (turbo)
    pub guidance_scale: f64,       // Default: 3.5 (lower for turbo)
    pub time_shift: f64,           // Default: 3.0 (SNR shift)
    pub use_flash_attn: bool,      // Memory optimization
}

impl Default for SD35TurboConfig {
    fn default() -> Self {
        Self {
            steps: 4,
            guidance_scale: 3.5,
            time_shift: 3.0,
            use_flash_attn: false,
        }
    }
}
```

## SUBTASK 2: Implement Model Loading from HuggingFace

**Reference**: `./tmp/candle-examples/candle-examples/examples/stable-diffusion-3/main.rs:113-166`

**Implementation**:
```rust
impl StableDiffusion35Turbo {
    pub fn from_pretrained(device: &Device) -> Result<Self, String> {
        // 1. Initialize HuggingFace API
        let api = hf_hub::api::sync::Api::new()
            .map_err(|e| format!("HF API init failed: {}", e))?;
        
        // 2. Get repository for text encoders
        let text_repo = api.repo(hf_hub::Repo::model(
            "stabilityai/stable-diffusion-3.5-large-turbo".to_string()
        ));
        
        // 3. Download text encoder weights
        let clip_g_file = text_repo.get("text_encoders/clip_g.safetensors")
            .map_err(|e| format!("CLIP-G download failed: {}", e))?;
        let clip_l_file = text_repo.get("text_encoders/clip_l.safetensors")
            .map_err(|e| format!("CLIP-L download failed: {}", e))?;
        let t5xxl_file = text_repo.get("text_encoders/t5xxl_fp16.safetensors")
            .map_err(|e| format!("T5-XXL download failed: {}", e))?;
        
        // 4. Download main model
        let model_file = text_repo.get("sd3.5_large_turbo.safetensors")
            .map_err(|e| format!("MMDiT download failed: {}", e))?;
        
        // 5. Load text encoders
        let triple_clip = StableDiffusion3TripleClipWithTokenizer::new_split(
            &clip_g_file,
            &clip_l_file,
            &t5xxl_file,
            device,
        ).map_err(|e| format!("Text encoder load failed: {}", e))?;
        
        // 6. Load MMDiT model
        let vb = unsafe {
            candle_nn::VarBuilder::from_mmaped_safetensors(
                &[model_file], 
                DType::F16, 
                device
            ).map_err(|e| format!("VarBuilder creation failed: {}", e))?
        };
        
        let mmdit_config = MMDiTConfig::sd3_5_large();
        let mmdit = MMDiT::new(&mmdit_config, false, vb.pp("model.diffusion_model"))
            .map_err(|e| format!("MMDiT creation failed: {}", e))?;
        
        // 7. Load VAE
        let vae = build_sd3_vae_autoencoder(vb.pp("first_stage_model"))
            .map_err(|e| format!("VAE creation failed: {}", e))?;
        
        Ok(Self {
            mmdit,
            vae,
            text_encoder_clip_g: triple_clip.clip_g,
            text_encoder_clip_l: triple_clip.clip_l,
            text_encoder_t5: triple_clip.t5,
            config: SD35TurboConfig::default(),
        })
    }
}
```

## SUBTASK 3: Implement Text Encoding

**Reference**: `./tmp/candle-examples/candle-examples/examples/stable-diffusion-3/main.rs:175-193`

**Add method**:
```rust
impl StableDiffusion35Turbo {
    fn encode_prompts(
        &self,
        prompt: &str,
        negative_prompt: &str,
        device: &Device,
    ) -> Result<(Tensor, Tensor), String> {
        // 1. Encode positive prompt
        let (context, y) = self.triple_clip.encode_text_to_embedding(prompt, device)
            .map_err(|e| format!("Prompt encoding failed: {}", e))?;
        
        // 2. Encode negative prompt for CFG
        let (context_uncond, y_uncond) = self.triple_clip.encode_text_to_embedding(
            negative_prompt, 
            device
        ).map_err(|e| format!("Negative prompt encoding failed: {}", e))?;
        
        // 3. Concatenate for batch processing [cond, uncond]
        let context = Tensor::cat(&[context, context_uncond], 0)
            .map_err(|e| format!("Context concatenation failed: {}", e))?;
        let y = Tensor::cat(&[y, y_uncond], 0)
            .map_err(|e| format!("Y concatenation failed: {}", e))?;
        
        Ok((context, y))
    }
}
```

## SUBTASK 4: Implement Euler Sampling Loop

**Reference**: `./tmp/candle-examples/candle-examples/examples/stable-diffusion-3/sampling.rs:14-69`

**Add method**:
```rust
impl StableDiffusion35Turbo {
    fn euler_sample(
        &self,
        y: &Tensor,
        context: &Tensor,
        config: &ImageGenerationConfig,
        device: &Device,
        sender: &ystream::Sender<ImageGenerationChunk>,
    ) -> Result<Tensor, String> {
        // 1. Initialize noise
        let mut x = flux::sampling::get_noise(1, config.height, config.width, device)
            .map_err(|e| format!("Noise generation failed: {}", e))?
            .to_dtype(DType::F16)
            .map_err(|e| format!("Dtype conversion failed: {}", e))?;
        
        // 2. Calculate timesteps with SNR shift
        let sigmas: Vec<f64> = (0..=config.steps)
            .map(|i| i as f64 / config.steps as f64)
            .rev()
            .map(|t| self.time_snr_shift(self.config.time_shift, t))
            .collect();
        
        // 3. Denoising loop
        for (step, window) in sigmas.windows(2).enumerate() {
            let (s_curr, s_prev) = (window[0], window[1]);
            let timestep = s_curr * 1000.0;
            
            // Forward pass through MMDiT
            let noise_pred = self.mmdit.forward(
                &Tensor::cat(&[&x, &x], 0)
                    .map_err(|e| format!("Latent concat failed: {}", e))?,
                &Tensor::full(timestep as f32, (2,), device)
                    .map_err(|e| format!("Timestep tensor failed: {}", e))?
                    .contiguous()
                    .map_err(|e| format!("Contiguous failed: {}", e))?,
                y,
                context,
                None,
            ).map_err(|e| format!("MMDiT forward failed at step {}: {}", step, e))?;
            
            // Apply Classifier-Free Guidance
            let guidance = self.apply_cfg(config.guidance_scale, &noise_pred)?;
            
            // Update latent: x = x + guidance * (s_prev - s_curr)
            x = (x + (guidance * (s_prev - s_curr))
                .map_err(|e| format!("Guidance scaling failed: {}", e))?)
                .map_err(|e| format!("Latent update failed: {}", e))?;
            
            // Stream progress
            let _ = sender.send(ImageGenerationChunk::Step {
                step,
                total: config.steps,
                latent: x.clone(),
            });
        }
        
        Ok(x)
    }
    
    fn time_snr_shift(&self, alpha: f64, t: f64) -> f64 {
        alpha * t / (1.0 + (alpha - 1.0) * t)
    }
    
    fn apply_cfg(&self, scale: f64, noise_pred: &Tensor) -> Result<Tensor, String> {
        // CFG: guidance = scale * cond - (scale - 1) * uncond
        let cond = noise_pred.narrow(0, 0, 1)
            .map_err(|e| format!("Cond narrow failed: {}", e))?;
        let uncond = noise_pred.narrow(0, 1, 1)
            .map_err(|e| format!("Uncond narrow failed: {}", e))?;
        
        ((scale * cond).map_err(|e| format!("Cond scale failed: {}", e))?
            - ((scale - 1.0) * uncond).map_err(|e| format!("Uncond scale failed: {}", e))?)
            .map_err(|e| format!("CFG subtraction failed: {}", e))
    }
}
```

## SUBTASK 5: Implement VAE Decoding

**Reference**: `./tmp/candle-examples/candle-examples/examples/stable-diffusion-3/main.rs:250-255`

**Add method**:
```rust
impl StableDiffusion35Turbo {
    fn decode_latent(&self, latent: &Tensor) -> Result<Tensor, String> {
        // 1. VAE decode
        let img = self.vae.decode(latent)
            .map_err(|e| format!("VAE decode failed: {}", e))?;
        
        // 2. Post-process: scale and clamp to [0, 1]
        let img = ((img / 2.0).map_err(|e| format!("Division failed: {}", e))? + 0.5)
            .map_err(|e| format!("Addition failed: {}", e))?
            .clamp(0.0, 1.0)
            .map_err(|e| format!("Clamp failed: {}", e))?;
        
        Ok(img)
    }
}
```

## SUBTASK 6: Implement ImageGenerationModel Trait

**Integration of all methods**:
```rust
impl ImageGenerationModel for StableDiffusion35Turbo {
    fn generate(
        &self,
        prompt: &str,
        config: &ImageGenerationConfig,
        device: &Device,
    ) -> AsyncStream<ImageGenerationChunk> {
        AsyncStream::with_channel(|sender| {
            // 1. Set random seed if specified
            if let Some(seed) = config.seed {
                if let Err(e) = device.set_seed(seed) {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("Seed setting failed: {}", e)
                    ));
                    return;
                }
            }
            
            // 2. Encode text prompts
            let (context, y) = match self.encode_prompts(
                prompt,
                config.negative_prompt.as_deref().unwrap_or(""),
                device,
            ) {
                Ok(result) => result,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(e));
                    return;
                }
            };
            
            // 3. Sampling loop
            let latent = match self.euler_sample(&y, &context, config, device, &sender) {
                Ok(result) => result,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(e));
                    return;
                }
            };
            
            // 4. VAE decode
            let image = match self.decode_latent(&latent) {
                Ok(result) => result,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(e));
                    return;
                }
            };
            
            // 5. Send final result
            let _ = sender.send(ImageGenerationChunk::Complete { image });
        })
    }
    
    fn model_name(&self) -> &str {
        "stable-diffusion-3.5-large-turbo"
    }
    
    fn default_steps(&self) -> usize {
        4  // Turbo variant uses 4 steps
    }
}
```

## REQUIRED IMPORTS

```rust
use std::sync::Arc;
use candle_core::{Device, DType, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::{
    mmdit::model::{Config as MMDiTConfig, MMDiT},
    stable_diffusion::{
        vae::AutoEncoderKL,
        clip::ClipTextTransformer,
    },
    t5::T5EncoderModel,
    flux,
};
use ystream::AsyncStream;
use crate::domain::image_generation::{
    ImageGenerationModel, 
    ImageGenerationConfig, 
    ImageGenerationChunk
};
```

## DEFINITION OF DONE

- [ ] File created: `packages/candle/src/providers/stable_diffusion_35_turbo.rs`
- [ ] StableDiffusion35Turbo struct with all model components
- [ ] SD35TurboConfig struct with Default impl
- [ ] from_pretrained() loads model from HuggingFace Hub
- [ ] encode_prompts() handles text encoding with triple CLIP
- [ ] euler_sample() implements 4-step denoising
- [ ] decode_latent() converts latent to image tensor
- [ ] ImageGenerationModel trait fully implemented
- [ ] All errors use Result and map_err() with context
- [ ] No unwrap() or expect() in code
- [ ] cargo check passes

## REFERENCES

**Primary Implementation Reference**:
- SD3 example: `./tmp/candle-examples/candle-examples/examples/stable-diffusion-3/main.rs`
- Sampling logic: `./tmp/candle-examples/candle-examples/examples/stable-diffusion-3/sampling.rs`
- VAE utilities: `./tmp/candle-examples/candle-examples/examples/stable-diffusion-3/vae.rs`
- CLIP encoding: `./tmp/candle-examples/candle-examples/examples/stable-diffusion-3/clip.rs`

**Pattern Reference**:
- Provider structure: `./packages/candle/src/providers/kimi_k2.rs`

## IMPORTANT CONSTRAINTS

- ❌ NO unit tests (separate team)
- ❌ NO benchmarks (separate team)
- ❌ NO examples in this file (next task)
- ✅ YES comprehensive error handling
- ✅ YES streaming progress chunks
- ✅ YES proper HuggingFace Hub integration

## VERIFICATION

```bash
# Build check
cargo check -p paraphym_candle

# Verify no unwrap/expect
grep -n "unwrap\|expect" packages/candle/src/providers/stable_diffusion_35_turbo.rs
# Should return nothing

# Check HF model exists
curl -I https://huggingface.co/stabilityai/stable-diffusion-3.5-large-turbo
```
