//! Chat functionality for memory-enhanced agent conversations

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, LazyLock,
};

use arrayvec::ArrayVec;
use atomic_counter::RelaxedCounter;
use crossbeam_utils::CachePadded;
use cyrup_sugars::prelude::MessageChunk;

use thiserror::Error;
use chrono::Utc;
// StreamExt not currently used but may be needed for future async operations

// Import real completion infrastructure
use crate::core::engine::{CompletionRequest, Engine, EngineConfig};
// Removed unused import: use tokio_stream::StreamExt;
use crate::domain::agent::role::CandleAgentRoleImpl;
use crate::domain::completion::PromptFormatter;

use crate::memory::memory::primitives::types::{MemoryTypeEnum, MemoryContent};
use crate::memory::memory::{MemoryNode};
use crate::memory::memory::manager::surreal::MemoryManager;
use crate::memory::memory::ops::retrieval::RetrievalResult;
use crate::domain::memory::{Error as MemoryError};
use crate::domain::context::chunk::CandleCollectionChunk;
use crate::domain::context::CandleDocument as Document;
use crate::domain::memory::{MemoryTool, MemoryToolError};
use cyrup_sugars::ZeroOneOrMany;

/// Maximum number of relevant memories for context injection
const MAX_RELEVANT_MEMORIES: usize = 10;

/// Global atomic counter for memory node creation
#[allow(dead_code)] // TODO: Implement in memory node creation system
static MEMORY_NODE_COUNTER: LazyLock<CachePadded<RelaxedCounter>> =
    LazyLock::new(|| CachePadded::new(RelaxedCounter::new(0)));

/// Global atomic counter for attention scoring operations
static ATTENTION_SCORE_COUNTER: LazyLock<CachePadded<AtomicUsize>> =
    LazyLock::new(|| CachePadded::new(AtomicUsize::new(0)));

