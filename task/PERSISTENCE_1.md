# Task: Cognitive State Persistence

**Status**: Ready for Execution
**Priority**: Medium
**Complexity**: Medium

## Overview

Implement save/load functionality for cognitive state to enable persistent learning across sessions. All cognitive types already have `Serialize` and `Deserialize` derives - we just need to implement the persistence layer using serde_json and tokio::fs.

## Objective

Enable automatic cognitive state persistence by:
1. Implementing save_state() method to persist cognitive state to disk
2. Implementing load_state() method to restore cognitive state from disk
3. Creating SerializableCognitiveState wrapper for atomic types
4. Auto-saving state periodically or on shutdown
5. Maintaining zero public API changes (persistence happens "automagically")

## Background: What's Already Serializable

### Types with Serialize/Deserialize Derives

From grep results in domain/memory/cognitive/types/:

1. **AlignedActivationPattern** (activation.rs:7)
   - Has: `#[derive(Debug, Clone, Serialize, Deserialize)]`
   - Fields: data (Vec<f32>), dimension, last_update (SystemTime)

2. **TemporalContext** (temporal.rs:8)
   - Has: `#[derive(Debug, Clone, Serialize, Deserialize)]`
   - Fields: window_start, window_end, causal_links

3. **CausalLink** (temporal.rs:84)
   - Has: `#[derive(Debug, Clone, Serialize, Deserialize)]`
   - Fields: from_id, to_id, strength, link_type

4. **QuantumSignature** (quantum.rs:35)
   - Has: `#[derive(Debug, Clone, Serialize, Deserialize)]`
   - Fields: coherence, entropy, collapse_probability, phase, entanglement_bonds

5. **EntanglementBond** (quantum.rs:353)
   - Has: `#[derive(Debug, Clone, Serialize, Deserialize)]`
   - Fields: target_id, bond_strength, entanglement_type, created_at

6. **EntanglementType** (quantum.rs:381)
   - Has: `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]`
   - Enum variants: Semantic, Temporal, Causal, Emergent, Werner, Weak

7. **WorkingMemoryItem** (memory_items.rs:11)
   - Has: `#[derive(Debug, Clone, Serialize, Deserialize)]`
   - Fields: id, content, activation, created_at, ttl, access_count, is_transient

8. **CognitiveMemoryEntry** (memory_items.rs:53)
   - Has: `#[derive(Debug, Clone, Serialize, Deserialize)]`
   - Fields: content, strength, access_count, last_access, decay_rate

9. **CognitiveProcessor Config Types** (processor.rs):
   - CognitiveMemoryConfig (line 44)
   - CognitiveProcessorConfig (line 57)
   - CognitivePattern (line 70)
   - Decision (line 131)
   - DecisionOutcome (line 144)

### Types Needing Serialization Wrappers

**CognitiveState** (state.rs) - NOT directly serializable due to Arc/Mutex/Atomic types:
- activation_pattern: AlignedActivationPattern ✅
- attention_weights: Arc<AtomicAttentionWeights> ❌ (atomic fields)
- working_memory: Arc<(mpsc, Mutex)> ❌ (channels/mutexes)
- long_term_memory: Arc<SkipMap> ❌ (concurrent structure)
- temporal_context: Arc<TemporalContext> ✅
- quantum_signature: Arc<QuantumSignature> ✅
- uncertainty: Arc<AtomicF32> ❌
- confidence: Arc<AtomicF32> ❌
- meta_awareness: Arc<AtomicF32> ❌
- stats: Arc<CognitiveStats> ❌

**Strategy:** Create a SerializableCognitiveState that extracts values from atomic types

## Technical Details

### New File: packages/candle/src/domain/memory/cognitive/persistence.rs

