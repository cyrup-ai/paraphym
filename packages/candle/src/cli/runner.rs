//! CLI runner - main orchestration for interactive chat application using fluent API
//!
//! This module demonstrates the canonical use of CandleFluentAi::agent_role() builder pattern
//! with proper defaults, smart input resolution, and all CLI flags wired to builder methods.

use anyhow::{Context, Result};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use super::args::CliArgs;
use super::config::CliConfig;
use super::handler::{CommandResult, InputHandler, InputHandlerResult};
use super::prompt::PromptBuilder;

use crate::builders::agent_role::{CandleAgentBuilder, CandleAgentRoleBuilder, CandleFluentAi};
use crate::domain::chat::CandleChatLoop;
use crate::util::input_resolver::resolve_input;

/// CLI runner for interactive chat
pub struct CliRunner {
    args: CliArgs,
    config: CliConfig,
    prompt_builder: PromptBuilder,
    handler: InputHandler,
}

impl CliRunner {
    /// Create new CLI runner from arguments
    pub fn new(args: CliArgs) -> Result<Self> {
        // Validate arguments
        args.validate()
            .map_err(|e| anyhow::anyhow!("Invalid arguments: {}", e))?;

        // Load or create config
        let config = if let Some(config_path) = &args.config {
            CliConfig::load(Some(config_path))
                .map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?
        } else {
            CliConfig::load(None)
                .map_err(|e| anyhow::anyhow!("Failed to load default config: {}", e))?
        };

        let handler = InputHandler::new(config.clone());

        Ok(Self {
            args,
            config,
            prompt_builder: PromptBuilder::new(),
            handler,
        })
    }

