use std::io::{self, Write};
use std::sync::Arc;
use clap::Parser;
use log::{error, info};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// Initialize Rustls crypto provider for HTTPS connections
use rustls::crypto::aws_lc_rs;

use paraphym_candle::capability::text_to_text::{CandleKimiK2Provider, CandleQwen3CoderProvider};

use paraphym_candle::{
    builders::{CandleFluentAi, CandleAgentRoleBuilder, CandleAgentBuilder},
    domain::{
        chat::CandleChatLoop,
    },
    memory::{
        core::{
            manager::surreal::{MemoryManager, SurrealDBMemoryManager},
        },
        utils::config::MemoryConfig,
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

/// Initialize the memory manager with kv-surrealkv persistence
async fn init_memory_manager() -> Result<Arc<dyn MemoryManager>, Box<dyn std::error::Error>> {
    // Create memory config with kv-surrealkv persistence
    let mut config = MemoryConfig::default();
    config.database.connection_string = "surrealkv://./data/agent_memory.db".to_string();

    // Initialize the memory manager with kv-surrealkv backend
    let manager = SurrealDBMemoryManager::with_config(config).await?;

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
    writeln!(&mut stdout, "📚 Memory system initialized with kv-surrealkv persistence at ./data/agent_memory.db")?;
    stdout.reset()?;

    Ok(Arc::new(manager) as Arc<dyn MemoryManager>)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize env_logger for application logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Application starting");

    // Initialize Rustls crypto provider for TLS/HTTPS connections
    // This must happen before any HTTPS connections are made
    aws_lc_rs::default_provider()
        .install_default()
        .map_err(|e| format!("Failed to install rustls crypto provider: {:?}", e))?;

    // Initialize Candle performance optimizations
    paraphym_candle::init_candle();

    let args = Args::parse();

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(
        &mut stdout,
        "🤖 Starting Candle Agent Chat Completion with model: {}",
        args.model
    )?;
    stdout.reset()?;

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
    writeln!(&mut stdout, "📦 Downloading model and initializing provider...")?;
    stdout.reset()?;

    // Create the provider based on the model argument
    match args.model.as_str() {
        "kimi-k2" => {
            let mut stdout = StandardStream::stdout(ColorChoice::Auto);
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
            writeln!(&mut stdout, "🚀 Initializing Kimi-K2 provider with ProgressHub...")?;
            stdout.reset()?;
            
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
            let mut stdout = StandardStream::stdout(ColorChoice::Auto);
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
            writeln!(&mut stdout, "🚀 Initializing Qwen3-Coder provider with ProgressHub...")?;
            stdout.reset()?;
            
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
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(&mut stdout, "✅ Provider ready! Starting chat...")?;
    stdout.reset()?;

    // Initialize memory manager with kv-surrealkv persistence
    let memory_manager = init_memory_manager().await?;

    // Use the beautiful fluent API with memory support
    let _stream = CandleFluentAi::agent_role("helpful-assistant")
        .completion_provider(provider)
        .temperature(temperature)
        .max_tokens(max_tokens)
        .system_prompt(system_prompt)
        .memory(memory_manager)
        .on_chunk(|chunk| {
            // Real-time streaming - print each token as it arrives
            match &chunk {
                CandleMessageChunk::Text(text) => print!("{}", text),
                CandleMessageChunk::Complete { text, .. } => print!("{}", text),
                other => print!("{:?}", other),
            }
            if let Err(e) = io::stdout().flush() {
                error!("Failed to flush stdout: {}", e);
            }
            chunk
        })
        .into_agent()
        .chat(|conversation| {
            let user_input = conversation.latest_user_message();

            match user_input.to_lowercase().as_str() {
                "quit" | "exit" | "bye" => {
                    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
                    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
                    let _ = writeln!(&mut stdout, "\n👋 Goodbye!");
                    let _ = stdout.reset();
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
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(&mut stdout, "✅ Provider ready! Starting chat...")?;
    stdout.reset()?;

    // Initialize memory manager with kv-surrealkv persistence
    let memory_manager = init_memory_manager().await?;

    // Use the same fluent API as KimiK2 with memory support
    let _stream = CandleFluentAi::agent_role("helpful-coder")
        .completion_provider(provider)
        .temperature(temperature)
        .max_tokens(max_tokens)
        .system_prompt(system_prompt)
        .memory(memory_manager)
        .on_chunk(|chunk| {
            match &chunk {
                CandleMessageChunk::Text(text) => print!("{}", text),
                CandleMessageChunk::Complete { text, .. } => print!("{}", text),
                other => print!("{:?}", other),
            }
            if let Err(e) = io::stdout().flush() {
                error!("Failed to flush stdout: {}", e);
            }
            chunk
        })
        .into_agent()
        .chat(|conversation| {
            let user_input = conversation.latest_user_message();
            match user_input.to_lowercase().as_str() {
                "quit" | "exit" | "bye" => {
                    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
                    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
                    let _ = writeln!(&mut stdout, "\n👋 Goodbye!");
                    let _ = stdout.reset();
                    CandleChatLoop::Break
                },
                _ => {
                    CandleChatLoop::Reprompt("Hello, can you help me with coding tasks using Qwen3-Coder?".to_string())
                }
            }
        })?;

    Ok(())
}