```rust
//! Cognitive state persistence using serde_json

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Deserialize, Serialize};

use super::types::{
    AlignedActivationPattern, TemporalContext, QuantumSignature,
    CognitiveState, CognitiveError, CognitiveResult,
};

/// Serializable snapshot of cognitive state
///
/// Extracts values from atomic/concurrent types into plain serializable fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableCognitiveState {
    /// Activation pattern (already serializable)
    pub activation_pattern: AlignedActivationPattern,

    /// Attention weights snapshot
    pub attention_weights: AttentionWeightsSnapshot,

    /// Temporal context (already serializable via Arc deref)
    pub temporal_context: TemporalContext,

    /// Quantum signature (already serializable via Arc deref)
    pub quantum_signature: QuantumSignature,

    /// Uncertainty value snapshot
    pub uncertainty: f32,

    /// Confidence value snapshot
    pub confidence: f32,

    /// Meta-awareness value snapshot
    pub meta_awareness: f32,

    /// Statistics snapshot
    pub stats: CognitiveStatsSnapshot,
}

/// Snapshot of atomic attention weights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionWeightsSnapshot {
    pub primary: f32,
    pub secondary: f32,
    pub background: f32,
    pub meta: f32,
}

/// Snapshot of atomic statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveStatsSnapshot {
    pub working_memory_accesses: u64,
    pub long_term_memory_accesses: u64,
    pub quantum_operations: u64,
    pub attention_updates: u64,
    pub last_update_nanos: u64,
}

impl SerializableCognitiveState {
    /// Create snapshot from live cognitive state
    pub fn from_cognitive_state(state: &CognitiveState) -> Self {
        Self {
            activation_pattern: state.activation_pattern().clone(),
            attention_weights: AttentionWeightsSnapshot {
                primary: state.attention_weights().primary(),
                secondary: state.attention_weights().secondary(),
                background: state.attention_weights().background(),
                meta: state.attention_weights().meta(),
            },
            temporal_context: (**state.temporal_context()).clone(),
            quantum_signature: (**state.quantum_signature()).clone(),
            uncertainty: state.uncertainty(),
            confidence: state.confidence(),
            meta_awareness: state.meta_awareness(),
            stats: CognitiveStatsSnapshot {
                working_memory_accesses: state.stats()
                    .working_memory_accesses
                    .load(Ordering::Relaxed),
                long_term_memory_accesses: state.stats()
                    .long_term_memory_accesses
                    .load(Ordering::Relaxed),
                quantum_operations: state.stats()
                    .quantum_operations
                    .load(Ordering::Relaxed),
                attention_updates: state.stats()
                    .attention_updates
                    .load(Ordering::Relaxed),
                last_update_nanos: state.stats()
                    .last_update_nanos
                    .load(Ordering::Relaxed),
            },
        }
    }

    /// Restore cognitive state from snapshot
    ///
    /// Creates new CognitiveState with quantum signature from snapshot,
    /// then restores atomic values.
    pub fn to_cognitive_state(&self) -> CognitiveState {
        let mut state = CognitiveState::new_with_quantum_signature(self.quantum_signature.clone());

        // Restore attention weights
        state.attention_weights().set_primary(self.attention_weights.primary);
        state.attention_weights().set_secondary(self.attention_weights.secondary);
        state.attention_weights().set_background(self.attention_weights.background);
        state.attention_weights().set_meta(self.attention_weights.meta);

        // Restore uncertainty/confidence/meta-awareness
        state.set_uncertainty(self.uncertainty);
        state.set_confidence(self.confidence);
        state.set_meta_awareness(self.meta_awareness);

        // Restore activation pattern
        state.set_activation_pattern(self.activation_pattern.clone());

        // Stats are restored via new state (counters reset to zero)
        // This is intentional - stats track current session performance

        state
    }
}

/// Save cognitive state to file
///
/// Serializes state to JSON and writes atomically using temp file + rename
pub async fn save_cognitive_state(
    state: &CognitiveState,
    path: impl AsRef<Path>,
) -> CognitiveResult<()> {
    let path = path.as_ref();

    // Create snapshot
    let snapshot = SerializableCognitiveState::from_cognitive_state(state);

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&snapshot).map_err(|e| {
        CognitiveError::OperationFailed(format!("Failed to serialize cognitive state: {}", e))
    })?;

    // Write to temporary file
    let temp_path = path.with_extension("tmp");
    let mut file = tokio::fs::File::create(&temp_path).await.map_err(|e| {
        CognitiveError::OperationFailed(format!("Failed to create temp file: {}", e))
    })?;

    file.write_all(json.as_bytes()).await.map_err(|e| {
        CognitiveError::OperationFailed(format!("Failed to write cognitive state: {}", e))
    })?;

    file.flush().await.map_err(|e| {
        CognitiveError::OperationFailed(format!("Failed to flush cognitive state: {}", e))
    })?;

    // Atomic rename
    tokio::fs::rename(&temp_path, path).await.map_err(|e| {
        CognitiveError::OperationFailed(format!("Failed to rename temp file: {}", e))
    })?;

    log::info!("Saved cognitive state to: {}", path.display());
    Ok(())
}

/// Load cognitive state from file
///
/// Reads JSON file and deserializes to CognitiveState
pub async fn load_cognitive_state(path: impl AsRef<Path>) -> CognitiveResult<CognitiveState> {
    let path = path.as_ref();

    // Read file
    let mut file = tokio::fs::File::open(path).await.map_err(|e| {
        CognitiveError::OperationFailed(format!("Failed to open cognitive state file: {}", e))
    })?;

    let mut json = String::new();
    file.read_to_string(&mut json).await.map_err(|e| {
        CognitiveError::OperationFailed(format!("Failed to read cognitive state: {}", e))
    })?;

    // Deserialize
    let snapshot: SerializableCognitiveState = serde_json::from_str(&json).map_err(|e| {
        CognitiveError::OperationFailed(format!("Failed to deserialize cognitive state: {}", e))
    })?;

    // Restore state
    let state = snapshot.to_cognitive_state();

    log::info!("Loaded cognitive state from: {}", path.display());
    Ok(state)
}

/// Try to load cognitive state, return new state if file doesn't exist
pub async fn load_or_new_cognitive_state(path: impl AsRef<Path>) -> CognitiveState {
    match load_cognitive_state(&path).await {
        Ok(state) => {
            log::info!("Restored cognitive state from disk");
            state
        }
        Err(e) => {
            log::info!("Creating new cognitive state (load failed: {})", e);
            CognitiveState::new()
        }
    }
}
```

