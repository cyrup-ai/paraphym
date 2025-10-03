# VISION_4: Stable Diffusion Provider Infrastructure

## OBJECTIVE

Create the foundational infrastructure for Stable Diffusion 3.5, including model loading (VAE, UNet, text encoder), configuration management, and text prompt encoding. This sets up the components needed for image generation in VISION_5.

---

## BACKGROUND

**Current State**:
- ImageGenerationModel trait exists in `packages/candle/src/domain/image_generation/mod.rs`
- No implementations of the trait
- tensor_to_image() conversion function exists

**What This Task Accomplishes**:
- Stable Diffusion provider struct and configuration
- Model component loading (VAE, UNet, CLIP text encoder)
- Text prompt encoding pipeline
- Scheduler initialization

---

## SUBTASK 1: Create Stable Diffusion Configuration

**File**: `packages/candle/src/providers/stable_diffusion.rs` (NEW)

**What to Create**:
```rust
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::stable_diffusion::{
    vae::AutoEncoderKL,
    unet_2d::UNet2DConditionModel,
    clip::ClipTextTransformer,
};
use crate::domain::image_generation::{
    ImageGenerationModel,
    ImageGenerationConfig,
    ImageGenerationChunk,
};
use crate::builders::image::{Image, ResizeFilter};
use ystream::AsyncStream;

/// Stable Diffusion 3.5 configuration
#[derive(Clone)]
pub struct SD35Config {
    /// VAE scale factor (latent space compression, typically 8)
    pub vae_scale_factor: usize,
    /// Number of UNet channels
    pub unet_channels: usize,
    /// CLIP text encoder hidden size
    pub text_encoder_hidden_size: usize,
    /// Default image dimensions
    pub default_width: usize,
    pub default_height: usize,
}

impl Default for SD35Config {
    fn default() -> Self {
        Self {
            vae_scale_factor: 8,           // 8x compression
            unet_channels: 320,            // SD3.5 Medium
            text_encoder_hidden_size: 768, // CLIP base
            default_width: 1024,           // SD3.5 native
            default_height: 1024,
        }
    }
}
```

**Why**: Configuration struct for model parameters and defaults.

**Definition of Done**:
- ✅ SD35Config struct created
- ✅ Default values match SD3.5 specifications
- ✅ All fields documented

---

## SUBTASK 2: Create StableDiffusion35Provider Struct

**File**: `packages/candle/src/providers/stable_diffusion.rs`

**What to Add**:
```rust
/// Stable Diffusion 3.5 provider for text-to-image generation
pub struct StableDiffusion35Provider {
    /// VAE for encoding/decoding images ↔ latents
    vae: AutoEncoderKL,
    /// UNet for denoising latents
    unet: UNet2DConditionModel,
    /// CLIP text encoder for prompt embeddings
    text_encoder: ClipTextTransformer,
    /// Noise scheduler (DDPM, DDIM, etc.)
    scheduler: DdpmScheduler,
    /// Model configuration
    config: SD35Config,
    /// Device (CPU/CUDA/Metal)
    device: Device,
}

impl StableDiffusion35Provider {
    pub fn from_pretrained(model_path: &str, device: Device) -> Result<Self, String> {
        let config = SD35Config::default();
        
        // This subtask loads the models - implementation below
        
        Ok(Self {
            vae,
            unet,
            text_encoder,
            scheduler,
            config,
            device,
        })
    }
}
```

**Why**: Main provider struct holding all SD components.

**Definition of Done**:
- ✅ Struct defined with all components
- ✅ `from_pretrained()` signature correct
- ✅ Device parameter stored

---

## SUBTASK 3: Implement Model Loading

**File**: `packages/candle/src/providers/stable_diffusion.rs`

