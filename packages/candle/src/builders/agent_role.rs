//! Builders are behavioral/construction logic, separate from core domain models

use std::num::NonZeroU64;
use std::sync::Arc;

use cyrup_sugars::ZeroOneOrMany;

use ystream::AsyncStream;

use crate::capability::traits::TextToTextCapable;
use crate::capability::registry::{TextToTextModel, TextEmbeddingModel};
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
use crate::memory::primitives::node::MemoryNode;

// Candle domain types - self-contained

/// Agent helper type for conversation control in `on_conversation_turn` callbacks.
/// Holds Arc<AgentBuilderState> for recursive inference.
#[derive(Clone)]
pub struct CandleAgentRoleAgent {
    state: Arc<AgentBuilderState>,
}

impl CandleAgentRoleAgent {
    /// Chat method for use in on_conversation_turn closure - enables recursion
    pub fn chat(&self, chat_loop: CandleChatLoop) -> AsyncStream<CandleMessageChunk> {
        match chat_loop {
            CandleChatLoop::Break => AsyncStream::with_channel(|sender| {
                let final_chunk = CandleMessageChunk::Complete {
                    text: String::new(),
                    finish_reason: Some("break".to_string()),
                    usage: None,
                };
                let _ = sender.send(final_chunk);
            }),
            CandleChatLoop::UserPrompt(user_message) | CandleChatLoop::Reprompt(user_message) => {
                self.run_inference_cycle(user_message)
            }
        }
    }

