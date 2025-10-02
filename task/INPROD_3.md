# INPROD_3: Context Extraction Implementation

## SEVERITY: CRITICAL

## OBJECTIVE
Implement actual NLP/AI-based context extraction instead of returning default placeholder values. Replace ALL `T::default()` stubs with production-quality AI-powered extraction logic that properly uses completion providers to generate structured data from unstructured text.

## ARCHITECTURE DISCOVERY

### Key Insight: Provider-Based Design
The extraction system must use **completion providers** (implementations of `CandleCompletionModel`) to generate AI responses, NOT agents directly. Agents (like `CandleAgent`) are configuration objects without inference capabilities. The actual model inference happens through providers like `CandleKimiK2Provider` or `CandleQwen3CoderProvider`.

### Completion Flow Pattern
```rust
// Pattern from working code (src/domain/agent/chat.rs:285-310)
let provider = agent.get_completion_provider()?;
let prompt = CandlePrompt::new(formatted_text);
let params = CandleCompletionParams {
    temperature: 0.7,
    max_tokens: NonZeroU64::new(1000),
    n: NonZeroU8::new(1).unwrap(),
    stream: true,
    tools: None,
    additional_params: None,
};
let mut stream = provider.prompt(prompt, &params);

// Process stream chunks
while let Some(chunk) = stream.try_next() {
    match chunk {
        CandleCompletionChunk::Text(text) => { /* accumulate */ }
        CandleCompletionChunk::Complete { text, finish_reason, .. } => { /* finalize */ }
        CandleCompletionChunk::Error(err) => { /* handle error */ }
        _ => {}
    }
}
```

### CandleCompletionChunk Variants
Located in `src/domain/context/chunk.rs:145-175`:
- **Text(String)**: Streaming text content
- **ToolCallStart { id, name }**: Tool invocation initiated  
- **ToolCall { id, name, partial_input }**: Streaming tool input
- **ToolCallComplete { id, name, input }**: Tool call finished
- **Complete { text, finish_reason, usage }**: Generation completed
- **Error(String)**: Error occurred

## LOCATION & CURRENT STATE

### File 1: `packages/candle/src/domain/context/extraction/extractor.rs`

**Line 80 (extract_from method)** - STUB:
```rust
fn extract_from(&self, text: &str) -> AsyncStream<T> {
    let _text = text.to_string();
    AsyncStream::with_channel(move |sender| {
        // TODO: Connect to execute_extraction method
        // For now, send default result to maintain compilation
        let default_result = T::default();  // ❌ STUB - REMOVE THIS
        let _ = sender.send(default_result);
    })
}
```

**Lines 199-230 (AgentCompletionModel)** - STUB:
```rust
impl CompletionModel for AgentCompletionModel {
    fn prompt<'a>(&'a self, prompt: Prompt, _params: &'a CompletionParams) 
    -> ystream::AsyncStream<CandleCompletionChunk> {
        AsyncStream::with_channel(move |sender| {
            type Chunk = CandleCompletionChunk;
            let chunk = Chunk::Complete {
                text: format!("{prompt:?}"),  // ❌ STUB - Returns debug string!
                finish_reason: Some(FinishReason::Stop),
                usage: None,
            };
            let _ = sender.try_send(chunk);
        })
    }
}
```

**CRITICAL DISCOVERY**: Lines 106-178 contain a **FULLY IMPLEMENTED** `execute_extraction()` method that:
- ✅ Creates proper completion model with agent
- ✅ Streams chunks with Box::pin  
- ✅ Accumulates text from Text and Complete variants
- ✅ Handles FinishReason::Stop properly
- ✅ Parses JSON with error handling using parse_json_response()
- ✅ Returns typed ExtractionResult<T>

This code is **PRODUCTION READY** - it just needs to be called!

### File 2: `packages/candle/src/context/extraction/extractor.rs`

