# MEMORY_DECAY_WORKER: Background Temporal Decay Worker

## Context

The current temporal decay system in `temporal.rs` uses **lazy evaluation** - decay is only calculated when memories are retrieved via `search_memories()` or `get_memory()`. This is too expensive for high-frequency read operations and doesn't proactively manage memory importance over time.

### Current Architecture (Verified by Code Reading)

**Existing Decay Logic** (`temporal.rs:13-59`):
```rust
pub(super) async fn apply_temporal_decay(
    &self,
    memory: &mut crate::domain::memory::primitives::node::MemoryNode,
) -> Result<()> {
    // Calculate age from created_at to now
    let age = now.signed_duration_since(created_time);
    let days_old = age.num_seconds() as f64 / 86400.0;

    // Apply exponential decay: e^(-decay_rate * days)
    let decay = (-self.decay_rate * days_old).exp();

    // Update importance (min threshold 0.01)
    let new_importance = (current_importance * decay as f32).max(0.01);
    memory.set_importance(new_importance)?;

    // Update quantum coherence level
    quantum_state.coherence_level = (quantum_state.coherence_level * decay).max(0.01);
}
```

**Current Call Sites**:
1. `search.rs:63-68` - Applied to all search results before filtering
2. `operations.rs:183-184` - Applied during `get_memory()` before returning

**Problem**: Both call sites are in the read path, making every memory retrieval more expensive.

### Existing Worker Infrastructure (Verified by Code Reading)

The codebase ALREADY has a worker pattern we can leverage:

**CognitiveWorker Pattern** (`cognitive_worker/worker_core.rs:46-73`):
```rust
pub async fn run(&self) {
    loop {
        match self.queue.dequeue().await {
            Ok(task) => self.process_task(task).await,
            Err(e) => break, // Channel disconnected - exit worker
        }
    }
}
```

**Spawning** (`coordinator/lifecycle.rs:65-80`):
```rust
for worker_id in 0..num_workers {
    tokio::spawn(async move {
        log::info!("Cognitive worker {} started", worker_id);
        worker.run().await;
        log::info!("Cognitive worker {} stopped", worker_id);
    });
}
```

## Objective

Create a **background decay worker** that:
1. Runs continuously while the application is running (NOT a cron job)
2. Proactively applies temporal decay to memories based on age thresholds
3. Processes memories in batches to avoid overwhelming the system
4. Persists importance changes to SurrealDB
5. Uses reasonable thresholds to minimize redundant processing

## Architecture Design

### DecayWorker Structure

```rust
/// Background worker for proactive temporal decay
pub struct DecayWorker {
    /// Memory manager for database access
    memory_manager: Arc<SurrealDBMemoryManager>,
    /// Coordinator for decay logic
    coordinator: Arc<MemoryCoordinator>,
    /// Worker configuration
    config: DecayWorkerConfig,
    /// Pagination cursor (tracks position in memory list)
    cursor: Arc<AtomicUsize>,
}
```

### Configuration

```rust
pub struct DecayWorkerConfig {
    /// How long to sleep between decay cycles (default: 60 minutes)
    pub cycle_interval_secs: u64,
    /// How many memories to process per batch (default: 500)
    pub batch_size: usize,
    /// Minimum age before applying decay (default: 24 hours)
    /// Prevents thrashing on very fresh memories
    pub min_age_hours: u64,
    /// Maximum age to process in one cycle (default: 30 days)
    /// Older memories already have minimal importance
    pub max_age_days: u64,
}
```

### Worker Loop Strategy

**Continuous Rotation Pattern**:
1. Wake every N minutes (configurable)
2. Fetch next batch of memories using pagination cursor
3. Filter memories by age criteria
4. Apply decay to qualifying memories
5. Persist importance updates to database
6. Advance cursor, wrap around when reaching end
7. Sleep and repeat

**Why Rotation Instead of Staleness Tracking**:
- No need for new "last_decayed_at" field
- Eventually processes all memories
- Simple cursor-based state (atomic usize)
- Natural rate limiting via batch_size

## Implementation Plan

### Phase 1: Create DecayWorker Module

**File**: `packages/candle/src/memory/core/decay_worker/mod.rs`

```rust
//! Background worker for proactive temporal decay

mod config;
mod worker_impl;

pub use config::DecayWorkerConfig;
pub use worker_impl::DecayWorker;
```

**File**: `packages/candle/src/memory/core/decay_worker/config.rs`

