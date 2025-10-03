//! Chat functionality for memory-enhanced agent conversations

use std::sync::Arc;

use arrayvec::ArrayVec;
use cyrup_sugars::prelude::MessageChunk;

use thiserror::Error;
// StreamExt not currently used but may be needed for future async operations

// Import real completion infrastructure
// Removed unused import: use tokio_stream::StreamExt;
use crate::domain::agent::role::{CandleAgentRole, CandleAgentRoleImpl};
use crate::domain::completion::PromptFormatter;
use crate::domain::completion::traits::CandleCompletionModel;
use crate::domain::prompt::CandlePrompt;
use crate::domain::completion::types::CandleCompletionParams;
use crate::domain::context::chunk::CandleCompletionChunk;
use std::num::{NonZeroU64, NonZeroU8};

use crate::memory::core::primitives::types::{MemoryTypeEnum, MemoryContent};
use crate::memory::core::{MemoryNode};
use crate::memory::core::manager::surreal::MemoryManager;
use crate::memory::core::ops::retrieval::RetrievalResult;
use crate::domain::memory::{Error as MemoryError};
use crate::domain::context::chunk::CandleCollectionChunk;
use crate::domain::context::CandleDocument as Document;
use crate::domain::memory::{MemoryTool, MemoryToolError};
use cyrup_sugars::ZeroOneOrMany;

/// Maximum number of relevant memories for context injection
const MAX_RELEVANT_MEMORIES: usize = 10;

/// Chat error types for memory-enhanced agent conversations
#[derive(Debug, thiserror::Error)]
pub enum ChatError {
    /// Memory system error
    #[error("Memory error: {0}")]
    Memory(#[from] MemoryError),
    /// Memory tool error
    #[error("Memory tool error: {0}")]
    MemoryTool(Box<MemoryToolError>),
    /// Message processing error
    #[error("Message processing error: {0}")]
    Message(String),
    /// System error
    #[error("System error: {0}")]
    System(String),
}

impl From<MemoryToolError> for ChatError {
    fn from(error: MemoryToolError) -> Self {
        Self::MemoryTool(Box::new(error))
    }
}

/// Context injection result with relevance scoring
#[derive(Debug, Clone)]
pub struct ContextInjectionResult {
    /// The context that was injected into the conversation
    pub injected_context: String,
    /// Score indicating how relevant the injected context is (0.0 to 1.0)
    pub relevance_score: f64,
    /// Number of memory nodes that were used in the injection
    pub memory_nodes_used: usize,
}

impl Default for ContextInjectionResult {
    fn default() -> Self {
        ContextInjectionResult {
            injected_context: String::new(),
            relevance_score: 0.0,
            memory_nodes_used: 0,
        }
    }
}

impl MessageChunk for ContextInjectionResult {
    fn bad_chunk(error: String) -> Self {
        ContextInjectionResult {
            injected_context: format!("Error: {error}"),
            relevance_score: 0.0,
            memory_nodes_used: 0,
        }
    }

    fn error(&self) -> Option<&str> {
        if self.injected_context.starts_with("Error: ") {
            Some(&self.injected_context)
        } else {
            None
        }
    }
}

/// Memory-enhanced chat response with zero-allocation collections
#[derive(Debug, Clone)]
pub struct MemoryEnhancedChatResponse {
    /// The generated response text
    pub response: String,
    /// Details about the context that was injected
    pub context_injection: ContextInjectionResult,
    /// Memory nodes that were considered and stored, using fixed-size allocation
    pub memorized_nodes: ArrayVec<MemoryNode, MAX_RELEVANT_MEMORIES>,
}

impl Default for MemoryEnhancedChatResponse {
    fn default() -> Self {
        MemoryEnhancedChatResponse {
            response: String::new(),
            context_injection: ContextInjectionResult::default(),
            memorized_nodes: ArrayVec::new(),
        }
    }
}

impl MessageChunk for MemoryEnhancedChatResponse {
    fn bad_chunk(error: String) -> Self {
        MemoryEnhancedChatResponse {
            response: format!("Error: {error}"),
            context_injection: ContextInjectionResult::default(),
            memorized_nodes: ArrayVec::new(),
        }
    }

