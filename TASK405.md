# TASK405: Integration Testing and Validation

## Objective
Create comprehensive tests to validate the complete Candle generation pipeline works end-to-end according to the architecture defined in CANDLE_GENERATION_PIPELINE.md.

## Target Architecture Validation
Verify the complete flow: **Chat → Provider → Engine Orchestration → TextGenerator → SIMD Generation**

## Implementation

### 1. End-to-End Integration Test
**Create test file**: `/packages/candle/tests/integration_generation_pipeline.rs`
```rust
#[tokio::test]
async fn test_complete_generation_pipeline() {
    // 1. Create agent with default provider
    let agent = CandleFluentAi::agent_role("test-agent")
        .system_prompt("You are a helpful assistant")
        .build();
    
    // 2. Verify provider is configured
    let provider = agent.get_completion_provider();
    assert!(matches!(provider, CandleCompletionProviderType::KimiK2(_)));
    
    // 3. Test chat generation
    let chat_impl = CandleAgentRoleImpl::new("test");
    let response = chat_impl.chat("Hello, how are you?").await;
    
    // 4. Verify response is generated (not error)
    assert!(response.is_ok());
    assert!(!response.unwrap().is_empty());
}
```

### 2. Provider Orchestration Test
**Test provider uses Engine orchestration**:
```rust
#[test]
fn test_provider_uses_engine_orchestration() {
    let provider = CandleKimiK2Provider::new().unwrap();
    let prompt = CandlePrompt::new("Test prompt");
    let params = CandleCompletionParams::default();
    
    // Call provider.prompt() and verify it uses Engine orchestration
    let stream = provider.prompt(prompt, &params);
    
    // Verify stream produces CandleCompletionChunk
    let chunks: Vec<_> = stream.collect();
    assert!(!chunks.is_empty());
    
    // Verify chunks are proper completion chunks
    for chunk in chunks {
        match chunk {
            CandleCompletionChunk::Text(_) | 
            CandleCompletionChunk::Complete { .. } => {}, // Valid
            CandleCompletionChunk::Error(e) => panic!("Unexpected error: {}", e),
            _ => panic!("Unexpected chunk type"),
        }
    }
}
```

### 3. Engine Metrics Test
**Verify Engine orchestration updates metrics**:
```rust
#[test]
fn test_engine_metrics_tracking() {
    let engine = Engine::new(EngineConfig::default()).unwrap();
    let initial_count = engine.request_count();
    
    // Simulate generation through orchestration
    let _stream = engine.coordinate_generation(|| {
        AsyncStream::with_channel(|sender| {
            let _ = sender.send(CandleStringChunk("test".to_string()));
        })
    });
    
    // Verify metrics updated
    assert_eq!(engine.request_count(), initial_count + 1);
}
```

### 4. TextGenerator Integration Test
**Verify TextGenerator is properly integrated**:
```rust
#[test]
fn test_text_generator_integration() {
    // Create TextGenerator with test model
    let config = SamplingConfig::new(0.7);
    let device = Device::Cpu;
    
    // Mock model for testing
    let model = create_test_model();
    let tokenizer = create_test_tokenizer();
    
    let text_generator = TextGenerator::new(
        Box::new(model),
        tokenizer,
        device,
        config
    );
    
    // Test generation
    let stream = text_generator.generate(
        "Hello".to_string(),
        10,
        SpecialTokens::with_eos(2)
    );
    
    // Verify SIMD operations are used
    let chunks: Vec<_> = stream.collect();
    assert!(!chunks.is_empty());
}
```

### 5. Memory Integration Test
**Verify memory system still works with new architecture**:
```rust
#[tokio::test]
async fn test_memory_integration_preserved() {
    let agent = CandleFluentAi::agent_role("memory-test")
        .with_memory_manager(create_test_memory_manager())
        .build();
    
    // Test memory-enhanced chat
    let response1 = agent.chat("My name is Alice").await.unwrap();
    let response2 = agent.chat("What is my name?").await.unwrap();
    
    // Verify memory is working (response should reference Alice)
    assert!(response2.to_lowercase().contains("alice"));
}
```

### 6. Performance Benchmark
**Verify SIMD acceleration is working**:
```rust
#[test]
fn benchmark_simd_performance() {
    let provider = CandleKimiK2Provider::new().unwrap();
    
    let start = std::time::Instant::now();
    
    // Generate multiple completions
    for _ in 0..10 {
        let stream = provider.prompt(
            CandlePrompt::new("Test"),
            &CandleCompletionParams::default()
        );
        let _: Vec<_> = stream.collect();
    }
    
    let elapsed = start.elapsed();
    
    // Verify reasonable performance (adjust threshold as needed)
    assert!(elapsed.as_secs() < 30, "Generation too slow: {:?}", elapsed);
}
```

### 7. Error Handling Test
**Verify proper error propagation**:
```rust
#[test]
fn test_error_handling() {
    // Test with invalid model path
    let result = CandleKimiK2Provider::with_config_sync(
        "/invalid/path".to_string(),
        CandleKimiK2Config::default()
    );
    
    // Should handle error gracefully
    assert!(result.is_err());
    
    // Test error propagation through chat system
    // (Implementation depends on specific error handling approach)
}
```

## Success Criteria
- All integration tests pass
- End-to-end generation pipeline works correctly
- Memory integration is preserved
- SIMD acceleration is functioning
- Error handling works properly
- Performance is acceptable
- Metrics tracking is working

## Files Created
- `/packages/candle/tests/integration_generation_pipeline.rs`
- `/packages/candle/tests/test_helpers.rs` (for test utilities)

## Dependencies
- TASK400, TASK401, TASK402, TASK403, TASK404 (all implementation tasks must be complete)
