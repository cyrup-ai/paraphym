# Memory Issue: Memory Leak on Worker Load Failure

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs`

Lines 245-258, 321

## Severity
**CRITICAL** - Memory leak on failure path

## Issue Description

When model loading fails, memory is cleaned up (line 255), but the worker handle is still registered (line 318).

```rust
// Line 245-258 - Inside spawned task
Err(e) => {
    log::error!("TextEmbedding worker {} failed: {}", worker_id, e);
    state_clone.store(WorkerState::Failed as u32, Ordering::Release);
    
    // Clean up memory tracking
    text_embedding_pool().remove_memory(per_worker_mb_clone);
    
    return; // Exit thread without running worker loop
}

// Line 318 - BEFORE task is spawned
self.register_worker(registry_key.to_string(), full_handle);

// Line 321 - AFTER task is spawned
self.add_memory(per_worker_mb);
```

## Race Condition

Timeline:
1. Worker handle registered (line 318)
2. Memory added (line 321)
3. Task spawned (line 224)
4. Model loading fails (line 245)
5. Memory removed (line 255)
6. **Worker handle still in pool with Failed state**

Result: Failed worker remains in pool forever.

## Impact

1. Pool contains dead workers
2. `validate_workers()` may not clean them up properly
3. Worker count appears higher than reality
4. May prevent new workers from spawning

## Fix Required

Register worker AFTER successful load, or ensure cleanup on failure:

```rust
// Option 1: Register after load
tokio::spawn(async move {
    let model = match model_loader().await {
        Ok(m) => m,
        Err(e) => {
            text_embedding_pool().remove_memory(per_worker_mb_clone);
            return;
        }
    };
    
    // Register AFTER successful load
    text_embedding_pool().register_worker(
        registry_key_clone.clone(),
        full_handle_clone
    );
    
    // Run worker
    text_embedding_worker(model, channels, context).await;
});

// Option 2: Cleanup on failure
// Add cleanup code in pool maintenance
```

## Recommendation

Option 1 is cleaner but requires restructuring. Option 2 is safer.
