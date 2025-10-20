//! Main chat orchestration with memory and tool support

use super::super::*;
use super::memory_ops::*;

pub(super) fn execute_chat<F, Fut>(
    builder: CandleAgentBuilderImpl,
    handler: F,
) -> Result<Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>, AgentError>
where
    F: FnOnce(&CandleAgentConversation) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = CandleChatLoop> + Send + 'static,
{
    let provider = builder.text_to_text_model;
    let embedding_model = builder.text_embedding_model;
    let temperature = builder.temperature;
    let max_tokens = builder.max_tokens;
    let memory_read_timeout = builder.memory_read_timeout;
    let system_prompt = builder.system_prompt.clone();
    let tools = builder.tools;
    let on_conversation_turn_handler = builder.on_conversation_turn_handler;
    let on_chunk_handler = builder.on_chunk_handler;
    let on_tool_result_handler = builder.on_tool_result_handler;
    let context_file = builder.context_file;
    let context_files = builder.context_files;
    let context_directory = builder.context_directory;
    let context_github = builder.context_github;
    let additional_params = builder.additional_params;
    let metadata = builder.metadata;
    let conversation_history = builder.conversation_history;

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
                let memory: Option<Arc<dyn crate::memory::core::manager::surreal::MemoryManager>> = if let Some(ref emb_model) = embedding_model {
                    match initialize_memory_manager(emb_model).await {
                        Ok(mem) => {
                            // Load context into memory
                            if let Err(e) = load_context_into_memory(
                                &mem,
                                context_file,
                                context_files,
                                context_directory,
                                context_github,
                            ).await {
                                let _ = sender.send(CandleMessageChunk::Error(e));
                                return;
                            }
                            Some(mem)
                        }
                        Err(e) => {
                            let _ = sender.send(CandleMessageChunk::Error(e));
                            return;
                        }
                    }
                } else {
                    None
                };

                // Initialize tool router
                let tool_router = {
                    let reasoner_schema = crate::domain::agent::role::convert_serde_to_sweet_json(
                        serde_json::json!({
                            "type": "object",
                            "properties": {
                                "thought": {"type": "string", "description": "Current reasoning step"},
                                "thoughtNumber": {"type": "integer", "description": "Current step number", "minimum": 1},
                                "totalThoughts": {"type": "integer", "description": "Total expected steps", "minimum": 1},
                                "nextThoughtNeeded": {"type": "boolean", "description": "Whether another step is needed"}
                            },
                            "required": ["thought", "thoughtNumber", "totalThoughts", "nextThoughtNeeded"]
                        }),
                    );

                    let default_plugin_config = crate::domain::tool::router::PluginConfig {
                        tool_name: "mcp-reasoner".to_string(),
                        wasm_path: "packages/sweetmcp/plugins/reasoner/target/wasm32-unknown-unknown/release/sweetmcp_plugin_reasoner.wasm".to_string(),
                        description: "Advanced reasoning tool".to_string(),
                        input_schema: reasoner_schema,
                    };

                    let plugin_configs = vec![default_plugin_config];
                    let mut router = crate::domain::tool::router::SweetMcpRouter::with_configs(
                        plugin_configs,
                        None,
                    );

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

                // Search memory for relevant context
                let memory_context = if let Some(ref memory_manager) = memory {
                    match search_memory_with_timeout(memory_manager, &user_message, memory_read_timeout).await {
                        Some(memories) => {
                            if memories.is_empty() {
                                String::new()
                            } else {
                                format_memory_context(&memories, 2000)
                            }
                        }
                        None => String::new(),
                    }
                } else {
                    String::new()
                };

                // Build prompt with memory context
                let full_prompt = if memory_context.is_empty() {
                    format!("{}\n\nUser: {}", system_prompt, user_message)
                } else {
                    format!("{}\n\n{}\n\nUser: {}", system_prompt, memory_context, user_message)
                };

                // Call provider
                let prompt = CandlePrompt::new(full_prompt);
                let mut params = crate::domain::completion::CandleCompletionParams {
                    temperature,
                    max_tokens: std::num::NonZeroU64::new(max_tokens),
                    ..Default::default()
                };

                // Add tools
                if let Some(ref router) = tool_router {
                    let mut all_tools: Vec<sweet_mcp_type::ToolInfo> = tools.into();
                    let auto_generated_tools = router.get_available_tools().await;
                    all_tools.extend(auto_generated_tools);

                    if !all_tools.is_empty() {
                        params.tools = Some(ZeroOneOrMany::from(all_tools));
                    }
                }

                let completion_stream = provider.prompt(prompt, &params);
                tokio::pin!(completion_stream);
                let mut assistant_response = String::new();

                // Track metrics for performance visibility
                let start_time = std::time::Instant::now();
                let mut token_count = 0u32;

                // Stream chunks
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
                            if let Some(ref router) = tool_router {
                                match serde_json::from_str::<serde_json::Value>(&input) {
                                    Ok(args_json) => {
                                        let sweet_args =
                                            crate::domain::agent::role::convert_serde_to_sweet_json(
                                                args_json,
                                            );
                                        match router.call_tool(&name, sweet_args).await {
                                            Ok(response) => {
                                                // Call tool result handler if configured
                                                if let Some(ref handler) = on_tool_result_handler {
                                                    let results = vec![format!("{:?}", response)];
                                                    handler(&results).await;
                                                }

                                                CandleMessageChunk::Text(format!(
                                                    "Tool '{}' executed: {:?}",
                                                    name, response
                                                ))
                                            }
                                            Err(e) => CandleMessageChunk::Error(format!(
                                                "Tool '{}' failed: {}",
                                                name, e
                                            )),
                                        }
                                    }
                                    Err(e) => {
                                        CandleMessageChunk::Error(format!("Invalid JSON: {}", e))
                                    }
                                }
                            } else {
                                CandleMessageChunk::ToolCallComplete { id, name, input }
                            }
                        }
                        CandleCompletionChunk::Error(error) => CandleMessageChunk::Error(error),
                    };

                    // Apply chunk handler if configured
                    let final_chunk = if let Some(ref handler) = on_chunk_handler {
                        handler(message_chunk).await
                    } else {
                        message_chunk
                    };
                    let _ = sender.send(final_chunk);
                }

                // Store conversation turn in memory after completion
                if let Some(ref memory_manager) = memory {
                    if !assistant_response.is_empty() {
                        let user_memory = create_user_memory(&user_message, &metadata);
                        let assistant_memory = create_assistant_memory(&assistant_response, &metadata);

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
