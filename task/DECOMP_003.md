# DECOMP_003: Decompose `types.rs`

**File:** `packages/candle/src/domain/memory/cognitive/types.rs`  
**Current Size:** 1,970 lines  
**Module Area:** domain / memory / cognitive

## OBJECTIVE

Select 1 file in ./src/ >= 500 lines of code and decompose it into logical separation of concerns with no single module >= 500 lines of code. Ensure all the sum of parts exactly equals the original with ONLY production quality source code. Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED. Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

## CONSTRAINTS

- **NO TESTS:** Do not write any unit tests, integration tests, or test code.
- **NO BENCHMARKS:** Do not write any benchmark code.
- **NO DOCUMENTATION:** Do not write extensive documentation beyond essential module comments.
- **MAINTAIN FUNCTIONALITY:** All existing functionality must be preserved exactly as-is.
- **SINGLE SESSION:** This task must be completable in one focused Claude session.
- **DELETE ORIGINAL:** The original `types.rs` must be deleted after decomposition.
- **NO BACKUPS:** Do not create backup files like `types.rs.bak` or similar.

## FILE STRUCTURE ANALYSIS

After analyzing the 1,970-line `types.rs` file, the structure is:

### Current Contents (line ranges approximate)

1. **Lines 1-125**: AlignedActivationPattern (~125 lines)
   - SIMD-aligned activation pattern struct
   - Methods for pattern updates and transformations
   - Default implementation

2. **Lines 126-243**: AtomicAttentionWeights (~118 lines)
   - Atomic attention weight management
   - Concurrent update methods
   - Default implementation


3. **Lines 244-285**: WorkingMemoryItem (~42 lines)
   - Working memory item struct
   - Priority and decay methods
   - Constructor

4. **Lines 286-337**: CognitiveMemoryEntry (~52 lines)
   - Long-term memory entry struct
   - Relevance scoring
   - Constructor

5. **Lines 338-420**: TemporalContext (~83 lines)
   - Temporal window management
   - Causal links tracking
   - Time-aware memory operations

6. **Lines 421-455**: CausalLink (~35 lines)
   - Causal dependency tracking between memories
   - Strength and confidence scoring

7. **Lines 456-792**: QuantumSignature (~337 lines)
   - Quantum-inspired coherence tracking
   - AlignedCoherenceFingerprint (SIMD-aligned)
   - Entanglement routing
   - Complex quantum state management

8. **Lines 793-842**: EntanglementBond and EntanglementType (~50 lines)
   - Entanglement bond struct
   - EntanglementType enum (Semantic, Bell, BellPair, Temporal, Causal)
   - Bond methods

9. **Lines 843-919**: Atomic wrappers (~77 lines)
   - AtomicF32 struct and implementation
   - AtomicF64 struct and implementation
   - Thread-safe float operations

10. **Lines 920-983**: CognitiveStats (~64 lines)
    - Statistics tracking for cognitive operations
    - Atomic counters for concurrent updates

11. **Lines 984-1457**: CognitiveState (~474 lines)
    - Main cognitive state struct
    - Complex state management methods
    - Integration of all components
    - Default implementation

12. **Lines 1458-1487**: CognitiveError (~30 lines)
    - Error enum for cognitive operations

13. **Lines 1488-1633**: High-level cognitive system (~146 lines)
    - CognitiveMemory struct
    - CognitiveProcessor struct
    - CognitiveMemoryConfig struct
    - CognitiveProcessorConfig struct
    - CognitivePattern struct
    - CognitiveMetrics struct
    - ProcessingState struct
    - PatternMatcher struct
    - DecisionEngine struct
    - Decision struct
    - DecisionOutcome enum

14. **Lines 1634-1971**: Implementations (~338 lines)
    - `impl CognitiveMemory` methods
    - `impl CognitiveProcessor` methods
    - Default implementations for configs
    - `impl CognitiveMetrics`, `impl ProcessingState`
    - `impl PatternMatcher` with pattern matching algorithms
    - `impl DecisionEngine` with decision logic

### Public API (must be preserved)

From `[cognitive/mod.rs](../../packages/candle/src/domain/memory/cognitive/mod.rs)`:
```rust
pub mod types;

// Re-export the main cognitive types that are imported elsewhere
pub use types::{CognitiveMemory, CognitiveProcessor};
```

Only `CognitiveMemory` and `CognitiveProcessor` are re-exported, but all public types in `types.rs` are accessible via `types::`.

### Dependencies