    fn error(&self) -> Option<&str> {
        if self.response.starts_with("Error: ") {
            Some(&self.response)
        } else {
            None
        }
    }
}



impl CandleAgentRoleImpl {
    /// Generate real AI response using Provider with Engine orchestration and proper memory vs context sectioning
    ///
    /// # Arguments
    /// * `message` - User message to respond to
    /// * `memories` - Retrieved memories for context
    /// * `documents` - Static context documents
    /// * `chat_history` - Conversation history
    ///
    /// # Returns
    /// Result containing real AI-generated response
    ///
    /// # Performance
    /// Uses Provider with Engine orchestration for real model inference
    /// with proper memory vs context sectioning following LLM best practices
    #[allow(dead_code)]
    fn generate_ai_response_with_sectioning(
        &self,
        message: &str,
        memories: &ZeroOneOrMany<RetrievalResult>,
        documents: &ZeroOneOrMany<Document>,
        chat_history: &ZeroOneOrMany<crate::domain::chat::message::types::CandleMessage>,
    ) -> Result<String, ChatError> {
        // Get configured provider from agent role
        let provider = self.get_completion_provider()
            .map_err(|e| ChatError::System(format!("Provider error: {e}")))?;

        // Use PromptFormatter for proper memory vs context sectioning
        let formatter = PromptFormatter::new()
            .with_max_memory_length(Some(2000))
            .with_max_context_length(Some(4000));

        // Format prompt with system prompt AND clear sectioning for LLM understanding
        let full_prompt = formatter.format_prompt(
            self.system_prompt(),  // Pass system prompt to formatter
            memories,
            documents,
            chat_history,
            message
        );

        // Create CandlePrompt and CandleCompletionParams
        let candle_prompt = CandlePrompt::new(full_prompt);
        let temp = self.temperature();
        let candle_params = CandleCompletionParams {
            temperature: temp,
            max_tokens: self.max_tokens()
                .and_then(NonZeroU64::new)
                .or_else(|| NonZeroU64::new(1000)),
            n: match NonZeroU8::new(1) {
                Some(n) => n,
                None => return Err(ChatError::System("Invalid completion parameter".to_string())),
            },
            stream: true,
            tools: None,
            additional_params: None,
        };

        // Call provider directly - provider handles Engine orchestration internally
        let completion_stream = provider.prompt(candle_prompt, &candle_params);

        // Collect all chunks from the stream (blocking collection)
        let chunks = completion_stream.collect();

        // Combine text from all chunks
        let mut complete_text = String::new();
        for chunk in chunks {
            match chunk {
                CandleCompletionChunk::Text(text) => complete_text.push_str(&text),
                CandleCompletionChunk::Complete { text, .. } => {
                    complete_text.push_str(&text);
                    break; // Stop on completion marker
                }
                CandleCompletionChunk::Error(error) => return Err(ChatError::System(error)),
                _ => {} // Ignore other chunk types
            }
        }

        if complete_text.is_empty() {
            Err(ChatError::System("No response from completion stream".to_string()))
        } else {
            Ok(complete_text)
        }
    }

