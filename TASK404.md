# TASK404: Add Response Type Conversion Utilities

## Objective
Document and utilize existing conversion utilities between CandleStringChunk (from TextGenerator) and CandleCompletionChunk (expected by providers) to ensure proper streaming response handling across all providers.

## Current State

**IMPORTANT DISCOVERY**: The conversion utilities requested in this task are **ALREADY FULLY IMPLEMENTED** in the codebase. The task is now about refactoring providers to use these existing utilities instead of manual conversion.

### Existing Implementation

#### 1. Engine Conversion Utilities (ALREADY IMPLEMENTED)
Location: [`packages/candle/src/core/engine.rs`](./packages/candle/src/core/engine.rs)

**`Engine::coordinate_generation()` method** (lines 422-443):
```rust
/// Coordinate text generation with metrics and streaming management
/// 
/// Provides orchestration services for providers:
/// - Automatic metrics tracking (request_count, active_requests, etc.)
/// - Stream conversion from CandleStringChunk to CandleCompletionChunk  
/// - Error handling and health monitoring
/// - Performance timing and throughput calculation
pub fn coordinate_generation<F>(&self, generation_fn: F) 
    -> AsyncStream<CandleCompletionChunk>
where 
    F: FnOnce() -> AsyncStream<CandleStringChunk> + Send + 'static
{
    // Update metrics atomically
    self.request_count.fetch_add(1, Ordering::Relaxed);
    self.active_requests.fetch_add(1, Ordering::Relaxed);
    
    // Execute provider's generation function
    let text_stream = generation_fn();
    
    // Convert and manage streaming response with metrics
    self.manage_streaming_response(text_stream)
}
```

**`Engine::manage_streaming_response()` method** (lines 445-488):
```rust
/// Convert TextGenerator output to completion chunks with metrics tracking
fn manage_streaming_response(&self, text_stream: AsyncStream<CandleStringChunk>) 
    -> AsyncStream<CandleCompletionChunk> {
    
    let active_requests = Arc::clone(&self.active_requests);
    let successful_requests = Arc::clone(&self.successful_requests);
    let failed_requests = Arc::clone(&self.failed_requests);
    
    AsyncStream::with_channel(move |sender| {
        let start_time = std::time::Instant::now();
        let mut token_count = 0u32;
        let mut has_error = false;
        
        // Process each text chunk from TextGenerator
        for string_chunk in text_stream {
            token_count += 1;
            
            // Convert CandleStringChunk to CandleCompletionChunk::Text
            let completion_chunk = CandleCompletionChunk::Text(string_chunk.0);
            
            if sender.send(completion_chunk).is_err() {
                // Client disconnected
                has_error = true;
                break;
            }
        }
        
        // Send completion marker with performance metrics
        let final_chunk = CandleCompletionChunk::Complete {
            text: String::new(),
            finish_reason: if has_error { 
                Some(FinishReason::Error) 
            } else { 
                Some(FinishReason::Stop) 
            },
            usage: Some(CandleUsage {
                input_tokens: 0,
                output_tokens: token_count,
                total_tokens: token_count,
            }),
        };
        let _ = sender.send(final_chunk);
        
        // Update metrics atomically on completion
        active_requests.fetch_sub(1, Ordering::Relaxed);
        if has_error {
            failed_requests.fetch_add(1, Ordering::Relaxed);
        } else {
            successful_requests.fetch_add(1, Ordering::Relaxed);
        }
    })
}
```

#### 2. Chunk Type Definitions (ALREADY IMPLEMENTED)
Location: [`packages/candle/src/domain/context/chunk.rs`](./packages/candle/src/domain/context/chunk.rs)

**CandleStringChunk** (line 203):
```rust
/// Simple wrapper for String to implement `MessageChunk`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CandleStringChunk(pub String);
```

**CandleCompletionChunk** (lines 140-176):
```rust
/// Comprehensive completion chunk supporting all streaming features
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CandleCompletionChunk {
    /// Text content chunk
    Text(String),

    /// Tool call started
    ToolCallStart { id: String, name: String },

    /// Partial tool call with streaming input
    ToolCall {
        id: String,
        name: String,
        partial_input: String,
    },

    /// Tool call completed
    ToolCallComplete {
        id: String,
        name: String,
        input: String,
    },

    /// Completion finished with final information
    Complete {
        text: String,
        finish_reason: Option<FinishReason>,
        usage: Option<Usage>,
    },

    /// Error occurred during streaming
    Error(String),
}
```

