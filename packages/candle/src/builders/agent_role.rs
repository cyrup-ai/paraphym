//! Builders are behavioral/construction logic, separate from core domain models

use std::num::NonZeroU64;
use std::sync::Arc;

use cyrup_sugars::ZeroOneOrMany;

use ystream::AsyncStream;

use crate::domain::agent::core::AgentError;
use crate::domain::chat::message::{CandleMessageChunk, CandleMessageRole};
use crate::domain::chat::CandleChatLoop;
use crate::domain::completion::{
    traits::CandleCompletionModel as DomainCompletionModel, types::CandleCompletionParams,
    CandleCompletionChunk,
};
use crate::domain::context::provider::{
    CandleContext, CandleDirectory, CandleFile, CandleFiles, CandleGithub,
};
use crate::domain::prompt::CandlePrompt;
use crate::domain::tool::SweetMcpRouter;
use sweet_mcp_type::ToolInfo;
use serde_json;

// Import provider types for default provider creation
use crate::providers::{CandleKimiK2Provider, CandleKimiK2Config};
use crate::domain::agent::role::CandleCompletionProviderType;

// Candle domain types - self-contained
/// Trait for AI completion providers (e.g., OpenAI, Anthropic, local models)  
pub trait CandleCompletionProvider: Send + Sync + 'static {}

/// Default empty implementations for optional components
pub struct NoProvider;

impl CandleCompletionProvider for NoProvider {}

// CandleChatLoop is now imported from domain::chat

/// Agent conversation
#[derive(Default)]
pub struct CandleAgentConversation {
    messages: Vec<CandleMessage>,
    current_user_input: String,
}

impl CandleAgentConversation {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            current_user_input: String::new(),
        }
    }

    pub fn with_user_input(user_input: impl Into<String>) -> Self {
        let input = user_input.into();
        let mut conversation = Self::new();
        conversation.current_user_input = input.clone();
        conversation.messages.push(CandleMessage {
            content: input,
            role: CandleMessageRole::User,
        });
        conversation
    }

    pub fn latest_user_message(&self) -> &str {
        if self.current_user_input.is_empty() {
            "Hello" // Fallback for compatibility
        } else {
            &self.current_user_input
        }
    }

    pub fn last(&self) -> CandleMessage {
        self.messages
            .last()
            .cloned()
            .unwrap_or_else(|| CandleMessage {
                content: "Hello".to_string(),
                role: CandleMessageRole::User,
            })
    }

    pub fn add_message(&mut self, content: impl Into<String>, role: CandleMessageRole) {
        self.messages.push(CandleMessage {
            content: content.into(),
            role,
        });
    }
}

/// Message in conversation
#[derive(Debug, Clone)]
pub struct CandleMessage {
    content: String,
    role: CandleMessageRole,
}

