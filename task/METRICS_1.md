# Task: Cognitive Metrics Collection Wiring

**Status**: Ready for Execution
**Priority**: Medium
**Complexity**: Low

## Overview

Wire the fully-implemented `CognitiveStats` into the memory coordinator hotpath to enable automatic collection of cognitive operation metrics. CognitiveStats provides atomic counters for working memory access, long-term memory access, quantum operations, and attention updates - it just needs call sites in the memory operations.

## Objective

Enable transparent cognitive metrics collection by:
1. Adding `CognitiveStats` field to `MemoryCoordinator`
2. Recording metrics at key cognitive operation points
3. Providing access to metrics via public getter
4. Maintaining zero public API changes (metrics collection happens "automagically")

## Background: What's Already Built

### CognitiveStats (domain/memory/cognitive/types/memory_items.rs)

**Fully Implemented Features (lines 110-181):**

```rust
pub struct CognitiveStats {
    /// Working memory access count
    pub working_memory_accesses: AtomicU64,
    /// Long-term memory access count
    pub long_term_memory_accesses: AtomicU64,
    /// Quantum operations count
    pub quantum_operations: AtomicU64,
    /// Attention updates count
    pub attention_updates: AtomicU64,
    /// Last update timestamp
    pub last_update_nanos: AtomicU64,
}
```

**Available Methods:**
- `CognitiveStats::new()` - Create with all counters at zero (line 128)
- `record_working_memory_access()` - Increment working memory counter (line 141)
- `record_long_term_memory_access()` - Increment long-term memory counter (line 148)
- `record_quantum_operation()` - Increment quantum ops counter (line 155)
- `record_attention_update()` - Increment attention counter (line 163)
- `update_timestamp()` - Update last access time (line 170)

**Features:**
- All operations are atomic (Ordering::Relaxed)
- Thread-safe for concurrent access
- Zero-allocation design
- Timestamp tracking with unix_timestamp_nanos()

**Current Status**: ✅ Fully implemented, ❌ Not wired into coordinator

## Technical Details

### File: packages/candle/src/memory/core/manager/coordinator/lifecycle.rs

**Current Structure:**
```rust
pub(super) cognitive_state: Arc<RwLock<CognitiveState>>,
```

**Required Addition:**
```rust
use crate::domain::memory::cognitive::types::memory_items::CognitiveStats;

pub struct MemoryCoordinator {
    // ... existing fields ...
    pub(super) cognitive_state: Arc<RwLock<CognitiveState>>,
    pub(super) cognitive_stats: Arc<CognitiveStats>,
}
```

**Initialization in `new()`:**
```rust
cognitive_state: Arc::new(RwLock::new(CognitiveState::new())),
cognitive_stats: Arc::new(CognitiveStats::new()),
```

**Add Getter Method:**
```rust
impl MemoryCoordinator {
    /// Get cognitive performance statistics
    ///
    /// Returns atomic counters for cognitive operations:
    /// - Working memory accesses
    /// - Long-term memory accesses
    /// - Quantum operations
    /// - Attention updates
    pub fn cognitive_stats(&self) -> Arc<CognitiveStats> {
        Arc::clone(&self.cognitive_stats)
    }
}
```

### File: packages/candle/src/memory/core/manager/coordinator/operations.rs

**Metric Collection Point 1: After activation update (after line 188)**

```rust
// Record metrics after cognitive activation update
if let Some(ref embedding) = domain_memory.embedding {
    let stimulus = embedding.data.clone();
    match self.cognitive_state.write().await.update_activation_from_stimulus(stimulus) {
        Ok(()) => {
            log::trace!("Updated cognitive activation from memory retrieval: {}", memory_id);
            // Record attention update metric
            self.cognitive_stats.record_attention_update();
        }
        Err(e) => {
            log::warn!("Failed to update cognitive activation from memory retrieval: {}", e);
        }
    }
}
```

**Metric Collection Point 2: In get_memory() start (after line 163)**

```rust
pub async fn get_memory(&self, memory_id: &str) -> Result<MemoryNode> {
    // Record long-term memory access
    self.cognitive_stats.record_long_term_memory_access();

    // ... existing get_memory implementation ...
}
```

**Metric Collection Point 3: Quantum operations (if quantum routing active)**

Add after quantum signature operations (look for QuantumSignature usage):

```rust
// After any quantum operation
self.cognitive_stats.record_quantum_operation();
```

### File: packages/candle/src/domain/memory/cognitive/types/state.rs

**Metric Collection Point 4: Working memory operations**

The CognitiveState already has a `stats` field! Check line 59:

```rust
pub struct CognitiveState {
    // ... other fields ...
    stats: Arc<CognitiveStats>,
}
```

This means CognitiveState is ALREADY tracking stats internally. We need to:

1. **Expose the stats getter in CognitiveState** (add to impl block around line 105):

```rust
impl CognitiveState {
    /// Get cognitive statistics
    #[inline]
    #[must_use]
    pub fn stats(&self) -> &Arc<CognitiveStats> {
        &self.stats
    }
}
```

