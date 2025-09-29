//! Candle Agent role trait and implementation - EXACT REPLICA of domain with Candle prefixes

use std::fmt;
use std::sync::{atomic::AtomicUsize, LazyLock};

// Ultra-high-performance zero-allocation imports
use atomic_counter::RelaxedCounter;
use crossbeam_utils::CachePadded;
use cyrup_sugars::ZeroOneOrMany;
use hashbrown::HashMap;
use crate::builders::document::DocumentBuilder;

use serde_json::Value;

use crate::domain::chat::CandleMessageRole;
use crate::domain::completion::traits::CandleCompletionModel;
use crate::providers::{CandleKimiK2Provider, CandleQwen3CoderProvider};
use crate::domain::context::document::CandleDocument;
use crate::domain::tool::traits::CandleTool;
use crate::memory::memory::manager::MemoryManager;
use std::sync::Arc;
use std::path::Path;

/// Maximum number of relevant memories for context injection
#[allow(dead_code)]
const MAX_RELEVANT_MEMORIES: usize = 10;

/// Global atomic counter for memory node creation
#[allow(dead_code)]
static MEMORY_NODE_COUNTER: LazyLock<CachePadded<RelaxedCounter>> =
    LazyLock::new(|| CachePadded::new(RelaxedCounter::new(0)));

/// Global atomic counter for attention scoring operations
#[allow(dead_code)]
static ATTENTION_SCORE_COUNTER: LazyLock<CachePadded<AtomicUsize>> =
    LazyLock::new(|| CachePadded::new(AtomicUsize::new(0)));

/// MCP Server configuration
#[derive(Debug, Clone)]
pub struct McpServerConfig {
    /// MCP server type identification (stdio, socket, etc.)
    server_type: String,
    /// MCP server binary executable path
    bin_path: Option<String>,
    /// MCP server initialization command
    init_command: Option<String>,
}

impl McpServerConfig {
    /// Create a new MCP server configuration
    #[inline]
    pub fn new(
        server_type: String,
        bin_path: Option<String>,
        init_command: Option<String>,
    ) -> Self {
        Self {
            server_type,
            bin_path,
            init_command,
        }
    }

    /// Create a stdio-based MCP server configuration
    #[inline]
    pub fn stdio(bin_path: impl Into<String>) -> Self {
        Self {
            server_type: "stdio".to_string(),
            bin_path: Some(bin_path.into()),
            init_command: None,
        }
    }

    /// Create a socket-based MCP server configuration
    #[inline]
    pub fn socket(init_command: impl Into<String>) -> Self {
        Self {
            server_type: "socket".to_string(),
            bin_path: None,
            init_command: Some(init_command.into()),
        }
    }

    /// Get the server type
    #[inline]
    pub fn server_type(&self) -> &str {
        &self.server_type
    }

    /// Get the binary path
    #[inline]
    pub fn bin_path(&self) -> Option<&str> {
        self.bin_path.as_deref()
    }

    /// Get the initialization command
    #[inline]
    pub fn init_command(&self) -> Option<&str> {
        self.init_command.as_deref()
    }
}

/// Completion provider types that can be used with the agent
#[derive(Debug, Clone)]
pub enum CandleCompletionProviderType {
    /// Kimi K2 local model provider
    KimiK2(CandleKimiK2Provider),
    /// Qwen3 Coder local model provider
    Qwen3Coder(CandleQwen3CoderProvider),
}

impl CandleCompletionModel for CandleCompletionProviderType {
    fn prompt(
        &self,
        prompt: crate::domain::prompt::CandlePrompt,
        params: &crate::domain::completion::types::CandleCompletionParams,
    ) -> ystream::AsyncStream<crate::domain::completion::CandleCompletionChunk> {
        match self {
            CandleCompletionProviderType::KimiK2(provider) => provider.prompt(prompt, params),
            CandleCompletionProviderType::Qwen3Coder(provider) => provider.prompt(prompt, params),
        }
    }
}

/// Core agent role trait defining all operations and properties
pub trait CandleAgentRole: Send + Sync + fmt::Debug + Clone {
    /// Get the name of the agent role
    fn name(&self) -> &str;

    /// Get the temperature setting
    fn temperature(&self) -> Option<f64>;

