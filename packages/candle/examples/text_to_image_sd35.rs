//! Example: Text-to-Image Generation with Stable Diffusion 3.5 Large Turbo
//!
//! Demonstrates 4-step image generation from text prompts using the SD3.5 provider.
//!
//! Usage:
//!   cargo run --example text_to_image_sd35 --features cuda --release
//!   cargo run --example text_to_image_sd35 --features metal --release

use paraphym_candle::{
    StableDiffusion35Turbo,
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
    let provider = StableDiffusion35Turbo::from_pretrained()
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
