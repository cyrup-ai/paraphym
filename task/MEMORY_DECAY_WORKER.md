# MEMORY_DECAY_WORKER: Background Temporal Decay Worker

## OBJECTIVE

Move temporal decay from **lazy evaluation** (calculated on every memory read) to **proactive background processing** via a dedicated worker, eliminating expensive read-path overhead.

**Current Problem:** Decay is applied in [`search.rs:63-68`](../packages/candle/src/memory/core/manager/coordinator/search.rs) and [`operations.rs:183-184`](../packages/candle/src/memory/core/manager/coordinator/operations.rs) making EVERY memory retrieval slower.

**Solution:** Background worker that continuously processes memories in batches, applying decay and persisting changes to the database.

---

## EXISTING CODE TO REUSE

### 1. Conversion Methods (COMPLETE - DO NOT RECREATE)

**File:** [`packages/candle/src/memory/core/manager/coordinator/conversions.rs`](../packages/candle/src/memory/core/manager/coordinator/conversions.rs)

Already implements **BOTH** conversion methods needed:

```rust
// Core → Domain (for processing)
pub(super) fn convert_memory_to_domain_node(
    &self,
    memory_node: &crate::memory::core::primitives::node::MemoryNode,
) -> Result<crate::domain::memory::primitives::node::MemoryNode>

// Domain → Core (for persistence)
pub(super) fn convert_domain_to_memory_node(
    &self,
    domain_node: &crate::domain::memory::primitives::node::MemoryNode,
) -> crate::memory::core::primitives::node::MemoryNode
```

**Status:** ✅ Lines 1-257 - Ready to use

### 2. Temporal Decay Logic (COMPLETE - DO NOT RECREATE)

**File:** [`packages/candle/src/memory/core/manager/coordinator/temporal.rs`](../packages/candle/src/memory/core/manager/coordinator/temporal.rs)

Already implements decay calculation and application:

```rust
// Lines 13-59
pub(super) async fn apply_temporal_decay(
    &self,
    memory: &mut crate::domain::memory::primitives::node::MemoryNode,
) -> Result<()> {
    // Calculates age from created_at
    // Applies exponential decay: e^(-decay_rate * days)
    // Updates memory importance via set_importance()
    // Updates quantum coherence in quantum_state
}
```

**Status:** ✅ Ready to use

### 3. Worker Spawning Pattern (REFERENCE FOR NEW CODE)

**File:** [`packages/candle/src/memory/core/manager/coordinator/lifecycle.rs`](../packages/candle/src/memory/core/manager/coordinator/lifecycle.rs)

Existing pattern for spawning cognitive workers (lines 62-79):

```rust
for worker_id in 0..num_workers {
    let queue = cognitive_queue.clone();
    let manager = surreal_manager.clone();
    let evaluator = committee_evaluator.clone();

    let worker = crate::memory::core::cognitive_worker::CognitiveWorker::new(
        queue, manager, evaluator,
    );

    tokio::spawn(async move {
        log::info!("Cognitive worker {} started", worker_id);
        worker.run().await;
        log::info!("Cognitive worker {} stopped", worker_id);
    });
}
```

**Pattern to follow:** Spawn inline in constructor, Arc clone dependencies, fire-and-forget (no JoinHandle storage).

### 4. MemoryCoordinator Structure

**File:** [`packages/candle/src/memory/core/manager/coordinator/lifecycle.rs`](../packages/candle/src/memory/core/manager/coordinator/lifecycle.rs)

Current structure (lines 26-42):

```rust
pub struct MemoryCoordinator {
    pub(super) surreal_manager: Arc<SurrealDBMemoryManager>,
    pub(super) repository: Arc<RwLock<MemoryRepository>>,
    pub(super) embedding_model: TextEmbeddingModel,
    pub(super) cognitive_queue: Arc<CognitiveProcessingQueue>,
    pub(super) committee_evaluator: Arc<ModelCommitteeEvaluator>,
    pub(super) quantum_router: Arc<QuantumRouter>,
    pub(super) quantum_state: Arc<RwLock<QuantumState>>,
    pub(super) cognitive_state: Arc<RwLock<CognitiveState>>,
    pub(super) cognitive_workers: Arc<tokio::sync::RwLock<Vec<tokio::task::JoinHandle<()>>>>,
    pub(super) lazy_eval_strategy: LazyEvalStrategy,
    pub(super) evaluation_cache: Cache<String, f64>,
    pub(super) decay_rate: f64,  // ← ALREADY EXISTS
}
```

**Key finding:** `decay_rate` field ALREADY EXISTS (line 42) - no need to add it!

### 5. Database Operations

