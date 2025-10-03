# IMGGEN_3: FLUX.1-schnell Provider

## OBJECTIVE
Implement FLUX.1-schnell provider for fast 4-step text-to-image generation with T5-XXL + CLIP dual text encoding.

## PRIORITY
🔴 CRITICAL - Fast alternative image generation model

## ARCHITECTURE OVERVIEW

### What Already Exists in Candle-Transformers

FLUX has **complete implementation** in `candle-transformers::models::flux`:

**Available Models** (`./tmp/candle/candle-transformers/src/models/flux/model.rs`):
- `Flux` transformer struct with `Config::schnell()` and `Config::dev()`
- `Config::schnell()`: guidance_embed = false (no CFG), 4-step inference
- `Config::dev()`: guidance_embed = true (supports CFG), 20-50 steps
- Complete WithForward trait implementation

**Available Sampling** (`./tmp/candle/candle-transformers/src/models/flux/sampling.rs`):
- `get_noise(num_samples, height, width, device)` - Initialize latent noise
- `State::new(t5_emb, clip_emb, img)` - Pack embeddings and image for diffusion
- `get_schedule(num_steps, shift)` - Timestep scheduling (4 steps for schnell)
- `denoise(model, img, img_ids, txt, txt_ids, vec, timesteps, guidance)` - Denoising loop
- `unpack(xs, height, width)` - Unpack latent patches to spatial dimensions

**Available VAE** (`./tmp/candle/candle-transformers/src/models/flux/autoencoder.rs`):
- `AutoEncoder` with `Config::schnell()` and `Config::dev()`
- Encoder/Decoder with ResNet blocks and attention
- DiagonalGaussian for latent distribution
- Scale/shift factors: scale_factor=0.3611, shift_factor=0.1159

### What Needs Implementation

**Text Encoding Wrappers** (adapt from `./tmp/candle-examples/candle-examples/examples/flux/main.rs:82-159`):
- T5-XXL tokenizer + encoder wrapper (256 token context)
- CLIP tokenizer + encoder wrapper (77 token context)
- Dual encoding: T5 for context embeddings, CLIP for pooled vector

**Provider Integration**:
- FluxSchnell struct wrapping Flux + AutoEncoder + encoders
- ImageGenerationModel trait implementation
- AsyncStream pattern for progress chunks
- HuggingFace Hub model loading

### Key Architectural Differences from SD3.5

| Feature | FLUX Schnell | SD3.5 Turbo |
|---------|-------------|-------------|
| Text Encoding | T5-XXL (256) + CLIP (77) | 3x CLIP (77 each) |
| Denoising | `denoise()` function | Manual Euler loop |
| Steps | 4 (fast inference) | 4 (turbo mode) |
| CFG | No (guidance_scale=0.0) | Yes (guidance_scale=3.5) |
| State | Packed patches + IDs | Direct latent |
| Dtype | BF16 preferred | F16/F32 |

## SUBTASK 1: Create Provider Structure

**What needs to change**:
- Create FLUX provider file following ImageGenerationModel pattern
- Use BF16 dtype (FLUX preference via `device.bf16_default_to_f32()`)

**Where changes happen**:
- NEW: `packages/candle/src/providers/flux_schnell.rs`

**Implementation**:
```rust
use std::sync::Arc;
use candle_core::{Device, DType, Tensor};
use candle_transformers::models::{flux, t5, clip};
use ystream::AsyncStream;
use crate::domain::image_generation::{
    ImageGenerationModel, 
    ImageGenerationConfig, 
    ImageGenerationChunk
};

pub struct FluxSchnell {
    flux_transformer: flux::model::Flux,
    t5_encoder: t5::T5EncoderModel,
    clip_encoder: clip::text_model::ClipTextTransformer,
    vae: flux::autoencoder::AutoEncoder,
    config: FluxConfig,
}

#[derive(Debug, Clone)]
pub struct FluxConfig {
    pub guidance_scale: f64,  // FLUX schnell uses 0.0 (no CFG)
    pub use_bf16: bool,       // Prefer bf16 for FLUX
}

impl Default for FluxConfig {
    fn default() -> Self {
        Self {
            guidance_scale: 0.0,  // FLUX schnell doesn't use CFG
            use_bf16: true,
        }
    }
}
```

**Reference**: 
- Flux model: `./tmp/candle/candle-transformers/src/models/flux/model.rs:40-57`
- Config::schnell() has guidance_embed=false (line 54)


## SUBTASK 2: Implement Model Loading from HuggingFace

**Reference**: `./tmp/candle-examples/candle-examples/examples/flux/main.rs:82-103`

