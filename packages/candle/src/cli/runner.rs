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
use crate::capability::text_to_text::{
    CandleKimiK2Model, CandlePhi4ReasoningModel, CandleQwen3CoderModel,
};
use crate::capability::traits::TextToTextCapable;
use crate::domain::model::traits::CandleModel;
use crate::domain::chat::CandleChatLoop;
use crate::util::input_resolver::resolve_smart_input;

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
        // Step 1: Model selection (prompt if not provided)
        let model = self.select_model()?;
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        writeln!(&mut stdout, "Using model: {}", model)?;
        stdout.reset()?;

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
            resolve_smart_input(prompt_input)
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

        // Initialize memory system with embedding model from CLI
        let memory_manager = {
            use crate::capability::registry;
            use crate::memory::core::manager::surreal::SurrealDBMemoryManager;
            use std::sync::Arc;
            use surrealdb::engine::any::connect;

            // Connect to SurrealDB (in-memory for CLI)
            let db = connect("memory://")
                .await
                .context("Failed to connect to memory database")?;

            db.use_ns("candle")
                .use_db("cli")
                .await
                .context("Failed to initialize database namespace")?;

            // Map CLI arg to registry key
            let registry_key = match self.args.embedding_model.to_lowercase().as_str() {
                "bert" => "bert-base-uncased",
                "stella" => "dunzhang/stella_en_1.5B_v5",
                "gte-qwen" => "Alibaba-NLP/gte-Qwen2-1.5B-instruct",
                "jina-bert" => "jinaai/jina-embeddings-v2-base-en",
                "nvembed" => "nvidia/NV-Embed-v2",
                _ => {
                    // Default to Stella if unrecognized
                    log::warn!("Unknown embedding model '{}', using Stella as default", self.args.embedding_model);
                    "dunzhang/stella_en_1.5B_v5"
                }
            };

            // Get model from registry - THE ONLY SOURCE OF TRUTH
            // Use generic get<T>() to return concrete TextEmbeddingModel type
            let embedding_model: crate::capability::registry::TextEmbeddingModel = registry::get(registry_key)
                .ok_or_else(|| anyhow::anyhow!("Embedding model '{}' not found in registry", registry_key))?;

            // Create memory manager with registry model
            let manager = SurrealDBMemoryManager::with_embedding_model(db, embedding_model)
                .await
                .context("Failed to create memory manager")?;
            
            manager
                .initialize()
                .await
                .context("Failed to initialize memory tables")?;
            
            Arc::new(manager) as Arc<dyn crate::memory::core::manager::surreal::MemoryManager>
        };

        // Ingest documents into memory if provided
        if !self.args.documents.is_empty() {
            use crate::memory::primitives::node::MemoryNode;
            use crate::memory::primitives::types::{MemoryContent, MemoryTypeEnum};
            use chrono::Utc;

            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
            writeln!(
                &mut stdout,
                "Loading {} document(s) into memory...",
                self.args.documents.len()
            )?;
            stdout.reset()?;

            for doc_path in &self.args.documents {
                // Resolve document content (file/URL/text)
                let content = resolve_smart_input(doc_path.to_str().unwrap_or(""))
                    .await
                    .unwrap_or_else(|e| {
                        let mut stderr = StandardStream::stderr(ColorChoice::Always);
                        let _ = stderr.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
                        let _ = writeln!(
                            &mut stderr,
                            "⚠️  Failed to load document {:?}: {}",
                            doc_path, e
                        );
                        let _ = stderr.reset();
                        String::new()
                    });

                if content.is_empty() {
                    continue;
                }

                // Calculate content hash for deduplication
                let content_hash = crate::domain::memory::serialization::content_hash(&content);

                // Create memory node with proper timestamps
                let memory = MemoryNode {
                    id: format!("doc_{}", content_hash),
                    content: MemoryContent::new(&content),
                    content_hash,
                    memory_type: MemoryTypeEnum::Semantic,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    embedding: None, // Will be auto-generated by memory manager
                    evaluation_status:
                        crate::memory::monitoring::operations::OperationStatus::Pending,
                    metadata: crate::memory::primitives::metadata::MemoryMetadata::new(),
                    relevance_score: None,
                }
                .with_custom_metadata(
                    "source".to_string(),
                    serde_json::Value::String(doc_path.to_string_lossy().to_string()),
                )
                .with_custom_metadata(
                    "content_hash".to_string(),
                    serde_json::Value::Number(content_hash.into()),
                );

                // Ingest into memory
                match memory_manager.create_memory(memory).await {
                    Ok(_) => {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                        writeln!(&mut stdout, "✓ Loaded: {:?}", doc_path)?;
                        stdout.reset()?;
                    }
                    Err(e) => {
                        let mut stderr = StandardStream::stderr(ColorChoice::Always);
                        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                        writeln!(&mut stderr, "✗ Failed to ingest {:?}: {}", doc_path, e)?;
                        stderr.reset()?;
                    }
                }
            }

            writeln!(&mut stdout)?;
        }

        // Load model from registry - use concrete TextToTextModel enum
        use crate::capability::registry::TextToTextModel;
        use std::sync::Arc;
        
        let text_model: TextToTextModel = match model.to_lowercase().as_str() {
            "phi4" | "phi-4" | "phi4-reasoning" => {
                TextToTextModel::Phi4Reasoning(Arc::new(
                    CandlePhi4ReasoningModel::default_for_builder()
                        .map_err(|e| anyhow::anyhow!("Failed to load Phi-4-Reasoning model: {}", e))?,
                ))
            }
            "kimi" | "kimi-k2" => {
                TextToTextModel::KimiK2(Arc::new(
                    CandleKimiK2Model::default_for_builder()
                        .map_err(|e| anyhow::anyhow!("Failed to load Kimi-K2 model: {}", e))?,
                ))
            }
            "qwen" | "qwen3" | "qwen3-coder" => {
                TextToTextModel::Qwen3Coder(Arc::new(
                    CandleQwen3CoderModel::new()
                        .await
                        .map_err(|e| anyhow::anyhow!("Failed to load Qwen3-Coder model: {}", e))?,
                ))
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Unknown model alias: {}. Supported: phi4, kimi, qwen3",
                    model
                ));
            }
        };

        // Clone for use in closure - wrap in Arc<Mutex<>> for interior mutability
        use std::sync::Mutex;
        let handler = Arc::new(Mutex::new(self.handler.clone()));
        let prompt_builder = self.prompt_builder.clone();

        // Build agent with on_conversation_turn for recursive loop
        // Note: on_conversation_turn must be called immediately after .model()
        // before other trait methods that return opaque types
        let agent_with_provider =
            CandleFluentAi::agent_role(&self.args.agent_role).model(text_model);

        let agent = agent_with_provider
            .on_conversation_turn(
                move |_conversation: &crate::domain::agent::role::CandleAgentConversation,
                      agent| {
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
                            let _ =
                                writeln!(&mut stderr, "⚠️  Handler mutex poisoned, recovering...");
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
                            // Resolve with smart input detection
                            let resolved = tokio::task::block_in_place(|| {
                                tokio::runtime::Handle::current().block_on(async {
                                    resolve_smart_input(&message).await.unwrap_or(message)
                                })
                            });
                            agent.chat(CandleChatLoop::UserPrompt(resolved))
                        }
                    }
                },
            )
            .temperature(self.args.temperature)
            .system_prompt(system_prompt.clone())
            .memory(memory_manager.clone())
            .memory_read_timeout(self.args.memory_read_timeout)
            .max_tokens(self.args.max_tokens.unwrap_or(2000));

        // Start conversation
        let stream = agent
            .into_agent()
            .chat(|_| CandleChatLoop::UserPrompt("Ready to chat!".to_string()))?;

        // Consume stream - loop happens via on_conversation_turn recursion
        print!("Assistant: ");
        while let Some(chunk) = stream.next().await {
            use crate::domain::chat::message::CandleMessageChunk;
            match chunk {
                CandleMessageChunk::Text(text) => {
                    print!("{}", text);
                    std::io::Write::flush(&mut std::io::stdout()).ok();
                }
                CandleMessageChunk::Complete { text, .. } => {
                    if !text.is_empty() {
                        print!("{}", text);
                    }
                    let mut stdout = StandardStream::stdout(ColorChoice::Always);
                    let _ = writeln!(&mut stdout);
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

    /// Select model from CLI args, config, or interactive prompt
    fn select_model(&mut self) -> Result<String> {
        // Use arg if provided
        if let Some(model) = &self.args.model {
            self.config.set_last_model(model.clone());
            return Ok(model.clone());
        }

        // Use last model from config
        if let Some(last_model) = self.config.get_last_model() {
            let use_last = self
                .prompt_builder
                .confirm(&format!("Use last model '{}'?", last_model))
                .unwrap_or(true);

            if use_last {
                return Ok(last_model.to_string());
            }
        }

        // Prompt for selection
        let model = self
            .prompt_builder
            .select_model(None)
            .map_err(|e| anyhow::anyhow!("Model selection failed: {}", e))?;
        self.config.set_last_model(model.clone());

        Ok(model)
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