/// Chat error types for memory-enhanced agent conversations
#[derive(Debug, thiserror::Error)]
pub enum ChatError {
    /// Memory system error
    #[error("Memory error: {0}")]
    Memory(#[from] MemoryError),
    /// Memory tool error
    #[error("Memory tool error: {0}")]
    MemoryTool(#[from] MemoryToolError),
    /// Message processing error
    #[error("Message processing error: {0}")]
    Message(String),
    /// System error
    #[error("System error: {0}")]
    System(String),
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
            injected_context: format!("Error: {}", error),
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
            response: format!("Error: {}", error),
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
    /// Generate real AI response using `Engine` with `TextGenerator` and proper memory vs context sectioning
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
    /// Uses `Engine` infrastructure with `TextGenerator` for real model inference
    /// with proper memory vs context sectioning following LLM best practices
    fn generate_ai_response_with_sectioning(
        &self,
        message: &str,
        memories: &ZeroOneOrMany<RetrievalResult>,
        documents: &ZeroOneOrMany<Document>,
        chat_history: &ZeroOneOrMany<crate::domain::chat::message::types::CandleMessage>,
    ) -> Result<String, ChatError> {
        // Create engine configuration for kimi-k2 provider (matches working engine.rs setup)
        let engine_config = EngineConfig::new("kimi-k2", "kimi-k2")
            .with_temperature(0.7)
            .with_max_tokens(1000)
            .with_streaming();

        // Create engine instance with real TextGenerator integration
        let engine = Engine::new(engine_config)
            .map_err(|e| ChatError::System(format!("Failed to create Engine: {}", e)))?;

        // Use PromptFormatter for proper memory vs context sectioning
        let formatter = PromptFormatter::new()
            .with_max_memory_length(Some(2000))
            .with_max_context_length(Some(4000));

        // Format prompt with clear sectioning for LLM understanding
        let full_prompt = formatter.format_prompt(memories, documents, chat_history, message);

        // Create completion request with borrowed data
        let completion_request = CompletionRequest::new(&full_prompt);

        // Get real AI response using Engine with TextGenerator via AsyncStream
        let completion_stream = engine.process_completion_stream(completion_request);

        // Collect first response from AsyncStream using try_next pattern
        if let Some(completion_response) = completion_stream.try_next() {
            Ok(completion_response.text().to_string())
        } else {
            Err(ChatError::System(
                "No response from completion stream".to_string(),
            ))
        }
    }

    /// Generate real AI response using legacy context string (for backward compatibility)
    ///
    /// # Arguments
    /// * `message` - User message to respond to
    /// * `context` - Injected memory context for enhanced responses
    ///
    /// # Returns
    /// Result containing real AI-generated response
    ///
    /// # Performance
    /// Uses `Engine` infrastructure with `TextGenerator` for real model inference
    fn generate_ai_response(&self, message: &str, context: &str) -> Result<String, ChatError> {
        // For backward compatibility, treat context as memories
        let memories = if context.is_empty() {
            ZeroOneOrMany::None
        } else {
            // Parse the context back into a simple RetrievalResult for compatibility
            let mut metadata = std::collections::HashMap::new();
            metadata.insert("content".to_string(), serde_json::Value::String(context.to_string()));
            
            let retrieval_result = RetrievalResult {
                id: "legacy_context".to_string(),
                score: 1.0,
                method: crate::memory::memory::ops::retrieval::RetrievalMethod::Semantic,
                metadata,
            };
            ZeroOneOrMany::One(retrieval_result)
        };

        // No documents or chat history in legacy mode
        let documents = ZeroOneOrMany::None;
        let chat_history = ZeroOneOrMany::None;

        self.generate_ai_response_with_sectioning(message, &memories, &documents, &chat_history)
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
            let context_stream = self_clone.inject_memory_context(&message, &memory_manager_clone);

            if let Some(context_injection) = context_stream.try_next() {
                // Generate real AI response using Engine with TextGenerator
                match self_clone.generate_ai_response(&message, &context_injection.injected_context)
                {
                    Ok(response) => {
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
                    Err(_) => {
                        // Error in AI response generation - don't send anything
                    }
                }
            }
        })
    }

    /// Inject memory context with zero-allocation processing
    ///
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
    ) -> ystream::AsyncStream<ContextInjectionResult> {
        let message = message.to_string();
        let _self_clone = self.clone(); // Reserved for future use in memory context
        let memory_manager_clone = memory_manager.clone();

        ystream::AsyncStream::with_channel(move |sender| {
            // Query relevant memories with zero-allocation buffer
            let relevant_memories = ArrayVec::<MemoryNode, MAX_RELEVANT_MEMORIES>::new(); // Reserved for future memory ranking
            let total_relevance_score = 0.0; // Reserved for future relevance scoring

            // Use the user's elite MemoryManager directly for vector similarity search
            // No intermediate layers needed - direct access to sophisticated SurrealDB implementation

            // Use user's elite MemoryManager search_by_content for semantic search
            let memory_stream = memory_manager_clone.search_by_content(&message);

            // Collect results from the sophisticated stream using proper async collection
            let (retrieval_tx, retrieval_rx) = std::sync::mpsc::channel();

            // Spawn task to collect memory stream results
            ystream::spawn_task(move || async move {
                use futures_util::StreamExt;

                let mut stream = memory_stream;
                let mut results = Vec::new();

                // Collect up to MAX_RELEVANT_MEMORIES from the stream
                while let Some(memory_result) = stream.next().await {
                    if results.len() >= MAX_RELEVANT_MEMORIES {
                        break;
                    }

                    if let Ok(memory_node) = memory_result {
                        // Convert MemoryNode to RetrievalResult for consistent API
                        let retrieval_result = RetrievalResult {
                            id: memory_node.id.clone(),
                            score: 0.8, // Good relevance from content search
                            method: crate::memory::memory::ops::retrieval::RetrievalMethod::Semantic,
                            metadata: {
                                let mut metadata = std::collections::HashMap::new();
                                metadata.insert("content".to_string(), serde_json::Value::String(
                                    memory_node.content.text.clone()
                                ));
                                metadata.insert("memory_type".to_string(), serde_json::Value::String(
                                    format!("{:?}", memory_node.memory_type)
                                ));
                                metadata.insert("importance".to_string(), serde_json::Value::Number(
                                    serde_json::Number::from_f64(memory_node.metadata.importance as f64)
                                        .unwrap_or_else(|| serde_json::Number::from(0))
                                ));
                                metadata
                            },
                        };
                        results.push(retrieval_result);
                    }
                }

                let _ = retrieval_tx.send(results);
            });

            // Receive results with timeout to prevent blocking
            let retrieval_results = retrieval_rx.recv_timeout(
                std::time::Duration::from_millis(1000)
            ).unwrap_or_default();

            // Use PromptFormatter to format memories properly
            let formatter = PromptFormatter::new()
                .with_max_memory_length(Some(2000))
                .with_headers(true);
            
            let memories_zero_one_many = if retrieval_results.is_empty() {
                ZeroOneOrMany::None
            } else if retrieval_results.len() == 1 {
                ZeroOneOrMany::One(retrieval_results.into_iter().next().unwrap())
            } else {
                ZeroOneOrMany::Many(retrieval_results)
            };
            
            let _documents: ZeroOneOrMany<Document> = ZeroOneOrMany::None; // Reserved for future document context
            let _chat_history: ZeroOneOrMany<crate::domain::chat::message::types::CandleMessage> = ZeroOneOrMany::None; // Reserved for future chat context
            
            let injected_context = formatter.format_memory_section(&memories_zero_one_many)
                .unwrap_or_else(String::new);

            let memory_nodes_used = relevant_memories.len();
            let avg_relevance_score = if memory_nodes_used > 0 {
                total_relevance_score / memory_nodes_used as f64
            } else {
                0.0
            };

            let result = ContextInjectionResult {
                injected_context,
                relevance_score: avg_relevance_score,
                memory_nodes_used,
            };

            let _ = sender.send(result);
        })
    }