    /// Get the max tokens setting
    fn max_tokens(&self) -> Option<u64>;

    /// Get the system prompt
    fn system_prompt(&self) -> Option<&str>;

    /// Create a new agent role with the given name
    fn new(name: impl Into<String>) -> Self;
}

/// Default implementation of the `CandleAgentRole` trait
pub struct CandleAgentRoleImpl {
    name: String,
    /// Completion provider integration (KimiK2, Qwen3Coder, etc.) - Local models only
    completion_provider: Option<CandleCompletionProviderType>,
    temperature: Option<f64>,
    max_tokens: Option<u64>,
    system_prompt: Option<String>,
    /// Document context loading and management
    contexts: Option<ZeroOneOrMany<CandleDocument>>,
    /// Tool integration and function calling
    tools: Option<ZeroOneOrMany<Arc<dyn CandleTool>>>,
    /// MCP server configuration and management
    mcp_servers: Option<ZeroOneOrMany<McpServerConfig>>,
    /// Provider-specific parameters (model paths, quantization options)
    additional_params: Option<HashMap<String, Value>>,
    /// Persistent memory and conversation storage
    memory: Option<Arc<dyn MemoryManager>>,
    /// Agent metadata and custom attributes
    metadata: Option<HashMap<String, Value>>,
    /// Tool result processing and callback handling
    on_tool_result_handler: Option<Box<dyn Fn(ZeroOneOrMany<Value>) + Send + Sync>>,
    /// Conversation turn event handling and logging
    on_conversation_turn_handler:
        Option<Box<dyn Fn(&CandleAgentConversation, &CandleAgentRoleAgent) + Send + Sync>>,
}

impl std::fmt::Debug for CandleAgentRoleImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CandleAgentRoleImpl")
            .field("name", &self.name)
            .field("temperature", &self.temperature)
            .field("max_tokens", &self.max_tokens)
            .field("system_prompt", &self.system_prompt)
            .field("completion_provider", &self.completion_provider.is_some())
            .finish()
    }
}

impl Clone for CandleAgentRoleImpl {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            completion_provider: self.completion_provider.clone(),
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            system_prompt: self.system_prompt.clone(),
            contexts: self.contexts.clone(),
            tools: self.tools.clone(), // Arc<dyn Tool> can be cloned
            mcp_servers: self.mcp_servers.clone(),
            additional_params: self.additional_params.clone(),
            memory: self.memory.clone(), // Arc<dyn MemoryManager> can be cloned
            metadata: self.metadata.clone(),
            on_tool_result_handler: None, // Cannot clone function pointer
            on_conversation_turn_handler: None, // Cannot clone function pointer
        }
    }
}

impl CandleAgentRole for CandleAgentRoleImpl {
    fn name(&self) -> &str {
        &self.name
    }

    fn temperature(&self) -> Option<f64> {
        self.temperature
    }

    fn max_tokens(&self) -> Option<u64> {
        self.max_tokens
    }

    fn system_prompt(&self) -> Option<&str> {
        self.system_prompt.as_deref()
    }

    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            completion_provider: None,
            temperature: None,
            max_tokens: None,
            system_prompt: None,
            contexts: None,
            tools: None,
            mcp_servers: None,
            additional_params: None,
            memory: None,
            metadata: None,
            on_tool_result_handler: None,
            on_conversation_turn_handler: None,
        }
    }
}

impl CandleAgentRoleImpl {
    /// Set completion provider for agent role
    ///
    /// # Arguments
    /// * `provider` - Completion provider to use
    ///
    /// # Returns
    /// Updated agent role instance
    #[inline]
    pub fn with_completion_provider(mut self, provider: CandleCompletionProviderType) -> Self {
        self.completion_provider = Some(provider);
        self
    }

    /// Get completion provider reference if available
    ///
    /// # Returns
    /// Optional reference to completion provider
    #[inline]
    pub fn get_completion_provider(&self) -> Option<&CandleCompletionProviderType> {
        self.completion_provider.as_ref()
    }