**What to Add** (in `from_pretrained()` method):
```rust
pub fn from_pretrained(model_path: &str, device: Device) -> Result<Self, String> {
    let config = SD35Config::default();
    
    // Load VAE
    let vae_path = format!("{}/vae/diffusion_pytorch_model.safetensors", model_path);
    let vae_vb = VarBuilder::from_mmaped_safetensors(
        &[vae_path.into()],
        candle_core::DType::F32,
        &device
    ).map_err(|e| format!("Failed to load VAE from {}: {}", model_path, e))?;
    
    let vae = AutoEncoderKL::new(vae_vb, 3, 3)  // in_channels=3 (RGB), out_channels=3
        .map_err(|e| format!("Failed to create VAE: {}", e))?;
    
    // Load UNet
    let unet_path = format!("{}/unet/diffusion_pytorch_model.safetensors", model_path);
    let unet_vb = VarBuilder::from_mmaped_safetensors(
        &[unet_path.into()],
        candle_core::DType::F32,
        &device
    ).map_err(|e| format!("Failed to load UNet from {}: {}", model_path, e))?;
    
    let unet = UNet2DConditionModel::new(
        unet_vb,
        /* in_channels */ 4,  // Latent space
        /* out_channels */ 4,
        /* cross_attention_dim */ config.text_encoder_hidden_size,
    ).map_err(|e| format!("Failed to create UNet: {}", e))?;
    
    // Load text encoder (CLIP)
    let text_encoder_path = format!("{}/text_encoder/model.safetensors", model_path);
    let text_encoder_vb = VarBuilder::from_mmaped_safetensors(
        &[text_encoder_path.into()],
        candle_core::DType::F32,
        &device
    ).map_err(|e| format!("Failed to load text encoder from {}: {}", model_path, e))?;
    
    let text_encoder = ClipTextTransformer::new(text_encoder_vb, &Default::default())
        .map_err(|e| format!("Failed to create text encoder: {}", e))?;
    
    // Initialize scheduler (DDPM for SD3.5)
    let scheduler = DdpmScheduler::new(1000)?;  // 1000 timesteps
    
    Ok(Self {
        vae,
        unet,
        text_encoder,
        scheduler,
        config,
        device,
    })
}
```

**Why**: Loads all three model components from disk.

**Note**: Adjust paths and parameters based on actual SD3.5 model structure.

**Definition of Done**:
- ✅ VAE loads successfully
- ✅ UNet loads successfully
- ✅ Text encoder loads successfully
- ✅ Scheduler initialized
- ✅ Error messages are descriptive with paths

---

## SUBTASK 4: Implement Text Prompt Encoding

**File**: `packages/candle/src/providers/stable_diffusion.rs`

**What to Add**:
```rust
impl StableDiffusion35Provider {
    /// Encode text prompt to embeddings for conditioning
    /// 
    /// Returns text embeddings that condition the denoising process.
    fn encode_prompt(&self, prompt: &str) -> Result<Tensor, String> {
        // Tokenize prompt
        let tokens = self.text_encoder.tokenizer()
            .encode(prompt, true)
            .map_err(|e| format!("Failed to tokenize prompt '{}': {}", prompt, e))?;
        
        let token_ids = tokens.get_ids();
        let token_tensor = Tensor::new(token_ids, &self.device)
            .map_err(|e| format!("Failed to create token tensor: {}", e))?;
        
        // Encode through CLIP text transformer
        let embeddings = self.text_encoder.forward(&token_tensor)
            .map_err(|e| format!("Text encoding failed: {}", e))?;
        
        Ok(embeddings)
    }

    /// Encode both prompt and negative prompt for CFG
    /// 
    /// Classifier-free guidance requires both positive and negative embeddings.
    fn encode_prompt_pair(
        &self,
        prompt: &str,
        negative_prompt: Option<&str>
    ) -> Result<(Tensor, Tensor), String> {
        // Encode positive prompt
        let positive_emb = self.encode_prompt(prompt)?;
        
        // Encode negative prompt (or empty string)
        let neg_text = negative_prompt.unwrap_or("");
        let negative_emb = self.encode_prompt(neg_text)?;
        
        Ok((positive_emb, negative_emb))
    }
}
```

**Why**: Text conditioning is required for Stable Diffusion generation.

**Definition of Done**:
- ✅ `encode_prompt()` converts text to embeddings
- ✅ `encode_prompt_pair()` handles CFG (positive + negative)
- ✅ Tokenization and encoding work correctly
- ✅ Embeddings have correct shape for UNet conditioning

---

## SUBTASK 5: Implement Scheduler Helper Methods

**File**: `packages/candle/src/providers/stable_diffusion.rs`