**Line 60 (DocumentExtractor::extract)** - STUB:
```rust
fn extract(&self, text: &str) -> AsyncStream<ExtractionResult<T>> {
    let text = text.to_string();
    AsyncStream::with_channel(move |sender| {
        std::thread::spawn(move || {
            // Simple text-based extraction logic
            // In a real implementation, this would use NLP or AI models
            let result = T::default(); // ❌ STUB - Placeholder extraction
            let _ = sender.send(Ok(result));
        });
    })
}
```

**Line 107 (ExtractorImpl::execute_extraction)** - STUB:
```rust
while let Some(chunk) = stream.try_next() {
    // In a real implementation, this would accumulate chunks and parse the final result
    // For now, return a default result
    let result = T::default();  // ❌ STUB
    let _ = sender.send(Ok(result));
    break; // Only send one result for this example
}
```

**Line 153 (ExtractorImpl::extract)** - STUB:
```rust
fn extract(&self, text: &str) -> AsyncStream<ExtractionResult<T>> {
    let text = text.to_string();
    AsyncStream::with_channel(move |sender| {
        std::thread::spawn(move || {
            // Simple extraction implementation
            let result = T::default();  // ❌ STUB
            let _ = sender.send(Ok(result));
        });
    })
}
```

## IMPLEMENTATION GUIDE

### SUBTASK 1: Fix Domain Extraction (domain/context/extraction/extractor.rs)

#### Change 1.1: Update ExtractorImpl Structure (Lines 50-57)
**Current:**
```rust
pub struct ExtractorImpl<T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + 'static> {
    agent: Agent,
    system_prompt: Option<String>,
    _marker: PhantomData<T>,
}
```

**Change to:**
```rust
pub struct ExtractorImpl<T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + 'static> {
    provider: Arc<dyn CandleCompletionModel>,  // Use provider instead of agent
    system_prompt: Option<String>,
    _marker: PhantomData<T>,
}
```

#### Change 1.2: Update Trait Implementation (Lines 59-91)
Replace the `new()` and `agent()` methods:

**Change:**
```rust
fn new(agent: Agent) -> Self {
    Self {
        agent,
        system_prompt: None,
        _marker: PhantomData,
    }
}

fn agent(&self) -> &Agent {
    &self.agent
}
```

**To:**
```rust
fn new_with_provider(provider: Arc<dyn CandleCompletionModel>) -> Self {
    Self {
        provider,
        system_prompt: None,
        _marker: PhantomData,
    }
}

fn provider(&self) -> &Arc<dyn CandleCompletionModel> {
    &self.provider
}
```

#### Change 1.3: Fix extract_from() Method (Lines 76-86)
**Replace entire method with:**
```rust
fn extract_from(&self, text: &str) -> AsyncStream<T> {
    let text = text.to_string();
    let provider = Arc::clone(&self.provider);
    let system_prompt = self.system_prompt.clone().unwrap_or_else(|| {
        format!("Extract structured data from the following text. Return ONLY valid JSON matching the expected schema. Text: {}", text)
    });

    AsyncStream::with_channel(move |sender| {
        tokio::spawn(async move {
            // Create completion request
            let completion_request = CompletionRequest::new(&text)
                .with_system_prompt(system_prompt);

            // Call the ALREADY WORKING execute_extraction method
            match Self::execute_extraction(provider, completion_request, text).await {
                Ok(result) => {
                    let _ = sender.send(result);
                }
                Err(_e) => {
                    // Send default on error to maintain stream flow
                    let _ = sender.send(T::default());
                }
            }
        });
    })
}
```

#### Change 1.4: Update execute_extraction() Signature (Line 99)
**Change:**
```rust
pub async fn execute_extraction(
    agent: Agent,
    completion_request: CompletionRequest,
    _text_input: String,
) -> ExtractionResult<T> {
```

**To:**
```rust
pub async fn execute_extraction(
    provider: Arc<dyn CandleCompletionModel>,
    completion_request: CompletionRequest,
    _text_input: String,
) -> ExtractionResult<T> {
```