The file imports from:
- `std::sync::atomic` - Atomic operations
- `tokio::sync` - Async primitives (RwLock, Mutex, mpsc)
- `crossbeam_skiplist::SkipMap` - Lock-free ordered map
- `paraphym_simd::similarity::cosine_similarity` - SIMD similarity
- `crate::domain::util::unix_timestamp_nanos` - Timestamp utilities

## DECOMPOSITION PLAN

Create a `cognitive/types/` subdirectory with 8 focused modules:

```
packages/candle/src/domain/memory/cognitive/
├── mod.rs (UPDATE THIS)
├── types/ (NEW DIRECTORY)
│   ├── mod.rs (NEW - aggregates and re-exports)
│   ├── activation.rs (NEW - ~130 lines)
│   ├── attention.rs (NEW - ~120 lines)
│   ├── memory_items.rs (NEW - ~100 lines)
│   ├── temporal.rs (NEW - ~120 lines)
│   ├── quantum.rs (NEW - ~390 lines)
│   ├── atomics.rs (NEW - ~150 lines)
│   ├── state.rs (NEW - ~480 lines)
│   └── processor.rs (NEW - ~490 lines)
└── types.rs (DELETE AFTER DECOMPOSITION)
```

### Module Breakdown

#### 1. `types/activation.rs` (~130 lines)
**Purpose:** SIMD-aligned activation patterns for neural processing

**Contents:**
- `AlignedActivationPattern` struct with #[repr(align(32))]
- Methods: `new()`, `update()`, `apply_activation()`, `decay()`
- SIMD optimization hints and aligned operations
- Default implementation

**Rationale:** Isolates SIMD-specific activation pattern logic

#### 2. `types/attention.rs` (~120 lines)
**Purpose:** Atomic attention weight management

**Contents:**
- `AtomicAttentionWeights` struct
- Methods for concurrent attention updates
- Attention shifting and normalization
- Default implementation

**Rationale:** Separates attention mechanism from other cognitive components

#### 3. `types/memory_items.rs` (~100 lines)
**Purpose:** Memory item definitions

**Contents:**
- `WorkingMemoryItem` struct
- `CognitiveMemoryEntry` struct
- `CognitiveStats` struct
- Associated methods for relevance, priority, decay
- Default implementations

**Rationale:** Groups memory data structures together

#### 4. `types/temporal.rs` (~120 lines)
**Purpose:** Temporal and causal context tracking

**Contents:**
- `TemporalContext` struct
- `CausalLink` struct
- Temporal window management
- Causal dependency tracking
- Time-aware operations
- Default implementations


**Rationale:** Temporal logic is distinct from quantum and attention mechanics

#### 5. `types/quantum.rs` (~390 lines)
**Purpose:** Quantum-inspired signatures and entanglement

**Contents:**
- `QuantumSignature` struct (complex quantum state)
- `AlignedCoherenceFingerprint` struct (SIMD-aligned)
- `EntanglementBond` struct
- `EntanglementType` enum (5 variants)
- Quantum routing and coherence methods
- Entanglement operations
- Default implementations

**Rationale:** Quantum logic is substantial and self-contained, warrants its own module

#### 6. `types/atomics.rs` (~150 lines)
**Purpose:** Atomic wrapper types for lock-free operations

**Contents:**
- `AtomicF32` struct with load/store/update methods
- `AtomicF64` struct with load/store/update methods
- Thread-safe floating-point operations
- Default implementations

**Rationale:** Reusable atomic primitives, separate from domain logic

#### 7. `types/state.rs` (~480 lines)
**Purpose:** Core CognitiveState implementation

**Contents:**
- `CognitiveState` struct (main state)
- Complete `impl CognitiveState` with all methods:
  - State initialization and management
  - Working memory operations
  - Long-term memory operations
  - Attention updates
  - Quantum signature management
  - Statistical tracking
- Default implementation
- `CognitiveError` enum

**Rationale:** This is the central cognitive state machine, kept together as a cohesive unit

#### 8. `types/processor.rs` (~490 lines)
**Purpose:** High-level cognitive processing system

**Contents:**
- `CognitiveMemory` struct
- `CognitiveProcessor` struct
- `CognitiveMemoryConfig` struct
- `CognitiveProcessorConfig` struct
- `CognitivePattern` struct
- `CognitiveMetrics` struct
- `ProcessingState` struct
- `PatternMatcher` struct with pattern matching algorithms
- `DecisionEngine` struct with decision logic
- `Decision` struct
- `DecisionOutcome` enum
- Complete implementations for all types
- Default implementations

**Rationale:** High-level processing components grouped together, separate from low-level state

#### 9. `types/mod.rs` (~40 lines)
**Purpose:** Module aggregator and public API

