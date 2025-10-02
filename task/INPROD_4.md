# INPROD_4: Core Inference Engine Implementation

## SEVERITY: CRITICAL

## OBJECTIVE
Replace placeholder inference chunks with actual AI model inference in the KimiK2 provider integration. The inference engine is **FULLY IMPLEMENTED** - this task is about calling the existing `provider.prompt()` method instead of returning a hardcoded placeholder.

## CORE DISCOVERY
**The inference code already exists and works perfectly.** Both KimiK2Provider and Qwen3CoderProvider have complete, production-ready implementations of the `prompt()` method that perform real model inference with streaming tokens. The bug is simply that `lib.rs` doesn't call this method for KimiK2.

### Evidence:
- **KimiK2Provider.prompt()**: [./packages/candle/src/providers/kimi_k2.rs:409-551](../../packages/candle/src/providers/kimi_k2.rs)
  - Loads tokenizer from disk
  - Creates quantized GGUF model via `ModelFactory::create_quantized_llama()`
  - Initializes `TextGenerator` with sampling config
  - Streams real tokens using `Engine.coordinate_generation()`
  
- **Qwen3CoderProvider.prompt()**: [./packages/candle/src/providers/qwen3_coder.rs:396-537](../../packages/candle/src/providers/qwen3_coder.rs)
  - Identical implementation pattern to KimiK2
  - Proves the inference pipeline works end-to-end

## LOCATION
**Primary File**: `packages/candle/src/lib.rs`
- **Bug Location**: Lines 135-157 (KimiK2 case in CandleModels::prompt())
- **Working Reference**: Lines 162-191 (Qwen3Coder case - shows correct implementation)

## CURRENT STATE (THE BUG)

### KimiK2 Implementation (lib.rs:135-157)
```rust
CandleModels::KimiK2 => {
    // Route to KimiK2Provider with async handling
    ystream::spawn_task(|| async move {
        // Create provider with default config
        let provider_result =
            crate::providers::kimi_k2::CandleKimiK2Provider::new().await;

        match provider_result {
            Ok(_provider) => {  // ❌ Provider is created but IGNORED (note the underscore)
                // ❌ PLACEHOLDER - Returns hardcoded text instead of running inference
                let chunk =
                    crate::domain::completion::CandleCompletionChunk::Text(
                        format!("KimiK2 completion: {}", prompt.content()),
                    );
                let _ = sender.send(chunk);
            }
            Err(_err) => {
                let error_chunk =
                    crate::domain::completion::CandleCompletionChunk::Error(
                        "Failed to initialize KimiK2 provider".to_string(),
                    );
                let _ = sender.send(error_chunk);
            }
        }
    });
}
```

**Problems**:
1. Provider created successfully but variable named `_provider` (underscore = intentionally unused)
2. Returns placeholder chunk instead of calling `provider.prompt()`
3. Doesn't clone prompt/params before async closure
4. Doesn't stream real tokens from the model

### Working Implementation (Qwen3Coder - lib.rs:162-191)
```rust
CandleModels::Qwen3Coder => {
    // Clone values before moving into async closure
    let prompt_clone = prompt.clone();
    let params_clone = params.clone();
    
    // Route to Qwen3CoderProvider with async handling
    ystream::spawn_task(|| async move {
        // Create provider with default config
        let provider_result =
            crate::providers::qwen3_coder::CandleQwen3CoderProvider::new().await;

        match provider_result {
            Ok(provider) => {  // ✅ Provider is used (no underscore)
                // ✅ Call real provider.prompt() method for inference
                let completion_stream = provider.prompt(prompt_clone, &params_clone);
                
                // ✅ Stream all chunks from provider to outer sender
                while let Some(chunk) = completion_stream.try_next() {
                    if sender.send(chunk).is_err() {
                        // Receiver dropped, exit gracefully
                        return;
                    }
                }
            }
            Err(err) => {
                // Send error chunk if provider creation fails
                let error_chunk =
                    crate::domain::completion::CandleCompletionChunk::Error(
                        format!("Failed to initialize Qwen3Coder provider: {}", err)
                    );
                let _ = sender.send(error_chunk);
            }
        }
    });
}
```

## THE FIX: Make KimiK2 Match Qwen3Coder

### Step-by-Step Changes in lib.rs

**1. Add prompt/params cloning BEFORE the KimiK2 match arm** (before line 135):
```rust
CandleModels::KimiK2 => {
    // Clone values before moving into async closure
    let prompt_clone = prompt.clone();
    let params_clone = params.clone();
```

**2. Rename `_provider` to `provider`** (line 143):
```rust
Ok(provider) => {  // Remove underscore - we're using it now
```

**3. Replace placeholder chunk with real inference call** (lines 145-149):
```rust
// Call real provider.prompt() method for inference
let completion_stream = provider.prompt(prompt_clone, &params_clone);

// Stream all chunks from provider to outer sender
while let Some(chunk) = completion_stream.try_next() {
    if sender.send(chunk).is_err() {
        // Receiver dropped, exit gracefully
        return;
    }
}
```

**4. Improve error message** (line 153):
```rust
format!("Failed to initialize KimiK2 provider: {}", err)  // Include error details
```

