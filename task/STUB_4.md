# STUB_4: Restore Causal Link Storage and Reasoning

## RESEARCH VERIFICATION ✅

**Codebase Analysis Completed**: All claims in this document have been verified against the actual source code:
- ✅ CausalLink type exists and is complete (temporal.rs:83-110)
- ✅ CausalLink creation happens in entanglement_discovery.rs:277-301
- ✅ CausalLinks are logged but NEVER stored (confirmed via code search)
- ✅ NO `create_causal_edge()` method exists anywhere in codebase
- ✅ NO `caused` relation table exists in database schema
- ✅ Entanglement edge pattern verified in operations.rs:533-566
- ✅ MemoryManager trait structure verified in trait_def.rs

**Code References**: Only 2 files reference CausalLink:
1. `domain/memory/cognitive/types/temporal.rs` - Type definition + in-memory helpers
2. `memory/core/cognitive_worker/entanglement_discovery.rs` - Creation point (no storage)

---

## EXECUTIVE SUMMARY

**Current State**: The causal reasoning system is **95% complete but non-functional**. CausalLink structures are created during entanglement discovery with full causal metadata (strength, temporal distance, direction), but are immediately discarded after logging. The complete pipeline exists except for database storage and query methods.

**Core Issue**: Line 277-301 of `entanglement_discovery.rs` creates CausalLinks that are logged but never persisted or used for reasoning.

**Solution**: Add database storage (mirroring the existing `entangled` RELATE pattern) and query methods to make the causal graph queryable.

---

## OBJECTIVE

Complete the broken causal reasoning pipeline by adding storage and query capabilities for CausalLinks that are already being discovered and created.

---

## BACKGROUND: WHAT ALREADY EXISTS

### 1. CausalLink Type Definition (COMPLETE)
**Location**: [`packages/candle/src/domain/memory/cognitive/types/temporal.rs:83-110`](../packages/candle/src/domain/memory/cognitive/types/temporal.rs)

```rust
/// Causal link between memories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalLink {
    /// Source event ID (the cause)
    pub source_id: Uuid,
    /// Target event ID (the effect)
    pub target_id: Uuid,
    /// Causal strength (0.0 to 1.0) - how confident we are in causality
    pub strength: f32,
    /// Temporal distance in milliseconds (signed: negative = target is older)
    pub temporal_distance: i64,
}

impl CausalLink {
    #[allow(dead_code)] // ← REMOVE THIS
    pub fn new(source_id: Uuid, target_id: Uuid, strength: f32, temporal_distance: i64) -> Self {
        Self {
            source_id,
            target_id,
            strength: strength.clamp(0.0, 1.0),
            temporal_distance,
        }
    }
}
```

**Key Details**:
- Fields are `pub` (direct construction possible)
- `temporal_distance` is signed i64 milliseconds:
  - Positive: target is newer (forward causality)
  - Negative: target is older (backward inference)
  - Zero: simultaneous events
- `strength` is clamped to [0.0, 1.0] - represents causal confidence
- Constructor validates strength but allows any temporal_distance

### 2. CausalLink Creation (FUNCTIONAL BUT DISCONNECTED)
**Location**: [`packages/candle/src/memory/core/cognitive_worker/entanglement_discovery.rs:277-301`](../packages/candle/src/memory/core/cognitive_worker/entanglement_discovery.rs)

```rust
// Line 277-301 (verified)
if let Some(temporal_dist) = temporal_distance_ms
    && let Ok(source_uuid) = uuid::Uuid::parse_str(&memory.id) {
    let causal_link = CausalLink::new(
        source_uuid,
        related_id_uuid,
        entanglement_strength,
        temporal_dist,
    );

    log::info!(
        "Causal link: {} -> {} (strength: {:.3}, temporal: {}ms, direction: {})",
        memory.id,
        related_memory.id,
        causal_link.strength,
        causal_link.temporal_distance,
        if temporal_dist > 0 {
            "forward"
        } else if temporal_dist < 0 {
            "backward"
        } else {
            "simultaneous"
        }
    );
    
    // ❌ CRITICAL GAP: CausalLink is dropped here - never stored!
}
```

