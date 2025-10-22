# Task: Text Embedding Worker — Remove Redundant Arc Clone

## User Objective
- **Clarify ownership** and **remove unnecessary Arc cloning** in the text embedding worker spawn path without changing runtime behavior.
- **Keep scope minimal**: only update the text embedding capability implementation.

## Look Around (What Already Exists)
- **Pool + Worker architecture** already implemented in `candle`.
  - **Source file**: [../packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs](../packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs)
  - **Related traits**: [../packages/candle/src/capability/traits.rs](../packages/candle/src/capability/traits.rs)
  - **Worker state type**: [../packages/candle/src/capability/registry/pool/core/worker_state.rs](../packages/candle/src/capability/registry/pool/core/worker_state.rs)
- **Redundant Arc clone pattern** observed here (this task) and similarly in other capabilities (e.g., `image_embedding`, `text_to_text`). Do not broaden scope—fix only `text_embedding.rs` in this task.

## Current Behavior (Problem Summary)
- In `spawn_text_embedding_worker(...)`, state is created, immediately cloned, and that clone is cloned again when building the worker context. The original `state` is kept by the handle.
- This extra intermediate clone (`state_clone`) adds noise without functional benefit.

## Source of Truth (Paths & Symbols)
- **File**: [../packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs](../packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs)
- **Function**: `spawn_text_embedding_worker<T, F, Fut>(...)`
- **Context struct**: `TextEmbeddingWorkerContext { worker_id, registry_key, state: Arc<AtomicU32> }`
- **Worker entry**: `text_embedding_worker(model, TextEmbeddingWorkerChannels { ... }, TextEmbeddingWorkerContext { ... })`

## Minimal Change Plan (Exact Edits)
- **Goal**: Keep one original `Arc<AtomicU32>` (`state`) for the handle, and a single clone moved into the spawned task, used directly by the worker context.

1) In `spawn_text_embedding_worker(...)`, replace the intermediate clone with an intent-revealing single clone for the task.

Before:
```rust
let state = Arc::new(AtomicU32::new(0)); // Spawning state
let state_clone = Arc::clone(&state);
// ...
TextEmbeddingWorkerContext {
    worker_id,
    registry_key: registry_key_clone.clone(),
    state: Arc::clone(&state_clone),
},
```

After:
```rust
let state = Arc::new(AtomicU32::new(0)); // Spawning state
let state_for_task = Arc::clone(&state);
// ...
TextEmbeddingWorkerContext {
    worker_id,
    registry_key: registry_key_clone.clone(),
    state: state_for_task,
},
```

2) Inside the spawned task (where state transitions occur), **rename all occurrences** of the cloned state variable to the new binding:
   - Replace `state_clone.store(...)` with `state_for_task.store(...)` for all transitions: `Loading`, `Ready`, `Failed`, and final `Dead`.
   - This keeps pre-worker-load transitions and post-worker-exit transitions operating on the same `Arc` moved into the context.

3) The `WorkerHandle` stored in `full_handle.core` should continue referencing the original `state` created before spawn. No change to handle construction is needed.

- **Net effect**: The intermediate `state_clone` binding is removed. There is exactly one `Arc::clone(...)` for the worker task, and the original `state` remains in the handle.

## Rationale (Why Safe)
- **Arc semantics**: Cloning `Arc<T>` is O(1) and safe; avoiding unnecessary clones improves clarity. See references below.
- **Behavior preserved**: The same two references exist—one in the handle (`state`), one in the worker task (`state_for_task`). All state transitions target the same underlying atomic value.
- **No API change**: Only internal variable bindings are simplified.

## Definition of Done
- **File** [../packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs](../packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs)
  - Uses a single, clearly named clone (`state_for_task`) for the spawned task.
  - Does not call `Arc::clone(&state_clone)` when building `TextEmbeddingWorkerContext`.
  - Replaces all `state_clone.*` usages inside the spawned task with `state_for_task.*`.
  - Maintains original `state` in `WorkerHandle`.
  - No remaining identifier named `state_clone` in this file.
- **Builds successfully** with no functional changes to the pool/worker behavior.

## References (Relative Links)
- **Source file (edit target)**: [../packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs](../packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs)
- **Arc + Atomics notes**: [../tmp/references/rust_arc_atomic_notes.md](../tmp/references/rust_arc_atomic_notes.md)
- **Worker state type**: [../packages/candle/src/capability/registry/pool/core/worker_state.rs](../packages/candle/src/capability/registry/pool/core/worker_state.rs)

## Questions / Assumptions
- **Question**: Any diagnostics or metrics expecting specific clone naming in logs? Assumed no; variable renaming is local.
- **Assumption**: Idle/ready/processing transitions observed in `text_embedding_worker(...)` remain unaffected by the ownership simplification.
- **Assumption**: Scope remains limited to text embedding capability; other capabilities will be handled in separate tasks if needed.

## Third-Party Libraries
- **None required** for this change.
