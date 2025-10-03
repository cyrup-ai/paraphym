# VISION_5: Stable Diffusion Generation Pipeline

## OBJECTIVE

Implement the complete Stable Diffusion generation pipeline including text-to-image generation with streaming progress and image-to-image transformation. This completes the vision AI implementation by enabling image generation from text descriptions.

---

## BACKGROUND

**Current State**:
- StableDiffusion35Provider exists with loaded models (from VISION_4)
- Text encoding pipeline functional
- ImageGenerationModel trait needs implementation

**What This Task Accomplishes**:
- Text-to-image generation with streaming
- Image-to-image transformation
- Progress monitoring during generation
- Integration with ImageGenerationModel trait

---

## SUBTASK 1: Implement ImageGenerationModel Trait

**File**: `packages/candle/src/providers/stable_diffusion.rs`

**What to Add**:
```rust
impl ImageGenerationModel for StableDiffusion35Provider {
    fn generate(
        &self,
        prompt: &str,
        config: &ImageGenerationConfig,
        device: &Device,
    ) -> AsyncStream<ImageGenerationChunk> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        
        let prompt = prompt.to_string();
        let config = config.clone();
        let device = device.clone();
        let provider = self.clone();
        
        tokio::spawn(async move {
            if let Err(e) = provider.generate_internal(&prompt, &config, &device, tx.clone()).await {
                let _ = tx.send(ImageGenerationChunk::Error(e));
            }
        });
        
        AsyncStream::new(rx)
    }

    fn model_name(&self) -> &str {
        "stable-diffusion-3.5-medium"
    }

    fn default_steps(&self) -> usize {
        4  // SD3.5 Turbo default
    }
}
```

**Why**: Implements the standard image generation interface.

**Definition of Done**:
- ✅ `generate()` returns AsyncStream<ImageGenerationChunk>
- ✅ `model_name()` returns correct identifier
- ✅ `default_steps()` returns 4 for turbo mode

---

## SUBTASK 2: Implement Internal Generation Logic

**File**: `packages/candle/src/providers/stable_diffusion.rs`

**What to Add**:
```rust
use tokio::sync::mpsc::UnboundedSender;

impl StableDiffusion35Provider {
    /// Internal generation implementation
    async fn generate_internal(
        &self,
        prompt: &str,
        config: &ImageGenerationConfig,
        device: &Device,
        tx: UnboundedSender<ImageGenerationChunk>,
    ) -> Result<(), String> {
        // 1. Encode text prompt (positive + negative for CFG)
        let (positive_emb, negative_emb) = self.encode_prompt_pair(
            prompt,
            config.negative_prompt.as_deref()
        )?;
        
        // 2. Initialize random latents
        let mut latents = self.randn_latents(
            1,  // batch_size
            config.height,
            config.width
        )?;
        
        // 3. Apply noise schedule scaling
        latents = (latents * self.scheduler.init_noise_sigma())?;
        
        // 4. Get denoising timesteps
        let timesteps = self.get_timesteps(config.steps);
        
        // 5. Denoising loop
        for (step, &timestep) in timesteps.iter().enumerate() {
            // Predict noise with CFG
            let noise_pred = self.predict_noise_cfg(
                &latents,
                timestep,
                &positive_emb,
                &negative_emb,
                config.guidance_scale
            )?;
            
            // Scheduler step (update latents)
            latents = self.scheduler_step(&noise_pred, timestep, &latents)?;
            
            // Emit progress chunk
            let _ = tx.send(ImageGenerationChunk::Step {
                step,
                total: config.steps,
                latent: latents.clone(),
            });
        }
        
        // 6. Decode latents to image via VAE
        let image = self.vae.decode(&latents)
            .map_err(|e| format!("VAE decode failed: {}", e))?;
        
        // 7. Normalize to [0, 1] range
        let normalized = image
            .affine(1.0 / 255.0, 0.0)
            .and_then(|t| t.clamp(0.0, 1.0))
            .map_err(|e| format!("Normalization failed: {}", e))?;
        
        // 8. Emit final image
        let _ = tx.send(ImageGenerationChunk::Complete {
            image: normalized,
        });
        
        Ok(())
    }
}
```

**Why**: Core generation logic with denoising loop and progress updates.

**Definition of Done**:
- ✅ Text prompts encoded correctly
- ✅ Random latents initialized and scaled
- ✅ Denoising loop runs for configured steps
- ✅ Progress chunks emitted during generation
- ✅ Final image decoded and normalized
- ✅ Complete chunk emitted with tensor

---

## SUBTASK 3: Implement CFG Noise Prediction

**File**: `packages/candle/src/providers/stable_diffusion.rs`

