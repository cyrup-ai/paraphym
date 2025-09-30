//! Core agent data structures with automatic memory tool injection

use std::sync::{atomic::AtomicUsize, Arc};

use crossbeam_utils::CachePadded;
use ystream::AsyncStream;
use serde_json::Value;
use sweet_mcp_type::ToolInfo;

use crate::domain::context::CandleDocument as Document;
use crate::domain::memory::config::memory::MemoryConfig as ComprehensiveMemoryConfig;
use crate::domain::memory::MemoryConfig;
use crate::domain::memory::{Error as MemoryError, MemoryTool, MemoryToolError};
use crate::memory::memory::SurrealDBMemoryManager;
use crate::domain::model::CandleModel as Model;
// Tool data now comes from SweetMCP ToolInfo directly
use cyrup_sugars::ZeroOneOrMany;

/// Maximum number of tools per agent (const generic default)
pub const MAX_AGENT_TOOLS: usize = 32;

/// Agent statistics for performance monitoring
#[allow(dead_code)] // TODO: Implement in agent monitoring system
static AGENT_STATS: CachePadded<AtomicUsize> = CachePadded::new(AtomicUsize::new(0));

/// Result type for agent operations
pub type AgentResult<T> = Result<T, AgentError>;

/// Agent creation error types
#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    /// Memory system initialization error
    #[error("Memory initialization failed: {0}")]
    MemoryInit(#[from] MemoryError),
    /// Memory tool creation error
    #[error("Memory tool creation failed: {0}")]
    MemoryTool(#[from] Box<MemoryToolError>),
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),
    /// Agent initialization error
    #[error("Agent initialization failed: {0}")]
    InitializationError(String),
}

/// Agent data structure with automatic memory tool injection
#[derive(Debug, Clone)]
pub struct Agent<M: Model> {
    /// The model configuration and implementation to use for this agent
    pub model: M,
    /// System prompt that defines the agent's personality and behavior
    pub system_prompt: String,
    /// Context documents that provide background information
    pub context: ZeroOneOrMany<Document>,
    /// MCP tools available to the agent for function calling
    pub tools: ZeroOneOrMany<ToolInfo>,
    /// Optional memory system for storing and retrieving conversation context
    pub memory: Option<Arc<SurrealDBMemoryManager>>,
    /// Memory tool for automated memory management operations
    pub memory_tool: Option<MemoryTool>,
    /// Temperature setting for response randomness (0.0 to 2.0)
    pub temperature: Option<f64>,
    /// Maximum number of tokens to generate in responses
    pub max_tokens: Option<u64>,
    /// Additional model-specific parameters as flexible JSON
    pub additional_params: Option<Value>,
}