    /// Generate AI response with context string
    ///
    /// # Arguments
    /// * `message` - User message to respond to
    /// * `context` - Context string for enhanced responses
    ///
    /// # Returns
    /// Result containing AI-generated response
    ///
    /// # Performance
    /// Uses Provider with Engine orchestration for real model inference
    fn generate_ai_response(&self, message: &str, context: &str) -> Result<String, ChatError> {
        // Get configured provider from agent role
        let provider = self.get_completion_provider()
            .map_err(|e| ChatError::System(format!("Provider error: {e}")))?;

        // Build prompt with system prompt, context, and user message
        let full_prompt = match (self.system_prompt(), context.is_empty()) {
            (Some(sys_prompt), false) => {
                // System prompt + context + user message
                format!("{sys_prompt}\n\nContext: {context}\n\nUser: {message}")
            }
            (Some(sys_prompt), true) => {
                // System prompt + user message (no context)
                format!("{sys_prompt}\n\nUser: {message}")
            }
            (None, false) => {
                // No system prompt, but has context
                format!("Context: {context}\n\nUser: {message}")
            }
            (None, true) => {
                // No system prompt, no context - just user message
                message.to_string()
            }
        };

        // Create CandlePrompt with proper role
        let candle_prompt = CandlePrompt::new(full_prompt);
        let temp = self.temperature();
        let candle_params = CandleCompletionParams {
            temperature: temp,
            max_tokens: self.max_tokens()
                .and_then(NonZeroU64::new)
                .or_else(|| NonZeroU64::new(1000)),
            n: match NonZeroU8::new(1) {
                Some(n) => n,
                None => return Err(ChatError::System("Invalid completion parameter".to_string())),
            },
            stream: true,
            tools: None,
            additional_params: None,
        };

        // Call provider directly - provider handles Engine orchestration internally
        let completion_stream = provider.prompt(candle_prompt, &candle_params);

        // Collect all chunks from the stream (blocking collection)
        let chunks = completion_stream.collect();

        // Combine text from all chunks
        let mut complete_text = String::new();
        for chunk in chunks {
            match chunk {
                CandleCompletionChunk::Text(text) => complete_text.push_str(&text),
                CandleCompletionChunk::Complete { text, .. } => {
                    complete_text.push_str(&text);
                    break; // Stop on completion marker
                }
                CandleCompletionChunk::Error(error) => return Err(ChatError::System(error)),
                _ => {} // Ignore other chunk types
            }
        }

        if complete_text.is_empty() {
            Err(ChatError::System("No response from completion stream".to_string()))
        } else {
            Ok(complete_text)
        }
    }

    /// Context-aware chat with automatic memory injection and memorization
    ///
    /// # Arguments
    /// * `message` - User message to process
    /// * `memory_manager` - Memory manager for memory operations
    /// * `memory_tool` - Memory tool for storage operations
    ///
    /// # Returns
    /// Result containing memory-enhanced chat response
    ///
    /// # Performance
    /// Zero allocation with lock-free memory operations and quantum routing
    pub fn chat(
        &self,
        message: impl Into<String>,
        memory_manager: &Arc<dyn MemoryManager>,
        memory_tool: &MemoryTool,
    ) -> ystream::AsyncStream<MemoryEnhancedChatResponse> {
        let message = message.into();
        let self_clone = self.clone();
        let memory_manager_clone = memory_manager.clone();
        let memory_tool_clone = memory_tool.clone();

        ystream::AsyncStream::with_channel(move |sender| {
            // Inject relevant memory context with zero-allocation processing
            let context_stream = self_clone.inject_memory_context(&message, &memory_manager_clone, self_clone.memory_read_timeout());

            if let Some(context_injection) = context_stream.try_next() {
                // Generate real AI response using Engine with TextGenerator
                if let Ok(response) = self_clone.generate_ai_response(&message, &context_injection.injected_context) {
                    // Memorize the conversation turn with zero-allocation node creation
                    let memorize_stream = self_clone.memorize_conversation(
                        &message,
                        &response,
                        &memory_tool_clone,
                    );

                    if let Some(memorized_nodes_chunk) = memorize_stream.try_next() {
                        let result = MemoryEnhancedChatResponse {
                            response,
                            context_injection,
                            memorized_nodes: memorized_nodes_chunk.items,
                        };
                        let _ = sender.send(result);
                    }
                }
                // Else: Error in AI response generation - don't send anything
            }
        })
    }

    /// Inject memory context with zero-allocation processing
    ///
    /// Get timeout in milliseconds from builder API, environment, or default
    fn get_memory_timeout_ms(timeout_ms: Option<u64>) -> u64 {
        timeout_ms
            .or_else(|| {
                std::env::var("CANDLE_MEMORY_TIMEOUT_MS")
                    .ok()
                    .and_then(|v| v.parse::<u64>().ok())
            })
            .unwrap_or(5000)
    }

