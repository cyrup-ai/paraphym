# Logical Error: Redundant State Clone in Worker Spawn

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs`

Lines 220-221, 273

## Severity
**LOW** - Code clarity and minor inefficiency

## Issue Description

The state Arc is cloned multiple times unnecessarily:

```rust
// Line 220-221
let state = Arc::new(AtomicU32::new(0)); // Spawning state
let state_clone = Arc::clone(&state);

// Line 273 - Inside worker context
state: Arc::clone(&state_clone),
```

This creates confusion:
1. `state` is created
2. `state_clone` is created (Arc clone #1)
3. `state_clone` is cloned again in context (Arc clone #2)
4. `state` is used in handle (line 309)

## Impact

- **Minor performance**: Extra Arc clone operations
- **Code clarity**: Confusing naming (`state_clone` suggests it's different)
- **Maintenance**: Harder to understand ownership flow

## Current Flow

```
state (Arc #1)
  ├─> state_clone (Arc #2) -> moved to task
  │     └─> Arc::clone(&state_clone) (Arc #3) -> worker context
  └─> state (Arc #1) -> worker handle
```

## Fix

Simplify to single clone:

```rust
// Create state
let state = Arc::new(AtomicU32::new(0));

// Clone once for task
let state_for_task = Arc::clone(&state);

// Spawn task
tokio::spawn(async move {
    // Use state_for_task directly
    state_for_task.store(WorkerState::Loading as u32, Ordering::Release);
    
    // ...
    
    text_embedding_worker(
        model,
        channels,
        TextEmbeddingWorkerContext {
            worker_id,
            registry_key: registry_key_clone,
            state: state_for_task,  // No extra clone needed
        },
    ).await;
});

// Use original state in handle
let full_handle = TextEmbeddingWorkerHandle {
    core: WorkerHandle {
        // ...
        state,  // Original Arc
    },
    // ...
};
```

## Recommendation

While this is a minor issue, cleaning it up improves code readability and makes the ownership model clearer.
