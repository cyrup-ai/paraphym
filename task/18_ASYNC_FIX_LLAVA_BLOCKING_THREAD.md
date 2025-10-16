# Task: Convert LLaVA Model Thread from Blocking to Async

## Location
`packages/candle/src/capability/vision/llava.rs` lines 295-350

## Problem
**Dedicated blocking thread with runtime.block_on()**

```rust
fn model_thread_with_config(
    model: LLaVA,
    tokenizer: Tokenizer,
    config: LLaVAModelConfig,
    context: LLaVAThreadContext,
) {
    let LLaVAThreadContext { request_rx, rt } = context;
    
    while let Ok(request) = request_rx.recv() {  // ❌ Blocking recv
        match request {
            LLaVARequest::Ask { .. } => {
                let result = rt.block_on(async {  // ❌ Blocking async
                    Self::process_ask(...).await
                });
                let _ = response_tx.send(result);
            }
            LLaVARequest::AskUrl { .. } => {
                let result = rt.block_on(async {  // ❌ Blocking async
                    Self::process_ask_url(...).await
                });
                let _ = response_tx.send(result);
            }
        }
    }
}
```

**Issues:**
1. Spawned as `std::thread` with dedicated tokio Runtime
2. Blocks thread on channel recv
3. Uses `rt.block_on()` to run async operations
4. Thread sits idle between requests

## Solution

Convert to async task:

```rust
async fn model_task_with_config(
    model: LLaVA,
    tokenizer: Tokenizer,
    config: LLaVAModelConfig,
    mut request_rx: mpsc::UnboundedReceiver<LLaVARequest>,
    // Remove rt - we're in tokio already
) {
    while let Some(request) = request_rx.recv().await {  // ✅ Async recv
        match request {
            LLaVARequest::Ask { 
                image_path,
                question,
                response_tx,
            } => {
                let result = Self::process_ask(  // ✅ Direct await
                    LLaVAModelRefs {
                        model: &model,
                        tokenizer: &tokenizer,
                        llava_config: &llava_config,
                        device: &device,
                    },
                    &image_path,
                    &question,
                    LLaVAConfigs {
                        image_config,
                        gen_config,
                    },
                ).await;
                let _ = response_tx.send(result);
            }
            LLaVARequest::AskUrl {
                image_url,
                question,
                response_tx,
            } => {
                let result = Self::process_ask_url(  // ✅ Direct await
                    LLaVAModelRefs {
                        model: &model,
                        tokenizer: &tokenizer,
                        llava_config: &llava_config,
                        device: &device,
                    },
                    &image_url,
                    &question,
                    LLaVAConfigs {
                        image_config,
                        gen_config,
                    },
                ).await;
                let _ = response_tx.send(result);
            }
        }
    }
}
```

## Changes Required

### 1. Remove Runtime from Context
```rust
// Before
struct LLaVAThreadContext {
    request_rx: Receiver<LLaVARequest>,
    rt: tokio::runtime::Runtime,  // ❌ Remove this
}

// After  
struct LLaVAThreadContext {
    request_rx: mpsc::UnboundedReceiver<LLaVARequest>,
    // No runtime needed - we're in tokio
}
```

### 2. Change Thread Spawn to Task Spawn

**Before** (around line 250):
```rust
std::thread::Builder::new()
    .name(format!("llava-model-{}", model_id))
    .spawn(move || {
        Self::model_thread_with_config(model, tokenizer, config, context);
    })?;
```

**After**:
```rust
tokio::spawn(async move {
    Self::model_task_with_config(model, tokenizer, config, request_rx).await;
});
```

### 3. Update Channel Types
Switch from `std::sync::mpsc` to `tokio::sync::mpsc` for async-compatible channels:
- `request_rx`: Use `tokio::sync::mpsc::UnboundedReceiver`
- All response channels should use tokio channels too

### 4. Remove Runtime Creation
Find where `rt: tokio::runtime::Runtime` is created and remove it.

## Benefits
- No dedicated thread sitting idle
- No `block_on` overhead
- Better integration with tokio scheduler
- Can cancel task cleanly
- Reduces thread count

## Steps
1. Rename `model_thread_with_config` → `model_task_with_config`
2. Make it `async fn`
3. Change signature to take `UnboundedReceiver` directly
4. Replace `recv()` with `recv().await`
5. Remove `rt.block_on()` wrappers - call async functions directly with `.await`
6. Remove `Runtime` from `LLaVAThreadContext`
7. Change `std::thread::spawn` to `tokio::spawn`
8. Update all channel types to tokio channels
9. Test vision model functionality

## Priority
🟡 **MEDIUM** - Not as critical as hot-path blocking, but still architectural improvement

## Status
⏳ TODO