**Implementation**:
```rust
impl FluxSchnell {
    pub fn from_pretrained(device: &Device) -> Result<Self, String> {
        let api = hf_hub::api::sync::Api::new()
            .map_err(|e| format!("HF API init failed: {}", e))?;
        
        // 1. Load FLUX transformer
        let flux_repo = api.repo(hf_hub::Repo::model(
            "black-forest-labs/FLUX.1-schnell".to_string()
        ));
        let flux_model_file = flux_repo.get("flux1-schnell.safetensors")
            .map_err(|e| format!("FLUX model download failed: {}", e))?;
        
        // 2. Load T5-XXL text encoder (with PR #2 revision for compatibility)
        let t5_repo = api.repo(hf_hub::Repo::with_revision(
            "google/t5-v1_1-xxl".to_string(),
            hf_hub::RepoType::Model,
            "refs/pr/2".to_string(),
        ));
        let t5_model_file = t5_repo.get("model.safetensors")
            .map_err(|e| format!("T5 model download failed: {}", e))?;
        let t5_config_file = t5_repo.get("config.json")
            .map_err(|e| format!("T5 config download failed: {}", e))?;
        
        // 3. Load CLIP text encoder
        let clip_repo = api.repo(hf_hub::Repo::model(
            "openai/clip-vit-large-patch14".to_string()
        ));
        let clip_model_file = clip_repo.get("model.safetensors")
            .map_err(|e| format!("CLIP model download failed: {}", e))?;
        
        // 4. Determine dtype (prefer bf16 if available)
        // Reference: ./tmp/candle-examples/candle-examples/examples/flux/main.rs:76
        let dtype = device.bf16_default_to_f32();
        
        // 5. Load T5 encoder
        let t5_config = std::fs::read_to_string(t5_config_file)
            .map_err(|e| format!("T5 config read failed: {}", e))?;
        let t5_config: t5::Config = serde_json::from_str(&t5_config)
            .map_err(|e| format!("T5 config parse failed: {}", e))?;
        let t5_vb = unsafe {
            candle_nn::VarBuilder::from_mmaped_safetensors(
                &[t5_model_file], 
                dtype, 
                device
            ).map_err(|e| format!("T5 VarBuilder failed: {}", e))?
        };
        let t5_encoder = t5::T5EncoderModel::load(t5_vb, &t5_config)
            .map_err(|e| format!("T5 encoder load failed: {}", e))?;
        
        // 6. Load CLIP encoder
        // Config from: ./tmp/candle-examples/candle-examples/examples/flux/main.rs:133-143
        let clip_vb = unsafe {
            candle_nn::VarBuilder::from_mmaped_safetensors(
                &[clip_model_file], 
                dtype, 
                device
            ).map_err(|e| format!("CLIP VarBuilder failed: {}", e))?
        };
        let clip_config = clip::text_model::ClipTextConfig {
            vocab_size: 49408,
            projection_dim: 768,
            activation: clip::text_model::Activation::QuickGelu,
            intermediate_size: 3072,
            embed_dim: 768,
            max_position_embeddings: 77,
            pad_with: None,
            num_hidden_layers: 12,
            num_attention_heads: 12,
        };
        let clip_encoder = clip::text_model::ClipTextTransformer::new(
            clip_vb.pp("text_model"), 
            &clip_config
        ).map_err(|e| format!("CLIP encoder load failed: {}", e))?;
        
        // 7. Load FLUX transformer
        // Reference: ./tmp/candle-examples/candle-examples/examples/flux/main.rs:169
        let flux_vb = unsafe {
            candle_nn::VarBuilder::from_mmaped_safetensors(
                &[flux_model_file], 
                dtype, 
                device
            ).map_err(|e| format!("FLUX VarBuilder failed: {}", e))?
        };
        let flux_transformer = flux::model::Flux::new(
            &flux::model::Config::schnell(), 
            flux_vb
        ).map_err(|e| format!("FLUX model load failed: {}", e))?;
        
        // 8. Load VAE
        // Reference: ./tmp/candle-examples/candle-examples/examples/flux/main.rs:233-238
        let vae_file = flux_repo.get("ae.safetensors")
            .map_err(|e| format!("VAE download failed: {}", e))?;
        let vae_vb = unsafe {
            candle_nn::VarBuilder::from_mmaped_safetensors(
                &[vae_file], 
                dtype, 
                device
            ).map_err(|e| format!("VAE VarBuilder failed: {}", e))?
        };
        let vae = flux::autoencoder::AutoEncoder::new(
            &flux::autoencoder::Config::schnell(), 
            vae_vb
        ).map_err(|e| format!("VAE load failed: {}", e))?;
        
        Ok(Self {
            flux_transformer,
            t5_encoder,
            clip_encoder,
            vae,
            config: FluxConfig::default(),
        })
    }
}
```

