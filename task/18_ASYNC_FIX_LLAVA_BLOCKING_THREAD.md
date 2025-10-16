# Task: Complete LLaVA Async Conversion - Remove Remaining Blocking Code

## Status: 60% Complete - Critical Issues Remain

## What's Been Completed ✅

1. ✅ All `std::sync::mpsc` replaced with `tokio::sync::mpsc::unbounded_channel()`
2. ✅ `LLaVAThreadContext` struct deleted entirely
3. ✅ `model_thread_with_config` renamed to `model_task_with_config` and marked `async fn`
4. ✅ Worker loop fully async: uses `recv().await` and direct `.await` on process functions
5. ✅ Tokio channels used throughout (enum, struct, response channels)
6. ✅ Code compiles without errors

## Critical Issues Remaining ❌

### ISSUE #1: Still Uses `spawn_blocking` Instead of `tokio::spawn`

**Location**: Line 210 in `packages/candle/src/capability/vision/llava.rs`

**Current Code**:
```rust
tokio::task::spawn_blocking(move || {
    let Some(rt) = crate::runtime::shared_runtime() else {
        log::error!("Shared runtime unavailable for LLaVA worker initialization");
        let _ = init_tx.send(Err("Runtime unavailable".to_string()));
        return;
    };
    
    let local = tokio::task::LocalSet::new();
    rt.block_on(local.run_until(async move {
        // ... model loading and worker spawn ...
    }));
});
```

**Required**:
```rust
tokio::spawn(async move {
    // Direct async model loading
    let tokenizer = Tokenizer::from_file(&tokenizer_path)?;
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(...)?; };
    let model = LLaVA::load(vb, &llava_config, None)?;
    
    // Direct async worker spawn
    Self::model_task_with_config(model, tokenizer, config, request_rx).await;
});
```

**Problem**: Using `spawn_blocking` defeats the purpose of async conversion. The task should run on the tokio async scheduler, not the blocking thread pool.

### ISSUE #2: `rt.block_on()` Still Present in Initialization

**Location**: Line 221

**Current**: `rt.block_on(local.run_until(async move {`

**Required**: Complete removal. No `rt.block_on()` calls anywhere.

### ISSUE #3: Runtime Reference Still Retrieved

**Location**: Lines 212-216

**Current**: `let Some(rt) = crate::runtime::shared_runtime() else {`

**Required**: No runtime reference needed. The spawned async task runs on the ambient runtime.

## Critical Constraint: !Send Model

**The LLaVA model is `!Send`** due to Candle's `Module` trait. This means:
- Model cannot be moved between threads
- Model cannot be used in regular `tokio::spawn` without additional work

## Possible Solutions

### Option 1: Keep Model in Single Async Task (Recommended)
If the model MUST stay `!Send`, use `tokio::task::spawn_local` within a LocalSet that's managed by the main async runtime, NOT in a blocking context.

### Option 2: Refactor to Box Model Operations
Box all model operations as trait objects that ARE Send, allowing use of regular `tokio::spawn`.

### Option 3: Use Arc<Mutex<Model>>
Wrap the model in `Arc<Mutex<>>` to make the handle `Send`, though this may impact performance.

## Definition of Done

This task is complete when:

1. ❌ `spawn_blocking` removed - using `tokio::spawn` or `spawn_local` within async context
2. ❌ **ALL** `rt.block_on()` calls removed from the file
3. ❌ Runtime reference (`crate::runtime::shared_runtime()`) removed completely
4. ✅ Worker loop remains fully async (already done)
5. ✅ Code compiles without errors (already done)
6. ⚠️  LLaVA model processes requests successfully (needs runtime testing)

## Files to Modify

- `packages/candle/src/capability/vision/llava.rs` (lines 210-277)

## Priority

**HIGH** - Blocking calls in async code create performance bottlenecks and defeat the purpose of this refactor.