**File:** [`packages/candle/src/memory/core/manager/surreal/trait_def.rs`](../packages/candle/src/memory/core/manager/surreal/trait_def.rs)

Available methods (lines 23-43):

```rust
fn list_all_memories(&self, limit: usize, offset: usize) -> MemoryStream;
fn update_memory(&self, memory: MemoryNode) -> PendingMemory;
```

**Status:** ✅ Ready to use for pagination and persistence

---

## NEW CODE TO CREATE

### Directory Structure

```
packages/candle/src/memory/core/
└── decay_worker/
    ├── mod.rs        ← Module exports
    ├── config.rs     ← DecayWorkerConfig struct
    └── worker.rs     ← DecayWorker implementation
```

### File 1: `decay_worker/mod.rs`

**Path:** `packages/candle/src/memory/core/decay_worker/mod.rs`

```rust
//! Background worker for proactive temporal decay
//!
//! Processes memories in batches, applying exponential decay to importance scores
//! and persisting changes to SurrealDB. Eliminates expensive on-read decay calculations.

mod config;
mod worker;

pub use config::DecayWorkerConfig;
pub use worker::DecayWorker;
```

### File 2: `decay_worker/config.rs`

**Path:** `packages/candle/src/memory/core/decay_worker/config.rs`

```rust
//! Decay worker configuration

use serde::{Deserialize, Serialize};

/// Configuration for background decay worker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayWorkerConfig {
    /// Sleep interval between decay cycles (seconds)
    pub cycle_interval_secs: u64,
    
    /// Number of memories to process per batch
    pub batch_size: usize,
    
    /// Minimum memory age before applying decay (hours)
    /// Prevents thrashing on fresh memories
    pub min_age_hours: u64,
    
    /// Maximum memory age to process (days)
    /// Memories older than this already have minimal importance
    pub max_age_days: u64,
}

impl Default for DecayWorkerConfig {
    fn default() -> Self {
        Self {
            cycle_interval_secs: 3600,  // 1 hour between cycles
            batch_size: 500,             // Process 500 memories per wake
            min_age_hours: 24,           // Skip memories < 1 day old
            max_age_days: 365,           // Skip memories > 1 year old
        }
    }
}
```

### File 3: `decay_worker/worker.rs`

**Path:** `packages/candle/src/memory/core/decay_worker/worker.rs`

```rust
//! Decay worker implementation
//!
//! Implements continuous rotation pattern:
//! 1. Wake every N minutes
//! 2. Fetch batch of memories via pagination
//! 3. Filter by age criteria
//! 4. Apply decay using existing temporal.rs logic
//! 5. Persist importance changes to SurrealDB
//! 6. Advance cursor, wrap at end
//! 7. Sleep and repeat

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use chrono::Utc;
use futures_util::StreamExt;

use crate::memory::core::manager::coordinator::MemoryCoordinator;
use crate::memory::core::manager::surreal::{SurrealDBMemoryManager, MemoryManager};
use super::config::DecayWorkerConfig;

/// Background worker for proactive temporal decay
pub struct DecayWorker {
    /// SurrealDB manager for database access
    memory_manager: Arc<SurrealDBMemoryManager>,
    
    /// Memory coordinator for decay logic and conversions
    coordinator: Arc<MemoryCoordinator>,
    
    /// Worker configuration
    config: DecayWorkerConfig,
    
    /// Pagination cursor (tracks current offset in memory list)
    cursor: Arc<AtomicUsize>,
}

impl DecayWorker {
    /// Create new decay worker
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

    /// Main worker loop - runs indefinitely
    pub async fn run(self) {
        log::info!(
            "Decay worker started: cycle={}s, batch={}, min_age={}h, max_age={}d",
            self.config.cycle_interval_secs,
            self.config.batch_size,
            self.config.min_age_hours,
            self.config.max_age_days
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
            "Decay batch starting: offset={}, batch_size={}",
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
            // Reached end, wrap cursor to beginning
            log::debug!("Decay worker: end of memory list, resetting cursor to 0");
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
                    // Convert core → domain for decay processing
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

                    // Store original importance
                    let original_importance = domain_memory.importance();

                    // Apply temporal decay (reuses existing logic from temporal.rs)
                    if let Err(e) = self.coordinator.apply_temporal_decay(&mut domain_memory).await {
                        log::warn!("Failed to apply decay to {}: {}", domain_memory.id(), e);
                        continue;
                    }

                    // Check if importance changed significantly
                    let new_importance = domain_memory.importance();
                    let importance_delta = (original_importance - new_importance).abs();

                    if importance_delta > 0.001 {
                        // Convert domain → core for persistence
                        let updated_core = self.coordinator
                            .convert_domain_to_memory_node(&domain_memory);

                        // Persist to SurrealDB
                        match self.memory_manager.update_memory(updated_core).await {
                            Ok(_) => {
                                updated_count += 1;
                                log::trace!(
                                    "Decayed memory {}: {:.4} → {:.4}",
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
            "Decay batch complete: processed={}, updated={}, offset={}",
            processed_count,
            updated_count,
            offset
        );

        Ok(())
    }
}
```

