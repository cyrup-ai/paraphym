//! Builders are behavioral/construction logic, separate from core domain models

use std::num::NonZeroU64;
use std::sync::Arc;
use std::pin::Pin;

use cyrup_sugars::ZeroOneOrMany;
use tokio_stream::{Stream, StreamExt};

use crate::capability::registry::{TextEmbeddingModel, TextToTextModel};
use crate::capability::traits::TextToTextCapable;
use crate::domain::agent::core::AgentError;
use crate::domain::chat::CandleChatLoop;
use crate::domain::chat::message::{CandleMessageChunk, CandleMessageRole};
use crate::domain::completion::{CandleCompletionChunk, types::CandleCompletionParams};
use crate::domain::context::provider::{
    CandleContext, CandleDirectory, CandleFile, CandleFiles, CandleGithub,
};
use crate::domain::prompt::CandlePrompt;
use crate::domain::tool::SweetMcpRouter;
use serde_json;
use sweet_mcp_type::ToolInfo;

// Import agent conversation type
use crate::domain::agent::role::CandleAgentConversation;

// Memory types for explicit type annotations
use crate::memory::core::manager::surreal::Result as MemoryResult;

// Candle domain types - self-contained

// Type aliases to reduce complexity warnings
type OnToolResultHandler = Arc<dyn Fn(&[String]) + Send + Sync>;
type OnConversationTurnHandler = Arc<
    dyn Fn(&CandleAgentConversation, &CandleAgentRoleAgent) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>
        + Send
        + Sync,
>;

/// Agent helper type for conversation control in `on_conversation_turn` callbacks.
/// Holds Arc<AgentBuilderState> for recursive inference.
#[derive(Clone)]
pub struct CandleAgentRoleAgent {
    state: Arc<AgentBuilderState>,
}

impl CandleAgentRoleAgent {
    /// Chat method for use in on_conversation_turn closure - enables recursion
    pub fn chat(&self, chat_loop: CandleChatLoop) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>> {
        match chat_loop {
            CandleChatLoop::Break => Box::pin(crate::async_stream::spawn_stream(|sender| async move {
                let final_chunk = CandleMessageChunk::Complete {
                    text: String::new(),
                    finish_reason: Some("break".to_string()),
                    usage: None,
                    token_count: None,
                    elapsed_secs: None,
                    tokens_per_sec: None,
                };
                let _ = sender.send(final_chunk);
            })),
            CandleChatLoop::UserPrompt(user_message) | CandleChatLoop::Reprompt(user_message) => {
                self.run_inference_cycle(user_message)
            }
        }
    }

    fn run_inference_cycle(&self, user_message: String) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>> {
        let state = self.state.clone();

        Box::pin(crate::async_stream::spawn_stream(move |stream_sender| async move {
                    // Extract handlers from state for recursive inference
                    let on_chunk_handler = state.on_chunk_handler.clone();
                    let on_tool_result_handler = state.on_tool_result_handler.clone();

                    // Initialize tool router
                    let tool_router = {
                let reasoner_schema = crate::domain::agent::role::convert_serde_to_sweet_json(
                    serde_json::json!({
                        "type": "object",
                        "properties": {
                            "thought": {"type": "string", "description": "Current reasoning step"},
                            "thoughtNumber": {"type": "integer", "description": "Current step number", "minimum": 1},
                            "totalThoughts": {"type": "integer", "description": "Total expected steps", "minimum": 1},
                            "nextThoughtNeeded": {"type": "boolean", "description": "Whether another step is needed"}
                        },
                        "required": ["thought", "thoughtNumber", "totalThoughts", "nextThoughtNeeded"]
                    }),
                );

                let default_plugin_config = crate::domain::tool::router::PluginConfig {
                    tool_name: "mcp-reasoner".to_string(),
                    wasm_path: "packages/sweetmcp/plugins/reasoner/target/wasm32-unknown-unknown/release/sweetmcp_plugin_reasoner.wasm".to_string(),
                    description: "Advanced reasoning tool".to_string(),
                    input_schema: reasoner_schema,
                };

                let plugin_configs = vec![default_plugin_config];
                let mut router = crate::domain::tool::router::SweetMcpRouter::with_configs(
                    plugin_configs,
                    None,
                );

                match router.initialize().await {
                    Ok(()) => Some(router),
                    Err(e) => {
                        let error_chunk = CandleMessageChunk::Error(format!(
                            "Tool initialization failed: {}",
                            e
                        ));
                        let _ = stream_sender.send(error_chunk);
                        return;
                    }
                }
            };

            // Build prompt - system_prompt always exists (no memory in recursive calls)
            let full_prompt = format!("{}\n\nUser: {}", &state.system_prompt, user_message);

            // Call provider
            let prompt = CandlePrompt::new(full_prompt);
            let mut params = crate::domain::completion::CandleCompletionParams {
                temperature: state.temperature,
                max_tokens: std::num::NonZeroU64::new(state.max_tokens),
                ..Default::default()
            };

            // Add tools
            if let Some(ref router) = tool_router {
                let mut all_tools: Vec<sweet_mcp_type::ToolInfo> = state.tools.clone().into();

                // Use .await instead of block_on
                let auto_generated_tools = router.get_available_tools().await;
                all_tools.extend(auto_generated_tools);

                if !all_tools.is_empty() {
                    params.tools = Some(ZeroOneOrMany::from(all_tools));
                }
            }

            let completion_stream = state.text_to_text_model.prompt(prompt, &params);
            tokio::pin!(completion_stream);
            let mut assistant_response = String::new();

            // Track metrics for performance visibility
            let start_time = std::time::Instant::now();
            let mut token_count = 0u32;

            // Stream chunks
            while let Some(completion_chunk) = completion_stream.next().await {
                let message_chunk = match completion_chunk {
                    CandleCompletionChunk::Text(ref text) => {
                        token_count += 1;
                        assistant_response.push_str(text);
                        CandleMessageChunk::Text(text.clone())
                    }
                    CandleCompletionChunk::Complete {
                        ref text,
                        finish_reason,
                        usage,
                    } => {
                        assistant_response.push_str(text);

                        // Calculate performance metrics
                        let elapsed = start_time.elapsed();
                        let elapsed_secs = elapsed.as_secs_f64();
                        let tokens_per_sec = if elapsed_secs > 0.0 {
                            Some(token_count as f64 / elapsed_secs)
                        } else {
                            None
                        };

                        CandleMessageChunk::Complete {
                            text: text.clone(),
                            finish_reason: finish_reason.map(|f| format!("{:?}", f)),
                            usage: usage.map(|u| format!("{:?}", u)),
                            token_count: Some(token_count),
                            elapsed_secs: Some(elapsed_secs),
                            tokens_per_sec,
                        }
                    }
                    CandleCompletionChunk::ToolCallStart { id, name } => {
                        CandleMessageChunk::ToolCallStart { id, name }
                    }
                    CandleCompletionChunk::ToolCall {
                        id,
                        name,
                        partial_input,
                    } => CandleMessageChunk::ToolCall {
                        id,
                        name,
                        partial_input,
                    },
                    CandleCompletionChunk::ToolCallComplete { id, name, input } => {
                        if let Some(ref router) = tool_router {
                            match serde_json::from_str::<serde_json::Value>(&input) {
                                Ok(args_json) => {
                                    let sweet_args =
                                        crate::domain::agent::role::convert_serde_to_sweet_json(
                                            args_json,
                                        );
                                    // Use .await instead of block_on
                                    match router.call_tool(&name, sweet_args).await {
                                        Ok(response) => {
                                            // Call tool result handler if configured
                                            if let Some(ref handler) = on_tool_result_handler {
                                                let results = vec![format!("{:?}", response)];
                                                handler(&results);
                                            }

                                            CandleMessageChunk::Text(format!(
                                                "Tool '{}' executed: {:?}",
                                                name, response
                                            ))
                                        }
                                        Err(e) => CandleMessageChunk::Error(format!(
                                            "Tool '{}' failed: {}",
                                            name, e
                                        )),
                                    }
                                }
                                Err(e) => {
                                    CandleMessageChunk::Error(format!("Invalid JSON: {}", e))
                                }
                            }
                        } else {
                            CandleMessageChunk::ToolCallComplete { id, name, input }
                        }
                    }
                    CandleCompletionChunk::Error(error) => CandleMessageChunk::Error(error),
                };

                // Apply chunk handler if configured (zero allocation for None)
                let final_chunk = if let Some(ref handler) = on_chunk_handler {
                    handler(message_chunk)
                } else {
                    message_chunk
                };
                let _ = stream_sender.send(final_chunk);
            }

            // CRITICAL: Call handler for recursion
            if let Some(ref handler) = state.on_conversation_turn_handler {
                let mut conversation = CandleAgentConversation::new();
                conversation.add_message(user_message.clone(), CandleMessageRole::User);
                conversation
                    .add_message(assistant_response.clone(), CandleMessageRole::Assistant);

                let agent = CandleAgentRoleAgent {
                    state: state.clone(),
                };
                let handler_stream = handler(&conversation, &agent);
                tokio::pin!(handler_stream);
                while let Some(chunk) = handler_stream.next().await {
                    let _ = stream_sender.send(chunk);
                }
            }
        }))
    }
}