**Key Points**:
- BF16 via `device.bf16_default_to_f32()` (GPU: BF16, CPU: F32 fallback)
- T5 uses revision `refs/pr/2` for compatibility
- CLIP config hardcoded (matches openai/clip-vit-large-patch14)
- VAE loaded from `ae.safetensors` in FLUX repo


## SUBTASK 3: Implement Text Encoding

**Reference**: `./tmp/candle-examples/candle-examples/examples/flux/main.rs:91-159`

**Add method**:
```rust
impl FluxSchnell {
    fn encode_prompt(
        &self,
        prompt: &str,
        device: &Device,
    ) -> Result<(Tensor, Tensor), String> {
        // 1. T5-XXL encoding (longer context, 256 tokens)
        // Tokenizer from: ./tmp/candle-examples/candle-examples/examples/flux/main.rs:115-117
        let t5_tokenizer_path = hf_hub::api::sync::Api::new()
            .map_err(|e| format!("HF API failed: {}", e))?
            .model("lmz/mt5-tokenizers".to_string())
            .get("t5-v1_1-xxl.tokenizer.json")
            .map_err(|e| format!("T5 tokenizer download failed: {}", e))?;
        
        let t5_tokenizer = tokenizers::Tokenizer::from_file(t5_tokenizer_path)
            .map_err(|e| format!("T5 tokenizer load failed: {}", e))?;
        
        let mut t5_tokens = t5_tokenizer
            .encode(prompt, true)
            .map_err(|e| format!("T5 tokenization failed: {}", e))?
            .get_ids()
            .to_vec();
        
        // Resize to exactly 256 tokens (FLUX requirement)
        // Reference: ./tmp/candle-examples/candle-examples/examples/flux/main.rs:122
        t5_tokens.resize(256, 0);
        
        let t5_input = Tensor::new(&t5_tokens[..], device)
            .map_err(|e| format!("T5 tensor creation failed: {}", e))?
            .unsqueeze(0)
            .map_err(|e| format!("T5 unsqueeze failed: {}", e))?;
        
        let t5_emb = self.t5_encoder.forward(&t5_input)
            .map_err(|e| format!("T5 forward failed: {}", e))?;
        
        // 2. CLIP encoding (for pooled embeddings, 77 tokens max)
        // Reference: ./tmp/candle-examples/candle-examples/examples/flux/main.rs:145-156
        let clip_tokenizer_path = hf_hub::api::sync::Api::new()
            .map_err(|e| format!("HF API failed: {}", e))?
            .model("openai/clip-vit-large-patch14".to_string())
            .get("tokenizer.json")
            .map_err(|e| format!("CLIP tokenizer download failed: {}", e))?;
        
        let clip_tokenizer = tokenizers::Tokenizer::from_file(clip_tokenizer_path)
            .map_err(|e| format!("CLIP tokenizer load failed: {}", e))?;
        
        let clip_tokens = clip_tokenizer
            .encode(prompt, true)
            .map_err(|e| format!("CLIP tokenization failed: {}", e))?
            .get_ids()
            .to_vec();
        
        let clip_input = Tensor::new(&clip_tokens[..], device)
            .map_err(|e| format!("CLIP tensor creation failed: {}", e))?
            .unsqueeze(0)
            .map_err(|e| format!("CLIP unsqueeze failed: {}", e))?;
        
        let clip_emb = self.clip_encoder.forward(&clip_input)
            .map_err(|e| format!("CLIP forward failed: {}", e))?;
        
        Ok((t5_emb, clip_emb))
    }
}
```

**Key Points**:
- T5: Exactly 256 tokens (resize with padding/truncation)
- CLIP: Natural tokenization (max 77 via tokenizer)
- Both embeddings needed for FLUX State preparation


## SUBTASK 4: Implement 4-Step Denoising Generation

**FLUX uses 4-step denoising, not single-step** (original task had misconception)

**Reference**: `./tmp/candle-examples/candle-examples/examples/flux/main.rs:164-191`