**What to Add**:
```rust
use candle_transformers::models::stable_diffusion::schedulers::{
    DdpmScheduler,
    SchedulerConfig,
};

impl StableDiffusion35Provider {
    /// Initialize random latent tensor for generation start
    fn randn_latents(&self, batch_size: usize, height: usize, width: usize) -> Result<Tensor, String> {
        let latent_height = height / self.config.vae_scale_factor;
        let latent_width = width / self.config.vae_scale_factor;
        
        // Random normal distribution for initial noise
        Tensor::randn(
            0f32,  // mean
            1f32,  // std
            (batch_size, 4, latent_height, latent_width),  // 4 channels in latent space
            &self.device
        ).map_err(|e| format!("Failed to create random latents: {}", e))
    }

    /// Get timesteps for denoising loop
    fn get_timesteps(&self, num_steps: usize) -> Vec<usize> {
        self.scheduler.timesteps(num_steps)
    }

    /// Apply single denoising step
    fn scheduler_step(
        &self,
        noise_pred: &Tensor,
        timestep: usize,
        latents: &Tensor
    ) -> Result<Tensor, String> {
        self.scheduler.step(noise_pred, timestep, latents)
            .map_err(|e| format!("Scheduler step failed at t={}: {}", timestep, e))
    }
}
```

**Why**: Scheduler manages the denoising process timing and steps.

**Definition of Done**:
- ✅ `randn_latents()` creates initial noise
- ✅ `get_timesteps()` returns denoising schedule
- ✅ `scheduler_step()` applies single denoising iteration
- ✅ Latent dimensions match VAE compression ratio

---

## SUBTASK 6: Update Providers Module

**File**: `packages/candle/src/providers/mod.rs`

**What to Add**:
```rust
pub mod stable_diffusion;
pub use stable_diffusion::{StableDiffusion35Provider, SD35Config};
```

**Why**: Makes Stable Diffusion provider accessible.

**Definition of Done**:
- ✅ Module export added
- ✅ Both provider and config exported
- ✅ `use paraphym_candle::providers::StableDiffusion35Provider;` compiles

---

## RESEARCH REFERENCES

### Stable Diffusion Architecture
- **VAE**: Encodes images to latent space (8x compression)
- **UNet**: Denoises latents conditioned on text embeddings
- **CLIP**: Encodes text prompts to conditioning vectors
- **Scheduler**: Manages denoising timesteps (DDPM, DDIM, etc.)

### Model Loading Pattern
- SafeTensors format used by Candle
- Separate files for VAE, UNet, text encoder
- VarBuilder loads mmap'd tensors for efficiency

### Latent Space
- 4 channels (vs 3 for RGB)
- 8x spatial compression (1024×1024 image → 128×128 latent)
- Gaussian distribution for initial noise

### CFG (Classifier-Free Guidance)
- Requires both positive and negative prompt embeddings
- guidance_scale controls prompt adherence
- Formula: `guided = negative + scale * (positive - negative)`

---

## CRITICAL REQUIREMENTS

### ✅ Model Component Loading
- VAE, UNet, and text encoder loaded separately
- Correct parameter counts and dimensions
- Error handling for missing or corrupt files
- Device placement consistent across components

### ✅ Text Encoding
- Tokenization handles prompt length limits
- Embeddings have correct shape for UNet
- CFG prompt pairs work correctly
- Empty negative prompt supported

### ✅ Scheduler Setup
- Timesteps computed correctly
- Noise schedule appropriate for model
- Step function updates latents properly

---

## DEFINITION OF DONE

1. ✅ File `packages/candle/src/providers/stable_diffusion.rs` created
2. ✅ SD35Config struct with correct defaults
3. ✅ StableDiffusion35Provider struct defined
4. ✅ `from_pretrained()` loads VAE successfully
5. ✅ `from_pretrained()` loads UNet successfully
6. ✅ `from_pretrained()` loads text encoder successfully
7. ✅ Scheduler initialized (DDPM with 1000 steps)
8. ✅ `encode_prompt()` converts text to embeddings
9. ✅ `encode_prompt_pair()` handles CFG prompts
10. ✅ `randn_latents()` creates initial noise
11. ✅ `get_timesteps()` returns denoising schedule
12. ✅ `scheduler_step()` applies denoising iteration
13. ✅ Module export in `providers/mod.rs` complete
14. ✅ File compiles without errors

---

## NO TESTS OR BENCHMARKS

**Do NOT create**:
- Unit tests for model loading
- Integration tests for text encoding
- Benchmark performance measurements
- Example prompts or test generations

**Reason**: Testing team handles validation. Focus on infrastructure only.

**Next Task**: VISION_5 will implement the actual generation pipeline using these components.