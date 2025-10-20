# DECOMP_019: Decompose `vector_search.rs`

**File:** `packages/candle/src/memory/vector/vector_search.rs`  
**Current Size:** 907 lines  
**Module Area:** memory / vector  
**Target:** 7 focused modules, each < 360 lines

---

## OBJECTIVE

Decompose the monolithic `vector_search.rs` (907 lines) into smaller, focused, maintainable modules while preserving all existing functionality and the public API.

---

## CONSTRAINTS

- **NO TESTS:** Do not write any unit tests, integration tests, or test code.
- **NO BENCHMARKS:** Do not write any benchmark code.
- **NO EXTENSIVE DOCUMENTATION:** Keep existing inline docs, but don't add comprehensive module documentation beyond what exists.
- **MAINTAIN FUNCTIONALITY:** All existing functionality must be preserved exactly as-is.
- **PRESERVE PUBLIC API:** The public API surface must remain identical for backward compatibility.
- **SINGLE SESSION:** This task must be completable in one focused Claude session.

---

## FILE ANALYSIS

### Current Structure ([source](../packages/candle/src/memory/vector/vector_search.rs))

The 907-line file contains these logical components:

1. **Helper Functions** (lines 26-29)
   - `task_string()` - Convert static string to Option<String>

2. **Type Definitions** (lines 21-24, 31-61)
   - `RequestInfoCallback` - Type alias for callback functions
   - `DeferredResult` - Type alias for deferred search results
   - `FinalResult` - Type alias for final results
   - `KeywordSearchFn` - Type alias for keyword search functions
   - `CognitiveSearchState` - State for multi-stage cognitive filtering

3. **SearchResult** (lines 63-158)
   - Main result struct with comprehensive metadata
   - Builder methods (with_rank, with_combined_score, with_decision_confidence)
   - Utility methods (effective_score, memory_usage)

4. **SearchOptions** (lines 160-267)
   - Configuration struct with extensive options
   - Default implementation
   - Factory methods (fast(), comprehensive())
   - Validation logic

5. **VectorSearch** (lines 269-730)
   - Core search implementation with SIMD optimization
   - Methods: new(), search_by_text(), search_by_vector(), batch operations
   - Cognitive filtering integration
   - Embedding generation
   - Store and model management

6. **Cognitive Processing** (lines 733-763)
   - `process_deferred_results()` - Secondary threshold evaluation
   - Two-stage filtering for medium-confidence items

7. **HybridSearch** (lines 765-907)
   - Combines vector + keyword search strategies
   - Result merging and ranking algorithms
   - Configurable weighting between strategies

### Public API Surface (MUST PRESERVE)

From [`vector/mod.rs`](../packages/candle/src/memory/vector/mod.rs) line 19: `pub use vector_search::*;`

**Public exports:**
- `SearchResult` struct
- `SearchOptions` struct  
- `VectorSearch` struct
- `HybridSearch` struct
- `KeywordSearchFn` type alias

### Existing Vector Module Files

Current files in `packages/candle/src/memory/vector/`:
- `in_memory.rs` - In-memory vector storage
- `mod.rs` - Module aggregator
- `multimodal_service.rs` - Multimodal embedding service
- `vector_index.rs` - Vector indexing implementations
- `vector_repository.rs` - Vector collection management (312 lines, well-organized)
- `vector_search.rs` - **TARGET FILE** (907 lines, needs decomposition)
- `vector_store.rs` - Vector store trait and implementations

### Decomposition Patterns in Codebase

Observed patterns from [`domain/memory/cognitive/types/`](../packages/candle/src/domain/memory/cognitive/types/):
- Subdirectory with focused modules (activation.rs, atomics.rs, attention.rs, etc.)
- Each file < 200 lines
- mod.rs aggregates and re-exports

Observed from [`domain/chat/commands/types/`](../packages/candle/src/domain/chat/commands/types/):
- Nested subdirectories for complex features (actions/, code_execution/, events/)
- Clear separation of concerns
- Module files: types, errors, executor implementations, etc.

**Codebase convention**: Create subdirectory with focused module files + mod.rs for re-exports

---

## DECOMPOSITION PLAN

### Target Structure

Create new directory: `packages/candle/src/memory/vector/vector_search/`

### Module Breakdown

#### 1. `helpers.rs` (~30 lines)

**Purpose:** Utility functions  
**Contains:**
- `task_string()` function (line 26-29 from original)

**Imports needed:**
```rust
// No external imports needed
```

**Code snippet:**
```rust
/// Convert static string to Option<String> for embedding tasks
#[inline]
pub(super) fn task_string(task: &'static str) -> Option<String> {
    Some(task.to_string())
}
```

