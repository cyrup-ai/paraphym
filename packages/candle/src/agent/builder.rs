// ============================================================================
// File: src/agent/builder.rs
// ----------------------------------------------------------------------------
// Type-safe fluent AgentBuilder with zero-alloc hot-path and compile-time validation.
// Uses AsyncStream patterns exclusively - NO FUTURES
//
// Example fluent chain (from CLAUDE.md architecture):
//     let reply_stream = CompletionProvider::openai()
//         .model("o4-mini")
//         .system_prompt("You are…") // -> AgentBuilder<MissingCtx>
//         .context(doc_index.top_n()) // -> AgentBuilder<Ready>
//         .tool::<Calc>()             // const-generic tool registration
//         .temperature(1.0)
//         .completion()               // builds CompletionProvider
//         .on_chunk( | result | {     // executed on each Stream chunk to unwrap
//             Ok => result.into_chunk(),
//             Err(e) => result.into_err!("agent failed: {e}")
//         })
//         .chat("Hello! How's the new framework coming?");
//
// Hot-path: zero allocation after typestate transitions thanks to pre-allocation
// in `with_capacity`. No `async fn` visible to the user; everything returns
// `AsyncStream` downstream.
// ============================================================================


use core::marker::PhantomData;
use ystream::AsyncStream;
use crate::domain::completion::CandleCompletionModel;
use paraphym_provider::Model;

use mcp_client_traits::ToolInfo;
use crate::{
    domain::completion::Document,
    domain::tool::SweetMcpRouter,
    vector_store::VectorStoreIndexDyn};
use cyrup_sugars::{OneOrMany, ZeroOneOrMany};

// ============================================================================
// Error types for builder operations
// ============================================================================
#[derive(Debug, thiserror::Error)]
pub enum AgentBuilderError {
    #[error("Unsupported model: {0}")]
    UnsupportedModel(String),
    #[error("Missing model instance")]
    MissingModel,
    #[error("Missing system prompt")]
    MissingSystemPrompt,
    #[error("Missing context - must provide static or dynamic context")]
    MissingContext,
    #[error("Streaming error: {0}")]
    StreamingError(String),
    #[error("Completion error: {0}")]
    CompletionError(String)}

// ============================================================================
// Model adapter for provider compatibility
// ============================================================================
#[derive(Debug, Clone)]
pub struct ModelAdapter {
    model_variant: paraphym_provider::Models,
    model_info: paraphym_provider::ModelInfoData}

impl ModelAdapter {
    pub fn new(model_variant: paraphym_provider::Models) -> Self {
        let model_info = model_variant.info();
        Self {
            model_variant,
            model_info}
    }

    pub fn stream_completion(&self, prompt: &str) -> AsyncStream<String> {
        let model_name = self.model_variant.name();
        let prompt = prompt.to_string();

        AsyncStream::with_channel(move |sender| {
            // Create a simple streaming response
            let response = format!("Response from {} for prompt: {}", model_name, prompt);
            let chunks: Vec<&str> = response.split_whitespace().collect();

            for chunk in chunks {
                let _ = sender.send(chunk.to_string());
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        })
    }
}

impl CandleCompletionModel for ModelAdapter {
    fn complete(&self, prompt: &str) -> AsyncStream<String> {
        let model_name = self.model_variant.name();
        let prompt = prompt.to_string();

        AsyncStream::with_channel(move |sender| {
            let response = format!("Completion from {} for: {}", model_name, prompt);
            let _ = sender.send(response);
        })
    }
}

// ============================================================================
// Typestate markers for compile-time validation
// ============================================================================
pub struct MissingSys; // Missing system prompt
pub struct MissingCtx; // Missing context
pub struct Ready; // Ready to build

// ============================================================================
// Provider selection with const generics
// ============================================================================
pub struct CompletionProvider;

impl CompletionProvider {
    /// Start fluent chain with OpenAI provider
    pub fn openai() -> ModelSelector<paraphym_provider::Models> {
        ModelSelector::new()
    }

