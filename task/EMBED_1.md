# EMBED_1: Implement Embedding Cache System

## OBJECTIVE

Implement the embedding dimension tracking system to enable proper cache validation and embedding compatibility checks.

## BACKGROUND

The embedding service has a dimension field marked as dead code. This field is critical for validating embedding compatibility and cache correctness. Research revealed that the `InMemoryEmbeddingCache` in `packages/candle/src/domain/embedding/service.rs` stores embeddings without dimension validation, allowing incompatible embeddings from different models to be mixed in cache.

## RESEARCH FINDINGS

### Existing Codebase Analysis

**Capability Traits**: The `TextEmbeddingCapable` trait in `packages/candle/src/capability/traits.rs` already provides `embedding_dimension()` and `supported_dimensions()` methods. Models like Jina-BERT (`packages/candle/src/capability/text_embedding/jina_bert.rs`) implement these with hardcoded dimensions (768 for Jina-BERT).

**Model Configuration**: `ModelConfig` in `packages/candle/src/core/model_config.rs` does not currently include embedding dimensions. Dimensions are determined by the specific model implementations.

**Cache Implementation**: The `InMemoryEmbeddingCache` used `HashMap<String, Vec<f32>>` without dimension metadata, allowing dimension mismatches.

**EmbeddingService Trait**: The trait exists in `packages/candle/src/domain/embedding/service.rs` but was not implemented anywhere in the codebase.

### Integration Points Discovered

- `EmbeddingPool` in `service.rs` already supports dimension-aware allocation
- Capability models provide dimension information via traits
- No existing usage of `EmbeddingService` trait found
- Cache operations lack dimension validation

## SUBTASK 1: Enable Dimension Tracking

**Location:** `packages/candle/src/domain/embedding/service.rs:101-102`

**Current State (Before):**
```rust
#[allow(dead_code)] // TODO: Implement in embedding cache system
dimension: usize,
```

**Implemented Changes:**
- Removed `#[allow(dead_code)]` attribute
- Dimension field now active for validation
- Integrated with cache operations for dimension checking

**Why:** Dimension tracking ensures embedding compatibility and prevents cache corruption.

## SUBTASK 2: Modify Cache Storage Structure

**Location:** `packages/candle/src/domain/embedding/service.rs`

**Changes Made:**
- Changed cache storage from `HashMap<String, Vec<f32>>` to `HashMap<String, (Vec<f32>, usize)>`
- Each cached entry now stores embedding vector + dimension metadata
- Enables dimension validation on retrieval and storage

**Code Example:**
```rust
pub struct InMemoryEmbeddingCache {
    cache: tokio::sync::RwLock<HashMap<String, (Vec<f32>, usize)>>,
    pool: EmbeddingPool,
    dimension: usize,
}
```

## SUBTASK 3: Add Dimension Validation

**Location:** `packages/candle/src/domain/embedding/service.rs`

**New Methods Added:**
```rust
/// Validate embedding dimensions match expected dimension
pub fn validate_dimensions(&self, embedding: &[f32]) -> bool {
    embedding.len() == self.dimension
}

/// Clear cache entries with invalid dimensions
pub async fn clear_invalid_entries(&self) -> usize
```

**Integration:** Validation called in `store()` and `get_cached()` operations.

## SUBTASK 4: Update Cache Operations

**Location:** `packages/candle/src/domain/embedding/service.rs`

**get_cached() Changes:**
- Now validates cached embedding dimension matches cache dimension
- Returns `None` for dimension mismatches (treated as cache miss)
- Prevents returning incompatible embeddings

**store() Changes:**
- Validates embedding dimension before storage
- Returns `Result<(), VectorStoreError>` for dimension mismatches
- Stores dimension metadata with embedding

**generate_deterministic() Changes:**
- Ensures generated embeddings match cache dimension
- Uses pool allocation with correct size

## SUBTASK 5: Implement EmbeddingService

**Location:** `packages/candle/src/domain/embedding/service.rs`

**New Implementation:**
```rust
pub struct EmbeddingServiceImpl<M: crate::capability::traits::TextEmbeddingCapable> {
    model: M,
    cache: InMemoryEmbeddingCache,
}

impl<M: crate::capability::traits::TextEmbeddingCapable> EmbeddingServiceImpl<M> {
    pub fn new(model: M) -> Self {
        let dimension = model.embedding_dimension();
        let cache = InMemoryEmbeddingCache::new(dimension);
        Self { model, cache }
    }
}
```

**Integration:** Extracts dimension from model configuration via capability trait, initializes cache with correct dimension, implements full `EmbeddingService` trait with caching.

## SUBTASK 6: Add Error Handling

**Location:** `packages/candle/src/domain/embedding/service.rs`

**Error Types:**
- `VectorStoreError::OperationFailed` for dimension mismatches
- Clear error messages indicating expected vs actual dimensions
- Dimension validation errors prevent cache corruption

## INTEGRATION POINTS

### With Capability System
- `EmbeddingServiceImpl` uses `TextEmbeddingCapable` models
- Extracts dimension via `model.embedding_dimension()`
- Compatible with all existing embedding providers

### With Model Configuration
- Dimensions extracted from model capability traits
- No changes needed to `ModelConfig` structure
- Works with existing model registry system

### With Existing Cache Usage
- `InMemoryEmbeddingCache` maintains same public API
- Dimension validation is internal implementation detail
- Backward compatible for dimension-matching operations

## DEFINITION OF DONE

- [x] No `#[allow(dead_code)]` attribute on dimension field
- [x] Dimension initialized from model configuration via capability traits
- [x] Validation ensures embedding vectors match expected dimension
- [x] Cache operations check dimension compatibility
- [x] Dimension mismatch returns clear error
- [x] Documentation explains dimension validation
- [x] EmbeddingService implementation provides integration point
- [x] NO test code written (separate team responsibility)
- [x] NO benchmark code written (separate team responsibility)

## IMPLEMENTATION NOTES

### Source Code Citations
- **Cache Structure**: `packages/candle/src/domain/embedding/service.rs:87-91`
- **Validation Methods**: `packages/candle/src/domain/embedding/service.rs:135-145`
- **Cache Operations**: `packages/candle/src/domain/embedding/service.rs:95-125`
- **EmbeddingService Impl**: `packages/candle/src/domain/embedding/service.rs:180-220`
- **Capability Traits**: `packages/candle/src/capability/traits.rs:64-105`
- **Model Dimensions**: `packages/candle/src/capability/text_embedding/jina_bert.rs:264`

### Dimension Compatibility
Different embedding models produce different dimension outputs:
- Jina-BERT: 768 dimensions
- Stella: 256, 768, 1024, 2048, 4096, 6144, 8192 dimensions
- GTE-Qwen: 1536 dimensions
- BERT: 384 dimensions

The cache system now prevents mixing these incompatible embeddings.

### Performance Impact
- Dimension validation adds minimal overhead (single length check)
- Cache storage increases by 8 bytes per entry (usize dimension metadata)
- No impact on embedding generation or model operations

## CONSTRAINTS

- DO NOT write unit tests (separate team handles testing)
- DO NOT write benchmarks (separate team handles benchmarks)
- Maintain backward compatibility with existing cache API
- Use existing error types from domain