---

#### 2. `types.rs` (~135 lines)

**Purpose:** Core type definitions and SearchResult implementation  
**Contains:**
- Type aliases: RequestInfoCallback, DeferredResult, FinalResult, KeywordSearchFn
- SearchResult struct definition
- All SearchResult impl blocks (new, with_metadata, builder methods, utility methods)

**Imports needed:**
```rust
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use surrealdb::Value;
use crate::memory::utils::Result;
```

**Code snippet:**
```rust
/// Type alias for request info callback function
pub type RequestInfoCallback = Arc<dyn Fn(&str, f32, f32) -> bool + Send + Sync>;

/// Type alias for deferred search result with confidence
pub(crate) type DeferredResult = (String, Vec<f32>, f32, Option<HashMap<String, Value>>, f32);

/// Type alias for final search result
pub(crate) type FinalResult = (String, Vec<f32>, f32, Option<HashMap<String, Value>>);

/// Type alias for keyword search function
pub type KeywordSearchFn =
    Arc<dyn Fn(&str, Option<super::SearchOptions>) -> Result<Vec<SearchResult>> + Send + Sync>;

/// Search result with comprehensive metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub vector: Vec<f32>,
    pub similarity: f32,
    pub metadata: Option<HashMap<String, Value>>,
    pub rank: Option<usize>,
    pub combined_score: Option<f32>,
    pub decision_confidence: Option<f32>,
}

impl SearchResult {
    // ... all methods from lines 85-158
}
```

---

#### 3. `options.rs` (~110 lines)

**Purpose:** Search configuration and options  
**Contains:**
- SearchOptions struct definition
- Debug impl (custom, handles callback)
- Default impl
- Factory methods: fast(), comprehensive()
- validate() method

**Imports needed:**
```rust
use std::collections::HashMap;
use std::fmt;
use serde::{Deserialize, Serialize};
use surrealdb::Value;
use crate::memory::utils::error::Result;
use super::types::RequestInfoCallback;
```

**Code snippet:**
```rust
/// Search options for fine-tuning search behavior
#[derive(Clone, Serialize, Deserialize)]
pub struct SearchOptions {
    pub limit: Option<usize>,
    pub min_similarity: Option<f32>,
    pub filters: Option<HashMap<String, Value>>,
    pub include_vectors: Option<bool>,
    pub include_metadata: Option<bool>,
    pub include_rank: Option<bool>,
    pub candidate_limit: Option<usize>,
    pub enable_simd: Option<bool>,
    #[serde(skip)]
    pub request_info_callback: Option<RequestInfoCallback>,
}

impl Default for SearchOptions {
    // ... implementation from lines 211-224
}

impl SearchOptions {
    pub fn fast() -> Self { /* ... */ }
    pub fn comprehensive() -> Self { /* ... */ }
    pub fn validate(mut self) -> Result<Self> { /* ... */ }
}
```

---

#### 4. `cognitive.rs` (~120 lines)

**Purpose:** Cognitive filtering and state management  
**Contains:**
- CognitiveSearchState struct
- process_deferred_results() function
- Cognitive filtering logic

**Imports needed:**
```rust
use std::collections::HashMap;
use surrealdb::Value;
use super::types::{DeferredResult, FinalResult};
```

**Code snippet:**
```rust
/// State for multi-stage cognitive filtering
pub(crate) struct CognitiveSearchState {
    /// Results deferred for secondary evaluation with confidence scores
    pub(crate) deferred_results: Vec<DeferredResult>,
    /// Final accepted results
    pub(crate) final_results: Vec<FinalResult>,
}

impl CognitiveSearchState {
    pub(crate) fn new() -> Self {
        Self {
            deferred_results: Vec::new(),
            final_results: Vec::new(),
        }
    }
}

/// Process deferred results with secondary threshold evaluation
pub(crate) fn process_deferred_results(state: &mut CognitiveSearchState, threshold: f32) {
    // ... implementation from lines 733-763
}
```

---

#### 5. `core.rs` (~360 lines)

**Purpose:** Main VectorSearch implementation  
**Contains:**
- VectorSearch struct definition
- Constructor: new()
- Search methods: search_by_text(), search_by_vector()
- Batch operations: batch_search_by_text(), batch_search_by_vector()
- Getters/setters: store(), embedding_model(), set_default_options(), default_options()
- Cognitive filtering integration

