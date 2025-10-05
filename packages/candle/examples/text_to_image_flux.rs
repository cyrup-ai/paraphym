//! Example: Text-to-Image Generation with FLUX.1-schnell
//!
//! Demonstrates fast 4-step image generation with FLUX Schnell provider.
//!
//! Usage:
//!   cargo run --example text_to_image_flux --features cuda --release
//!   cargo run --example text_to_image_flux --features metal --release

use paraphym_candle::{
    FluxSchnell,
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
    let provider = FluxSchnell::from_pretrained()
        .map_err(|e| format!("Model load failed: {}", e))?;
    println!("✅ Model loaded: {}", provider.model_name());
    
    // 3. Configure generation (FLUX uses guidance_scale = 0.0)
    let config = ImageGenerationConfig {
        width: 1024,
        height: 1024,
        steps: 4,  // FLUX schnell is 4-step
        guidance_scale: 0.0,  // No CFG for FLUX schnell
        negative_prompt: None,
        seed: Some(123),
        use_flash_attn: false,
    };
    
    let prompt = "cyberpunk cityscape at sunset, neon lights, \
                  futuristic architecture, high detail";
    
    println!("\n📝 Prompt: {}", prompt);
    println!("⚙️  Config: {}x{}, {} steps (fast)", 
        config.width, config.height, config.steps);
    println!("\n⚡ Generating...\n");
    
    // 4. Generate image
    let stream = provider.generate(prompt, &config, &device);
    
    let mut final_image = None;
    while let Some(chunk) = stream.next().await {
        match chunk {
            ImageGenerationChunk::Step { step, total, .. } => {
                println!("   ⚡ Step {}/{}", step + 1, total);
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
