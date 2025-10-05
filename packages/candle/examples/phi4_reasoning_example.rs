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
use paraphym_candle::model::phi4_reasoning::{ChatMessage, Phi4ReasoningModel};

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
    let args = Args::parse();

    println!("🔧 Initializing Phi-4-Reasoning model...");
    println!("Model: {}", args.model_path);
    println!("Tokenizer: {}", args.tokenizer_path);

    // Initialize device
    let device = if args.cpu {
        Device::Cpu
    } else {
        Device::cuda_if_available(0)?
    };
    println!("Device: {:?}", device);

    // Load model
    println!("\n📥 Loading model (this may take a moment)...");
    let mut model =
        Phi4ReasoningModel::load_from_gguf(&args.model_path, &args.tokenizer_path, &device)?;

    println!("✅ Model loaded successfully!");

    // Create chat messages
    let messages = vec![ChatMessage::user(&args.query)];

    println!("\n💬 User Query:");
    println!("{}", args.query);

    // Apply chat template
    println!("\n🔄 Applying chat template...");
    let prompt = model.apply_chat_template(&messages)?;

    // Generate response
    println!("\n🤔 Generating response with reasoning...");
    let response = model.generate(&prompt, args.max_tokens)?;

    // Extract reasoning and solution
    let (reasoning, solution) = model.extract_reasoning(&response);

    // Display results
    println!("\n{}", "=".repeat(80));
    if let Some(thinking) = reasoning {
        println!("🧠 REASONING PROCESS:");
        println!("{}", "─".repeat(80));
        println!("{}", thinking);
        println!("{}", "─".repeat(80));
    }

    println!("\n💡 SOLUTION:");
    println!("{}", "─".repeat(80));
    println!("{}", solution);
    println!("{}", "=".repeat(80));

    Ok(())
}