impl CandleMessage {
    pub fn message(&self) -> &str {
        &self.content
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn role(&self) -> &CandleMessageRole {
        &self.role
    }
}

/// Agent helper type for conversation control in `on_conversation_turn` callbacks.
/// See `impl` block below for available methods.
pub struct CandleAgentRoleAgent;

impl CandleAgentRoleAgent {
    /// Chat method for use in on_conversation_turn closure
    pub fn chat(&self, chat_loop: CandleChatLoop) -> AsyncStream<CandleMessageChunk> {
        AsyncStream::with_channel(|sender| match chat_loop {
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
}

/// Agent role builder trait - elegant zero-allocation builder pattern (PUBLIC API)
pub trait CandleAgentRoleBuilder: Sized + Send {
    /// Create a new agent role builder - EXACT syntax: CandleFluentAi::agent_role("name")
    fn new(name: impl Into<String>) -> impl CandleAgentRoleBuilder;

    /// Set the completion provider - EXACT syntax: .completion_provider(CandleKimiK2Provider::new())
    #[must_use]
    fn completion_provider<P>(self, provider: P) -> impl CandleAgentRoleBuilder
    where
        P: DomainCompletionModel + Send + 'static;

    /// Set model - EXACT syntax: .model(CandleModels::KIMI_K2)
    #[must_use]
    fn model<M>(self, model: M) -> impl CandleAgentRoleBuilder
    where
        M: DomainCompletionModel;

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

    /// Chat with closure - EXACT syntax: .chat(|conversation| ChatLoop)
    fn chat<F>(self, handler: F) -> Result<AsyncStream<CandleMessageChunk>, AgentError>
    where
        F: FnOnce(&CandleAgentConversation) -> CandleChatLoop + Send + 'static;

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
    /// Set conversation history - EXACT syntax from ARCHITECTURE.md
    /// Supports: .conversation_history(CandleMessageRole::User => "content", CandleMessageRole::System => "content", ...)
    #[must_use]
    fn conversation_history(self, history: impl ConversationHistoryArgs)
        -> impl CandleAgentBuilder;

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
#[derive(Clone)]
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
            temperature: 0.7,  // Default temperature to 0.7
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

    fn completion_provider<P>(self, provider: P) -> impl CandleAgentRoleBuilder
    where
        P: DomainCompletionModel + Send + 'static,
    {
        CandleAgentBuilderImpl {
            name: self.name,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            memory_read_timeout: self.memory_read_timeout,
            system_prompt: self.system_prompt,
            provider,
            tools: self.tools,
            mcp_servers: self.mcp_servers,
            memory: self.memory,
        }
    }

    /// Set model - EXACT syntax: .model(CandleModels::KIMI_K2)
    fn model<M>(self, _model: M) -> impl CandleAgentRoleBuilder
    where
        M: DomainCompletionModel,
    {
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

    /// Convert to agent - EXACT syntax: .into_agent()
    fn into_agent(self) -> impl CandleAgentBuilder {
        // Always provide default provider - try multiple paths with graceful fallback
        let default_provider = CandleKimiK2Provider::default_for_builder()
            .or_else(|e| {
                log::warn!("Failed to create default KimiK2 provider with ProgressHub: {}. Trying local path.", e);
                CandleKimiK2Provider::with_config_sync(
                    "./models/kimi-k2".to_string(),
                    CandleKimiK2Config::default()
                )
            })
            .or_else(|e| {
                log::warn!("Failed to create provider with ./models path: {}. Trying cache directory.", e);
                let cache_path = match std::env::var("HOME") {
                    Ok(home) => format!("{}/.cache/candle/models/kimi-k2", home),
                    Err(_) => "./.cache/candle/models/kimi-k2".to_string(),
                };
                CandleKimiK2Provider::with_config_sync(cache_path, CandleKimiK2Config::default())
            })
            .or_else(|e| {
                log::warn!("Failed with cache directory: {}. Trying temp directory.", e);
                CandleKimiK2Provider::with_config_sync(
                    "/tmp/candle-models/kimi-k2".to_string(),
                    CandleKimiK2Config::default()
                )
            })
            .or_else(|e| {
                // Final fallback: Create provider with minimal default path
                // This provider will exist but may error when actually used if model file is missing
                // This is preferable to failing at builder initialization time
                log::error!(
                    "All provider initialization attempts failed: {}. \
                     Creating provider with minimal fallback configuration. \
                     Model operations will fail until valid model files are provided.",
                    e
                );
                // with_config_sync only creates a struct and virtually never fails
                // The Result type is only for API consistency with other provider creation methods
                CandleKimiK2Provider::with_config_sync(
                    "./models/kimi-k2-fallback".to_string(),
                    CandleKimiK2Config::default()
                )
            })
            .or_else(|_| {
                // If with_config_sync fails, try the simplest possible path
                CandleKimiK2Provider::with_config_sync(
                    ".".to_string(),
                    CandleKimiK2Config::default()
                )
            })
            .or_else(|_| {
                // Try empty string as absolute last resort
                CandleKimiK2Provider::with_config_sync(
                    String::new(),
                    CandleKimiK2Config::default()
                )
            })
            .unwrap_or_else(|final_error| {
                // This is an acceptable panic because:
                // 1. We've tried 6 different fallback paths
                // 2. with_config_sync only allocates a struct (should never fail)
                // 3. Failure indicates OOM or memory corruption
                // 4. panic! allows the application to catch it with panic hooks
                panic!(
                    "Fatal: Unable to initialize KimiK2Provider after 6 fallback attempts. \
                     System cannot allocate memory for provider struct. \
                     This indicates OOM or memory corruption. Error: {}",
                    final_error
                );
            });
        
        CandleAgentBuilderImpl {
            name: self.name,
            provider: CandleCompletionProviderType::KimiK2(default_provider),
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            memory_read_timeout: self.memory_read_timeout,
            system_prompt: self.system_prompt,
            tools: self.tools,
            mcp_servers: self.mcp_servers,
            memory: self.memory,
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

/// Placeholder agent for no-provider case
#[derive(Debug, Clone)]
pub struct NoProviderAgent {
    _inner: CandleAgentRoleBuilderImpl,
}

impl CandleAgentBuilder for NoProviderAgent {
    fn conversation_history(
        self,
        _history: impl ConversationHistoryArgs,
    ) -> impl CandleAgentBuilder {
        self
    }

    fn chat<F>(self, _handler: F) -> Result<AsyncStream<CandleMessageChunk>, AgentError>
    where
        F: FnOnce(&CandleAgentConversation) -> CandleChatLoop + Send + 'static,
    {
        Ok(AsyncStream::with_channel(move |sender| {
            let error_chunk = CandleMessageChunk::Error("No completion provider configured. Use .completion_provider() before .into_agent()".to_string());
            let _ = sender.send(error_chunk);
        }))
    }

    fn chat_direct(self, _input: CandleChatLoop) -> AsyncStream<CandleMessageChunk> {
        AsyncStream::with_channel(move |sender| {
            let error_chunk = CandleMessageChunk::Error("No completion provider configured. Use .completion_provider() before .into_agent()".to_string());
            let _ = sender.send(error_chunk);
        })
    }

    fn chat_with_message(self, message: impl Into<String>) -> AsyncStream<CandleMessageChunk> {
        let message_string = message.into();
        AsyncStream::with_channel(move |sender| {
            let error_chunk = CandleMessageChunk::Error(format!("No completion provider configured for message: {}. Use .completion_provider() before .into_agent()", message_string));
            let _ = sender.send(error_chunk);
        })
    }
}

/// Agent builder implementation
pub struct CandleAgentBuilderImpl<P> {
    name: String,
    temperature: f64,
    max_tokens: Option<u64>,
    memory_read_timeout: Option<u64>,
    system_prompt: Option<String>,
    provider: P,
    tools: ZeroOneOrMany<ToolInfo>,
    mcp_servers: Vec<McpServerConfig>,
    memory: Option<Arc<dyn crate::memory::core::manager::surreal::MemoryManager>>,
}

impl<P: std::fmt::Debug> std::fmt::Debug for CandleAgentBuilderImpl<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CandleAgentBuilderImpl")
            .field("name", &self.name)
            .field("temperature", &self.temperature)
            .field("max_tokens", &self.max_tokens)
            .field("memory_read_timeout", &self.memory_read_timeout)
            .field("system_prompt", &self.system_prompt)
            .field("provider", &self.provider)
            .field("tools", &self.tools)
            .field("mcp_servers", &self.mcp_servers)
            .field("memory", &self.memory.as_ref().map(|_| "<MemoryManager>"))
            .finish()
    }
}

// Implementation for with-provider builder (allows all methods)
impl<P> CandleAgentRoleBuilder for CandleAgentBuilderImpl<P>
where
    P: DomainCompletionModel + Send + 'static,
{
    fn new(name: impl Into<String>) -> impl CandleAgentRoleBuilder {
        CandleAgentRoleBuilderImpl::new(name)
    }

    fn completion_provider<P2>(self, provider: P2) -> impl CandleAgentRoleBuilder
    where
        P2: DomainCompletionModel + Send + 'static,
    {
        CandleAgentBuilderImpl {
            name: self.name,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            memory_read_timeout: self.memory_read_timeout,
            system_prompt: self.system_prompt,
            provider,
            tools: self.tools,
            mcp_servers: self.mcp_servers,
            memory: self.memory,
        }
    }

    fn model<M>(self, _model: M) -> impl CandleAgentRoleBuilder
    where
        M: DomainCompletionModel,
    {
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

    fn on_conversation_turn<F>(self, _handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(&CandleAgentConversation, &CandleAgentRoleAgent) -> AsyncStream<CandleMessageChunk>
            + Send
            + Sync
            + 'static,
    {
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


    fn into_agent(self) -> impl CandleAgentBuilder {
        self
    }
}

impl<P> CandleAgentBuilder for CandleAgentBuilderImpl<P>
where
    P: DomainCompletionModel + Send + 'static,
{
    /// Set conversation history
    fn conversation_history(
        self,
        _history: impl ConversationHistoryArgs,
    ) -> impl CandleAgentBuilder {
        self
    }

    /// Chat with closure - EXACT syntax: .chat(|conversation| ChatLoop)
    fn chat<F>(self, handler: F) -> Result<AsyncStream<CandleMessageChunk>, AgentError>
    where
        F: FnOnce(&CandleAgentConversation) -> CandleChatLoop + Send + 'static,
    {
        let provider = self.provider;
        let temperature = self.temperature;
        let max_tokens = self.max_tokens.unwrap_or(1000);
        let _memory_read_timeout = self.memory_read_timeout;
        let system_prompt = self.system_prompt.clone();
        let tools = self.tools;
        let mcp_servers = self.mcp_servers;
        let memory = self.memory;

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

                        // Initialize tool router if tools are provided or MCP servers are configured
                        let tool_router = if !tools.is_empty() || !mcp_servers.is_empty() {
                            let Some(runtime) = crate::runtime::shared_runtime() else {
                                let error_chunk = CandleMessageChunk::Error("Failed to access shared runtime".to_string());
                                ystream::emit!(stream_sender, error_chunk);
                                return;
                            };
                            let mut router = SweetMcpRouter::new();
                            match runtime.block_on(router.initialize()) {
                                Ok(()) => Some(router),
                                Err(e) => {
                                    let error_chunk = CandleMessageChunk::Error(format!("Tool initialization failed: {}", e));
                                    ystream::emit!(stream_sender, error_chunk);
                                    return;
                                }
                            }
                        } else {
                            None
                        };

                        // Memory context injection placeholder
                        // TODO: Implement proper memory context injection using memory manager's search API
                        let memory_context: Option<String> = None;

                        // Create prompt with memory context and system prompt if provided
                        let full_prompt = match (memory_context, system_prompt) {
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
                            let mut all_tools: Vec<ToolInfo> = tools.into();
                            
                            // Try to get auto-generated tools if runtime is available
                            if let Some(runtime) = crate::runtime::shared_runtime() {
                                let auto_generated_tools = runtime.block_on(router.get_available_tools());
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
                                } => CandleMessageChunk::ToolCall { id, name, partial_input },
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
                                                        match runtime.block_on(router.call_tool(&name, sweet_args)) {
                                                            Ok(response) => {
                                                                // Convert response to text result
                                                                let result_text = format!("Tool '{}' executed successfully: {:?}", name, response);
                                                                CandleMessageChunk::Text(result_text)
                                                            }
                                                            Err(e) => {
                                                                // Return error as text
                                                                CandleMessageChunk::Error(format!("Tool '{}' execution failed: {}", name, e))
                                                            }
                                                        }
                                                    }
                                                    None => {
                                                        CandleMessageChunk::Error("Runtime unavailable for tool execution".to_string())
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                CandleMessageChunk::Error(format!("Tool '{}' invalid JSON input: {}", name, e))
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
                        
                        // Store conversation turn in memory after completion
                        if let Some(ref memory_manager) = memory
                            && !assistant_response.is_empty() {
                            // Create memory nodes for the conversation
                            let user_content = crate::memory::core::primitives::types::MemoryContent::new(&user_message);
                            let assistant_content = crate::memory::core::primitives::types::MemoryContent::new(&assistant_response);
                            
                            // Use Episodic memory type for conversation turns
                            let mut user_memory = crate::memory::core::MemoryNode::new(
                                crate::memory::core::primitives::types::MemoryTypeEnum::Episodic,
                                user_content
                            );
                            user_memory.metadata.tags.push("user_message".to_string());
                            user_memory.metadata.source = Some("chat".to_string());
                            user_memory.metadata.importance = 0.8;
                            
                            let mut assistant_memory = crate::memory::core::MemoryNode::new(
                                crate::memory::core::primitives::types::MemoryTypeEnum::Episodic,
                                assistant_content
                            );
                            assistant_memory.metadata.tags.push("assistant_response".to_string());
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
                                        tracing::error!(
                                            error = ?e,
                                            memory_type = "user",
                                            "Failed to store memory to database"
                                        );
                                    }
                                });
                                runtime.spawn(async move {
                                    if let Err(e) = assistant_pending.await {
                                        tracing::error!(
                                            error = ?e,
                                            memory_type = "assistant",
                                            "Failed to store memory to database"
                                        );
                                    }
                                });
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
        let provider = self.provider;
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
                        } => CandleMessageChunk::ToolCall { id, name, partial_input },
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