**Imports needed:**
```rust
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};
use surrealdb::Value;

use crate::capability::registry::TextEmbeddingModel;
use crate::capability::traits::TextEmbeddingCapable;
use crate::memory::constants::SEARCH_TASK;
use crate::memory::utils::error::Result;
use crate::memory::vector::vector_store::VectorStore;
use crate::domain::memory::cognitive::types::{
    CognitiveProcessor, CognitiveProcessorConfig, DecisionOutcome,
};

use super::types::{SearchResult, DeferredResult, FinalResult};
use super::options::SearchOptions;
use super::cognitive::{CognitiveSearchState, process_deferred_results};
use super::helpers::task_string;
```

**Code snippet:**
```rust
/// High-performance vector search implementation
#[derive(Debug, Clone)]
pub struct VectorSearch {
    store: Arc<dyn VectorStore>,
    embedding_model: TextEmbeddingModel,
    default_options: SearchOptions,
    cognitive_processor: Arc<CognitiveProcessor>,
}

impl VectorSearch {
    pub fn new(
        store: Arc<dyn VectorStore>,
        embedding_model: TextEmbeddingModel,
    ) -> Self {
        // ... implementation from lines 292-309
    }

    pub async fn search_by_text(
        &self,
        text: &str,
        options: Option<SearchOptions>,
    ) -> Result<Vec<SearchResult>> {
        // ... implementation with cognitive filtering
    }

    pub async fn search_by_vector(
        &self,
        query_vector: &[f32],
        options: Option<SearchOptions>,
    ) -> Result<Vec<SearchResult>> {
        // ... implementation
    }

    // ... all other methods
}
```

---

#### 6. `hybrid.rs` (~145 lines)

**Purpose:** Hybrid search combining vector + keyword strategies  
**Contains:**
- HybridSearch struct definition
- Constructor: new()
- search() method - combines both strategies
- combine_results() - merges and ranks results
- Weight management: set_vector_weight(), vector_weight(), keyword_weight()
- Accessor: vector_search()

**Imports needed:**
```rust
use std::cmp::Ordering;
use std::collections::HashMap;
use surrealdb::Value;

use crate::memory::utils::error::Result;
use super::types::{SearchResult, KeywordSearchFn};
use super::options::SearchOptions;
use super::core::VectorSearch;
```

**Code snippet:**
```rust
/// Hybrid search combining vector and keyword search strategies
#[derive(Clone)]
pub struct HybridSearch {
    vector_search: VectorSearch,
    keyword_search: KeywordSearchFn,
    vector_weight: f32,
    keyword_weight: f32,
}

impl HybridSearch {
    pub fn new(
        vector_search: VectorSearch,
        keyword_search: KeywordSearchFn,
        vector_weight: Option<f32>,
    ) -> Self {
        // ... implementation from lines 803-816
    }

    pub async fn search(
        &self,
        text: &str,
        options: Option<SearchOptions>,
    ) -> Result<Vec<SearchResult>> {
        // ... parallel execution of vector + keyword search
        // ... combine and rank results
    }

    fn combine_results(
        &self,
        vector_results: Vec<SearchResult>,
        keyword_results: Vec<SearchResult>,
        options: Option<SearchOptions>,
    ) -> Vec<SearchResult> {
        // ... sophisticated merging algorithm
    }

    // ... weight management methods
}
```

---

#### 7. `mod.rs` (~40 lines)

**Purpose:** Module aggregator and public API re-exports  
**Contains:**
- Module declarations
- Public re-exports to maintain API compatibility
- Module-level documentation

**Complete code:**
```rust
//! Vector search functionality - THREAD-SAFE SYNCHRONOUS OPERATIONS
//!
//! This module provides comprehensive vector search capabilities using:
//! - Synchronous vector similarity search with SIMD acceleration
//! - Thread-safe embedding generation and caching
//! - Hybrid search combining vector and keyword approaches
//! - Zero-allocation search result processing
//! - Advanced filtering and ranking algorithms

mod helpers;
mod types;
mod options;
mod cognitive;
mod core;
mod hybrid;

// Re-export public API (maintains backward compatibility)
pub use types::{SearchResult, KeywordSearchFn};
pub use options::SearchOptions;
pub use core::VectorSearch;
pub use hybrid::HybridSearch;

// Internal helpers available to other vector modules if needed
pub(crate) use cognitive::{CognitiveSearchState, process_deferred_results};
pub(crate) use helpers::task_string;
```

---

## EXECUTION STEPS

### Step 1: Create Directory Structure

```bash
mkdir -p packages/candle/src/memory/vector/vector_search
```

### Step 2: Create Module Files (in dependency order)

Create files in this order to avoid forward reference issues:

1. **helpers.rs** - No dependencies
2. **types.rs** - Uses SearchOptions from options (will need `use super::options::SearchOptions` or circular fix)
3. **options.rs** - Uses RequestInfoCallback from types
4. **cognitive.rs** - Uses types from types.rs
5. **core.rs** - Uses all above modules
6. **hybrid.rs** - Uses core.rs
7. **mod.rs** - Aggregates all modules