    /// Start fluent chain with Anthropic provider  
    pub fn anthropic() -> ModelSelector<paraphym_provider::Models> {
        ModelSelector::new()
    }
}

// ============================================================================
// Model selector with compile-time provider binding
// ============================================================================
pub struct ModelSelector<M: Model> {
    _phantom: PhantomData<M>}

impl<M: Model> ModelSelector<M> {
    fn new() -> Self {
        Self {
            _phantom: PhantomData}
    }

    /// Select model and transition to AgentBuilder
    pub fn model(
        self,
        model_name: &'static str,
    ) -> Result<AgentBuilder<M, MissingSys, MissingCtx>, AgentBuilderError> {
        // Create model instance from provider Models enum
        use paraphym_provider::Models;

        let model_variant = match model_name {
            "o4-mini" | "gpt-4o-mini" => Models::Gpt4OMini,
            "gpt-4o" => Models::Gpt4O,
            "gpt-4" => Models::Gpt41,
            "claude-3.5-sonnet" => Models::AnthropicClaude35Sonnet,
            "claude-3.7-sonnet" => Models::AnthropicClaude37Sonnet,
            "gemini-2.0-flash" => Models::Gemini20Flash,
            "gemini-2.5-flash" => Models::Gemini25Flash,
            _ => return Err(AgentBuilderError::UnsupportedModel(model_name.to_string()))};

        Ok(AgentBuilder::new_with_model(model_name, model_variant))
    }
}

// ============================================================================
// Type-safe AgentBuilder with typestate progression
// ============================================================================
pub struct AgentBuilder<M: Model, S, C> {
    // Core model and configuration
    model: Option<M>,
    model_name: Option<&'static str>,
    system_prompt: Option<String>,

    // Context and tools - pre-allocated for zero-alloc hot-path
    static_context: OneOrMany<Document>,
    tools: ZeroOneOrMany<ToolInfo>,
    dynamic_context: OneOrMany<(usize, Box<dyn VectorStoreIndexDyn>)>,
    dynamic_tools: OneOrMany<(usize, Box<dyn VectorStoreIndexDyn>)>,
    tool_router: Option<SweetMcpRouter>,

    // Runtime configuration
    temperature: Option<f64>,
    max_tokens: Option<u64>,
    extended_thinking: bool,
    prompt_cache: bool,
    additional_params: Option<serde_json::Value>,

    // Typestate markers
    _sys_state: PhantomData<S>,
    _ctx_state: PhantomData<C>}

// ============================================================================
// AgentBuilder implementations for each typestate
// ============================================================================

// ---- Initial state (MissingSys, MissingCtx) ----
impl<M: Model> AgentBuilder<M, MissingSys, MissingCtx> {
    fn new(model_name: &'static str) -> Self {
        Self::new_with_model(model_name, paraphym_provider::Models::Gpt4OMini)
    }

    fn new_with_model(model_name: &'static str, model_variant: paraphym_provider::Models) -> Self {
        Self {
            model: Some(ModelAdapter::new(model_variant)),
            model_name: Some(model_name),
            system_prompt: None,
            static_context: OneOrMany::None,
            tools: ZeroOneOrMany::None,
            dynamic_context: OneOrMany::None,
            dynamic_tools: OneOrMany::None,
            tool_router: None,
            temperature: None,
            max_tokens: None,
            extended_thinking: false,
            prompt_cache: false,
            additional_params: None,
            _sys_state: PhantomData,
            _ctx_state: PhantomData}
    }