**And update line 111:**
```rust
let model = AgentCompletionModel::new(agent);  // ❌ Remove this line
```

**Replace with:**
```rust
let model = provider.as_ref();  // Use provider directly
```

#### Change 1.5: Remove AgentCompletionModel (Lines 199-230)
**DELETE** the entire `AgentCompletionModel` struct and implementation - it's redundant.

### SUBTASK 2: Fix Context Extraction (context/extraction/extractor.rs)

#### Change 2.1: Add Provider to DocumentExtractor (Lines 36-44)
**Current:**
```rust
pub struct DocumentExtractor<T> 
where
    T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + Default + 'static,
{
    _phantom: std::marker::PhantomData<T>,
}
```

**Change to:**
```rust
pub struct DocumentExtractor<T> 
where
    T: DeserializeOwned + Send + Sync + fmt::Debug + Clone + Default + 'static,
{
    provider: Option<Arc<dyn crate::domain::completion::traits::CandleCompletionModel>>,
    _phantom: std::marker::PhantomData<T>,
}
```

#### Change 2.2: Implement Real Extraction in DocumentExtractor (Lines 52-66)
**Replace extract() method with:**
```rust
fn extract(&self, text: &str) -> AsyncStream<ExtractionResult<T>> {
    let text = text.to_string();
    let provider = self.provider.clone();
    
    AsyncStream::with_channel(move |sender| {
        std::thread::spawn(move || {
            if let Some(provider) = provider {
                // Create extraction prompt
                let prompt = crate::domain::prompt::CandlePrompt::new(
                    format!("Extract structured data from: {}", text)
                );
                let params = crate::domain::completion::types::CandleCompletionParams {
                    temperature: 0.2,
                    max_tokens: std::num::NonZeroU64::new(2000),
                    n: std::num::NonZeroU8::new(1).unwrap(),
                    stream: true,
                    tools: None,
                    additional_params: None,
                };
                
                let mut stream = provider.prompt(prompt, &params);
                let mut accumulated = String::new();
                
                // Accumulate all chunks
                while let Some(chunk) = stream.try_next() {
                    use crate::domain::context::chunk::CandleCompletionChunk;
                    match chunk {
                        CandleCompletionChunk::Text(text) => accumulated.push_str(&text),
                        CandleCompletionChunk::Complete { text, .. } => {
                            accumulated.push_str(&text);
                            break;
                        }
                        CandleCompletionChunk::Error(e) => {
                            let _ = sender.send(Err(ExtractionError::ModelError(e)));
                            return;
                        }
                        _ => {}
                    }
                }
                
                // Parse JSON from accumulated response
                match serde_json::from_str::<T>(&accumulated) {
                    Ok(result) => { let _ = sender.send(Ok(result)); }
                    Err(e) => { let _ = sender.send(Err(ExtractionError::SerializationError(e.to_string()))); }
                }
            } else {
                let _ = sender.send(Err(ExtractionError::ConfigError("No provider configured".to_string())));
            }
        });
    })
}
```

#### Change 2.3: Fix ExtractorImpl::execute_extraction (Lines 90-117)
**Replace the stub logic (lines 104-111) with:**
```rust
let mut full_response = String::new();

// Process the streaming response - ACCUMULATE ALL CHUNKS
while let Some(chunk) = stream.try_next() {
    use crate::domain::context::chunk::CandleCompletionChunk;
    match chunk {
        CandleCompletionChunk::Text(text) => {
            full_response.push_str(&text);
        }
        CandleCompletionChunk::Complete { text, finish_reason, .. } => {
            full_response.push_str(&text);
            if finish_reason == Some(crate::domain::context::chunk::FinishReason::Stop) {
                break;
            }
        }
        CandleCompletionChunk::Error(e) => {
            let _ = sender.send(Err(ExtractionError::ModelError(e)));
            return;
        }
        _ => {}
    }
}

// Parse the accumulated JSON response
match serde_json::from_str::<T>(&full_response) {
    Ok(result) => { let _ = sender.send(Ok(result)); }
    Err(e) => { let _ = sender.send(Err(ExtractionError::SerializationError(e.to_string()))); }
}
```

