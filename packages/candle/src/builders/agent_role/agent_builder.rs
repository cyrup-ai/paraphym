//! CandleAgentBuilderImpl - builder with model

use super::*;

pub struct AgentDebugInfo {
    pub name: String,
    pub temperature: Option<f64>,
    pub max_tokens: Option<u64>,
    pub has_system_prompt: bool,
}

/// Agent builder implementation
pub struct CandleAgentBuilderImpl {
    name: String,
    text_to_text_model: TextToTextModel,
    text_embedding_model: Option<TextEmbeddingModel>,
    temperature: f64,
    max_tokens: u64,
    memory_read_timeout: u64,
    system_prompt: String,
    tools: ZeroOneOrMany<ToolInfo>,
    context_file: Option<CandleContext<CandleFile>>,
    context_files: Option<CandleContext<CandleFiles>>,
    context_directory: Option<CandleContext<CandleDirectory>>,
    context_github: Option<CandleContext<CandleGithub>>,
    additional_params: std::collections::HashMap<String, String>,
    metadata: std::collections::HashMap<String, String>,
    on_chunk_handler: Option<OnChunkHandler>,
    on_tool_result_handler: Option<OnToolResultHandler>,
    on_conversation_turn_handler: Option<OnConversationTurnHandler>,
}

impl std::fmt::Debug for CandleAgentBuilderImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CandleAgentBuilderImpl")
            .field("name", &self.name)
            .field("text_to_text_model", &self.text_to_text_model)
            .field("text_embedding_model", &self.text_embedding_model)
            .field("temperature", &self.temperature)
            .field("max_tokens", &self.max_tokens)
            .field("memory_read_timeout", &self.memory_read_timeout)
            .field(
                "system_prompt",
                &format!(
                    "{}...",
                    &self.system_prompt[..self.system_prompt.len().min(50)]
                ),
            )
            .field("tools", &self.tools)
            .field("additional_params", &self.additional_params)
            .field("metadata", &self.metadata)
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
        self.max_tokens = max;
        self
    }

    fn memory_read_timeout(mut self, timeout_ms: u64) -> impl CandleAgentRoleBuilder {
        self.memory_read_timeout = timeout_ms;
        self
    }

    fn system_prompt(mut self, prompt: impl Into<String>) -> impl CandleAgentRoleBuilder {
        self.system_prompt = prompt.into();
        self
    }

    fn additional_params<P2>(mut self, params: P2) -> impl CandleAgentRoleBuilder
    where
        P2: IntoIterator<Item = (&'static str, &'static str)>,
    {
        self.additional_params.extend(
            params
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string())),
        );
        self
    }

    fn metadata<Meta>(mut self, metadata: Meta) -> impl CandleAgentRoleBuilder
    where
        Meta: IntoIterator<Item = (&'static str, &'static str)>,
    {
        self.metadata.extend(
            metadata
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string())),
        );
        self
    }

    fn context(
        mut self,
        context1: CandleContext<CandleFile>,
        context2: CandleContext<CandleFiles>,
        context3: CandleContext<CandleDirectory>,
        context4: CandleContext<CandleGithub>,
    ) -> impl CandleAgentRoleBuilder {
        self.context_file = Some(context1);
        self.context_files = Some(context2);
        self.context_directory = Some(context3);
        self.context_github = Some(context4);
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
    fn add_mcp_server_config(self, _config: McpServerConfig) -> impl CandleAgentRoleBuilder {
        // MCP servers are handled through tools
        self
    }

    fn on_chunk<F, Fut>(mut self, handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(CandleMessageChunk) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = CandleMessageChunk> + Send + 'static,
    {
        let wrapped = move |chunk: CandleMessageChunk| Box::pin(handler(chunk)) as Pin<Box<dyn std::future::Future<Output = CandleMessageChunk> + Send>>;
        self.on_chunk_handler = Some(Arc::new(wrapped));
        self
    }

    fn on_tool_result<F, Fut>(mut self, handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(&[String]) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        let wrapped = move |results: &[String]| Box::pin(handler(results)) as Pin<Box<dyn std::future::Future<Output = ()> + Send>>;
        self.on_tool_result_handler = Some(Arc::new(wrapped));
        self
    }

    /// Set conversation turn handler - EXACT syntax: .on_conversation_turn(|conversation, agent| async move { ... })
    fn on_conversation_turn<F, Fut>(mut self, handler: F) -> impl CandleAgentRoleBuilder
    where
        F: Fn(&CandleAgentConversation, &CandleAgentRoleAgent) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>> + Send + 'static,
    {
        // Wrap the async handler to match the type alias
        let wrapped_handler = move |conv: &CandleAgentConversation, agent: &CandleAgentRoleAgent| {
            Box::pin(handler(conv, agent)) as Pin<Box<dyn std::future::Future<Output = Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>> + Send>>
        };
        self.on_conversation_turn_handler = Some(Arc::new(wrapped_handler));
        self
    }

    fn conversation_history(
        self,
        _history: impl ConversationHistoryArgs,
    ) -> impl CandleAgentRoleBuilder {
        self
    }

    /// Chat with closure - EXACT syntax: .chat(|conversation| ChatLoop)
    fn chat<F>(self, _handler: F) -> Result<Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>, AgentError>
    where
        F: FnOnce(&CandleAgentConversation) -> CandleChatLoop + Send + 'static,
    {
        Ok(Box::pin(crate::async_stream::spawn_stream(|sender| async move {
            let _ = sender.send(CandleMessageChunk::Text("Hello from Candle!".to_string()));
        })))
    }

    fn chat_with_message(self, message: impl Into<String>) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>> {
        let msg = message.into();
        // Use CandleAgentRoleBuilder::chat explicitly to avoid ambiguity
        CandleAgentRoleBuilder::chat(self, move |_| CandleChatLoop::UserPrompt(msg)).unwrap_or_else(
            |_| {
                Box::pin(crate::async_stream::spawn_stream(|sender| async move {
                    let _ = sender.send(CandleMessageChunk::Error("Chat failed".to_string()));
                }))
            },
        )
    }

    fn into_agent(self) -> impl CandleAgentBuilder {
        self
    }
}