**What Happens**:
1. ✅ Temporal distance calculated from memory timestamps
2. ✅ CausalLink created with strength and distance
3. ✅ Detailed logging of causal relationship
4. ❌ **Link is dropped - never stored or used**

### 3. Entanglement Edge Storage (WORKING PATTERN TO FOLLOW)
**Location**: [`packages/candle/src/memory/core/manager/surreal/operations.rs:533-566`](../packages/candle/src/memory/core/manager/surreal/operations.rs)

```rust
fn create_entanglement_edge(
    &self,
    source_id: &str,
    target_id: &str,
    entanglement_type: EntanglementType,
    strength: f32,
) -> PendingEntanglementEdge {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let db = self.db.clone();
    let source_id = source_id.to_string();
    let target_id = target_id.to_string();

    tokio::spawn(async move {
        let result = async {
            let now = crate::memory::utils::current_timestamp_ms();
            let entanglement_type_str = format!("{:?}", entanglement_type);

            let query = format!(
                "RELATE {}->entangled->{} SET entanglement_type = $entanglement_type, strength = $strength, created_at = $created_at",
                source_id, target_id
            );

            db.query(&query)
                .bind(("entanglement_type", entanglement_type_str))
                .bind(("strength", strength))
                .bind(("created_at", now))
                .await
                .map_err(|e| Error::Database(format!("{:?}", e)))?;

            Ok(())
        }
        .await;

        let _ = tx.send(result);
    });

    PendingEntanglementEdge::new(rx)
}
```

**Key Pattern**:
- Uses SurrealDB RELATE statement for graph edges
- Stores in `entangled` relation table
- Async with oneshot channel result
- Returns `PendingEntanglementEdge` future

**Database Schema** (from [`manager.rs:140-153`](../packages/candle/src/memory/core/manager/surreal/manager.rs)):
```rust
// Existing entanglement edges schema
self.db.query("
    DEFINE TABLE IF NOT EXISTS entangled SCHEMAFULL TYPE RELATION FROM memory TO memory;
    DEFINE FIELD IF NOT EXISTS entanglement_type ON entangled TYPE string;
    DEFINE FIELD IF NOT EXISTS strength ON entangled TYPE float;
    DEFINE FIELD IF NOT EXISTS created_at ON entangled TYPE int;
").await?;
```

### 4. TemporalContext Query Methods (COMPLETE BUT UNUSED)
**Location**: [`packages/candle/src/domain/memory/cognitive/types/temporal.rs:56-73`](../packages/candle/src/domain/memory/cognitive/types/temporal.rs)

```rust
impl TemporalContext {
    /// Get causal predecessors of a memory (what caused this?)
    pub fn get_causal_predecessors(&self, memory_id: Uuid) -> Vec<Uuid> {
        self.causal_links
            .iter()
            .filter(|link| link.target_id == memory_id)
            .map(|link| link.source_id)
            .collect()
    }

    /// Get causal successors of a memory (what did this cause?)
    pub fn get_causal_successors(&self, memory_id: Uuid) -> Vec<Uuid> {
        self.causal_links
            .iter()
            .filter(|link| link.source_id == memory_id)
            .map(|link| link.target_id)
            .collect()
    }
}
```

**Status**: These methods work correctly but only on the in-memory `causal_links` Vec, which is never populated from the database.

---

## THE GAP: WHAT'S MISSING

### Missing Component 1: Database Storage for CausalLinks
- ❌ No `caused` RELATE table defined in schema
- ❌ No `create_causal_edge()` method in MemoryManager trait
- ❌ No database persistence when CausalLink is created
- ❌ CausalLinks exist only in logs, never queryable

### Missing Component 2: Query Methods
- ❌ No `get_causal_predecessors()` database query (only in-memory)
- ❌ No `get_causal_successors()` database query
- ❌ No `trace_causal_chain()` graph traversal
- ❌ No way to answer "what caused X?" or "what did X cause?"

### Missing Component 3: Integration Points
- ❌ CausalLinks not stored when created in entanglement discovery
- ❌ No retrieval during memory queries or chat context building

---

## IMPLEMENTATION PLAN

### STEP 1: Add Database Schema for Causal Relations

**File**: `packages/candle/src/memory/core/manager/surreal/manager.rs`  
**Location**: In `initialize()` method, after line 153 (after the `entangled` table definition)