### Complete Fixed Implementation
```rust
CandleModels::KimiK2 => {
    // Clone values before moving into async closure
    let prompt_clone = prompt.clone();
    let params_clone = params.clone();
    
    // Route to KimiK2Provider with async handling
    ystream::spawn_task(|| async move {
        // Create provider with default config
        let provider_result =
            crate::providers::kimi_k2::CandleKimiK2Provider::new().await;

        match provider_result {
            Ok(provider) => {
                // Call real provider.prompt() method for inference
                let completion_stream = provider.prompt(prompt_clone, &params_clone);
                
                // Stream all chunks from provider to outer sender
                while let Some(chunk) = completion_stream.try_next() {
                    if sender.send(chunk).is_err() {
                        // Receiver dropped, exit gracefully
                        return;
                    }
                }
            }
            Err(err) => {
                // Send error chunk if provider creation fails
                let error_chunk =
                    crate::domain::completion::CandleCompletionChunk::Error(
                        format!("Failed to initialize KimiK2 provider: {}", err)
                    );
                let _ = sender.send(error_chunk);
            }
        }
    });
}
```

## TECHNICAL ARCHITECTURE REFERENCE

### Inference Pipeline (Already Implemented)
The providers implement a complete inference pipeline in their `prompt()` methods:

1. **Engine Coordination** ([./packages/candle/src/core/engine.rs](../../packages/candle/src/core/engine.rs))
   - `Engine::coordinate_generation()` orchestrates the generation process
   - Handles metrics collection automatically
   - Converts string chunks to `CandleCompletionChunk`

2. **Model Loading** ([./packages/candle/src/core/generation/models.rs](../../packages/candle/src/core/generation/models.rs))
   - `ModelFactory::create_quantized_llama()` loads GGUF models
   - Supports quantized models (Q4_K_M, Q5_K_M, etc.)
   - Handles both KimiK2 and Qwen3Coder architectures

3. **Text Generation** ([./packages/candle/src/core/generation/generator.rs](../../packages/candle/src/core/generation/generator.rs))
   - `TextGenerator::generate()` performs actual inference
   - Streams tokens using `AsyncStream`
   - Respects max_tokens and stop sequences

4. **Sampling Configuration** ([./packages/candle/src/core/generation/config.rs](../../packages/candle/src/core/generation/config.rs))
   - `SamplingConfig` handles temperature, top_k, top_p
   - Supports repetition penalty, frequency penalty, presence penalty
   - SIMD-optimized sampling operations

### Key Types and Traits

**CandleCompletionModel trait** ([./packages/candle/src/domain/completion/traits.rs](../../packages/candle/src/domain/completion/traits.rs)):
```rust
pub trait CandleCompletionModel {
    fn prompt(
        &self,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> AsyncStream<CandleCompletionChunk>;
}
```

**CandleCompletionChunk enum** ([./packages/candle/src/domain/completion/core.rs](../../packages/candle/src/domain/completion/core.rs)):
```rust
pub enum CandleCompletionChunk {
    Text(String),
    Complete { text: String, finish_reason: Option<String>, usage: Option<UsageInfo> },
    Error(String),
    // ... other variants
}
```

## WHAT NEEDS TO CHANGE

### File: `packages/candle/src/lib.rs`
**Lines to modify**: 135-157 (KimiK2 match arm in `CandleModels::prompt()`)

**Changes**:
1. **Before line 135**: Add prompt/params cloning
2. **Line 143**: Change `_provider` to `provider`
3. **Lines 145-149**: Replace placeholder chunk with:
   - Call to `provider.prompt(prompt_clone, &params_clone)`
   - While loop to stream chunks to sender
4. **Line 153**: Include error details in error message

**Pattern to follow**: Copy Qwen3Coder implementation (lines 162-191) and adapt variable names.

## DEFINITION OF DONE

### Functional Requirements
- [x] KimiK2Provider's `prompt()` method is called instead of placeholder
- [x] Real tokens are streamed from the model via `completion_stream.try_next()`
- [x] Prompt and params are properly cloned before async closure
- [x] Error messages include error details from provider initialization
- [x] Implementation matches Qwen3Coder pattern exactly

### Code Quality
- [x] No placeholder chunks remain in KimiK2 case
- [x] Variable naming is correct (`provider` not `_provider`)
- [x] Async closure has proper variable ownership
- [x] Streaming loop handles receiver drop gracefully

### Verification Steps
1. Run `cargo check -p paraphym_candle` - should compile without errors
2. Inspect lib.rs:135-157 - should match Qwen3Coder pattern
3. Verify no placeholder text is returned for KimiK2
4. Confirm all chunks come from `provider.prompt()` call

## CONSTRAINTS
- NO tests to be written (separate team responsibility)
- NO benchmarks to be written (separate team responsibility)  
- NO documentation files to be created
- Focus solely on fixing lib.rs lines 135-157

## SUMMARY
This is a **copy-paste fix**, not a new implementation. The inference engine is complete and proven to work in Qwen3CoderProvider. Simply apply the same pattern to KimiK2Provider's integration point in lib.rs. The change is approximately 15 lines that activate existing, tested inference code.