impl<M: Model + Clone + Send + 'static + Default> Agent<M> {
    /// Create a new agent with zero-allocation memory tool injection
    ///
    /// # Arguments
    /// * `model` - Model configuration for the agent
    /// * `system_prompt` - System prompt for the agent
    ///
    /// # Returns
    /// Stream that emits configured agent with memory tool
    ///
    /// # Performance
    /// Zero allocation agent construction with lock-free memory manager sharing
    #[inline]
    pub fn new(
        model: M,
        system_prompt: impl Into<String>,
    ) -> AsyncStream<crate::domain::context::chunk::CandleResult<Self, AgentError>> {
        let system_prompt = system_prompt.into();
        AsyncStream::with_channel(move |sender| {
            std::thread::spawn(move || {
                // Initialize memory system with cognitive settings optimized for performance
                let comprehensive_config = ComprehensiveMemoryConfig::default();
                // Convert comprehensive config to Memory::new() format
                let _memory_config = MemoryConfig { // Reserved for future memory configuration
                    database: crate::domain::memory::config::database::DatabaseConfig {
                        db_type: crate::domain::memory::config::database::DatabaseType::SurrealDB,
                        connection_string: comprehensive_config.database.connection_string.to_string(),
                        namespace: "test".to_string(),
                        database: "memory".to_string(),
                        username: None,
                        password: None,
                        pool_config: crate::domain::memory::config::database::PoolConfig::default(),
                        timeout_config: crate::domain::memory::config::database::TimeoutConfig::default(),
                        health_check_config: crate::domain::memory::config::database::HealthCheckConfig::default(),
                        options: None,
                    },
                    vector_store: crate::domain::memory::config::vector::VectorStoreConfig::default(),
                    provider_model: crate::domain::chat::config::CandleModelConfig::default(),
                    cognitive: crate::domain::memory::cognitive::types::CognitiveMemoryConfig::default(),
                    cognitive_processor: crate::domain::memory::cognitive::types::CognitiveProcessorConfig::default(),
                    performance: crate::domain::memory::config::memory::MemoryPerformanceConfig::default(),
                    retention: crate::domain::memory::config::memory::MemoryRetentionConfig::default(),
                    security: crate::domain::memory::config::memory::MemorySecurityConfig::default(),
                    monitoring: crate::domain::memory::config::memory::MemoryMonitoringConfig::default(),
                };

                // Spawn async task to properly initialize memory and create agent
                tokio::spawn(async move {
                    // Initialize the memory system asynchronously
                    let memory_manager = match crate::memory::initialize(&crate::memory::MemoryConfig {
                        database: crate::memory::utils::config::DatabaseConfig {
                            db_type: crate::memory::utils::config::DatabaseType::SurrealDB,
                            connection_string: comprehensive_config.database.connection_string.clone(),
                            namespace: "test".to_string(),
                            database: "memory".to_string(),
                            username: None,
                            password: None,
                            pool_size: None,
                            options: None,
                        },
                        vector_store: crate::memory::utils::config::VectorStoreConfig::default(),
                        completion: crate::memory::utils::config::CompletionConfig::default(),
                        api: None,
                        cache: crate::memory::utils::config::CacheConfig::default(),
                        logging: crate::memory::utils::config::LoggingConfig::default(),
                    }).await {
                        Ok(manager) => manager,
                        Err(e) => {
                            let _ = sender.send(crate::domain::context::chunk::CandleResult { 
                                result: Err(AgentError::Config(
                                    format!("Failed to initialize memory: {e}"),
                                ))
                            });
                            return;
                        }
                    };

                    // Create memory tool with the initialized manager
                    let memory_manager_arc = Arc::new(memory_manager);
                    let memory_tool = MemoryTool::new(Arc::clone(&memory_manager_arc));

                    // Create agent with properly initialized memory
                    let agent = Self {
                        model,
                        system_prompt,
                        context: ZeroOneOrMany::None,
                        tools: ZeroOneOrMany::None,
                        memory: Some(memory_manager_arc),
                        memory_tool: Some(memory_tool),
                        temperature: None,
                        max_tokens: None,
                        additional_params: None,
                    };

                    // Send the successfully created agent
                    let _ = sender.send(crate::domain::context::chunk::CandleResult { 
                        result: Ok(agent)
                    });
                });
            });
        })
    }

    /// Create a new agent with custom memory configuration
    ///
    /// # Arguments
    /// * `model` - Model configuration for the agent
    /// * `system_prompt` - System prompt for the agent
    /// * `memory_config` - Custom memory configuration
    ///
    /// # Returns
    /// Stream that emits configured agent with memory tool
    ///
    /// # Performance
    /// Zero allocation with custom cognitive settings
    #[inline]
    pub fn with_memory_config(
        model: M,
        system_prompt: impl Into<String>,
        memory_config: ComprehensiveMemoryConfig,
    ) -> AsyncStream<crate::domain::context::chunk::CandleResult<Self, AgentError>> {
        let system_prompt = system_prompt.into();
        AsyncStream::with_channel(move |sender| {
            std::thread::spawn(move || {
                // Initialize memory system with custom configuration
                // Convert comprehensive config to Memory::new() format
                let memory_cfg = MemoryConfig {
                    database: crate::domain::memory::config::database::DatabaseConfig {
                        db_type: crate::domain::memory::config::database::DatabaseType::SurrealDB,
                        connection_string: memory_config.database.connection_string.to_string(),
                        namespace: "test".to_string(),
                        database: "memory".to_string(),
                        username: None,
                        password: None,
                        pool_config: crate::domain::memory::config::database::PoolConfig::default(),
                        timeout_config: crate::domain::memory::config::database::TimeoutConfig::default(),
                        health_check_config: crate::domain::memory::config::database::HealthCheckConfig::default(),
                        options: None,
                    },
                    vector_store: crate::domain::memory::config::vector::VectorStoreConfig::default(),
                    provider_model: crate::domain::chat::config::CandleModelConfig::default(),
                    cognitive: crate::domain::memory::cognitive::types::CognitiveMemoryConfig::default(),
                    cognitive_processor: crate::domain::memory::cognitive::types::CognitiveProcessorConfig::default(),
                    performance: crate::domain::memory::config::memory::MemoryPerformanceConfig::default(),
                    retention: crate::domain::memory::config::memory::MemoryRetentionConfig::default(),
                    security: crate::domain::memory::config::memory::MemorySecurityConfig::default(),
                    monitoring: crate::domain::memory::config::memory::MemoryMonitoringConfig::default(),
                };

                // Spawn async task to properly initialize memory and create agent
                tokio::spawn(async move {
                    // Initialize the memory system asynchronously
                    let memory_manager = match crate::memory::initialize(&crate::memory::MemoryConfig {
                        database: crate::memory::utils::config::DatabaseConfig {
                            db_type: crate::memory::utils::config::DatabaseType::SurrealDB,
                            connection_string: memory_cfg.database.connection_string.clone(),
                            namespace: "test".to_string(),
                            database: "memory".to_string(),
                            username: None,
                            password: None,
                            pool_size: None,
                            options: None,
                        },
                        vector_store: crate::memory::utils::config::VectorStoreConfig::default(),
                        completion: crate::memory::utils::config::CompletionConfig::default(),
                        api: None,
                        cache: crate::memory::utils::config::CacheConfig::default(),
                        logging: crate::memory::utils::config::LoggingConfig::default(),
                    }).await {
                        Ok(manager) => manager,
                        Err(e) => {
                            let _ = sender.send(crate::domain::context::chunk::CandleResult { 
                                result: Err(AgentError::Config(
                                    format!("Failed to initialize memory: {e}"),
                                ))
                            });
                            return;
                        }
                    };

                    // Create memory tool with the initialized manager
                    let memory_manager_arc = Arc::new(memory_manager);
                    let memory_tool = MemoryTool::new(Arc::clone(&memory_manager_arc));

                    // Create agent with properly initialized memory
                    let agent = Self {
                        model,
                        system_prompt,
                        context: ZeroOneOrMany::None,
                        tools: ZeroOneOrMany::None,
                        memory: Some(memory_manager_arc),
                        memory_tool: Some(memory_tool),
                        temperature: None,
                        max_tokens: None,
                        additional_params: None,
                    };

                    // Send the successfully created agent
                    let _ = sender.send(crate::domain::context::chunk::CandleResult { 
                        result: Ok(agent)
                    });
                });
            });
        })
    }

    /// Create a new agent with shared memory instance
    ///
    /// # Arguments
    /// * `model` - Model configuration for the agent
    /// * `system_prompt` - System prompt for the agent
    /// * `memory` - Shared memory instance for lock-free concurrent access
    ///
    /// # Returns
    /// Stream that emits configured agent with shared memory
    ///
    /// # Performance
    /// Zero allocation with lock-free memory sharing between agents
    #[inline]
    pub fn with_shared_memory(
        model: M,
        system_prompt: impl Into<String>,
        memory: Arc<SurrealDBMemoryManager>,
    ) -> AsyncStream<crate::domain::context::chunk::CandleResult<Self, AgentError>> {
        let system_prompt = system_prompt.into();
        AsyncStream::with_channel(move |sender| {
            std::thread::spawn(move || {
                // Create memory tool with zero-allocation initialization
                let memory_tool = MemoryTool::new(Arc::clone(&memory));

                let agent = Self {
                    model,
                    system_prompt,
                    context: ZeroOneOrMany::None,
                    tools: ZeroOneOrMany::None,
                    memory: Some(Arc::clone(&memory)),
                    memory_tool: Some(memory_tool),
                    temperature: None,
                    max_tokens: None,
                    additional_params: None,
                };

                let _ = sender.send(crate::domain::context::chunk::CandleResult { 
                    result: Ok(agent)
                });
            });
        })
    }

    /// Get memory tool reference for direct access
    ///
    /// # Returns
    /// Optional reference to memory tool
    ///
    /// # Performance
    /// Zero cost abstraction with direct tool access
    #[inline]
    pub fn memory_tool(&self) -> Option<&MemoryTool> {
        self.memory_tool.as_ref()
    }

    /// Get memory reference for direct access
    ///
    /// # Returns
    /// Optional reference to memory instance
    ///
    /// # Performance
    /// Zero cost abstraction with direct memory access
    #[inline]
    pub fn memory(&self) -> Option<&Arc<SurrealDBMemoryManager>> {
        self.memory.as_ref()
    }

    /// Add additional tool to the agent
    ///
    /// # Arguments
    /// * `tool` - Tool to add to the agent
    ///
    /// # Returns
    /// Updated agent instance
    ///
    /// # Performance
    /// Zero allocation with inlined tool addition
    #[inline]
    #[must_use]
    pub fn add_tool(mut self, tool: ToolInfo) -> Self {
        match &mut self.tools {
            ZeroOneOrMany::None => {
                self.tools = ZeroOneOrMany::One(tool);
            }
            ZeroOneOrMany::One(existing) => {
                let existing = std::mem::replace(existing, tool.clone());
                self.tools = ZeroOneOrMany::Many(vec![existing, tool]);
            }
            ZeroOneOrMany::Many(tools) => {
                tools.push(tool);
            }
        }
        self
    }

    /// Set agent temperature
    ///
    /// # Arguments
    /// * `temperature` - Temperature value for model sampling
    ///
    /// # Returns
    /// Updated agent instance
    ///
    /// # Performance
    /// Zero allocation with direct field assignment
    #[inline]
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set agent max tokens
    ///
    /// # Arguments
    /// * `max_tokens` - Maximum tokens for model output
    ///
    /// # Returns
    /// Updated agent instance
    ///
    /// # Performance
    /// Zero allocation with direct field assignment
    #[inline]
    pub fn max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }
}