**Add this schema definition**:
```rust
// Define causal relation edges (separate from general entanglement)
self.db
    .query(
        "
        DEFINE TABLE IF NOT EXISTS caused SCHEMAFULL TYPE RELATION FROM memory TO memory;
        DEFINE FIELD IF NOT EXISTS strength ON caused TYPE float;
        DEFINE FIELD IF NOT EXISTS temporal_distance ON caused TYPE int;
        DEFINE FIELD IF NOT EXISTS created_at ON caused TYPE int;
        ",
    )
    .await
    .map_err(|e| {
        Error::Database(format!("Failed to define causal edges: {:?}", e))
    })?;
```

**Rationale**:
- Separate `caused` table from `entangled` for semantic clarity
- `temporal_distance` is int (i64 milliseconds, signed for direction)
- Follows same RELATION pattern as `entangled`
- Fields match CausalLink structure exactly

---

### STEP 2: Add create_causal_edge() to MemoryManager Trait

**File**: `packages/candle/src/memory/core/manager/surreal/trait_def.rs`  
**Location**: After line 78 (after `create_entanglement_edge()` in trait definition)

**Add trait method**:
```rust
/// Create a causal edge between two memories
fn create_causal_edge(
    &self,
    source_id: &str,
    target_id: &str,
    strength: f32,
    temporal_distance: i64,
) -> PendingEntanglementEdge;  // Reuse PendingEntanglementEdge type
```

**Also add to Arc<T> blanket impl** (after line 165, after the `create_entanglement_edge` Arc impl):
```rust
fn create_causal_edge(
    &self,
    source_id: &str,
    target_id: &str,
    strength: f32,
    temporal_distance: i64,
) -> PendingEntanglementEdge {
    (**self).create_causal_edge(source_id, target_id, strength, temporal_distance)
}
```

---

### STEP 3: Implement create_causal_edge() in SurrealDBMemoryManager

**File**: `packages/candle/src/memory/core/manager/surreal/operations.rs`  
**Location**: After line 566 (after `create_entanglement_edge()` implementation)

**Add implementation** (mirror the entanglement pattern):
```rust
fn create_causal_edge(
    &self,
    source_id: &str,
    target_id: &str,
    strength: f32,
    temporal_distance: i64,
) -> PendingEntanglementEdge {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let db = self.db.clone();
    let source_id = source_id.to_string();
    let target_id = target_id.to_string();

    tokio::spawn(async move {
        let result = async {
            let now = crate::memory::utils::current_timestamp_ms();

            let query = format!(
                "RELATE {}->caused->{} SET strength = $strength, temporal_distance = $temporal_distance, created_at = $created_at",
                source_id, target_id
            );

            db.query(&query)
                .bind(("strength", strength))
                .bind(("temporal_distance", temporal_distance))
                .bind(("created_at", now))
                .await
                .map_err(|e| Error::Database(format!("{:?}", e)))?;

            Ok(())
        }
        .await;

        let _ = tx.send(result);
    });

    PendingEntanglementEdge::new(rx)
}
```

**Key differences from create_entanglement_edge**:
- Uses `caused` relation instead of `entangled`
- Stores `temporal_distance` (i64) instead of `entanglement_type`
- Otherwise identical async pattern

---

### STEP 4: Store CausalLinks in Entanglement Discovery

**File**: `packages/candle/src/memory/core/cognitive_worker/entanglement_discovery.rs`  
**Location**: After line 300 (immediately after the log::info! statement that logs the causal link)

**Add storage code**:
```rust
// EXISTING CODE (lines 277-300):
if let Some(temporal_dist) = temporal_distance_ms
    && let Ok(source_uuid) = uuid::Uuid::parse_str(&memory.id) {
    let causal_link = CausalLink::new(
        source_uuid,
        related_id_uuid,
        entanglement_strength,
        temporal_dist,
    );

    log::info!(
        "Causal link: {} -> {} (strength: {:.3}, temporal: {}ms, direction: {})",
        memory.id,
        related_memory.id,
        causal_link.strength,
        causal_link.temporal_distance,
        if temporal_dist > 0 {
            "forward"
        } else if temporal_dist < 0 {
            "backward"
        } else {
            "simultaneous"
        }
    );

    // 🆕 ADD THIS: Store the causal link in database
    if let Err(e) = manager
        .create_causal_edge(
            &memory.id,
            &related_memory.id,
            causal_link.strength,
            causal_link.temporal_distance,
        )
        .await
    {
        log::error!(
            "Failed to create causal edge {} -> {}: {:?}",
            memory.id,
            related_memory.id,
            e
        );
    } else {
        log::debug!(
            "Stored causal edge: {} ->caused-> {} (strength: {:.3}, temporal: {}ms)",
            memory.id,
            related_memory.id,
            causal_link.strength,
            causal_link.temporal_distance
        );
    }
}
```