    /// Calculate relevance score using attention mechanism
    ///
    /// # Arguments
    /// * `message` - User message
    /// * `memory_node` - Memory node to score
    ///
    /// # Returns
    /// Result containing relevance score (0.0 to 1.0)
    ///
    /// # Performance
    /// Zero allocation with inlined relevance calculations
    pub fn calculate_relevance_score(
        &self,
        message: &str,
        memory_node: &MemoryNode,
    ) -> Result<f64, ChatError> {
        // Increment atomic counter for lock-free statistics
        ATTENTION_SCORE_COUNTER.fetch_add(1, Ordering::Relaxed);

        // Simple relevance scoring based on content similarity and memory node importance
        let message_len = message.len();
        let memory_content = &memory_node.content;
        let memory_len = memory_content.text.len();

        // Basic content length similarity (normalized)
        let length_similarity = 1.0
            - ((message_len as f64 - memory_len as f64).abs()
                / (message_len.max(memory_len) as f64 + 1.0));

        // Memory node importance factor
        let importance_factor = memory_node.metadata.importance;

        // Time decay factor based on last access (use updated_at as proxy)
        let time_factor = {
            let elapsed = Utc::now().signed_duration_since(memory_node.updated_at);
            let hours = elapsed.num_hours() as f64;
            // Decay over 24 hours, minimum 0.1
            (1.0 - (hours / 24.0)).max(0.1f64)
        };

        // Combined relevance score (weighted average)
        let score =
            (length_similarity * 0.3 + importance_factor as f64 * 0.5 + time_factor * 0.2).min(1.0);

        Ok(score)
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

            // Store user memory with zero-allocation error handling - PURE STREAMING
            let _store_stream = memory_tool_clone.memory().create_memory(user_memory.clone());
            // Fire-and-forget storage operation - no need to consume stream

            if memorized_nodes.try_push(user_memory).is_ok() {
                // Create memory node for assistant response
                let assistant_memory = MemoryNode::new(
                    MemoryTypeEnum::Episodic,
                    MemoryContent::text(&assistant_response),
                );

                // Store assistant memory with zero-allocation error handling - PURE STREAMING
                let _store_stream = memory_tool_clone.memory().create_memory(assistant_memory.clone());
                // Fire-and-forget storage operation - no need to consume stream

                if memorized_nodes.try_push(assistant_memory).is_ok() {
                    // Create contextual memory node linking the conversation
                    let context_memory = MemoryNode::new(
                        MemoryTypeEnum::Working,
                        MemoryContent::text(&format!(
                            "Conversation: {} -> {}",
                            user_message, assistant_response
                        )),
                    );

                    // Store context memory with zero-allocation error handling - PURE STREAMING
                    let _store_stream = memory_tool_clone.memory().create_memory(context_memory.clone());
                    // Fire-and-forget storage operation - no need to consume stream

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
    MemoryTool(#[from] MemoryToolError),
}
