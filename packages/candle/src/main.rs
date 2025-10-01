use std::io::{self, Write};
use clap::Parser;

// Initialize Rustls crypto provider for HTTPS connections
use rustls::crypto::aws_lc_rs;

use paraphym_candle::providers::{kimi_k2::CandleKimiK2Provider, qwen3_coder::CandleQwen3CoderProvider};

use paraphym_candle::{
    builders::{CandleFluentAi, CandleAgentRoleBuilder, CandleAgentBuilder},
    domain::{
        chat::CandleChatLoop,
    },
    CandleMessageChunk,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The model to use for inference
    #[arg(short, long, default_value = "kimi-k2")]
    model: String,

    /// Temperature for sampling (0.0 to 2.0)
    #[arg(short, long, default_value = "0.7")]
    temperature: f64,

    /// Maximum tokens to generate
    #[arg(long, default_value = "2000")]
    max_tokens: u64,

    /// System prompt to use
    #[arg(
        long,
        default_value = "You are a helpful AI assistant using the Candle ML framework for local inference."
    )]
    system_prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Rustls crypto provider for TLS/HTTPS connections
    // This must happen before any HTTPS connections are made
    // APPROVED BY DAVID MAPLE 09/30/2025: Panic is appropriate for crypto provider initialization failure
    aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    // Initialize Candle performance optimizations
    paraphym_candle::init_candle();

    let args = Args::parse();

    println!(
        "🤖 Starting Candle Agent Chat Completion with model: {}",
        args.model
    );
    println!("📦 Downloading model and initializing provider...");

    // Create the provider based on the model argument
    match args.model.as_str() {
        "kimi-k2" => {
            println!("🚀 Initializing Kimi-K2 provider with ProgressHub...");
            let provider = CandleKimiK2Provider::new()
                .await
                .map_err(|e| format!("Failed to create Kimi-K2 provider: {}", e))?;
            run_chat(
                provider,
                args.temperature,
                args.max_tokens,
                &args.system_prompt,
            )
            .await
        }
        "qwen-coder" => {
            println!("🚀 Initializing Qwen3-Coder provider with ProgressHub...");
            let provider = CandleQwen3CoderProvider::new()
                .await
                .map_err(|e| format!("Failed to create Qwen3-Coder provider: {}", e))?;
            run_chat_qwen(
                provider,
                args.temperature,
                args.max_tokens,
                &args.system_prompt,
            )
            .await
        }
        _ => {
            return Err(format!(
                "Unknown model: {}. Available models: kimi-k2, qwen-coder",
                args.model
            )
            .into());
        }
    }
}

async fn run_chat(
    provider: CandleKimiK2Provider,
    temperature: f64,
    max_tokens: u64,
    system_prompt: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("✅ Provider ready! Starting chat...");

    // Use the beautiful fluent API
    let _stream = CandleFluentAi::agent_role("helpful-assistant")
        .completion_provider(provider)
        .temperature(temperature)
        .max_tokens(max_tokens)
        .system_prompt(system_prompt)
        .on_chunk(|chunk| {
            // Real-time streaming - print each token as it arrives
            match &chunk {
                CandleMessageChunk::Text(text) => print!("{}", text),
                CandleMessageChunk::Complete { text, .. } => print!("{}", text),
                other => print!("{:?}", other),
            }
            if let Err(e) = io::stdout().flush() {
                eprintln!("Warning: Failed to flush stdout: {}", e);
            }
            chunk
        })
        .into_agent()
        .chat(|conversation| {
            let user_input = conversation.latest_user_message();

            match user_input.to_lowercase().as_str() {
                "quit" | "exit" | "bye" => {
                    println!("\n👋 Goodbye!");
                    CandleChatLoop::Break
                },
                _ => {
                    CandleChatLoop::Reprompt("Hello, can you help me understand how the Candle ML framework works with the Kimi-K2 model?".to_string())
                }
            }
        })?;

    Ok(())
}

async fn run_chat_qwen(
    provider: CandleQwen3CoderProvider,
    temperature: f64,
    max_tokens: u64,
    system_prompt: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("✅ Provider ready! Starting chat...");

    // Use the same fluent API as KimiK2
    let _stream = CandleFluentAi::agent_role("helpful-coder")
        .completion_provider(provider)
        .temperature(temperature)
        .max_tokens(max_tokens)
        .system_prompt(system_prompt)
        .on_chunk(|chunk| {
            match &chunk {
                CandleMessageChunk::Text(text) => print!("{}", text),
                CandleMessageChunk::Complete { text, .. } => print!("{}", text),
                other => print!("{:?}", other),
            }
            if let Err(e) = io::stdout().flush() {
                eprintln!("Warning: Failed to flush stdout: {}", e);
            }
            chunk
        })
        .into_agent()
        .chat(|conversation| {
            let user_input = conversation.latest_user_message();
            match user_input.to_lowercase().as_str() {
                "quit" | "exit" | "bye" => {
                    println!("\n👋 Goodbye!");
                    CandleChatLoop::Break
                },
                _ => {
                    CandleChatLoop::Reprompt("Hello, can you help me with coding tasks using Qwen3-Coder?".to_string())
                }
            }
        })?;

    Ok(())
}