**Contents:**
```rust
//! Quantum-inspired cognitive types for memory systems
//! 
//! Decomposed from a 1,970-line monolithic file into focused modules.

pub mod activation;
pub mod attention;
pub mod memory_items;
pub mod temporal;
pub mod quantum;
pub mod atomics;
pub mod state;
pub mod processor;

// Re-export all public items to maintain API compatibility
pub use activation::*;
pub use attention::*;
pub use memory_items::*;
pub use temporal::*;
pub use quantum::*;
pub use atomics::*;
pub use state::*;
pub use processor::*;
```

**Rationale:** Preserves the exact public API that `cognitive/mod.rs` expects

## EXECUTION STEPS

### STEP 1: Create the types subdirectory

```bash
cd /Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/cognitive
mkdir types
```

### STEP 2: Create `types/activation.rs`

Extract lines 66-133 from `types.rs`:
- `AlignedActivationPattern` struct with SIMD alignment
- All methods and implementations
- Default trait impl

**Key structure:**
```rust
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(align(32))] // Align to 32 bytes for AVX2 SIMD operations
pub struct AlignedActivationPattern {
    pub data: Vec<f32>,
    pub dimension: usize,
    pub last_update: SystemTime,
}

impl AlignedActivationPattern {
    #[inline]
    #[must_use]
    pub fn new(data: Vec<f32>) -> Self {
        let dimension = data.len();
        Self {
            data,
            dimension,
            last_update: SystemTime::now(),
        }
    }
    
    pub fn update(&mut self, new_data: Vec<f32>) { /* ... */ }
    pub fn apply_activation(&mut self, activation_fn: fn(f32) -> f32) { /* ... */ }
    pub fn decay(&mut self, rate: f32) { /* ... */ }
}

impl Default for AlignedActivationPattern {
    fn default() -> Self { /* ... */ }
}
```

### STEP 3: Create `types/attention.rs`

Extract lines 134-243 from `types.rs`:
- `AtomicAttentionWeights` struct
- All concurrent attention methods
- Default implementation

**Key structure:**
```rust
use std::sync::Arc;
use super::atomics::AtomicF32;

#[derive(Debug, Clone)]
pub struct AtomicAttentionWeights {
    pub focus: Arc<AtomicF32>,
    pub salience: Arc<AtomicF32>,
    pub novelty: Arc<AtomicF32>,
}

impl AtomicAttentionWeights {
    pub fn new(focus: f32, salience: f32, novelty: f32) -> Self { /* ... */ }
    pub fn update_focus(&self, delta: f32) { /* ... */ }
    pub fn update_salience(&self, delta: f32) { /* ... */ }
    pub fn update_novelty(&self, delta: f32) { /* ... */ }
    pub fn normalize(&self) { /* ... */ }
}
```

### STEP 4: Create `types/memory_items.rs`

Extract lines 244-337 and 920-983 from `types.rs`:
- `WorkingMemoryItem` struct
- `CognitiveMemoryEntry` struct
- `CognitiveStats` struct
- All associated methods

**Key structures:**
```rust
use std::time::{Duration, SystemTime};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingMemoryItem {
    pub content: Vec<f32>,
    pub priority: f32,
    pub creation_time: SystemTime,
    pub decay_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveMemoryEntry {
    pub id: Uuid,
    pub embedding: Vec<f32>,
    pub activation_level: f32,
    pub access_count: u32,
    pub last_access: SystemTime,
}

#[derive(Debug, Clone)]
pub struct CognitiveStats {
    pub total_operations: Arc<AtomicU64>,
    pub successful_operations: Arc<AtomicU64>,
    pub failed_operations: Arc<AtomicU64>,
}
```

### STEP 5: Create `types/temporal.rs`

Extract lines 338-455 from `types.rs`:
- `TemporalContext` struct
- `CausalLink` struct
- All temporal methods
- Default implementations

**Key structures:**
```rust
use std::time::{Duration, SystemTime};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct TemporalContext {
    pub current_window: Duration,
    pub window_start: SystemTime,
    pub causal_links: Arc<RwLock<Vec<CausalLink>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalLink {
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub strength: f32,
    pub confidence: f32,
}
```

### STEP 6: Create `types/quantum.rs`

Extract lines 456-842 from `types.rs`:
- `QuantumSignature` struct
- `AlignedCoherenceFingerprint` struct
- `EntanglementBond` struct
- `EntanglementType` enum
- All quantum methods and implementations