    /// Add context from file path
    ///
    /// # Arguments
    /// * `file_path` - Path to the file to load as context
    ///
    /// # Returns
    /// Updated agent role instance
    #[inline]
    pub fn add_context_from_file(mut self, file_path: impl AsRef<Path>) -> Self {
        use crate::domain::context::document::CandleDocument;
        
        let document = CandleDocument::from_file(file_path.as_ref()).load();
        
        match self.contexts {
            None => {
                self.contexts = Some(ZeroOneOrMany::One(document));
            }
            Some(ZeroOneOrMany::None) => {
                self.contexts = Some(ZeroOneOrMany::One(document));
            }
            Some(ZeroOneOrMany::One(existing)) => {
                self.contexts = Some(ZeroOneOrMany::Many(vec![existing, document]));
            }
            Some(ZeroOneOrMany::Many(ref mut existing)) => {
                existing.push(document);
            }
        }
        
        self
    }

    /// Add context from directory path
    ///
    /// # Arguments
    /// * `dir_path` - Path to the directory to load as context
    ///
    /// # Returns
    /// Updated agent role instance
    #[inline]
    pub fn add_context_from_directory(mut self, dir_path: impl AsRef<Path>) -> Self {
        use crate::domain::context::document::CandleDocument;
        use std::fs;
        
        let dir_path = dir_path.as_ref();
        
        // Read directory entries and create documents for each file
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_file() {
                    let document = CandleDocument::from_file(&path).load();
                    
                    match self.contexts {
                        None => {
                            self.contexts = Some(ZeroOneOrMany::One(document));
                        }
                        Some(ZeroOneOrMany::None) => {
                            self.contexts = Some(ZeroOneOrMany::One(document));
                        }
                        Some(ZeroOneOrMany::One(existing)) => {
                            self.contexts = Some(ZeroOneOrMany::Many(vec![existing, document]));
                        }
                        Some(ZeroOneOrMany::Many(ref mut existing)) => {
                            existing.push(document);
                        }
                    }
                }
            }
        }
        
        self
    }

    /// Register a tool with the agent
    ///
    /// # Arguments
    /// * `tool` - Tool implementation to register
    ///
    /// # Returns
    /// Updated agent role instance
    #[inline]
    pub fn register_tool<T: CandleTool + 'static>(mut self, tool: T) -> Self {
        let tool_arc = Arc::new(tool);
        match self.tools {
            None => self.tools = Some(ZeroOneOrMany::One(tool_arc)),
            Some(ZeroOneOrMany::None) => self.tools = Some(ZeroOneOrMany::One(tool_arc)),
            Some(ZeroOneOrMany::One(existing)) => {
                self.tools = Some(ZeroOneOrMany::Many(vec![existing, tool_arc]));
            }
            Some(ZeroOneOrMany::Many(ref mut vec)) => {
                vec.push(tool_arc);
            }
        }
        self
    }

    /// Execute a tool by name with given arguments
    ///
    /// # Arguments
    /// * `tool_name` - Name of the tool to execute
    /// * `args` - Arguments to pass to the tool
    ///
    /// # Returns
    /// AsyncStream of tool execution results or error
    pub fn execute_tool(&self, tool_name: &str, args: Value) -> Result<ystream::AsyncStream<Value>, CandleChatError> {
        if let Some(tools) = &self.tools {
            for tool in tools.iter() {
                if tool.name() == tool_name {
                    return Ok(tool.execute(args));
                }
            }
        }
        Err(CandleChatError::System(format!("Tool '{}' not found", tool_name)))
    }

    /// Add MCP server configuration
    ///
    /// # Arguments
    /// * `config` - MCP server configuration
    ///
    /// # Returns
    /// Updated agent role instance
    #[inline]
    pub fn add_mcp_server(mut self, config: McpServerConfig) -> Self {
        match self.mcp_servers {
            None => self.mcp_servers = Some(ZeroOneOrMany::One(config)),
            Some(ZeroOneOrMany::None) => self.mcp_servers = Some(ZeroOneOrMany::One(config)),
            Some(ZeroOneOrMany::One(existing)) => {
                self.mcp_servers = Some(ZeroOneOrMany::Many(vec![existing, config]));
            }
            Some(ZeroOneOrMany::Many(ref mut vec)) => {
                vec.push(config);
            }
        }
        self
    }

    /// Get MCP server configurations
    ///
    /// # Returns
    /// Optional reference to MCP server configurations
    #[inline]
    pub fn get_mcp_servers(&self) -> Option<&ZeroOneOrMany<McpServerConfig>> {
        self.mcp_servers.as_ref()
    }

    /// Set memory manager for agent role
    ///
    /// # Arguments
    /// * `memory_manager` - Memory manager instance
    ///
    /// # Returns
    /// Updated agent role instance
    #[inline]
    pub fn with_memory_manager(mut self, memory_manager: Arc<dyn MemoryManager>) -> Self {
        self.memory = Some(memory_manager);
        self
    }

    /// Get memory manager reference if available
    ///
    /// # Returns
    /// Optional reference to memory manager
    #[inline]
    pub fn get_memory_manager(&self) -> Option<&Arc<dyn MemoryManager>> {
        self.memory.as_ref()
    }

    /// Set tool result handler
    ///
    /// # Arguments
    /// * `handler` - Function to handle tool results
    ///
    /// # Returns
    /// Updated agent role instance
    #[inline]
    pub fn on_tool_result<F>(mut self, handler: F) -> Self
    where
        F: Fn(ZeroOneOrMany<Value>) + Send + Sync + 'static,
    {
        self.on_tool_result_handler = Some(Box::new(handler));
        self
    }

    /// Set conversation turn handler
    ///
    /// # Arguments
    /// * `handler` - Function to handle conversation turns
    ///
    /// # Returns
    /// Updated agent role instance
    #[inline]
    pub fn on_conversation_turn<F>(mut self, handler: F) -> Self
    where
        F: Fn(&CandleAgentConversation, &CandleAgentRoleAgent) + Send + Sync + 'static,
    {
        self.on_conversation_turn_handler = Some(Box::new(handler));
        self
    }




}