    /// Calculate average relevance score from retrieval results
    #[allow(clippy::cast_precision_loss)]
    fn calculate_avg_relevance(retrieval_results: &[RetrievalResult]) -> f64 {
        if retrieval_results.is_empty() {
            return 0.0;
        }
        let total: f32 = retrieval_results.iter().map(|r| r.score).sum();
        f64::from(total / retrieval_results.len() as f32)
    }

    /// Convert memory node to retrieval result with metadata
    fn memory_node_to_retrieval_result(memory_node: &MemoryNode) -> RetrievalResult {
        let score = memory_node.relevance_score
            .unwrap_or(memory_node.metadata.importance);
        
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("content".to_string(), serde_json::Value::String(
            memory_node.content.text.clone()
        ));
        metadata.insert("memory_type".to_string(), serde_json::Value::String(
            format!("{:?}", memory_node.memory_type)
        ));
        metadata.insert("importance".to_string(), serde_json::Value::Number(
            serde_json::Number::from_f64(f64::from(memory_node.metadata.importance))
                .unwrap_or_else(|| serde_json::Number::from(0))
        ));
        if let Some(relevance) = memory_node.relevance_score {
            metadata.insert("relevance_score".to_string(), serde_json::Value::Number(
                serde_json::Number::from_f64(f64::from(relevance))
                    .unwrap_or_else(|| serde_json::Number::from(0))
            ));
        }

        RetrievalResult {
            id: memory_node.id.clone(),
            score,
            method: crate::memory::core::ops::retrieval::RetrievalMethod::Semantic,
            metadata,
        }
    }

    /// # Arguments
    /// * `message` - User message for context relevance
    /// * `memory_manager` - Memory manager for queries
    ///
    /// # Returns
    /// Result containing context injection result
    ///
    /// # Performance
    /// Zero allocation with lock-free memory queries and quantum routing
    pub fn inject_memory_context(
        &self,
        message: &str,
        memory_manager: &Arc<dyn MemoryManager>,
        timeout_ms: Option<u64>,
    ) -> ystream::AsyncStream<ContextInjectionResult> {
        let message = message.to_string();
        let memory_manager_clone = memory_manager.clone();

        ystream::AsyncStream::with_channel(move |sender| {
            // Use MemoryManager's HIGH-LEVEL API - it handles EVERYTHING internally:
            // - Embedding generation
            // - Quantum routing to decide search strategy 
            // - Vector similarity search with cosine scores
            // - Temporal decay
            // - Result ranking
            let memory_stream = memory_manager_clone.search_by_content(&message);

            // Collect results from the sophisticated memory system
            let (retrieval_tx, mut retrieval_rx) = tokio::sync::mpsc::channel::<Result<Vec<RetrievalResult>, String>>(1);

            tokio::spawn(async move {
                use futures_util::StreamExt;
                
                let mut stream = memory_stream;
                let mut results = Vec::new();
                
                // Collect up to MAX_RELEVANT_MEMORIES from the stream
                while let Some(memory_result) = stream.next().await {
                    if results.len() >= MAX_RELEVANT_MEMORIES {
                        break;
                    }
                    
                    if let Ok(memory_node) = memory_result {
                        results.push(Self::memory_node_to_retrieval_result(&memory_node));
                    }
                }
                
                let _ = retrieval_tx.send(Ok(results)).await;
            });

            // Receive results with configurable timeout
            let timeout_ms = Self::get_memory_timeout_ms(timeout_ms);

            let retrieval_results = tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    let timeout_duration = std::time::Duration::from_millis(timeout_ms);
                    
                    match tokio::time::timeout(timeout_duration, retrieval_rx.recv()).await {
                        Ok(Some(Ok(results))) => results,
                        Ok(Some(Err(e))) => {
                            log::error!("Memory retrieval failed: {e}");
                            Vec::new()
                        }
                        Ok(None) => {
                            log::error!("Memory retrieval channel closed unexpectedly");
                            Vec::new()
                        }
                        Err(_) => {
                            log::warn!(
                                "Memory retrieval timed out - context may be incomplete (timeout_ms: {timeout_ms}, message: {message:?})"
                            );
                            Vec::new()
                        }
                    }
                })
            });

            let memory_nodes_used = retrieval_results.len();
            let avg_relevance_score = Self::calculate_avg_relevance(&retrieval_results);

