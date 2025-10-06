# STUB_7: Cognitive State Fields Integration (LOW PRIORITY)

**Priority:** 🔵 LOW  
**Severity:** Future Feature  
**Estimated Effort:** 1 session

## OBJECTIVE

Integrate unused `activation_pattern` and `temporal_context` fields into CognitiveState operations, preparing architecture for future cognitive AI feature activation.

## BACKGROUND

Two fields in CognitiveState are defined but unused:
- `activation_pattern`: AlignedActivationPattern for SIMD-optimized neural activations
- `temporal_context`: Arc<CachePadded<TemporalContext>> for time-aware memory operations

Both marked `#[allow(dead_code)]` with TODO comments indicating future cognitive system implementation.

## LOCATION

**File:** `packages/candle/src/domain/memory/cognitive/types.rs`  
**Lines:** 28, 41

## SUBTASK 1: Implement Activation Pattern Updates

**What:** Add method to update activation patterns from stimulus  
**Where:** `impl CognitiveState` block

**Why:** activation_pattern field needs integration point for cognitive updates

**Implementation:**
```rust
impl CognitiveState {
    /// Update activation pattern from external stimulus
    ///
    /// Applies stimulus vector to activation pattern and updates
    /// attention weights based on resulting activation energy.
    ///
    /// # Arguments
    ///
    /// * `stimulus` - Input activation values (must match dimension)
    ///
    /// # Returns
    ///
    /// `Ok(())` if update successful, error if dimension mismatch
    pub fn update_activation(&mut self, stimulus: &[f32]) -> Result<(), CognitiveError> {
        // Validate dimension
        if stimulus.len() != self.activation_pattern.dimension {
            return Err(CognitiveError::DimensionMismatch {
                expected: self.activation_pattern.dimension,
                got: stimulus.len(),
            });
        }
        
        // Update activation pattern
        self.activation_pattern.update(stimulus.to_vec());
        
        // Apply activation function (sigmoid for bounded output)
        self.activation_pattern.apply_activation(|x| {
            1.0 / (1.0 + (-x).exp()) // Sigmoid: maps to [0, 1]
        });
        
        // Update attention weights based on activation energy
        let energy = self.activation_pattern.energy();
        self.update_attention_from_activation(energy)?;
        
        Ok(())
    }
    
    /// Update attention weights based on activation energy
    fn update_attention_from_activation(&self, energy: f32) -> Result<(), CognitiveError> {
        // Normalize energy to attention weight [0, 1]
        let attention = (energy / self.activation_pattern.dimension as f32)
            .clamp(0.0, 1.0);
        
        // Store in atomic attention weights
        // (Implementation depends on AtomicAttentionWeights structure)
        self.attention_weights.update_from_energy(attention);
        
        Ok(())
    }
}
```

## SUBTASK 2: Implement Temporal Context Usage

**What:** Add memory consolidation decisions based on temporal context  
**Where:** New methods in `impl CognitiveState`

**Why:** temporal_context field needs integration for time-aware memory operations