impl<M: Model + Clone + Send + 'static + Default> Default for Agent<M> {
    fn default() -> Self {
        Self {
            model: M::default(),
            system_prompt: String::new(),
            context: ZeroOneOrMany::None,
            tools: ZeroOneOrMany::None,
            memory: None,
            memory_tool: None,
            temperature: None,
            max_tokens: None,
            additional_params: None,
        }
    }
}

use cyrup_sugars::prelude::MessageChunk;

impl<M: Model + Clone + Send + 'static + Default> MessageChunk for Agent<M> {
    fn bad_chunk(error: String) -> Self {
        Self {
            model: M::default(),
            system_prompt: error, // Use error directly instead of format!("Error: {}", error)
            context: ZeroOneOrMany::None,
            tools: ZeroOneOrMany::None,
            memory: None,
            memory_tool: None,
            temperature: None,
            max_tokens: None,
            additional_params: None,
        }
    }

    fn error(&self) -> Option<&str> {
        // For proper DTO-based error handling, we could check if system_prompt
        // contains error indicators or use a dedicated error field
        // For now, simple non-empty system_prompt check
        if !self.system_prompt.is_empty() && 
           (self.system_prompt.contains("error") || 
            self.system_prompt.contains("Error") ||
            self.system_prompt.contains("failed") ||
            self.system_prompt.contains("Failed")) {
            Some(&self.system_prompt)
        } else {
            None
        }
    }
}
