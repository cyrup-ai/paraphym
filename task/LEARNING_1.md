# Task: Adaptive Learning Integration

**Status**: Ready for Execution
**Priority**: High
**Complexity**: Medium

## Overview

Wire the fully-implemented `CognitiveProcessor` and `PatternMatcher` into the memory coordinator to enable automatic adaptive learning from memory access patterns. The processor includes pattern matching with SIMD-optimized cosine similarity, decision engine with confidence scoring, and learning rate configuration - it just needs hotpath integration.

## Objective

Enable transparent adaptive pattern learning by:
1. Adding `CognitiveProcessor` field to `MemoryCoordinator`
2. Processing memory access patterns through the pattern matcher
3. Recording decisions and updating learned patterns automatically
4. Maintaining zero public API changes (everything happens "automagically")

## Background: What's Already Built

### CognitiveProcessor (domain/memory/cognitive/types/processor.rs)

**Fully Implemented Components:**

1. **CognitiveProcessor** (lines 30-42):
   - Configuration: batch_size, decision_threshold, learning_rate, max_iterations
   - ProcessingState: is_processing, current_iteration, start_time (atomic)
   - PatternMatcher with SIMD cosine similarity
   - DecisionEngine with confidence-based decisions
   - Pattern cache with SkipMap for O(log n) lookup

2. **PatternMatcher** (lines 108-117):
   - Threshold-based matching (default 0.8)
   - Stored reference patterns
   - Pattern cache (SkipMap<Uuid, f32>)
   - SIMD-optimized matching via cyrup_simd::cosine_similarity
   - Methods: add_pattern(), match_pattern(), cache_pattern_result()

3. **DecisionEngine** (lines 119-128):
   - Decision threshold (default 0.7)
   - Decision history via mpsc channel
   - Decision outcomes: Accept, Reject, Defer, RequestInfo
   - Confidence-based outcome selection

4. **CognitiveMemory** (lines 17-28):
   - Pattern storage with SkipMap for lock-free access
   - CognitiveMetrics tracking
   - Configuration for max_patterns, consolidation_threshold, retention

**Key Methods:**
- `CognitiveProcessor::process(input: &[f32])` - Main processing entry point
- `PatternMatcher::match_pattern(input)` - SIMD-optimized matching (lines 387-427)
- `DecisionEngine::make_decision(strength)` - Confidence-based decisions (lines 473-491)
- `CognitiveProcessor::clear_pattern_cache()` - Cache management

**Current Status**: ✅ Fully implemented, ❌ Not wired into coordinator

## Technical Details

### File: packages/candle/src/memory/core/manager/coordinator/lifecycle.rs

**Current Structure:**
```rust
pub(super) cognitive_state: Arc<RwLock<CognitiveState>>,
```

**Required Addition:**
```rust
use crate::domain::memory::cognitive::types::processor::{
    CognitiveProcessor, CognitiveProcessorConfig, CognitiveMemory, CognitiveMemoryConfig
};

pub struct MemoryCoordinator {
    // ... existing fields ...
    pub(super) cognitive_state: Arc<RwLock<CognitiveState>>,
    pub(super) cognitive_processor: Option<Arc<CognitiveProcessor>>,
    pub(super) cognitive_memory: Option<Arc<CognitiveMemory>>,
}
```

**Initialization in `new()`:**
```rust
cognitive_state: Arc::new(RwLock::new(CognitiveState::new())),
cognitive_processor: None, // Initialize as None
cognitive_memory: None,
```

**Add Constructor Methods:**
```rust
impl MemoryCoordinator {
    /// Enable adaptive learning with default configuration
    pub fn with_adaptive_learning(mut self) -> Self {
        let processor_config = CognitiveProcessorConfig::default();
        let memory_config = CognitiveMemoryConfig::default();

        self.cognitive_processor = Some(Arc::new(CognitiveProcessor::new(processor_config)));
        self.cognitive_memory = Some(Arc::new(CognitiveMemory::new(memory_config)));
        self
    }

    /// Enable adaptive learning with custom configuration
    pub fn with_adaptive_learning_config(
        mut self,
        processor_config: CognitiveProcessorConfig,
        memory_config: CognitiveMemoryConfig,
    ) -> Self {
        self.cognitive_processor = Some(Arc::new(CognitiveProcessor::new(processor_config)));
        self.cognitive_memory = Some(Arc::new(CognitiveMemory::new(memory_config)));
        self
    }
}
```

### File: packages/candle/src/memory/core/manager/coordinator/operations.rs

**Integration Point 1: After Cognitive Activation (after line 188)**