**Key structures:**
```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use paraphym_simd::similarity::cosine_similarity;
use super::atomics::AtomicF32;

#[derive(Debug, Clone)]
pub struct QuantumSignature {
    pub coherence: Arc<AtomicF32>,
    pub entanglement_strength: Arc<AtomicF32>,
    pub phase: Arc<RwLock<f64>>,
    pub fingerprint: Arc<RwLock<AlignedCoherenceFingerprint>>,
    pub last_collapse: Arc<RwLock<SystemTime>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(align(32))]
pub struct AlignedCoherenceFingerprint {
    pub data: Vec<f32>,
    pub dimension: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntanglementBond {
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub bond_type: EntanglementType,
    pub strength: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EntanglementType {
    Semantic,
    Bell,
    BellPair,
    Temporal,
    Causal,
}
```

This module contains substantial quantum logic (~390 lines).

### STEP 7: Create `types/atomics.rs`

Extract lines 843-919 from `types.rs`:
- `AtomicF32` struct with thread-safe float operations
- `AtomicF64` struct with thread-safe double operations
- Load, store, update methods
- Default implementations

**Key structures:**
```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AtomicF32 {
    inner: Arc<AtomicU64>,
}

impl AtomicF32 {
    pub fn new(value: f32) -> Self {
        Self {
            inner: Arc::new(AtomicU64::new(value.to_bits() as u64)),
        }
    }
    
    pub fn load(&self) -> f32 {
        f32::from_bits(self.inner.load(Ordering::Relaxed) as u32)
    }
    
    pub fn store(&self, value: f32) {
        self.inner.store(value.to_bits() as u64, Ordering::Relaxed);
    }
    
    pub fn update<F>(&self, f: F) where F: Fn(f32) -> f32 {
        let current = self.load();
        let new_value = f(current);
        self.store(new_value);
    }
}

#[derive(Debug, Clone)]
pub struct AtomicF64 {
    inner: Arc<AtomicU64>,
}

impl AtomicF64 {
    // Similar implementation for f64
}
```

### STEP 8: Create `types/state.rs`

Extract lines 25-65 (CognitiveState struct definition) and 1033-1487 from `types.rs`:
- `CognitiveState` struct definition
- Complete `impl CognitiveState` with all methods
- `CognitiveError` enum
- Default implementation


**Key structure:**
```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use crossbeam_skiplist::SkipMap;
use uuid::Uuid;
use super::activation::AlignedActivationPattern;
use super::attention::AtomicAttentionWeights;
use super::memory_items::*;
use super::temporal::TemporalContext;
use super::quantum::QuantumSignature;
use super::atomics::AtomicF32;

#[derive(Debug, Clone)]
pub struct CognitiveState {
    activation_pattern: AlignedActivationPattern,
    attention_weights: Arc<AtomicAttentionWeights>,
    working_memory: Arc<(/* mpsc channels */)>,
    long_term_memory: Arc<SkipMap<Uuid, CognitiveMemoryEntry>>,
    temporal_context: Arc<TemporalContext>,
    quantum_signature: Arc<QuantumSignature>,
    uncertainty: Arc<AtomicF32>,
    confidence: Arc<AtomicF32>,
    meta_awareness: Arc<AtomicF32>,
    stats: Arc<CognitiveStats>,
}

impl CognitiveState {
    pub fn new() -> Self { /* ... */ }
    pub async fn add_to_working_memory(&self, item: WorkingMemoryItem) -> Result<(), CognitiveError> { /* ... */ }
    pub async fn retrieve_from_long_term(&self, id: &Uuid) -> Option<CognitiveMemoryEntry> { /* ... */ }
    pub fn update_attention(&self, focus: f32, salience: f32, novelty: f32) { /* ... */ }
    pub async fn update_quantum_signature(&self, /* ... */) -> Result<(), CognitiveError> { /* ... */ }
    // ... many more methods
}

#[derive(Debug, Clone)]
pub enum CognitiveError {
    MemoryOperationFailed(String),
    InvalidState(String),
    QuantumCoherenceLoss(String),
    TemporalInconsistency(String),
}
```

This is the core state machine (~480 lines).

### STEP 9: Create `types/processor.rs`

Extract lines 1488-1971 from `types.rs`:
- `CognitiveMemory` struct
- `CognitiveProcessor` struct
- Configuration structs
- Pattern and metrics structs
- `PatternMatcher` struct with algorithms
- `DecisionEngine` struct with decision logic
- All implementations