**Add method**:
```rust
impl FluxSchnell {
    fn generate_image(
        &self,
        t5_emb: &Tensor,
        clip_emb: &Tensor,
        config: &ImageGenerationConfig,
        device: &Device,
        sender: &ystream::Sender<ImageGenerationChunk>,
    ) -> Result<Tensor, String> {
        // 1. Initialize noise
        // Reference: ./tmp/candle/candle-transformers/src/models/flux/sampling.rs:3-11
        let img = flux::sampling::get_noise(1, config.height, config.width, device)
            .map_err(|e| format!("Noise generation failed: {}", e))?
            .to_dtype(t5_emb.dtype())
            .map_err(|e| format!("Noise dtype conversion failed: {}", e))?;
        
        // 2. Prepare State (packs embeddings + image for diffusion)
        // Reference: ./tmp/candle/candle-transformers/src/models/flux/sampling.rs:13-60
        let state = flux::sampling::State::new(t5_emb, clip_emb, &img)
            .map_err(|e| format!("State preparation failed: {}", e))?;
        
        // 3. Get timestep schedule (4 steps for schnell, no shift)
        // Reference: ./tmp/candle/candle-transformers/src/models/flux/sampling.rs:67-84
        let timesteps = flux::sampling::get_schedule(4, None);
        
        // 4. Denoise loop (emits progress chunks)
        // Reference: ./tmp/candle/candle-transformers/src/models/flux/sampling.rs:90-112
        let guidance = 0.0; // FLUX schnell has no CFG
        
        // Track progress through denoising steps
        let total_steps = timesteps.len() - 1; // Windows = steps
        for (step, _window) in timesteps.windows(2).enumerate() {
            // Emit progress chunk
            let _ = sender.send(ImageGenerationChunk::Step {
                step,
                total: total_steps,
                latent: state.img.clone(),
            });
        }
        
        // Run full denoise (no per-step streaming in candle-transformers implementation)
        let denoised = flux::sampling::denoise(
            &self.flux_transformer,
            &state.img,
            &state.img_ids,
            &state.txt,
            &state.txt_ids,
            &state.vec,
            &timesteps,
            guidance,
        ).map_err(|e| format!("Denoising failed: {}", e))?;
        
        // 5. Unpack latent patches back to spatial dimensions
        // Reference: ./tmp/candle/candle-transformers/src/models/flux/sampling.rs:86-93
        let unpacked = flux::sampling::unpack(&denoised, config.height, config.width)
            .map_err(|e| format!("Unpack failed: {}", e))?;
        
        // 6. VAE decode to pixel space
        // Reference: ./tmp/candle-examples/candle-examples/examples/flux/main.rs:233-242
        let decoded = self.vae.decode(&unpacked)
            .map_err(|e| format!("VAE decode failed: {}", e))?;
        
        // 7. Post-process: scale from [-1, 1] to [0, 1]
        // Reference: ./tmp/candle-examples/candle-examples/examples/flux/main.rs:244
        let image = ((decoded.clamp(-1f32, 1f32)? + 1.0)? * 0.5)?;
        
        Ok(image)
    }
}
```

**Key Differences from Original Task**:
- ❌ NOT single-step generation
- ✅ 4-step denoising via `denoise()` function
- ✅ State preparation packs embeddings + image patches + positional IDs
- ✅ No manual Euler loop (handled by `denoise()`)
- ✅ Unpack required after denoising
- ✅ No CFG (guidance = 0.0)


## SUBTASK 5: Implement ImageGenerationModel Trait

**Integration of all methods**:
```rust
impl ImageGenerationModel for FluxSchnell {
    fn generate(
        &self,
        prompt: &str,
        config: &ImageGenerationConfig,
        device: &Device,
    ) -> AsyncStream<ImageGenerationChunk> {
        AsyncStream::with_channel(|sender| {
            // 1. Set seed if specified
            if let Some(seed) = config.seed {
                if let Err(e) = device.set_seed(seed) {
                    let _ = sender.send(ImageGenerationChunk::Error(
                        format!("Seed setting failed: {}", e)
                    ));
                    return;
                }
            }
            
            // 2. Encode text prompt (T5 + CLIP)
            let (t5_emb, clip_emb) = match self.encode_prompt(prompt, device) {
                Ok(result) => result,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(e));
                    return;
                }
            };
            
            // 3. Generate image (4-step denoise)
            let image = match self.generate_image(
                &t5_emb, 
                &clip_emb, 
                config, 
                device, 
                &sender
            ) {
                Ok(result) => result,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(e));
                    return;
                }
            };
            
            // 4. Send final result
            let _ = sender.send(ImageGenerationChunk::Complete { image });
        })
    }
    
    fn model_name(&self) -> &str {
        "flux.1-schnell"
    }
    
    fn default_steps(&self) -> usize {
        4  // FLUX schnell is 4-step
    }
}
```

## REQUIRED IMPORTS

