# ASYNC_2: Remove Memory Stream Blocking

## OBJECTIVE
Refactor memory stream processing from blocking to proper async streaming to prevent thread starvation in memory-intensive operations.

## PROBLEM ANALYSIS

### Root Cause
The `inject_memory_context` function in [`packages/candle/src/domain/agent/chat.rs:440-470`](../packages/candle/src/domain/agent/chat.rs) uses a blocking pattern to consume an async stream, causing thread starvation:

1. **Line 440**: `ystream::spawn_task` spawns a sync thread
2. **Line 451**: `runtime.block_on` blocks the thread to consume async MemoryStream
3. **Line 439**: Uses `std::sync::mpsc` blocking channel for communication
4. **Line 473**: Uses `recv_timeout` blocking receive

This anti-pattern defeats async streaming and can cause thread pool exhaustion under load.

### Current Blocking Pattern
```rust
// ANTI-PATTERN: Blocking in spawned task
let (retrieval_tx, retrieval_rx) = std::sync::mpsc::channel::<Result<Vec<RetrievalResult>, String>>();

ystream::spawn_task(move || {
    let Some(runtime) = shared_runtime() else {
        let _ = retrieval_tx.send(Err("Runtime not initialized".to_string()));
        return;
    };
    
    runtime.block_on(async move {  // ❌ BLOCKS thread to consume async stream
        use futures_util::StreamExt;
        let mut stream = memory_stream;
        let mut results = Vec::new();
        
        while let Some(memory_result) = stream.next().await {
            if results.len() >= MAX_RELEVANT_MEMORIES {
                break;
            }
            if let Ok(memory_node) = memory_result {
                results.push(Self::memory_node_to_retrieval_result(&memory_node));
            }
        }
        
        let _ = retrieval_tx.send(Ok(results));
    });
});

// Blocking receive with timeout
let retrieval_results = match retrieval_rx.recv_timeout(Duration::from_millis(timeout_ms)) {
    Ok(Ok(results)) => results,
    // ...
};
```

## ARCHITECTURE CONTEXT

### MemoryStream Implementation
[`packages/candle/src/memory/core/manager/surreal.rs:289-308`](../packages/candle/src/memory/core/manager/surreal.rs)

```rust
/// A stream of memory nodes backed by tokio async channel
pub struct MemoryStream {
    rx: tokio::sync::mpsc::Receiver<Result<MemoryNode>>,
}

impl futures_util::Stream for MemoryStream {
    type Item = Result<MemoryNode>;
    
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}
```

**Key Insight**: MemoryStream implements `futures_util::Stream`, making it fully async-compatible. It should be consumed in an async context, not blocked on.

### ystream Pattern Reference
See [`/Volumes/samsung_t9/ystream/examples/with_channel_pattern.rs`](../../ystream/examples/with_channel_pattern.rs) for proper AsyncStream usage with `emit!()` macro.

### Tokio Async Patterns in Codebase
- [`packages/candle/src/domain/agent/core.rs:165`](../packages/candle/src/domain/agent/core.rs) - tokio::spawn for async tasks
- [`packages/candle/src/memory/monitoring/health.rs:195`](../packages/candle/src/memory/monitoring/health.rs) - tokio::time::timeout for async timeouts

## SOLUTION APPROACH

### Pattern: Async Task with Timeout
Replace blocking pattern with full async streaming using tokio primitives:

```rust
// ✅ CORRECT: Full async pattern
let (retrieval_tx, mut retrieval_rx) = tokio::sync::mpsc::channel::<Result<Vec<RetrievalResult>, String>>(1);

// Spawn async task to consume MemoryStream
tokio::spawn(async move {
    use futures_util::StreamExt;
    
    let mut stream = memory_stream;
    let mut results = Vec::new();
    
    // Asynchronously collect from stream
    while let Some(memory_result) = stream.next().await {
        if results.len() >= MAX_RELEVANT_MEMORIES {
            break;
        }
        if let Ok(memory_node) = memory_result {
            results.push(Self::memory_node_to_retrieval_result(&memory_node));
        }
    }
    
    let _ = retrieval_tx.send(Ok(results)).await;
});

// Async timeout instead of blocking recv_timeout
let timeout_duration = Duration::from_millis(Self::get_memory_timeout_ms(timeout_ms));

let retrieval_results = match tokio::time::timeout(timeout_duration, retrieval_rx.recv()).await {
    Ok(Some(Ok(results))) => results,
    Ok(Some(Err(e))) => {
        log::error!("Memory retrieval failed: {e}");
        Vec::new()
    }
    Ok(None) => {
        log::error!("Channel closed unexpectedly");
        Vec::new()
    }
    Err(_) => {
        log::warn!("Memory retrieval timed out - context may be incomplete");
        Vec::new()
    }
};
```

### Integration with ystream::AsyncStream::with_channel

The function operates within `ystream::AsyncStream::with_channel` which provides a sync context. To bridge async operations:

```rust
pub fn inject_memory_context(
    &self,
    message: &str,
    memory_manager: &Arc<dyn MemoryManager>,
    timeout_ms: Option<u64>,
) -> ystream::AsyncStream<ContextInjectionResult> {
    let message = message.to_string();
    let memory_manager_clone = memory_manager.clone();

    ystream::AsyncStream::with_channel(move |sender| {
        let memory_stream = memory_manager_clone.search_by_content(&message);
        
        // Create tokio channel for async communication
        let (retrieval_tx, mut retrieval_rx) = tokio::sync::mpsc::channel(1);
        
        // Spawn async task
        tokio::spawn(async move {
            // ... async stream consumption ...
        });
        
        // Bridge async result to sync context with block_in_place
        let retrieval_results = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                match tokio::time::timeout(timeout_duration, retrieval_rx.recv()).await {
                    // ... timeout handling ...
                }
            })
        });
        
        // Process results and emit via ystream sender
        let result = ContextInjectionResult { /* ... */ };
        let _ = sender.send(result);
    })
}
```