### Update: packages/candle/src/domain/memory/cognitive/mod.rs

Add persistence module:

```rust
pub mod persistence;
pub mod types;

// Re-export persistence functions
pub use persistence::{
    save_cognitive_state, load_cognitive_state, load_or_new_cognitive_state,
    SerializableCognitiveState,
};
```

### File: packages/candle/src/domain/memory/cognitive/types/state.rs

**Add Missing Getters and Setters:**

```rust
impl CognitiveState {
    // ... existing methods ...

    /// Get activation pattern
    #[inline]
    #[must_use]
    pub fn activation_pattern(&self) -> &AlignedActivationPattern {
        &self.activation_pattern
    }

    /// Set activation pattern
    #[inline]
    pub fn set_activation_pattern(&mut self, pattern: AlignedActivationPattern) {
        self.activation_pattern = pattern;
    }

    /// Get attention weights
    #[inline]
    #[must_use]
    pub fn attention_weights(&self) -> &Arc<AtomicAttentionWeights> {
        &self.attention_weights
    }

    /// Get temporal context
    #[inline]
    #[must_use]
    pub fn temporal_context(&self) -> &Arc<TemporalContext> {
        &self.temporal_context
    }

    /// Set uncertainty
    #[inline]
    pub fn set_uncertainty(&self, value: f32) {
        self.uncertainty.store(value, Ordering::Relaxed);
    }

    /// Set confidence
    #[inline]
    pub fn set_confidence(&self, value: f32) {
        self.confidence.store(value, Ordering::Relaxed);
    }

    /// Set meta-awareness
    #[inline]
    pub fn set_meta_awareness(&self, value: f32) {
        self.meta_awareness.store(value, Ordering::Relaxed);
    }
}
```

### File: packages/candle/src/domain/memory/cognitive/types/attention.rs

**Add Setters for AtomicAttentionWeights:**

```rust
impl AtomicAttentionWeights {
    // ... existing methods ...

    /// Set primary attention weight
    #[inline]
    pub fn set_primary(&self, value: f32) {
        self.primary.store(value, Ordering::Relaxed);
    }

    /// Set secondary attention weight
    #[inline]
    pub fn set_secondary(&self, value: f32) {
        self.secondary.store(value, Ordering::Relaxed);
    }

    /// Set background attention weight
    #[inline]
    pub fn set_background(&self, value: f32) {
        self.background.store(value, Ordering::Relaxed);
    }

    /// Set meta attention weight
    #[inline]
    pub fn set_meta(&self, value: f32) {
        self.meta.store(value, Ordering::Relaxed);
    }
}
```

### File: packages/candle/src/memory/core/manager/coordinator/lifecycle.rs

**Add Persistence Configuration:**

```rust
use crate::domain::memory::cognitive::persistence::load_or_new_cognitive_state;

impl MemoryCoordinator {
    /// Create new memory coordinator with persistent cognitive state
    ///
    /// Loads cognitive state from disk if file exists, otherwise creates new state
    pub async fn with_persistent_cognitive_state(
        mut self,
        persistence_path: impl AsRef<std::path::Path>,
    ) -> Self {
        let state = load_or_new_cognitive_state(persistence_path).await;
        self.cognitive_state = Arc::new(RwLock::new(state));
        self
    }
}
```

### File: packages/candle/src/memory/core/manager/coordinator/operations.rs

**Add Auto-Save on Shutdown:**