**Key structures:**
```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::state::CognitiveState;
use super::memory_items::CognitiveMemoryEntry;

#[derive(Debug, Clone)]
pub struct CognitiveMemory {
    pub state: Arc<CognitiveState>,
    pub config: CognitiveMemoryConfig,
}

#[derive(Debug, Clone)]
pub struct CognitiveProcessor {
    pub memory: Arc<CognitiveMemory>,
    pub config: CognitiveProcessorConfig,
    pub metrics: Arc<RwLock<CognitiveMetrics>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitivePattern {
    pub pattern_type: String,
    pub features: Vec<f32>,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub struct PatternMatcher {
    pub patterns: Arc<RwLock<Vec<CognitivePattern>>>,
    pub threshold: f32,
}

impl PatternMatcher {
    pub async fn match_pattern(&self, input: &[f32]) -> Option<CognitivePattern> {
        // Pattern matching algorithm
    }
}

#[derive(Debug, Clone)]
pub struct DecisionEngine {
    pub rules: Arc<RwLock<Vec<Decision>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionOutcome {
    Accept,
    Reject,
    Defer,
    Escalate,
}
```

This module contains high-level processing logic (~490 lines).

### STEP 10: Create `types/mod.rs`

Create the aggregator module:

```rust
//! Quantum-inspired cognitive types for memory systems
//! 
//! This module was decomposed from a 1,970-line monolithic file
//! into 8 focused modules for better maintainability.
//!
//! ## Modules
//! - `activation`: SIMD-aligned activation patterns
//! - `attention`: Atomic attention weight management
//! - `memory_items`: Memory item definitions
//! - `temporal`: Temporal and causal context
//! - `quantum`: Quantum signatures and entanglement
//! - `atomics`: Atomic wrapper types
//! - `state`: Core cognitive state
//! - `processor`: High-level processing system

pub mod activation;
pub mod attention;
pub mod memory_items;
pub mod temporal;
pub mod quantum;
pub mod atomics;
pub mod state;
pub mod processor;

// Re-export all public items to maintain API compatibility
pub use activation::*;
pub use attention::*;
pub use memory_items::*;
pub use temporal::*;
pub use quantum::*;
pub use atomics::*;
pub use state::*;
pub use processor::*;
```

### STEP 11: Update `cognitive/mod.rs`

**NO CHANGES NEEDED!** The mod.rs already declares:
```rust
pub mod types;
```

Rust treats both `types.rs` and `types/mod.rs` identically, so the existing import continues to work.

The re-exports also continue to work:
```rust
pub use types::{CognitiveMemory, CognitiveProcessor};
```

### STEP 12: Delete the original `types.rs`

**CRITICAL:** Once all modules are created and verified, DELETE the original file:

```bash
rm /Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/cognitive/types.rs
```

**DO NOT:**
- Rename it to `types.rs.bak`
- Keep it as `types.rs.old`
- Move it to a backup directory

**The file must be completely deleted** so that the new `types/` module is the only version.

### STEP 13: Verify compilation

```bash
cd /Volumes/samsung_t9/paraphym/packages/candle
cargo check
```

Fix any issues:
- Missing imports
- Incorrect visibility (`pub` vs private)
- Incorrect module paths


### STEP 14: Check for backup pollution

Ensure no backup files were created:

```bash
find /Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/cognitive -name "*types*.bak" -o -name "*types*.old" -o -name "*types*.backup"
```

Should return nothing. If it finds files, delete them.

## WHAT CHANGES IN ./src FILES

### Files to CREATE:
1. `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/cognitive/types/mod.rs`
2. `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/cognitive/types/activation.rs`
3. `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/cognitive/types/attention.rs`
4. `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/cognitive/types/memory_items.rs`
5. `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/cognitive/types/temporal.rs`
6. `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/cognitive/types/quantum.rs`
7. `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/cognitive/types/atomics.rs`
8. `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/cognitive/types/state.rs`
9. `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/cognitive/types/processor.rs`

### File to DELETE:
1. `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/cognitive/types.rs` ⚠️ **MUST DELETE**

### Files that need NO changes:
- `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/memory/cognitive/mod.rs` (already correct)
- All importing files (API preserved via re-exports)

## DEFINITION OF DONE

- [ ] Directory `types/` created with 9 new `.rs` files
- [ ] `types/activation.rs` exists and contains ~130 lines
- [ ] `types/attention.rs` exists and contains ~120 lines
- [ ] `types/memory_items.rs` exists and contains ~100 lines
- [ ] `types/temporal.rs` exists and contains ~120 lines
- [ ] `types/quantum.rs` exists and contains ~390 lines
- [ ] `types/atomics.rs` exists and contains ~150 lines
- [ ] `types/state.rs` exists and contains ~480 lines
- [ ] `types/processor.rs` exists and contains ~490 lines
- [ ] `types/mod.rs` exists and re-exports all public items
- [ ] Original `types.rs` is **DELETED** (not renamed, not moved)
- [ ] No `.bak`, `.old`, or `.backup` files exist
- [ ] `cargo check` passes without errors or warnings
- [ ] All functionality preserved (verified by compilation)
- [ ] Public API unchanged (imports still work)
- [ ] No single module exceeds 490 lines


