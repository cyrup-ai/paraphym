# REASON_1: Implement Semantic Coherence

## OBJECTIVE
Replace placeholder word overlap coherence function with proper semantic similarity using embeddings.

## LOCATION
`packages/sweetmcp/plugins/reasoner/src/reasoner/strategies/mcts_002_alpha.rs:190`

## SUBTASK 1: Add embedding model support
- Determine if embedding model already exists in codebase
- If not, add lightweight embedding model dependency
- Consider using existing paraphym_candle embedding infrastructure

## SUBTASK 2: Implement semantic coherence
```rust
async fn thought_coherence(&self, thought1: &str, thought2: &str) -> f64 {
    // Use embedding model for semantic similarity
    let emb1 = self.embedding_model.embed(thought1).await?;
    let emb2 = self.embedding_model.embed(thought2).await?;
    
    // Cosine similarity
    cosine_similarity(&emb1, &emb2)
}
```

## SUBTASK 3: Add embedding cache
- Implement caching to avoid re-computing embeddings
- Use LRU cache or similar for recent thoughts
- Consider memory constraints

## SUBTASK 4: Replace placeholder logic
- Remove simple word overlap implementation
- Remove "placeholder" comment
- Wire up new semantic coherence function

## DEFINITION OF DONE
- Semantic similarity using embeddings
- Coherence based on meaning, not word overlap
- Caching implemented for performance
- Code compiles without warnings

## RESEARCH NOTES
- Cosine similarity implementation (may exist in paraphym_simd)
- Embedding models in paraphym_candle
- LRU cache patterns in Rust

## CONSTRAINTS
- Do NOT write unit tests
- Do NOT write integration tests
- Do NOT write benchmarks
- Focus solely on src modification
