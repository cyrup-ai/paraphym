# IMGGEN_4: Module Integration and Examples

## OBJECTIVE
Integrate image generation providers into paraphym's module system and create working examples demonstrating usage.

## PRIORITY
🔴 CRITICAL - Makes image generation accessible to users

## SUBTASK 1: Update Provider Module Exports

**What needs to change**:
- Export new image generation providers from providers module

**Where changes happen**:
- UPDATE: `packages/candle/src/providers/mod.rs`

**Implementation**:
```rust
// Existing text completion providers
pub mod kimi_k2;
pub mod qwen3_coder;
pub mod bert_embedding;
pub mod gte_qwen_embedding;
pub mod jina_bert_embedding;
pub mod nvembed_embedding;
pub mod stella_embedding;
pub mod tokenizer;

// NEW: Image generation providers
pub mod stable_diffusion_35_turbo;
pub mod flux_schnell;

// Re-export image generation providers
pub use stable_diffusion_35_turbo::StableDiffusion35Turbo;
pub use flux_schnell::FluxSchnell;
```

## SUBTASK 2: Update Library Exports

**What needs to change**:
- Re-export image generation types for public API

**Where changes happen**:
- UPDATE: `packages/candle/src/lib.rs`

**Implementation**:
```rust
// Existing exports (leave unchanged)
pub mod image;          // Image INPUT preprocessing
pub mod providers;
// ... other existing modules ...

// NEW: Re-export image generation types
pub use domain::image_generation::{
    ImageGenerationConfig,
    ImageGenerationChunk, 
    ImageGenerationModel,
    tensor_to_image,
};
```

## SUBTASK 3: Create SD3.5 Turbo Example

**What needs to change**:
- Create example demonstrating SD3.5 Turbo usage

**Where changes happen**:
- NEW: `examples/text_to_image_sd35.rs`

**Implementation**:
```rust
//! Example: Text-to-Image Generation with Stable Diffusion 3.5 Large Turbo
//!
//! Demonstrates 4-step image generation from text prompts.
//!
//! Usage:
//!   cargo run --example text_to_image_sd35 --features cuda --release

use paraphym_candle::{
    providers::StableDiffusion35Turbo,
    ImageGenerationConfig,
    ImageGenerationChunk,
    ImageGenerationModel,
    tensor_to_image,
};
use candle_core::Device;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Stable Diffusion 3.5 Large Turbo - Text to Image");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // 1. Setup device (prefer GPU)
    let device = Device::cuda_if_available(0)?;
    println!("📱 Device: {:?}", device);
    
    // 2. Load model
    println!("📥 Loading model from HuggingFace Hub...");
    let provider = StableDiffusion35Turbo::from_pretrained(&device)
        .map_err(|e| format!("Model load failed: {}", e))?;
    println!("✅ Model loaded: {}", provider.model_name());
    
    // 3. Configure generation
    let config = ImageGenerationConfig {
        width: 1024,
        height: 1024,
        steps: 4,
        guidance_scale: 3.5,
        negative_prompt: Some("blurry, low quality, distorted, ugly".to_string()),
        seed: Some(42),
        use_flash_attn: false,
    };
    
    let prompt = "a rusty robot holding a candle torch in a futuristic city, \
                  high quality, detailed, 4k";
    
    println!("\n📝 Prompt: {}", prompt);
    println!("⚙️  Config: {}x{}, {} steps, CFG {}", 
        config.width, config.height, config.steps, config.guidance_scale);
    println!("\n🔄 Generating...\n");
    
    // 4. Generate image with progress tracking
    let mut stream = provider.generate(prompt, &config, &device);
    
    let mut final_image = None;
    while let Some(chunk) = stream.next().await {
        match chunk {
            ImageGenerationChunk::Step { step, total, .. } => {
                let progress = ((step + 1) as f32 / total as f32 * 100.0) as u32;
                println!("   Step {}/{} [{}%]", step + 1, total, progress);
            }
            ImageGenerationChunk::Complete { image } => {
                final_image = Some(image);
                println!("\n✨ Generation complete!");
            }
            ImageGenerationChunk::Error(e) => {
                eprintln!("❌ Error: {}", e);
                return Err(e.into());
            }
        }
    }
    
    // 5. Save image
    if let Some(tensor) = final_image {
        let dynamic_image = tensor_to_image(&tensor)
            .map_err(|e| format!("Tensor conversion failed: {}", e))?;
        
        let output_path = "sd35_output.png";
        dynamic_image.save(output_path)?;
        
        println!("💾 Saved to: {}", output_path);
        println!("🎉 Done!");
    }
    
    Ok(())
}
```

## SUBTASK 4: Create FLUX Example

**What needs to change**:
- Create example demonstrating FLUX Schnell usage

**Where changes happen**:
- NEW: `examples/text_to_image_flux.rs`