## RESEARCH NOTES

### File Location
`[types.rs](../../packages/candle/src/domain/memory/cognitive/types.rs)` - 1,970 lines

### Current Module Structure
```
packages/candle/src/domain/memory/
├── cognitive/
│   ├── mod.rs (5 lines - re-exports CognitiveMemory, CognitiveProcessor)
│   └── types.rs (1,970 lines - THIS TASK)
├── primitives/
├── config/
└── ... (other memory modules)
```

### Imports Used Throughout

The file heavily uses these crates and modules:
- `std::sync::Arc` - Shared ownership
- `std::sync::atomic::{AtomicU64, Ordering}` - Lock-free operations
- `tokio::sync::{RwLock, Mutex, mpsc}` - Async primitives
- `crossbeam_skiplist::SkipMap` - Lock-free ordered map
- `serde::{Deserialize, Serialize}` - Serialization
- `uuid::Uuid` - Unique identifiers
- `paraphym_simd::similarity::cosine_similarity` - SIMD similarity computation
- `crate::domain::util::unix_timestamp_nanos` - Timestamp utilities

### Key Implementation Patterns

#### Pattern 1: SIMD-Aligned Structures

Multiple structures use explicit alignment for SIMD operations:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(align(32))] // Align to 32 bytes for AVX2 SIMD operations
pub struct AlignedActivationPattern {
    pub data: Vec<f32>,
    pub dimension: usize,
    pub last_update: SystemTime,
}

#[repr(align(32))]
pub struct AlignedCoherenceFingerprint {
    pub data: Vec<f32>,
    pub dimension: usize,
}
```

The `#[repr(align(32))]` attribute ensures data is aligned for AVX2 SIMD instructions, enabling vectorized operations on activation patterns and quantum fingerprints.

#### Pattern 2: Atomic Float Wrappers

Thread-safe float operations using atomic u64 conversion:
```rust
#[derive(Debug, Clone)]
pub struct AtomicF32 {
    inner: Arc<AtomicU64>,
}

impl AtomicF32 {
    pub fn new(value: f32) -> Self {
        Self {
            inner: Arc::new(AtomicU64::new(value.to_bits() as u64)),
        }
    }
    
    pub fn load(&self) -> f32 {
        f32::from_bits(self.inner.load(Ordering::Relaxed) as u32)
    }
    
    pub fn store(&self, value: f32) {
        self.inner.store(value.to_bits() as u64, Ordering::Relaxed);
    }
    
    pub fn update<F>(&self, f: F) where F: Fn(f32) -> f32 {
        let current = self.load();
        let new_value = f(current);
        self.store(new_value);
    }
}
```

This pattern enables lock-free concurrent float operations by converting to/from u64 bits.

#### Pattern 3: Lock-Free SkipMap for Memory

Long-term memory uses `crossbeam_skiplist::SkipMap` for O(log n) concurrent access:
```rust
pub struct CognitiveState {
    // ...
    long_term_memory: Arc<SkipMap<Uuid, CognitiveMemoryEntry>>,
    // ...
}

impl CognitiveState {
    pub async fn retrieve_from_long_term(&self, id: &Uuid) -> Option<CognitiveMemoryEntry> {
        self.long_term_memory.get(id).map(|entry| entry.value().clone())
    }
    
    pub async fn store_in_long_term(&self, entry: CognitiveMemoryEntry) {
        self.long_term_memory.insert(entry.id, entry);
    }
}
```

SkipMap provides concurrent, lock-free access with ordered iteration.

#### Pattern 4: mpsc Channels for Working Memory

Working memory uses unbounded channels for queue-like behavior:
```rust
pub struct CognitiveState {
    working_memory: Arc<(
        mpsc::UnboundedSender<WorkingMemoryItem>,
        Mutex<mpsc::UnboundedReceiver<WorkingMemoryItem>>
    )>,
}

impl CognitiveState {
    pub async fn add_to_working_memory(&self, item: WorkingMemoryItem) -> Result<(), CognitiveError> {
        self.working_memory.0.send(item)
            .map_err(|e| CognitiveError::MemoryOperationFailed(e.to_string()))
    }
}
```