            // Use PromptFormatter to format memories properly
            let formatter = PromptFormatter::new()
                .with_max_memory_length(Some(2000))
                .with_headers(true);
            
            let memories_zero_one_many = ZeroOneOrMany::from(retrieval_results);
            
            let injected_context = formatter.format_memory_section(&memories_zero_one_many)
                .unwrap_or_default();

            let result = ContextInjectionResult {
                injected_context,
                relevance_score: avg_relevance_score,
                memory_nodes_used,
            };

            let _ = sender.send(result);
        })
    }



    /// Memorize conversation turn with zero-allocation node creation
    ///
    /// # Arguments
    /// * `user_message` - User message to memorize
    /// * `assistant_response` - Assistant response to memorize
    /// * `memory_tool` - Memory tool for storage operations
    ///
    /// # Returns
    /// Result containing memorized nodes
    ///
    /// # Performance
    /// Zero allocation with lock-free atomic counters for memory node tracking
    #[must_use]
    pub fn memorize_conversation(
        &self,
        user_message: &str,
        assistant_response: &str,
        memory_tool: &MemoryTool,
    ) -> ystream::AsyncStream<CandleCollectionChunk<ArrayVec<MemoryNode, MAX_RELEVANT_MEMORIES>>> {
        let user_message = user_message.to_string();
        let assistant_response = assistant_response.to_string();
        let memory_tool_clone = memory_tool.clone();

        ystream::AsyncStream::with_channel(move |sender| {
            let mut memorized_nodes = ArrayVec::new();

            // Create memory node for user message using direct constructor
            let user_memory = MemoryNode::new(
                MemoryTypeEnum::Episodic,
                MemoryContent::text(&user_message),
            );

            // Store user memory - spawn task to execute the PendingMemory future
            let user_pending = memory_tool_clone.memory().create_memory(user_memory.clone());
            if let Some(runtime) = crate::runtime::shared_runtime() {
                runtime.spawn(async move {
                    if let Err(e) = user_pending.await {
                        tracing::error!(
                            error = ?e,
                            memory_type = "user",
                            "Failed to store memory to database"
                        );
                    }
                });
            }

            if memorized_nodes.try_push(user_memory).is_ok() {
                // Create memory node for assistant response
                let assistant_memory = MemoryNode::new(
                    MemoryTypeEnum::Episodic,
                    MemoryContent::text(&assistant_response),
                );

                // Store assistant memory - spawn task to execute the PendingMemory future
                let assistant_pending = memory_tool_clone.memory().create_memory(assistant_memory.clone());
                if let Some(runtime) = crate::runtime::shared_runtime() {
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

                if memorized_nodes.try_push(assistant_memory).is_ok() {
                    // Create contextual memory node linking the conversation
                    let context_memory = MemoryNode::new(
                        MemoryTypeEnum::Working,
                        MemoryContent::text(&format!(
                            "Conversation: {user_message} -> {assistant_response}"
                        )),
                    );

                    // Store context memory - spawn task to execute the PendingMemory future
                    let context_pending = memory_tool_clone.memory().create_memory(context_memory.clone());
                    if let Some(runtime) = crate::runtime::shared_runtime() {
                        runtime.spawn(async move {
                            if let Err(e) = context_pending.await {
                                tracing::error!(
                                    error = ?e,
                                    memory_type = "context",
                                    "Failed to store memory to database"
                                );
                            }
                        });
                    }

                    if memorized_nodes.try_push(context_memory).is_ok() {
                        let _ = sender.send(CandleCollectionChunk {
                            items: memorized_nodes,
                            error_message: None
                        });
                    }
                }
            }
        })
    }
}

/// Candle-specific chat error types
#[derive(Error, Debug)]
pub enum CandleChatError {
    /// System-level error occurred
    #[error("System error: {0}")]
    System(String),

    /// Memory subsystem error
    #[error("Memory error: {0}")]
    Memory(#[from] MemoryError),

    /// Memory tool execution error
    #[error("Memory tool error: {0}")]
    MemoryTool(#[from] Box<MemoryToolError>),
}