/// Placeholder for Stdio type
pub struct CandleStdio;

/// Agent type placeholder for agent role
pub struct CandleAgentRoleAgent;

/// Agent conversation type
pub struct CandleAgentConversation {
    /// Conversation messages as role-content pairs
    pub messages: Option<ZeroOneOrMany<(CandleMessageRole, String)>>,
}

impl CandleAgentConversation {
    /// Get the last message from the conversation
    #[inline]
    pub fn last(&self) -> CandleAgentConversationMessage {
        CandleAgentConversationMessage {
            content: self
                .messages
                .as_ref()
                .and_then(|msgs| {
                    // Get the last element from ZeroOneOrMany
                    let all: Vec<_> = msgs.clone().into_iter().collect();
                    all.last().map(|(_, m)| m.clone())
                })
                .unwrap_or_default(),
        }
    }

    /// Get the latest user message from the conversation
    #[inline]
    pub fn latest_user_message(&self) -> String {
        self.messages
            .as_ref()
            .and_then(|msgs| {
                // Find the last user message
                let all: Vec<_> = msgs.clone().into_iter().collect();
                all.iter()
                    .rev()
                    .find(|(role, _)| matches!(role, CandleMessageRole::User))
                    .map(|(_, content)| content.clone())
            })
            .unwrap_or_default()
    }
}

/// A single message in an agent conversation
pub struct CandleAgentConversationMessage {
    content: String,
}

impl CandleAgentConversationMessage {
    /// Get the message content as a string slice
    #[inline]
    pub fn message(&self) -> &str {
        &self.content
    }
}

/// Trait for context arguments - moved to paraphym/src/builders/
pub trait CandleContextArgs {
    /// Add this context to the collection of contexts
    fn add_to(self, contexts: &mut Option<ZeroOneOrMany<Box<dyn std::any::Any + Send + Sync>>>);
}

/// Trait for tool arguments - moved to paraphym/src/builders/
pub trait CandleToolArgs {
    /// Add this tool to the collection of tools
    fn add_to(self, tools: &mut Option<ZeroOneOrMany<Box<dyn std::any::Any + Send + Sync>>>);
}

/// Trait for conversation history arguments - moved to paraphym/src/builders/
pub trait CandleConversationHistoryArgs {
    /// Convert this into conversation history format
    fn into_history(self) -> Option<ZeroOneOrMany<(CandleMessageRole, String)>>;
}

// Import CandleChatError from chat module
use crate::domain::agent::chat::CandleChatError;
