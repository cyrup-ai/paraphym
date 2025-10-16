//! Example: Text-to-Image Generation with FLUX.1-schnell
//!
//! Demonstrates fast 4-step image generation with FLUX Schnell provider.
//!
//! Usage:
//!   cargo run --example text_to_image_flux --features cuda --release
//!   cargo run --example text_to_image_flux --features metal --release

use candle_core::Device;
use log::error;
use paraphym_candle::{
    FluxSchnell, ImageGenerationChunk, ImageGenerationConfig, ImageGenerationModel, tensor_to_image,
    StreamExt,
};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true))?;
    writeln!(&mut stdout, "⚡ FLUX.1-schnell - Fast Text to Image")?;
    writeln!(&mut stdout, "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")?;
    stdout.reset()?;

    // 1. Setup device
    let device = Device::cuda_if_available(0)?;
    writeln!(&mut stdout, "📱 Device: {:?}", device)?;

    // 2. Create provider
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    writeln!(&mut stdout, "📥 Creating FLUX provider...")?;
    stdout.reset()?;
    let provider = FluxSchnell::new();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(
        &mut stdout,
        "✅ Provider ready: {}",
        provider.registry_key()
    )?;
    stdout.reset()?;

    // 3. Configure generation (FLUX uses guidance_scale = 0.0)
    let config = ImageGenerationConfig {
        width: 1024,
        height: 1024,
        steps: 4,            // FLUX schnell is 4-step
        guidance_scale: 0.0, // No CFG for FLUX schnell
        negative_prompt: None,
        seed: Some(123),
        use_flash_attn: false,
    };

    let prompt = "cyberpunk cityscape at sunset, neon lights, \
                  futuristic architecture, high detail";

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
    writeln!(&mut stdout, "\n📝 Prompt: {}", prompt)?;
    writeln!(
        &mut stdout,
        "⚙️  Config: {}x{}, {} steps (fast)",
        config.width, config.height, config.steps
    )?;
    writeln!(&mut stdout, "\n⚡ Generating...\n")?;
    stdout.reset()?;

    // 4. Generate image
    let mut stream = provider.generate(prompt, &config, &device);

    let mut final_image = None;
    while let Some(chunk) = stream.next().await {
        match chunk {
            ImageGenerationChunk::Step { step, total, .. } => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
                writeln!(&mut stdout, "   ⚡ Step {}/{}", step + 1, total)?;
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
        let dynamic_image =
            tensor_to_image(&tensor).map_err(|e| format!("Tensor conversion failed: {}", e))?;

        let output_path = "flux_output.png";
        dynamic_image.save(output_path)?;

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        writeln!(&mut stdout, "💾 Saved to: {}", output_path)?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        writeln!(&mut stdout, "🎉 Done!")?;
        stdout.reset()?;
    }

    Ok(())
}
