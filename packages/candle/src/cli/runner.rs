//! CLI runner - main orchestration for interactive chat application using fluent API
//!
//! This module demonstrates the canonical use of CandleFluentAi::agent_role() builder pattern
//! with proper defaults, smart input resolution, and all CLI flags wired to builder methods.

use anyhow::{Context, Result};

use super::args::CliArgs;
use super::handler::{InputHandler, InputHandlerResult, CommandResult};
use super::prompt::PromptBuilder;
use super::config::CliConfig;

use crate::builders::agent_role::{CandleFluentAi, CandleAgentRoleBuilder, CandleAgentBuilder};
use crate::domain::chat::CandleChatLoop;
use crate::domain::model::factory::ModelFactory;
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
        println!("Using model: {}", model);

        // Log warning if tools are specified (WASM loading not yet implemented)
        if !self.args.tools.is_empty() {
            eprintln!("⚠️  Warning: {} tool(s) specified but dynamic WASM loading not yet implemented:", self.args.tools.len());
            for tool in &self.args.tools {
                eprintln!("    - {}", tool);
            }
            eprintln!("    Tools will be available in a future release.");
        }

        // Display banner
        println!("\n=== Candle Chat CLI ===");
        println!("Agent Role: {}", self.args.agent_role);
        println!("Temperature: {}", self.args.temperature);
        println!("Type your message or use commands (/help for list)");
        println!("Use Ctrl+C to exit\n");

        // Resolve system prompt using smart input resolution
        let system_prompt = if let Some(ref prompt_input) = self.args.system_prompt {
            resolve_smart_input(prompt_input).await
                .context("Failed to resolve system prompt")?
        } else {
            // Use default architect prompt from task specification
            String::from(r#"# Well-Informed Software Architect

You think out loud as you work through problems, sharing your process in addition to the solutions.
You track every task you do or needs doing in `TODO.md`, updating it religiously before and after a meaningful change to code.
You maintain `ARCHITECTURE.md` and carefully curate the vision for the modules we create.
You prototype exploratory code ideas, quickly putting together a prototype, so we talk about the "heart of the matter" and get on the same page.
If you don't know the answer, you ALWAYS RESEARCH on the web and talk it through with me. You know that planned takes less time in the end that hastily forged.You never pretend to have answers unless you are highly confident.
You really LOVE programming and the art of it. You craft applications that are fast, efficient, and blazing fast.
You produce clean, maintainable, *production quality* code all the time.
You are a master at debugging and fixing bugs.
You are a master at refactoring code, remembering to check for code that ALREADY EXISTS before writing new code that might duplicate existing functionality."#)
        };

        // Initialize memory system with embedding model from CLI
        let memory_manager = {
            use crate::memory::vector::embedding_factory::EmbeddingModelFactory;
            use crate::domain::embedding::config::EmbeddingConfig;
            use crate::memory::core::manager::surreal::SurrealDBMemoryManager;
            use surrealdb::engine::any::connect;
            use std::sync::Arc;
            
            // Parse embedding model name (handle "stella 1024" format)
            let (model_name, dimensions) = if self.args.embedding_model.contains(' ') {
                let parts: Vec<&str> = self.args.embedding_model.split_whitespace().collect();
                let name = parts[0];
                let dims = parts.get(1).and_then(|d| d.parse::<usize>().ok());
                (name, dims)
            } else {
                (self.args.embedding_model.as_str(), None)
            };
            
            // Create embedding model using factory
            let embedding_config = if let Some(dims) = dimensions {
                EmbeddingConfig::default()
                    .with_model(model_name)
                    .with_dimensions(dims)
            } else {
                EmbeddingConfig::default().with_model(model_name)
            };
            
            let embedding_model = EmbeddingModelFactory::create_embedding_model(embedding_config)
                .await
                .context("Failed to create embedding model")?;
            
            // Connect to SurrealDB (in-memory for CLI)
            let db = connect("memory://")
                .await
                .context("Failed to connect to memory database")?;
            
            db.use_ns("candle").use_db("cli")
                .await
                .context("Failed to initialize database namespace")?;
            
            // Create memory manager with custom embedding model
            let manager = SurrealDBMemoryManager::with_embedding_model(db, embedding_model).await
                .context("Failed to create memory manager")?;
            
            manager.initialize().await
                .context("Failed to initialize memory tables")?;
            
            Arc::new(manager) as Arc<dyn crate::memory::core::manager::surreal::MemoryManager>
        };

        // Ingest documents into memory if provided
        if !self.args.documents.is_empty() {
            use crate::memory::primitives::node::MemoryNode;
            use crate::memory::primitives::types::{MemoryTypeEnum, MemoryContent};
            use chrono::Utc;
            
            println!("Loading {} document(s) into memory...", self.args.documents.len());
            
            for doc_path in &self.args.documents {
                // Resolve document content (file/URL/text)
                let content = resolve_smart_input(doc_path.to_str().unwrap_or(""))
                    .await
                    .unwrap_or_else(|e| {
                        eprintln!("⚠️  Failed to load document {:?}: {}", doc_path, e);
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
                    evaluation_status: crate::memory::monitoring::operations::OperationStatus::Pending,
                    metadata: crate::memory::primitives::metadata::MemoryMetadata::new(),
                    relevance_score: None,
                }
                .with_custom_metadata(
                    "source".to_string(),
                    serde_json::Value::String(doc_path.to_string_lossy().to_string())
                )
                .with_custom_metadata(
                    "content_hash".to_string(),
                    serde_json::Value::Number(content_hash.into())
                );
                
                // Ingest into memory
                match memory_manager.create_memory(memory).await {
                    Ok(_) => println!("✓ Loaded: {:?}", doc_path),
                    Err(e) => eprintln!("✗ Failed to ingest {:?}: {}", doc_path, e),
                }
            }
            
            println!();
        }

        // Load provider once
        let provider = ModelFactory::create_from_alias(&model)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to load model: {}", e))?;

        // Clone for use in closure
        let handler = self.handler.clone();
        let prompt_builder = self.prompt_builder.clone();

        // Build agent with on_conversation_turn for recursive loop
        let mut builder = CandleFluentAi::agent_role(&self.args.agent_role)
            .completion_provider(provider)
            .temperature(self.args.temperature)
            .system_prompt(system_prompt.clone())
            .memory(memory_manager.clone());

        if let Some(timeout) = self.args.memory_read_timeout {
            builder = builder.memory_read_timeout(timeout);
        }

        if let Some(max_tokens) = self.args.max_tokens {
            builder = builder.max_tokens(max_tokens);
        }

        // Configure recursive handler - THIS IS THE LOOP
        let builder = builder.on_conversation_turn(move |_conversation, agent| {
            let input = match prompt_builder.get_user_input("You: ") {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("Input error: {}", e);
                    return agent.chat(CandleChatLoop::Break);
                }
            };

            match handler.handle(&input) {
                InputHandlerResult::Exit => {
                    println!("Goodbye!");
                    agent.chat(CandleChatLoop::Break)
                }
                InputHandlerResult::Command(cmd_result) => {
                    let output = CliRunner::format_command_result(&cmd_result);
                    println!("{}", output);
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
        });

        // Start conversation
        let mut stream = builder
            .into_agent()
            .chat(|_| CandleChatLoop::UserPrompt("Ready to chat!".to_string()))?;

        // Consume stream - loop happens via on_conversation_turn recursion
        use futures::StreamExt;
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
                    println!();
                }
                CandleMessageChunk::Error(err) => {
                    eprintln!("\nError: {}", err);
                }
                CandleMessageChunk::ToolCallStart { name, .. } => {
                    println!("\n[Calling tool: {}]", name);
                }
                CandleMessageChunk::ToolCallComplete { name, .. } => {
                    println!("[Tool {} completed]", name);
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
        let model = self.prompt_builder.select_model(None)
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
        self.config.save(self.args.config.as_deref())
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