```rust
// Process memory access pattern through cognitive processor for adaptive learning
if let (Some(ref processor), Some(ref embedding)) = (&self.cognitive_processor, &domain_memory.embedding) {
    let stimulus = embedding.data.clone();

    // Process pattern and get decision
    match processor.process(&stimulus) {
        Ok(decision) => {
            log::trace!(
                "Processed memory pattern: id={}, confidence={:.3}, outcome={:?}",
                memory_id,
                decision.confidence,
                decision.outcome
            );

            // If decision confidence is high, store pattern in cognitive memory
            if decision.confidence >= 0.8 {
                if let Some(ref memory) = self.cognitive_memory {
                    let pattern = crate::domain::memory::cognitive::types::processor::CognitivePattern {
                        id: decision.id,
                        data: stimulus,
                        strength: decision.confidence,
                        access_count: 1,
                        last_access: std::time::SystemTime::now(),
                    };

                    match memory.store_pattern(pattern) {
                        Ok(()) => {
                            log::trace!("Stored learned pattern: decision_id={}", decision.id);
                        }
                        Err(e) => {
                            log::debug!("Failed to store pattern (may be at capacity): {}", e);
                        }
                    }
                }
            }
        }
        Err(e) => {
            log::warn!("Failed to process cognitive pattern: {}", e);
        }
    }
}
```

### File: packages/candle/src/memory/core/ops/retrieval/semantic.rs

**Current Structure:**
```rust
pub struct SemanticRetrieval {
    // ... existing fields ...
    cognitive_state: Option<Arc<RwLock<CognitiveState>>>,
}
```

**Add Processor Fields:**
```rust
use crate::domain::memory::cognitive::types::processor::{CognitiveProcessor, CognitiveMemory};

cognitive_processor: Option<Arc<CognitiveProcessor>>,
cognitive_memory: Option<Arc<CognitiveMemory>>,
```

**Update Constructor:**
```rust
pub fn new(/* ... existing params ... */) -> Self {
    Self {
        // ... existing fields ...
        cognitive_state: None,
        cognitive_processor: None,
        cognitive_memory: None,
    }
}

pub fn with_cognitive_processor(
    mut self,
    processor: Arc<CognitiveProcessor>,
    memory: Arc<CognitiveMemory>,
) -> Self {
    self.cognitive_processor = Some(processor);
    self.cognitive_memory = Some(memory);
    self
}
```

**Integration Point 2: Process Query Patterns (after query embedding generation)**

```rust
let cognitive_processor = self.cognitive_processor.clone();
let cognitive_memory = self.cognitive_memory.clone();

// ... in spawned task, after query embedding generation:

// Process query pattern for adaptive learning
if let Some(ref processor) = cognitive_processor {
    match processor.process(&query_embedding) {
        Ok(decision) => {
            log::trace!(
                "Processed query pattern: confidence={:.3}, outcome={:?}",
                decision.confidence,
                decision.outcome
            );

            // Store high-confidence query patterns
            if decision.confidence >= 0.8 {
                if let Some(ref memory) = cognitive_memory {
                    let pattern = crate::domain::memory::cognitive::types::processor::CognitivePattern {
                        id: decision.id,
                        data: query_embedding.clone(),
                        strength: decision.confidence,
                        access_count: 1,
                        last_access: std::time::SystemTime::now(),
                    };

                    let _ = memory.store_pattern(pattern);
                }
            }
        }
        Err(e) => {
            log::warn!("Failed to process query pattern: {}", e);
        }
    }
}
```

### New File: packages/candle/src/memory/core/manager/coordinator/learning.rs

**Create new module for learning operations:**

```rust
//! Adaptive learning operations for memory coordinator

use std::sync::Arc;
use crate::domain::memory::cognitive::types::processor::{
    CognitiveProcessor, CognitiveMemory, CognitivePattern
};
use crate::memory::utils::error::{Error, Result};

impl crate::memory::core::manager::MemoryCoordinator {
    /// Update learned patterns from successful memory retrievals
    ///
    /// Called periodically to reinforce patterns that led to successful retrievals
    pub(super) async fn update_learned_patterns(&self) -> Result<()> {
        if let (Some(ref processor), Some(ref memory)) = (&self.cognitive_processor, &self.cognitive_memory) {
            // Get metrics to assess learning performance
            let metrics = memory.metrics();
            let patterns_processed = metrics.patterns_processed.load(std::sync::atomic::Ordering::Relaxed);
            let decisions_made = metrics.decisions_made.load(std::sync::atomic::Ordering::Relaxed);

            log::debug!(
                "Learning statistics: patterns_processed={}, decisions_made={}",
                patterns_processed,
                decisions_made
            );

            // Clear pattern cache periodically to prevent memory growth
            let (cache_size, needs_cleanup) = processor.cache_performance();
            if needs_cleanup {
                log::debug!("Clearing pattern cache: size={}", cache_size);
                processor.clear_pattern_cache();
            }

            Ok(())
        } else {
            Ok(()) // Adaptive learning not enabled
        }
    }

    /// Get cognitive learning statistics
    pub fn learning_stats(&self) -> Option<LearningStats> {
        if let Some(ref memory) = self.cognitive_memory {
            let metrics = memory.metrics();
            Some(LearningStats {
                patterns_processed: metrics.patterns_processed.load(std::sync::atomic::Ordering::Relaxed),
                decisions_made: metrics.decisions_made.load(std::sync::atomic::Ordering::Relaxed),
                avg_processing_time_us: metrics.avg_processing_time_us.load(std::sync::atomic::Ordering::Relaxed),
                success_rate: metrics.success_rate.load(std::sync::atomic::Ordering::Relaxed),
            })
        } else {
            None
        }
    }
}

/// Learning performance statistics
#[derive(Debug, Clone, Copy)]
pub struct LearningStats {
    pub patterns_processed: u64,
    pub decisions_made: u64,
    pub avg_processing_time_us: u64,
    pub success_rate: f32,
}
```

