//! Phi-4-Reasoning Q4_K_M Example
//!
//! This example demonstrates how to use the Phi-4-reasoning model
//! with integrated chain-of-thought reasoning capabilities using the
//! TextToTextCapable trait and streaming API.
//!
//! # Usage
//!
//! ```bash
//! # The model will be automatically downloaded from HuggingFace on first run
//! cargo run --example phi4_reasoning_example --release
//! ```

use clap::Parser;
use paraphym_candle::{
    capability::text_to_text::phi4_reasoning::CandlePhi4ReasoningModel,
    capability::traits::TextToTextCapable,
    domain::{
        completion::CandleCompletionParams, context::chunk::CandleCompletionChunk,
        prompt::CandlePrompt,
    },
    StreamExt,
};
use std::io::Write;
use std::num::NonZeroU64;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// User query to process
    #[arg(long, default_value = "If x + 5 = 12, what is x?")]
    query: String,

    /// Maximum tokens to generate
    #[arg(long, default_value_t = 2048)]
    max_tokens: u64,

    /// Temperature (0.0-2.0, default 0.7)
    #[arg(long, default_value_t = 0.7)]
    temperature: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = Args::parse();

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
    writeln!(&mut stdout, "🔧 Initializing Phi-4-Reasoning model...")?;
    stdout.reset()?;

    // Create the model (downloads happen lazily during first generation)
    let model = CandlePhi4ReasoningModel::new();

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(&mut stdout, "✅ Model initialized!")?;
    stdout.reset()?;

    // Create prompt
    let prompt = CandlePrompt::new(&args.query);

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true))?;
    writeln!(&mut stdout, "\n💬 User Query:")?;
    stdout.reset()?;
    writeln!(&mut stdout, "{}", args.query)?;

    // Create completion parameters
    let params = CandleCompletionParams::new()
        .with_max_tokens(NonZeroU64::new(args.max_tokens))
        .with_temperature(args.temperature)
        .map_err(|e| format!("Invalid temperature: {}", e))?;

    // Generate response
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    writeln!(&mut stdout, "\n🤔 Generating response with reasoning...\n")?;
    stdout.reset()?;

    let mut stream = model.prompt(prompt, &params);
    let mut full_response = String::new();

    while let Some(chunk) = stream.next().await {
        match chunk {
            CandleCompletionChunk::Text(text) => {
                full_response.push_str(&text);
            }
            CandleCompletionChunk::Complete { text, .. } => {
                full_response.push_str(&text);
                break;
            }
            CandleCompletionChunk::Error(e) => {
                eprintln!("\n❌ Error: {}", e);
                return Err(e.into());
            }
            _ => {}
        }
    }

    // Extract and clean reasoning and solution
    let (reasoning, solution) = extract_reasoning(&full_response);

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

/// Extract reasoning (in <think> tags) and solution from response
fn extract_reasoning(response: &str) -> (Option<String>, String) {
    // Find <think>...</think> section
    if let Some(think_start) = response.find("<think>")
        && let Some(think_end) = response.find("</think>")
    {
        let reasoning = response[think_start + 7..think_end].trim().to_string();
        let solution = response[think_end + 8..].trim().to_string();
        return (Some(reasoning), solution);
    }

    // No thinking tags found - entire response is solution
    (None, response.trim().to_string())
}
