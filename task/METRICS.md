# Task: Automatic Cognitive Metrics Collection

**Status**: Ready for Execution
**Priority**: Medium
**Complexity**: Very Low

## Overview

Restore automatic cognitive metrics collection that was disabled. The `CognitiveStats` field ALREADY EXISTS in `CognitiveState` (state.rs:59) but is not being used. This task simply adds record_*() calls to existing methods to track cognitive operations automatically.

## Objective

Enable transparent metrics collection by:
1. Exposing the EXISTING stats field via getter
2. Adding record_*() calls in existing methods
3. NO new fields (stats already exists!)
4. NO user API changes
5. Metrics collected automatically during normal operations

## Key Principle: USE EXISTING FIELD

From domain/memory/cognitive/types/state.rs line 59:
```rust
pub struct CognitiveState {
    // ... other fields ...
    /// Statistics for monitoring cognitive performance
    stats: Arc<CognitiveStats>,  // ← ALREADY EXISTS!
}
```

The stats field is ALREADY THERE but:
- No getter to access it
- No calls to record_*() methods
- Metrics never collected

## Background: What's Already Built

From memory_items.rs lines 110-174, `CognitiveStats` is FULLY IMPLEMENTED:

```rust
pub struct CognitiveStats {
    pub working_memory_accesses: AtomicU64,
    pub long_term_memory_accesses: AtomicU64,
    pub quantum_operations: AtomicU64,
    pub attention_updates: AtomicU64,
    pub last_update_nanos: AtomicU64,
}

impl CognitiveStats {
    pub fn record_working_memory_access(&self) { ... }
    pub fn record_long_term_memory_access(&self) { ... }
    pub fn record_quantum_operation(&self) { ... }
    pub fn record_attention_update(&self) { ... }
}
```

All we need to do is:
1. Add getter to expose stats
2. Call the record_*() methods

## Technical Details

### File: packages/candle/src/domain/memory/cognitive/types/state.rs

**Location 1: Add stats() getter (after line 104, in impl CognitiveState block)**

```rust
/// Get cognitive statistics
#[inline]
#[must_use]
pub fn stats(&self) -> &Arc<CognitiveStats> {
    &self.stats
}
```

**Location 2: Record metric in update_activation_from_stimulus() (after line 423)**

Current code (lines 395-426):
```rust
pub fn update_activation_from_stimulus(
    &mut self,
    stimulus: Vec<f32>,
) -> Result<(), CognitiveError> {
    // ... validation and processing ...

    // Update attention weights based on activation energy
    self.attention_weights.update_from_energy(normalized_energy);

    Ok(())  // ← line 425
}
```

**Add BEFORE the Ok(()) at line 425:**
```rust
    // Record attention update metric
    self.stats.record_attention_update();

    Ok(())
```

### File: packages/candle/src/memory/core/manager/coordinator/operations.rs

**Location 3: Record long-term memory access (at start of get_memory(), after line 160)**

Current code (lines 160-165):
```rust
pub async fn get_memory(&self, memory_id: &str) -> Result<Option<MemoryNode>> {
    // Retrieve from SurrealDB
    let memory_node = match self.surreal_manager.get_memory(memory_id).await? {
        Some(node) => node,
        None => return Ok(None),
    };
```

**Add after line 160 (right after function signature):**
```rust
pub async fn get_memory(&self, memory_id: &str) -> Result<Option<MemoryNode>> {
    // Record long-term memory access
    self.cognitive_state.read().await.stats().record_long_term_memory_access();

    // Retrieve from SurrealDB
    let memory_node = match self.surreal_manager.get_memory(memory_id).await? {
```

**Location 4: Add public getter for stats (at end of operations.rs impl block)**

```rust
/// Get cognitive performance statistics
///
/// Returns atomic counters for cognitive operations. All counters are
/// thread-safe and can be read without blocking.
pub fn cognitive_stats(&self) -> Arc<CognitiveStats> {
    // Note: This is async read but we just clone the Arc, not reading the state
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            let state = self.cognitive_state.read().await;
            Arc::clone(state.stats())
        })
    })
}
```

## Architecture Flow

```
User calls coordinator.get_memory(id)
         │
         ▼
get_memory() starts
         │
         ├──> [NEW] Record long_term_memory_access
         │
         ├──> Retrieve from SurrealDB
         │
         └──> Process through cognitive state
                    │
                    └──> update_activation_from_stimulus()
                               │
                               └──> [NEW] Record attention_update

User can query metrics:
         │
         ▼
coordinator.cognitive_stats()
         │
         └──> Returns Arc<CognitiveStats> with atomic counters
```

## Implementation Checklist

### Phase 1: Expose Stats Field
- [ ] Add stats() getter to CognitiveState (state.rs after line 104)
- [ ] Verify it returns &Arc<CognitiveStats>

### Phase 2: Record Attention Updates
- [ ] Find update_activation_from_stimulus() in state.rs
- [ ] Add stats.record_attention_update() before Ok(()) at line 425

### Phase 3: Record Memory Access
- [ ] Find get_memory() in operations.rs (line 160)
- [ ] Add stats recording at function start
- [ ] Use read().await.stats().record_long_term_memory_access()

### Phase 4: Add Public Stats Getter
- [ ] Add cognitive_stats() method to MemoryCoordinator impl
- [ ] Use async read to access cognitive_state.stats()
- [ ] Return Arc<CognitiveStats> for thread-safe access

### Phase 5: Verification
- [ ] Run `cargo check -p paraphym_candle --color=never`
- [ ] Verify no new fields added
- [ ] Confirm stats are recorded automatically
- [ ] Check atomic operations compile correctly

## Success Criteria

✅ Stats field exposed via getter
✅ Attention updates recorded in update_activation_from_stimulus()
✅ Long-term memory accesses recorded in get_memory()
✅ Public cognitive_stats() getter available
✅ NO new fields added (using existing stats field)
✅ NO user configuration needed
✅ All counters thread-safe (AtomicU64)
✅ Zero public API changes (just adding getter)

## Non-Goals (Out of Scope)

❌ Adding tests or benchmarks
❌ Writing documentation
❌ Creating metrics dashboard
❌ Recording working memory access (not in current hotpath)
❌ Recording quantum operations (separate task if needed)
❌ Changing stats structure
❌ Adding new metric types

## Notes

- CognitiveStats is ALREADY in CognitiveState - no new fields needed!
- All operations use Ordering::Relaxed (no synchronization overhead)
- Counters are AtomicU64 - thread-safe lock-free reads
- Stats track current session only (reset on restart)
- Timestamp updates automatically on each record_*() call
- Working memory metrics can be added later when working memory is active
- Quantum operation metrics can be added when quantum routing is verified active
- This is the simplest task - just exposing and using existing infrastructure!
