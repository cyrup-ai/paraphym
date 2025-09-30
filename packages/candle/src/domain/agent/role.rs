//! Candle Agent role trait and implementation - EXACT REPLICA of domain with Candle prefixes

use std::fmt;
use cyrup_sugars::ZeroOneOrMany;
use hashbrown::HashMap;
use crate::builders::document::DocumentBuilder;

use serde_json::Value;


use crate::domain::chat::CandleMessageRole;
use crate::domain::completion::traits::CandleCompletionModel;
use crate::providers::{CandleKimiK2Provider, CandleQwen3CoderProvider};
use crate::domain::context::document::CandleDocument;
use crate::domain::context::chunk::CandleJsonChunk;
use crate::domain::tool::unified::{UnifiedToolExecutor, ToolError};
use sweet_mcp_type::ToolInfo;
use sweet_mcp_type::JsonValue as SweetJsonValue;
use simd_json::value::owned::Object as JsonObject;
use ystream::AsyncStream;
use crate::memory::core::manager::MemoryManager;
use std::sync::Arc;
use std::path::Path;



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
#[derive(Debug)]
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

/// Type alias for conversation turn handler callback
type ConversationTurnHandler = Box<dyn Fn(&CandleAgentConversation, &CandleAgentRoleAgent) + Send + Sync>;

/// Default implementation of the `CandleAgentRole` trait
pub struct CandleAgentRoleImpl {
    name: String,
    /// Completion provider integration (`KimiK2`, `Qwen3Coder`, etc.) - Local models only
    completion_provider: Option<Arc<CandleCompletionProviderType>>,
    temperature: Option<f64>,
    max_tokens: Option<u64>,
    system_prompt: Option<String>,
    /// Document context loading and management
    contexts: Option<ZeroOneOrMany<CandleDocument>>,
    /// Unified tool executor for both MCP and native tools
    tool_executor: Option<Arc<UnifiedToolExecutor>>,
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
    on_conversation_turn_handler: Option<ConversationTurnHandler>,
}

