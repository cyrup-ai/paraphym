//! Helper types and utility functions

use super::*;

pub struct CandleAgentRoleAgent {
    pub(super) state: Arc<AgentBuilderState>,
}

impl CandleAgentRoleAgent {
    /// Chat method for use in on_conversation_turn closure - enables recursion
    pub fn chat(&self, chat_loop: CandleChatLoop) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>> {
        match chat_loop {
            CandleChatLoop::Break => Box::pin(crate::async_stream::spawn_stream(|sender| async move {
                let final_chunk = CandleMessageChunk::Complete {
                    text: String::new(),
                    finish_reason: Some("break".to_string()),
                    usage: None,
                    token_count: None,
                    elapsed_secs: None,
                    tokens_per_sec: None,
                };
                let _ = sender.send(final_chunk);
            })),
            CandleChatLoop::UserPrompt(user_message) | CandleChatLoop::Reprompt(user_message) => {
                self.run_inference_cycle(user_message)
            }
        }
    }

    fn run_inference_cycle(&self, user_message: String) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>> {
        let state = self.state.clone();

        Box::pin(crate::async_stream::spawn_stream(move |stream_sender| async move {
                    // Extract handlers from state for recursive inference
                    let on_chunk_handler = state.on_chunk_handler.clone();
                    let on_tool_result_handler = state.on_tool_result_handler.clone();

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
                        let _ = stream_sender.send(error_chunk);
                        return;
                    }
                }
            };

            // Build prompt - system_prompt always exists (no memory in recursive calls)
            let full_prompt = format!("{}\n\nUser: {}", &state.system_prompt, user_message);

            // Call provider
            let prompt = CandlePrompt::new(full_prompt);
            let mut params = crate::domain::completion::CandleCompletionParams {
                temperature: state.temperature,
                max_tokens: std::num::NonZeroU64::new(state.max_tokens),
                ..Default::default()
            };

            // Add tools
            if let Some(ref router) = tool_router {
                let mut all_tools: Vec<sweet_mcp_type::ToolInfo> = state.tools.clone().into();

                // Use .await instead of block_on
                let auto_generated_tools = router.get_available_tools().await;
                all_tools.extend(auto_generated_tools);

                if !all_tools.is_empty() {
                    params.tools = Some(ZeroOneOrMany::from(all_tools));
                }
            }

            let completion_stream = state.text_to_text_model.prompt(prompt, &params);
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
                                    // Use .await instead of block_on
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

                // Apply chunk handler if configured (zero allocation for None)
                let final_chunk = if let Some(ref handler) = on_chunk_handler {
                    handler(message_chunk).await
                } else {
                    message_chunk
                };
                let _ = stream_sender.send(final_chunk);
            }

            // CRITICAL: Call handler for recursion
            if let Some(ref handler) = state.on_conversation_turn_handler {
                let mut conversation = CandleAgentConversation::new();
                conversation.add_message(user_message.clone(), CandleMessageRole::User);
                conversation
                    .add_message(assistant_response.clone(), CandleMessageRole::Assistant);

                let agent = CandleAgentRoleAgent {
                    state: state.clone(),
                };
                // Await the async handler to get the stream
                let handler_stream = handler(&conversation, &agent).await;
                tokio::pin!(handler_stream);
                while let Some(chunk) = handler_stream.next().await {
                    let _ = stream_sender.send(chunk);
                }
            }
        }))
    }
}

/// Agent role builder trait - elegant zero-allocation builder pattern (PUBLIC API)

pub(crate) fn format_memory_context(
    memories: &[crate::domain::memory::primitives::node::MemoryNode],
    max_tokens: usize,
) -> String {
    use std::fmt::Write;

    let mut context = String::from("# Relevant Context from Memory:\n\n");
    let mut token_count = 0usize;

    for memory in memories {
        // Approximate token count: chars / 4
        let content_text = memory.content().to_string();
        let memory_tokens = content_text.chars().count() / 4;

        if token_count + memory_tokens > max_tokens {
            break; // Exceed budget, stop adding
        }

        // Format with relevance indicator
        let relevance = memory.importance();
        let _ = writeln!(
            &mut context,
            "[Relevance: {:.2}] {}\n",
            relevance, content_text
        );

        token_count += memory_tokens;
    }

    context
}


pub trait ConversationHistoryArgs {
    /// Convert this into conversation history format
    fn into_history(self) -> ZeroOneOrMany<(CandleMessageRole, String)>;
}

