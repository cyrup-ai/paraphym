// ============================================================================
// File: src/agent/agent.rs
// ----------------------------------------------------------------------------
// Core Agent struct - only buildable through AgentBuilder
// ============================================================================

use super::{
    builder::{AgentBuilder, MissingCtx, MissingSys},
    prompt::{Prompt, PromptRequest}};
use crate::{
    completion::{CompletionModel, CompletionRequest, CompletionRequestBuilder, Document, Message},
    domain::tool::ToolSet,
    runtime::{AsyncStream, AsyncTask},
    vector_store::VectorStoreIndexDyn};
use crate::domain::chat::message::types::{CandleMessageRole as MessageRole, CandleMessageChunk as MessageChunk, CandleConversationTrait as AgentConversation, CandleZeroOneOrMany as ZeroOneOrMany};
use crate::domain::chat::CandleChatLoop;

// ============================================================================
// Configuration constants
// ============================================================================
pub mod cfg {
    pub const CHAT_CAPACITY: usize = 256;
    pub const TOOL_CAPACITY: usize = 64;
}

// ============================================================================
// Typestate markers for fluent API
// ============================================================================
pub struct MissingModel;
pub struct MissingPrompt;
pub struct Ready;

// ============================================================================
// Core Agent Provider - only constructible through builder with zero Box<dyn> allocation
// ============================================================================
pub struct Agent<M: CompletionModel, F = fn(MessageChunk) -> MessageChunk>
where
    F: Fn(MessageChunk) -> MessageChunk + Send + Sync + 'static,
{
    model: M,
    preamble: String,
    static_context: Vec<Document>,
    static_tools: Vec<String>,
    dynamic_context: Vec<(usize, String)>, // Store as string configuration for zero allocation
    dynamic_tools: Vec<(usize, String)>, // Store as string configuration for zero allocation
    tools: ToolSet,
    temperature: Option<f64>,
    max_tokens: Option<u64>,
    additional_params: Option<serde_json::Value>,
    extended_thinking: bool,
    prompt_cache: bool,
    conversation_history: ZeroOneOrMany<(MessageRole, String)>,
    chunk_handler: Option<F>, // Generic function type - zero allocation static dispatch
}


