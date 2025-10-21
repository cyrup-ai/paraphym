# EMBED_1: Implement Embedding Cache System

## OBJECTIVE

Implement the embedding dimension tracking system to enable proper cache validation and embedding compatibility checks.

## BACKGROUND

The embedding service has a dimension field marked as dead code. This field is critical for validating embedding compatibility and cache correctness.

## SUBTASK 1: Enable Dimension Tracking

**Location:** `packages/candle/src/domain/embedding/service.rs:101-102`

**Current State:**
```rust
#[allow(dead_code)] // TODO: Implement in embedding cache system
dimension: usize,
```

**Required Changes:**
- Remove `#[allow(dead_code)]` attribute
- Store embedding dimension from model configuration
- Initialize dimension in `EmbeddingService::new()`
- Validate dimension matches model output on first embedding generation

**Why:** Dimension tracking ensures embedding compatibility and prevents cache corruption.

## SUBTASK 2: Add Dimension Validation

**Location:** `packages/candle/src/domain/embedding/service.rs`

**Required Changes:**
- Add `validate_dimensions()` method to check embedding vector sizes
- Validate cached embeddings match service dimension
- Add dimension check before cache insertion
- Return error if dimension mismatch occurs
- Document dimension validation in service docs

**Why:** Prevents mixing incompatible embeddings in cache.

## SUBTASK 3: Integrate with Cache Operations

**Location:** `packages/candle/src/domain/embedding/service.rs`

**Required Changes:**
- Check dimension in cache lookup operations
- Invalidate cached entries with wrong dimensions
- Add dimension metadata to cache entries
- Use dimension in cache key generation to separate different models

**Why:** Cache must be dimension-aware to prevent incorrect retrievals.

## SUBTASK 4: Add Dimension Configuration

**Location:** Integration with embedding pool and model config

**Required Changes:**
- Extract dimension from model configuration
- Pass dimension through to EmbeddingService
- Add dimension to EmbeddingPool metadata
- Expose dimension in service API for monitoring

**Why:** Dimension must be configurable per model type.

## DEFINITION OF DONE

- [ ] No `#[allow(dead_code)]` attribute on dimension field
- [ ] Dimension initialized from model configuration
- [ ] Validation ensures embedding vectors match expected dimension
- [ ] Cache operations check dimension compatibility
- [ ] Dimension mismatch returns clear error
- [ ] Documentation explains dimension validation
- [ ] NO test code written (separate team responsibility)
- [ ] NO benchmark code written (separate team responsibility)

## RESEARCH NOTES

### Embedding Dimensions by Model Type
- Text embeddings: typically 384, 768, 1024, or 1536 dimensions
- Different models produce different dimensions
- Mixing dimensions causes incorrect similarity calculations

### Integration Points
- `EmbeddingPool` in same file
- Model configuration in `packages/candle/src/core/model_config.rs`
- Cache storage in `cache: tokio::sync::RwLock<HashMap<String, Vec<f32>>>`

### Validation Pattern
- Check `embedding.len() == self.dimension` after generation
- Store dimension with cached embeddings
- Compare dimensions before cache retrieval

## CONSTRAINTS

- DO NOT write unit tests (separate team handles testing)
- DO NOT write benchmarks (separate team handles benchmarks)
- Maintain backward compatibility with existing cache
- Use existing error types from domain