```rust
//! Decay worker configuration

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayWorkerConfig {
    /// Sleep interval between decay cycles (seconds)
    pub cycle_interval_secs: u64,
    /// Batch size for processing
    pub batch_size: usize,
    /// Minimum memory age before decay (hours)
    pub min_age_hours: u64,
    /// Maximum memory age to process (days)
    pub max_age_days: u64,
}

impl Default for DecayWorkerConfig {
    fn default() -> Self {
        Self {
            cycle_interval_secs: 3600,  // 1 hour
            batch_size: 500,
            min_age_hours: 24,           // 1 day
            max_age_days: 365,           // 1 year
        }
    }
}
```

**File**: `packages/candle/src/memory/core/decay_worker/worker_impl.rs`

```rust
//! Decay worker implementation

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use chrono::Utc;
use futures_util::StreamExt;

use crate::memory::core::manager::coordinator::MemoryCoordinator;
use crate::memory::core::manager::surreal::{SurrealDBMemoryManager, MemoryManager};
use super::config::DecayWorkerConfig;

pub struct DecayWorker {
    memory_manager: Arc<SurrealDBMemoryManager>,
    coordinator: Arc<MemoryCoordinator>,
    config: DecayWorkerConfig,
    cursor: Arc<AtomicUsize>,
}

impl DecayWorker {
    pub fn new(
        memory_manager: Arc<SurrealDBMemoryManager>,
        coordinator: Arc<MemoryCoordinator>,
        config: DecayWorkerConfig,
    ) -> Self {
        Self {
            memory_manager,
            coordinator,
            config,
            cursor: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Main worker loop
    pub async fn run(self) {
        log::info!(
            "Decay worker started: cycle_interval={}s, batch_size={}",
            self.config.cycle_interval_secs,
            self.config.batch_size
        );

        loop {
            // Process one batch
            if let Err(e) = self.process_batch().await {
                log::error!("Decay worker batch processing failed: {}", e);
            }

            // Sleep until next cycle
            tokio::time::sleep(Duration::from_secs(self.config.cycle_interval_secs)).await;
        }
    }

    /// Process one batch of memories
    async fn process_batch(&self) -> Result<(), String> {
        let offset = self.cursor.load(Ordering::Relaxed);

        log::debug!(
            "Decay worker: processing batch at offset={}, batch_size={}",
            offset,
            self.config.batch_size
        );

        // Fetch batch using pagination
        let memory_stream = self.memory_manager.list_all_memories(
            self.config.batch_size,
            offset,
        );

        let memories: Vec<_> = memory_stream.collect().await;

        if memories.is_empty() {
            // Reached end of memories, wrap cursor to beginning
            log::debug!("Decay worker: reached end, resetting cursor to 0");
            self.cursor.store(0, Ordering::Relaxed);
            return Ok(());
        }

        let now = Utc::now();
        let min_age = chrono::Duration::hours(self.config.min_age_hours as i64);
        let max_age = chrono::Duration::days(self.config.max_age_days as i64);

        let mut processed_count = 0;
        let mut updated_count = 0;

        for memory_result in memories {
            match memory_result {
                Ok(core_memory) => {
                    // Convert to domain MemoryNode
                    let mut domain_memory = match self.coordinator
                        .convert_memory_to_domain_node(&core_memory)
                    {
                        Ok(m) => m,
                        Err(e) => {
                            log::warn!("Failed to convert memory {}: {}", core_memory.id, e);
                            continue;
                        }
                    };

                    // Calculate age
                    let age = now.signed_duration_since(
                        chrono::DateTime::<chrono::Utc>::from(core_memory.created_at)
                    );

                    // Skip if outside age window
                    if age < min_age || age > max_age {
                        continue;
                    }

                    processed_count += 1;

                    // Store original importance for comparison
                    let original_importance = domain_memory.importance();

                    // Apply temporal decay (from existing logic)
                    if let Err(e) = self.coordinator.apply_temporal_decay(&mut domain_memory).await {
                        log::warn!("Failed to apply decay to {}: {}", domain_memory.id(), e);
                        continue;
                    }

                    // Only persist if importance changed significantly
                    let new_importance = domain_memory.importance();
                    let importance_delta = (original_importance - new_importance).abs();

                    if importance_delta > 0.001 {
                        // Convert back to core MemoryNode for persistence
                        let updated_core = match self.coordinator
                            .convert_domain_to_core_node(&domain_memory)
                        {
                            Ok(m) => m,
                            Err(e) => {
                                log::warn!("Failed to convert domain memory {}: {}", domain_memory.id(), e);
                                continue;
                            }
                        };

                        // Persist to database
                        match self.memory_manager.update_memory(updated_core).await {
                            Ok(_) => {
                                updated_count += 1;
                                log::trace!(
                                    "Decayed memory {}: {:.4} -> {:.4}",
                                    domain_memory.id(),
                                    original_importance,
                                    new_importance
                                );
                            }
                            Err(e) => {
                                log::warn!("Failed to persist decay for {}: {}", domain_memory.id(), e);
                            }
                        }
                    }
                }
                Err(e) => {
                    log::warn!("Failed to retrieve memory from batch: {}", e);
                }
            }
        }

        // Advance cursor for next batch
        self.cursor.fetch_add(self.config.batch_size, Ordering::Relaxed);

        log::info!(
            "Decay worker: processed={}, updated={} memories at offset={}",
            processed_count,
            updated_count,
            offset
        );

        Ok(())
    }
}
```