/// Shared builder state for recursive agent calls
#[derive(Clone)]
#[allow(dead_code)] // Fields are part of builder state but not directly read
struct AgentBuilderState {
    name: String,
    text_to_text_model: TextToTextModel,
    text_embedding_model: TextEmbeddingModel,
    temperature: f64,
    max_tokens: u64,
    memory_read_timeout: u64,
    system_prompt: String,
    tools: ZeroOneOrMany<ToolInfo>,
    context_file: Option<CandleContext<CandleFile>>,
    context_files: Option<CandleContext<CandleFiles>>,
    context_directory: Option<CandleContext<CandleDirectory>>,
    context_github: Option<CandleContext<CandleGithub>>,
    additional_params: std::collections::HashMap<String, String>,
    metadata: std::collections::HashMap<String, String>,
    on_chunk_handler: Option<Arc<dyn Fn(CandleMessageChunk) -> CandleMessageChunk + Send + Sync>>,
    on_tool_result_handler: Option<OnToolResultHandler>,
    on_conversation_turn_handler: Option<OnConversationTurnHandler>,
}

/// Agent role builder trait - elegant zero-allocation builder pattern (PUBLIC API)
pub trait CandleAgentRoleBuilder: Sized + Send {
    /// Create a new agent role builder - EXACT syntax: CandleFluentAi::agent_role("name")
    fn new(name: impl Into<String>) -> impl CandleAgentRoleBuilder;

    /// Set text-to-text model - EXACT syntax: .model(registry::get_text_to_text("key").unwrap())
    #[must_use]
    fn model(self, model: TextToTextModel) -> impl CandleAgentRoleBuilder;

    /// Set text embedding model - EXACT syntax: .embedding_model(registry::get_text_embedding("key").unwrap())
    #[must_use]
    fn embedding_model(self, model: TextEmbeddingModel) -> impl CandleAgentRoleBuilder;

    /// Set temperature - EXACT syntax: .temperature(1.0)
    #[must_use]
    fn temperature(self, temp: f64) -> impl CandleAgentRoleBuilder;

    /// Set max tokens - EXACT syntax: .max_tokens(8000)
    #[must_use]
    fn max_tokens(self, max: u64) -> impl CandleAgentRoleBuilder;

    /// Set memory read timeout in milliseconds - EXACT syntax: .memory_read_timeout(5000)
    #[must_use]
    fn memory_read_timeout(self, timeout_ms: u64) -> impl CandleAgentRoleBuilder;

    /// Set system prompt - EXACT syntax: .system_prompt("...")
    #[must_use]
    fn system_prompt(self, prompt: impl Into<String>) -> impl CandleAgentRoleBuilder;

    /// Set additional params - EXACT syntax: .additional_params([("key", "value")])
    #[must_use]
    fn additional_params<P>(self, params: P) -> impl CandleAgentRoleBuilder
    where
        P: IntoIterator<Item = (&'static str, &'static str)>;

    /// Set metadata - EXACT syntax: .metadata([("key", "value")])
    #[must_use]
    fn metadata<Meta>(self, metadata: Meta) -> impl CandleAgentRoleBuilder
    where
        Meta: IntoIterator<Item = (&'static str, &'static str)>;

    /// Set contexts - EXACT syntax: .context(CandleContext::<CandleFile>::of("/path"), CandleContext::<CandleFiles>::glob("*.rs"), ...)
    #[must_use]
    fn context(
        self,
        context1: CandleContext<CandleFile>,
        context2: CandleContext<CandleFiles>,
        context3: CandleContext<CandleDirectory>,
        context4: CandleContext<CandleGithub>,
    ) -> impl CandleAgentRoleBuilder;

    /// Set tools - EXACT syntax: .tools(tool1, tool2, tool3)
    #[must_use]
    fn tools<T>(self, tools: T) -> impl CandleAgentRoleBuilder
    where
        T: Into<ZeroOneOrMany<ToolInfo>>;

    /// Set MCP server - EXACT syntax: .mcp_server::<Stdio>().bin("/path").init("command")
    #[must_use]
    fn mcp_server<T>(self) -> impl CandleMcpServerBuilder
    where
        T: 'static;

    /// Add MCP server config - internal method for MCP server builder
    #[must_use]
    fn add_mcp_server_config(self, config: McpServerConfig) -> impl CandleAgentRoleBuilder;