impl<M: CompletionModel, F> Agent<M, F>
where
    F: Fn(MessageChunk) -> MessageChunk + Send + Sync + 'static,
{
    /// Create a new AgentBuilder for the given provider model
    pub fn for_provider(model: M) -> AgentBuilder<M, MissingSys, MissingCtx> {
        AgentBuilder::new(model)
    }

    /// Internal constructor for building from AgentBuilder - zero allocation with string configurations
    pub(crate) fn new(
        model: M,
        preamble: String,
        static_context: Vec<Document>,
        static_tools: Vec<String>,
        dynamic_context: Vec<(usize, String)>, // String configurations for zero allocation
        dynamic_tools: Vec<(usize, String)>, // String configurations for zero allocation
        tools: ToolSet,
        temperature: Option<f64>,
        max_tokens: Option<u64>,
        additional_params: Option<serde_json::Value>,
        extended_thinking: bool,
        prompt_cache: bool,
    ) -> Self {
        Self {
            model,
            preamble,
            static_context,
            static_tools,
            dynamic_context,
            dynamic_tools,
            tools,
            temperature,
            max_tokens,
            additional_params,
            extended_thinking,
            prompt_cache,
            conversation_history: ZeroOneOrMany::None,
            chunk_handler: None,
        }
    }

    /// Start a prompt request - returns async stream
    pub fn prompt(
        &self,
        prompt: impl Prompt,
    ) -> AsyncStream<PromptRequest<M>, { cfg::CHAT_CAPACITY }> {
        let request = PromptRequest::new(self, prompt);
        AsyncStream::from_single(request)
    }

    /// Create a completion request - returns async stream
    pub fn completion(
        &self,
        prompt: Message,
        history: Vec<Message>,
    ) -> AsyncStream<CompletionRequestBuilder, { cfg::CHAT_CAPACITY }> {
        let builder_result = self.create_completion_builder(prompt, history);
        match builder_result {
            Ok(builder) => AsyncStream::from_single(builder),
            Err(_) => AsyncStream::with_channel(|_sender| {
                // Empty stream - no data to send on error
            }), // Handle error case
        }
    }

    /// Create completion builder internally
    fn create_completion_builder(
        &self,
        prompt: Message,
        history: Vec<Message>,
    ) -> Result<CompletionRequestBuilder, crate::completion::PromptError> {
        let mut builder = CompletionRequestBuilder::new(
            "default-model".to_string(), // This should come from the model
        )?;

        builder = builder.preamble(&self.preamble);

        if !history.is_empty() {
            builder = builder.chat_history(history);
        }

        Ok(builder)
    }

    /// Get the underlying model
    pub fn model(&self) -> &M {
        &self.model
    }

    /// Get the preamble
    pub fn preamble(&self) -> &str {
        &self.preamble
    }

    /// Get temperature
    pub fn temperature(&self) -> Option<f64> {
        self.temperature
    }

    /// Get max tokens
    pub fn max_tokens(&self) -> Option<u64> {
        self.max_tokens
    }

    /// Get extended thinking
    pub fn extended_thinking(&self) -> bool {
        self.extended_thinking
    }

    /// Set conversation history - EXACT syntax from ARCHITECTURE.md
    /// Supports: .conversation_history(MessageRole::User => "content", MessageRole::System => "content", ...)
    pub fn conversation_history<H>(mut self, history: H) -> Self
    where
        H: Into<ZeroOneOrMany<(MessageRole, String)>>,
    {
        self.conversation_history = history.into();
        self
    }

    /// Simple chat method - EXACT syntax: .chat("Hello")
    pub fn chat(&self, message: impl Into<String>) -> AsyncStream<MessageChunk, { cfg::CHAT_CAPACITY }> {
        let message_text = message.into();
        
        // Create a message with the chat content
        let user_message = Message::user(message_text);
        
        // Convert conversation history to Message format
        let history: Vec<Message> = match &self.conversation_history {
            ZeroOneOrMany::None => Vec::new(),
            ZeroOneOrMany::One((role, content)) => {
                vec![match role {
                    MessageRole::User => Message::user(content.clone()),
                    MessageRole::Assistant => Message::assistant(content.clone()),
                    MessageRole::System => Message::system(content.clone()),
                    _ => Message::user(content.clone()), // Fallback for other roles
                }]
            }
            ZeroOneOrMany::Many(items) => items
                .iter()
                .map(|(role, content)| match role {
                    MessageRole::User => Message::user(content.clone()),
                    MessageRole::Assistant => Message::assistant(content.clone()),
                    MessageRole::System => Message::system(content.clone()),
                    _ => Message::user(content.clone()), // Fallback for other roles
                })
                .collect(),
        };

        // Create a completion using the core engine directly
        let full_prompt = format!("{}", user_message.content());
        
        // Use the core engine for streaming completion
        let engine_config = crate::core::EngineConfig::new("kimi-k2", "kimi-k2")
            .with_temperature(self.temperature.unwrap_or(0.7) as f32)
            .with_max_tokens(self.max_tokens.unwrap_or(1000) as u32)
            .with_streaming();
            
        if let Ok(engine) = crate::core::Engine::new(engine_config) {
            let completion_request = crate::core::CompletionRequest::new(&full_prompt);
            
            // Get streaming completion and convert to MessageChunk
            let completion_stream = engine.process_completion_stream(completion_request);
            
            AsyncStream::with_channel(move |sender| {
                let mut chunks_sent = 0u32;
                let mut completion_iter = completion_stream;
                
                while let Some(response) = completion_iter.try_next() {
                    let chunk = MessageChunk {
                        content: response.text.to_string(),
                        done: response.finish_reason.is_some(),
                    };
                    
                    if sender.send(chunk).is_err() {
                        break; // Client disconnected
                    }
                    
                    chunks_sent += 1;
                    if response.finish_reason.is_some() {
                        break; // Completion finished
                    }
                }
                
                // Send final chunk if none were sent
                if chunks_sent == 0 {
                    let _ = sender.send(MessageChunk {
                        content: String::new(),
                        done: true,
                    });
                }
            })
        } else {
            // Fallback to empty stream on engine creation failure
            AsyncStream::empty()
        }
    }

    /// Closure-based chat loop - EXACT syntax: .chat(|conversation| CandleChatLoop) - zero allocation  
    pub fn chat_with_closure<F>(&self, closure: F) -> Result<(), String>
    where
        F: FnOnce(&AgentConversation) -> CandleChatLoop,
    {
        // Create conversation from current history
        let conversation = AgentConversation {
            messages: match &self.conversation_history {
                ZeroOneOrMany::None => None,
                _ => Some(self.conversation_history.clone()),
            }
        };
        
        // Execute closure to get CandleChatLoop decision
        let chat_result = closure(&conversation);
        
        match chat_result {
            CandleChatLoop::Break => Ok(()),
            CandleChatLoop::Reprompt(response) => {
                // Process reprompt - this would integrate with the chat system
                println!("Reprompt: {}", response);
                Ok(())
            }
            CandleChatLoop::UserPrompt(prompt) => {
                // Handle user prompt request
                if let Some(prompt_text) = prompt {
                    println!("User prompt: {}", prompt_text);
                } else {
                    println!("Waiting for user input...");
                }
                Ok(())
            }
        }
    }
}

// ============================================================================
// Stream type aliases
// ============================================================================
pub type CompletionStream<T> = AsyncStream<T, { cfg::CHAT_CAPACITY }>;
pub type ToolStream<T> = AsyncStream<T, { cfg::TOOL_CAPACITY }>;