**Add module to coordinator/mod.rs:**
```rust
pub(super) mod learning;
```

## Integration Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                  Adaptive Learning Pipeline                  │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  Memory Access ──> Extract Embedding Pattern                │
│                           │                                   │
│                           ▼                                   │
│                  CognitiveProcessor.process()                │
│                           │                                   │
│                           ├──> PatternMatcher                │
│                           │       │                           │
│                           │       ├──> SIMD Cosine Similarity │
│                           │       └──> Cache Lookup           │
│                           │                                   │
│                           └──> DecisionEngine                 │
│                                   │                           │
│                                   ├──> Confidence Score       │
│                                   └──> Accept/Reject/Defer    │
│                                             │                 │
│                                             ▼                 │
│                              High Confidence (≥0.8)?          │
│                                             │                 │
│                                        Yes  │  No             │
│                                             ▼   └──> Discard  │
│                                   CognitiveMemory             │
│                                             │                 │
│                                             └──> Store Pattern│
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

## Implementation Checklist

### Phase 1: Add Processor Fields to MemoryCoordinator
- [ ] Add imports in lifecycle.rs for CognitiveProcessor, CognitiveMemory, configs
- [ ] Add fields: `cognitive_processor: Option<Arc<CognitiveProcessor>>`
- [ ] Add fields: `cognitive_memory: Option<Arc<CognitiveMemory>>`
- [ ] Initialize in new(): both as None
- [ ] Add with_adaptive_learning() constructor
- [ ] Add with_adaptive_learning_config() constructor

### Phase 2: Wire Pattern Processing in operations.rs
- [ ] After cognitive activation (line 188), add processor check
- [ ] Call processor.process() with embedding data
- [ ] Log decision confidence and outcome
- [ ] Store high-confidence patterns (≥0.8) in cognitive_memory
- [ ] Add proper error handling with log::warn

### Phase 3: Wire Query Pattern Processing in semantic.rs
- [ ] Add cognitive_processor and cognitive_memory fields
- [ ] Update new() to initialize as None
- [ ] Add with_cognitive_processor() constructor
- [ ] Clone processor/memory before spawning task
- [ ] Process query_embedding through processor
- [ ] Store high-confidence query patterns

### Phase 4: Create Learning Operations Module
- [ ] Create learning.rs in coordinator directory
- [ ] Implement update_learned_patterns() method
- [ ] Implement learning_stats() method
- [ ] Add LearningStats struct
- [ ] Add module to coordinator/mod.rs

### Phase 5: Remove Dead Code Annotations
- [ ] Verify no #[allow(dead_code)] on CognitiveProcessor methods
- [ ] Verify no #[allow(dead_code)] on PatternMatcher methods
- [ ] Check DecisionEngine has no dead_code warnings

### Phase 6: Verification
- [ ] Run `cargo check -p paraphym_candle --color=never 2>&1 | grep -i "processor\|pattern.*matcher\|decision.*engine\|dead_code"`
- [ ] Verify no new compilation errors introduced
- [ ] Verify fields are Optional (no breaking changes)
- [ ] Confirm pattern processing works in hotpath

## Success Criteria

✅ CognitiveProcessor integrated into MemoryCoordinator
✅ Pattern matching runs automatically on memory access
✅ High-confidence patterns stored in CognitiveMemory
✅ Decision engine tracks confidence scores
✅ Learning statistics accessible via learning_stats()
✅ Zero public API changes (opt-in via with_adaptive_learning())
✅ All changes transparent to existing code
✅ No new dead_code warnings
✅ Proper error handling (no unwrap/expect)
✅ SIMD-optimized pattern matching active

## Non-Goals (Out of Scope)

❌ Adding tests or benchmarks
❌ Writing documentation
❌ Implementing new learning algorithms
❌ Changing public API surface
❌ Fixing unrelated compilation errors
❌ Modifying CognitiveProcessor implementation
❌ Pattern persistence (covered in PERSISTENCE_1.md)

## Notes

- CognitiveProcessor is FULLY IMPLEMENTED with SIMD optimization
- Default config: learning_rate=0.01, decision_threshold=0.7, batch_size=32
- Pattern cache uses SkipMap for lock-free O(log n) access
- Confidence threshold 0.8 for storing patterns (configurable)
- Pattern matching uses cyrup_simd::cosine_similarity (AVX2/NEON)
- Decision outcomes: Accept (≥threshold), Defer (≥threshold*0.5), Reject (<threshold*0.5)
- Follows same integration pattern as STUB_2 activation processing