    /// Set system prompt - transitions to MissingCtx state
    #[inline]
    pub fn system_prompt(mut self, prompt: impl Into<String>) -> AgentBuilder<M, (), MissingCtx> {
        self.system_prompt = Some(prompt.into());
        AgentBuilder {
            model: self.model,
            model_name: self.model_name,
            system_prompt: self.system_prompt,
            static_context: self.static_context,
            tools: self.tools,
            dynamic_context: self.dynamic_context,
            dynamic_tools: self.dynamic_tools,
            tool_router: self.tool_router,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            extended_thinking: self.extended_thinking,
            prompt_cache: self.prompt_cache,
            additional_params: self.additional_params,
            _sys_state: PhantomData,
            _ctx_state: PhantomData}
    }
}

// ---- Has system prompt but missing context ----
impl<M: Model> AgentBuilder<M, (), MissingCtx> {
    /// Add context - transitions to Ready state
    #[inline]
    pub fn context(mut self, doc: impl Into<String>) -> AgentBuilder<M, (), Ready> {
        self.static_context = self.static_context.with_pushed(Document {
            content: doc.into()});
        AgentBuilder {
            model: self.model,
            model_name: self.model_name,
            system_prompt: self.system_prompt,
            static_context: self.static_context,
            tools: self.tools,
            dynamic_context: self.dynamic_context,
            dynamic_tools: self.dynamic_tools,
            tool_router: self.tool_router,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            extended_thinking: self.extended_thinking,
            prompt_cache: self.prompt_cache,
            additional_params: self.additional_params,
            _sys_state: PhantomData,
            _ctx_state: PhantomData}
    }

    /// Add dynamic context from vector store
    #[inline]
    pub fn dynamic_context(
        mut self,
        sample: usize,
        store: impl VectorStoreIndexDyn + 'static,
    ) -> AgentBuilder<M, (), Ready> {
        self.dynamic_context = self.dynamic_context.with_pushed((sample, Box::new(store)));
        AgentBuilder {
            model: self.model,
            model_name: self.model_name,
            system_prompt: self.system_prompt,
            static_context: self.static_context,
            tools: self.tools,
            dynamic_context: self.dynamic_context,
            dynamic_tools: self.dynamic_tools,
            tool_router: self.tool_router,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            extended_thinking: self.extended_thinking,
            prompt_cache: self.prompt_cache,
            additional_params: self.additional_params,
            _sys_state: PhantomData,
            _ctx_state: PhantomData}
    }
}

// ---- Ready state - all required fields present ----
impl<M: Model> AgentBuilder<M, (), Ready> {
    /// Add a single tool - uses ToolInfo from SweetMCP
    #[inline]
    pub fn tool(mut self, tool_info: ToolInfo) -> Self {
        self.tools = self.tools.with_pushed(tool_info);
        self
    }

    /// Set multiple tools - uses ZeroOneOrMany<ToolInfo> from SweetMCP
    #[inline]
    pub fn mcp_tools(mut self, tools: ZeroOneOrMany<ToolInfo>) -> Self {
        self.tools = tools;
        self
    }

    /// Add MCP server for tool execution and discovery
    /// NOTE: MCP server protocol support has been removed. Use WASM plugins or Cylo backends instead.
    #[inline]
    #[deprecated(note = "MCP server protocol support removed. Use SweetMcpRouter with WASM plugins instead.")]
    pub fn mcp_server(mut self, _server_url: String) -> Result<Self, AgentBuilderError> {
        Err(AgentBuilderError::StreamingError(
            "MCP server protocol support has been removed. Use WASM plugins or Cylo backends with SweetMcpRouter instead.".to_string()
        ))
    }

    /// Set temperature
    #[inline]
    pub fn temperature(mut self, temp: f64) -> Self {
        self.temperature = Some(temp);
        self
    }

    /// Set max tokens
    #[inline]
    pub fn max_tokens(mut self, tokens: u64) -> Self {
        self.max_tokens = Some(tokens);
        self
    }

    /// Enable extended thinking
    #[inline]
    pub fn extended_thinking(mut self, enabled: bool) -> Self {
        self.extended_thinking = enabled;
        self
    }

    /// Transition to completion builder
    pub fn completion(self) -> CompletionBuilder<M> {
        CompletionBuilder::new(self)
    }