**What to Add**:
```rust
impl StableDiffusion35Provider {
    /// Predict noise with Classifier-Free Guidance
    /// 
    /// CFG formula: noise = uncond + scale * (cond - uncond)
    fn predict_noise_cfg(
        &self,
        latents: &Tensor,
        timestep: usize,
        positive_emb: &Tensor,
        negative_emb: &Tensor,
        guidance_scale: f64,
    ) -> Result<Tensor, String> {
        // Duplicate latents for batched prediction
        let latent_model_input = Tensor::cat(&[latents, latents], 0)
            .map_err(|e| format!("Failed to cat latents: {}", e))?;
        
        // Concatenate embeddings (negative first, then positive)
        let text_embeddings = Tensor::cat(&[negative_emb, positive_emb], 0)
            .map_err(|e| format!("Failed to cat embeddings: {}", e))?;
        
        // Predict noise for both conditions
        let noise_pred = self.unet.forward(
            &latent_model_input,
            timestep as f64,
            &text_embeddings
        ).map_err(|e| format!("UNet forward failed at t={}: {}", timestep, e))?;
        
        // Split predictions
        let noise_pred_uncond = noise_pred.narrow(0, 0, 1)?;
        let noise_pred_text = noise_pred.narrow(0, 1, 1)?;
        
        // Apply CFG: uncond + scale * (text - uncond)
        let diff = (noise_pred_text - &noise_pred_uncond)?;
        let scaled_diff = (diff * guidance_scale)?;
        let guided_noise = (noise_pred_uncond + scaled_diff)?;
        
        Ok(guided_noise)
    }
}
```

**Why**: Classifier-Free Guidance improves prompt adherence quality.

**Definition of Done**:
- ✅ Latents duplicated for batched inference
- ✅ Embeddings concatenated correctly (negative, positive order)
- ✅ UNet predicts noise for both conditions
- ✅ CFG formula applied correctly
- ✅ Guided noise returned

---

## SUBTASK 4: Implement Image-to-Image

**File**: `packages/candle/src/providers/stable_diffusion.rs`

**What to Add**:
```rust
impl StableDiffusion35Provider {
    /// Image-to-image generation (transform existing image)
    /// 
    /// Takes input image, encodes to latent, adds noise, denoises with prompt.
    /// Uses Image builder for preprocessing.
    pub async fn img2img(
        &self,
        input_image_path: &str,
        prompt: &str,
        config: &ImageGenerationConfig,
    ) -> Result<Tensor, String> {
        // 1. Preprocess input image using Image builder
        let input_tensor = Image::from_path(input_image_path)
            .resize(
                config.width,
                config.height,
                ResizeFilter::CatmullRom
            )
            .normalize_unsigned()  // [0,255] → [0,1]
            .clamp(0.0, 1.0)
            .to_tensor(&self.device)
            .await?;
        
        // 2. Encode image to latent space via VAE
        let latents = self.vae.encode(&input_tensor.unsqueeze(0)?)
            .map_err(|e| format!("VAE encode failed: {}", e))?;
        
        // 3. Add noise based on strength (0.0 = no change, 1.0 = full denoise)
        let strength = config.strength.unwrap_or(0.8);
        let noise = self.randn_latents(1, config.height, config.width)?;
        let noisy_latents = self.add_noise(&latents, &noise, strength)?;
        
        // 4. Encode text prompt
        let (positive_emb, negative_emb) = self.encode_prompt_pair(
            prompt,
            config.negative_prompt.as_deref()
        )?;
        
        // 5. Denoise (fewer steps than text-to-image)
        let start_step = (config.steps as f32 * (1.0 - strength)) as usize;
        let timesteps = &self.get_timesteps(config.steps)[start_step..];
        
        let mut current_latents = noisy_latents;
        for &timestep in timesteps {
            let noise_pred = self.predict_noise_cfg(
                &current_latents,
                timestep,
                &positive_emb,
                &negative_emb,
                config.guidance_scale
            )?;
            
            current_latents = self.scheduler_step(&noise_pred, timestep, &current_latents)?;
        }
        
        // 6. Decode final latents to image
        let output = self.vae.decode(&current_latents)
            .map_err(|e| format!("VAE decode failed: {}", e))?;
        
        // 7. Normalize to [0, 1]
        let normalized = output
            .affine(1.0 / 255.0, 0.0)
            .and_then(|t| t.clamp(0.0, 1.0))
            .map_err(|e| format!("Normalization failed: {}", e))?;
        
        Ok(normalized)
    }

    /// Image-to-image from URL
    pub async fn img2img_url(
        &self,
        input_url: &str,
        prompt: &str,
        config: &ImageGenerationConfig,
    ) -> Result<Tensor, String> {
        let input_tensor = Image::from_url(input_url)
            .resize(config.width, config.height, ResizeFilter::CatmullRom)
            .normalize_unsigned()
            .clamp(0.0, 1.0)
            .to_tensor(&self.device)
            .await?;
        
        // (Rest of img2img pipeline same as above)
        // ... encode, denoise, decode ...
    }

    /// Helper: Add noise to latents based on strength
    fn add_noise(&self, latents: &Tensor, noise: &Tensor, strength: f32) -> Result<Tensor, String> {
        // Weighted sum: latents * (1 - strength) + noise * strength
        let latent_part = (latents * (1.0 - strength as f64))?;
        let noise_part = (noise * strength as f64)?;
        (latent_part + noise_part)
            .map_err(|e| format!("Noise addition failed: {}", e))
    }
}
```

