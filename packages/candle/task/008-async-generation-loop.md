# Task 008: Make entire generation loop properly async

## Problem
The ENTIRE generation loop in `generator.rs:90-255` runs synchronously inside `async_stream::spawn_stream()`. This blocks the async runtime for the entire generation (seconds to minutes).

## Current Code
```rust
pub fn generate(...) -> impl Stream<...> {
    async_stream::spawn_stream(move |tx| async move {
        // ALL OF THIS IS SYNC:
        // - tokenizer.encode()
        // - model.forward() x2048
        // - logits.to_vec1() x2048
        // - sample_token() x2048  
        // - tokenizer.decode() x2048
        for _index in 1..max_tokens {
            // ... all sync operations ...
        }
    })
}
```

## Impact
**CRITICAL** - Blocks async runtime for 30-60 seconds per generation

## Solution Strategy

### Phase 1: Convert individual operations to async (Tasks 001-004)
First make all the building blocks async:
- model.forward() → async
- tokenizer ops → async
- tensor ops → async  
- sample_token() → async

### Phase 2: Make generate() properly async
```rust
pub async fn generate(...) -> impl Stream<...> {
    async_stream::spawn_stream(move |tx| async move {
        // Now all operations are truly async
        let tokens = self.tokenizer.encode_async(prompt).await?;
        
        for _index in 1..max_tokens {
            let input = Tensor::new_async(&[next_token], &self.device).await?;
            let logits = self.model.forward(&input, position).await?;
            let logits_vec = logits.to_vec1_async().await?;
            next_token = self.sample_token(&logits_vec, &all_tokens).await?;
            let token_str = self.tokenizer.decode_async(&[next_token]).await?;
            tx.send(CandleStringChunk(token_str))?;
        }
    })
}
```

## Files to Change
- `src/core/generation/generator.rs:84-256` - Entire generate method

## Dependencies
**MUST COMPLETE FIRST:**
- Task 001 (async model.forward)
- Task 002 (async tokenizer)
- Task 003 (async tensor ops)
- Task 004 (async SIMD sampling)

## Estimated Effort
4 hours (after dependencies complete)

## Testing
After this task, the fluent_builder example should:
1. Not hang on model loading
2. Not block async runtime during generation
3. Stream tokens smoothly
4. Support concurrent requests without blocking