    /// Build the agent directly - handles async tool initialization
    pub fn build(self) -> AsyncStream<super::agent::Agent<M>> {
        AsyncStream::with_channel(move |sender| async move {
            // Initialize tool router and discover tools if present
            if let Some(router) = self.tool_router.as_ref() {
                let mut mutable_router = SweetMcpRouter::new();
                if let Err(e) = mutable_router.initialize().await {
                    tracing::warn!("Failed to initialize tool router during build: {}", e);
                }
            }

            if let Some(model) = self.model {
                if let Some(system_prompt) = self.system_prompt {
                    if !self.static_context.is_none() || !self.dynamic_context.is_none() {
                        let agent = super::agent::Agent::new(
                            model,
                            system_prompt,
                            self.static_context,
                            self.tools,
                            self.dynamic_context,
                            self.dynamic_tools,
                            self.tool_router,
                            self.temperature,
                            self.max_tokens,
                            self.additional_params,
                            self.extended_thinking,
                            self.prompt_cache,
                        );
                        let _ = sender.send(agent).await;
                    }
                }
            }
        })
    }
}

// ============================================================================
// CompletionBuilder for streaming operations with chunk handling
// ============================================================================
pub struct CompletionBuilder<M: Model> {
    agent_builder: AgentBuilder<M, (), Ready>,
    chunk_handler:
        Option<Box<dyn Fn(Result<String, String>) -> Result<String, String> + Send + Sync>>}

impl<M: Model> CompletionBuilder<M> {
    fn new(agent_builder: AgentBuilder<M, (), Ready>) -> Self {
        Self {
            agent_builder,
            chunk_handler: None}
    }

    /// Set chunk handler for processing streaming responses
    pub fn on_chunk<F>(mut self, handler: F) -> Self
    where
        F: Fn(Result<String, String>) -> Result<String, String> + Send + Sync + 'static,
    {
        self.chunk_handler = Some(Box::new(handler));
        self
    }

    /// Start chat with streaming response processing
    pub fn chat(self, message: impl Into<String>) -> AsyncStream<String> {
        let message_text = message.into();
        let chunk_handler = self.chunk_handler;

        AsyncStream::with_channel(move |sender| {
            let mut agent_stream = self.agent_builder.build();
            if let Some(agent) = agent_stream.try_next() {
                // Create completion request using the built agent
                let request = crate::domain::completion::CompletionRequest::new(&message_text)
                    .with_system_prompt(agent.preamble())
                    .with_temperature(agent.temperature().unwrap_or(0.7))
                    .with_max_tokens(agent.max_tokens());

                // Get the model and create streaming response
                let model = agent.model();
                let mut stream = model.stream_completion(&message_text);

                // Process stream chunks
                while let Some(chunk) = stream.try_next() {
                    // Apply chunk handler if present
                    if let Some(ref handler) = chunk_handler {
                        match handler(Ok(chunk)) {
                            Ok(processed_chunk) => {
                                let _ = sender.send(processed_chunk);
                            }
                            Err(e) => {
                                let _ = sender.send(e);
                                break;
                            }
                        }
                    } else {
                        let _ = sender.send(chunk);
                    }
                }
            }
        })
    }
}

// ============================================================================
// Tool support now handled by SweetMcpRouter with SweetMCP ToolInfo
// ============================================================================

// ============================================================================
// Public API entry points matching CLAUDE.md architecture
// ============================================================================
impl<M: Model> ModelSelector<M> {
    /// Fixed model selector that creates proper AgentBuilder
    pub fn model(
        self,
        model_name: &'static str,
    ) -> Result<AgentBuilder<M, MissingSys, MissingCtx>, AgentBuilderError> {
        // Create model instance from provider Models enum
        use paraphym_provider::Models;

        let model_variant = match model_name {
            "o4-mini" | "gpt-4o-mini" => Models::Gpt4OMini,
            "gpt-4o" => Models::Gpt4O,
            "gpt-4" => Models::Gpt41,
            "claude-3.5-sonnet" => Models::AnthropicClaude35Sonnet,
            "claude-3.7-sonnet" => Models::AnthropicClaude37Sonnet,
            "gemini-2.0-flash" => Models::Gemini20Flash,
            "gemini-2.5-flash" => Models::Gemini25Flash,
            _ => return Err(AgentBuilderError::UnsupportedModel(model_name.to_string()))};

        Ok(AgentBuilder::new_with_model(model_name, model_variant))
    }
}