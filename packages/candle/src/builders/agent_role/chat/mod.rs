//! Chat implementation for CandleAgentBuilder
//!
//! This module coordinates the chat functionality across several focused submodules:
//! - memory_ops: Memory initialization, context loading, and storage
//! - chat_orchestration: Main chat loop with tools and memory

mod memory_ops;
mod chat_orchestration;

use super::*;

impl CandleAgentBuilder for CandleAgentBuilderImpl {
    fn model(mut self, model: TextToTextModel) -> Self {
        self.text_to_text_model = model;
        self
    }

    fn embedding_model(mut self, model: TextEmbeddingModel) -> Self {
        self.text_embedding_model = Some(model);
        self
    }

    fn temperature(mut self, temp: f64) -> Self {
        self.temperature = temp;
        self
    }

    fn max_tokens(mut self, max: u64) -> Self {
        self.max_tokens = max;
        self
    }

    fn memory_read_timeout(mut self, timeout_ms: u64) -> Self {
        self.memory_read_timeout = timeout_ms;
        self
    }

    fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = prompt.into();
        self
    }

    fn additional_params<P2>(mut self, params: P2) -> Self
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

    fn metadata<Meta>(mut self, metadata: Meta) -> Self
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
    ) -> Self {
        self.context_file = Some(context1);
        self.context_files = Some(context2);
        self.context_directory = Some(context3);
        self.context_github = Some(context4);
        self
    }

    fn tools<T>(mut self, tools: T) -> Self
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

    fn add_mcp_server_config(self, _config: McpServerConfig) -> Self {
        // MCP servers are handled through tools
        self
    }

    fn on_chunk<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(CandleMessageChunk) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = CandleMessageChunk> + Send + 'static,
    {
        let wrapped = move |chunk: CandleMessageChunk| Box::pin(handler(chunk)) as Pin<Box<dyn std::future::Future<Output = CandleMessageChunk> + Send>>;
        self.on_chunk_handler = Some(Arc::new(wrapped));
        self
    }

    fn on_tool_result<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(&[String]) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        let wrapped = move |results: &[String]| Box::pin(handler(results)) as Pin<Box<dyn std::future::Future<Output = ()> + Send>>;
        self.on_tool_result_handler = Some(Arc::new(wrapped));
        self
    }

    fn on_conversation_turn<F, Fut>(mut self, handler: F) -> Self
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

    fn conversation_history(self, _history: impl ConversationHistoryArgs) -> Self {
        self
    }

    fn chat<F, Fut>(self, handler: F) -> Result<Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>, AgentError>
    where
        F: FnOnce(&CandleAgentConversation) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = CandleChatLoop> + Send + 'static,
    {
        chat_orchestration::execute_chat(self, handler)
    }

    fn chat_with_message(self, message: impl Into<String>) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>> {
        let msg = message.into();
        match chat_orchestration::execute_chat(self, move |_| {
            let msg = msg.clone();
            async move { CandleChatLoop::UserPrompt(msg) }
        }) {
            Ok(stream) => stream,
            Err(_) => Box::pin(crate::async_stream::spawn_stream(|sender| async move {
                let _ = sender.send(CandleMessageChunk::Error("Chat failed".to_string()));
            })),
        }
    }
}