**Note**: We still need `block_in_place` to bridge from sync `with_channel` context to async, BUT the critical difference is:
- ❌ Old: Blocks to consume stream items sequentially
- ✅ New: Blocks only to await already-spawned async task, stream consumption is async

## IMPLEMENTATION STEPS

### Step 1: Update Channel Type
**File**: `packages/candle/src/domain/agent/chat.rs:439`

Replace:
```rust
let (retrieval_tx, retrieval_rx) = std::sync::mpsc::channel::<Result<Vec<RetrievalResult>, String>>();
```

With:
```rust
let (retrieval_tx, mut retrieval_rx) = tokio::sync::mpsc::channel::<Result<Vec<RetrievalResult>, String>>(1);
```

### Step 2: Replace spawn_task with tokio::spawn
**File**: `packages/candle/src/domain/agent/chat.rs:441-470`

Remove the entire `ystream::spawn_task` block and runtime check. Replace with:
```rust
tokio::spawn(async move {
    use futures_util::StreamExt;
    
    let mut stream = memory_stream;
    let mut results = Vec::new();
    
    while let Some(memory_result) = stream.next().await {
        if results.len() >= MAX_RELEVANT_MEMORIES {
            break;
        }
        if let Ok(memory_node) = memory_result {
            results.push(Self::memory_node_to_retrieval_result(&memory_node));
        }
    }
    
    let _ = retrieval_tx.send(Ok(results)).await;
});
```

### Step 3: Replace Blocking Receive with Async Timeout
**File**: `packages/candle/src/domain/agent/chat.rs:473-487`

Replace:
```rust
let retrieval_results = match retrieval_rx.recv_timeout(
    std::time::Duration::from_millis(timeout_ms)
) {
    Ok(Ok(results)) => results,
    Ok(Err(e)) => {
        log::error!("Memory retrieval failed: {e}");
        Vec::new()
    }
    Err(_) => {
        log::warn!("Memory retrieval timed out - context may be incomplete");
        Vec::new()
    }
};
```

With:
```rust
let retrieval_results = tokio::task::block_in_place(|| {
    tokio::runtime::Handle::current().block_on(async {
        let timeout_duration = std::time::Duration::from_millis(timeout_ms);
        
        match tokio::time::timeout(timeout_duration, retrieval_rx.recv()).await {
            Ok(Some(Ok(results))) => results,
            Ok(Some(Err(e))) => {
                log::error!("Memory retrieval failed: {e}");
                Vec::new()
            }
            Ok(None) => {
                log::error!("Memory retrieval channel closed unexpectedly");
                Vec::new()
            }
            Err(_) => {
                log::warn!(
                    "Memory retrieval timed out - context may be incomplete (timeout_ms: {}, message: {:?})",
                    timeout_ms, message
                );
                Vec::new()
            }
        }
    })
});
```

### Step 4: Verify Error Handling
Ensure error propagation works correctly:
- Timeout errors log appropriately
- Channel close errors are detected
- Stream errors are caught and logged

## DEFINITION OF DONE

### Success Criteria
1. ✅ No `runtime.block_on` inside `ystream::spawn_task`
2. ✅ `tokio::spawn` used for async stream consumption
3. ✅ `tokio::sync::mpsc` used instead of `std::sync::mpsc`
4. ✅ `tokio::time::timeout` used for async timeout handling
5. ✅ `futures_util::StreamExt` used for stream operations
6. ✅ Code compiles without warnings
7. ✅ Proper error handling for timeout, channel close, and stream errors

### Verification
```bash
# Check compilation
cargo check -p paraphym_candle

# Verify no blocking patterns remain
rg "runtime\.block_on" packages/candle/src/domain/agent/chat.rs
# Should return no results at line 451

# Verify tokio::spawn is used
rg "tokio::spawn" packages/candle/src/domain/agent/chat.rs  
# Should show the new async task

# Verify async channel
rg "tokio::sync::mpsc" packages/candle/src/domain/agent/chat.rs
# Should show the channel creation
```

## RELATED CODE LOCATIONS

- Memory stream source: [`packages/candle/src/memory/core/manager/coordinator.rs:1008`](../packages/candle/src/memory/core/manager/coordinator.rs)
- MemoryStream definition: [`packages/candle/src/memory/core/manager/surreal.rs:289`](../packages/candle/src/memory/core/manager/surreal.rs)
- Tokio spawn example: [`packages/candle/src/domain/agent/core.rs:165`](../packages/candle/src/domain/agent/core.rs)
- Timeout pattern example: [`packages/candle/src/memory/monitoring/health.rs:195`](../packages/candle/src/memory/monitoring/health.rs)
- ystream patterns: [`/Volumes/samsung_t9/ystream/examples/`](../../ystream/examples/)

## KEY INSIGHTS

1. **MemoryStream is already async** - It implements `futures::Stream` with tokio channel, so blocking defeats its design
2. **Thread starvation occurs** - Using `block_on` inside spawned tasks exhausts thread pool under load  
3. **ystream context is sync** - We need `block_in_place` to bridge, but only for the final await, not stream consumption
4. **Timeout must be async** - `tokio::time::timeout` provides proper async timeout without blocking
5. **Pattern is established** - The codebase already uses tokio::spawn and async timeouts elsewhere; follow that pattern