impl ConversationHistoryArgs for (CandleMessageRole, &str) {
    fn into_history(self) -> ZeroOneOrMany<(CandleMessageRole, String)> {
        ZeroOneOrMany::one((self.0, self.1.to_string()))
    }
}

impl ConversationHistoryArgs for (CandleMessageRole, String) {
    fn into_history(self) -> ZeroOneOrMany<(CandleMessageRole, String)> {
        ZeroOneOrMany::one(self)
    }
}

impl<T1, T2> ConversationHistoryArgs for (T1, T2)
where
    T1: ConversationHistoryArgs,
    T2: ConversationHistoryArgs,
{
    fn into_history(self) -> ZeroOneOrMany<(CandleMessageRole, String)> {
        let h1 = self.0.into_history();
        let h2 = self.1.into_history();
        
        match (h1, h2) {
            (ZeroOneOrMany::None, h) | (h, ZeroOneOrMany::None) => h,
            (ZeroOneOrMany::One(m1), ZeroOneOrMany::One(m2)) => {
                ZeroOneOrMany::Many(vec![m1, m2])
            }
            (ZeroOneOrMany::One(m), ZeroOneOrMany::Many(mut msgs)) => {
                msgs.insert(0, m);
                ZeroOneOrMany::Many(msgs)
            }
            (ZeroOneOrMany::Many(mut msgs), ZeroOneOrMany::One(m)) => {
                msgs.push(m);
                ZeroOneOrMany::Many(msgs)
            }
            (ZeroOneOrMany::Many(mut msgs1), ZeroOneOrMany::Many(msgs2)) => {
                msgs1.extend(msgs2);
                ZeroOneOrMany::Many(msgs1)
            }
        }
    }
}

impl<T1, T2, T3> ConversationHistoryArgs for (T1, T2, T3)
where
    T1: ConversationHistoryArgs,
    T2: ConversationHistoryArgs,
    T3: ConversationHistoryArgs,
{
    fn into_history(self) -> ZeroOneOrMany<(CandleMessageRole, String)> {
        let h1 = self.0.into_history();
        let h2 = self.1.into_history();
        let h3 = self.2.into_history();
        
        // Merge all three by first merging h1 and h2, then merging result with h3
        let combined_12 = match (h1, h2) {
            (ZeroOneOrMany::None, h) | (h, ZeroOneOrMany::None) => h,
            (ZeroOneOrMany::One(m1), ZeroOneOrMany::One(m2)) => {
                ZeroOneOrMany::Many(vec![m1, m2])
            }
            (ZeroOneOrMany::One(m), ZeroOneOrMany::Many(mut msgs)) => {
                msgs.insert(0, m);
                ZeroOneOrMany::Many(msgs)
            }
            (ZeroOneOrMany::Many(mut msgs), ZeroOneOrMany::One(m)) => {
                msgs.push(m);
                ZeroOneOrMany::Many(msgs)
            }
            (ZeroOneOrMany::Many(mut msgs1), ZeroOneOrMany::Many(msgs2)) => {
                msgs1.extend(msgs2);
                ZeroOneOrMany::Many(msgs1)
            }
        };
        
        // Now merge combined_12 with h3
        match (combined_12, h3) {
            (ZeroOneOrMany::None, h) | (h, ZeroOneOrMany::None) => h,
            (ZeroOneOrMany::One(m1), ZeroOneOrMany::One(m2)) => {
                ZeroOneOrMany::Many(vec![m1, m2])
            }
            (ZeroOneOrMany::One(m), ZeroOneOrMany::Many(mut msgs)) => {
                msgs.insert(0, m);
                ZeroOneOrMany::Many(msgs)
            }
            (ZeroOneOrMany::Many(mut msgs), ZeroOneOrMany::One(m)) => {
                msgs.push(m);
                ZeroOneOrMany::Many(msgs)
            }
            (ZeroOneOrMany::Many(mut msgs1), ZeroOneOrMany::Many(msgs2)) => {
                msgs1.extend(msgs2);
                ZeroOneOrMany::Many(msgs1)
            }
        }
    }
}

/// CandleFluentAi entry point for creating agent roles
pub struct CandleFluentAi;

impl CandleFluentAi {
    /// Create a new agent role builder - main entry point
    pub fn agent_role(name: impl Into<String>) -> impl CandleAgentRoleBuilder {
        CandleAgentRoleBuilderImpl::new(name)
    }
}
