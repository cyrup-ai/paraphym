# Fix Chat Loop - Recursive Handler Implementation

## QA Rating: 2/10 - Core Implementation Missing

**Status:** Infrastructure exists (CandleChatLoop, Document builder, CLI modules) but recursive loop mechanism is NOT implemented.

**Critical Issues:**
1. ❌ CandleAgentRoleAgent is empty stub: `pub struct CandleAgentRoleAgent;` [agent_role.rs:119](../packages/candle/src/builders/agent_role.rs#L119)
2. ❌ agent.chat() just echoes messages, doesn't run inference [agent_role.rs:123-140](../packages/candle/src/builders/agent_role.rs#L123-L140)
3. ❌ No Arc<BuilderState> pattern implemented
4. ❌ CLI runner uses external loop instead of recursive handler [runner.rs:267-373](../packages/candle/src/cli/runner.rs#L267-L373)
5. ❌ on_conversation_turn never configured in runner

---

## Required Implementation

### Task 1: Create AgentBuilderState Struct

**File:** [src/builders/agent_role.rs](../packages/candle/src/builders/agent_role.rs)  
**Location:** After line 119

```rust
struct AgentBuilderState<P> 
where 
    P: DomainCompletionModel + Send + Clone + 'static 
{
    name: String,
    provider: P,
    temperature: f64,
    max_tokens: Option<u64>,
    memory_read_timeout: Option<u64>,
    system_prompt: Option<String>,
    tools: ZeroOneOrMany<ToolInfo>,
    mcp_servers: Vec<McpServerConfig>,
    memory: Option<Arc<dyn crate::memory::core::manager::surreal::MemoryManager>>,
    on_conversation_turn_handler: Option<Arc<dyn Fn(&CandleAgentConversation, &CandleAgentRoleAgent<P>) -> AsyncStream<CandleMessageChunk> + Send + Sync>>,
}

impl<P> Clone for AgentBuilderState<P> 
where 
    P: DomainCompletionModel + Send + Clone + 'static 
{
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            provider: self.provider.clone(),
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            memory_read_timeout: self.memory_read_timeout,
            system_prompt: self.system_prompt.clone(),
            tools: self.tools.clone(),
            mcp_servers: self.mcp_servers.clone(),
            memory: self.memory.clone(),
            on_conversation_turn_handler: self.on_conversation_turn_handler.clone(),
        }
    }
}
```

**Issue:** Provider must implement Clone. Verify or add Clone bound to provider traits.

### Task 2: Redesign CandleAgentRoleAgent

**File:** [src/builders/agent_role.rs](../packages/candle/src/builders/agent_role.rs)  
**Location:** Lines 119-140 (replace entire implementation)

```rust
pub struct CandleAgentRoleAgent<P> 
where 
    P: DomainCompletionModel + Send + Clone + 'static 
{
    state: Arc<AgentBuilderState<P>>,
}

impl<P> Clone for CandleAgentRoleAgent<P> 
where 
    P: DomainCompletionModel + Send + Clone + 'static 
{
    fn clone(&self) -> Self {
        Self { state: self.state.clone() }
    }
}
```

### Task 3: Implement agent.chat() with Real Inference

**File:** [src/builders/agent_role.rs](../packages/candle/src/builders/agent_role.rs)  
**Location:** Lines 123-140 (replace stub implementation)

```rust
impl<P> CandleAgentRoleAgent<P> 
where 
    P: DomainCompletionModel + Send + Clone + 'static 
{
    pub fn chat(&self, chat_loop: CandleChatLoop) -> AsyncStream<CandleMessageChunk> {
        match chat_loop {
            CandleChatLoop::Break => {
                AsyncStream::with_channel(|sender| {
                    let final_chunk = CandleMessageChunk::Complete {
                        text: String::new(),
                        finish_reason: Some("break".to_string()),
                        usage: None,
                    };
                    let _ = sender.send(final_chunk);
                })
            }
            CandleChatLoop::UserPrompt(message) | CandleChatLoop::Reprompt(message) => {
                self.run_inference_cycle(message)
            }
        }
    }
    
    fn run_inference_cycle(&self, user_message: String) -> AsyncStream<CandleMessageChunk> {
        let state = self.state.clone();
        
        AsyncStream::with_channel(move |sender| {
            let _background_stream = ystream::spawn_stream(move |stream_sender| {
                // EXTRACT full inference logic from CandleAgentBuilderImpl::chat()
                // Lines ~900-1100 in agent_role.rs
                // Include:
                // 1. Memory search (if memory exists)
                // 2. Tool router initialization
                // 3. Prompt construction with system_prompt + memory context
                // 4. Provider inference call
                // 5. Chunk streaming
                // 6. Tool execution
                // 7. Memory storage
                // 8. on_conversation_turn handler invocation (enables recursion!)
                
                // Stub for now - implement full pipeline
                let full_prompt = match &state.system_prompt {
                    Some(sys) => format!("{}\n\nUser: {}", sys, user_message),
                    None => format!("User: {}", user_message),
                };
                
                let prompt = CandlePrompt::new(full_prompt);
                let params = CandleCompletionParams {
                    temperature: state.temperature,
                    max_tokens: NonZeroU64::new(state.max_tokens.unwrap_or(1000)),
                    ..Default::default()
                };
                
                let completion_stream = state.provider.prompt(prompt, &params);
                let completion_results = completion_stream.collect();
                let mut assistant_response = String::new();
                
                for chunk in completion_results {
                    match chunk {
                        CandleCompletionChunk::Text(ref text) => {
                            assistant_response.push_str(text);
                            ystream::emit!(stream_sender, CandleMessageChunk::Text(text.clone()));
                        }
                        CandleCompletionChunk::Complete { ref text, finish_reason, usage } => {
                            assistant_response.push_str(text);
                            ystream::emit!(stream_sender, CandleMessageChunk::Complete {
                                text: text.clone(),
                                finish_reason: finish_reason.map(|f| format!("{:?}", f)),
                                usage: usage.map(|u| format!("{:?}", u)),
                            });
                        }
                        _ => {}
                    }
                }
                
                // CRITICAL: Call handler for recursion
                if let Some(ref handler) = state.on_conversation_turn_handler {
                    let mut conversation = CandleAgentConversation::new();
                    conversation.add_message(user_message.clone(), CandleMessageRole::User);
                    conversation.add_message(assistant_response.clone(), CandleMessageRole::Assistant);
                    
                    let agent = CandleAgentRoleAgent { state: state.clone() };
                    let handler_stream = handler(&conversation, &agent);
                    let handler_chunks = handler_stream.collect();
                    for chunk in handler_chunks {
                        ystream::emit!(stream_sender, chunk);
                    }
                }
            });
        })
    }
}
```

**Issue:** This is simplified. Full implementation must include memory search, tool routing, memory storage. Copy from existing chat() method.

### Task 4: Update Builder's chat() to Create Configured Agent

**File:** [src/builders/agent_role.rs](../packages/candle/src/builders/agent_role.rs)  
**Location:** Around line 1180 (where handler is called)

**Current:**
```rust
let agent = CandleAgentRoleAgent;
let handler_stream = handler(&conversation, &agent);
```

**Change to:**
```rust
let builder_state = Arc::new(AgentBuilderState {
    name: self.name.clone(),
    provider: provider.clone(),
    temperature,
    max_tokens: Some(max_tokens),
    memory_read_timeout,
    system_prompt: system_prompt.clone(),
    tools: tools.clone(),
    mcp_servers: Vec::new(),
    memory: memory.clone(),
    on_conversation_turn_handler: on_conversation_turn_handler.clone()
        .map(|h| Arc::new(h) as Arc<_>),
});

let agent = CandleAgentRoleAgent { state: builder_state };
let handler_stream = handler(&conversation, &agent);
```

**Issue:** May need to adjust trait signatures if they reference CandleAgentRoleAgent without generic parameter.

### Task 5: Rewrite CLI Runner to Use on_conversation_turn

**File:** [src/cli/runner.rs](../packages/candle/src/cli/runner.rs)  
**Location:** Lines 48-382 (replace run() method)

**Remove:** External `loop {}` starting at line 267

**Add:** on_conversation_turn configuration:
```rust
pub async fn run(&mut self) -> Result<()> {
    let model = self.select_model()?;
    let provider = ModelFactory::create_from_alias(&model).await?;
    let system_prompt = /* existing system prompt logic */;
    let memory_manager = /* existing memory initialization */;
    
    // Clone for closure
    let handler = self.handler.clone();
    let prompt_builder = self.prompt_builder.clone();

    // Configure recursive handler
    let builder = CandleFluentAi::agent_role(&self.args.agent_role)
        .completion_provider(provider)
        .temperature(self.args.temperature)
        .system_prompt(system_prompt)
        .memory(memory_manager)
        .on_conversation_turn(move |_conversation, agent| {
            let input = match prompt_builder.get_user_input("You: ") {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("Input error: {}", e);
                    return agent.chat(CandleChatLoop::Break);
                }
            };

            match handler.handle(&input) {
                InputHandlerResult::Exit => {
                    println!("Goodbye!");
                    agent.chat(CandleChatLoop::Break)
                }
                InputHandlerResult::Command(cmd_result) => {
                    println!("{}", CliRunner::format_command_result(&cmd_result));
                    agent.chat(CandleChatLoop::UserPrompt("".to_string()))
                }
                InputHandlerResult::Chat(message) => {
                    // Resolve with Document builder
                    let resolved = tokio::task::block_in_place(|| {
                        tokio::runtime::Handle::current().block_on(async {
                            resolve_smart_input(&message).await.unwrap_or(message)
                        })
                    });
                    agent.chat(CandleChatLoop::UserPrompt(resolved))
                }
                InputHandlerResult::None => {
                    agent.chat(CandleChatLoop::UserPrompt("".to_string()))
                }
            }
        });

    // Start conversation
    let mut stream = builder.into_agent()
        .chat(|_| CandleChatLoop::UserPrompt("Ready to chat!".to_string()))?;

    // Consume stream
    print!("Assistant: ");
    while let Some(chunk) = stream.next().await {
        match chunk {
            CandleMessageChunk::Text(text) => {
                print!("{}", text);
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }
            CandleMessageChunk::Complete { text, .. } => {
                if !text.is_empty() { print!("{}", text); }
                println!();
            }
            _ => {}
        }
    }

    self.save_config()?;
    Ok(())
}
```

**Issues:** 
- InputHandler and PromptBuilder must derive Clone
- Need to verify resolve_smart_input exists in util::input_resolver

---

## Definition of Done

1. CandleAgentRoleAgent holds Arc<AgentBuilderState<P>>
2. agent.chat() runs full inference pipeline (memory, tools, provider, streaming)
3. agent.chat() calls on_conversation_turn at end (enables recursion)
4. CLI runner configures on_conversation_turn with stdin reading
5. No external loop in runner.rs
6. Loop happens naturally: handler calls agent.chat() → inference → handler calls agent.chat() → ...

**Test Behavior:**
- Run CLI
- Type message
- See streaming response
- See "You: " prompt again (automatic, no restart)
- Type another message
- Loop continues until /exit

**Success:** One stream drives entire conversation through recursive callbacks.