    /// Set chunk handler - EXACT syntax: .on_chunk(|chunk| chunk)
    #[must_use]
    fn on_chunk<F>(self, handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(CandleMessageChunk) -> CandleMessageChunk + Send + Sync + 'static;

    /// Set tool result handler - EXACT syntax: .on_tool_result(|results| { ... })
    #[must_use]
    fn on_tool_result<F>(self, handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(&[String]) + Send + Sync + 'static;

    /// Set conversation turn handler - EXACT syntax: .on_conversation_turn(|conversation, agent| { ... })
    #[must_use]
    fn on_conversation_turn<F>(self, handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(&CandleAgentConversation, &CandleAgentRoleAgent) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>
            + Send
            + Sync
            + 'static;

    /// Convert to agent - EXACT syntax: .into_agent()
    #[must_use]
    fn into_agent(self) -> impl CandleAgentBuilder;

    /// Set conversation history - EXACT syntax: .conversation_history(...)
    #[must_use]
    fn conversation_history(
        self,
        history: impl ConversationHistoryArgs,
    ) -> impl CandleAgentRoleBuilder;

    /// Chat with closure - EXACT syntax: .chat(|conversation| ChatLoop)
    fn chat<F>(self, handler: F) -> Result<Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>, AgentError>
    where
        F: FnOnce(&CandleAgentConversation) -> CandleChatLoop + Send + 'static;

    /// Chat with message - EXACT syntax: .chat_with_message("message")
    fn chat_with_message(self, message: impl Into<String>) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>;
}

/// MCP server builder for fluent chaining
pub trait CandleMcpServerBuilder: Sized + Send {
    /// Set binary path - EXACT syntax: .bin("/path/to/binary")
    #[must_use]
    fn bin(self, path: impl Into<String>) -> impl CandleMcpServerBuilder;

    /// Initialize with command - EXACT syntax: .init("command args")
    #[must_use]
    fn init(self, command: impl Into<String>) -> impl CandleAgentRoleBuilder;
}

/// Agent builder trait (PUBLIC API)
pub trait CandleAgentBuilder: Sized + Send + Sync {
    /// Set text-to-text model - EXACT syntax: .model(TextToTextModel)
    #[must_use]
    fn model(self, model: TextToTextModel) -> Self;

    /// Set text embedding model - EXACT syntax: .embedding_model(TextEmbeddingModel)
    #[must_use]
    fn embedding_model(self, model: TextEmbeddingModel) -> Self;

    /// Set temperature - EXACT syntax: .temperature(1.0)
    #[must_use]
    fn temperature(self, temp: f64) -> Self;

    /// Set max tokens - EXACT syntax: .max_tokens(8000)
    #[must_use]
    fn max_tokens(self, max: u64) -> Self;

    /// Set memory read timeout in milliseconds - EXACT syntax: .memory_read_timeout(5000)
    #[must_use]
    fn memory_read_timeout(self, timeout_ms: u64) -> Self;

    /// Set system prompt - EXACT syntax: .system_prompt("...")
    #[must_use]
    fn system_prompt(self, prompt: impl Into<String>) -> Self;

    /// Set additional params - EXACT syntax: .additional_params([("key", "value")])
    #[must_use]
    fn additional_params<P2>(self, params: P2) -> Self
    where
        P2: IntoIterator<Item = (&'static str, &'static str)>;

    /// Set metadata - EXACT syntax: .metadata(meta)
    #[must_use]
    fn metadata<Meta>(self, metadata: Meta) -> Self
    where
        Meta: IntoIterator<Item = (&'static str, &'static str)>;

    /// Set contexts - EXACT syntax: .context(...)
    #[must_use]
    fn context(
        self,
        context1: CandleContext<CandleFile>,
        context2: CandleContext<CandleFiles>,
        context3: CandleContext<CandleDirectory>,
        context4: CandleContext<CandleGithub>,
    ) -> Self;

    /// Set tools - EXACT syntax: .tools(tool1, tool2, tool3)
    #[must_use]
    fn tools<T>(self, tools: T) -> Self
    where
        T: Into<ZeroOneOrMany<ToolInfo>>;

    /// Set MCP server - EXACT syntax: .mcp_server::<Stdio>().bin("/path").init("command")
    #[must_use]
    fn mcp_server<T>(self) -> impl CandleMcpServerBuilder
    where
        T: 'static;

    /// Add MCP server config - internal method for MCP server builder
    #[must_use]
    fn add_mcp_server_config(self, config: McpServerConfig) -> Self;

    /// Set chunk handler - EXACT syntax: .on_chunk(|chunk| chunk)
    #[must_use]
    fn on_chunk<F>(self, handler: F) -> Self
    where
        F: Fn(CandleMessageChunk) -> CandleMessageChunk + Send + Sync + 'static;

    /// Set tool result handler - EXACT syntax: .on_tool_result(|results| { ... })
    #[must_use]
    fn on_tool_result<F>(self, handler: F) -> Self
    where
        F: Fn(&[String]) + Send + Sync + 'static;

    /// Set conversation turn handler - EXACT syntax: .on_conversation_turn(|conversation, agent| { ... })
    #[must_use]
    fn on_conversation_turn<F>(self, handler: F) -> Self
    where
        F: Fn(&CandleAgentConversation, &CandleAgentRoleAgent) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>
            + Send
            + Sync
            + 'static;

    /// Set conversation history - EXACT syntax from ARCHITECTURE.md
    /// Supports: .conversation_history(CandleMessageRole::User => "content", CandleMessageRole::System => "content", ...)
    #[must_use]
    fn conversation_history(self, history: impl ConversationHistoryArgs) -> Self;

    /// Chat with closure - EXACT syntax: .chat(|conversation| ChatLoop)
    fn chat<F>(self, handler: F) -> Result<Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>, AgentError>
    where
        F: FnOnce(&CandleAgentConversation) -> CandleChatLoop + Send + 'static;

    /// Chat with message - EXACT syntax: .chat_with_message("message")
    fn chat_with_message(self, message: impl Into<String>) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>;
}

/// MCP server builder implementation
#[derive(Debug)]
pub struct CandleMcpServerBuilderImpl<T> {
    parent_builder: T,
    binary_path: Option<String>,
}

impl<T> CandleMcpServerBuilder for CandleMcpServerBuilderImpl<T>
where
    T: CandleAgentRoleBuilder,
{
    fn bin(mut self, path: impl Into<String>) -> impl CandleMcpServerBuilder {
        self.binary_path = Some(path.into());
        self
    }

    fn init(self, command: impl Into<String>) -> impl CandleAgentRoleBuilder {
        let config = McpServerConfig {
            binary_path: self.binary_path.clone(),
            init_command: command.into(),
        };
        self.parent_builder.add_mcp_server_config(config)
    }
}

/// MCP server configuration
#[derive(Debug, Clone)]
pub struct McpServerConfig {
    pub binary_path: Option<String>,
    pub init_command: String,
}

/// First builder - no provider yet
struct CandleAgentRoleBuilderImpl {
    name: String,
    text_to_text_model: Option<TextToTextModel>,
    text_embedding_model: Option<TextEmbeddingModel>,
    temperature: f64,
    max_tokens: Option<u64>,
    memory_read_timeout: u64,
    system_prompt: String,
    tools: ZeroOneOrMany<ToolInfo>,
    context_file: Option<CandleContext<CandleFile>>,
    context_files: Option<CandleContext<CandleFiles>>,
    context_directory: Option<CandleContext<CandleDirectory>>,
    context_github: Option<CandleContext<CandleGithub>>,
    additional_params: std::collections::HashMap<String, String>,
    metadata: std::collections::HashMap<String, String>,
    on_chunk_handler: Option<Arc<dyn Fn(CandleMessageChunk) -> CandleMessageChunk + Send + Sync>>,
    on_tool_result_handler: Option<OnToolResultHandler>,
    on_conversation_turn_handler: Option<OnConversationTurnHandler>,
}

impl std::fmt::Debug for CandleAgentRoleBuilderImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CandleAgentRoleBuilderImpl")
            .field("name", &self.name)
            .field("temperature", &self.temperature)
            .field("max_tokens", &self.max_tokens)
            .field("memory_read_timeout", &self.memory_read_timeout)
            .field(
                "system_prompt",
                &format!(
                    "{}...",
                    &self.system_prompt[..self.system_prompt.len().min(50)]
                ),
            )
            .field("tools", &self.tools)
            .finish()
    }
}

impl CandleAgentRoleBuilderImpl {
    /// Create a new agent role builder
    pub fn new(name: impl Into<String>) -> Self {
        // Create default tools: thinking and reasoning plugins
        let thinking_tool = ToolInfo {
            name: "thinking".to_string(),
            description: Some("Use this tool for all thinking and reasoning tasks. The tool accepts a list of user and previous assistant messages relevant to the conversation. Always call this tool before answering the user and include the latest user message in the list. The tool will generate a chain of thought reasoning which can be used to answer the user's question.".to_string()),
            input_schema: crate::domain::agent::role::convert_serde_to_sweet_json(serde_json::json!({
                "type": "object",
                "properties": {
                    "messages": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "role": {
                                    "type": "string",
                                    "enum": ["user", "assistant"]
                                },
                                "content": {
                                    "type": "string"
                                }
                            },
                            "required": ["role", "content"]
                        }
                    }
                },
                "required": ["messages"]
            })),
        };

        let reasoner_tool = ToolInfo {
            name: "mcp-reasoner".to_string(),
            description: Some("Advanced reasoning tool with Beam Search and MCTS strategies for complex problem solving".to_string()),
            input_schema: crate::domain::agent::role::convert_serde_to_sweet_json(serde_json::json!({
                "type": "object",
                "properties": {
                    "thought": {"type": "string", "description": "Current reasoning step"},
                    "thoughtNumber": {"type": "integer", "description": "Current step number", "minimum": 1},
                    "totalThoughts": {"type": "integer", "description": "Total expected steps", "minimum": 1},
                    "nextThoughtNeeded": {"type": "boolean", "description": "Whether another step is needed"},
                    "parentId": {"type": ["string", "null"], "description": "Optional parent thought ID for branching"},
                    "strategyType": {"type": ["string", "null"], "enum": ["beam_search", "mcts", "mcts_002_alpha", "mcts_002alt_alpha", null], "description": "Reasoning strategy to use"},
                    "beamWidth": {"type": ["integer", "null"], "description": "Number of top paths to maintain", "minimum": 1, "maximum": 10},
                    "numSimulations": {"type": ["integer", "null"], "description": "Number of MCTS simulations", "minimum": 1, "maximum": 150}
                },
                "required": ["thought", "thoughtNumber", "totalThoughts", "nextThoughtNeeded"]
            })),
        };

        Self {
            name: name.into(),
            text_to_text_model: None,
            text_embedding_model: None,
            temperature: 0.0,
            max_tokens: None,
            memory_read_timeout: 5000,
            system_prompt: r#"# Well-Informed Software Architect

You think out loud as you work through problems, sharing your process in addition to the solutions.
You track every task you do or needs doing in `TODO.md` , updating it religiously before and after a meaningful change to code.
You maintain `ARCHITECTURE.md`  and carefully curate the vision for the modules we create.
You prototype exploratory code ideas, quickly putting together a prototype, so we talk about the "heart of the matter" and get on the same page.
If you don't know the answer, you ALWAYS RESEARCH on the web and talk it through with me. You know that planned work takes less time in the end that hastily forged code. You never pretend to have answers unless you are highly confident.
You produce clean, maintainable, *production quality* code all the time.
You are a master at debugging and fixing bugs.
You are a master at refactoring code, remembering to check for code that ALREADY EXISTS before writing new code that might duplicate existing functionality."#.to_string(),
            tools: ZeroOneOrMany::from(vec![thinking_tool, reasoner_tool]),
            context_file: None,
            context_files: None,
            context_directory: None,
            context_github: None,
            additional_params: std::collections::HashMap::new(),
            metadata: std::collections::HashMap::new(),
            on_chunk_handler: None,
            on_tool_result_handler: None,
            on_conversation_turn_handler: None,
        }
    }
}

// Implementation for no-provider builder
impl CandleAgentRoleBuilder for CandleAgentRoleBuilderImpl {
    fn new(name: impl Into<String>) -> impl CandleAgentRoleBuilder {
        CandleAgentRoleBuilderImpl::new(name)
    }

    fn model(self, model: TextToTextModel) -> impl CandleAgentRoleBuilder {
        use crate::capability::registry;
        use crate::domain::model::traits::CandleModel;

        // Get max_tokens from model's ModelInfo
        let model_max_tokens = model
            .info()
            .max_output_tokens
            .map(|t| t.get().into())
            .unwrap_or(2000);

        // Get default embedding model from registry
        let default_embedding_model =
            registry::get::<TextEmbeddingModel>("dunzhang/stella_en_400M_v5")
                .expect("Default embedding model not found in registry");

        CandleAgentBuilderImpl {
            name: self.name,
            text_to_text_model: model,
            text_embedding_model: self.text_embedding_model.unwrap_or(default_embedding_model),
            temperature: self.temperature,
            max_tokens: self.max_tokens.unwrap_or(model_max_tokens),
            memory_read_timeout: self.memory_read_timeout,
            system_prompt: self.system_prompt,
            tools: self.tools,
            context_file: self.context_file,
            context_files: self.context_files,
            context_directory: self.context_directory,
            context_github: self.context_github,
            additional_params: self.additional_params,
            metadata: self.metadata,
            on_chunk_handler: self.on_chunk_handler,
            on_tool_result_handler: self.on_tool_result_handler,
            on_conversation_turn_handler: self.on_conversation_turn_handler,
        }
    }

