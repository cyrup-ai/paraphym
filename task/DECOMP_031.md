# DECOMP_031: Decompose `retrieval.rs`

**File:** `packages/candle/src/memory/core/ops/retrieval.rs`  
**Current Size:** 735 lines  
**Module Area:** memory / core / ops

## OBJECTIVE

Decompose the monolithic `retrieval.rs` (735 lines) into smaller, focused, maintainable modules while preserving all existing functionality and public API.

## CORE UNDERSTANDING

The `retrieval.rs` file implements **memory retrieval strategies** with multiple approaches to finding relevant memories:
- **Vector similarity search** using embeddings
- **Semantic search** through text understanding
- **Temporal proximity** based on time decay
- **Hybrid strategies** combining multiple approaches with weighted scoring
- **Cognitive processing** for intelligent result filtering

The file is used by [`domain/completion/request.rs`](../packages/candle/src/domain/completion/request.rs#L15) which imports `RetrievalResult` to attach retrieved memories to completion requests.

### Current Public API (Must Be Preserved)

From [`memory/core/ops/mod.rs`](../packages/candle/src/memory/core/ops/mod.rs):
```rust
pub mod retrieval;
pub use retrieval::*;
```

All types are currently public and must remain accessible through the same import paths.

## EXISTING CODE STRUCTURE ANALYSIS

### Section 1: Core Types & Traits (Lines 1-85)
**Size:** ~85 lines  
**Purpose:** Foundation types used across all retrieval strategies

```rust
// Enums and result types
pub enum RetrievalMethod {
    VectorSimilarity,
    Semantic,
    Temporal,
    Keyword,
    Hybrid,
}

pub struct RetrievalResult {
    pub id: String,
    pub score: f32,
    pub method: RetrievalMethod,
    pub metadata: HashMap<String, serde_json::Value>,
}

// Future wrapper for async operations
pub struct PendingRetrieval {
    rx: oneshot::Receiver<Result<Vec<RetrievalResult>>>,
}

// Strategy trait - ALL implementations must implement this
pub trait RetrievalStrategy: Send + Sync {
    fn retrieve(&self, query: String, limit: usize, filter: Option<MemoryFilter>) -> PendingRetrieval;
    fn name(&self) -> &str;
}
```

**Dependencies:**
- `std::collections::HashMap`
- `std::future::Future`
- `futures_util::stream::StreamExt`
- `serde::{Deserialize, Serialize}`
- `tokio::sync::oneshot`
- `crate::memory::filter::MemoryFilter`
- `crate::memory::utils::Result`
- `crate::memory::vector::VectorStore`

### Section 2: HybridRetrieval Strategy (Lines 86-297)
**Size:** ~212 lines  
**Purpose:** Combines multiple retrieval strategies with weighted scoring and cognitive filtering

```rust
pub struct HybridRetrieval<V: VectorStore> {
    vector_store: V,
    strategies: Arc<Vec<Arc<dyn RetrievalStrategy>>>,
    weights: Arc<HashMap<String, f32>>,
    cognitive_processor: Arc<CognitiveProcessor>,  // KEY: Integrates cognitive decision making
}

impl<V: VectorStore> HybridRetrieval<V> {
    pub fn new(vector_store: V) -> Self { /* ... */ }
    pub fn add_strategy(mut self, strategy: Arc<dyn RetrievalStrategy>) -> Self { /* ... */ }
    pub fn set_weight(mut self, strategy_name: &str, weight: f32) -> Self { /* ... */ }
    
    // CRITICAL: ~120 lines of cognitive processing logic
    pub async fn get_vector_similarity(&self, query_vector: Vec<f32>, limit: usize) -> Result<Vec<RetrievalResult>> {
        // 1. Process query with CognitiveProcessor
        // 2. Evaluate decision outcome: Accept/Defer/Reject
        // 3. Weight results by cognitive confidence
        // 4. Add cognitive metadata to results
    }
}

impl<V: VectorStore + Send + Sync + 'static> RetrievalStrategy for HybridRetrieval<V> {
    // Combines results from multiple strategies using weighted scores
}
```

**Key Insight:** This is the most complex strategy and integrates with the **Cognitive Processing System** at [`domain/memory/cognitive/types/processor.rs`](../packages/candle/src/domain/memory/cognitive/types/processor.rs).

**Dependencies:**
- `Arc<CognitiveProcessor>` from `crate::domain::memory::cognitive::types`
- `DecisionOutcome` enum for result filtering

### Section 3: SemanticRetrieval Strategy (Lines 299-374)
**Size:** ~76 lines  
**Purpose:** Pure semantic search using vector embeddings

```rust
pub struct SemanticRetrieval<V: VectorStore> {
    vector_store: Arc<V>,
}

impl<V: VectorStore + Send + Sync + 'static> RetrievalStrategy for SemanticRetrieval<V> {
    fn retrieve(&self, query: String, limit: usize, filter: Option<MemoryFilter>) -> PendingRetrieval {
        // 1. Generate query embedding via vector_store.embed()
        // 2. Search vector store
        // 3. Collect stream results
        // 4. Convert to RetrievalResult
    }
}
```

**Pattern Used:** Spawns async task with oneshot channel, matching the pattern in [`storage.rs`](../packages/candle/src/memory/core/ops/storage.rs) for `PendingStore`, `PendingRetrieve`, etc.

### Section 4: TemporalRetrieval Strategy (Lines 376-604)
**Size:** ~229 lines  
**Purpose:** Time-based retrieval with exponential decay scoring

```rust
pub struct TemporalRetrieval {
    time_decay_factor: f32,
    memory_manager: Arc<dyn crate::memory::MemoryManager>,  // NOTE: Different from VectorStore
}

impl RetrievalStrategy for TemporalRetrieval {
    // COMPLEX LOGIC:
    // 1. Apply time range filter (default: last 30 days)
    // 2. Search by content if query provided
    // 3. Fall back to type-based queries
    // 4. Apply exponential decay: exp(-(age_hours * time_decay))
    // 5. Combine temporal score (70%) with relevance score (30%)
    // 6. Return top results sorted by combined score
}
```

**Key Difference:** Uses `MemoryManager` instead of `VectorStore`, integrating with the full memory system including episodic/semantic/procedural memory types.

### Section 5: RetrievalManager (Lines 606-735)
**Size:** ~130 lines  
**Purpose:** Orchestrates multiple strategies and provides unified API

```rust
pub struct RetrievalManager<V: VectorStore> {
    strategies: HashMap<String, Arc<dyn RetrievalStrategy>>,
    default_strategy: String,
    vector_store: V,
}

impl<V: VectorStore + Clone + Send + Sync + 'static> RetrievalManager<V> {
    pub fn new(vector_store: V) -> Self { /* Initializes with semantic strategy */ }
    pub fn add_temporal_strategy(&mut self, memory_manager: Arc<dyn MemoryManager>, time_decay: f32) { /* ... */ }
    pub fn set_default_strategy(&mut self, strategy_name: String) { /* ... */ }
    pub fn add_strategy(&mut self, name: String, strategy: Arc<dyn RetrievalStrategy>) { /* ... */ }
    pub async fn direct_vector_search(&self, query_vector: Vec<f32>, limit: usize) -> Result<Vec<VectorSearchResult>> { /* ... */ }
    pub async fn retrieve(&self, query: &str, strategy_name: Option<&str>, limit: usize, filter: Option<&MemoryFilter>) -> Result<Vec<RetrievalResult>> { /* ... */ }
    pub async fn multi_strategy_retrieve(&self, query: &str, strategy_names: Vec<&str>, limit: usize, filter: Option<&MemoryFilter>) -> Result<Vec<RetrievalResult>> { /* ... */ }
}
```

## DECOMPOSITION PLAN

Following the successful pattern used in [`memory/cognitive/types/`](../packages/candle/src/domain/memory/cognitive/types/) and [`capability/`](../packages/candle/src/capability/) modules, create a **`retrieval/`** subdirectory:

```
memory/core/ops/
├── retrieval/
│   ├── mod.rs                    # Module aggregator + re-exports
│   ├── types.rs                  # Core types: RetrievalMethod, RetrievalResult, PendingRetrieval
│   ├── strategy.rs               # RetrievalStrategy trait definition
│   ├── semantic.rs               # SemanticRetrieval implementation
│   ├── temporal.rs               # TemporalRetrieval implementation
│   ├── hybrid.rs                 # HybridRetrieval implementation
│   └── manager.rs                # RetrievalManager orchestration
└── retrieval.rs                  # DEPRECATED - points to retrieval/mod.rs
```

### Detailed Module Breakdown

#### 1. `retrieval/types.rs` (~90 lines)
**Responsibility:** Foundation types and async wrappers

```rust
//! Core retrieval types and result structures

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use crate::memory::utils::Result;

/// Retrieval method used to find the memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetrievalMethod {
    VectorSimilarity,
    Semantic,
    Temporal,
    Keyword,
    Hybrid,
}

/// Result from memory retrieval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalResult {
    pub id: String,
    pub score: f32,
    pub method: RetrievalMethod,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// A pending retrieval operation
pub struct PendingRetrieval {
    rx: oneshot::Receiver<Result<Vec<RetrievalResult>>>,
}

impl PendingRetrieval {
    pub fn new(rx: oneshot::Receiver<Result<Vec<RetrievalResult>>>) -> Self {
        Self { rx }
    }
}

impl Future for PendingRetrieval {
    type Output = Result<Vec<RetrievalResult>>;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.rx).poll(cx) {
            Poll::Ready(Ok(result)) => Poll::Ready(result),
            Poll::Ready(Err(_)) => Poll::Ready(Err(crate::memory::utils::error::Error::Internal(
                "Retrieval task failed".to_string(),
            ))),
            Poll::Pending => Poll::Pending,
        }
    }
}
```

**Public exports:**
- `RetrievalMethod`
- `RetrievalResult`
- `PendingRetrieval`

**No external dependencies** besides standard library and crate internals.

#### 2. `retrieval/strategy.rs` (~25 lines)
**Responsibility:** Define the strategy trait

```rust
//! Retrieval strategy trait definition

use super::{PendingRetrieval, types::RetrievalResult};
use crate::memory::filter::MemoryFilter;

/// Memory retrieval strategy trait
pub trait RetrievalStrategy: Send + Sync {
    /// Retrieve memories based on the strategy
    fn retrieve(
        &self,
        query: String,
        limit: usize,
        filter: Option<MemoryFilter>,
    ) -> PendingRetrieval;

    /// Get strategy name
    fn name(&self) -> &str;
}
```

**Public exports:**
- `RetrievalStrategy` trait

**Pattern:** Mirrors the `VectorStore` trait pattern from [`memory/vector/mod.rs`](../packages/candle/src/memory/vector/mod.rs#L121).

#### 3. `retrieval/semantic.rs` (~80 lines)
**Responsibility:** Semantic vector search implementation

```rust
//! Semantic similarity retrieval using vector embeddings

use std::sync::Arc;
use futures_util::stream::StreamExt;
use crate::memory::filter::MemoryFilter;
use crate::memory::vector::VectorStore;
use crate::memory::utils::Result;
use super::{
    strategy::RetrievalStrategy,
    types::{PendingRetrieval, RetrievalMethod, RetrievalResult},
};
use std::collections::HashMap;
use tokio::sync::oneshot;

pub struct SemanticRetrieval<V: VectorStore> {
    vector_store: Arc<V>,
}

impl<V: VectorStore> SemanticRetrieval<V> {
    pub fn new(vector_store: V) -> Self {
        Self {
            vector_store: Arc::new(vector_store),
        }
    }
}

impl<V: VectorStore + Send + Sync + 'static> RetrievalStrategy for SemanticRetrieval<V> {
    fn retrieve(
        &self,
        query: String,
        limit: usize,
        filter: Option<MemoryFilter>,
    ) -> PendingRetrieval {
        let (tx, rx) = oneshot::channel();
        let vector_store = self.vector_store.clone();

        tokio::spawn(async move {
            let result: Result<Vec<RetrievalResult>> = (async {
                let query_embedding = vector_store.embed(query).await?;
                let search_stream = vector_store.search(query_embedding, limit, filter);
                let results: Vec<_> = search_stream.collect().await;
                
                let retrieval_results = results
                    .into_iter()
                    .map(|r| RetrievalResult {
                        id: r.id,
                        score: r.score,
                        method: RetrievalMethod::Semantic,
                        metadata: HashMap::new(),
                    })
                    .collect();

                Ok(retrieval_results)
            }).await;

            let _ = tx.send(result);
        });

        PendingRetrieval::new(rx)
    }

    fn name(&self) -> &str {
        "semantic"
    }
}
```

**Public exports:**
- `SemanticRetrieval<V: VectorStore>`

**Dependencies:**
- `VectorStore` trait from `crate::memory::vector`

#### 4. `retrieval/temporal.rs` (~240 lines)
**Responsibility:** Time-based retrieval with decay scoring

```rust
//! Temporal proximity retrieval with production-ready database integration

use std::sync::Arc;
use futures_util::stream::StreamExt;
use crate::memory::filter::MemoryFilter;
use crate::memory::utils::Result;
use super::{
    strategy::RetrievalStrategy,
    types::{PendingRetrieval, RetrievalMethod, RetrievalResult},
};
use std::collections::HashMap;
use tokio::sync::oneshot;

pub struct TemporalRetrieval {
    time_decay_factor: f32,
    memory_manager: Arc<dyn crate::memory::MemoryManager>,
}

impl TemporalRetrieval {
    pub fn new(
        time_decay_factor: f32,
        memory_manager: Arc<dyn crate::memory::MemoryManager>,
    ) -> Self {
        Self {
            time_decay_factor,
            memory_manager,
        }
    }
}

impl RetrievalStrategy for TemporalRetrieval {
    fn retrieve(
        &self,
        query: String,
        limit: usize,
        filter: Option<MemoryFilter>,
    ) -> PendingRetrieval {
        let (tx, rx) = oneshot::channel();
        let time_decay = self.time_decay_factor;
        let memory_manager = self.memory_manager.clone();

        tokio::spawn(async move {
            let result: Result<Vec<RetrievalResult>> = (async {
                // [PRESERVE EXACT LOGIC FROM LINES 420-600]
                // 1. Create temporal filter with 30-day default window
                // 2. Query by content if query provided
                // 3. Fall back to type-based queries
                // 4. Apply exponential decay scoring
                // 5. Combine temporal (70%) + relevance (30%) scores
                // 6. Sort and truncate to limit
                
                // ... [Copy full implementation from original file] ...
            }).await;

            let _ = tx.send(result);
        });

        PendingRetrieval::new(rx)
    }

    fn name(&self) -> &str {
        "temporal"
    }
}
```

**Public exports:**
- `TemporalRetrieval`

**Dependencies:**
- `MemoryManager` trait from `crate::memory`
- `chrono::Utc` for time calculations

#### 5. `retrieval/hybrid.rs` (~220 lines)
**Responsibility:** Multi-strategy combination with cognitive filtering

```rust
//! Hybrid retrieval strategy combining multiple approaches

use std::sync::Arc;
use std::collections::HashMap;
use futures_util::stream::StreamExt;
use crate::domain::memory::cognitive::types::{
    CognitiveProcessor, CognitiveProcessorConfig, DecisionOutcome,
};
use crate::memory::filter::MemoryFilter;
use crate::memory::vector::VectorStore;
use crate::memory::utils::Result;
use super::{
    strategy::RetrievalStrategy,
    types::{PendingRetrieval, RetrievalMethod, RetrievalResult},
};
use tokio::sync::oneshot;

pub struct HybridRetrieval<V: VectorStore> {
    vector_store: V,
    strategies: Arc<Vec<Arc<dyn RetrievalStrategy>>>,
    weights: Arc<HashMap<String, f32>>,
    cognitive_processor: Arc<CognitiveProcessor>,
}

impl<V: VectorStore> HybridRetrieval<V> {
    pub fn new(vector_store: V) -> Self {
        let mut weights = HashMap::new();
        weights.insert("semantic".to_string(), 0.6);
        weights.insert("keyword".to_string(), 0.2);
        weights.insert("temporal".to_string(), 0.2);

        let processor_config = CognitiveProcessorConfig {
            batch_size: 32,
            decision_threshold: 0.7,
            learning_rate: 0.01,
            max_iterations: 1000,
        };

        Self {
            vector_store,
            strategies: Arc::new(Vec::new()),
            weights: Arc::new(weights),
            cognitive_processor: Arc::new(CognitiveProcessor::new(processor_config)),
        }
    }

    pub fn add_strategy(mut self, strategy: Arc<dyn RetrievalStrategy>) -> Self {
        Arc::make_mut(&mut self.strategies).push(strategy);
        self
    }

    pub fn set_weight(mut self, strategy_name: &str, weight: f32) -> Self {
        Arc::make_mut(&mut self.weights).insert(strategy_name.to_string(), weight);
        self
    }

    pub async fn get_vector_similarity(
        &self,
        query_vector: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<RetrievalResult>> {
        // [PRESERVE EXACT COGNITIVE PROCESSING LOGIC FROM LINES 140-245]
        // This is CRITICAL functionality integrating with CognitiveProcessor
        
        let filter = crate::memory::filter::MemoryFilter::new();
        let query_decision = self.cognitive_processor.process(&query_vector);
        let search_stream = self.vector_store.search(query_vector, limit, Some(filter));
        let results: Vec<_> = search_stream.collect().await;

        let retrieval_results: Vec<RetrievalResult> = match query_decision {
            Ok(decision) => {
                match decision.outcome {
                    DecisionOutcome::Accept => {
                        // Weight by cognitive confidence
                        // ... [Copy exact logic] ...
                    }
                    DecisionOutcome::Defer => {
                        // Reduce scores for uncertain queries
                        // ... [Copy exact logic] ...
                    }
                    DecisionOutcome::Reject | DecisionOutcome::RequestInfo => {
                        // Filter all results
                        Vec::new()
                    }
                }
            }
            Err(e) => {
                // Fallback on error
                // ... [Copy exact logic] ...
            }
        };

        Ok(retrieval_results)
    }
}

impl<V: VectorStore + Send + Sync + 'static> RetrievalStrategy for HybridRetrieval<V> {
    fn retrieve(
        &self,
        query: String,
        limit: usize,
        filter: Option<MemoryFilter>,
    ) -> PendingRetrieval {
        // [PRESERVE EXACT LOGIC FROM LINES 248-297]
        // Combines multiple strategies with weighted scoring
        // ... [Copy full implementation] ...
    }

    fn name(&self) -> &str {
        "hybrid"
    }
}
```

**Public exports:**
- `HybridRetrieval<V: VectorStore>`

**Critical Dependencies:**
- `CognitiveProcessor` from `crate::domain::memory::cognitive::types`
- Integration point with AI decision-making system

#### 6. `retrieval/manager.rs` (~140 lines)
**Responsibility:** Strategy orchestration and unified API

```rust
//! Memory retrieval manager orchestrating multiple strategies

use std::sync::Arc;
use std::collections::HashMap;
use crate::memory::filter::MemoryFilter;
use crate::memory::vector::{VectorStore, VectorSearchResult};
use crate::memory::utils::Result;
use super::{
    strategy::RetrievalStrategy,
    types::RetrievalResult,
    semantic::SemanticRetrieval,
    temporal::TemporalRetrieval,
};
use futures_util::stream::StreamExt;

pub struct RetrievalManager<V: VectorStore> {
    strategies: HashMap<String, Arc<dyn RetrievalStrategy>>,
    default_strategy: String,
    vector_store: V,
}

impl<V: VectorStore + Clone + Send + Sync + 'static> RetrievalManager<V> {
    pub fn new(vector_store: V) -> Self {
        let mut strategies: HashMap<String, Arc<dyn RetrievalStrategy>> = HashMap::new();
        
        strategies.insert(
            "semantic".to_string(),
            Arc::new(SemanticRetrieval::new(vector_store.clone())),
        );

        Self {
            strategies,
            default_strategy: "semantic".to_string(),
            vector_store,
        }
    }

    pub fn add_temporal_strategy(
        &mut self,
        memory_manager: Arc<dyn crate::memory::MemoryManager>,
        time_decay_factor: f32,
    ) {
        self.strategies.insert(
            "temporal".to_string(),
            Arc::new(TemporalRetrieval::new(time_decay_factor, memory_manager)),
        );
    }

    pub fn set_default_strategy(&mut self, strategy_name: String) {
        self.default_strategy = strategy_name;
    }

    pub fn add_strategy(&mut self, name: String, strategy: Arc<dyn RetrievalStrategy>) {
        self.strategies.insert(name, strategy);
    }

    pub async fn direct_vector_search(
        &self,
        query_vector: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<VectorSearchResult>> {
        let filter = crate::memory::filter::MemoryFilter::new();
        let search_stream = self.vector_store.search(query_vector, limit, Some(filter));
        let results: Vec<_> = search_stream.collect().await;
        Ok(results)
    }

    pub async fn retrieve(
        &self,
        query: &str,
        strategy_name: Option<&str>,
        limit: usize,
        filter: Option<&MemoryFilter>,
    ) -> Result<Vec<RetrievalResult>> {
        let strategy_name = strategy_name.unwrap_or(&self.default_strategy);

        if let Some(strategy) = self.strategies.get(strategy_name) {
            strategy.retrieve(query.to_string(), limit, filter.cloned()).await
        } else {
            Err(crate::memory::utils::error::Error::InvalidInput(format!(
                "Unknown retrieval strategy: {strategy_name}"
            )))
        }
    }

    pub async fn multi_strategy_retrieve(
        &self,
        query: &str,
        strategy_names: Vec<&str>,
        limit: usize,
        filter: Option<&MemoryFilter>,
    ) -> Result<Vec<RetrievalResult>> {
        // [PRESERVE EXACT LOGIC FROM LINES 690-735]
        // Deduplication and score merging logic
        // ... [Copy full implementation] ...
    }
}
```

**Public exports:**
- `RetrievalManager<V: VectorStore>`

#### 7. `retrieval/mod.rs` (~40 lines)
**Responsibility:** Module aggregation and re-exports

```rust
//! Memory retrieval strategies and algorithms
//!
//! This module provides multiple strategies for retrieving relevant memories:
//! - **Semantic**: Vector similarity using embeddings
//! - **Temporal**: Time-based with exponential decay
//! - **Hybrid**: Combines multiple strategies with cognitive filtering
//! - **Manager**: Orchestrates strategies with unified API

// Submodules
pub mod types;
pub mod strategy;
pub mod semantic;
pub mod temporal;
pub mod hybrid;
pub mod manager;

// Re-export all public types to maintain API compatibility
pub use types::{RetrievalMethod, RetrievalResult, PendingRetrieval};
pub use strategy::RetrievalStrategy;
pub use semantic::SemanticRetrieval;
pub use temporal::TemporalRetrieval;
pub use hybrid::HybridRetrieval;
pub use manager::RetrievalManager;
```

**Key Point:** This preserves the exact public API since `ops/mod.rs` does `pub use retrieval::*;`

## IMPLEMENTATION STEPS

### STEP 1: Create the directory structure

```bash
mkdir -p packages/candle/src/memory/core/ops/retrieval
```

### STEP 2: Create module files in order

1. **`retrieval/types.rs`** - Foundation types (no internal dependencies)
2. **`retrieval/strategy.rs`** - Trait definition (depends on types)
3. **`retrieval/semantic.rs`** - Simplest strategy implementation
4. **`retrieval/temporal.rs`** - Independent complex strategy
5. **`retrieval/hybrid.rs`** - Most complex, depends on cognitive system
6. **`retrieval/manager.rs`** - Orchestration layer
7. **`retrieval/mod.rs`** - Module aggregator

### STEP 3: Move code sections

For each file:
1. Copy the exact implementation from `retrieval.rs`
2. Add appropriate imports at the top
3. Ensure all `pub` modifiers are preserved
4. Update internal references to use `super::` or `crate::`

### STEP 4: Update `retrieval.rs` to point to new structure

**Option A:** Replace entire file with re-export:
```rust
//! Memory retrieval strategies - see retrieval/ submodule
pub use self::retrieval::*;

mod retrieval;
```

**Option B:** Delete `retrieval.rs` and update `ops/mod.rs`:
```rust
pub mod retrieval;  // Now points to retrieval/ directory
pub use retrieval::*;
```

**Recommended:** Option B (delete and let mod.rs handle it)

### STEP 5: Verify compilation

```bash
cargo check -p paraphym_candle
```

**Expected:** Zero errors, all imports resolve correctly.

### STEP 6: Verify public API unchanged

Check that downstream usage still compiles:
```bash
# This import path must still work
cargo check -p paraphym_candle --tests
```

## VERIFICATION CHECKLIST

- [ ] All 7 new files created in `retrieval/` subdirectory
- [ ] Each file is < 300 lines
- [ ] `retrieval.rs` replaced with module delegation or deleted
- [ ] `cargo check` passes without errors
- [ ] Public API preserved (all re-exports working)
- [ ] No circular dependencies between modules
- [ ] All cognitive processor integration preserved in `hybrid.rs`
- [ ] All temporal decay logic preserved in `temporal.rs`
- [ ] Manager orchestration logic intact

## CRITICAL PRESERVATION POINTS

### 1. Cognitive Processing Integration (in `hybrid.rs`)
The `get_vector_similarity` method integrates with the cognitive decision system. The exact logic for handling `DecisionOutcome::Accept`, `Defer`, `Reject`, and `RequestInfo` MUST be preserved byte-for-byte:

```rust
match decision.outcome {
    DecisionOutcome::Accept => {
        // Weight similarity by cognitive confidence
        let weighted_score = result.score * decision.confidence;
        // Include cognitive metadata
    }
    DecisionOutcome::Defer => {
        // Reduce score for deferred queries
        let reduced_score = result.score * 0.5;
    }
    DecisionOutcome::Reject | DecisionOutcome::RequestInfo => {
        // Filter out all results
        Vec::new()
    }
}
```

### 2. Temporal Scoring Formula (in `temporal.rs`)
The exponential decay and score combination is mathematically precise:

```rust
// Age in hours calculation
let age_hours = (now_timestamp - memory_timestamp) / 3600.0;

// Exponential decay
let temporal_score = -(age_hours * time_decay as f64).exp() as f32;

// Combined score: 70% temporal + 30% relevance
let final_score = (temporal_score * 0.7) + (relevance_score * 0.3);
```

### 3. Strategy Weighting (in `hybrid.rs` and `manager.rs`)
Default weights MUST remain:
- Semantic: 0.6
- Keyword: 0.2
- Temporal: 0.2

### 4. Async Patterns
All strategies use the same pattern:
```rust
let (tx, rx) = oneshot::channel();
tokio::spawn(async move {
    let result: Result<Vec<RetrievalResult>> = (async {
        // ... strategy logic ...
    }).await;
    let _ = tx.send(result);
});
PendingRetrieval::new(rx)
```

## DEFINITION OF DONE

- [x] File analyzed and decomposition boundaries identified
- [ ] 7 new module files created in `retrieval/` subdirectory:
  - [ ] `types.rs` (~90 lines)
  - [ ] `strategy.rs` (~25 lines)
  - [ ] `semantic.rs` (~80 lines)
  - [ ] `temporal.rs` (~240 lines)
  - [ ] `hybrid.rs` (~220 lines)
  - [ ] `manager.rs` (~140 lines)
  - [ ] `mod.rs` (~40 lines)
- [ ] Original `retrieval.rs` replaced with module delegation or removed
- [ ] `cargo check` passes without errors
- [ ] Public API unchanged (verified by checking dependent files)
- [ ] All cognitive processing logic preserved
- [ ] All temporal scoring formulas preserved
- [ ] No functionality lost or altered

## REFERENCES

### Related Decomposition Patterns in Codebase

1. **Cognitive Types Module** ([`domain/memory/cognitive/types/`](../packages/candle/src/domain/memory/cognitive/types/)):
   - `mod.rs` - Aggregator with re-exports
   - `processor.rs` - Complex processor implementation
   - `state.rs`, `atomics.rs`, etc. - Supporting types
   - **Pattern:** Large system broken into focused modules

2. **Capability Registry** ([`capability/registry/`](../packages/candle/src/capability/registry/)):
   - `mod.rs` - Re-exports
   - `text_embedding.rs`, `vision.rs`, etc. - Specialized implementations
   - **Pattern:** Strategy-based decomposition

3. **Memory Operations** ([`memory/core/ops/`](../packages/candle/src/memory/core/ops/)):
   - Current sibling files: `storage.rs`, `query.rs`, `filter.rs`
   - **Pattern:** Each operation type in separate file
   - **Note:** `storage.rs` is 445 lines, also a candidate for decomposition

### Key Dependencies to Preserve

- `CognitiveProcessor` from `domain/memory/cognitive/types/processor.rs`
- `VectorStore` trait from `memory/vector/mod.rs`
- `MemoryManager` trait from `memory/mod.rs`
- `MemoryFilter` from `memory/filter.rs`

### Import Path Preservation

All existing imports must continue to work:
```rust
use crate::memory::core::ops::retrieval::RetrievalResult;
use crate::memory::core::ops::retrieval::RetrievalManager;
// etc.
```

This is guaranteed by the `pub use retrieval::*;` in `ops/mod.rs`.