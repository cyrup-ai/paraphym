//! Phi-4-Reasoning Q4_K_M Example
//!
//! This example demonstrates how to use the Phi-4-reasoning model
//! with integrated chain-of-thought reasoning capabilities.
//!
//! # Usage
//!
//! ```bash
//! # Download the model first:
//! # huggingface-cli download unsloth/Phi-4-reasoning-GGUF phi-4-reasoning-Q4_K_M.gguf
//!
//! cargo run --example phi4_reasoning_example --release -- \
//!   --model-path /path/to/phi-4-reasoning-Q4_K_M.gguf \
//!   --tokenizer-path /path/to/tokenizer.json
//! ```

use anyhow::Result;
use candle_core::Device;
use clap::Parser;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use paraphym_candle::capability::text_to_text::phi4_reasoning::{ChatMessage, Phi4ReasoningModel};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the GGUF model file
    #[arg(long)]
    model_path: String,

    /// Path to the tokenizer file
    #[arg(long)]
    tokenizer_path: String,

    /// User query to process
    #[arg(long, default_value = "If x + 5 = 12, what is x?")]
    query: String,

    /// Maximum tokens to generate
    #[arg(long, default_value_t = 2048)]
    max_tokens: usize,

    /// Run on CPU instead of GPU
    #[arg(long)]
    cpu: bool,
}

fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = Args::parse();

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
    writeln!(&mut stdout, "🔧 Initializing Phi-4-Reasoning model...")?;
    stdout.reset()?;
    writeln!(&mut stdout, "Model: {}", args.model_path)?;
    writeln!(&mut stdout, "Tokenizer: {}", args.tokenizer_path)?;

    // Initialize device
    let device = if args.cpu {
        Device::Cpu
    } else {
        Device::cuda_if_available(0)?
    };
    writeln!(&mut stdout, "Device: {:?}", device)?;

    // Load model
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    writeln!(&mut stdout, "\n📥 Loading model (this may take a moment)...")?;
    stdout.reset()?;
    let mut model =
        Phi4ReasoningModel::load_from_gguf(&args.model_path, &args.tokenizer_path, &device)?;

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(&mut stdout, "✅ Model loaded successfully!")?;
    stdout.reset()?;

    // Create chat messages
    let messages = vec![ChatMessage::user(&args.query)];

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true))?;
    writeln!(&mut stdout, "\n💬 User Query:")?;
    stdout.reset()?;
    writeln!(&mut stdout, "{}", args.query)?;

    // Apply chat template
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    writeln!(&mut stdout, "\n🔄 Applying chat template...")?;
    stdout.reset()?;
    let prompt = model.apply_chat_template(&messages)?;

    // Generate response
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    writeln!(&mut stdout, "\n🤔 Generating response with reasoning...")?;
    stdout.reset()?;
    let response = model.generate(&prompt, args.max_tokens)?;

    // Extract reasoning and solution
    let (reasoning, solution) = model.extract_reasoning(&response);

    // Display results
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true))?;
    writeln!(&mut stdout, "\n{}", "=".repeat(80))?;
    if let Some(thinking) = reasoning {
        writeln!(&mut stdout, "🧠 REASONING PROCESS:")?;
        stdout.reset()?;
        writeln!(&mut stdout, "{}", "─".repeat(80))?;
        writeln!(&mut stdout, "{}", thinking)?;
        writeln!(&mut stdout, "{}", "─".repeat(80))?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true))?;
    }

    writeln!(&mut stdout, "\n💡 SOLUTION:")?;
    stdout.reset()?;
    writeln!(&mut stdout, "{}", "─".repeat(80))?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(&mut stdout, "{}", solution)?;
    stdout.reset()?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true))?;
    writeln!(&mut stdout, "{}", "=".repeat(80))?;
    stdout.reset()?;

    Ok(())
}