2. **Update activation method to record metrics** (line 395):

```rust
pub fn update_activation_from_stimulus(
    &mut self,
    stimulus: Vec<f32>,
) -> Result<(), CognitiveError> {
    // ... existing validation ...

    // Update activation pattern
    self.activation_pattern.update(stimulus);
    self.activation_pattern
        .apply_activation(|x| 1.0 / (1.0 + (-x).exp()));

    let energy = self.activation_pattern.energy();
    #[allow(clippy::cast_precision_loss)]
    let normalized_energy = (energy / self.activation_pattern.dimension as f32)
        .sqrt()
        .clamp(0.0, 1.0);

    // Update attention weights
    self.attention_weights.update_from_energy(normalized_energy);

    // Record metrics
    self.stats.record_attention_update();

    Ok(())
}
```

### File: packages/candle/src/memory/core/ops/retrieval/semantic.rs

**Metric Collection Point 5: Vector search operations**

After query embedding generation in retrieve():

```rust
// After generating query embedding
if let Some(ref cognitive_state) = cognitive_state {
    let stimulus = query_embedding.clone();
    match cognitive_state.write().await.update_activation_from_stimulus(stimulus) {
        Ok(()) => {
            log::trace!("Updated cognitive activation from query embedding");
            // Stats already recorded in update_activation_from_stimulus
        }
        Err(e) => {
            log::warn!("Failed to update cognitive activation from query: {}", e);
        }
    }
}
```

## Integration Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   Cognitive Metrics Flow                     │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  MemoryCoordinator.get_memory()                             │
│         │                                                     │
│         ├──> record_long_term_memory_access()               │
│         │                                                     │
│         └──> CognitiveState.update_activation()             │
│                    │                                          │
│                    └──> record_attention_update()            │
│                                                               │
│  CognitiveState.update_activation_from_stimulus()           │
│         │                                                     │
│         └──> record_attention_update()                       │
│                                                               │
│  Quantum Operations                                          │
│         │                                                     │
│         └──> record_quantum_operation()                      │
│                                                               │
│  Working Memory Operations                                   │
│         │                                                     │
│         └──> record_working_memory_access()                  │
│                                                               │
│  All metrics → CognitiveStats (AtomicU64 counters)          │
│         │                                                     │
│         └──> cognitive_stats() getter → User access          │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

## Implementation Checklist

### Phase 1: Expose CognitiveState Stats
- [ ] Add stats() getter to CognitiveState impl (state.rs)
- [ ] Update update_activation_from_stimulus() to record attention metric

### Phase 2: Add CognitiveStats to MemoryCoordinator
- [ ] Add import in lifecycle.rs: `use crate::domain::memory::cognitive::types::memory_items::CognitiveStats;`
- [ ] Add field: `pub(super) cognitive_stats: Arc<CognitiveStats>`
- [ ] Initialize in new(): `cognitive_stats: Arc::new(CognitiveStats::new())`
- [ ] Add cognitive_stats() public getter method

### Phase 3: Wire Metrics in operations.rs
- [ ] Record long_term_memory_access at start of get_memory()
- [ ] Record attention_update after activation stimulus update
- [ ] Search for quantum operations and add record_quantum_operation()

### Phase 4: Remove Dead Code Annotations
- [ ] Verify no #[allow(dead_code)] on CognitiveStats methods
- [ ] Check cargo check shows no dead_code warnings for memory_items.rs

### Phase 5: Verification
- [ ] Run `cargo check -p paraphym_candle --color=never 2>&1 | grep -i "cognitive.*stats\|dead_code"`
- [ ] Verify no new compilation errors introduced
- [ ] Verify metrics are collected transparently
- [ ] Confirm atomic operations work correctly

## Success Criteria

✅ CognitiveStats integrated into MemoryCoordinator
✅ Long-term memory accesses recorded in get_memory()
✅ Attention updates recorded in activation pipeline
✅ Quantum operations recorded (if quantum routing active)
✅ Stats accessible via cognitive_stats() getter
✅ Zero public API changes (stats collection is automatic)
✅ All changes transparent to existing code
✅ No new dead_code warnings
✅ Thread-safe atomic operations

## Non-Goals (Out of Scope)

❌ Adding tests or benchmarks
❌ Writing documentation
❌ Implementing new metrics
❌ Changing public API surface
❌ Fixing unrelated compilation errors
❌ Modifying CognitiveStats implementation
❌ Creating metrics dashboard (separate concern)

## Notes

- CognitiveStats is FULLY IMPLEMENTED with atomic counters
- CognitiveState ALREADY has stats field - just needs exposure
- All operations use Ordering::Relaxed (no synchronization overhead)
- Timestamp updates automatically on each metric recording
- Working memory metrics may need future wiring (not in current hotpath)
- Quantum operation metrics depend on quantum routing being active
- Stats are read-only from user perspective (atomic loads)
- Follows same integration pattern as STUB_2 activation processing
