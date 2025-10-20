//! Chat session orchestration executor

use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use futures::future::BoxFuture;
use tokio_stream::{Stream, StreamExt};

// Memory helper functions (copied from builders since they're not publicly exported)

// Import domain types
use crate::domain::chat::{
    config::{CandleModelConfig, CandleChatConfig},
    message::{CandleMessageChunk, CandleMessageRole},
    r#loop::CandleChatLoop,
};
use crate::domain::agent::role::CandleAgentConversation;
use crate::domain::agent::{
    role::convert_serde_to_sweet_json,
};
use crate::builders::agent_role::CandleAgentRoleAgent;
use crate::domain::completion::{CandleCompletionParams, CandleCompletionChunk};
use crate::domain::prompt::CandlePrompt;
use crate::domain::tool::router::{SweetMcpRouter, PluginConfig};

use crate::builders::agent_role::AgentBuilderState;
use crate::capability::registry::TextToTextModel;
use crate::capability::traits::TextToTextCapable;
use crate::memory::core::manager::coordinator::MemoryCoordinator;
use crate::domain::memory::primitives::node::MemoryNode;
use crate::domain::memory::primitives::types::MemoryTypeEnum;
use crate::memory::MemoryMetadata;

use sweet_mcp_type::ToolInfo;
use cyrup_sugars::collections::ZeroOneOrMany;

/// Simple document structure for session context
#[derive(Debug, Clone)]
pub struct SessionDocument {
    pub content: String,
    pub source: String,
    pub tags: Vec<String>,
}

// Helper functions for memory operations

fn format_memory_context(memories: &[MemoryNode], max_chars: usize) -> String {
    let mut result = String::from("## Relevant Context\n\n");
    let mut current_len = result.len();

    for memory in memories {
        let content = memory.content().to_string();
        let source = memory.metadata.custom.get("source")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let entry = format!("- [{}]: {}\n", source, content);

        if current_len + entry.len() > max_chars {
            break;
        }

        result.push_str(&entry);
        current_len += entry.len();
    }

    result
}

