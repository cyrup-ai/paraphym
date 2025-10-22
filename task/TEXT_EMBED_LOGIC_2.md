# Logical Error: Redundant String Clone in Worker Spawn

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs`

Lines 209-210, 272

## Severity
**LOW** - Minor memory waste

## Issue Description

The registry_key is cloned multiple times:

```rust
// Line 209-210
let registry_key_clone = registry_key.to_string();
let registry_key_for_handle = registry_key_clone.clone(); // Clone of clone

// Line 272 - Inside worker context
registry_key: registry_key_clone.clone(),  // Another clone
```

## Impact

- **Memory**: 2 extra String allocations per worker spawn
- **Code clarity**: Confusing naming pattern

## Current Flow

```
registry_key (&str parameter)
  ├─> registry_key_clone (String #1)
  │     ├─> registry_key_clone.clone() (String #2) -> worker context
  │     └─> (String #1 dropped)
  └─> registry_key_for_handle (String #3) -> worker handle
```

## Fix

Use Arc<str> for shared ownership:

```rust
// Single allocation
let registry_key = Arc::<str>::from(registry_key);

// Cheap clones (just increment ref count)
let registry_key_for_task = Arc::clone(&registry_key);

tokio::spawn(async move {
    text_embedding_worker(
        model,
        channels,
        TextEmbeddingWorkerContext {
            worker_id,
            registry_key: registry_key_for_task,  // Arc clone
            state: state_for_task,
        },
    ).await;
});

// Use in handle
let full_handle = TextEmbeddingWorkerHandle {
    core: WorkerHandle { /* ... */ },
    registry_key: registry_key.to_string(),  // Convert back to String if needed
    // ...
};
```

Or simpler - just clone when needed:

```rust
let registry_key_str = registry_key.to_string();

tokio::spawn(async move {
    let registry_key_for_worker = registry_key_str.clone();
    // ...
});

let full_handle = TextEmbeddingWorkerHandle {
    registry_key: registry_key_str,
    // ...
};
```

## Recommendation

This is a minor optimization but worth fixing for code clarity.