**Important:** To avoid circular dependencies between types.rs and options.rs:
- Put KeywordSearchFn in types.rs but have it reference `super::options::SearchOptions`
- OR move KeywordSearchFn to options.rs and have types.rs import it

**Recommended:** Keep KeywordSearchFn in types.rs and use forward reference to SearchOptions

### Step 3: Move Code Systematically

For each module file:
1. Add the file header comment
2. Add all necessary `use` statements
3. Copy the relevant code sections from original vector_search.rs
4. Adjust visibility (pub vs pub(crate) vs pub(super))
5. Verify imports work with the new structure

### Step 4: Backup Original File

```bash
mv packages/candle/src/memory/vector/vector_search.rs \
   packages/candle/src/memory/vector/vector_search.rs.backup
```

### Step 5: Update Parent Module

The parent `vector/mod.rs` already has:
```rust
pub mod vector_search;
```

This will automatically find `vector_search/mod.rs` when vector_search becomes a directory. **No change needed.**

### Step 6: Verify Compilation

```bash
cd packages/candle
cargo check
```

Fix any import or visibility issues that arise.

---

## MODULE DEPENDENCIES

Dependency graph (arrows indicate "depends on"):

```
helpers.rs (no dependencies)
    ↓
types.rs → options.rs (circular - use forward reference)
    ↓           ↓
    ↓    cognitive.rs
    ↓           ↓
    └─→ core.rs ←┘
            ↓
        hybrid.rs
            ↓
         mod.rs (re-exports all)
```

**Circular dependency resolution:** KeywordSearchFn in types.rs references SearchOptions from options.rs. This is resolved through Rust's module system using `super::options::SearchOptions` or by moving the type.

---

## VISIBILITY GUIDELINES

### Public (pub)
- SearchResult, SearchOptions, VectorSearch, HybridSearch structs
- KeywordSearchFn type alias
- All public methods on the above types

### Crate-visible (pub(crate))
- CognitiveSearchState struct
- process_deferred_results function
- task_string function
- May be used by other memory modules

### Module-visible (pub(super))
- Helper functions only needed within vector_search module
- Internal type aliases (DeferredResult, FinalResult)

### Private (no modifier)
- Internal implementation details
- Private helper methods

---

## DEFINITION OF DONE

- [ ] `vector_search.rs` is replaced by `vector_search/` directory with 7 module files
- [ ] All 7 module files are < 360 lines each (target: < 300)
- [ ] `mod.rs` properly aggregates and re-exports public API
- [ ] Public API surface is unchanged (backward compatible)
- [ ] All functionality is preserved
- [ ] `cargo check` passes without errors
- [ ] Original file backed up to `vector_search.rs.backup`
- [ ] No tests written (per constraints)
- [ ] No benchmarks written (per constraints)
- [ ] No extensive documentation added (per constraints)

---

## RELATED FILES

- [Original file to decompose](../packages/candle/src/memory/vector/vector_search.rs)
- [Parent module](../packages/candle/src/memory/vector/mod.rs)
- [Example: vector_repository.rs](../packages/candle/src/memory/vector/vector_repository.rs) - Well-organized 312-line file
- [Pattern: cognitive types/](../packages/candle/src/domain/memory/cognitive/types/) - Good decomposition example
- [Pattern: chat commands types/](../packages/candle/src/domain/chat/commands/types/) - Complex nested decomposition

---

## NOTES

### Why This Decomposition?

1. **Separation of Concerns**: Each module has a single, well-defined purpose
2. **Dependency Management**: Clear dependency hierarchy prevents circular issues
3. **Maintainability**: Smaller files are easier to understand and modify
4. **Follows Codebase Patterns**: Matches existing decomposition strategies in domain/memory/cognitive/types/
5. **Preserves API**: Public exports through mod.rs maintain backward compatibility

### Potential Challenges

1. **Circular dependency** between types.rs and options.rs (KeywordSearchFn needs SearchOptions)
   - **Solution**: Use `super::options::SearchOptions` in the type alias

2. **Import complexity** in core.rs due to many dependencies
   - **Solution**: Organize imports clearly with module comments

3. **Cognitive filtering** logic is integrated into VectorSearch methods
   - **Solution**: Keep it in core.rs, import from cognitive.rs

### Success Metrics

- ✅ Reduced file sizes (all < 360 lines)
- ✅ Improved code organization
- ✅ Maintained functionality
- ✅ Preserved public API
- ✅ Clean compilation