**Implementation:**
```rust
impl CognitiveState {
    /// Check if working memory item should be consolidated to long-term memory
    ///
    /// Uses temporal context to determine if memory is old enough and
    /// accessed frequently enough for long-term storage.
    ///
    /// # Arguments
    ///
    /// * `memory` - Working memory item to evaluate
    ///
    /// # Returns
    ///
    /// `true` if should consolidate, `false` otherwise
    pub fn should_consolidate_to_longterm(&self, memory: &WorkingMemoryItem) -> bool {
        // Get current temporal context
        let current_time = SystemTime::now();
        
        // Calculate age since creation
        let age = memory.created_at
            .duration_since(self.temporal_context.current_window_start)
            .unwrap_or(Duration::ZERO);
        
        // Consolidate if:
        // 1. Aged beyond consolidation threshold (5 minutes)
        // 2. Access count indicates frequent use (> 3 accesses)
        // 3. Not marked as transient
        age > Duration::from_secs(300) 
            && memory.access_count > 3
            && !memory.is_transient
    }
    
    /// Update temporal context window
    ///
    /// Slides temporal window forward and updates causal dependencies.
    pub fn update_temporal_window(&mut self) {
        let mut context = Arc::make_mut(&self.temporal_context);
        context.slide_window();
    }
    
    /// Add causal link with temporal awareness
    ///
    /// Links two memories with temporal distance calculation.
    pub fn add_temporal_causal_link(
        &mut self,
        source_id: Uuid,
        target_id: Uuid,
        strength: f32,
    ) {
        let temporal_distance = self.calculate_temporal_distance(source_id, target_id);
        
        let mut context = Arc::make_mut(&self.temporal_context);
        context.add_causal_dependency(CausalLink::new(
            source_id,
            target_id,
            strength,
            temporal_distance,
        ));
    }
    
    fn calculate_temporal_distance(&self, source_id: Uuid, target_id: Uuid) -> i64 {
        // Calculate temporal distance between two memory entries
        // (Implementation depends on memory timestamp tracking)
        0 // Placeholder - would need actual timestamp lookup
    }
}
```

## SUBTASK 3: Add CognitiveError Variants

**What:** Add error types for cognitive operations  
**Where:** CognitiveError enum definition

**Why:** Need proper error handling for new methods

**Implementation:**
```rust
#[derive(Debug, thiserror::Error)]
pub enum CognitiveError {
    #[error("Dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },
    
    #[error("Invalid activation energy: {0}")]
    InvalidEnergy(f32),
    
    #[error("Temporal context error: {0}")]
    TemporalError(String),
    
    // ... other variants
}
```

## SUBTASK 4: Add AtomicAttentionWeights Method

**What:** Implement attention weight update from energy  
**Where:** `impl AtomicAttentionWeights` block

**Why:** New `update_attention_from_activation` needs this integration point

**Implementation:**
```rust
impl AtomicAttentionWeights {
    /// Update attention weights from normalized energy value
    ///
    /// # Arguments
    ///
    /// * `energy` - Normalized energy value [0, 1]
    pub fn update_from_energy(&self, energy: f32) {
        // Store energy as attention weight
        // (Implementation depends on AtomicAttentionWeights internals)
        self.store_weight(energy);
    }
}
```

## SUBTASK 5: Remove TODO Comments

**What:** Remove TODO markers from field attributes  
**Where:** Lines 28 and 41

**Changes:**
```rust
// OLD:
#[allow(dead_code)] // TODO: Implement in cognitive state system
activation_pattern: AlignedActivationPattern,

// NEW:
/// SIMD-aligned activation pattern for parallel processing
///
/// Updated via `update_activation()` when processing stimuli.
/// Used for attention weight calculations and cognitive state updates.
activation_pattern: AlignedActivationPattern,
```

```rust
// OLD:
#[allow(dead_code)] // TODO: Implement in cognitive state system
temporal_context: Arc<CachePadded<TemporalContext>>,

// NEW:
/// Temporal context with optimized time operations
///
/// Used for memory consolidation decisions and causal link tracking.
/// Updated via `update_temporal_window()` and temporal methods.
temporal_context: Arc<CachePadded<TemporalContext>>,
```

Note: Keep `#[allow(dead_code)]` until cognitive system is fully activated.

## SUBTASK 6: Add Integration Documentation

**What:** Document cognitive system activation requirements  
**Where:** Top of cognitive/types.rs or in module documentation

**Implementation:**
```rust
// Cognitive State System Integration
// ====================================
// 
// This module provides infrastructure for future cognitive AI features.
// Current status: Architecture ready, awaiting full activation.
//
// Integration points:
// 1. Call `update_activation()` when processing neural stimuli
// 2. Use `should_consolidate_to_longterm()` in memory management
// 3. Call `update_temporal_window()` periodically for time-awareness
// 4. Use `add_temporal_causal_link()` for causal memory relationships
//
// Activation requirements:
// - Define neural stimulus source
// - Implement attention weight interpretation
// - Integrate with memory consolidation system
// - Add temporal memory tracking
//
// When ready, remove #[allow(dead_code)] and integrate with agent system.
```