### Phase 2: Add Conversion Methods to MemoryCoordinator

**File**: `packages/candle/src/memory/core/manager/coordinator/conversions.rs` (NEW)

```rust
//! Memory node conversion utilities

use crate::domain::memory::primitives::node::MemoryNode as DomainMemoryNode;
use crate::memory::core::primitives::node::MemoryNode as CoreMemoryNode;
use crate::memory::utils::Result;

use super::lifecycle::MemoryCoordinator;

impl MemoryCoordinator {
    /// Convert core MemoryNode to domain MemoryNode
    pub(super) fn convert_memory_to_domain_node(
        &self,
        core_node: &CoreMemoryNode,
    ) -> Result<DomainMemoryNode> {
        // Implementation already exists - extract from search.rs:54
        // This is just making it reusable
        todo!("Extract existing conversion logic")
    }

    /// Convert domain MemoryNode back to core MemoryNode
    pub(super) fn convert_domain_to_core_node(
        &self,
        domain_node: &DomainMemoryNode,
    ) -> Result<CoreMemoryNode> {
        // Reverse conversion for persistence
        todo!("Implement reverse conversion")
    }
}
```

### Phase 3: Spawn DecayWorker in MemoryCoordinator

**File**: `packages/candle/src/memory/core/manager/coordinator/lifecycle.rs`

**Add field to MemoryCoordinator struct** (after line 41):
```rust
pub(super) decay_worker_config: DecayWorkerConfig,
```

**Add to constructor** (after line 99):
```rust
decay_worker_config: DecayWorkerConfig::default(),
```

**Add method to spawn decay worker** (after shutdown_workers, line 142):
```rust
/// Spawn background decay worker
///
/// Returns a JoinHandle that can be used to gracefully shutdown the worker
pub fn spawn_decay_worker(&self) -> tokio::task::JoinHandle<()> {
    use crate::memory::core::decay_worker::DecayWorker;

    let worker = DecayWorker::new(
        self.surreal_manager.clone(),
        Arc::new(self.clone()), // MemoryCoordinator needs to be Arc-wrapped
        self.decay_worker_config.clone(),
    );

    tokio::spawn(async move {
        log::info!("Decay worker spawned");
        worker.run().await;
        log::info!("Decay worker stopped");
    })
}

/// Configure decay worker settings
pub fn set_decay_worker_config(&mut self, config: DecayWorkerConfig) {
    self.decay_worker_config = config;
    log::info!(
        "Decay worker config updated: cycle={}s, batch={}",
        config.cycle_interval_secs,
        config.batch_size
    );
}
```

### Phase 4: Update Module Declarations

**File**: `packages/candle/src/memory/core/mod.rs`

Add module declaration:
```rust
pub mod decay_worker;
```

**File**: `packages/candle/src/memory/core/manager/coordinator/mod.rs`

Add module:
```rust
mod conversions;
```

### Phase 5: Remove Lazy Decay from Read Path (OPTIONAL - Breaking Change)

**DECISION POINT**: Should we remove decay from `search_memories()` and `get_memory()`?

**Option A - Keep Both**:
- Pros: Graceful migration, more accurate decay
- Cons: Still has read-path overhead

**Option B - Remove from Read Path**:
- Pros: Zero read-path overhead
- Cons: Importance values slightly stale between worker cycles