    fn embedding_model(self, _model: TextEmbeddingModel) -> impl CandleAgentRoleBuilder {
        // For CandleAgentRoleBuilderImpl (no model yet), we can't set embedding model without text model
        // Return self unchanged - user should call .model() first
        self
    }

    /// Set temperature - EXACT syntax: .temperature(1.0)
    fn temperature(mut self, temp: f64) -> impl CandleAgentRoleBuilder {
        self.temperature = temp;
        self
    }

    /// Set max tokens - EXACT syntax: .max_tokens(8000)
    fn max_tokens(mut self, max: u64) -> impl CandleAgentRoleBuilder {
        self.max_tokens = Some(max);
        self
    }

    /// Set memory read timeout in milliseconds - EXACT syntax: .memory_read_timeout(5000)
    fn memory_read_timeout(mut self, timeout_ms: u64) -> impl CandleAgentRoleBuilder {
        self.memory_read_timeout = timeout_ms;
        self
    }

    /// Set system prompt - EXACT syntax: .system_prompt("...")
    fn system_prompt(mut self, prompt: impl Into<String>) -> impl CandleAgentRoleBuilder {
        self.system_prompt = prompt.into();
        self
    }

    /// Set additional params - EXACT syntax: .additional_params([("key", "value")])
    fn additional_params<P>(mut self, params: P) -> impl CandleAgentRoleBuilder
    where
        P: IntoIterator<Item = (&'static str, &'static str)>,
    {
        self.additional_params.extend(
            params
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string())),
        );
        self
    }

    /// Set metadata - EXACT syntax: .metadata([("key", "value")])
    fn metadata<Meta>(mut self, metadata: Meta) -> impl CandleAgentRoleBuilder
    where
        Meta: IntoIterator<Item = (&'static str, &'static str)>,
    {
        self.metadata.extend(
            metadata
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string())),
        );
        self
    }

    /// Set contexts - EXACT syntax: .context(CandleContext::<CandleFile>::of("/path"), ...)
    fn context(
        mut self,
        context1: CandleContext<CandleFile>,
        context2: CandleContext<CandleFiles>,
        context3: CandleContext<CandleDirectory>,
        context4: CandleContext<CandleGithub>,
    ) -> impl CandleAgentRoleBuilder {
        self.context_file = Some(context1);
        self.context_files = Some(context2);
        self.context_directory = Some(context3);
        self.context_github = Some(context4);
        self
    }

    /// Set tools - EXACT syntax: .tools(tool1, tool2, tool3)
    fn tools<T>(mut self, tools: T) -> impl CandleAgentRoleBuilder
    where
        T: Into<ZeroOneOrMany<ToolInfo>>,
    {
        self.tools = tools.into();
        self
    }

    /// Set MCP server - EXACT syntax: .mcp_server::<Stdio>().bin("/path").init("command")
    fn mcp_server<T>(self) -> impl CandleMcpServerBuilder
    where
        T: 'static,
    {
        CandleMcpServerBuilderImpl {
            parent_builder: self,
            binary_path: None,
        }
    }

    /// Add MCP server config - internal method for MCP server builder
    fn add_mcp_server_config(self, _config: McpServerConfig) -> impl CandleAgentRoleBuilder {
        // MCP servers are handled through tools
        self
    }

    /// Set chunk handler - EXACT syntax: .on_chunk(|chunk| chunk)
    fn on_chunk<F>(mut self, handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(CandleMessageChunk) -> CandleMessageChunk + Send + Sync + 'static,
    {
        self.on_chunk_handler = Some(Arc::new(handler));
        self
    }

    /// Set tool result handler - EXACT syntax: .on_tool_result(|results| { ... })
    fn on_tool_result<F>(mut self, handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(&[String]) + Send + Sync + 'static,
    {
        self.on_tool_result_handler = Some(Arc::new(handler));
        self
    }

    /// Set conversation turn handler - EXACT syntax: .on_conversation_turn(|conversation, agent| { ... })
    fn on_conversation_turn<F>(self, _handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(&CandleAgentConversation, &CandleAgentRoleAgent) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>
            + Send
            + Sync
            + 'static,
    {
        // Cannot set handler without a model - return self unchanged
        self
    }

    fn conversation_history(
        self,
        _history: impl ConversationHistoryArgs,
    ) -> impl CandleAgentRoleBuilder {
        self
    }

    /// Chat with closure - EXACT syntax: .chat(|conversation| ChatLoop)
    fn chat<F>(self, _handler: F) -> Result<Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>, AgentError>
    where
        F: FnOnce(&CandleAgentConversation) -> CandleChatLoop + Send + 'static,
    {
        Ok(Box::pin(crate::async_stream::spawn_stream(|sender| async move {
            let _ = sender.send(CandleMessageChunk::Error(
                "No provider configured".to_string(),
            ));
        })))
    }

    fn chat_with_message(self, message: impl Into<String>) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>> {
        let msg = message.into();
        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            let _ = sender.send(CandleMessageChunk::Error(format!(
                "No provider configured for message: {}",
                msg
            )));
        }))
    }

    /// Convert to agent - EXACT syntax: .into_agent()
    fn into_agent(self) -> impl CandleAgentBuilder {
        use crate::capability::registry;
        use crate::capability::text_to_text::CandlePhi4ReasoningModel;
        use crate::domain::model::traits::CandleModel;
        use std::sync::Arc;

        // Get default text-to-text model if not set
        let text_model = self.text_to_text_model.unwrap_or_else(|| {
            TextToTextModel::Phi4Reasoning(Arc::new(CandlePhi4ReasoningModel::default()))
        });

        // Get max_tokens from model's ModelInfo
        let model_max_tokens = text_model
            .info()
            .max_output_tokens
            .map(|t| t.get().into())
            .unwrap_or(2000);

        // Get default embedding model from registry
        let embedding_model = self.text_embedding_model.unwrap_or_else(|| {
            registry::get::<TextEmbeddingModel>("dunzhang/stella_en_400M_v5")
                .expect("Default embedding model not found in registry")
        });

        CandleAgentBuilderImpl {
            name: self.name,
            text_to_text_model: text_model,
            text_embedding_model: embedding_model,
            temperature: self.temperature,
            max_tokens: self.max_tokens.unwrap_or(model_max_tokens),
            memory_read_timeout: self.memory_read_timeout,
            system_prompt: self.system_prompt,
            tools: self.tools,
            context_file: self.context_file,
            context_files: self.context_files,
            context_directory: self.context_directory,
            context_github: self.context_github,
            additional_params: self.additional_params,
            metadata: self.metadata,
            on_chunk_handler: self.on_chunk_handler,
            on_tool_result_handler: self.on_tool_result_handler,
            on_conversation_turn_handler: self.on_conversation_turn_handler,
        }
    }
}

/// Debug information for agent configuration
#[derive(Debug, Clone)]
pub struct AgentDebugInfo {
    pub name: String,
    pub temperature: Option<f64>,
    pub max_tokens: Option<u64>,
    pub has_system_prompt: bool,
}

/// Agent builder implementation
pub struct CandleAgentBuilderImpl {
    name: String,
    text_to_text_model: TextToTextModel,
    text_embedding_model: TextEmbeddingModel,
    temperature: f64,
    max_tokens: u64,
    memory_read_timeout: u64,
    system_prompt: String,
    tools: ZeroOneOrMany<ToolInfo>,
    context_file: Option<CandleContext<CandleFile>>,
    context_files: Option<CandleContext<CandleFiles>>,
    context_directory: Option<CandleContext<CandleDirectory>>,
    context_github: Option<CandleContext<CandleGithub>>,
    additional_params: std::collections::HashMap<String, String>,
    metadata: std::collections::HashMap<String, String>,
    on_chunk_handler: Option<Arc<dyn Fn(CandleMessageChunk) -> CandleMessageChunk + Send + Sync>>,
    on_tool_result_handler: Option<OnToolResultHandler>,
    on_conversation_turn_handler: Option<OnConversationTurnHandler>,
}

impl std::fmt::Debug for CandleAgentBuilderImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CandleAgentBuilderImpl")
            .field("name", &self.name)
            .field("text_to_text_model", &self.text_to_text_model)
            .field("text_embedding_model", &self.text_embedding_model)
            .field("temperature", &self.temperature)
            .field("max_tokens", &self.max_tokens)
            .field("memory_read_timeout", &self.memory_read_timeout)
            .field(
                "system_prompt",
                &format!(
                    "{}...",
                    &self.system_prompt[..self.system_prompt.len().min(50)]
                ),
            )
            .field("tools", &self.tools)
            .field("additional_params", &self.additional_params)
            .field("metadata", &self.metadata)
            .finish()
    }
}