The sender is cloneable for concurrent writes, receiver is mutex-protected for exclusive reads.

#### Pattern 5: Quantum Coherence with SIMD

Quantum signatures use SIMD similarity for coherence calculations:
```rust
use paraphym_simd::similarity::cosine_similarity;

impl QuantumSignature {
    pub async fn calculate_coherence(&self, other: &QuantumSignature) -> f32 {
        let self_fingerprint = self.fingerprint.read().await;
        let other_fingerprint = other.fingerprint.read().await;
        
        // SIMD-optimized cosine similarity
        cosine_similarity(&self_fingerprint.data, &other_fingerprint.data)
    }
}
```

Leverages SIMD instructions from `paraphym_simd` crate for fast vector similarity.

### Critical Dependencies

The cognitive types integrate with:

1. **paraphym_simd** (external crate)
   - `similarity::cosine_similarity` - SIMD vector similarity
   - Used in quantum coherence calculations
   - Requires aligned data structures

2. **crossbeam_skiplist** (external crate)
   - Lock-free ordered map for long-term memory
   - O(log n) concurrent access
   - Used heavily in memory storage

3. **tokio async ecosystem**
   - `RwLock`, `Mutex` - Async synchronization
   - `mpsc` - Async channels
   - All methods are async-aware

4. **Domain utilities**
   - `crate::domain::util::unix_timestamp_nanos` - Timestamps
   - Used throughout for temporal operations

### Public API Preservation

From `[cognitive/mod.rs](../../packages/candle/src/domain/memory/cognitive/mod.rs)`:
```rust
pub use types::{CognitiveMemory, CognitiveProcessor};
```

Only these two types are re-exported to parent modules, but all public types in `types.rs` are accessible via `types::`.

### Line Count Verification

Target module sizes after decomposition:

| Module | Lines | Status |
|--------|-------|--------|
| `activation.rs` | ~130 | ✅ Well below 500 |
| `attention.rs` | ~120 | ✅ Well below 500 |
| `memory_items.rs` | ~100 | ✅ Well below 500 |
| `temporal.rs` | ~120 | ✅ Well below 500 |
| `quantum.rs` | ~390 | ✅ Well below 500 |
| `atomics.rs` | ~150 | ✅ Well below 500 |
| `state.rs` | ~480 | ✅ Just under 500 |
| `processor.rs` | ~490 | ✅ Just under 500 |
| `mod.rs` | ~40 | ✅ Minimal |
| **Total** | **~2,020** | ✅ Matches original + overhead |


The ~50 line difference from original 1,970 accounts for:
- Module declarations (9 files × ~3 lines)
- Re-export statements (9 files × ~5 lines)  
- Module documentation comments (9 files × ~2 lines)

### Cognitive System Features

The types implement a quantum-inspired cognitive memory system with:

1. **SIMD-Optimized Activation Patterns**
   - 32-byte aligned structures for AVX2
   - Vectorized activation functions
   - Fast decay calculations

2. **Atomic Attention Weights**
   - Lock-free concurrent updates
   - Focus, salience, novelty tracking
   - Real-time normalization

3. **Working Memory Queue**
   - MPSC channel-based
   - Priority and decay tracking
   - Concurrent producer, single consumer

4. **Long-Term Memory SkipMap**
   - O(log n) concurrent access
   - Lock-free ordered iteration
   - UUID-keyed storage

5. **Temporal Context**
   - Time window management
   - Causal link tracking
   - Temporal consistency checks

6. **Quantum Signatures**
   - Coherence tracking with SIMD
   - Entanglement bonds (5 types)
   - Phase and fingerprint management
   - Collapse tracking

7. **Pattern Matching**
   - Feature-based pattern recognition
   - Confidence scoring
   - Threshold-based matching

8. **Decision Engine**
   - Rule-based decision making
   - 4 outcome types (Accept, Reject, Defer, Escalate)
   - Configurable thresholds

### SIMD Alignment Requirements

Key structures requiring 32-byte alignment for AVX2:
- `AlignedActivationPattern` - Neural activation data
- `AlignedCoherenceFingerprint` - Quantum coherence data

These must maintain their `#[repr(align(32))]` attributes during decomposition.

### Atomic Operations

All atomic operations use `Ordering::Relaxed` for performance:
- `AtomicU64::load(Ordering::Relaxed)`
- `AtomicU64::store(val, Ordering::Relaxed)`

This is safe for the cognitive system's use case where exact ordering isn't critical.


## IMPLEMENTATION CHECKLIST

### Before Starting
- [ ] Read the complete `types.rs` file (1,970 lines)
- [ ] Understand the 8-module decomposition plan
- [ ] Create the `types/` directory