**Recommendation**: Start with Option A, migrate to Option B after worker is proven stable.

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_decay_worker_processes_batch() {
        // Setup mock memory manager with test data
        // Verify batch processing
        // Check importance updates
    }

    #[tokio::test]
    async fn test_decay_worker_wraps_cursor() {
        // Verify cursor resets at end
    }

    #[test]
    fn test_age_filtering() {
        // Verify min_age and max_age thresholds
    }
}
```

### Integration Tests

1. **Spawn worker with test config** (short cycle interval)
2. **Insert memories with different created_at timestamps**
3. **Verify importance decreases over multiple cycles**
4. **Check database persistence**
5. **Measure performance impact**

## Performance Considerations

### Batch Size Tuning

- **Small batches (100-200)**: Lower memory usage, more DB round-trips
- **Large batches (500-1000)**: Better throughput, higher memory usage
- **Recommendation**: Start with 500, tune based on metrics

### Cycle Interval Tuning

- **Frequent (15-30 min)**: More accurate decay, higher CPU usage
- **Infrequent (2-4 hours)**: Lower overhead, staleness acceptable
- **Recommendation**: Start with 60 minutes

### Database Load

- **Pagination query**: `O(batch_size)` per cycle
- **Update queries**: `O(qualifying_memories)` per cycle
- **Expected**: <100 updates per batch for typical workloads
- **Mitigation**: Use min_age threshold to skip fresh memories

### Memory Usage

- **Per batch**: ~1-5 MB (500 memories × embedding vectors)
- **Worker overhead**: <10 MB (cursor + config)
- **Total impact**: Negligible for production systems

## Monitoring and Observability

### Metrics to Track

```rust
pub struct DecayWorkerMetrics {
    pub cycles_completed: u64,
    pub memories_processed: u64,
    pub memories_updated: u64,
    pub avg_batch_duration_ms: f64,
    pub errors_encountered: u64,
}
```

### Logging Strategy

- **INFO**: Cycle completion with counts
- **DEBUG**: Batch offsets and cursor position
- **TRACE**: Individual memory decay calculations
- **WARN**: Conversion or persistence failures
- **ERROR**: Batch processing failures

## Migration Path

### Phase 1: Deploy with Dual System
1. Deploy decay worker
2. Keep lazy decay in read path
3. Monitor worker performance
4. Verify importance convergence

### Phase 2: Gradual Cutover
1. Add feature flag for lazy decay
2. Disable lazy decay for low-priority reads
3. Monitor for correctness issues
4. Expand to all reads

### Phase 3: Full Migration
1. Remove lazy decay code from read path
2. Update temporal.rs documentation
3. Deprecate on-demand decay methods
4. Celebrate zero-cost reads! 🎉

## Configuration Example

```rust
// In application initialization
let decay_config = DecayWorkerConfig {
    cycle_interval_secs: 3600,   // 1 hour
    batch_size: 500,
    min_age_hours: 24,           // Only process memories older than 1 day
    max_age_days: 365,           // Skip memories older than 1 year
};

coordinator.set_decay_worker_config(decay_config);
let decay_handle = coordinator.spawn_decay_worker();

// Graceful shutdown
decay_handle.abort();
```

## Success Criteria

1. ✅ Decay worker runs continuously without crashes
2. ✅ Memory importance values decrease over time according to decay_rate
3. ✅ Database updates persist correctly
4. ✅ No significant performance regression on read operations
5. ✅ Worker can be configured and restarted without data loss
6. ✅ Cursor-based pagination prevents duplicate processing
7. ✅ Age thresholds effectively filter unnecessary work

## Future Enhancements

### Adaptive Scheduling
- Adjust cycle_interval based on memory creation rate
- Process active memories more frequently

### Priority-Based Decay
- Apply decay more aggressively to low-importance memories
- Preserve high-importance memories longer

### Quantum Coherence Integration
- Update QuantumSignature entropy in worker
- Call apply_decoherence() for quantum memories

### Distributed Workers
- Shard memory space across multiple workers
- Use consistent hashing for memory assignment

## References

**Code Files Read**:
- `packages/candle/src/memory/core/manager/coordinator/lifecycle.rs` (Lines 1-144)
- `packages/candle/src/memory/core/manager/coordinator/temporal.rs` (Lines 1-89)
- `packages/candle/src/memory/core/manager/coordinator/search.rs` (Lines 1-100)
- `packages/candle/src/memory/core/manager/coordinator/operations.rs` (Lines 140-210)
- `packages/candle/src/memory/core/cognitive_worker/worker_core.rs` (Lines 1-119)
- `packages/candle/src/memory/core/cognitive_queue.rs` (Lines 1-233)
- `packages/candle/src/memory/core/primitives/node.rs` (Lines 1-100)
- `packages/candle/src/memory/core/primitives/metadata.rs` (Lines 1-251)
- `packages/candle/src/memory/core/manager/surreal/trait_def.rs` (Lines 1-269)

All architectural decisions are based on CERTAINTY from reading actual production code, not conjecture.