pub async fn execute_chat_session<F, Fut>(
    // Configuration
    model_config: CandleModelConfig,
    chat_config: CandleChatConfig,
    
    // Core components
    provider: TextToTextModel,
    memory: Arc<MemoryCoordinator>,
    tools: Arc<[ToolInfo]>,
    metadata: HashMap<String, String>,
    
    // Context (these replace context_file, context_files, etc.)
    context_documents: Vec<SessionDocument>,
    
    // Conversation history  
    conversation_history: ZeroOneOrMany<(CandleMessageRole, String)>,
    
    // Handler
    handler: F,
    
    // Optional callbacks
    on_chunk_handler: Option<Arc<dyn Fn(CandleMessageChunk) -> BoxFuture<'static, CandleMessageChunk> + Send + Sync>>,
    on_tool_result_handler: Option<Arc<dyn Fn(&[String]) -> BoxFuture<'static, ()> + Send + Sync>>,
    on_conversation_turn_handler: Option<Arc<dyn Fn(&CandleAgentConversation, &CandleAgentRoleAgent) -> BoxFuture<'static, Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>> + Send + Sync>>,
) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>>
where
    F: FnOnce(&CandleAgentConversation) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = CandleChatLoop> + Send + 'static,
{
    Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
        // Load context documents into memory before handler execution
        for doc in context_documents {
            let doc_meta = MemoryMetadata {
                user_id: metadata.get("user_id").cloned(),
                agent_id: metadata.get("agent_id").cloned(),
                context: "session_context".to_string(),
                importance: 0.5,
                keywords: vec![],
                tags: doc.tags.clone(),
                category: "context".to_string(),
                source: Some(doc.source.clone()),
                created_at: chrono::Utc::now(),
                last_accessed_at: None,
                embedding: None,
                custom: serde_json::Value::Object(serde_json::Map::new()),
            };

            if let Err(e) = memory.add_memory(
                doc.content,
                MemoryTypeEnum::Semantic,
                Some(doc_meta)
            ).await {
                log::warn!("Failed to load context document: {:?}", e);
            }
        }

        // Create conversation and ALWAYS populate with history (history is not optional)
        let mut initial_conversation = CandleAgentConversation::new();

        // Convert ZeroOneOrMany to vec for iteration
        let history_vec: Vec<(CandleMessageRole, String)> = match conversation_history {
            ZeroOneOrMany::None => vec![],
            ZeroOneOrMany::One(item) => vec![item],
            ZeroOneOrMany::Many(items) => items,
        };

        for (role, message) in history_vec {
            initial_conversation.add_message(message, role);
        }

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
                // Validate message length using chat_config
                if user_message.len() > chat_config.max_message_length {
                    let error_chunk = CandleMessageChunk::Error(format!(
                        "Message too long: {} characters (max: {})",
                        user_message.len(),
                        chat_config.max_message_length
                    ));
                    let _ = sender.send(error_chunk);
                    return;
                }

                // Initialize tool router
                let tool_router = {
                    let reasoner_schema = convert_serde_to_sweet_json(
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

                    let default_plugin_config = PluginConfig {
                        tool_name: "mcp-reasoner".to_string(),
                        wasm_path: "packages/sweetmcp/plugins/reasoner/target/wasm32-unknown-unknown/release/sweetmcp_plugin_reasoner.wasm".to_string(),
                        description: "Advanced reasoning tool".to_string(),
                        input_schema: reasoner_schema,
                    };

                    let plugin_configs = vec![default_plugin_config];
                    let mut router = SweetMcpRouter::with_configs(
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
                let memory_context = match memory.search_memories(&user_message, 10, None).await {
                    Ok(memories) => {
                        if memories.is_empty() {
                            String::new()
                        } else {
                            format_memory_context(&memories, 2000)
                        }
                    }
                    Err(e) => {
                        log::warn!("Memory search failed: {:?}", e);
                        String::new()
                    }
                };

                // Build prompt with personality and memory context
                let mut system_prompt = model_config.system_prompt.clone().unwrap_or_default();

                // Apply personality configuration from chat_config
                if let Some(custom) = &chat_config.personality.custom_instructions {
                    system_prompt.push_str("\n\n");
                    system_prompt.push_str(custom);
                }
                system_prompt.push_str(&format!(
                    "\n\nPersonality: {} (creativity: {:.1}, formality: {:.1}, empathy: {:.1})",
                    chat_config.personality.personality_type,
                    chat_config.personality.creativity,
                    chat_config.personality.formality,
                    chat_config.personality.empathy
                ));

                let full_prompt = if memory_context.is_empty() {
                    format!("{}\n\nUser: {}", system_prompt, user_message)
                } else {
                    format!("{}\n\n{}\n\nUser: {}", system_prompt, memory_context, user_message)
                };

                // Call provider
                let prompt = CandlePrompt::new(full_prompt);
                let mut params = CandleCompletionParams {
                    temperature: model_config.temperature as f64,
                    max_tokens: model_config.max_tokens.and_then(|t| std::num::NonZeroU64::new(t as u64)),
                    ..Default::default()
                };

                // Add tools
                if let Some(ref router) = tool_router {
                    let mut all_tools: Vec<ToolInfo> = tools.to_vec();
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
                                        let sweet_args = convert_serde_to_sweet_json(args_json);
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

                    // Apply response delay from behavior config
                    if !chat_config.behavior.response_delay.is_zero() {
                        tokio::time::sleep(chat_config.behavior.response_delay).await;
                    }

                    // Apply chunk handler if configured
                    let final_chunk = if let Some(ref handler) = on_chunk_handler {
                        handler(message_chunk).await
                    } else {
                        message_chunk
                    };
                    let _ = sender.send(final_chunk);
                }

                // Store conversation turn in memory after completion
                if !assistant_response.is_empty() {
                    // Create metadata for memories
                    let user_meta = MemoryMetadata {
                        user_id: metadata.get("user_id").cloned(),
                        agent_id: metadata.get("agent_id").cloned(),
                        context: "chat".to_string(),
                        importance: 0.8,
                        keywords: vec![],
                        tags: vec!["user_message".to_string()],
                        category: "conversation".to_string(),
                        source: Some("chat".to_string()),
                        created_at: chrono::Utc::now(),
                        last_accessed_at: None,
                        embedding: None,
                        custom: serde_json::Value::Object(serde_json::Map::new()),
                    };

                    let assistant_meta = MemoryMetadata {
                        tags: vec!["assistant_response".to_string()],
                        ..user_meta.clone()
                    };

                    // Use MemoryCoordinator.add_memory()
                    let memory_clone = memory.clone();
                    let user_msg = user_message.clone();
                    tokio::spawn(async move {
                        if let Err(e) = memory_clone.add_memory(
                            user_msg,
                            MemoryTypeEnum::Episodic,
                            Some(user_meta)
                        ).await {
                            log::error!("Failed to store user memory: {:?}", e);
                        }
                    });

                    let memory_clone = memory.clone();
                    let assistant_msg = assistant_response.clone();
                    tokio::spawn(async move {
                        if let Err(e) = memory_clone.add_memory(
                            assistant_msg,
                            MemoryTypeEnum::Episodic,
                            Some(assistant_meta)
                        ).await {
                            log::error!("Failed to store assistant memory: {:?}", e);
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
                    let builder_state = Arc::new(AgentBuilderState {
                        name: String::from("agent"),
                        text_to_text_model: provider.clone(),
                        text_embedding_model: None,  // No longer available here
                        temperature: model_config.temperature as f64,
                        max_tokens: model_config.max_tokens.unwrap_or(4096) as u64,
                        memory_read_timeout: model_config.timeout_ms,
                        system_prompt: model_config.system_prompt.clone().unwrap_or_default(),
                        tools: tools.to_vec().into(),
                        context_file: None,
                        context_files: None,
                        context_directory: None,
                        context_github: None,
                        additional_params: HashMap::new(),
                        metadata: HashMap::new(),
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
    }))
}