    /// Run the CLI application using fluent API
    pub async fn run(&mut self) -> Result<()> {
        // Initialize pool maintenance thread (lazy init)
        crate::pool::init_maintenance();

        // Setup Ctrl+C handler for graceful shutdown
        use std::sync::mpsc::channel;
        let (shutdown_tx, shutdown_rx) = channel();

        ctrlc::set_handler(move || {
            // Send shutdown signal (handler is called each time Ctrl+C pressed)
            if shutdown_tx.send(()).is_err() {
                // Channel closed - main thread already exiting
                eprintln!("Shutdown already in progress");
            }
        })
        .map_err(|e| anyhow::anyhow!("Failed to set Ctrl-C handler: {}", e))?;

        // Spawn shutdown monitor thread
        std::thread::spawn(move || {
            if shutdown_rx.recv().is_ok() {
                eprintln!("\nShutdown signal received, draining pools...");
                crate::pool::begin_shutdown(5); // 5 second timeout
                std::process::exit(0);
            }
        });

        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        // Log warning if tools are specified (WASM loading not yet implemented)
        if !self.args.tools.is_empty() {
            let mut stderr = StandardStream::stderr(ColorChoice::Always);
            stderr.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
            writeln!(
                &mut stderr,
                "⚠️  Warning: {} tool(s) specified but dynamic WASM loading not yet implemented:",
                self.args.tools.len()
            )?;
            for tool in &self.args.tools {
                writeln!(&mut stderr, "    - {}", tool)?;
            }
            writeln!(
                &mut stderr,
                "    Tools will be available in a future release."
            )?;
            stderr.reset()?;
        }

        // Display banner
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        writeln!(&mut stdout, "\n=== Candle Chat CLI ===")?;
        writeln!(&mut stdout, "Agent Role: {}", self.args.agent_role)?;
        writeln!(&mut stdout, "Temperature: {}", self.args.temperature)?;
        writeln!(
            &mut stdout,
            "Type your message or use commands (/help for list)"
        )?;
        writeln!(&mut stdout, "Use Ctrl+C to exit\n")?;
        stdout.reset()?;

        // Resolve system prompt using smart input resolution
        let system_prompt = if let Some(ref prompt_input) = self.args.system_prompt {
            resolve_input(prompt_input)
                .await
                .context("Failed to resolve system prompt")?
        } else {
            // Use default architect prompt from task specification
            String::from(
                r#"# Well-Informed Software Architect

You think out loud as you work through problems, sharing your process in addition to the solutions.
You track every task you do or needs doing in `TODO.md`, updating it religiously before and after a meaningful change to code.
You maintain `ARCHITECTURE.md` and carefully curate the vision for the modules we create.
You prototype exploratory code ideas, quickly putting together a prototype, so we talk about the "heart of the matter" and get on the same page.
If you don't know the answer, you ALWAYS RESEARCH on the web and talk it through with me. You know that planned takes less time in the end that hastily forged.You never pretend to have answers unless you are highly confident.
You really LOVE programming and the art of it. You craft applications that are fast, efficient, and blazing fast.
You produce clean, maintainable, *production quality* code all the time.
You are a master at debugging and fixing bugs.
You are a master at refactoring code, remembering to check for code that ALREADY EXISTS before writing new code that might duplicate existing functionality."#,
            )
        };

        // Clone for use in closure - wrap in Arc<Mutex<>> for interior mutability
        use std::sync::Arc;
        use std::sync::Mutex;
        let handler = Arc::new(Mutex::new(self.handler.clone()));
        let prompt_builder = self.prompt_builder.clone();

        // Build agent - use agent_role defaults, set properties, optionally override model
        let agent_builder = CandleFluentAi::agent_role(&self.args.agent_role).into_agent();

        let agent = if let Some(registry_key) = &self.args.model {
            use crate::capability::registry::{self, TextToTextModel};

            let text_model = registry::get::<TextToTextModel>(registry_key)
                .ok_or_else(|| anyhow::anyhow!("Model not found in registry: {}", registry_key))?;

            agent_builder
                .model(text_model)
                .temperature(self.args.temperature)
                .system_prompt(system_prompt.clone())
                .memory_read_timeout(self.args.memory_read_timeout)
                .max_tokens(self.args.max_tokens.unwrap_or(2000))
        } else {
            agent_builder
                .temperature(self.args.temperature)
                .system_prompt(system_prompt.clone())
                .memory_read_timeout(self.args.memory_read_timeout)
                .max_tokens(self.args.max_tokens.unwrap_or(2000))
        };

        let agent = agent.on_conversation_turn(
            move |_conversation: &crate::domain::agent::role::CandleAgentConversation, agent| {
                let input = match prompt_builder.get_user_input("You: ") {
                    Ok(i) => i,
                    Err(e) => {
                        let mut stderr = StandardStream::stderr(ColorChoice::Always);
                        let _ = stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
                        let _ = writeln!(&mut stderr, "Input error: {}", e);
                        let _ = stderr.reset();
                        return agent.chat(CandleChatLoop::Break);
                    }
                };

                let handler_result = match handler.lock() {
                    Ok(mut guard) => guard.handle(&input),
                    Err(poisoned) => {
                        // Mutex poisoned - recover data and continue
                        let mut stderr = StandardStream::stderr(ColorChoice::Always);
                        let _ = stderr.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
                        let _ = writeln!(&mut stderr, "⚠️  Handler mutex poisoned, recovering...");
                        let _ = stderr.reset();
                        poisoned.into_inner().handle(&input)
                    }
                };

                match handler_result {
                    InputHandlerResult::Exit => {
                        let mut stdout = StandardStream::stdout(ColorChoice::Always);
                        let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
                        let _ = writeln!(&mut stdout, "Goodbye!");
                        let _ = stdout.reset();
                        agent.chat(CandleChatLoop::Break)
                    }
                    InputHandlerResult::Command(cmd_result) => {
                        let output = CliRunner::format_command_result(&cmd_result);
                        let mut stdout = StandardStream::stdout(ColorChoice::Always);
                        let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
                        let _ = writeln!(&mut stdout, "{}", output);
                        let _ = stdout.reset();
                        agent.chat(CandleChatLoop::UserPrompt("".to_string()))
                    }
                    InputHandlerResult::None => {
                        agent.chat(CandleChatLoop::UserPrompt("".to_string()))
                    }
                    InputHandlerResult::Chat(message) => {
                        use ystream::{AsyncStream, emit, spawn_task};

                        let agent_clone = agent.clone();

                        AsyncStream::with_channel(move |sender| {
                            spawn_task(async move || {
                                // Async resolve without blocking
                                let resolved =
                                    resolve_input(&message).await.unwrap_or(message.clone());

                                // Get chat stream with resolved input
                                let chat_stream =
                                    agent_clone.chat(CandleChatLoop::UserPrompt(resolved));

                                // Forward all chunks
                                while let Some(chunk) = chat_stream.try_next() {
                                    emit!(sender, chunk);
                                }
                            });
                        })
                    }
                }
            },
        );

        // Start conversation
        let stream = agent.chat(|_| CandleChatLoop::UserPrompt("Ready to chat!".to_string()))?;

        // Consume stream - loop happens via on_conversation_turn recursion
        print!("Assistant: ");
        while let Some(chunk) = stream.next().await {
            use crate::domain::chat::message::CandleMessageChunk;
            match chunk {
                CandleMessageChunk::Text(text) => {
                    print!("{}", text);
                    std::io::Write::flush(&mut std::io::stdout()).ok();
                }
                CandleMessageChunk::Complete {
                    text,
                    token_count,
                    elapsed_secs,
                    tokens_per_sec,
                    ..
                } => {
                    if !text.is_empty() {
                        print!("{}", text);
                    }

                    let mut stdout = StandardStream::stdout(ColorChoice::Always);
                    let _ = writeln!(&mut stdout);

                    // Display metrics from library if available
                    if let (Some(tc), Some(es), Some(tps)) =
                        (token_count, elapsed_secs, tokens_per_sec)
                    {
                        let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
                        let _ = writeln!(
                            &mut stdout,
                            "✓ {} tokens in {:.2}s ({:.1} tokens/sec)",
                            tc, es, tps
                        );
                        let _ = stdout.reset();
                    }
                }
                CandleMessageChunk::Error(err) => {
                    let mut stderr = StandardStream::stderr(ColorChoice::Always);
                    let _ = stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
                    let _ = writeln!(&mut stderr, "\nError: {}", err);
                    let _ = stderr.reset();
                }
                CandleMessageChunk::ToolCallStart { name, .. } => {
                    let mut stdout = StandardStream::stdout(ColorChoice::Always);
                    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
                    let _ = writeln!(&mut stdout, "\n[Calling tool: {}]", name);
                    let _ = stdout.reset();
                }
                CandleMessageChunk::ToolCallComplete { name, .. } => {
                    let mut stdout = StandardStream::stdout(ColorChoice::Always);
                    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
                    let _ = writeln!(&mut stdout, "[Tool {} completed]", name);
                    let _ = stdout.reset();
                }
                _ => {}
            }
        }

        // Save config on exit
        self.save_config()?;

        Ok(())
    }

    /// Format command result for display
    fn format_command_result(result: &CommandResult) -> String {
        match result {
            CommandResult::Help(text) => text.clone(),
            CommandResult::SavedToFile(path) => format!("Saved to: {}", path),
            CommandResult::SearchResults(results) => {
                if results.is_empty() {
                    "No results found".to_string()
                } else {
                    format!("Found {} results:\n{}", results.len(), results.join("\n"))
                }
            }
            CommandResult::ConfigChanged(msg) => msg.clone(),
            CommandResult::HistoryCleared => "History cleared".to_string(),
            CommandResult::Error(err) => format!("Error: {}", err),
        }
    }

    /// Save config to disk
    fn save_config(&self) -> Result<()> {
        self.config
            .save(self.args.config.as_deref())
            .map_err(|e| anyhow::anyhow!("Failed to save config: {}", e))
    }
}

impl Default for CliRunner {
    fn default() -> Self {
        let config = CliConfig::default();
        let handler = InputHandler::new(config.clone());

        Self {
            args: CliArgs::default(),
            config,
            prompt_builder: PromptBuilder::default(),
            handler,
        }
    }
}