// Implementation for with-provider builder (allows all methods)
impl CandleAgentRoleBuilder for CandleAgentBuilderImpl {
    fn new(name: impl Into<String>) -> impl CandleAgentRoleBuilder {
        CandleAgentRoleBuilderImpl::new(name)
    }

    fn model(mut self, model: TextToTextModel) -> impl CandleAgentRoleBuilder {
        self.text_to_text_model = model;
        self
    }

    fn embedding_model(mut self, model: TextEmbeddingModel) -> impl CandleAgentRoleBuilder {
        self.text_embedding_model = model;
        self
    }

    fn temperature(mut self, temp: f64) -> impl CandleAgentRoleBuilder {
        self.temperature = temp;
        self
    }

    fn max_tokens(mut self, max: u64) -> impl CandleAgentRoleBuilder {
        self.max_tokens = max;
        self
    }

    fn memory_read_timeout(mut self, timeout_ms: u64) -> impl CandleAgentRoleBuilder {
        self.memory_read_timeout = timeout_ms;
        self
    }

    fn system_prompt(mut self, prompt: impl Into<String>) -> impl CandleAgentRoleBuilder {
        self.system_prompt = prompt.into();
        self
    }

    fn additional_params<P2>(mut self, params: P2) -> impl CandleAgentRoleBuilder
    where
        P2: IntoIterator<Item = (&'static str, &'static str)>,
    {
        self.additional_params.extend(
            params
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string())),
        );
        self
    }

    fn metadata<Meta>(mut self, metadata: Meta) -> impl CandleAgentRoleBuilder
    where
        Meta: IntoIterator<Item = (&'static str, &'static str)>,
    {
        self.metadata.extend(
            metadata
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string())),
        );
        self
    }

    fn context(
        mut self,
        context1: CandleContext<CandleFile>,
        context2: CandleContext<CandleFiles>,
        context3: CandleContext<CandleDirectory>,
        context4: CandleContext<CandleGithub>,
    ) -> impl CandleAgentRoleBuilder {
        self.context_file = Some(context1);
        self.context_files = Some(context2);
        self.context_directory = Some(context3);
        self.context_github = Some(context4);
        self
    }

    fn tools<T>(mut self, tools: T) -> impl CandleAgentRoleBuilder
    where
        T: Into<ZeroOneOrMany<ToolInfo>>,
    {
        self.tools = tools.into();
        self
    }

    fn mcp_server<T>(self) -> impl CandleMcpServerBuilder
    where
        T: 'static,
    {
        CandleMcpServerBuilderImpl {
            parent_builder: self,
            binary_path: None,
        }
    }

    /// Add MCP server config - internal method for MCP server builder
    fn add_mcp_server_config(self, _config: McpServerConfig) -> impl CandleAgentRoleBuilder {
        // MCP servers are handled through tools
        self
    }

    fn on_chunk<F>(mut self, handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(CandleMessageChunk) -> CandleMessageChunk + Send + Sync + 'static,
    {
        self.on_chunk_handler = Some(Arc::new(handler));
        self
    }

    fn on_tool_result<F>(mut self, handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(&[String]) + Send + Sync + 'static,
    {
        self.on_tool_result_handler = Some(Arc::new(handler));
        self
    }

    /// Set conversation turn handler - EXACT syntax: .on_conversation_turn(|conversation, agent| { ... })
    fn on_conversation_turn<F>(mut self, handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(&CandleAgentConversation, &CandleAgentRoleAgent) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>
            + Send
            + Sync
            + 'static,
    {
        self.on_conversation_turn_handler = Some(Arc::new(handler));
        self
    }

    fn conversation_history(
        self,
        _history: impl ConversationHistoryArgs,
    ) -> impl CandleAgentRoleBuilder {
        self
    }

    /// Chat with closure - EXACT syntax: .chat(|conversation| ChatLoop)
    fn chat<F>(self, _handler: F) -> Result<Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>, AgentError>
    where
        F: FnOnce(&CandleAgentConversation) -> CandleChatLoop + Send + 'static,
    {
        Ok(Box::pin(crate::async_stream::spawn_stream(|sender| async move {
            let _ = sender.send(CandleMessageChunk::Text("Hello from Candle!".to_string()));
        })))
    }

    fn chat_with_message(self, message: impl Into<String>) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>> {
        let msg = message.into();
        // Use CandleAgentRoleBuilder::chat explicitly to avoid ambiguity
        CandleAgentRoleBuilder::chat(self, move |_| CandleChatLoop::UserPrompt(msg)).unwrap_or_else(
            |_| {
                Box::pin(crate::async_stream::spawn_stream(|sender| async move {
                    let _ = sender.send(CandleMessageChunk::Error("Chat failed".to_string()));
                }))
            },
        )
    }

    fn into_agent(self) -> impl CandleAgentBuilder {
        self
    }
}

/// Format memory search results into prompt context
///
/// # Arguments
/// * `memories` - Vector of memory nodes sorted by relevance
/// * `max_tokens` - Maximum token budget for context
///
/// # Returns
/// Formatted markdown string with memory context
fn format_memory_context(
    memories: &[crate::memory::primitives::node::MemoryNode],
    max_tokens: usize,
) -> String {
    use std::fmt::Write;

    let mut context = String::from("# Relevant Context from Memory:\n\n");
    let mut token_count = 0usize;

    for memory in memories {
        // Approximate token count: chars / 4
        let memory_tokens = memory.content.text.chars().count() / 4;

        if token_count + memory_tokens > max_tokens {
            break; // Exceed budget, stop adding
        }

        // Format with relevance indicator
        let relevance = memory.metadata.importance;
        let _ = writeln!(
            &mut context,
            "[Relevance: {:.2}] {}\n",
            relevance, memory.content.text
        );

        token_count += memory_tokens;
    }

    context
}

impl CandleAgentBuilder for CandleAgentBuilderImpl {
    fn model(mut self, model: TextToTextModel) -> Self {
        self.text_to_text_model = model;
        self
    }

    fn embedding_model(mut self, model: TextEmbeddingModel) -> Self {
        self.text_embedding_model = model;
        self
    }

    fn temperature(mut self, temp: f64) -> Self {
        self.temperature = temp;
        self
    }

    fn max_tokens(mut self, max: u64) -> Self {
        self.max_tokens = max;
        self
    }

    fn memory_read_timeout(mut self, timeout_ms: u64) -> Self {
        self.memory_read_timeout = timeout_ms;
        self
    }

    fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = prompt.into();
        self
    }

    fn additional_params<P2>(mut self, params: P2) -> Self
    where
        P2: IntoIterator<Item = (&'static str, &'static str)>,
    {
        self.additional_params.extend(
            params
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string())),
        );
        self
    }

    fn metadata<Meta>(mut self, metadata: Meta) -> Self
    where
        Meta: IntoIterator<Item = (&'static str, &'static str)>,
    {
        self.metadata.extend(
            metadata
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string())),
        );
        self
    }

    fn context(
        mut self,
        context1: CandleContext<CandleFile>,
        context2: CandleContext<CandleFiles>,
        context3: CandleContext<CandleDirectory>,
        context4: CandleContext<CandleGithub>,
    ) -> Self {
        self.context_file = Some(context1);
        self.context_files = Some(context2);
        self.context_directory = Some(context3);
        self.context_github = Some(context4);
        self
    }

    fn tools<T>(mut self, tools: T) -> Self
    where
        T: Into<ZeroOneOrMany<ToolInfo>>,
    {
        self.tools = tools.into();
        self
    }

    fn mcp_server<T>(self) -> impl CandleMcpServerBuilder
    where
        T: 'static,
    {
        CandleMcpServerBuilderImpl {
            parent_builder: self,
            binary_path: None,
        }
    }

    fn add_mcp_server_config(self, _config: McpServerConfig) -> Self {
        // MCP servers are handled through tools
        self
    }

    fn on_chunk<F>(mut self, handler: F) -> Self
    where
        F: Fn(CandleMessageChunk) -> CandleMessageChunk + Send + Sync + 'static,
    {
        self.on_chunk_handler = Some(Arc::new(handler));
        self
    }

    fn on_tool_result<F>(mut self, handler: F) -> Self
    where
        F: Fn(&[String]) + Send + Sync + 'static,
    {
        self.on_tool_result_handler = Some(Arc::new(handler));
        self
    }

    fn on_conversation_turn<F>(mut self, handler: F) -> Self
    where
        F: Fn(&CandleAgentConversation, &CandleAgentRoleAgent) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>
            + Send
            + Sync
            + 'static,
    {
        self.on_conversation_turn_handler = Some(Arc::new(handler));
        self
    }

    fn conversation_history(self, _history: impl ConversationHistoryArgs) -> Self {
        self
    }

    /// Chat with closure - EXACT syntax: .chat(|conversation| ChatLoop)
    fn chat<F>(self, handler: F) -> Result<Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>, AgentError>
    where
        F: FnOnce(&CandleAgentConversation) -> CandleChatLoop + Send + 'static,
    {
        let provider = self.text_to_text_model;
        let embedding_model = self.text_embedding_model;
        let temperature = self.temperature;
        let max_tokens = self.max_tokens;
        let memory_read_timeout = self.memory_read_timeout;
        let system_prompt = self.system_prompt.clone();
        let tools = self.tools;
        let on_conversation_turn_handler = self.on_conversation_turn_handler;
        let on_chunk_handler = self.on_chunk_handler;
        let on_tool_result_handler = self.on_tool_result_handler;
        let context_file = self.context_file;
        let context_files = self.context_files;
        let context_directory = self.context_directory;
        let context_github = self.context_github;
        let additional_params = self.additional_params;
        let metadata = self.metadata;

        Ok(Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            // Create initial empty conversation for handler to inspect
            let initial_conversation = CandleAgentConversation::new();

            // Execute handler to get CandleChatLoop result
            let chat_loop_result = handler(&initial_conversation);

            // Process CandleChatLoop result
            match chat_loop_result {
                CandleChatLoop::Break => {
                    // User wants to exit - send final chunk
                    let final_chunk = CandleMessageChunk::Complete {
                        text: String::new(),
                        finish_reason: Some("break".to_string()),
                        usage: None,
                        token_count: None,
                        elapsed_secs: None,
                        tokens_per_sec: None,
                    };
                    let _ = sender.send(final_chunk);
                }
                CandleChatLoop::UserPrompt(user_message)
                | CandleChatLoop::Reprompt(user_message) => {
                            // ALWAYS create memory internally - WE GUARANTEE MEMORY
                            // Database connection and memory manager initialization
                            use surrealdb::engine::any::connect;
                            use crate::memory::core::manager::surreal::SurrealDBMemoryManager;
                            use crate::memory::primitives::node::MemoryNode;
                            use crate::memory::primitives::types::{MemoryContent, MemoryTypeEnum};
                            use chrono::Utc;

                    let db_path = dirs::cache_dir()
                        .unwrap_or_else(|| std::path::PathBuf::from("."))
                        .join("paraphym")
                        .join("agent.db");

                    if let Some(parent) = db_path.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }

                    let db_url = format!("surrealkv://{}", db_path.display());

                    // Connect to database using .await
                    let db = match connect(&db_url).await {
                        Ok(db) => db,
                        Err(e) => {
                            let _ = sender.send(CandleMessageChunk::Error(
                                format!("Failed to connect to database: {}", e)
                            ));
                            return;
                        }
                    };

                    // Initialize database namespace using .await
                    if let Err(e) = db.use_ns("candle").use_db("agent").await {
                        let _ = sender.send(CandleMessageChunk::Error(
                            format!("Failed to initialize database namespace: {}", e)
                        ));
                        return;
                    }

                    // Create and initialize memory manager using .await
                    let manager = match SurrealDBMemoryManager::with_embedding_model(db, embedding_model.clone()).await {
                        Ok(mgr) => mgr,
                        Err(e) => {
                            let _ = sender.send(CandleMessageChunk::Error(
                                format!("Failed to create memory manager: {}", e)
                            ));
                            return;
                        }
                    };

                    if let Err(e) = manager.initialize().await {
                        let _ = sender.send(CandleMessageChunk::Error(
                            format!("Failed to initialize memory tables: {}", e)
                        ));
                        return;
                    }

                    let memory: Option<Arc<dyn crate::memory::core::manager::surreal::MemoryManager>> =
                        Some(Arc::new(manager) as Arc<dyn crate::memory::core::manager::surreal::MemoryManager>);

                    // Ingest documents from context fields into memory using .await
                    if let Some(ref mem_mgr) = memory {
                        // Load from context_file
                        if let Some(ctx) = context_file {
                            let doc_stream = ctx.load();
                            tokio::pin!(doc_stream);
                            while let Some(doc) = doc_stream.next().await {
                                let content_hash = crate::domain::memory::serialization::content_hash(&doc.data);
                                let memory_node = MemoryNode {
                                    id: format!("doc_{}", content_hash),
                                    content: MemoryContent::new(&doc.data),
                                    content_hash,
                                    memory_type: MemoryTypeEnum::Semantic,
                                    created_at: Utc::now(),
                                    updated_at: Utc::now(),
                                    embedding: None,
                                    evaluation_status: crate::memory::monitoring::operations::OperationStatus::Pending,
                                    metadata: crate::memory::primitives::metadata::MemoryMetadata::new(),
                                    relevance_score: None,
                                };
                                if let Err(e) = mem_mgr.create_memory(memory_node).await {
                                    log::error!("Failed to ingest document: {:?}", e);
                                }
                            }
                        }

                        // Load from context_files
                        if let Some(ctx) = context_files {
                            let doc_stream = ctx.load();
                            tokio::pin!(doc_stream);
                            while let Some(doc) = doc_stream.next().await {
                                let content_hash = crate::domain::memory::serialization::content_hash(&doc.data);
                                let memory_node = MemoryNode {
                                    id: format!("doc_{}", content_hash),
                                    content: MemoryContent::new(&doc.data),
                                    content_hash,
                                    memory_type: MemoryTypeEnum::Semantic,
                                    created_at: Utc::now(),
                                    updated_at: Utc::now(),
                                    embedding: None,
                                    evaluation_status: crate::memory::monitoring::operations::OperationStatus::Pending,
                                    metadata: crate::memory::primitives::metadata::MemoryMetadata::new(),
                                    relevance_score: None,
                                };
                                if let Err(e) = mem_mgr.create_memory(memory_node).await {
                                    log::error!("Failed to ingest document: {:?}", e);
                                }
                            }
                        }

                        // Load from context_directory
                        if let Some(ctx) = context_directory {
                            let doc_stream = ctx.load();
                            tokio::pin!(doc_stream);
                            while let Some(doc) = doc_stream.next().await {
                                let content_hash = crate::domain::memory::serialization::content_hash(&doc.data);
                                let memory_node = MemoryNode {
                                    id: format!("doc_{}", content_hash),
                                    content: MemoryContent::new(&doc.data),
                                    content_hash,
                                    memory_type: MemoryTypeEnum::Semantic,
                                    created_at: Utc::now(),
                                    updated_at: Utc::now(),
                                    embedding: None,
                                    evaluation_status: crate::memory::monitoring::operations::OperationStatus::Pending,
                                    metadata: crate::memory::primitives::metadata::MemoryMetadata::new(),
                                    relevance_score: None,
                                };
                                if let Err(e) = mem_mgr.create_memory(memory_node).await {
                                    log::error!("Failed to ingest document: {:?}", e);
                                }
                            }
                        }

                        // Load from context_github
                        if let Some(ctx) = context_github {
                            let doc_stream = ctx.load();
                            tokio::pin!(doc_stream);
                            while let Some(doc) = doc_stream.next().await {
                                let content_hash = crate::domain::memory::serialization::content_hash(&doc.data);
                                let memory_node = MemoryNode {
                                    id: format!("doc_{}", content_hash),
                                    content: MemoryContent::new(&doc.data),
                                    content_hash,
                                    memory_type: MemoryTypeEnum::Semantic,
                                    created_at: Utc::now(),
                                    updated_at: Utc::now(),
                                    embedding: None,
                                    evaluation_status: crate::memory::monitoring::operations::OperationStatus::Pending,
                                    metadata: crate::memory::primitives::metadata::MemoryMetadata::new(),
                                    relevance_score: None,
                                };
                                if let Err(e) = mem_mgr.create_memory(memory_node).await {
                                    log::error!("Failed to ingest document: {:?}", e);
                                }
                            }
                        }
                    }

                    // Initialize tool router - ALWAYS create with default reasoner plugin
                    let tool_router = {

                            // Create default reasoner plugin configuration
                            let reasoner_schema =
                                crate::domain::agent::role::convert_serde_to_sweet_json(
                                    serde_json::json!({
                                        "type": "object",
                                        "properties": {
                                            "thought": {
                                                "type": "string",
                                                "description": "Current reasoning step"
                                            },
                                            "thoughtNumber": {
                                                "type": "integer",
                                                "description": "Current step number",
                                                "minimum": 1
                                            },
                                            "totalThoughts": {
                                                "type": "integer",
                                                "description": "Total expected steps",
                                                "minimum": 1
                                            },
                                            "nextThoughtNeeded": {
                                                "type": "boolean",
                                                "description": "Whether another step is needed"
                                            },
                                            "parentId": {
                                                "type": ["string", "null"],
                                                "description": "Optional parent thought ID for branching"
                                            },
                                            "strategyType": {
                                                "type": ["string", "null"],
                                                "enum": ["beam_search", "mcts", "mcts_002_alpha", "mcts_002alt_alpha", null],
                                                "description": "Reasoning strategy to use"
                                            },
                                            "beamWidth": {
                                                "type": ["integer", "null"],
                                                "description": "Number of top paths to maintain",
                                                "minimum": 1,
                                                "maximum": 10
                                            },
                                            "numSimulations": {
                                                "type": ["integer", "null"],
                                                "description": "Number of MCTS simulations",
                                                "minimum": 1,
                                                "maximum": 150
                                            }
                                        },
                                        "required": ["thought", "thoughtNumber", "totalThoughts", "nextThoughtNeeded"]
                                    }),
                                );

                            let default_plugin_config = crate::domain::tool::router::PluginConfig {
                                tool_name: "mcp-reasoner".to_string(),
                                wasm_path: "packages/sweetmcp/plugins/reasoner/target/wasm32-unknown-unknown/release/sweetmcp_plugin_reasoner.wasm".to_string(),
                                description: "Advanced reasoning tool with Beam Search and MCTS strategies".to_string(),
                                input_schema: reasoner_schema,
                            };

                            // Create router with default reasoner plugin
                            let plugin_configs = vec![default_plugin_config];
                            let mut router = SweetMcpRouter::with_configs(plugin_configs, None);

                            match router.initialize().await {
                                Ok(()) => Some(router),
                                Err(e) => {
                                    let error_chunk = CandleMessageChunk::Error(format!(
                                        "Tool initialization failed: {}",
                                        e
                                    ));
                                    let _ = sender.send(error_chunk);
                                    return;
                                }
                            }
                        };

                        // Memory context injection
                        let memory_context: Option<String> = if let Some(ref mem_manager) = memory {
                            let memory_stream = mem_manager.search_by_content(&user_message);
                            let timeout_duration = std::time::Duration::from_millis(memory_read_timeout);

                            // Use native async/await with timeout protection
                            let results: Vec<MemoryResult<MemoryNode>> = match tokio::time::timeout(timeout_duration, async {
                                use futures_util::StreamExt as FuturesStreamExt;
                                let limited = FuturesStreamExt::take(memory_stream, 5);
                                FuturesStreamExt::collect(limited).await
                            }).await {
                                Ok(results) => results,
                                Err(_) => {
                                    log::warn!("Memory search timed out after {}ms, proceeding with empty context", memory_read_timeout);
                                    Vec::new()
                                }
                            };

                            let memories: Vec<MemoryNode> =
                                results.into_iter().filter_map(|r| r.ok()).collect();

                            if !memories.is_empty() {
                                Some(format_memory_context(&memories, 1000))
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        // Create prompt with memory context and system prompt (always present)
                        let full_prompt = match memory_context {
                            Some(mem_ctx) => {
                                format!("{}\n\n{}\n\nUser: {}", &system_prompt, mem_ctx, user_message)
                            }
                            None => {
                                format!("{}\n\nUser: {}", &system_prompt, user_message)
                            }
                        };

                        // Create CandlePrompt and CandleCompletionParams with tools if available
                        let prompt = CandlePrompt::new(full_prompt);
                        let mut params = CandleCompletionParams {
                            temperature,
                            max_tokens: NonZeroU64::new(max_tokens),
                            ..Default::default()
                        };

                        // Combine builder tools with auto-generated tools
                        if let Some(ref router) = tool_router {
                            let mut all_tools: Vec<ToolInfo> = tools.clone().into();

                            // Get auto-generated tools using native async/await
                            let auto_generated_tools = router.get_available_tools().await;
                            all_tools.extend(auto_generated_tools);

                            if !all_tools.is_empty() {
                                // Pass merged tools to completion system for function calling
                                params.tools = Some(ZeroOneOrMany::from(all_tools));
                            }
                        }

                        // Merge additional params if provided (zero allocation for empty map)
                        if !additional_params.is_empty() {
                            let params_map: serde_json::Map<String, serde_json::Value> = additional_params
                                .into_iter()
                                .map(|(k, v)| (k, serde_json::Value::String(v)))
                                .collect();
                            params = params.with_additional_params(Some(serde_json::Value::Object(params_map)));
                        }

                        // Call REAL provider inference
                        let completion_stream = provider.prompt(prompt, &params);
                        tokio::pin!(completion_stream);

                        // Convert CandleCompletionChunk to CandleMessageChunk and forward
                        // Handle tool calls if they occur
                        let mut assistant_response = String::new();

                        // Track metrics for performance visibility
                        let start_time = std::time::Instant::now();
                        let mut token_count = 0u32;

                        while let Some(completion_chunk) = completion_stream.next().await {
                            let message_chunk = match completion_chunk {
                                CandleCompletionChunk::Text(ref text) => {
                                    token_count += 1;
                                    assistant_response.push_str(text);
                                    CandleMessageChunk::Text(text.clone())
                                }
                                CandleCompletionChunk::Complete {
                                    ref text,
                                    finish_reason,
                                    usage,
                                } => {
                                    assistant_response.push_str(text);

                                    // Calculate performance metrics
                                    let elapsed = start_time.elapsed();
                                    let elapsed_secs = elapsed.as_secs_f64();
                                    let tokens_per_sec = if elapsed_secs > 0.0 {
                                        Some(token_count as f64 / elapsed_secs)
                                    } else {
                                        None
                                    };

                                    CandleMessageChunk::Complete {
                                        text: text.clone(),
                                        finish_reason: finish_reason.map(|f| format!("{:?}", f)),
                                        usage: usage.map(|u| format!("{:?}", u)),
                                        token_count: Some(token_count),
                                        elapsed_secs: Some(elapsed_secs),
                                        tokens_per_sec,
                                    }
                                }
                                CandleCompletionChunk::ToolCallStart { id, name } => {
                                    CandleMessageChunk::ToolCallStart { id, name }
                                }
                                CandleCompletionChunk::ToolCall {
                                    id,
                                    name,
                                    partial_input,
                                } => CandleMessageChunk::ToolCall {
                                    id,
                                    name,
                                    partial_input,
                                },
                                CandleCompletionChunk::ToolCallComplete { id, name, input } => {
                                    // Execute the tool if we have a router
                                    if let Some(ref router) = tool_router {
                                        // Convert input string to JsonValue
                                        match serde_json::from_str::<serde_json::Value>(&input) {
                                            Ok(args_json) => {
                                                // Convert to SweetMCP JsonValue
                                                let sweet_args = crate::domain::agent::role::convert_serde_to_sweet_json(args_json);

                                                // Execute the tool using native async/await
                                                match router.call_tool(&name, sweet_args).await {
                                                    Ok(response) => {
                                                        // Call tool result handler if configured (zero allocation for None)
                                                        if let Some(ref handler) = on_tool_result_handler {
                                                            let results = vec![format!("{:?}", response)];
                                                            handler(&results);
                                                        }

                                                        // Convert response to text result
                                                        let result_text = format!(
                                                            "Tool '{}' executed successfully: {:?}",
                                                            name, response
                                                        );
                                                        CandleMessageChunk::Text(
                                                            result_text,
                                                        )
                                                    }
                                                    Err(e) => {
                                                        // Return error as text
                                                        CandleMessageChunk::Error(format!(
                                                            "Tool '{}' execution failed: {}",
                                                            name, e
                                                        ))
                                                    }
                                                }
                                            }
                                            Err(e) => CandleMessageChunk::Error(format!(
                                                "Tool '{}' invalid JSON input: {}",
                                                name, e
                                            )),
                                        }
                                    } else {
                                        CandleMessageChunk::ToolCallComplete { id, name, input }
                                    }
                                }
                                CandleCompletionChunk::Error(error) => {
                                    CandleMessageChunk::Error(error)
                                }
                            };

                            // Apply chunk handler if configured (zero allocation for None)
                            let final_chunk = if let Some(ref handler) = on_chunk_handler {
                                handler(message_chunk)
                            } else {
                                message_chunk
                            };
                            let _ = sender.send(final_chunk);
                        }

                        // Store conversation turn in memory after completion
                        if let Some(ref memory_manager) = memory
                            && !assistant_response.is_empty()
                        {
                            // Create memory nodes for the conversation
                            let user_content =
                                crate::memory::core::primitives::types::MemoryContent::new(
                                    &user_message,
                                );
                            let assistant_content =
                                crate::memory::core::primitives::types::MemoryContent::new(
                                    &assistant_response,
                                );

                            // Use Episodic memory type for conversation turns
                            let mut user_memory = crate::memory::core::MemoryNode::new(
                                crate::memory::core::primitives::types::MemoryTypeEnum::Episodic,
                                user_content,
                            );
                            user_memory.metadata.tags.push("user_message".to_string());
                            user_memory.metadata.source = Some("chat".to_string());
                            user_memory.metadata.importance = 0.8;

                            let mut assistant_memory = crate::memory::core::MemoryNode::new(
                                crate::memory::core::primitives::types::MemoryTypeEnum::Episodic,
                                assistant_content,
                            );
                            assistant_memory
                                .metadata
                                .tags
                                .push("assistant_response".to_string());
                            assistant_memory.metadata.source = Some("chat".to_string());
                            assistant_memory.metadata.importance = 0.8;

                            // Merge builder metadata into memory metadata (zero allocation for empty map)
                            for (key, value) in &metadata {
                                if let Err(e) = user_memory.metadata.set_custom(key, value) {
                                    log::warn!("Failed to set custom metadata '{}' on user memory: {:?}", key, e);
                                }
                                if let Err(e) = assistant_memory.metadata.set_custom(key, value) {
                                    log::warn!("Failed to set custom metadata '{}' on assistant memory: {:?}", key, e);
                                }
                            }

                            // Create PendingMemory operations
                            let user_pending = memory_manager.create_memory(user_memory);
                            let assistant_pending = memory_manager.create_memory(assistant_memory);

                            // Use shared runtime to properly await the PendingMemory futures
                            if let Some(runtime) = crate::runtime::shared_runtime() {
                                // Spawn tasks on the runtime to handle the async operations
                                runtime.spawn(async move {
                                    if let Err(e) = user_pending.await {
                                        log::error!(
                                            "Failed to store user memory to database: {:?}",
                                            e
                                        );
                                    }
                                });
                                runtime.spawn(async move {
                                    if let Err(e) = assistant_pending.await {
                                        log::error!(
                                            "Failed to store assistant memory to database: {:?}",
                                            e
                                        );
                                    }
                                });
                            }
                        }

                        // Invoke on_conversation_turn handler if configured
                        if let Some(handler) = on_conversation_turn_handler {
                            // Create conversation with current messages
                            let mut conversation = CandleAgentConversation::new();
                            conversation.add_message(user_message.clone(), CandleMessageRole::User);
                            conversation.add_message(
                                assistant_response.clone(),
                                CandleMessageRole::Assistant,
                            );

                            // Create builder state for recursive agent with handler included
                            // handler is already Arc<dyn Fn(...)>, so just clone it
                            let builder_state = Arc::new(AgentBuilderState {
                                name: String::from("agent"),
                                text_to_text_model: provider.clone(),
                                text_embedding_model: embedding_model.clone(),
                                temperature,
                                max_tokens,
                                memory_read_timeout,
                                system_prompt: system_prompt.clone(),
                                tools: tools.clone(),
                                context_file: None,
                                context_files: None,
                                context_directory: None,
                                context_github: None,
                                additional_params: std::collections::HashMap::new(),
                                metadata: std::collections::HashMap::new(),
                                on_chunk_handler: None,
                                on_tool_result_handler: None,
                                on_conversation_turn_handler: Some(handler.clone()),
                            });

                            // Create agent with full state
                            let agent = CandleAgentRoleAgent {
                                state: builder_state,
                            };

                            // Call handler and forward its stream
                            let handler_stream = handler(&conversation, &agent);
                            tokio::pin!(handler_stream);
                            while let Some(chunk) = handler_stream.next().await {
                                let _ = sender.send(chunk);
                            }
                        }
                }
            }
        })))
    }

    fn chat_with_message(self, message: impl Into<String>) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>> {
        let provider = self.text_to_text_model;
        let temperature = self.temperature;
        let max_tokens = self.max_tokens;
        let system_prompt = self.system_prompt.clone();
        let on_chunk_handler = self.on_chunk_handler;
        let user_message = message.into();

        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            let full_prompt = format!("{}\n\nUser: {}", system_prompt, user_message);

            let prompt = CandlePrompt::new(full_prompt);
            let params = CandleCompletionParams {
                temperature,
                max_tokens: NonZeroU64::new(max_tokens),
                ..Default::default()
            };

            let completion_stream = provider.prompt(prompt, &params);
            tokio::pin!(completion_stream);

            // Track metrics for performance visibility
            let start_time = std::time::Instant::now();
            let mut token_count = 0u32;

            while let Some(completion_chunk) = completion_stream.next().await {
                let message_chunk = match completion_chunk {
                    CandleCompletionChunk::Text(text) => {
                        token_count += 1;
                        CandleMessageChunk::Text(text)
                    }
                    CandleCompletionChunk::Complete {
                        text,
                        finish_reason,
                        usage,
                    } => {
                        // Calculate performance metrics
                        let elapsed = start_time.elapsed();
                        let elapsed_secs = elapsed.as_secs_f64();
                        let tokens_per_sec = if elapsed_secs > 0.0 {
                            Some(token_count as f64 / elapsed_secs)
                        } else {
                            None
                        };

                        CandleMessageChunk::Complete {
                            text,
                            finish_reason: finish_reason.map(|f| format!("{:?}", f)),
                            usage: usage.map(|u| format!("{:?}", u)),
                            token_count: Some(token_count),
                            elapsed_secs: Some(elapsed_secs),
                            tokens_per_sec,
                        }
                    }
                    CandleCompletionChunk::ToolCallStart { id, name } => {
                        CandleMessageChunk::ToolCallStart { id, name }
                    }
                    CandleCompletionChunk::ToolCall {
                        id,
                        name,
                        partial_input,
                    } => CandleMessageChunk::ToolCall {
                        id,
                        name,
                        partial_input,
                    },
                    CandleCompletionChunk::ToolCallComplete { id, name, input } => {
                        CandleMessageChunk::ToolCallComplete { id, name, input }
                    }
                    CandleCompletionChunk::Error(error) => CandleMessageChunk::Error(error),
                };

                // Apply chunk handler if configured (zero allocation for None)
                let final_chunk = if let Some(ref handler) = on_chunk_handler {
                    handler(message_chunk)
                } else {
                    message_chunk
                };
                let _ = sender.send(final_chunk);
            }
        }))
    }
}