impl std::fmt::Debug for CandleAgentRoleImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CandleAgentRoleImpl")
            .field("name", &self.name)
            .field("temperature", &self.temperature)
            .field("max_tokens", &self.max_tokens)
            .field("system_prompt", &self.system_prompt)
            .field("completion_provider", &self.completion_provider.is_some())
            .finish_non_exhaustive()
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
            tool_executor: self.tool_executor.clone(), // Arc<UnifiedToolExecutor> can be cloned
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
            tool_executor: None,
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
    #[must_use]
    pub fn with_completion_provider(mut self, provider: CandleCompletionProviderType) -> Self {
        self.completion_provider = Some(Arc::new(provider));
        self
    }

    /// Get completion provider - guaranteed to exist by builder
    ///
    /// # Returns
    /// Reference to completion provider (never None after builder initialization)
    ///
    /// # Panics
    /// Panics if completion provider is None. This should never happen in practice
    /// as the builder pattern guarantees provider initialization before use.
    #[inline]
    pub fn get_completion_provider(&self) -> &CandleCompletionProviderType {
        self.completion_provider.as_ref()
            .expect("Provider guaranteed by builder - this should never panic")
            .as_ref()
    }

    /// Add context from file path
    ///
    /// # Arguments
    /// * `file_path` - Path to the file to load as context
    ///
    /// # Returns
    /// Updated agent role instance
    #[inline]
    #[must_use]
    pub fn add_context_from_file(mut self, file_path: impl AsRef<Path>) -> Self {
        use crate::domain::context::document::CandleDocument;
        
        let document = CandleDocument::from_file(file_path.as_ref()).load();
        
        self.contexts = match self.contexts {
            None | Some(ZeroOneOrMany::None) => Some(ZeroOneOrMany::One(document)),
            Some(ZeroOneOrMany::One(existing)) => Some(ZeroOneOrMany::Many(vec![existing, document])),
            Some(ZeroOneOrMany::Many(mut existing)) => {
                existing.push(document);
                Some(ZeroOneOrMany::Many(existing))
            }
        };
        
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
    #[must_use]
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
                    
                    self.contexts = match self.contexts {
                        None | Some(ZeroOneOrMany::None) => Some(ZeroOneOrMany::One(document)),
                        Some(ZeroOneOrMany::One(existing)) => Some(ZeroOneOrMany::Many(vec![existing, document])),
                        Some(ZeroOneOrMany::Many(mut existing)) => {
                            existing.push(document);
                            Some(ZeroOneOrMany::Many(existing))
                        }
                    };
                }
            }
        }
        
        self
    }

    /// Enable code execution with the unified tool executor
    ///
    /// # Arguments
    /// * `enabled` - Whether to enable code execution via cylo
    ///
    /// # Returns
    /// Updated agent role instance
    #[inline]
    #[must_use]
    pub fn with_code_execution(mut self, _enabled: bool) -> Self {
        // Create or update the unified tool executor
        let executor = UnifiedToolExecutor::new(None); // MCP client will be added later
        self.tool_executor = Some(Arc::new(executor));
        self
    }

    /// Initialize tool execution system
    ///
    /// This method sets up the unified tool executor with MCP servers and code execution
    /// Called automatically during agent initialization
    ///
    /// # Errors
    /// Returns `ToolError` if tool executor initialization fails, including:
    /// - MCP server connection failures
    /// - Tool discovery errors
    /// - Backend initialization issues
    pub async fn initialize_tools(&mut self) -> Result<(), ToolError> {
        if let Some(ref executor) = self.tool_executor {
            executor.initialize().await?;
        }
        Ok(())
    }

    /// Execute a tool by name with given arguments (OpenAI-style function calling)
    ///
    /// # Arguments
    /// * `tool_name` - Name of the tool to execute
    /// * `args` - Arguments to pass to the tool as `serde_json::Value`
    ///
    /// # Returns
    /// `AsyncStream` of tool execution results for `ystream` compatibility
    pub fn execute_tool(&self, tool_name: &str, args: Value) -> ystream::AsyncStream<CandleJsonChunk> {
        if let Some(ref executor) = self.tool_executor {
            // Convert serde_json::Value to sweet_mcp_type::JsonValue
            let sweet_args = convert_serde_to_sweet_json(args);
            executor.call_tool_stream(tool_name, sweet_args)
        } else {
            // Return error stream if no tool executor
            AsyncStream::with_channel(move |sender| {
                let error_value = Value::Object([
                    ("error".to_string(), Value::String("No tool executor configured".to_string()))
                ].into_iter().collect::<serde_json::Map<_, _>>());
                ystream::emit!(sender, CandleJsonChunk(error_value));
            })
        }
    }

    /// Get all available tools for LLM function calling
    ///
    /// # Returns
    /// Vector of `ToolInfo` structs describing available tools
    pub async fn get_available_tools(&self) -> Vec<ToolInfo> {
        if let Some(ref executor) = self.tool_executor {
            executor.get_available_tools().await
        } else {
            Vec::new()
        }
    }

    /// Add MCP server configuration
    ///
    /// # Arguments
    /// * `config` - MCP server configuration
    ///
    /// # Returns
    /// Updated agent role instance
    #[inline]
    #[must_use]
    pub fn add_mcp_server(mut self, config: McpServerConfig) -> Self {
        self.mcp_servers = match self.mcp_servers {
            None | Some(ZeroOneOrMany::None) => Some(ZeroOneOrMany::One(config)),
            Some(ZeroOneOrMany::One(existing)) => Some(ZeroOneOrMany::Many(vec![existing, config])),
            Some(ZeroOneOrMany::Many(mut vec)) => {
                vec.push(config);
                Some(ZeroOneOrMany::Many(vec))
            }
        };
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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

// Import CandleChatError from chat module (removed - unused)

/// Convert `serde_json::Value` to `sweet_mcp_type::JsonValue`
///
/// This function bridges the gap between `serde_json` and `sweet_mcp_type` for
/// compatibility with existing `ystream`-based architecture while using
/// high-performance simd-json internally.
pub fn convert_serde_to_sweet_json(value: Value) -> SweetJsonValue {
    match value {
        Value::Null => SweetJsonValue::Static(simd_json::StaticNode::Null),
        Value::Bool(b) => SweetJsonValue::Static(simd_json::StaticNode::Bool(b)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                SweetJsonValue::from(i)
            } else if let Some(f) = n.as_f64() {
                SweetJsonValue::from(f)
            } else {
                SweetJsonValue::Static(simd_json::StaticNode::Null)
            }
        }
        Value::String(s) => SweetJsonValue::String(s),
        Value::Array(arr) => {
            let sweet_arr: Vec<SweetJsonValue> = arr
                .into_iter()
                .map(convert_serde_to_sweet_json)
                .collect();
            SweetJsonValue::Array(Box::new(sweet_arr))
        }
        Value::Object(obj) => {
            let sweet_obj: JsonObject = obj
                .into_iter()
                .map(|(k, v)| (k, convert_serde_to_sweet_json(v)))
                .collect();
            SweetJsonValue::Object(Box::new(sweet_obj))
        }
    }
}