---

### STEP 5: Add Causal Query Methods to MemoryManager Trait

**File**: `packages/candle/src/memory/core/manager/surreal/trait_def.rs`  
**Location**: After line 108 (after the entanglement methods, before the closing brace of the trait)

**Add trait methods**:
```rust
// === Causal Reasoning Operations ===

/// Get memories that causally preceded this one (what caused this?)
fn get_causal_predecessors(&self, memory_id: &str) -> MemoryStream;

/// Get memories that this causally influenced (what did this cause?)
fn get_causal_successors(&self, memory_id: &str) -> MemoryStream;

/// Traverse causal chain forward from a memory
fn trace_causal_chain_forward(
    &self,
    start_memory_id: &str,
    max_depth: usize,
) -> MemoryStream;

/// Traverse causal chain backward to find root causes
fn trace_causal_chain_backward(
    &self,
    start_memory_id: &str,
    max_depth: usize,
) -> MemoryStream;
```

**Add to Arc<T> blanket impl** (after line 202, before closing brace):
```rust
fn get_causal_predecessors(&self, memory_id: &str) -> MemoryStream {
    (**self).get_causal_predecessors(memory_id)
}

fn get_causal_successors(&self, memory_id: &str) -> MemoryStream {
    (**self).get_causal_successors(memory_id)
}

fn trace_causal_chain_forward(
    &self,
    start_memory_id: &str,
    max_depth: usize,
) -> MemoryStream {
    (**self).trace_causal_chain_forward(start_memory_id, max_depth)
}

fn trace_causal_chain_backward(
    &self,
    start_memory_id: &str,
    max_depth: usize,
) -> MemoryStream {
    (**self).trace_causal_chain_backward(start_memory_id, max_depth)
}
```

---

### STEP 6: Implement Causal Query Methods

**File**: `packages/candle/src/memory/core/manager/surreal/operations.rs`  
**Location**: After line 728 (after `expand_via_entanglement()`, at the end of the impl block)

**Implementation pattern** (mirror get_entangled_memories):