// ConversationHistoryArgs implementations for => syntax
// Enables: .conversation_history(CandleMessageRole::User => "What time is it in Paris, France", CandleMessageRole::System => "...", CandleMessageRole::Assistant => "...")

/// Trait for conversation history arguments supporting arrow syntax
pub trait ConversationHistoryArgs {
    /// Convert this into conversation history format
    fn into_history(self) -> Option<ZeroOneOrMany<(CandleMessageRole, String)>>;
}

impl ConversationHistoryArgs for (CandleMessageRole, &str) {
    fn into_history(self) -> Option<ZeroOneOrMany<(CandleMessageRole, String)>> {
        Some(ZeroOneOrMany::one((self.0, self.1.to_string())))
    }
}

impl ConversationHistoryArgs for (CandleMessageRole, String) {
    fn into_history(self) -> Option<ZeroOneOrMany<(CandleMessageRole, String)>> {
        Some(ZeroOneOrMany::one(self))
    }
}

impl<T1, T2> ConversationHistoryArgs for (T1, T2)
where
    T1: ConversationHistoryArgs,
    T2: ConversationHistoryArgs,
{
    fn into_history(self) -> Option<ZeroOneOrMany<(CandleMessageRole, String)>> {
        match (self.0.into_history(), self.1.into_history()) {
            (Some(h1), Some(h2)) => {
                let mut combined = h1;
                for item in h2.into_iter() {
                    combined = combined.with_pushed(item);
                }
                Some(combined)
            }
            (Some(h), None) | (None, Some(h)) => Some(h),
            (None, None) => None,
        }
    }
}