**Implementation**:
```rust
//! Example: Text-to-Image Generation with FLUX.1-schnell
//!
//! Demonstrates fast single-step image generation.
//!
//! Usage:
//!   cargo run --example text_to_image_flux --features cuda --release

use paraphym_candle::{
    providers::FluxSchnell,
    ImageGenerationConfig,
    ImageGenerationChunk,
    ImageGenerationModel,
    tensor_to_image,
};
use candle_core::Device;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ FLUX.1-schnell - Fast Text to Image");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // 1. Setup device
    let device = Device::cuda_if_available(0)?;
    println!("📱 Device: {:?}", device);
    
    // 2. Load model
    println!("📥 Loading FLUX model from HuggingFace Hub...");
    let provider = FluxSchnell::from_pretrained(&device)
        .map_err(|e| format!("Model load failed: {}", e))?;
    println!("✅ Model loaded: {}", provider.model_name());
    
    // 3. Configure generation (FLUX uses guidance_scale = 0.0)
    let config = ImageGenerationConfig {
        width: 1024,
        height: 1024,
        steps: 1,  // FLUX is single-step
        guidance_scale: 0.0,  // No CFG for FLUX
        negative_prompt: None,
        seed: Some(123),
        use_flash_attn: false,
    };
    
    let prompt = "cyberpunk cityscape at sunset, neon lights, \
                  futuristic architecture, high detail";
    
    println!("\n📝 Prompt: {}", prompt);
    println!("⚙️  Config: {}x{}, single-step generation", 
        config.width, config.height);
    println!("\n⚡ Generating (fast)...\n");
    
    // 4. Generate image
    let mut stream = provider.generate(prompt, &config, &device);
    
    let mut final_image = None;
    while let Some(chunk) = stream.next().await {
        match chunk {
            ImageGenerationChunk::Step { step, total, .. } => {
                println!("   ⚡ Step {}/{} (FLUX single-step)", step + 1, total);
            }
            ImageGenerationChunk::Complete { image } => {
                final_image = Some(image);
                println!("\n✨ Generation complete!");
            }
            ImageGenerationChunk::Error(e) => {
                eprintln!("❌ Error: {}", e);
                return Err(e.into());
            }
        }
    }
    
    // 5. Save image
    if let Some(tensor) = final_image {
        let dynamic_image = tensor_to_image(&tensor)
            .map_err(|e| format!("Tensor conversion failed: {}", e))?;
        
        let output_path = "flux_output.png";
        dynamic_image.save(output_path)?;
        
        println!("💾 Saved to: {}", output_path);
        println!("🎉 Done!");
    }
    
    Ok(())
}
```

## SUBTASK 5: Update Domain Module

**What needs to change**:
- Ensure domain module exports image_generation

**Where changes happen**:
- UPDATE: `packages/candle/src/domain/mod.rs`

**Implementation**:
Add this line if not already present:
```rust
pub mod image_generation;  // NEW
```

## DEFINITION OF DONE

- [ ] `providers/mod.rs` exports stable_diffusion_35_turbo and flux_schnell
- [ ] `lib.rs` re-exports ImageGenerationConfig, ImageGenerationChunk, ImageGenerationModel, tensor_to_image
- [ ] `domain/mod.rs` exports image_generation module
- [ ] Example created: `examples/text_to_image_sd35.rs`
- [ ] Example created: `examples/text_to_image_flux.rs`
- [ ] Both examples compile without errors
- [ ] cargo check passes for entire workspace
- [ ] cargo build --example text_to_image_sd35 succeeds
- [ ] cargo build --example text_to_image_flux succeeds

## VERIFICATION COMMANDS

```bash
# Check module exports compile
cargo check -p paraphym_candle

# Build examples
cargo build --example text_to_image_sd35 --features cuda --release
cargo build --example text_to_image_flux --features cuda --release

# Run SD3.5 example (if GPU available)
cargo run --example text_to_image_sd35 --features cuda --release

# Run FLUX example (if GPU available)
cargo run --example text_to_image_flux --features cuda --release

# Verify outputs
ls -lh sd35_output.png flux_output.png
```

## SUCCESS CRITERIA

The implementation is complete when:

1. ✅ Module system properly exports all image generation types
2. ✅ Public API accessible via `use paraphym_candle::{ImageGenerationConfig, ...}`
3. ✅ SD3.5 example runs and generates valid 1024x1024 PNG
4. ✅ FLUX example runs and generates valid 1024x1024 PNG
5. ✅ No compilation errors or warnings
6. ✅ Examples demonstrate proper error handling
7. ✅ Examples show progress tracking during generation

## IMPORTANT CONSTRAINTS

- ❌ NO unit tests in examples (separate team)
- ❌ NO benchmarks (separate team)
- ❌ NO extensive documentation (examples are self-documenting)
- ✅ YES working code that demonstrates features
- ✅ YES proper error handling in examples
- ✅ YES clear console output showing progress

## TROUBLESHOOTING

If examples fail to run:

1. **Out of Memory**:
   - Reduce image size: 512x512 instead of 1024x1024
   - Enable flash attention: `config.use_flash_attn = true`
   - Use CPU mode: `Device::Cpu` (slower)

2. **Model Download Issues**:
   - Set HF token: `export HF_TOKEN=your_token`
   - Check internet connection
   - Verify HuggingFace model availability

3. **Compilation Errors**:
   - Ensure all previous tasks (IMGGEN_1, IMGGEN_2, IMGGEN_3) are complete
   - Run `cargo clean` and rebuild
   - Check feature flags: `--features cuda` or `--features metal`

## REFERENCES

**Pattern References**:
- Example structure: Look at other examples in `./examples/` directory
- Error handling: Follow paraphym conventions (Result, map_err)
- Output formatting: Use println! with clear progress indicators

**Module System**:
- Provider exports: `./packages/candle/src/providers/mod.rs` (existing pattern)
- Library exports: `./packages/candle/src/lib.rs` (existing pattern)