---

## MODIFICATIONS TO EXISTING FILES

### Modification 1: Add decay_worker Module Declaration

**File:** `packages/candle/src/memory/core/mod.rs`

**Location:** After line 7 (`pub mod cognitive_worker;`)

**Add:**
```rust
pub mod decay_worker;
```

**Result:** Line 8 should be `pub mod decay_worker;`

### Modification 2: Add DecayWorkerConfig Field

**File:** `packages/candle/src/memory/core/manager/coordinator/lifecycle.rs`

**Location:** After line 42 (`pub(super) decay_rate: f64,`)

**Add:**
```rust
    // DECAY WORKER CONFIG:
    pub(super) decay_worker_config: crate::memory::core::decay_worker::DecayWorkerConfig,
```

**Result:** New field at line 43-44

### Modification 3: Initialize DecayWorkerConfig in Constructor

**File:** `packages/candle/src/memory/core/manager/coordinator/lifecycle.rs`

**Location:** In `MemoryCoordinator::new()`, after line 99 (`decay_rate: 0.1,`)

**Add:**
```rust
            decay_worker_config: crate::memory::core::decay_worker::DecayWorkerConfig::default(),
```

**Result:** New field initialization at line 100

### Modification 4: Spawn Decay Worker in Constructor

**File:** `packages/candle/src/memory/core/manager/coordinator/lifecycle.rs`

**Location:** After line 80 (`log::info!("Started {} cognitive worker tasks", num_workers);`)

**Add:**
```rust

        // Spawn decay worker
        let decay_config = crate::memory::core::decay_worker::DecayWorkerConfig::default();
        let decay_worker = crate::memory::core::decay_worker::DecayWorker::new(
            surreal_manager.clone(),
            // Note: We'll create a self-referential Arc after construction
            // For now, we'll spawn this AFTER MemoryCoordinator is created
        );
        
        log::info!("Decay worker will be spawned after coordinator initialization");
```

**WAIT - ARCHITECTURE ISSUE:** We need `Arc<MemoryCoordinator>` to pass to DecayWorker, but we're still IN the constructor. 

**SOLUTION:** Add a separate `spawn_decay_worker()` method that's called AFTER construction:

**Instead, add this method AFTER the `shutdown_workers()` method (after line 142):**

```rust

    /// Spawn background decay worker
    ///
    /// Must be called after coordinator is wrapped in Arc.
    /// Returns immediately - worker runs in background indefinitely.
    pub fn spawn_decay_worker(self: &Arc<Self>) {
        let config = self.decay_worker_config.clone();
        let worker = crate::memory::core::decay_worker::DecayWorker::new(
            self.surreal_manager.clone(),
            self.clone(), // Arc<MemoryCoordinator>
            config,
        );

        tokio::spawn(async move {
            log::info!("Decay worker spawned");
            worker.run().await;
        });
    }

    /// Configure decay worker settings
    ///
    /// Must be called before spawn_decay_worker().
    pub fn set_decay_worker_config(&mut self, config: crate::memory::core::decay_worker::DecayWorkerConfig) {
        self.decay_worker_config = config;
        log::info!(
            "Decay worker config updated: cycle={}s, batch={}",
            config.cycle_interval_secs,
            config.batch_size
        );
    }
```

---

## IMPLEMENTATION SEQUENCE

### Step 1: Create decay_worker Module

1. Create directory: `packages/candle/src/memory/core/decay_worker/`
2. Create `mod.rs` with module exports
3. Create `config.rs` with DecayWorkerConfig struct
4. Create `worker.rs` with DecayWorker implementation

### Step 2: Update Module Declarations

1. Edit `packages/candle/src/memory/core/mod.rs`
2. Add `pub mod decay_worker;` after line 7

### Step 3: Modify MemoryCoordinator

1. Edit `packages/candle/src/memory/core/manager/coordinator/lifecycle.rs`
2. Add `decay_worker_config` field to struct (after line 42)
3. Initialize field in constructor (after line 99)
4. Add `spawn_decay_worker()` method (after line 142)
5. Add `set_decay_worker_config()` method (after spawn method)

### Step 4: Usage Example

