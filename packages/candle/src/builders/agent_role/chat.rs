//! Chat implementation for CandleAgentBuilder

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

    /// Chat with async closure - EXACT syntax: .chat(|conversation| async { ChatLoop })
    fn chat<F, Fut>(self, handler: F) -> Result<Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>, AgentError>
    where
        F: FnOnce(&CandleAgentConversation) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = CandleChatLoop> + Send + 'static,
    {
        let provider = self.text_to_text_model;
        let embedding_model = self.text_embedding_model;
        let temperature = self.temperature;
        let max_tokens = self.max_tokens;
        let memory_read_timeout = self.memory_read_timeout;
        let system_prompt = self.system_prompt.clone();
        let tools = self.tools;
        let on_conversation_turn_handler = self.on_conversation_turn_handler;
        let on_chunk_handler = self.on_chunk_handler;
        let on_tool_result_handler = self.on_tool_result_handler;
        let context_file = self.context_file;
        let context_files = self.context_files;
        let context_directory = self.context_directory;
        let context_github = self.context_github;
        let additional_params = self.additional_params;
        let metadata = self.metadata;
        let conversation_history = self.conversation_history;

        Ok(Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            // Create initial empty conversation for handler to inspect
            let initial_conversation = CandleAgentConversation::new();

            // Execute async handler to get CandleChatLoop result
            let chat_loop_result = handler(&initial_conversation).await;

            // Process CandleChatLoop result
            match chat_loop_result {
                CandleChatLoop::Break => {
                    // User wants to exit - send final chunk
                    let final_chunk = CandleMessageChunk::Complete {
                        text: String::new(),
                        finish_reason: Some("break".to_string()),
                        usage: None,
                        token_count: None,
                        elapsed_secs: None,
                        tokens_per_sec: None,
                    };
                    let _ = sender.send(final_chunk);
                }
                CandleChatLoop::UserPrompt(user_message)
                | CandleChatLoop::Reprompt(user_message) => {
                    // Memory initialization (only if embedding model available)
                    // Memory features require embeddings for search functionality
                    let memory: Option<Arc<dyn crate::memory::core::manager::surreal::MemoryManager>> = if let Some(ref emb_model) = embedding_model {
                        use surrealdb::engine::any::connect;
                        use crate::memory::core::manager::surreal::SurrealDBMemoryManager;
                        use crate::memory::primitives::node::MemoryNode;
                        use crate::memory::primitives::types::{MemoryContent, MemoryTypeEnum};
                        use chrono::Utc;

                        let db_path = dirs::cache_dir()
                            .unwrap_or_else(|| std::path::PathBuf::from("."))
                            .join("paraphym")
                            .join("agent.db");

                        // Ensure database directory exists
                        if let Some(parent) = db_path.parent()
                            && let Err(e) = tokio::fs::create_dir_all(parent).await
                        {
                            let _ = sender.send(CandleMessageChunk::Error(
                                format!("Failed to create database directory: {}", e)
                            ));
                            return;
                        }

                        let db_url = format!("surrealkv://{}", db_path.display());

                        // Connect to database using .await
                        let db = match connect(&db_url).await {
                            Ok(db) => db,
                            Err(e) => {
                                let _ = sender.send(CandleMessageChunk::Error(
                                    format!("Failed to connect to database: {}", e)
                                ));
                                return;
                            }
                        };

                        // Initialize database namespace using .await
                        if let Err(e) = db.use_ns("candle").use_db("agent").await {
                            let _ = sender.send(CandleMessageChunk::Error(
                                format!("Failed to initialize database namespace: {}", e)
                            ));
                            return;
                        }

                        // Create and initialize memory manager using .await
                        let manager = match SurrealDBMemoryManager::with_embedding_model(db, emb_model.clone()).await {
                        Ok(mgr) => mgr,
                        Err(e) => {
                            let _ = sender.send(CandleMessageChunk::Error(
                                format!("Failed to create memory manager: {}", e)
                            ));
                            return;
                        }
                    };

                        if let Err(e) = manager.initialize().await {
                            let _ = sender.send(CandleMessageChunk::Error(
                                format!("Failed to initialize memory tables: {}", e)
                            ));
                            return;
                        }

                        let mem: Arc<dyn crate::memory::core::manager::surreal::MemoryManager> =
                            Arc::new(manager) as Arc<dyn crate::memory::core::manager::surreal::MemoryManager>;

                        // Ingest documents from context fields into memory using .await
                        // Load from context_file
                        if let Some(ctx) = context_file {
                            let doc_stream = ctx.load();
                            tokio::pin!(doc_stream);
                            while let Some(doc) = doc_stream.next().await {
                                let content_hash = crate::domain::memory::serialization::content_hash(&doc.data);
                                let memory_node = MemoryNode {
                                    id: format!("doc_{}", content_hash),
                                    content: MemoryContent::new(&doc.data),
                                    content_hash,
                                    memory_type: MemoryTypeEnum::Semantic,
                                    created_at: Utc::now(),
                                    updated_at: Utc::now(),
                                    embedding: None,
                                    evaluation_status: crate::memory::monitoring::operations::OperationStatus::Pending,
                                    metadata: crate::memory::primitives::metadata::MemoryMetadata::new(),
                                    relevance_score: None,
                                };
                                if let Err(e) = mem.create_memory(memory_node).await {
                                    log::error!("Failed to ingest document: {:?}", e);
                                }
                            }
                        }

                        // Load from context_files
                        if let Some(ctx) = context_files {
                            let doc_stream = ctx.load();
                            tokio::pin!(doc_stream);
                            while let Some(doc) = doc_stream.next().await {
                                let content_hash = crate::domain::memory::serialization::content_hash(&doc.data);
                                let memory_node = MemoryNode {
                                    id: format!("doc_{}", content_hash),
                                    content: MemoryContent::new(&doc.data),
                                    content_hash,
                                    memory_type: MemoryTypeEnum::Semantic,
                                    created_at: Utc::now(),
                                    updated_at: Utc::now(),
                                    embedding: None,
                                    evaluation_status: crate::memory::monitoring::operations::OperationStatus::Pending,
                                    metadata: crate::memory::primitives::metadata::MemoryMetadata::new(),
                                    relevance_score: None,
                                };
                                if let Err(e) = mem.create_memory(memory_node).await {
                                    log::error!("Failed to ingest document: {:?}", e);
                                }
                            }
                        }

                        // Load from context_directory
                        if let Some(ctx) = context_directory {
                            let doc_stream = ctx.load();
                            tokio::pin!(doc_stream);
                            while let Some(doc) = doc_stream.next().await {
                                let content_hash = crate::domain::memory::serialization::content_hash(&doc.data);
                                let memory_node = MemoryNode {
                                    id: format!("doc_{}", content_hash),
                                    content: MemoryContent::new(&doc.data),
                                    content_hash,
                                    memory_type: MemoryTypeEnum::Semantic,
                                    created_at: Utc::now(),
                                    updated_at: Utc::now(),
                                    embedding: None,
                                    evaluation_status: crate::memory::monitoring::operations::OperationStatus::Pending,
                                    metadata: crate::memory::primitives::metadata::MemoryMetadata::new(),
                                    relevance_score: None,
                                };
                                if let Err(e) = mem.create_memory(memory_node).await {
                                    log::error!("Failed to ingest document: {:?}", e);
                                }
                            }
                        }

                        // Load from context_github
                        if let Some(ctx) = context_github {
                            let doc_stream = ctx.load();
                            tokio::pin!(doc_stream);
                            while let Some(doc) = doc_stream.next().await {
                                let content_hash = crate::domain::memory::serialization::content_hash(&doc.data);
                                let memory_node = MemoryNode {
                                    id: format!("doc_{}", content_hash),
                                    content: MemoryContent::new(&doc.data),
                                    content_hash,
                                    memory_type: MemoryTypeEnum::Semantic,
                                    created_at: Utc::now(),
                                    updated_at: Utc::now(),
                                    embedding: None,
                                    evaluation_status: crate::memory::monitoring::operations::OperationStatus::Pending,
                                    metadata: crate::memory::primitives::metadata::MemoryMetadata::new(),
                                    relevance_score: None,
                                };
                                if let Err(e) = mem.create_memory(memory_node).await {
                                    log::error!("Failed to ingest document: {:?}", e);
                                }
                            }
                        }

                        Some(mem)
                    } else {
                        // No embedding model available - memory features disabled
                        None
                    };

                    // Initialize tool router - ALWAYS create with default reasoner plugin
                    let tool_router = {

                            // Create default reasoner plugin configuration
                            let reasoner_schema =
                                crate::domain::agent::role::convert_serde_to_sweet_json(
                                    serde_json::json!({
                                        "type": "object",
                                        "properties": {
                                            "thought": {
                                                "type": "string",
                                                "description": "Current reasoning step"
                                            },
                                            "thoughtNumber": {
                                                "type": "integer",
                                                "description": "Current step number",
                                                "minimum": 1
                                            },
                                            "totalThoughts": {
                                                "type": "integer",
                                                "description": "Total expected steps",
                                                "minimum": 1
                                            },
                                            "nextThoughtNeeded": {
                                                "type": "boolean",
                                                "description": "Whether another step is needed"
                                            },
                                            "parentId": {
                                                "type": ["string", "null"],
                                                "description": "Optional parent thought ID for branching"
                                            },
                                            "strategyType": {
                                                "type": ["string", "null"],
                                                "enum": ["beam_search", "mcts", "mcts_002_alpha", "mcts_002alt_alpha", null],
                                                "description": "Reasoning strategy to use"
                                            },
                                            "beamWidth": {
                                                "type": ["integer", "null"],
                                                "description": "Number of top paths to maintain",
                                                "minimum": 1,
                                                "maximum": 10
                                            },
                                            "numSimulations": {
                                                "type": ["integer", "null"],
                                                "description": "Number of MCTS simulations",
                                                "minimum": 1,
                                                "maximum": 150
                                            }
                                        },
                                        "required": ["thought", "thoughtNumber", "totalThoughts", "nextThoughtNeeded"]
                                    }),
                                );

                            let default_plugin_config = crate::domain::tool::router::PluginConfig {
                                tool_name: "mcp-reasoner".to_string(),
                                wasm_path: "packages/sweetmcp/plugins/reasoner/target/wasm32-unknown-unknown/release/sweetmcp_plugin_reasoner.wasm".to_string(),
                                description: "Advanced reasoning tool with Beam Search and MCTS strategies".to_string(),
                                input_schema: reasoner_schema,
                            };

                            // Create router with default reasoner plugin
                            let plugin_configs = vec![default_plugin_config];
                            let mut router = SweetMcpRouter::with_configs(plugin_configs, None);

                            match router.initialize().await {
                                Ok(()) => Some(router),
                                Err(e) => {
                                    let error_chunk = CandleMessageChunk::Error(format!(
                                        "Tool initialization failed: {}",
                                        e
                                    ));
                                    let _ = sender.send(error_chunk);
                                    return;
                                }
                            }
                        };

                        // Memory context injection
                        let memory_context: Option<String> = if let Some(ref mem_manager) = memory {
                            use crate::memory::primitives::node::MemoryNode;
                            
                            let memory_stream = mem_manager.search_by_content(&user_message);
                            let timeout_duration = std::time::Duration::from_millis(memory_read_timeout);

                            // Use native async/await with timeout protection
                            let results: Vec<MemoryResult<MemoryNode>> = match tokio::time::timeout(timeout_duration, async {
                                use futures_util::StreamExt as FuturesStreamExt;
                                let limited = FuturesStreamExt::take(memory_stream, 5);
                                FuturesStreamExt::collect(limited).await
                            }).await {
                                Ok(results) => results,
                                Err(_) => {
                                    log::warn!("Memory search timed out after {}ms, proceeding with empty context", memory_read_timeout);
                                    Vec::new()
                                }
                            };

                            let memories: Vec<MemoryNode> =
                                results.into_iter().filter_map(|r| r.ok()).collect();

                            if !memories.is_empty() {
                                Some(format_memory_context(&memories, 1000))
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        // Format conversation history if provided
                        let history_context = match &conversation_history {
                            ZeroOneOrMany::None => String::new(),
                            ZeroOneOrMany::One((role, message)) => {
                                format!("\n# Conversation History:\n{}: {}\n", 
                                    match role {
                                        CandleMessageRole::User => "User",
                                        CandleMessageRole::Assistant => "Assistant",
                                        _ => "System",
                                    },
                                    message
                                )
                            }
                            ZeroOneOrMany::Many(messages) => {
                                let mut hist = String::from("\n# Conversation History:\n");
                                for (role, message) in messages {
                                    match role {
                                        CandleMessageRole::User => hist.push_str(&format!("User: {}\n", message)),
                                        CandleMessageRole::Assistant => hist.push_str(&format!("Assistant: {}\n", message)),
                                        _ => {}
                                    }
                                }
                                hist
                            }
                        };

                        // Create prompt with memory context, conversation history, and system prompt
                        let full_prompt = match (memory_context, !history_context.is_empty()) {
                            (Some(mem_ctx), true) => {
                                format!("{}\n\n{}{}\n\nUser: {}", &system_prompt, mem_ctx, history_context, user_message)
                            }
                            (Some(mem_ctx), false) => {
                                format!("{}\n\n{}\n\nUser: {}", &system_prompt, mem_ctx, user_message)
                            }
                            (None, true) => {
                                format!("{}{}\n\nUser: {}", &system_prompt, history_context, user_message)
                            }
                            (None, false) => {
                                format!("{}\n\nUser: {}", &system_prompt, user_message)
                            }
                        };

                        // Create CandlePrompt and CandleCompletionParams with tools if available
                        let prompt = CandlePrompt::new(full_prompt);
                        let mut params = CandleCompletionParams {
                            temperature,
                            max_tokens: NonZeroU64::new(max_tokens),
                            ..Default::default()
                        };

                        // Combine builder tools with auto-generated tools
                        if let Some(ref router) = tool_router {
                            let mut all_tools: Vec<ToolInfo> = tools.clone().into();

                            // Get auto-generated tools using native async/await
                            let auto_generated_tools = router.get_available_tools().await;
                            all_tools.extend(auto_generated_tools);

                            if !all_tools.is_empty() {
                                // Pass merged tools to completion system for function calling
                                params.tools = Some(ZeroOneOrMany::from(all_tools));
                            }
                        }

                        // Merge additional params if provided (zero allocation for empty map)
                        if !additional_params.is_empty() {
                            let params_map: serde_json::Map<String, serde_json::Value> = additional_params
                                .into_iter()
                                .map(|(k, v)| (k, serde_json::Value::String(v)))
                                .collect();
                            params = params.with_additional_params(Some(serde_json::Value::Object(params_map)));
                        }

                        // Call REAL provider inference
                        let completion_stream = provider.prompt(prompt, &params);
                        tokio::pin!(completion_stream);

                        // Convert CandleCompletionChunk to CandleMessageChunk and forward
                        // Handle tool calls if they occur
                        let mut assistant_response = String::new();

                        // Track metrics for performance visibility
                        let start_time = std::time::Instant::now();
                        let mut token_count = 0u32;

                        while let Some(completion_chunk) = completion_stream.next().await {
                            let message_chunk = match completion_chunk {
                                CandleCompletionChunk::Text(ref text) => {
                                    token_count += 1;
                                    assistant_response.push_str(text);
                                    CandleMessageChunk::Text(text.clone())
                                }
                                CandleCompletionChunk::Complete {
                                    ref text,
                                    finish_reason,
                                    usage,
                                } => {
                                    assistant_response.push_str(text);

                                    // Calculate performance metrics
                                    let elapsed = start_time.elapsed();
                                    let elapsed_secs = elapsed.as_secs_f64();
                                    let tokens_per_sec = if elapsed_secs > 0.0 {
                                        Some(token_count as f64 / elapsed_secs)
                                    } else {
                                        None
                                    };

                                    CandleMessageChunk::Complete {
                                        text: text.clone(),
                                        finish_reason: finish_reason.map(|f| format!("{:?}", f)),
                                        usage: usage.map(|u| format!("{:?}", u)),
                                        token_count: Some(token_count),
                                        elapsed_secs: Some(elapsed_secs),
                                        tokens_per_sec,
                                    }
                                }
                                CandleCompletionChunk::ToolCallStart { id, name } => {
                                    CandleMessageChunk::ToolCallStart { id, name }
                                }
                                CandleCompletionChunk::ToolCall {
                                    id,
                                    name,
                                    partial_input,
                                } => CandleMessageChunk::ToolCall {
                                    id,
                                    name,
                                    partial_input,
                                },
                                CandleCompletionChunk::ToolCallComplete { id, name, input } => {
                                    // Execute the tool if we have a router
                                    if let Some(ref router) = tool_router {
                                        // Convert input string to JsonValue
                                        match serde_json::from_str::<serde_json::Value>(&input) {
                                            Ok(args_json) => {
                                                // Convert to SweetMCP JsonValue
                                                let sweet_args = crate::domain::agent::role::convert_serde_to_sweet_json(args_json);

                                                // Execute the tool using native async/await
                                                match router.call_tool(&name, sweet_args).await {
                                                    Ok(response) => {
                                                        // Call tool result handler if configured (zero allocation for None)
                                                        if let Some(ref handler) = on_tool_result_handler {
                                                            let results = vec![format!("{:?}", response)];
                                                            handler(&results).await;
                                                        }

                                                        // Convert response to text result
                                                        let result_text = format!(
                                                            "Tool '{}' executed successfully: {:?}",
                                                            name, response
                                                        );
                                                        CandleMessageChunk::Text(
                                                            result_text,
                                                        )
                                                    }
                                                    Err(e) => {
                                                        // Return error as text
                                                        CandleMessageChunk::Error(format!(
                                                            "Tool '{}' execution failed: {}",
                                                            name, e
                                                        ))
                                                    }
                                                }
                                            }
                                            Err(e) => CandleMessageChunk::Error(format!(
                                                "Tool '{}' invalid JSON input: {}",
                                                name, e
                                            )),
                                        }
                                    } else {
                                        CandleMessageChunk::ToolCallComplete { id, name, input }
                                    }
                                }
                                CandleCompletionChunk::Error(error) => {
                                    CandleMessageChunk::Error(error)
                                }
                            };

                            // Apply chunk handler if configured (zero allocation for None)
                            let final_chunk = if let Some(ref handler) = on_chunk_handler {
                                handler(message_chunk).await
                            } else {
                                message_chunk
                            };
                            let _ = sender.send(final_chunk);
                        }

                        // Store conversation turn in memory after completion
                        if let Some(ref memory_manager) = memory
                            && !assistant_response.is_empty()
                        {
                            // Create memory nodes for the conversation
                            let user_content =
                                crate::memory::core::primitives::types::MemoryContent::new(
                                    &user_message,
                                );
                            let assistant_content =
                                crate::memory::core::primitives::types::MemoryContent::new(
                                    &assistant_response,
                                );

                            // Use Episodic memory type for conversation turns
                            let mut user_memory = crate::memory::core::MemoryNode::new(
                                crate::memory::core::primitives::types::MemoryTypeEnum::Episodic,
                                user_content,
                            );
                            user_memory.metadata.tags.push("user_message".to_string());
                            user_memory.metadata.source = Some("chat".to_string());
                            user_memory.metadata.importance = 0.8;

                            let mut assistant_memory = crate::memory::core::MemoryNode::new(
                                crate::memory::core::primitives::types::MemoryTypeEnum::Episodic,
                                assistant_content,
                            );
                            assistant_memory
                                .metadata
                                .tags
                                .push("assistant_response".to_string());
                            assistant_memory.metadata.source = Some("chat".to_string());
                            assistant_memory.metadata.importance = 0.8;

                            // Merge builder metadata into memory metadata (zero allocation for empty map)
                            for (key, value) in &metadata {
                                if let Err(e) = user_memory.metadata.set_custom(key, value) {
                                    log::warn!("Failed to set custom metadata '{}' on user memory: {:?}", key, e);
                                }
                                if let Err(e) = assistant_memory.metadata.set_custom(key, value) {
                                    log::warn!("Failed to set custom metadata '{}' on assistant memory: {:?}", key, e);
                                }
                            }

                            // Create PendingMemory operations
                            let user_pending = memory_manager.create_memory(user_memory);
                            let assistant_pending = memory_manager.create_memory(assistant_memory);

                            // Spawn tasks to handle the async operations
                            tokio::spawn(async move {
                                if let Err(e) = user_pending.await {
                                    log::error!(
                                        "Failed to store user memory to database: {:?}",
                                        e
                                    );
                                }
                            });
                            tokio::spawn(async move {
                                if let Err(e) = assistant_pending.await {
                                    log::error!(
                                        "Failed to store assistant memory to database: {:?}",
                                            e
                                        );
                                    }
                                });
                        }

                        // Invoke on_conversation_turn handler if configured
                        if let Some(handler) = on_conversation_turn_handler {
                            // Create conversation with current messages
                            let mut conversation = CandleAgentConversation::new();
                            conversation.add_message(user_message.clone(), CandleMessageRole::User);
                            conversation.add_message(
                                assistant_response.clone(),
                                CandleMessageRole::Assistant,
                            );

                            // Create builder state for recursive agent with handler included
                            // handler is already Arc<dyn Fn(...)>, so just clone it
                            let builder_state = Arc::new(AgentBuilderState {
                                name: String::from("agent"),
                                text_to_text_model: provider.clone(),
                                text_embedding_model: embedding_model.clone(),
                                temperature,
                                max_tokens,
                                memory_read_timeout,
                                system_prompt: system_prompt.clone(),
                                tools: tools.clone(),
                                context_file: None,
                                context_files: None,
                                context_directory: None,
                                context_github: None,
                                additional_params: std::collections::HashMap::new(),
                                metadata: std::collections::HashMap::new(),
                                on_chunk_handler: None,
                                on_tool_result_handler: None,
                                on_conversation_turn_handler: Some(handler.clone()),
                            });

                            // Create agent with full state
                            let agent = CandleAgentRoleAgent {
                                state: builder_state,
                            };

                            // Call handler and await the future to get the stream
                            let handler_stream = handler(&conversation, &agent).await;
                            tokio::pin!(handler_stream);
                            while let Some(chunk) = handler_stream.next().await {
                                let _ = sender.send(chunk);
                            }
                        }
                }
            }
        })))
    }

    fn chat_with_message(self, message: impl Into<String>) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>> {
        let provider = self.text_to_text_model;
        let temperature = self.temperature;
        let max_tokens = self.max_tokens;
        let system_prompt = self.system_prompt.clone();
        let on_chunk_handler = self.on_chunk_handler;
        let user_message = message.into();

        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            let full_prompt = format!("{}\n\nUser: {}", system_prompt, user_message);

            let prompt = CandlePrompt::new(full_prompt);
            let params = CandleCompletionParams {
                temperature,
                max_tokens: NonZeroU64::new(max_tokens),
                ..Default::default()
            };

            let completion_stream = provider.prompt(prompt, &params);
            tokio::pin!(completion_stream);

            // Track metrics for performance visibility
            let start_time = std::time::Instant::now();
            let mut token_count = 0u32;

            while let Some(completion_chunk) = completion_stream.next().await {
                let message_chunk = match completion_chunk {
                    CandleCompletionChunk::Text(text) => {
                        token_count += 1;
                        CandleMessageChunk::Text(text)
                    }
                    CandleCompletionChunk::Complete {
                        text,
                        finish_reason,
                        usage,
                    } => {
                        // Calculate performance metrics
                        let elapsed = start_time.elapsed();
                        let elapsed_secs = elapsed.as_secs_f64();
                        let tokens_per_sec = if elapsed_secs > 0.0 {
                            Some(token_count as f64 / elapsed_secs)
                        } else {
                            None
                        };

                        CandleMessageChunk::Complete {
                            text,
                            finish_reason: finish_reason.map(|f| format!("{:?}", f)),
                            usage: usage.map(|u| format!("{:?}", u)),
                            token_count: Some(token_count),
                            elapsed_secs: Some(elapsed_secs),
                            tokens_per_sec,
                        }
                    }
                    CandleCompletionChunk::ToolCallStart { id, name } => {
                        CandleMessageChunk::ToolCallStart { id, name }
                    }
                    CandleCompletionChunk::ToolCall {
                        id,
                        name,
                        partial_input,
                    } => CandleMessageChunk::ToolCall {
                        id,
                        name,
                        partial_input,
                    },
                    CandleCompletionChunk::ToolCallComplete { id, name, input } => {
                        CandleMessageChunk::ToolCallComplete { id, name, input }
                    }
                    CandleCompletionChunk::Error(error) => CandleMessageChunk::Error(error),
                };

                // Apply chunk handler if configured (zero allocation for None)
                let final_chunk = if let Some(ref handler) = on_chunk_handler {
                    handler(message_chunk).await
                } else {
                    message_chunk
                };
                let _ = sender.send(final_chunk);
            }
        }))
    }
}
