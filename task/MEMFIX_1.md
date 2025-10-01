# MEMFIX_1: Add VectorSearch Integration to SurrealDBMemoryManager

## OBJECTIVE:
Integrate VectorSearch and EmbeddingModel fields into SurrealDBMemoryManager struct to enable internal embedding generation capability.

## SUBTASK1: Add VectorSearch fields to struct
**Location:** `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/core/manager/surreal.rs` (line ~283)

Add two new optional fields to the SurrealDBMemoryManager struct:
```rust
pub struct SurrealDBMemoryManager {
    db: Surreal<Any>,
    // ADD:
    vector_search: Option<Arc<VectorSearch>>,
    embedding_model: Option<Arc<dyn EmbeddingModel>>,
}
```

## SUBTASK2: Update imports
**Location:** Top of `surreal.rs` file

Add required imports:
```rust
use crate::memory::vector::vector_search::{VectorSearch, SearchOptions};
use crate::memory::vector::embedding_model::EmbeddingModel;
use crate::providers::bert_embedding::CandleBertEmbeddingProvider;
use std::sync::Arc;
use std::collections::HashMap;
```

## SUBTASK3: Create factory method with VectorSearch
**Location:** `surreal.rs` impl block for SurrealDBMemoryManager

Add new factory method that initializes with VectorSearch:
```rust
pub async fn with_embeddings(db: Surreal<Any>) -> Result<Self> {
    // Create BERT embedding model
    let embedding_model = Arc::new(
        CandleBertEmbeddingProvider::new().await?
    ) as Arc<dyn EmbeddingModel>;
    
    // Note: VectorSearch needs a VectorStore implementation
    // For now, we'll just store the embedding_model
    
    Ok(Self {
        db,
        vector_search: None, // Will be populated when VectorStore is available
        embedding_model: Some(embedding_model),
    })
}
```

## SUBTASK4: Update existing new() method
**Location:** `surreal.rs` existing `new()` method

Keep backward compatibility by making fields optional:
```rust
pub fn new(db: Surreal<Any>) -> Self {
    Self { 
        db,
        vector_search: None,
        embedding_model: None,
    }
}
```

## Definition of Done:
- SurrealDBMemoryManager struct has new optional fields for VectorSearch and EmbeddingModel
- Factory method `with_embeddings()` creates instance with BERT embedding model
- Original `new()` method maintains backward compatibility
- All imports are correctly added
- Code compiles without errors

## Research Notes:
- VectorSearch implementation exists at: `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/vector/vector_search.rs`
- VectorSearch requires a VectorStore trait implementation
- CandleBertEmbeddingProvider is the concrete embedding model at: `/Volumes/samsung_t9/paraphym/packages/candle/src/providers/bert_embedding.rs`
- EmbeddingModel trait is at: `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/vector/embedding_model.rs`

## Requirements:
- DO NOT write any unit tests
- DO NOT write any benchmarks  
- DO NOT create any documentation files
- Make only the minimal surgical changes required
- Use proper error handling with Result types, no unwrap() or expect() in src/