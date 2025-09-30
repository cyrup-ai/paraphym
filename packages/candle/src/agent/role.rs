//! Agent role trait and implementation

use std::fmt;
use std::sync::atomic::AtomicUsize;

// Ultra-high-performance zero-allocation imports
use atomic_counter::RelaxedCounter;
use crossbeam_utils::CachePadded;
use once_cell::sync::Lazy;
use serde_json::Value;

use hashbrown::HashMap;
use crate::domain::chat::message::types::CandleMessageRole as MessageRole;
use cyrup_sugars::ZeroOneOrMany;
// Unused imports cleaned up



/// MCP Server configuration
#[derive(Debug, Clone)]
struct McpServerConfig {
    #[allow(dead_code)] // TODO: Use for MCP server type identification (stdio, socket, etc.)
    server_type: String,
    #[allow(dead_code)] // TODO: Use for MCP server binary executable path
    bin_path: Option<String>,
    #[allow(dead_code)] // TODO: Use for MCP server initialization command
    init_command: Option<String>}

/// Core agent role trait defining all operations and properties
pub trait AgentRole: Send + Sync + fmt::Debug + Clone {
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

/// Default implementation of the AgentRole trait
pub struct AgentRoleImpl {
    name: String,
    #[allow(dead_code)] // TODO: Use for completion provider integration (OpenAI, Anthropic, etc.)
    completion_provider: Option<Box<dyn std::any::Any + Send + Sync>>,
    temperature: Option<f64>,
    max_tokens: Option<u64>,
    system_prompt: Option<String>,
    /// OpenAI API key for completions (reads from OPENAI_API_KEY environment variable if not set)
    api_key: Option<String>,
    #[allow(dead_code)] // TODO: Use for document context loading and management
    contexts: Option<ZeroOneOrMany<Box<dyn std::any::Any + Send + Sync>>>,
    #[allow(dead_code)] // TODO: Use for tool integration and function calling
    tools: Option<ZeroOneOrMany<Box<dyn std::any::Any + Send + Sync>>>,
    #[allow(dead_code)] // TODO: Use for MCP server configuration and management
    mcp_servers: Option<ZeroOneOrMany<McpServerConfig>>,
    #[allow(dead_code)]
    // TODO: Use for provider-specific parameters (beta features, custom options)
    additional_params: Option<HashMap<String, Value>>,
    #[allow(dead_code)] // TODO: Use for persistent memory and conversation storage
    memory: Option<Box<dyn std::any::Any + Send + Sync>>,
    #[allow(dead_code)] // TODO: Use for agent metadata and custom attributes
    metadata: Option<HashMap<String, Value>>,
    #[allow(dead_code)] // TODO: Use for tool result processing and callback handling
    on_tool_result_handler: Option<Box<dyn Fn(ZeroOneOrMany<Value>) + Send + Sync>>,
    #[allow(dead_code)] // TODO: Use for conversation turn event handling and logging
    on_conversation_turn_handler:
        Option<Box<dyn Fn(&AgentConversation, &AgentRoleAgent) + Send + Sync>>}

impl std::fmt::Debug for AgentRoleImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AgentRoleImpl")
            .field("name", &self.name)
            .field("temperature", &self.temperature)
            .field("max_tokens", &self.max_tokens)
            .field("system_prompt", &self.system_prompt)
            .field("api_key", &self.api_key.as_ref().map(|_| "***"))
            .finish()
    }
}

impl Clone for AgentRoleImpl {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            completion_provider: None, // Cannot clone Box<dyn Any>
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            system_prompt: self.system_prompt.clone(),
            api_key: self.api_key.clone(),
            contexts: None, // Cannot clone Box<dyn Any>
            tools: None,    // Cannot clone Box<dyn Any>
            mcp_servers: self.mcp_servers.clone(),
            additional_params: self.additional_params.clone(),
            memory: None, // Cannot clone Box<dyn Any>
            metadata: self.metadata.clone(),
            on_tool_result_handler: None, // Cannot clone function pointer
            on_conversation_turn_handler: None, // Cannot clone function pointer
        }
    }
}

