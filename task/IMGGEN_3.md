# IMGGEN_3: FLUX.1-schnell Provider

## OBJECTIVE
Implement FLUX.1-schnell provider for fast single-step text-to-image generation with T5-XXL + CLIP text encoding.

## PRIORITY
🔴 CRITICAL - Fast alternative image generation model

## SUBTASK 1: Create Provider Structure

**What needs to change**:
- Create FLUX provider file following SD3.5 pattern
- Use BF16 dtype (FLUX preference)

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
    vae: flux::AutoEncoderKL,
    config: FluxConfig,
}

#[derive(Debug, Clone)]
pub struct FluxConfig {
    pub guidance_scale: f64,  // FLUX uses 0.0 (no CFG)
    pub use_bf16: bool,       // Prefer bf16 for FLUX
}

impl Default for FluxConfig {
    fn default() -> Self {
        Self {
            guidance_scale: 0.0,  // FLUX doesn't use CFG
            use_bf16: true,
        }
    }
}
```

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
        
        // 2. Load T5-XXL text encoder
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
        let dtype = if device.is_cuda() || device.is_metal() {
            DType::BF16  // GPU supports bf16
        } else {
            DType::F32   // CPU fallback
        };
        
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
        let clip_encoder = clip::text_model::ClipTextTransformer::new(clip_vb, &clip_config)
            .map_err(|e| format!("CLIP encoder load failed: {}", e))?;
        
        // 7. Load FLUX transformer
        let flux_vb = unsafe {
            candle_nn::VarBuilder::from_mmaped_safetensors(
                &[flux_model_file], 
                dtype, 
                device
            ).map_err(|e| format!("FLUX VarBuilder failed: {}", e))?
        };
        let flux_transformer = flux::model::Flux::new(&flux::model::Config::schnell(), flux_vb)
            .map_err(|e| format!("FLUX model load failed: {}", e))?;
        
        // 8. Load VAE
        let vae = flux::AutoEncoderKL::new(device)
            .map_err(|e| format!("VAE load failed: {}", e))?;
        
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
        // 1. T5-XXL encoding (longer context)
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
        
        // Pad/truncate to 256 tokens (FLUX requirement)
        t5_tokens.resize(256, 0);
        
        let t5_input = Tensor::new(&t5_tokens[..], device)
            .map_err(|e| format!("T5 tensor creation failed: {}", e))?
            .unsqueeze(0)
            .map_err(|e| format!("T5 unsqueeze failed: {}", e))?;
        
        let t5_emb = self.t5_encoder.forward(&t5_input)
            .map_err(|e| format!("T5 forward failed: {}", e))?;
        
        // 2. CLIP encoding (for pooled embeddings)
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

## SUBTASK 4: Implement Single-Step Generation

**FLUX uses single-step generation (schnell variant)**

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
        let latent = flux::sampling::get_noise(
            1, 
            config.height, 
            config.width, 
            device
        ).map_err(|e| format!("Noise generation failed: {}", e))?;
        
        // 2. Single-step diffusion (FLUX schnell)
        // Note: FLUX uses guidance_scale = 0.0 (no CFG)
        let img_emb = self.flux_transformer.forward(
            &latent,
            t5_emb,
            clip_emb,
            &Tensor::new(&[1.0f32], device)
                .map_err(|e| format!("Timestep tensor failed: {}", e))?,
        ).map_err(|e| format!("FLUX forward failed: {}", e))?;
        
        // 3. Send progress (step 0 of 1)
        let _ = sender.send(ImageGenerationChunk::Step {
            step: 0,
            total: 1,
            latent: img_emb.clone(),
        });
        
        // 4. VAE decode
        let image = self.vae.decode(&img_emb)
            .map_err(|e| format!("VAE decode failed: {}", e))?;
        
        // 5. Post-process: clamp to [0, 1]
        let image = image.clamp(0.0, 1.0)
            .map_err(|e| format!("Clamp failed: {}", e))?;
        
        Ok(image)
    }
}
```

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
            
            // 2. Encode text prompt
            let (t5_emb, clip_emb) = match self.encode_prompt(prompt, device) {
                Ok(result) => result,
                Err(e) => {
                    let _ = sender.send(ImageGenerationChunk::Error(e));
                    return;
                }
            };
            
            // 3. Generate image (single-step)
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
        1  // FLUX schnell is single-step
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
- [ ] generate_image() implements single-step generation
- [ ] ImageGenerationModel trait fully implemented
- [ ] All errors use Result and map_err() with context
- [ ] No unwrap() or expect() in code
- [ ] cargo check passes

## REFERENCES

**Primary Implementation Reference**:
- FLUX example: `./tmp/candle-examples/candle-examples/examples/flux/main.rs`
- Model config: Look for `Config::schnell()` in candle-transformers
- Sampling: FLUX uses single-step, no Euler loop needed

**Pattern Reference**:
- Provider structure: `./packages/candle/src/providers/stable_diffusion_35_turbo.rs` (just created)
- AsyncStream pattern: `./packages/candle/src/providers/kimi_k2.rs`

## KEY DIFFERENCES FROM SD3.5

1. **Text Encoding**: T5-XXL (256 tokens) + CLIP instead of triple CLIP
2. **Guidance**: No CFG (guidance_scale = 0.0)
3. **Steps**: Single-step generation (fast)
4. **Dtype**: Prefers BF16 over F16
5. **Sampling**: Direct forward pass, no Euler loop

## IMPORTANT CONSTRAINTS

- ❌ NO unit tests (separate team)
- ❌ NO benchmarks (separate team)
- ❌ NO examples in this file (next task)
- ✅ YES comprehensive error handling
- ✅ YES single-step streaming (step 0 of 1)
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
```