```rust
use crate::domain::memory::cognitive::persistence::save_cognitive_state;

impl MemoryCoordinator {
    /// Save cognitive state to disk
    ///
    /// Should be called before coordinator shutdown to persist learned patterns
    pub async fn save_cognitive_state(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<()> {
        let state = self.cognitive_state.read().await;
        save_cognitive_state(&*state, path)
            .await
            .map_err(|e| Error::Internal(format!("Failed to save cognitive state: {}", e)))
    }
}
```

## Integration Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                Cognitive State Persistence                   │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  On Startup:                                                 │
│    MemoryCoordinator::with_persistent_cognitive_state()     │
│         │                                                     │
│         └──> load_or_new_cognitive_state()                  │
│                    │                                          │
│                    ├──> File exists?                         │
│                    │     │                                    │
│                    │     ├─Yes─> load_cognitive_state()      │
│                    │     │           │                        │
│                    │     │           ├─> Read JSON file       │
│                    │     │           ├─> Deserialize snapshot│
│                    │     │           └─> Restore CognitiveState│
│                    │     │                                    │
│                    │     └─No──> CognitiveState::new()       │
│                    │                                          │
│                    └──> Return initialized state             │
│                                                               │
│  On Shutdown / Periodic:                                     │
│    MemoryCoordinator::save_cognitive_state()                │
│         │                                                     │
│         └──> save_cognitive_state()                         │
│                    │                                          │
│                    ├──> Create SerializableCognitiveState   │
│                    │      (extract atomic values)            │
│                    ├──> Serialize to JSON                    │
│                    ├──> Write to temp file                   │
│                    └──> Atomic rename                        │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

## Implementation Checklist

### Phase 1: Create Persistence Module
- [ ] Create persistence.rs in domain/memory/cognitive/
- [ ] Implement SerializableCognitiveState struct
- [ ] Implement AttentionWeightsSnapshot struct
- [ ] Implement CognitiveStatsSnapshot struct
- [ ] Implement from_cognitive_state() method
- [ ] Implement to_cognitive_state() method

### Phase 2: Implement Save/Load Functions
- [ ] Implement save_cognitive_state() with atomic temp file write
- [ ] Implement load_cognitive_state() with error handling
- [ ] Implement load_or_new_cognitive_state() convenience function
- [ ] Add module to cognitive/mod.rs with re-exports

### Phase 3: Add Missing Getters/Setters
- [ ] Add activation_pattern() getter to CognitiveState
- [ ] Add set_activation_pattern() to CognitiveState
- [ ] Add attention_weights() getter to CognitiveState
- [ ] Add temporal_context() getter to CognitiveState
- [ ] Add set_uncertainty/confidence/meta_awareness() to CognitiveState
- [ ] Add set_primary/secondary/background/meta() to AtomicAttentionWeights

### Phase 4: Wire Persistence into MemoryCoordinator
- [ ] Add with_persistent_cognitive_state() constructor to lifecycle.rs
- [ ] Add save_cognitive_state() method to operations.rs
- [ ] Add import for persistence functions

### Phase 5: Verification
- [ ] Run `cargo check -p paraphym_candle --color=never 2>&1 | grep -i "persistence\|serialize\|dead_code"`
- [ ] Verify no new compilation errors introduced
- [ ] Verify SerializableCognitiveState can round-trip
- [ ] Confirm file I/O uses atomic writes

## Success Criteria

✅ Persistence module created with serialization wrappers
✅ save_cognitive_state() writes JSON atomically
✅ load_cognitive_state() restores state from JSON
✅ load_or_new_cognitive_state() handles missing files gracefully
✅ All atomic values extracted/restored correctly
✅ with_persistent_cognitive_state() loads state on startup
✅ save_cognitive_state() method available on coordinator
✅ Zero public API changes (persistence is opt-in)
✅ Proper error handling (no unwrap/expect)
✅ Atomic file writes (temp + rename)

## Non-Goals (Out of Scope)

❌ Adding tests or benchmarks
❌ Writing documentation
❌ Implementing auto-save timers (future enhancement)
❌ Compressing state files
❌ Versioning/migration logic
❌ Fixing unrelated compilation errors
❌ Persisting working memory queue (ephemeral by design)
❌ Persisting long-term memory SkipMap (separate concern)

## Notes

- All cognitive types ALREADY have Serialize/Deserialize derives
- Atomic types require value extraction before serialization
- Working memory and long-term memory are NOT persisted (transient data)
- Stats counters reset to zero on load (track current session only)
- Uses serde_json for human-readable state files
- Atomic writes prevent corruption (temp file + rename pattern)
- CognitiveState::new_with_quantum_signature() already exists (line 367)
- Persistence is opt-in via with_persistent_cognitive_state()
- Follows same pattern as other optional features (multimodal, learning)