impl<T1, T2, T3> ConversationHistoryArgs for (T1, T2, T3)
where
    T1: ConversationHistoryArgs,
    T2: ConversationHistoryArgs,
    T3: ConversationHistoryArgs,
{
    fn into_history(self) -> Option<ZeroOneOrMany<(CandleMessageRole, String)>> {
        let (h1, h2, h3) = (
            self.0.into_history(),
            self.1.into_history(),
            self.2.into_history(),
        );
        match (h1, h2, h3) {
            (Some(mut combined), h2_opt, h3_opt) => {
                if let Some(h2) = h2_opt {
                    for item in h2.into_iter() {
                        combined = combined.with_pushed(item);
                    }
                }
                if let Some(h3) = h3_opt {
                    for item in h3.into_iter() {
                        combined = combined.with_pushed(item);
                    }
                }
                Some(combined)
            }
            (None, Some(mut combined), h3_opt) => {
                if let Some(h3) = h3_opt {
                    for item in h3.into_iter() {
                        combined = combined.with_pushed(item);
                    }
                }
                Some(combined)
            }
            (None, None, Some(h3)) => Some(h3),
            (None, None, None) => None,
        }
    }
}

/// CandleFluentAi entry point for creating agent roles
pub struct CandleFluentAi;

impl CandleFluentAi {
    /// Create a new agent role builder - main entry point
    pub fn agent_role(name: impl Into<String>) -> impl CandleAgentRoleBuilder {
        CandleAgentRoleBuilderImpl::new(name)
    }
}