## SUBTASK 7: Add WorkingMemoryItem Fields

**What:** Ensure WorkingMemoryItem has required fields for integration  
**Where:** WorkingMemoryItem struct definition

**Check/Add:**
```rust
pub struct WorkingMemoryItem {
    pub id: Uuid,
    pub content: Vec<u8>,
    pub created_at: SystemTime,
    pub access_count: usize,     // Add if missing
    pub is_transient: bool,      // Add if missing
    // ... other fields
}
```

## DEFINITION OF DONE

- [ ] `update_activation()` method implemented
- [ ] `should_consolidate_to_longterm()` method implemented
- [ ] `update_temporal_window()` method implemented
- [ ] `add_temporal_causal_link()` method implemented
- [ ] CognitiveError enum has required variants
- [ ] AtomicAttentionWeights has `update_from_energy()` method
- [ ] TODO comments removed from field attributes
- [ ] Field documentation updated with usage information
- [ ] Integration documentation added to module
- [ ] WorkingMemoryItem has required fields
- [ ] `#[allow(dead_code)]` retained (not yet activated)
- [ ] Code compiles without warnings

## REQUIREMENTS

- ❌ **NO TESTS** - Testing team handles test coverage
- ❌ **NO BENCHMARKS** - Performance team handles benchmarking
- ✅ **PRODUCTION CODE ONLY** - Complete implementation, no stubs
- ⚠️ **FUTURE ACTIVATION** - Architecture ready but not yet integrated

## RESEARCH NOTES

### Cognitive Architecture

This implementation provides foundation for:
- Neural activation patterns (SIMD-optimized)
- Temporal memory consolidation
- Attention mechanisms
- Causal reasoning

Based on cognitive science principles:
- Working memory → Long-term memory consolidation
- Activation energy → Attention weights
- Temporal context → Causal relationships

### Activation Functions

Common choices:
- Sigmoid: `1 / (1 + exp(-x))` - Bounded [0, 1]
- ReLU: `max(0, x)` - Unbounded, faster
- Tanh: `tanh(x)` - Bounded [-1, 1]

Sigmoid chosen for initial implementation (bounded attention weights).

### Memory Consolidation

Consolidation criteria:
- **Age**: 5+ minutes in working memory
- **Access frequency**: 3+ accesses (indicates importance)
- **Transience**: Not marked as temporary

Based on cognitive psychology research on memory systems.

### Temporal Distance

Temporal distance affects causal strength:
- Recent events: Strong causal links
- Distant events: Weak causal links
- Measured in seconds or sequence numbers

### Integration with Agent System

When cognitive features activate:
```rust
// In agent processing loop:
agent.cognitive_state.update_activation(&stimulus);
agent.cognitive_state.update_temporal_window();

// In memory management:
for item in working_memory {
    if cognitive_state.should_consolidate_to_longterm(item) {
        consolidate_to_longterm(item);
    }
}
```

## VERIFICATION

After implementation, verify:
1. Methods compile and type-check correctly
2. Activation updates don't panic on valid input
3. Dimension mismatches caught with clear errors
4. Temporal consolidation logic is sound
5. Documentation explains integration requirements
6. Fields remain `#[allow(dead_code)]` (not yet used)
7. No breaking changes to existing cognitive types

## FUTURE ACTIVATION CHECKLIST

When ready to activate cognitive AI features:
- [ ] Define neural stimulus source (model outputs, embeddings)
- [ ] Implement attention weight interpretation
- [ ] Integrate with memory management system
- [ ] Add temporal tracking to memory entries
- [ ] Remove `#[allow(dead_code)]` from fields
- [ ] Call methods from agent processing loop
- [ ] Add cognitive system tests
- [ ] Benchmark cognitive operations
- [ ] Document cognitive AI capabilities
