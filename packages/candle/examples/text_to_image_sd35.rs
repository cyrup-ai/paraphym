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
use log::error;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true))?;
    writeln!(&mut stdout, "🎨 Stable Diffusion 3.5 Large Turbo - Text to Image")?;
    writeln!(&mut stdout, "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")?;
    stdout.reset()?;
    
    // 1. Setup device (prefer GPU)
    let device = Device::cuda_if_available(0)?;
    writeln!(&mut stdout, "📱 Device: {:?}", device)?;
    
    // 2. Load model
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    writeln!(&mut stdout, "📥 Loading model from HuggingFace Hub...")?;
    stdout.reset()?;
    let provider = StableDiffusion35Turbo::from_pretrained()
        .map_err(|e| format!("Model load failed: {}", e))?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(&mut stdout, "✅ Model loaded: {}", provider.model_name())?;
    stdout.reset()?;
    
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
    
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
    writeln!(&mut stdout, "\n📝 Prompt: {}", prompt)?;
    writeln!(&mut stdout, "⚙️  Config: {}x{}, {} steps, CFG {}", 
        config.width, config.height, config.steps, config.guidance_scale)?;
    writeln!(&mut stdout, "\n🔄 Generating...\n")?;
    stdout.reset()?;
    
    // 4. Generate image with progress tracking
    let stream = provider.generate(prompt, &config, &device);
    
    let mut final_image = None;
    while let Some(chunk) = stream.next().await {
        match chunk {
            ImageGenerationChunk::Step { step, total, .. } => {
                let progress = ((step + 1) as f32 / total as f32 * 100.0) as u32;
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
                writeln!(&mut stdout, "   Step {}/{} [{}%]", step + 1, total, progress)?;
                stdout.reset()?;
            }
            ImageGenerationChunk::Complete { image } => {
                final_image = Some(image);
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                writeln!(&mut stdout, "\n✨ Generation complete!")?;
                stdout.reset()?;
            }
            ImageGenerationChunk::Error(e) => {
                error!("❌ Error: {}", e);
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
        
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        writeln!(&mut stdout, "💾 Saved to: {}", output_path)?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        writeln!(&mut stdout, "🎉 Done!")?;
        stdout.reset()?;
    }
    
    Ok(())
}