impl AgentRole for AgentRoleImpl {
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
            api_key: None,
            contexts: None,
            tools: None,
            mcp_servers: None,
            additional_params: None,
            memory: None,
            metadata: None,
            on_tool_result_handler: None,
            on_conversation_turn_handler: None}
    }
}

impl AgentRoleImpl {
    /// Get memory tool reference if available
    ///
    /// # Returns
    /// Optional reference to memory tool
    ///
    /// # Performance
    /// Zero cost abstraction with direct memory access
    #[inline]
    pub fn get_memory_tool(&self) -> Option<&dyn std::any::Any> {
        self.memory
            .as_ref()
            .map(|m| m.as_ref() as &dyn std::any::Any)
    }

    /// Set memory tool for agent role
    ///
    /// # Arguments
    /// * `memory_tool` - Memory tool instance to set
    ///
    /// # Returns
    /// Updated agent role instance
    ///
    /// # Performance
    /// Zero allocation with direct field assignment
    #[inline]
    pub fn with_memory_tool(mut self, memory_tool: Box<dyn std::any::Any + Send + Sync>) -> Self {
        self.memory = Some(memory_tool);
        self
    }

    /// Set the API key for OpenAI completions
    /// Zero allocation with direct field assignment
    #[inline]
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Get the API key, falling back to environment variable if not set
    /// Zero allocation with efficient environment variable access
    #[allow(dead_code)] // TODO: Implement in API authentication system
    #[inline]
    fn get_api_key(&self) -> Result<String, ChatError> {
        if let Some(ref api_key) = self.api_key {
            Ok(api_key.clone())
        } else {
            std::env::var("OPENAI_API_KEY")
                .map_err(|_| ChatError::System(
                    "OpenAI API key not found. Set OPENAI_API_KEY environment variable or use with_api_key()".to_string()
                ))
        }
    }
}

/// Agent helper type for conversation turn callbacks (deprecated - use domain::agent::role::CandleAgentRoleAgent)
pub struct AgentRoleAgent;

/// Agent conversation type
pub struct AgentConversation {
    messages: Option<ZeroOneOrMany<(MessageRole, String)>>}

impl AgentConversation {
    /// Get the last message from the conversation
    pub fn last(&self) -> AgentConversationMessage {
        AgentConversationMessage {
            content: self
                .messages
                .as_ref()
                .and_then(|msgs| {
                    // Get the last element from ZeroOneOrMany
                    let all: Vec<_> = msgs.clone().into_iter().collect();
                    all.last().map(|(_, m)| m.clone())
                })
                .unwrap_or_default()}
    }
}

/// A single message in an agent conversation
pub struct AgentConversationMessage {
    content: String}

impl AgentConversationMessage {
    /// Get the message content as a string slice
    pub fn message(&self) -> &str {
        &self.content
    }
}


/// Trait for context arguments - moved to paraphym/src/builders/
pub trait ContextArgs {
    /// Add this context to the collection of contexts
    fn add_to(self, contexts: &mut Option<ZeroOneOrMany<Box<dyn std::any::Any + Send + Sync>>>);
}

/// Trait for tool arguments - moved to paraphym/src/builders/
pub trait ToolArgs {
    /// Add this tool to the collection of tools
    fn add_to(self, tools: &mut Option<ZeroOneOrMany<Box<dyn std::any::Any + Send + Sync>>>);
}

/// Trait for conversation history arguments - moved to paraphym/src/builders/
pub trait ConversationHistoryArgs {
    /// Convert this into conversation history format
    fn into_history(self) -> Option<ZeroOneOrMany<(MessageRole, String)>>;
}

// Forward declaration for ChatError - will be defined in chat.rs
use crate::agent::chat::ChatError;