    fn run_inference_cycle(&self, user_message: String) -> AsyncStream<CandleMessageChunk> {
        let state = self.state.clone();

        AsyncStream::with_channel(move |_sender| {
            let _background_stream = ystream::spawn_stream(move |stream_sender| {
                // Initialize tool router
                let tool_router = {
                    let Some(runtime) = crate::runtime::shared_runtime() else {
                        let error_chunk = CandleMessageChunk::Error(
                            "Failed to access shared runtime".to_string(),
                        );
                        ystream::emit!(stream_sender, error_chunk);
                        return;
                    };

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

                    match runtime.block_on(router.initialize()) {
                        Ok(()) => Some(router),
                        Err(e) => {
                            let error_chunk = CandleMessageChunk::Error(format!(
                                "Tool initialization failed: {}",
                                e
                            ));
                            ystream::emit!(stream_sender, error_chunk);
                            return;
                        }
                    }
                };

                // Memory search
                let memory_context: Option<String> = if let Some(ref mem_manager) = state.memory {
                    let memory_stream = mem_manager.search_by_content(&user_message);
                    let results: Vec<MemoryResult<crate::memory::core::MemoryNode>> =
                        tokio::task::block_in_place(|| {
                            tokio::runtime::Handle::current().block_on(async {
                                use futures_util::StreamExt;
                                memory_stream.take(5).collect().await
                            })
                        });
                    let memories: Vec<crate::memory::core::MemoryNode> =
                        results.into_iter().filter_map(|r| r.ok()).collect();
                    if !memories.is_empty() {
                        Some(crate::builders::agent_role::format_memory_context(
                            &memories, 1000,
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                };

                // Build prompt
                let full_prompt = match (memory_context, &state.system_prompt) {
                    (Some(mem_ctx), Some(sys_prompt)) => {
                        format!("{}\n\n{}\n\nUser: {}", sys_prompt, mem_ctx, user_message)
                    }
                    (Some(mem_ctx), None) => {
                        format!("{}\n\nUser: {}", mem_ctx, user_message)
                    }
                    (None, Some(sys_prompt)) => {
                        format!("{}\n\nUser: {}", sys_prompt, user_message)
                    }
                    (None, None) => {
                        format!("User: {}", user_message)
                    }
                };

                // Call provider
                let prompt = CandlePrompt::new(full_prompt);
                let mut params = crate::domain::completion::CandleCompletionParams {
                    temperature: state.temperature,
                    max_tokens: std::num::NonZeroU64::new(state.max_tokens.unwrap_or(1000)),
                    ..Default::default()
                };

                // Add tools
                if let Some(ref router) = tool_router {
                    let mut all_tools: Vec<sweet_mcp_type::ToolInfo> = state.tools.clone().into();
                    if let Some(runtime) = crate::runtime::shared_runtime() {
                        let auto_generated_tools = runtime.block_on(router.get_available_tools());
                        all_tools.extend(auto_generated_tools);
                    }
                    if !all_tools.is_empty() {
                        params.tools = Some(ZeroOneOrMany::from(all_tools));
                    }
                }

                let completion_stream = state.text_to_text_model.prompt(prompt, &params);
                let completion_results = completion_stream.collect();
                let mut assistant_response = String::new();

                // Stream chunks
                for completion_chunk in completion_results {
                    let message_chunk = match completion_chunk {
                        CandleCompletionChunk::Text(ref text) => {
                            assistant_response.push_str(text);
                            CandleMessageChunk::Text(text.clone())
                        }
                        CandleCompletionChunk::Complete {
                            ref text,
                            finish_reason,
                            usage,
                        } => {
                            assistant_response.push_str(text);
                            CandleMessageChunk::Complete {
                                text: text.clone(),
                                finish_reason: finish_reason.map(|f| format!("{:?}", f)),
                                usage: usage.map(|u| format!("{:?}", u)),
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
                                        match crate::runtime::shared_runtime() {
                                            Some(runtime) => {
                                                match runtime
                                                    .block_on(router.call_tool(&name, sweet_args))
                                                {
                                                    Ok(response) => {
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
                                            None => CandleMessageChunk::Error(
                                                "Runtime unavailable".to_string(),
                                            ),
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
                    ystream::emit!(stream_sender, message_chunk);
                }

                // Store in memory
                if let Some(ref memory_manager) = state.memory {
                    if !assistant_response.is_empty() {
                        let user_content =
                            crate::memory::core::primitives::types::MemoryContent::new(
                                &user_message,
                            );
                        let assistant_content =
                            crate::memory::core::primitives::types::MemoryContent::new(
                                &assistant_response,
                            );

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

                        let user_pending = memory_manager.create_memory(user_memory);
                        let assistant_pending = memory_manager.create_memory(assistant_memory);

                        if let Some(runtime) = crate::runtime::shared_runtime() {
                            runtime.spawn(async move {
                                if let Err(e) = user_pending.await {
                                    log::error!("Failed to store user memory: {:?}", e);
                                }
                            });
                            runtime.spawn(async move {
                                if let Err(e) = assistant_pending.await {
                                    log::error!("Failed to store assistant memory: {:?}", e);
                                }
                            });
                        }
                    }
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
                    let handler_chunks = handler_stream.collect();
                    for chunk in handler_chunks {
                        ystream::emit!(stream_sender, chunk);
                    }
                }
            });
        })
    }
}

/// Shared builder state for recursive agent calls
#[derive(Clone)]
struct AgentBuilderState {
    name: String,
    text_to_text_model: TextToTextModel,
    text_embedding_model: Option<TextEmbeddingModel>,
    temperature: f64,
    max_tokens: Option<u64>,
    memory_read_timeout: Option<u64>,
    system_prompt: Option<String>,
    tools: ZeroOneOrMany<ToolInfo>,
    mcp_servers: Vec<McpServerConfig>,
    memory: Option<Arc<dyn crate::memory::core::manager::surreal::MemoryManager>>,
    on_conversation_turn_handler: Option<
        Arc<
            dyn Fn(
                    &CandleAgentConversation,
                    &CandleAgentRoleAgent,
                ) -> AsyncStream<CandleMessageChunk>
                + Send
                + Sync,
        >,
    >,
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
    fn additional_params<P>(self, params: P) -> impl CandleAgentRoleBuilder;

    /// Set memory - EXACT syntax: .memory(CandleLibrary::named("name"))
    #[must_use]
    fn memory<M>(self, memory: M) -> impl CandleAgentRoleBuilder
    where
        M: Into<Arc<dyn crate::memory::core::manager::surreal::MemoryManager>>;

    /// Set metadata - EXACT syntax: .metadata([("key", "value")])
    #[must_use]
    fn metadata<Meta>(self, metadata: Meta) -> impl CandleAgentRoleBuilder;

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
        F: Fn(&CandleAgentConversation, &CandleAgentRoleAgent) -> AsyncStream<CandleMessageChunk>
            + Send
            + Sync
            + 'static;

    /// Convert to agent - EXACT syntax: .into_agent()
    #[must_use]
    fn into_agent(self) -> impl CandleAgentBuilder;

    /// Set conversation history - EXACT syntax: .conversation_history(...)
    #[must_use]
    fn conversation_history(self, history: impl ConversationHistoryArgs) -> impl CandleAgentRoleBuilder;

    /// Chat with closure - EXACT syntax: .chat(|conversation| ChatLoop)
    fn chat<F>(self, handler: F) -> Result<AsyncStream<CandleMessageChunk>, AgentError>
    where
        F: FnOnce(&CandleAgentConversation) -> CandleChatLoop + Send + 'static;

    /// Chat with direct input - EXACT syntax: .chat_direct(ChatLoop)
    fn chat_direct(self, input: CandleChatLoop) -> AsyncStream<CandleMessageChunk>;

    /// Chat with message - EXACT syntax: .chat_with_message("message")
    fn chat_with_message(self, message: impl Into<String>) -> AsyncStream<CandleMessageChunk>;
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
    fn additional_params<P2>(self, params: P2) -> Self;

    /// Set memory manager - EXACT syntax: .memory(manager)
    #[must_use]
    fn memory<M>(self, memory: M) -> Self
    where
        M: Into<Arc<dyn crate::memory::core::manager::surreal::MemoryManager>>;

    /// Set metadata - EXACT syntax: .metadata(meta)
    #[must_use]
    fn metadata<Meta>(self, metadata: Meta) -> Self;

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
        F: Fn(&CandleAgentConversation, &CandleAgentRoleAgent) -> AsyncStream<CandleMessageChunk>
            + Send
            + Sync
            + 'static;

    /// Set conversation history - EXACT syntax from ARCHITECTURE.md
    /// Supports: .conversation_history(CandleMessageRole::User => "content", CandleMessageRole::System => "content", ...)
    #[must_use]
    fn conversation_history(self, history: impl ConversationHistoryArgs) -> Self;

    /// Chat with closure - EXACT syntax: .chat(|conversation| ChatLoop)
    fn chat<F>(self, handler: F) -> Result<AsyncStream<CandleMessageChunk>, AgentError>
    where
        F: FnOnce(&CandleAgentConversation) -> CandleChatLoop + Send + 'static;

    /// Chat with direct input - EXACT syntax: .chat_direct(ChatLoop)
    fn chat_direct(self, input: CandleChatLoop) -> AsyncStream<CandleMessageChunk>;

    /// Chat with message - EXACT syntax: .chat_with_message("message")
    fn chat_with_message(self, message: impl Into<String>) -> AsyncStream<CandleMessageChunk>;
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
    temperature: f64,
    max_tokens: Option<u64>,
    memory_read_timeout: Option<u64>,
    system_prompt: Option<String>,
    tools: ZeroOneOrMany<ToolInfo>,
    mcp_servers: Vec<McpServerConfig>,
    memory: Option<Arc<dyn crate::memory::core::manager::surreal::MemoryManager>>,
}

impl std::fmt::Debug for CandleAgentRoleBuilderImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CandleAgentRoleBuilderImpl")
            .field("name", &self.name)
            .field("temperature", &self.temperature)
            .field("max_tokens", &self.max_tokens)
            .field("memory_read_timeout", &self.memory_read_timeout)
            .field("system_prompt", &self.system_prompt)
            .field("tools", &self.tools)
            .field("mcp_servers", &self.mcp_servers)
            .field("memory", &self.memory.as_ref().map(|_| "<MemoryManager>"))
            .finish()
    }
}

impl CandleAgentRoleBuilderImpl {
    /// Create a new agent role builder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            temperature: 0.7, // Default temperature to 0.7
            max_tokens: None,
            memory_read_timeout: None,
            system_prompt: None,
            tools: ZeroOneOrMany::none(),
            mcp_servers: Vec::new(),
            memory: None,
        }
    }
}

// Implementation for no-provider builder
impl CandleAgentRoleBuilder for CandleAgentRoleBuilderImpl {
    fn new(name: impl Into<String>) -> impl CandleAgentRoleBuilder {
        CandleAgentRoleBuilderImpl::new(name)
    }

    fn model(self, model: TextToTextModel) -> impl CandleAgentRoleBuilder {
        CandleAgentBuilderImpl {
            name: self.name,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            memory_read_timeout: self.memory_read_timeout,
            system_prompt: self.system_prompt,
            text_to_text_model: model,
            text_embedding_model: None,
            tools: self.tools,
            mcp_servers: self.mcp_servers,
            memory: self.memory,
            on_conversation_turn_handler: None,
        }
    }

    fn embedding_model(self, model: TextEmbeddingModel) -> impl CandleAgentRoleBuilder {
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
        self.memory_read_timeout = Some(timeout_ms);
        self
    }

    /// Set system prompt - EXACT syntax: .system_prompt("...")
    fn system_prompt(mut self, prompt: impl Into<String>) -> impl CandleAgentRoleBuilder {
        self.system_prompt = Some(prompt.into());
        self
    }

    /// Set additional params - EXACT syntax: .additional_params([("key", "value")])
    fn additional_params<P>(self, _params: P) -> impl CandleAgentRoleBuilder {
        self
    }

    /// Set memory - EXACT syntax: .memory(CandleLibrary::named("name"))
    fn memory<M>(mut self, memory: M) -> impl CandleAgentRoleBuilder
    where
        M: Into<Arc<dyn crate::memory::core::manager::surreal::MemoryManager>>,
    {
        self.memory = Some(memory.into());
        self
    }

    /// Set metadata - EXACT syntax: .metadata([("key", "value")])
    fn metadata<Meta>(self, _metadata: Meta) -> impl CandleAgentRoleBuilder {
        self
    }

    /// Set contexts - EXACT syntax: .context(CandleContext::<CandleFile>::of("/path"), ...)
    fn context(
        self,
        _context1: CandleContext<CandleFile>,
        _context2: CandleContext<CandleFiles>,
        _context3: CandleContext<CandleDirectory>,
        _context4: CandleContext<CandleGithub>,
    ) -> impl CandleAgentRoleBuilder {
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
    fn add_mcp_server_config(mut self, config: McpServerConfig) -> impl CandleAgentRoleBuilder {
        self.mcp_servers.push(config);
        self
    }

    /// Set chunk handler - EXACT syntax: .on_chunk(|chunk| chunk)
    fn on_chunk<F>(self, _handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(CandleMessageChunk) -> CandleMessageChunk + Send + Sync + 'static,
    {
        self
    }

    /// Set tool result handler - EXACT syntax: .on_tool_result(|results| { ... })
    fn on_tool_result<F>(self, _handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(&[String]) + Send + Sync + 'static,
    {
        self
    }

    /// Set conversation turn handler - EXACT syntax: .on_conversation_turn(|conversation, agent| { ... })
    fn on_conversation_turn<F>(self, _handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(&CandleAgentConversation, &CandleAgentRoleAgent) -> AsyncStream<CandleMessageChunk>
            + Send
            + Sync
            + 'static,
    {
        // Cannot set handler without a model - return self unchanged
        self
    }

    fn conversation_history(self, _history: impl ConversationHistoryArgs) -> impl CandleAgentRoleBuilder {
        self
    }

    /// Chat with closure - EXACT syntax: .chat(|conversation| ChatLoop)
    fn chat<F>(self, _handler: F) -> Result<AsyncStream<CandleMessageChunk>, AgentError>
    where
        F: FnOnce(&CandleAgentConversation) -> CandleChatLoop + Send + 'static,
    {
        Ok(AsyncStream::with_channel(|sender| {
            let _ = sender.send(CandleMessageChunk::Error(
                "No provider configured".to_string(),
            ));
        }))
    }

    fn chat_direct(self, input: CandleChatLoop) -> AsyncStream<CandleMessageChunk> {
        AsyncStream::with_channel(move |sender| match input {
            CandleChatLoop::Break => {
                let final_chunk = CandleMessageChunk::Complete {
                    text: String::new(),
                    finish_reason: Some("break".to_string()),
                    usage: None,
                };
                let _ = sender.send(final_chunk);
            }
            CandleChatLoop::UserPrompt(message) | CandleChatLoop::Reprompt(message) => {
                let _ = sender.send(CandleMessageChunk::Error(format!(
                    "No provider configured for message: {}",
                    message
                )));
            }
        })
    }

    fn chat_with_message(self, message: impl Into<String>) -> AsyncStream<CandleMessageChunk> {
        let msg = message.into();
        AsyncStream::with_channel(move |sender| {
            let _ = sender.send(CandleMessageChunk::Error(format!(
                "No provider configured for message: {}",
                msg
            )));
        })
    }

    /// Convert to agent - EXACT syntax: .into_agent()
    fn into_agent(self) -> impl CandleAgentBuilder {
        // Get default model from registry - use Phi4 as default
        use crate::capability::text_to_text::CandlePhi4ReasoningModel;
        use std::sync::Arc;
        
        let default_model = TextToTextModel::Phi4Reasoning(Arc::new(CandlePhi4ReasoningModel::default()));

        CandleAgentBuilderImpl {
            name: self.name,
            text_to_text_model: default_model,
            text_embedding_model: None,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            memory_read_timeout: self.memory_read_timeout,
            system_prompt: self.system_prompt,
            tools: self.tools,
            mcp_servers: self.mcp_servers,
            memory: self.memory,
            on_conversation_turn_handler: None,
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
    temperature: f64,
    max_tokens: Option<u64>,
    memory_read_timeout: Option<u64>,
    system_prompt: Option<String>,
    text_to_text_model: TextToTextModel,
    text_embedding_model: Option<TextEmbeddingModel>,
    tools: ZeroOneOrMany<ToolInfo>,
    mcp_servers: Vec<McpServerConfig>,
    memory: Option<Arc<dyn crate::memory::core::manager::surreal::MemoryManager>>,
    on_conversation_turn_handler: Option<
        Arc<
            dyn Fn(
                    &CandleAgentConversation,
                    &CandleAgentRoleAgent,
                ) -> AsyncStream<CandleMessageChunk>
                + Send
                + Sync,
        >,
    >,
}

impl std::fmt::Debug for CandleAgentBuilderImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CandleAgentBuilderImpl")
            .field("name", &self.name)
            .field("temperature", &self.temperature)
            .field("max_tokens", &self.max_tokens)
            .field("memory_read_timeout", &self.memory_read_timeout)
            .field("system_prompt", &self.system_prompt)
            .field("text_to_text_model", &self.text_to_text_model)
            .field("text_embedding_model", &self.text_embedding_model)
            .field("tools", &self.tools)
            .field("mcp_servers", &self.mcp_servers)
            .field("memory", &self.memory.as_ref().map(|_| "<MemoryManager>"))
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
        self.text_embedding_model = Some(model);
        self
    }

    fn temperature(mut self, temp: f64) -> impl CandleAgentRoleBuilder {
        self.temperature = temp;
        self
    }

    fn max_tokens(mut self, max: u64) -> impl CandleAgentRoleBuilder {
        self.max_tokens = Some(max);
        self
    }

    fn memory_read_timeout(mut self, timeout_ms: u64) -> impl CandleAgentRoleBuilder {
        self.memory_read_timeout = Some(timeout_ms);
        self
    }

    fn system_prompt(mut self, prompt: impl Into<String>) -> impl CandleAgentRoleBuilder {
        self.system_prompt = Some(prompt.into());
        self
    }

    fn additional_params<P2>(self, _params: P2) -> impl CandleAgentRoleBuilder {
        self
    }

    fn memory<M>(mut self, memory: M) -> impl CandleAgentRoleBuilder
    where
        M: Into<Arc<dyn crate::memory::core::manager::surreal::MemoryManager>>,
    {
        self.memory = Some(memory.into());
        self
    }

    fn metadata<Meta>(self, _metadata: Meta) -> impl CandleAgentRoleBuilder {
        self
    }

    fn context(
        self,
        _context1: CandleContext<CandleFile>,
        _context2: CandleContext<CandleFiles>,
        _context3: CandleContext<CandleDirectory>,
        _context4: CandleContext<CandleGithub>,
    ) -> impl CandleAgentRoleBuilder {
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
    fn add_mcp_server_config(mut self, config: McpServerConfig) -> impl CandleAgentRoleBuilder {
        self.mcp_servers.push(config);
        self
    }

    fn on_chunk<F>(self, _handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(CandleMessageChunk) -> CandleMessageChunk + Send + Sync + 'static,
    {
        self
    }

    fn on_tool_result<F>(self, _handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(&[String]) + Send + Sync + 'static,
    {
        self
    }

    /// Set conversation turn handler - EXACT syntax: .on_conversation_turn(|conversation, agent| { ... })
    fn on_conversation_turn<F>(mut self, handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(&CandleAgentConversation, &CandleAgentRoleAgent) -> AsyncStream<CandleMessageChunk>
            + Send
            + Sync
            + 'static,
    {
        self.on_conversation_turn_handler = Some(Arc::new(handler));
        self
    }

    fn conversation_history(self, _history: impl ConversationHistoryArgs) -> impl CandleAgentRoleBuilder {
        self
    }

    /// Chat with closure - EXACT syntax: .chat(|conversation| ChatLoop)
    fn chat<F>(self, _handler: F) -> Result<AsyncStream<CandleMessageChunk>, AgentError>
    where
        F: FnOnce(&CandleAgentConversation) -> CandleChatLoop + Send + 'static,
    {
        Ok(AsyncStream::with_channel(|sender| {
            let _ = sender.send(CandleMessageChunk::Text("Hello from Candle!".to_string()));
        }))
    }

    fn chat_direct(self, input: CandleChatLoop) -> AsyncStream<CandleMessageChunk> {
        match input {
            CandleChatLoop::Break => AsyncStream::with_channel(move |sender| {
                let final_chunk = CandleMessageChunk::Complete {
                    text: String::new(),
                    finish_reason: Some("break".to_string()),
                    usage: None,
                };
                let _ = sender.send(final_chunk);
            }),
            CandleChatLoop::UserPrompt(message) | CandleChatLoop::Reprompt(message) => {
                // Use CandleAgentRoleBuilder::chat explicitly to avoid ambiguity
                CandleAgentRoleBuilder::chat(self, move |_| CandleChatLoop::UserPrompt(message))
                    .unwrap_or_else(|_| AsyncStream::with_channel(|sender| {
                        let _ = sender.send(CandleMessageChunk::Error("Chat failed".to_string()));
                    }))
            }
        }
    }

    fn chat_with_message(self, message: impl Into<String>) -> AsyncStream<CandleMessageChunk> {
        let msg = message.into();
        // Use CandleAgentRoleBuilder::chat explicitly to avoid ambiguity
        CandleAgentRoleBuilder::chat(self, move |_| CandleChatLoop::UserPrompt(msg))
            .unwrap_or_else(|_| AsyncStream::with_channel(|sender| {
                let _ = sender.send(CandleMessageChunk::Error("Chat failed".to_string()));
            }))
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
        self.text_embedding_model = Some(model);
        self
    }

    fn temperature(mut self, temp: f64) -> Self {
        self.temperature = temp;
        self
    }

    fn max_tokens(mut self, max: u64) -> Self {
        self.max_tokens = Some(max);
        self
    }

    fn memory_read_timeout(mut self, timeout_ms: u64) -> Self {
        self.memory_read_timeout = Some(timeout_ms);
        self
    }

    fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    fn additional_params<P2>(self, _params: P2) -> Self {
        self
    }

    fn memory<M>(mut self, memory: M) -> Self
    where
        M: Into<Arc<dyn crate::memory::core::manager::surreal::MemoryManager>>,
    {
        self.memory = Some(memory.into());
        self
    }

    fn metadata<Meta>(self, _metadata: Meta) -> Self {
        self
    }

    fn context(
        self,
        _context1: CandleContext<CandleFile>,
        _context2: CandleContext<CandleFiles>,
        _context3: CandleContext<CandleDirectory>,
        _context4: CandleContext<CandleGithub>,
    ) -> Self {
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

    fn add_mcp_server_config(mut self, config: McpServerConfig) -> Self {
        self.mcp_servers.push(config);
        self
    }

    fn on_chunk<F>(self, _handler: F) -> Self
    where
        F: Fn(CandleMessageChunk) -> CandleMessageChunk + Send + Sync + 'static,
    {
        self
    }

    fn on_tool_result<F>(self, _handler: F) -> Self
    where
        F: Fn(&[String]) + Send + Sync + 'static,
    {
        self
    }

    fn on_conversation_turn<F>(mut self, handler: F) -> Self
    where
        F: Fn(&CandleAgentConversation, &CandleAgentRoleAgent) -> AsyncStream<CandleMessageChunk>
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
    fn chat<F>(self, handler: F) -> Result<AsyncStream<CandleMessageChunk>, AgentError>
    where
        F: FnOnce(&CandleAgentConversation) -> CandleChatLoop + Send + 'static,
    {
        let provider = self.text_to_text_model;
        let temperature = self.temperature;
        let max_tokens = self.max_tokens.unwrap_or(1000);
        let _memory_read_timeout = self.memory_read_timeout;
        let system_prompt = self.system_prompt.clone();
        let tools = self.tools;
        let _mcp_servers = self.mcp_servers;
        let memory = self.memory;
        let on_conversation_turn_handler = self.on_conversation_turn_handler;

        Ok(AsyncStream::with_channel(move |sender| {
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
                    };
                    let _ = sender.send(final_chunk);
                }
                CandleChatLoop::UserPrompt(user_message)
                | CandleChatLoop::Reprompt(user_message) => {
                    // Spawn stream to handle operations (uses shared runtime for async)
                    // BLOCKING CODE APPROVED: Using shared_runtime().block_on() for async operations within ystream closure (2025-01-XX)
                    let _background_stream = ystream::spawn_stream(move |stream_sender| {
                        // Create conversation with real user input for this inference
                        let _conversation_with_input =
                            CandleAgentConversation::with_user_input(&user_message);

                        // Initialize tool router - ALWAYS create with default reasoner plugin
                        let tool_router = {
                            let Some(runtime) = crate::runtime::shared_runtime() else {
                                let error_chunk = CandleMessageChunk::Error(
                                    "Failed to access shared runtime".to_string(),
                                );
                                ystream::emit!(stream_sender, error_chunk);
                                return;
                            };

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

                            match runtime.block_on(router.initialize()) {
                                Ok(()) => Some(router),
                                Err(e) => {
                                    let error_chunk = CandleMessageChunk::Error(format!(
                                        "Tool initialization failed: {}",
                                        e
                                    ));
                                    ystream::emit!(stream_sender, error_chunk);
                                    return;
                                }
                            }
                        };

                        // Memory context injection
                        let memory_context: Option<String> = if let Some(ref mem_manager) = memory {
                            let memory_stream = mem_manager.search_by_content(&user_message);

                            // Use tokio::task::block_in_place pattern (matches domain/agent/chat.rs:460)
                            let results: Vec<MemoryResult<MemoryNode>> =
                                tokio::task::block_in_place(|| {
                                    tokio::runtime::Handle::current().block_on(async {
                                        use futures_util::StreamExt;
                                        memory_stream.take(5).collect().await
                                    })
                                });

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

                        // Create prompt with memory context and system prompt if provided
                        let full_prompt = match (memory_context, &system_prompt) {
                            (Some(mem_ctx), Some(sys_prompt)) => {
                                format!("{}\n\n{}\n\nUser: {}", sys_prompt, mem_ctx, user_message)
                            }
                            (Some(mem_ctx), None) => {
                                format!("{}\n\nUser: {}", mem_ctx, user_message)
                            }
                            (None, Some(sys_prompt)) => {
                                format!("{}\n\nUser: {}", sys_prompt, user_message)
                            }
                            (None, None) => {
                                format!("User: {}", user_message)
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

                            // Try to get auto-generated tools if runtime is available
                            if let Some(runtime) = crate::runtime::shared_runtime() {
                                let auto_generated_tools =
                                    runtime.block_on(router.get_available_tools());
                                all_tools.extend(auto_generated_tools);
                            }

                            if !all_tools.is_empty() {
                                // Pass merged tools to completion system for function calling
                                params.tools = Some(ZeroOneOrMany::from(all_tools));
                            }
                        }

                        // Call REAL provider inference
                        let completion_stream = provider.prompt(prompt, &params);

                        // Convert CandleCompletionChunk to CandleMessageChunk and forward
                        // Handle tool calls if they occur
                        let completion_results = completion_stream.collect();
                        let mut assistant_response = String::new();

                        for completion_chunk in completion_results {
                            let message_chunk = match completion_chunk {
                                CandleCompletionChunk::Text(ref text) => {
                                    assistant_response.push_str(text);
                                    CandleMessageChunk::Text(text.clone())
                                }
                                CandleCompletionChunk::Complete {
                                    ref text,
                                    finish_reason,
                                    usage,
                                } => {
                                    assistant_response.push_str(text);
                                    CandleMessageChunk::Complete {
                                        text: text.clone(),
                                        finish_reason: finish_reason.map(|f| format!("{:?}", f)),
                                        usage: usage.map(|u| format!("{:?}", u)),
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

                                                // Execute the tool if runtime is available
                                                match crate::runtime::shared_runtime() {
                                                    Some(runtime) => {
                                                        match runtime.block_on(
                                                            router.call_tool(&name, sweet_args),
                                                        ) {
                                                            Ok(response) => {
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
                                                    None => CandleMessageChunk::Error(
                                                        "Runtime unavailable for tool execution"
                                                            .to_string(),
                                                    ),
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

                            ystream::emit!(stream_sender, message_chunk);
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
                                text_embedding_model: None,
                                temperature,
                                max_tokens: Some(max_tokens),
                                memory_read_timeout: _memory_read_timeout,
                                system_prompt: system_prompt.clone(),
                                tools: tools.clone(),
                                mcp_servers: _mcp_servers.clone(),
                                memory: memory.clone(),
                                on_conversation_turn_handler: Some(handler.clone()),
                            });

                            // Create agent with full state
                            let agent = CandleAgentRoleAgent {
                                state: builder_state,
                            };

                            // Call handler and forward its stream
                            let handler_stream = handler(&conversation, &agent);
                            let handler_chunks = handler_stream.collect();
                            for chunk in handler_chunks {
                                ystream::emit!(stream_sender, chunk);
                            }
                        }
                    });
                }
            }
        }))
    }

    fn chat_direct(self, input: CandleChatLoop) -> AsyncStream<CandleMessageChunk> {
        AsyncStream::with_channel(move |sender| match input {
            CandleChatLoop::Break => {
                let final_chunk = CandleMessageChunk::Complete {
                    text: String::new(),
                    finish_reason: Some("break".to_string()),
                    usage: None,
                };
                let _ = sender.send(final_chunk);
            }
            CandleChatLoop::UserPrompt(message) | CandleChatLoop::Reprompt(message) => {
                let _ = sender.send(CandleMessageChunk::Text(message));
            }
        })
    }

    fn chat_with_message(self, message: impl Into<String>) -> AsyncStream<CandleMessageChunk> {
        let provider = self.text_to_text_model;
        let temperature = self.temperature;
        let max_tokens = self.max_tokens.unwrap_or(1000);
        let system_prompt = self.system_prompt.clone();
        let user_message = message.into();

        AsyncStream::with_channel(move |_sender| {
            let full_prompt = if let Some(sys_prompt) = system_prompt {
                format!("{}\n\nUser: {}", sys_prompt, user_message)
            } else {
                format!("User: {}", user_message)
            };

            let prompt = CandlePrompt::new(full_prompt);
            let params = CandleCompletionParams {
                temperature,
                max_tokens: NonZeroU64::new(max_tokens),
                ..Default::default()
            };

            let completion_stream = provider.prompt(prompt, &params);

            // Use ystream spawn pattern instead of tokio::spawn for proper thread safety
            let _background_stream = ystream::spawn_stream(move |stream_sender| {
                let completion_results = completion_stream.collect();
                for completion_chunk in completion_results {
                    let message_chunk = match completion_chunk {
                        CandleCompletionChunk::Text(text) => CandleMessageChunk::Text(text),
                        CandleCompletionChunk::Complete {
                            text,
                            finish_reason,
                            usage,
                        } => CandleMessageChunk::Complete {
                            text,
                            finish_reason: finish_reason.map(|f| format!("{:?}", f)),
                            usage: usage.map(|u| format!("{:?}", u)),
                        },
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

                    ystream::emit!(stream_sender, message_chunk);
                }
            });
        })
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