#### 3. TextGenerator Output (ALREADY WORKING)
Location: [`packages/candle/src/core/generation/generator.rs`](./packages/candle/src/core/generation/generator.rs)

The TextGenerator's `generate()` method (line 87) returns `AsyncStream<CandleStringChunk>`:
```rust
pub fn generate(
    mut self,
    prompt: String,
    max_tokens: u32,
    special_tokens: SpecialTokens,
) -> AsyncStream<CandleStringChunk> {
    AsyncStream::with_channel(move |sender| {
        // ... token generation loop ...
        emit!(sender, CandleStringChunk(token_str));
    })
}
```

### Current Problem: Manual Conversion in Providers

Providers like KimiK2 are doing **manual conversion** instead of using Engine utilities.

**Example from** [`packages/candle/src/providers/kimi_k2.rs`](./packages/candle/src/providers/kimi_k2.rs) (lines 565-574):
```rust
// Generate text using TextGenerator
let text_stream = text_generator.generate(
    _prompt_text,
    max_tokens_u32,
    special_tokens,
);

// ❌ MANUAL CONVERSION - should use Engine instead
let text_chunks: Vec<CandleStringChunk> = text_stream.collect();
for string_chunk in text_chunks {
    let completion_chunk = CandleCompletionChunk::Text(string_chunk.0);
    emit!(sender, completion_chunk);
}
```

## What Needs to Change

### Refactor Providers to Use Engine Utilities

Instead of manual conversion, providers should use `Engine::coordinate_generation()`:

**BEFORE (Manual - Current):**
```rust
// In provider's prompt() method
AsyncStream::with_channel(move |sender| {
    // ... setup code ...
    
    let text_stream = text_generator.generate(prompt, max_tokens, special_tokens);
    
    // Manual conversion
    let text_chunks: Vec<CandleStringChunk> = text_stream.collect();
    for string_chunk in text_chunks {
        let completion_chunk = CandleCompletionChunk::Text(string_chunk.0);
        emit!(sender, completion_chunk);
    }
})
```

**AFTER (Using Engine - Target):**
```rust
// In provider's prompt() method
let engine = Engine::new(engine_config)?;

engine.coordinate_generation(move || {
    // ... setup code ...
    text_generator.generate(prompt, max_tokens, special_tokens)
})
```

### Files That Need Modification

1. **[`packages/candle/src/providers/kimi_k2.rs`](./packages/candle/src/providers/kimi_k2.rs)**
   - Lines 490-580: Refactor `prompt()` method to use Engine
   - Remove manual stream collection and iteration
   - Pass generation function to `engine.coordinate_generation()`

2. **[`packages/candle/src/providers/qwen3_coder.rs`](./packages/candle/src/providers/qwen3_coder.rs)** (if it exists and has similar pattern)
   - Apply same refactoring pattern

### Implementation Steps

#### Step 1: Add Engine to Provider Struct
```rust
pub struct CandleKimiK2Provider {
    model_path: String,
    gguf_file_path: String,
    config: CandleKimiK2Config,
    model_config: LlamaConfig,
    engine: Arc<Engine>, // ← Add this
}
```

#### Step 2: Initialize Engine in Constructor
```rust
impl CandleKimiK2Provider {
    pub fn with_config_sync_gguf(
        model_cache_dir: String,
        gguf_file_path: String,
        config: CandleKimiK2Config,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // ... existing code ...
        
        // Create engine configuration
        let engine_config = EngineConfig::new("kimi-k2", "candle-kimi")
            .with_streaming()
            .with_max_tokens(config.max_context)
            .with_temperature(config.temperature as f32);
        
        let engine = Arc::new(Engine::new(engine_config)?);
        
        Ok(Self {
            model_path: model_cache_dir,
            gguf_file_path,
            config,
            model_config,
            engine, // ← Add this
        })
    }
}
```