```rust
fn get_causal_predecessors(&self, memory_id: &str) -> MemoryStream {
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let db = self.db.clone();
    let memory_id = memory_id.to_string();

    tokio::spawn(async move {
        // Query: SELECT in.* FROM memory_id<-caused
        // This gets all memories that caused this one
        let query = format!("SELECT in.* FROM {}<-caused", memory_id);

        match db.query(&query).await {
            Ok(mut response) => {
                let results: Vec<MemoryNodeSchema> = response.take(0).unwrap_or_default();

                for schema in results {
                    let memory = SurrealDBMemoryManager::from_schema(schema);
                    if tx.send(Ok(memory)).await.is_err() {
                        break;
                    }
                }
            }
            Err(e) => {
                let _ = tx.send(Err(Error::Database(format!("{:?}", e)))).await;
            }
        }
    });

    MemoryStream::new(rx)
}

fn get_causal_successors(&self, memory_id: &str) -> MemoryStream {
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let db = self.db.clone();
    let memory_id = memory_id.to_string();

    tokio::spawn(async move {
        // Query: SELECT out.* FROM memory_id->caused
        // This gets all memories that were caused by this one
        let query = format!("SELECT out.* FROM {}->caused", memory_id);

        match db.query(&query).await {
            Ok(mut response) => {
                let results: Vec<MemoryNodeSchema> = response.take(0).unwrap_or_default();

                for schema in results {
                    let memory = SurrealDBMemoryManager::from_schema(schema);
                    if tx.send(Ok(memory)).await.is_err() {
                        break;
                    }
                }
            }
            Err(e) => {
                let _ = tx.send(Err(Error::Database(format!("{:?}", e)))).await;
            }
        }
    });

    MemoryStream::new(rx)
}

fn trace_causal_chain_forward(
    &self,
    start_memory_id: &str,
    max_depth: usize,
) -> MemoryStream {
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let db = self.db.clone();
    let start_id = start_memory_id.to_string();

    tokio::spawn(async move {
        let safe_depth = max_depth.min(10); // Safety limit

        // Build chain: ->caused->memory->caused->memory...
        let mut chain = String::from("->caused");
        for _ in 1..safe_depth {
            chain.push_str("->memory->caused");
        }

        let query = format!(
            "SELECT DISTINCT out.* FROM {}{}",
            start_id, chain
        );

        match db.query(&query).await {
            Ok(mut response) => {
                let results: Vec<MemoryNodeSchema> = response.take(0).unwrap_or_default();

                for schema in results {
                    let memory = SurrealDBMemoryManager::from_schema(schema);
                    if tx.send(Ok(memory)).await.is_err() {
                        break;
                    }
                }
            }
            Err(e) => {
                let _ = tx.send(Err(Error::Database(format!("{:?}", e)))).await;
            }
        }
    });

    MemoryStream::new(rx)
}

fn trace_causal_chain_backward(
    &self,
    start_memory_id: &str,
    max_depth: usize,
) -> MemoryStream {
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let db = self.db.clone();
    let start_id = start_memory_id.to_string();

    tokio::spawn(async move {
        let safe_depth = max_depth.min(10); // Safety limit

        // Build chain: <-caused<-memory<-caused<-memory...
        let mut chain = String::from("<-caused");
        for _ in 1..safe_depth {
            chain.push_str("<-memory<-caused");
        }

        let query = format!(
            "SELECT DISTINCT in.* FROM {}{}",
            start_id, chain
        );

        match db.query(&query).await {
            Ok(mut response) => {
                let results: Vec<MemoryNodeSchema> = response.take(0).unwrap_or_default();

                for schema in results {
                    let memory = SurrealDBMemoryManager::from_schema(schema);
                    if tx.send(Ok(memory)).await.is_err() {
                        break;
                    }
                }
            }
            Err(e) => {
                let _ = tx.send(Err(Error::Database(format!("{:?}", e)))).await;
            }
        }
    });

    MemoryStream::new(rx)
}
```

---

### STEP 7: Implement in Coordinator (Delegation)

**File**: `packages/candle/src/memory/core/manager/coordinator/trait_impl.rs`  
**Location**: After line 114 (after the `expand_via_entanglement` method, before the Drop impl)

**Add delegating implementations**:
```rust
fn create_causal_edge(
    &self,
    source_id: &str,
    target_id: &str,
    strength: f32,
    temporal_distance: i64,
) -> PendingEntanglementEdge {
    self.surreal_manager
        .create_causal_edge(source_id, target_id, strength, temporal_distance)
}

fn get_causal_predecessors(&self, memory_id: &str) -> MemoryStream {
    self.surreal_manager.get_causal_predecessors(memory_id)
}

fn get_causal_successors(&self, memory_id: &str) -> MemoryStream {
    self.surreal_manager.get_causal_successors(memory_id)
}

fn trace_causal_chain_forward(
    &self,
    start_memory_id: &str,
    max_depth: usize,
) -> MemoryStream {
    self.surreal_manager
        .trace_causal_chain_forward(start_memory_id, max_depth)
}

fn trace_causal_chain_backward(
    &self,
    start_memory_id: &str,
    max_depth: usize,
) -> MemoryStream {
    self.surreal_manager
        .trace_causal_chain_backward(start_memory_id, max_depth)
}
```

---

### STEP 8: Remove Dead Code Markers

**File**: `packages/candle/src/domain/memory/cognitive/types/temporal.rs`  
**Location**: Line 99

**Remove**:
```rust
#[allow(dead_code)] // TODO: Implement causal reasoning in cognitive state system
```

---

## DEFINITION OF DONE

When implementation is complete:

- [ ] Added `caused` RELATE table to schema (manager.rs after line 153)
- [ ] Created `create_causal_edge()` method in MemoryManager trait (trait_def.rs after line 78)
- [ ] Implemented `create_causal_edge()` in SurrealDBMemoryManager (operations.rs after line 566)
- [ ] Wired up causal link storage in entanglement discovery (entanglement_discovery.rs after line 300)
- [ ] Added causal query methods to trait (trait_def.rs after line 108)
- [ ] Implemented causal query methods in operations.rs (after line 728)
- [ ] Added coordinator delegations for new methods (trait_impl.rs after line 114)
- [ ] Removed `#[allow(dead_code)]` from CausalLink::new() (temporal.rs line 99)
- [ ] `cargo check -p paraphym_candle` compiles without errors
- [ ] Causal links are now stored and queryable in database

---

## FILE MODIFICATION SUMMARY

```
packages/candle/src/
├── domain/memory/cognitive/types/
│   └── temporal.rs              # MODIFY: Remove dead_code marker (line 99)
├── memory/core/
│   ├── cognitive_worker/
│   │   └── entanglement_discovery.rs  # MODIFY: Add storage after line 300
│   └── manager/
│       ├── coordinator/
│       │   └── trait_impl.rs    # MODIFY: Add delegations after line 114
│       └── surreal/
│           ├── manager.rs       # MODIFY: Add caused table schema after line 153
│           ├── trait_def.rs     # MODIFY: Add methods after lines 78 and 108
│           └── operations.rs    # MODIFY: Add implementations after lines 566 and 728
```

---

## SURREALDB QUERY PATTERNS

### Graph Traversal Examples

```surrealql
-- Get what caused a specific memory
SELECT in.* FROM memory:some_id<-caused;

-- Get what a memory caused
SELECT out.* FROM memory:some_id->caused;

-- Trace causal chain forward (2 hops)
SELECT DISTINCT out.* FROM memory:some_id->caused->memory->caused;

-- Trace causal chain backward to root causes
SELECT DISTINCT in.* FROM memory:some_id<-caused<-memory<-caused;

-- Get causal links with metadata
SELECT * FROM caused WHERE strength > 0.8;

-- Get temporal causality (events within 1 second)
SELECT * FROM caused WHERE temporal_distance > 0 AND temporal_distance < 1000;
```

---

## ARCHITECTURAL NOTES

### Why Separate `caused` from `entangled`?

1. **Semantic Clarity**: Causal relationships are fundamentally different from general entanglement
2. **Query Performance**: Specialized queries don't need to filter entanglement_type
3. **Schema Simplicity**: `caused` has temporal_distance; `entangled` has entanglement_type  
4. **Future Extensibility**: Causal reasoning may need additional metadata (confidence intervals, causal mechanisms)

### Temporal Distance Interpretation

- **Positive**: `memory_id` happened BEFORE `related_id` (forward causality)
- **Negative**: `memory_id` happened AFTER `related_id` (backward inference)
- **Zero**: Simultaneous events (correlation, not causation)

The sign indicates temporal direction, the magnitude indicates how far apart in time.

### Entanglement Type Classification

Events < 1 second apart are classified as `Causal` entanglement type (see entanglement_discovery.rs lines 245-270). This is a heuristic threshold. The `caused` relation stores ALL causal links regardless of this classification, providing a complete causal graph.

---

## REFERENCES

### Existing Code Patterns
- Entanglement edge creation: [`operations.rs:533-566`](../packages/candle/src/memory/core/manager/surreal/operations.rs)
- Entanglement schema: [`manager.rs:140-153`](../packages/candle/src/memory/core/manager/surreal/manager.rs)
- Graph traversal: [`operations.rs:631-728`](../packages/candle/src/memory/core/manager/surreal/operations.rs)
- Trait pattern: [`trait_def.rs:15-204`](../packages/candle/src/memory/core/manager/surreal/trait_def.rs)
- Coordinator delegation: [`trait_impl.rs:60-115`](../packages/candle/src/memory/core/manager/coordinator/trait_impl.rs)

### SurrealDB Documentation
- [Graph Relations](https://surrealdb.com/docs/surrealql/statements/relate)
- [Graph Queries](https://surrealdb.com/docs/surrealql/statements/select#graph-queries)
