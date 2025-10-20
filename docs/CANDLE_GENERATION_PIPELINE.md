# Candle Generation Pipeline Architecture

## Overview

The Candle generation pipeline provides local AI model inference using SIMD-accelerated text generation. The architecture follows a clean separation of concerns where Providers encapsulate model-specific generation logic, Engine provides orchestration utilities, and TextGenerator handles the actual SIMD-optimized inference.

## Architecture Components

### 1. Chat System (Entry Point)
- **Location**: `/packages/candle/src/domain/agent/chat.rs`
- **Responsibility**: Initiates text generation requests
- **Interface**: Calls `provider.prompt(prompt, params)`

### 2. Provider (Model Encapsulation)
- **Location**: `/packages/candle/src/providers/`
- **Examples**: `kimi_k2.rs`, `qwen3_coder.rs`
- **Responsibility**: 
  - Encapsulates model-specific generation logic
  - Manages TextGenerator lifecycle
  - Handles model loading via ProgressHub
  - Owns the generation process
- **Interface**: Implements `CandleCompletionModel::prompt()`

### 3. Engine (Orchestration Utilities)
- **Location**: `/packages/candle/src/core/engine.rs`
- **Responsibility**:
  - Coordinates streaming from TextGenerator
  - Handles metrics, statistics, and performance tracking
  - Manages request lifecycle and validation
  - Provides error handling and circuit breaker functionality
  - Formats responses and manages streaming coordination
- **Interface**: `coordinate_generation()` method called by Providers

### 4. TextGenerator (SIMD Generation Engine)
- **Location**: `/packages/candle/src/core/generation/generator.rs`
- **Responsibility**:
  - Performs actual AI inference using Candle models
  - SIMD-accelerated sampling (temperature, top-k, top-p, softmax, argmax)
  - Token streaming via AsyncStream
  - Model forward passes and tokenization
- **Interface**: `generate(prompt, max_tokens, special_tokens)`

### 5. SIMD Operations
- **Location**: `/packages/candle/src/core/simd_adapters.rs` + `cyrup_simd`
- **Responsibility**: High-performance sampling operations
- **Operations**: Temperature scaling, softmax, argmax, nucleus sampling, penalties

## Data Flow

```
Chat System
    ↓ provider.prompt(prompt, params)
Provider (KimiK2/Qwen3Coder)
    ↓ engine.coordinate_generation()
Engine (Orchestration)
    ↓ text_generator.generate()
TextGenerator
    ↓ SIMD operations
SIMD Sampling + Model Forward Passes
    ↓ AsyncStream<CandleStringChunk>
Back up the chain with streaming tokens
```

## Detailed Component Interactions

### Provider Implementation
```rust
impl CandleCompletionModel for KimiK2Provider {
    fn prompt(&self, prompt: CandlePrompt, params: &CandleCompletionParams) 
        -> AsyncStream<CandleCompletionChunk> {
        
        // Provider encapsulates generation through Engine orchestration
        let engine = Engine::new(engine_config)?;
        
        // Engine coordinates TextGenerator streaming
        engine.coordinate_generation(|| {
            self.text_generator.generate(
                prompt.text(), 
                params.max_tokens.unwrap_or(1000), 
                special_tokens
            )
        })
    }
}
```

### Engine Orchestration
```rust
impl Engine {
    pub fn coordinate_generation<F>(&self, generation_fn: F) 
        -> AsyncStream<CandleCompletionChunk>
    where F: FnOnce() -> AsyncStream<CandleStringChunk>
    {
        // Handle metrics, lifecycle, error handling
        self.request_count.fetch_add(1, Ordering::Relaxed);
        
        // Coordinate the actual generation
        let generation_stream = generation_fn();
        
        // Convert and manage streaming response
        self.manage_streaming_response(generation_stream)
    }
}
```

### TextGenerator Usage
```rust
impl KimiK2Provider {
    fn new() -> Self {
        // Load model via ProgressHub
        let model = ModelFactory::create_quantized_llama(gguf_path, config, device)?;
        let text_generator = TextGenerator::new(model, tokenizer, device, sampling_config);
        
        Self { text_generator, ... }
    }
}
```

## Key Architectural Principles

### 1. Encapsulation
- **Provider owns TextGenerator**: Model-specific logic is encapsulated
- **Engine provides utilities**: Orchestration services available to all providers
- **Clean interfaces**: Each component has a single, clear responsibility

### 2. No Routing Confusion
- **Chat already knows which provider**: No engine routing needed
- **Provider context**: We're already in the context of a specific model
- **Direct delegation**: Provider calls Engine utilities, not vice versa

### 3. Separation of Concerns
- **Provider**: Model loading, configuration, generation encapsulation
- **Engine**: Metrics, lifecycle, streaming coordination, error handling
- **TextGenerator**: SIMD inference, token generation, model forward passes
- **Chat**: Request initiation, memory integration, response handling

### 4. Performance Optimization
- **Model loading once**: Provider loads model during initialization
- **SIMD acceleration**: TextGenerator uses cyrup_simd for sampling
- **Streaming**: AsyncStream for real-time token delivery
- **Zero allocation**: Hot path optimizations throughout

## Benefits of This Architecture

1. **Clear Responsibility**: Each component has one job
2. **Reusable Engine**: All providers can use Engine orchestration utilities
3. **Encapsulated Generation**: Provider owns the complete generation process
4. **No Routing Complexity**: No hardcoded provider matching
5. **Performance**: SIMD acceleration with efficient streaming
6. **Maintainable**: Clean interfaces and separation of concerns
7. **Extensible**: New providers can easily use existing Engine utilities

## How It Works

The Candle generation pipeline operates through a clean orchestration flow:

1. **Chat System** receives user input and retrieves the configured provider from the agent role
2. **Provider** encapsulates all model-specific logic and owns the TextGenerator instance
3. **Engine** provides orchestration utilities (metrics, lifecycle, streaming coordination) that Providers use
4. **TextGenerator** performs SIMD-accelerated inference using the Candle ML framework
5. **Streaming Response** flows back through the chain as `AsyncStream<CandleCompletionChunk>`

### Generation Flow Example

```rust
// Chat initiates generation
let provider = self.get_completion_provider().unwrap();
let stream = provider.prompt(candle_prompt, &candle_params);

// Provider orchestrates through Engine
let engine = Engine::new(engine_config)?;
let coordinated_stream = engine.coordinate_generation(|| {
    self.text_generator.generate(prompt, max_tokens, special_tokens)
});

// TextGenerator performs SIMD inference
let token_stream = self.model.forward(&input, position)
    .and_then(|logits| self.sample_token_simd(&logits))
    .map(|token| CandleStringChunk(decoded_token));
```

This architecture provides optimal performance through SIMD acceleration, clean separation of concerns, and efficient streaming while maintaining extensibility for future model types.