### Module Creation (in order)
- [ ] Create `types/activation.rs` (lines 66-133)
- [ ] Create `types/attention.rs` (lines 134-243)
- [ ] Create `types/memory_items.rs` (lines 244-337 + 920-983)
- [ ] Create `types/temporal.rs` (lines 338-455)
- [ ] Create `types/quantum.rs` (lines 456-842)
- [ ] Create `types/atomics.rs` (lines 843-919)
- [ ] Create `types/state.rs` (lines 25-65 + 1033-1487)
- [ ] Create `types/processor.rs` (lines 1488-1971)
- [ ] Create `types/mod.rs` (aggregator)

### Verification
- [ ] Run `cargo check` - should pass
- [ ] Verify no backup files exist
- [ ] **DELETE** `types.rs` completely
- [ ] Run `cargo check` again - should still pass
- [ ] Check that re-exports in `cognitive/mod.rs` still work

### Cleanup
- [ ] Remove any `.bak`, `.old`, `.backup` files
- [ ] Verify `types.rs` no longer exists
- [ ] Commit the decomposition

## SUCCESS CRITERIA

This task is successful when:

1. ✅ The original 1,970-line `types.rs` is **completely deleted**
2. ✅ Nine new files exist in `types/` directory
3. ✅ No single module exceeds 490 lines
4. ✅ `cargo check` passes without errors
5. ✅ Public API is preserved (all imports still work)
6. ✅ No backup files pollute the codebase
7. ✅ SIMD alignment attributes are preserved
8. ✅ Atomic operations function correctly
9. ✅ Code is production quality with no stubs or placeholders

## REFERENCES

### Source Files
- Current file: `[types.rs](../../packages/candle/src/domain/memory/cognitive/types.rs)`
- Parent module: `[cognitive/mod.rs](../../packages/candle/src/domain/memory/cognitive/mod.rs)`

### Dependencies
- SIMD operations: `paraphym_simd::similarity`
- Lock-free structures: `crossbeam_skiplist::SkipMap`
- Async primitives: `tokio::sync`
- Utilities: `crate::domain::util`

### Related Modules
- `[domain/memory/primitives/](../../packages/candle/src/domain/memory/primitives/)` - Memory primitives
- `[domain/memory/config/](../../packages/candle/src/domain/memory/config/)` - Configuration

### External Documentation
- Crossbeam SkipList: https://docs.rs/crossbeam-skiplist/latest/crossbeam_skiplist/
- Tokio Sync: https://docs.rs/tokio/latest/tokio/sync/
- SIMD Alignment: https://doc.rust-lang.org/reference/type-layout.html#the-alignment-modifiers
- Atomic Operations: https://doc.rust-lang.org/std/sync/atomic/

### Key Concepts
- **Quantum-Inspired Memory**: Uses quantum mechanics metaphors (coherence, entanglement) for memory organization
- **SIMD Alignment**: 32-byte alignment enables AVX2 vectorized operations
- **Lock-Free Concurrency**: SkipMap and atomic operations avoid locks for better performance
- **Temporal Context**: Time-aware memory with causal links
- **Attention Mechanism**: Dynamic focus, salience, and novelty tracking
- **Pattern Matching**: Feature-based cognitive pattern recognition
- **Decision Engine**: Rule-based decision making with multiple outcomes

### Performance Considerations

1. **SIMD Operations**: Aligned structures enable 8× faster vector operations
2. **Lock-Free SkipMap**: O(log n) concurrent access without mutex contention
3. **Atomic Floats**: Lock-free float updates for high-frequency metrics
4. **MPSC Channels**: Efficient single-consumer queue for working memory
5. **RwLock for Rarely-Updated Data**: Used for quantum phase and fingerprints

### Special Attention Required

1. **Preserve SIMD Alignment**
   - `#[repr(align(32))]` must stay on aligned structures
   - Don't add padding or change field order

2. **Maintain Atomic Semantics**
   - All atomic operations use `Ordering::Relaxed`
   - Don't change ordering without understanding memory models

3. **Async/Await Boundaries**
   - Many methods are `async fn`
   - Maintain `.await` points correctly

4. **SkipMap Usage**
   - Entries() returns iterator, not Vec
   - Be careful with clone() on entry values

---

**Task Created:** 2024-10-19  
**Estimated Time:** 3-4 hours  
**Complexity:** High (quantum-inspired system, SIMD, lock-free)  
**Prerequisites:** Understanding of Rust async, SIMD, atomic operations, lock-free data structures, quantum-inspired computing concepts