#### Step 3: Refactor prompt() Method
```rust
impl CandleCompletionModel for CandleKimiK2Provider {
    fn prompt(
        &self,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> AsyncStream<CandleCompletionChunk> {
        let engine = Arc::clone(&self.engine);
        let model_path = self.model_path.clone();
        let gguf_file_path = self.gguf_file_path.clone();
        let config = self.config.clone();
        let model_config = self.model_config.clone();
        
        let prompt_text = format!("User: {}\nAssistant: ", prompt);
        let max_tokens = params.max_tokens
            .map(|n| n.get())
            .unwrap_or(1000)
            .try_into()
            .unwrap_or(u32::MAX);
        
        // Use Engine's coordinate_generation
        engine.coordinate_generation(move || {
            // All setup code goes inside the closure
            let device = Device::Cpu;
            
            let tokenizer = Tokenizer::from_file(
                format!("{}/tokenizer.json", model_path)
            ).expect("tokenizer load");
            
            let candle_model_config = Arc::new(CandleConfig::new(/* ... */));
            let quantized_model = ModelFactory::create_quantized_llama(
                &gguf_file_path, 
                candle_model_config, 
                device.clone()
            ).expect("model load");
            
            let sampling_config = SamplingConfig::new(params.temperature as f32);
            let text_generator = TextGenerator::new(
                Box::new(quantized_model),
                tokenizer,
                device,
                sampling_config,
            );
            
            let special_tokens = SpecialTokens {
                bos_token_id: Some(1),
                eos_token_id: Some(2),
                pad_token_id: None,
            };
            
            // Return the text stream - Engine handles conversion
            text_generator.generate(prompt_text, max_tokens, special_tokens)
        })
    }
}
```

## Benefits of This Refactoring

1. **Automatic Metrics**: Engine tracks request counts, success/failure rates, active requests
2. **Consistent Error Handling**: Standardized across all providers
3. **Performance Monitoring**: Automatic timing and throughput calculation
4. **Cleaner Code**: Providers focus on generation, Engine handles orchestration
5. **Maintenance**: Single point for conversion logic updates

## Architecture Overview

```
┌─────────────────┐
│   Provider      │
│  (KimiK2, etc)  │
└────────┬────────┘
         │ calls
         ▼
┌─────────────────────────────┐
│   Engine                    │
│  .coordinate_generation()   │
│   - Updates metrics         │
│   - Executes generation fn  │
│   - Converts stream         │
└────────┬────────────────────┘
         │ executes
         ▼
┌─────────────────────────────┐
│   TextGenerator             │
│   .generate()               │
│   Returns:                  │
│   AsyncStream<              │
│     CandleStringChunk       │
│   >                         │
└────────┬────────────────────┘
         │ converts via
         │ manage_streaming_response()
         ▼
┌─────────────────────────────┐
│   AsyncStream<              │
│     CandleCompletionChunk   │
│   >                         │
│   - Text(String)            │
│   - Complete { usage, ... } │
│   - Error(String)           │
└─────────────────────────────┘
```

## Definition of Done

- [x] Engine conversion utilities exist and are documented (ALREADY DONE)
- [ ] KimiK2 provider refactored to use `Engine::coordinate_generation()`
- [ ] Qwen3Coder provider refactored (if applicable)
- [ ] Manual stream collection removed from all providers
- [ ] All providers using Engine benefit from automatic metrics
- [ ] No compilation errors in workspace

## Files Modified (Planned)

- [`packages/candle/src/providers/kimi_k2.rs`](./packages/candle/src/providers/kimi_k2.rs) - Refactor to use Engine
- [`packages/candle/src/providers/qwen3_coder.rs`](./packages/candle/src/providers/qwen3_coder.rs) - Refactor to use Engine (if exists)
- No changes needed to [`packages/candle/src/core/engine.rs`](./packages/candle/src/core/engine.rs) - utilities already complete
- No changes needed to [`packages/candle/src/domain/context/chunk.rs`](./packages/candle/src/domain/context/chunk.rs) - types already complete

## Related Code References

- **Engine implementation**: [packages/candle/src/core/engine.rs:422-488](./packages/candle/src/core/engine.rs)
- **Chunk definitions**: [packages/candle/src/domain/context/chunk.rs:140-210](./packages/candle/src/domain/context/chunk.rs)
- **TextGenerator**: [packages/candle/src/core/generation/generator.rs:87-195](./packages/candle/src/core/generation/generator.rs)
- **KimiK2 Provider (needs refactor)**: [packages/candle/src/providers/kimi_k2.rs:490-580](./packages/candle/src/providers/kimi_k2.rs)

## Dependencies

- ~~TASK400 (Engine orchestration utilities)~~ - **ALREADY IMPLEMENTED**
- All required utilities are already in the codebase
- No external dependencies needed
- No new crates or libraries required