**Why**: Image-to-image enables style transfer and image transformation.

**Critical**: Image builder MUST be used for input preprocessing.

**Definition of Done**:
- ✅ `img2img()` transforms images based on prompts
- ✅ Image builder handles input preprocessing
- ✅ VAE encodes input to latent space
- ✅ Noise added based on strength parameter
- ✅ Denoising starts from intermediate step
- ✅ Output image normalized correctly
- ✅ URL variant works

---

## SUBTASK 5: Add ImageGenerationConfig Extensions

**File**: `packages/candle/src/domain/image_generation/mod.rs`

**What to Add** (if not already present):
```rust
#[derive(Debug, Clone)]
pub struct ImageGenerationConfig {
    pub width: usize,
    pub height: usize,
    pub steps: usize,
    pub guidance_scale: f64,
    pub negative_prompt: Option<String>,
    pub seed: Option<u64>,
    pub use_flash_attn: bool,
    pub strength: Option<f32>,  // NEW: for img2img (0.0 to 1.0)
}

impl Default for ImageGenerationConfig {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 1024,
            steps: 4,
            guidance_scale: 3.5,
            negative_prompt: None,
            seed: None,
            use_flash_attn: false,
            strength: Some(0.8),  // NEW: default 80% transformation
        }
    }
}
```

**Why**: Adds strength parameter for img2img control.

**Definition of Done**:
- ✅ `strength` field added if missing
- ✅ Default value set to 0.8
- ✅ Field properly optional (Some/None)

---

## RESEARCH REFERENCES

### Denoising Loop
- Start with random Gaussian noise
- Iteratively predict and remove noise
- Conditioning via text embeddings
- Scheduler manages timestep progression

### CFG (Classifier-Free Guidance)
- **Formula**: `noise = uncond + scale * (cond - uncond)`
- **uncond**: Prediction with empty/negative prompt
- **cond**: Prediction with actual prompt
- **scale**: Controls prompt adherence (higher = stricter)

### Image-to-Image
- Encode input image to latent space
- Add noise proportional to strength
- Denoise starting from intermediate step
- Fewer denoising steps than txt2img

### VAE Latent Space
- 8x compression: 1024×1024 → 128×128
- 4 channels instead of 3 (RGB)
- Gaussian distribution
- Decode reverses encoding

### Image Builder Integration (img2img only)
- **File**: [`packages/candle/src/builders/image.rs`](../packages/candle/src/builders/image.rs)
- Input images preprocessed via Image builder
- `.resize()` + `.normalize_unsigned()` + `.clamp()`
- CatmullRom filter for high quality
- Output from SD goes through tensor_to_image()

---

## CRITICAL REQUIREMENTS

### ✅ Generation Pipeline
- Random latents initialized correctly
- Text embeddings condition denoising
- CFG improves prompt adherence
- Progress chunks emitted each step
- Final image properly decoded and normalized

### ✅ Image-to-Image
- **MUST USE** Image builder for input preprocessing
- **MUST USE** `.resize(width, height, ResizeFilter::CatmullRom)`
- **MUST USE** `.normalize_unsigned()` and `.clamp(0.0, 1.0)`
- **NO MANUAL** image loading allowed
- Strength parameter controls transformation amount

### ✅ Streaming
- AsyncStream for progress monitoring
- Step chunks show denoising progress
- Complete chunk has final image tensor
- Error chunks for failures
- Proper async/await usage

---

## DEFINITION OF DONE

1. ✅ `ImageGenerationModel` trait implemented for StableDiffusion35Provider
2. ✅ `generate()` returns streaming AsyncStream<ImageGenerationChunk>
3. ✅ `generate_internal()` implements complete txt2img pipeline
4. ✅ `predict_noise_cfg()` applies Classifier-Free Guidance correctly
5. ✅ Progress chunks emitted during denoising loop
6. ✅ Final image decoded via VAE and normalized to [0,1]
7. ✅ Complete chunk emitted with final tensor
8. ✅ `img2img()` transforms images based on prompts
9. ✅ `img2img_url()` works for web-hosted images
10. ✅ Image builder used for all img2img input preprocessing
11. ✅ Strength parameter controls transformation amount
12. ✅ `add_noise()` helper mixes latents and noise correctly
13. ✅ ImageGenerationConfig has strength field (if missing)
14. ✅ File compiles without errors

---

## NO TESTS OR BENCHMARKS

**Do NOT create**:
- Unit tests for generation quality
- Integration tests comparing outputs
- Benchmark speed measurements
- Example prompts or test generations
- Image quality comparisons

**Reason**: Testing team validates generation quality. Focus on implementation only.

---

## COMPLETION

With VISION_5 complete, the vision AI implementation is finished:

✅ **VISION_1**: CLIP provider for image embeddings
✅ **VISION_2**: Embedding service multimodal integration  
✅ **VISION_3**: LLaVA provider for vision-language chat
✅ **VISION_4**: Stable Diffusion infrastructure setup
✅ **VISION_5**: Stable Diffusion generation pipeline

**Result**: Complete multimodal AI framework with image understanding, visual chat, and image generation capabilities.