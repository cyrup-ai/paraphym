# Remove block_on from cognitive_worker.rs:274, 347, 801 (HIGH)

**Locations:**
- `src/memory/core/cognitive_worker.rs:274` - process_committee_evaluation
- `src/memory/core/cognitive_worker.rs:347` - process_entanglement_discovery
- `src/memory/core/cognitive_worker.rs:801` - process_batch_evaluation

**Priority:** HIGH - Using spawn_blocking + block_on for async work

## Current Code Pattern

```rust
fn process_committee_evaluation(&self, memory_id: &str) {
    let memory_id = memory_id.to_string();
    let manager = self.memory_manager.clone();
    let evaluator = self.committee_evaluator.clone();
    let tracker = self.operation_tracker.clone();
    
    let op_id = tracker.start_operation(OperationType::CommitteeEvaluation, None);
    
    // WRONG: spawn_blocking + block_on for async work
    tokio::task::spawn_blocking(move || {
        if let Some(runtime) = crate::runtime::shared_runtime() {
            runtime.block_on(async move {  // ← Line 274
                let start_time = Instant::now();
                
                let mut memory = match manager.get_memory(&memory_id).await {
                    Ok(Some(mem)) => mem,
                    // ... handle async operations ...
                };
                
                // ... more async work ...
            });
        }
    });
}
```

## Problem: spawn_blocking + block_on for Async Work

These methods are fire-and-forget background workers. They:
1. Use `tokio::task::spawn_blocking` (for blocking CPU work)
2. Inside, use `runtime.block_on(async { ... })` (for async work)

This is wrong because:
- `spawn_blocking` is for CPU-bound blocking work, not async work
- `block_on` inside spawn_blocking is wasteful
- Should just use `tokio::spawn` with async block directly

## Solution: Use tokio::spawn with Async Block

```rust
fn process_committee_evaluation(&self, memory_id: &str) {
    let memory_id = memory_id.to_string();
    let manager = self.memory_manager.clone();
    let evaluator = self.committee_evaluator.clone();
    let tracker = self.operation_tracker.clone();
    
    let op_id = tracker.start_operation(OperationType::CommitteeEvaluation, None);
    
    // CORRECT: tokio::spawn with async block
    tokio::spawn(async move {
        let start_time = Instant::now();
        
        // No block_on - just .await!
        let mut memory = match manager.get_memory(&memory_id).await {
            Ok(Some(mem)) => mem,
            Ok(None) => {
                log::warn!("Memory {} not found", memory_id);
                tracker.fail_operation(op_id, "Memory not found".to_string());
                return;
            }
            Err(e) => {
                log::error!("Failed to fetch memory {}: {:?}", memory_id, e);
                tracker.fail_operation(op_id, format!("Fetch error: {:?}", e));
                return;
            }
        };
        
        // All async work uses .await directly
        match Self::evaluate_with_timeout_and_retry(
            evaluator,
            &memory.content.text,
            2,
        ).await {
            Ok(score) => {
                // ... handle success ...
            }
            Err(e) => {
                // ... handle error ...
            }
        }
    });
}
```

Apply same pattern to other two methods.

**Pattern Explanation:**
- **ANTIPATTERN (current):** `spawn_blocking(|| runtime.block_on(async { ... }))`
- **CORRECT (fix):** `tokio::spawn(async move { ... })` with `.await` directly

## Implementation Notes

1. Replace `tokio::task::spawn_blocking` with `tokio::spawn`
2. Change closure from `move || { ... }` to `async move { ... }`
3. Remove `if let Some(runtime) = shared_runtime()` wrapper
4. Remove `runtime.block_on(async move { ... })` wrapper
5. All async operations use `.await` directly
6. Apply to all 3 methods

## Files to Modify

- `src/memory/core/cognitive_worker.rs` - Update 3 process_* methods (lines 262, 336, 794)

## Benefits

1. No unnecessary spawn_blocking for async work
2. No runtime.block_on() overhead
3. Proper async task spawning
4. Cleaner code - fewer wrappers