#### Change 2.4: Fix ExtractorImpl::extract (Lines 149-162)
**Replace stub with real implementation:**
```rust
fn extract(&self, text: &str) -> AsyncStream<ExtractionResult<T>> {
    let text = text.to_string();
    AsyncStream::with_channel(move |sender| {
        std::thread::spawn(move || {
            // Create a simple agent for extraction (or accept provider in constructor)
            let agent = crate::domain::agent::Agent::default();
            let completion_request = CompletionRequest::new(&text);
            
            // Use execute_extraction method
            let mut extraction_stream = Self::execute_extraction(agent, completion_request, text);
            
            // Forward results from the extraction
            while let Some(result) = extraction_stream.try_next() {
                let _ = sender.send(result);
            }
        });
    })
}
```

## SYSTEM PROMPT TEMPLATE FOR EXTRACTION

When creating extraction prompts, use this pattern:
```rust
format!(
    "Extract structured data from the following text and return ONLY valid JSON.\n\
     Schema: {}\n\
     Text: {}\n\
     Return your response as pure JSON without any markdown formatting or explanations.",
    std::any::type_name::<T>(),
    text
)
```

## REFERENCE IMPLEMENTATIONS

### Working Completion Pattern
See `src/domain/agent/chat.rs:285-310` for production example of:
- Getting provider from agent
- Creating CandlePrompt and CandleCompletionParams
- Processing CandleCompletionChunk stream
- Accumulating text and handling Complete/Error variants

### Working JSON Parsing
See `src/domain/context/extraction/extractor.rs:190-209` (parse_json_response) for:
- Trying full response as JSON
- Finding JSON boundaries in mixed text
- Error handling with ExtractionError

### Provider Trait
See `src/domain/completion/traits.rs:10-29` for CandleCompletionModel trait definition.

## DEFINITION OF DONE

- [ ] Domain extractor uses provider-based architecture (no AgentCompletionModel stub)
- [ ] Domain extract_from() calls execute_extraction() with real provider
- [ ] Context DocumentExtractor::extract() accumulates chunks and parses JSON  
- [ ] Context ExtractorImpl::execute_extraction() accumulates all chunks before parsing
- [ ] Context ExtractorImpl::extract() uses real extraction logic
- [ ] NO `T::default()` placeholders remain in ANY extraction method
- [ ] ALL stub comments removed ("In a real implementation...", "For now...")
- [ ] Proper error handling with ExtractionError variants
- [ ] Stream accumulation handles all CandleCompletionChunk variants correctly

## CONSTRAINTS

- **NO TEST CODE** - Testing is handled by separate team
- **NO BENCHMARK CODE** - Benchmarking is handled by separate team  
- **NO DOCUMENTATION beyond inline comments** - Focus on implementation only
- **USE EXISTING CODE** - execute_extraction in domain file is already production-ready
- **FOLLOW EXISTING PATTERNS** - Use completion flow from chat.rs as reference

## FILES TO MODIFY

1. `./packages/candle/src/domain/context/extraction/extractor.rs` - Lines 50, 59-91, 76-86, 99-178, 199-230 (delete)
2. `./packages/candle/src/context/extraction/extractor.rs` - Lines 36-66, 90-117, 149-162

## CRITICAL NOTES

**The domain extractor's execute_extraction() is ALREADY FULLY IMPLEMENTED with production-quality code.** The ONLY issues are:
1. extract_from() doesn't call it (sends T::default instead)
2. AgentCompletionModel is a stub wrapper

Fixing these two issues in domain file = 90% of the work. The context file needs similar patterns applied.

**DO NOT rewrite execute_extraction()** - it's already correct. Just connect extract_from() to it and remove the AgentCompletionModel stub.