```rust
use std::sync::Arc;
use candle_core::{Device, DType, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::{
    flux,
    t5,
    clip,
};
use tokenizers::Tokenizer;
use ystream::AsyncStream;
use crate::domain::image_generation::{
    ImageGenerationModel, 
    ImageGenerationConfig, 
    ImageGenerationChunk
};
```


## DEFINITION OF DONE

- [ ] File created: `packages/candle/src/providers/flux_schnell.rs`
- [ ] FluxSchnell struct with transformer, T5, CLIP, VAE
- [ ] FluxConfig struct with Default impl (guidance_scale: 0.0)
- [ ] from_pretrained() loads models from HuggingFace Hub
- [ ] encode_prompt() handles T5-XXL + CLIP encoding
- [ ] generate_image() implements 4-step denoising via denoise()
- [ ] ImageGenerationModel trait fully implemented
- [ ] All errors use Result and map_err() with context
- [ ] No unwrap() or expect() in code
- [ ] cargo check passes

## REFERENCES

**Primary Implementation Reference**:
- FLUX example: `./tmp/candle-examples/candle-examples/examples/flux/main.rs`
- Model implementation: `./tmp/candle/candle-transformers/src/models/flux/model.rs`
- Sampling functions: `./tmp/candle/candle-transformers/src/models/flux/sampling.rs`
- AutoEncoder: `./tmp/candle/candle-transformers/src/models/flux/autoencoder.rs`

**Key Code References**:
- BF16 dtype: `main.rs:76` (`device.bf16_default_to_f32()`)
- T5 loading: `main.rs:91-125`
- CLIP loading: `main.rs:129-156`
- FLUX loading: `main.rs:164-175`
- State preparation: `sampling.rs:13-60` (`State::new()`)
- Denoising: `sampling.rs:90-112` (`denoise()`)
- Timesteps: `sampling.rs:67-84` (`get_schedule(4, None)`)
- Unpacking: `sampling.rs:86-93` (`unpack()`)
- VAE decode: `main.rs:233-242`

**Pattern Reference**:
- Provider structure: `./packages/candle/src/providers/stable_diffusion_35_turbo.rs`
- AsyncStream pattern: `./packages/candle/src/providers/kimi_k2.rs`
- Domain types: `./packages/candle/src/domain/image_generation/mod.rs`

## KEY DIFFERENCES FROM SD3.5

1. **Text Encoding**: T5-XXL (256 tokens) + CLIP (77 tokens) instead of triple CLIP
2. **Guidance**: No CFG (guidance_scale = 0.0, guidance_embed = false)
3. **Steps**: 4 steps via `denoise()` function (not single-step as originally thought)
4. **Dtype**: BF16 preferred over F16
5. **Sampling**: `denoise()` function instead of manual Euler loop
6. **State**: Requires State::new() for packing embeddings + image patches + positional IDs
7. **Unpacking**: Must call `unpack()` after denoising before VAE decode

## IMPORTANT CONSTRAINTS

- ❌ NO unit tests (separate team)
- ❌ NO benchmarks (separate team)
- ❌ NO examples in this file (IMGGEN_4 task)
- ✅ YES comprehensive error handling
- ✅ YES 4-step streaming (step 0-3 of 4)
- ✅ YES BF16 optimization when possible

## VERIFICATION

```bash
# Build check
cargo check -p paraphym_candle

# Verify no unwrap/expect
grep -n "unwrap\|expect" packages/candle/src/providers/flux_schnell.rs
# Should return nothing

# Check HF models exist
curl -I https://huggingface.co/black-forest-labs/FLUX.1-schnell
curl -I https://huggingface.co/google/t5-v1_1-xxl
curl -I https://huggingface.co/openai/clip-vit-large-patch14
```

## TROUBLESHOOTING

If implementation fails:

1. **State preparation errors**:
   - Ensure T5 output is (batch, 256, hidden_dim)
   - Ensure CLIP output is (batch, seq_len, hidden_dim)
   - Check image tensor is (batch, channels, height, width)

2. **Denoising errors**:
   - Verify timesteps from `get_schedule(4, None)` has 5 elements (0.0 to 1.0)
   - Check guidance = 0.0 (FLUX schnell has no CFG)
   - Ensure img_ids and txt_ids properly created by State::new()

3. **VAE decode errors**:
   - Must call `unpack()` before decode
   - Check unpacked shape matches (batch, 16, height*2, width*2)
   - Verify scale/shift factors: 0.3611 and 0.1159

4. **Out of Memory**:
   - FLUX schnell is 12B parameters (large model)
   - Reduce image size: 512x512 instead of 1024x1024
   - Use quantized model: `flux1-schnell.gguf` from `lmz/candle-flux`
   - Try CPU mode (very slow): `Device::Cpu`