```rust
// In application initialization
let coordinator = Arc::new(MemoryCoordinator::new(surreal_manager, embedding_model).await?);

// Optional: Configure before spawning
// coordinator.set_decay_worker_config(DecayWorkerConfig { ... });

// Spawn decay worker
coordinator.spawn_decay_worker();

// Worker now runs in background indefinitely
```

---

## KEY ARCHITECTURE PATTERNS

### Pattern 1: Conversion Pipeline

```rust
// Core → Domain (for processing)
let mut domain_mem = coordinator.convert_memory_to_domain_node(&core_mem)?;

// Apply decay (modifies in-place)
coordinator.apply_temporal_decay(&mut domain_mem).await?;

// Domain → Core (for persistence)
let updated_core = coordinator.convert_domain_to_memory_node(&domain_mem);

// Persist
manager.update_memory(updated_core).await?;
```

### Pattern 2: Pagination with Cursor

```rust
let cursor = Arc::new(AtomicUsize::new(0));

loop {
    let offset = cursor.load(Ordering::Relaxed);
    let batch = manager.list_all_memories(batch_size, offset);
    
    if batch.is_empty() {
        cursor.store(0, Ordering::Relaxed); // Wrap to start
    } else {
        cursor.fetch_add(batch_size, Ordering::Relaxed); // Advance
    }
}
```

### Pattern 3: Age Filtering

```rust
let now = Utc::now();
let min_age = chrono::Duration::hours(24);
let max_age = chrono::Duration::days(365);

let age = now.signed_duration_since(memory.created_at);

if age < min_age || age > max_age {
    continue; // Skip this memory
}
```

---

## DEFINITION OF DONE

1. ✅ `decay_worker` module exists with 3 files (mod.rs, config.rs, worker.rs)
2. ✅ Module declared in `core/mod.rs`
3. ✅ `decay_worker_config` field added to MemoryCoordinator
4. ✅ Field initialized in constructor with Default
5. ✅ `spawn_decay_worker()` method added to MemoryCoordinator
6. ✅ `set_decay_worker_config()` method added to MemoryCoordinator
7. ✅ Worker can be spawned: `coordinator.spawn_decay_worker()`
8. ✅ Worker logs "Decay worker spawned" on start
9. ✅ Worker processes batches every N seconds (configurable)
10. ✅ Worker applies decay using existing `apply_temporal_decay()`
11. ✅ Worker persists changes via `update_memory()`
12. ✅ Worker wraps cursor at end of memory list
13. ✅ Code compiles without errors

---

## REFERENCES

### Existing Code Files (Verified by Reading)

- [conversions.rs](../packages/candle/src/memory/core/manager/coordinator/conversions.rs) - Lines 1-257
- [lifecycle.rs](../packages/candle/src/memory/core/manager/coordinator/lifecycle.rs) - Lines 1-144  
- [temporal.rs](../packages/candle/src/memory/core/manager/coordinator/temporal.rs) - Lines 1-89
- [search.rs](../packages/candle/src/memory/core/manager/coordinator/search.rs) - Lines 1-100
- [operations.rs](../packages/candle/src/memory/core/manager/coordinator/operations.rs) - Lines 140-210
- [trait_def.rs](../packages/candle/src/memory/core/manager/surreal/trait_def.rs) - Lines 1-269
- [worker_core.rs](../packages/candle/src/memory/core/cognitive_worker/worker_core.rs) - Lines 1-119

### Module Structure (Verified by lsd --tree)

```
packages/candle/src/memory/core/
├── cognitive_queue.rs
├── cognitive_worker/
│   ├── committee_evaluation.rs
│   ├── entanglement_discovery.rs
│   ├── mod.rs
│   ├── temporal_maintenance.rs
│   └── worker_core.rs
├── manager/
│   ├── coordinator/
│   │   ├── conversions.rs      ← REUSE THIS
│   │   ├── lifecycle.rs         ← MODIFY THIS
│   │   ├── mod.rs
│   │   ├── operations.rs
│   │   ├── relationships.rs
│   │   ├── search.rs
│   │   ├── temporal.rs          ← REUSE THIS
│   │   ├── trait_impl.rs
│   │   ├── types.rs
│   │   └── workers.rs
│   ├── mod.rs
│   └── surreal/
│       ├── futures.rs
│       ├── manager.rs
│       ├── mod.rs
│       ├── operations.rs
│       ├── queries.rs
│       ├── trait_def.rs         ← REFERENCE THIS
│       └── types.rs
├── mod.rs                       ← MODIFY THIS
└── primitives/
    ├── metadata.rs
    ├── mod.rs
    ├── node.rs
    ├── relationship.rs
    └── types.rs
```

All architectural decisions based on reading actual source code, not speculation.
