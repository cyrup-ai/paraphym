# ASYNC_THREADS: Convert std::thread::spawn to tokio::spawn

## OBJECTIVE
Replace all `std::thread::spawn` with `tokio::spawn` for 100% tokio async runtime.
No OS thread spawning - everything should use tokio's async task scheduler.

**Context:** Main application uses `#[tokio::main]` ([src/main.rs](../packages/candle/src/main.rs)), providing a tokio runtime from the start. All thread spawning should use tokio's cooperative task scheduling instead of OS threads.

---

## CORE PATTERNS

### Pattern 1: Background Loop with Sleep
```rust
// ❌ BEFORE: OS thread with blocking sleep
std::thread::spawn(move || {
    loop {
        std::thread::sleep(Duration::from_secs(1));
        do_work();
    }
});

// ✅ AFTER: Tokio task with async sleep
tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        do_work();
    }
});
```

### Pattern 2: Channel Receiver (Blocking)
```rust
// ❌ BEFORE: Blocking channel recv in thread
std::thread::spawn(move || {
    if rx.recv().is_ok() {
        handle_message();
    }
});

// ✅ AFTER: Async channel recv
tokio::spawn(async move {
    if let Some(msg) = rx.recv().await {
        handle_message();
    }
});
```

### Pattern 3: Simple Work Spawning
```rust
// ❌ BEFORE: Thread for simple work
std::thread::spawn(move || {
    let result = compute();
    tx.send(result);
});

// ✅ AFTER: Tokio task
tokio::spawn(async move {
    let result = compute();
    let _ = tx.send(result);
});
```

### Pattern 4: Runtime Handle (Not Creation)
```rust
// ❌ BEFORE: Creating separate runtime
let runtime = std::thread::spawn(|| Runtime::new()).join().unwrap();

// ✅ AFTER: Use existing runtime handle
let handle = tokio::runtime::Handle::current();
// or just use tokio::spawn directly
```

---

## FILES TO CONVERT (9 instances)

### 1. [src/runtime/mod.rs](../packages/candle/src/runtime/mod.rs) - Line 27

**Current Code:**
```rust
pub fn shared_runtime() -> Option<&'static Runtime> {
    SHARED_RUNTIME
        .get_or_init(|| {
            std::thread::spawn(|| Runtime::new().ok())
                .join()
                .ok()
                .flatten()
        })
        .as_ref()
}
```

**Problem:** Creating a separate runtime when `#[tokio::main]` already provides one. This causes nested runtime issues.

**Solution:** Remove this pattern entirely. Replace all `shared_runtime()` usages with `tokio::runtime::Handle::current()` or direct `tokio::spawn()`.

**Files using `shared_runtime()`:**
- [src/domain/tool/router.rs:181](../packages/candle/src/domain/tool/router.rs) - Line 181

**Required Changes:**
1. In `tool/router.rs`, replace:
```rust
let Some(runtime) = crate::runtime::shared_runtime() else { ... };
runtime.spawn(async move { ... });
```
With:
```rust
tokio::spawn(async move { ... });
```

2. Delete or deprecate `runtime/mod.rs` entirely since it's unnecessary.

---

### 2. [src/agent/prompt.rs](../packages/candle/src/agent/prompt.rs) - Line 128

**Current Code:**
```rust
fn drive_streams(mut self, sender: tokio::sync::mpsc::UnboundedSender<String>) {
    std::thread::spawn(move || {
        // ... agent completion loop with streams
    });
}
```

**Context:** Agent is processing completions in a synchronous loop. This is already inside an async context ([spawn_stream](../packages/candle/src/async_stream.rs)).

**Solution:** Make `drive_streams` async and use `tokio::spawn`:
```rust
async fn drive_streams(mut self, sender: tokio::sync::mpsc::UnboundedSender<String>) {
    tokio::spawn(async move {
        // ... agent completion loop (already async-compatible)
    });
}
```

**Note:** The completion streams inside are already async-friendly. Just remove the thread wrapper.

---

### 3. [src/pool/core/memory_governor.rs](../packages/candle/src/pool/core/memory_governor.rs) - Line 431

**Current Code:**
```rust
fn start_pressure_monitor(&self) {
    let governor = self.clone();
    let interval = self.config.pressure_check_interval;

    std::thread::spawn(move || {
        loop {
            std::thread::sleep(interval);
            
            // Refresh system memory
            {
                let mut sys = governor.system.write();
                sys.refresh_memory();
                // ...
            }
            
            governor.update_pressure();
            if governor.get_pressure() == MemoryPressure::Critical {
                governor.handle_critical_pressure();
            }
        }
    });
}
```

**Solution:** Convert to tokio task with async sleep:
```rust
fn start_pressure_monitor(&self) {
    let governor = self.clone();
    let interval = self.config.pressure_check_interval;

    tokio::spawn(async move {
        let mut interval_timer = tokio::time::interval(interval);
        loop {
            interval_timer.tick().await;
            
            // Refresh system memory (sync operations are fine in async context)
            {
                let mut sys = governor.system.write();
                sys.refresh_memory();
                // ...
            }
            
            governor.update_pressure();
            if governor.get_pressure() == MemoryPressure::Critical {
                governor.handle_critical_pressure();
            }
        }
    });
}
```

**Key Change:** `std::thread::sleep(interval)` → `tokio::time::interval(interval).tick().await`

---

### 4. [src/cli/runner.rs](../packages/candle/src/cli/runner.rs) - Line 74

**Current Code:**
```rust
// Spawn shutdown monitor thread
std::thread::spawn(move || {
    if shutdown_rx.recv().is_ok() {
        eprintln!("\nShutdown signal received, draining pools...");
        crate::pool::begin_shutdown(5);
        std::process::exit(0);
    }
});
```

**Context:** Using `std::sync::mpsc` channel. This is blocking.

**Solution Option 1 - Convert to tokio channel:**
```rust
// Change channel type from std::sync::mpsc to tokio::sync::mpsc
// Then:
tokio::spawn(async move {
    if let Some(_) = shutdown_rx.recv().await {
        eprintln!("\nShutdown signal received, draining pools...");
        crate::pool::begin_shutdown(5);
        std::process::exit(0);
    }
});
```

**Solution Option 2 - Use spawn_blocking (if channel can't change):**
```rust
tokio::task::spawn_blocking(move || {
    if shutdown_rx.recv().is_ok() {
        eprintln!("\nShutdown signal received, draining pools...");
        crate::pool::begin_shutdown(5);
        std::process::exit(0);
    }
});
```

**Recommended:** Option 1 (convert channel to tokio::sync::mpsc for full async)

---

### 5. [src/domain/agent/core.rs](../packages/candle/src/domain/agent/core.rs) - Line 327

**Current Code:**
```rust
Box::pin(crate::async_stream::spawn_stream(move |tx| async move {
    std::thread::spawn(move || {
        let memory_tool = MemoryTool::new(Arc::clone(&memory));
        
        let agent = Self {
            model,
            system_prompt,
            // ... field initialization
        };
        
        let _ = tx.send(CandleResult { result: Ok(agent) });
    });
}))
```

**Problem:** Spawning a thread inside an async context just to construct an agent. This is unnecessary.

**Solution:** Remove the thread entirely:
```rust
Box::pin(crate::async_stream::spawn_stream(move |tx| async move {
    let memory_tool = MemoryTool::new(Arc::clone(&memory));
    
    let agent = Self {
        model,
        system_prompt,
        // ... field initialization
    };
    
    let _ = tx.send(CandleResult { result: Ok(agent) });
}))
```

**Key Insight:** Agent construction is not blocking work. No thread needed.

---

### 6. [src/domain/chat/config.rs](../packages/candle/src/domain/chat/config.rs) - Line 853

**Current Code:**
```rust
// Initialize default validators using shared references
let validation_rules = manager.validation_rules.clone();
std::thread::spawn(move || {
    let mut rules = validation_rules
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    rules.insert("personality".into(), Arc::new(CandlePersonalityValidator));
    rules.insert("behavior".into(), Arc::new(CandleBehaviorValidator));
    rules.insert("ui".into(), Arc::new(CandleUIValidator));
});
```

**Problem:** Spawning thread just to insert into a HashMap. This is completely unnecessary.

**Solution:** Remove thread entirely - do it synchronously:
```rust
// Initialize default validators
{
    let mut rules = manager.validation_rules
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    rules.insert("personality".into(), Arc::new(CandlePersonalityValidator));
    rules.insert("behavior".into(), Arc::new(CandleBehaviorValidator));
    rules.insert("ui".into(), Arc::new(CandleUIValidator));
}
```

**Key Insight:** HashMap insertion is not blocking I/O. No async needed.

---

### 7. [src/domain/memory/cache.rs](../packages/candle/src/domain/memory/cache.rs) - Line 58

**Current Code:**
```rust
INIT.call_once(|| {
    update_cached_timestamp();
    
    // Start background thread for periodic updates
    std::thread::spawn(|| {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
            update_cached_timestamp();
        }
    });
});
```

**Solution:** Convert to tokio task with async sleep:
```rust
INIT.call_once(|| {
    update_cached_timestamp();
    
    // Start background task for periodic updates
    tokio::spawn(async {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(100));
        loop {
            interval.tick().await;
            update_cached_timestamp();
        }
    });
});
```

**Key Change:** `std::thread::sleep` → `tokio::time::interval().tick().await`

---

### 8. [src/domain/chat/realtime/connection.rs](../packages/candle/src/domain/chat/realtime/connection.rs) - Line 301

**Current Code:**
```rust
std::thread::spawn(move || {
    log::info!("Health check thread started...");
    
    while running.load(Ordering::Acquire) {
        // Check connections for staleness
        // ... health check logic
        
        std::thread::sleep(Duration::from_secs(1));
    }
});
```

**Solution:** Convert to tokio task:
```rust
tokio::spawn(async move {
    log::info!("Health check task started...");
    
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    while running.load(Ordering::Acquire) {
        interval.tick().await;
        
        // Check connections for staleness
        // ... health check logic (sync operations are fine)
    }
});
```

**Key Change:** Loop with sleep → interval.tick().await

---

### 9. [src/domain/chat/realtime/typing.rs](../packages/candle/src/domain/chat/realtime/typing.rs) - Line 315

**Current Code:**
```rust
Box::pin(crate::async_stream::spawn_stream(move |tx| async move {
    std::thread::spawn(move || {
        loop {
            let cleanup_interval = Duration::from_nanos(
                cleanup_interval_nanos.load(Ordering::Acquire)
            );
            std::thread::sleep(cleanup_interval);
            
            // Find and clean up expired typing states
            // ... cleanup logic
        }
    });
}))
```

**Solution:** Remove thread, use tokio task:
```rust
Box::pin(crate::async_stream::spawn_stream(move |tx| async move {
    tokio::spawn(async move {
        loop {
            let cleanup_interval = Duration::from_nanos(
                cleanup_interval_nanos.load(Ordering::Acquire)
            );
            tokio::time::sleep(cleanup_interval).await;
            
            // Find and clean up expired typing states
            // ... cleanup logic (sync operations are fine)
        }
    });
}))
```

**Key Change:** `std::thread::sleep` → `tokio::time::sleep().await`

---

## IMPLEMENTATION STRATEGY

### Step 1: Convert Simple Cases (No Dependencies)
- **File 5** (agent/core.rs): Remove thread wrapper
- **File 6** (config.rs): Remove thread, do synchronously

### Step 2: Convert Background Loops
- **File 3** (memory_governor.rs): Convert to tokio::time::interval
- **File 7** (cache.rs): Convert to tokio::time::interval
- **File 8** (connection.rs): Convert to tokio::time::interval
- **File 9** (typing.rs): Convert to tokio::time::sleep

### Step 3: Convert Channels
- **File 4** (cli/runner.rs): Convert to tokio::sync::mpsc OR use spawn_blocking

### Step 4: Convert Async Context Spawns
- **File 2** (agent/prompt.rs): Make function async, remove thread

### Step 5: Remove Shared Runtime Pattern
- **File 1** (runtime/mod.rs): Replace usages with Handle::current() or direct tokio::spawn
- Update tool/router.rs to use tokio::spawn directly

---

## COMMON PITFALLS TO AVOID

### ❌ Don't Use `block_on` in Async Context
```rust
// ❌ WRONG
tokio::spawn(async {
    runtime.block_on(async_work());  // Deadlock!
});

// ✅ RIGHT
tokio::spawn(async {
    async_work().await;
});
```

### ❌ Don't Spawn Threads for Quick Work
```rust
// ❌ WRONG
std::thread::spawn(|| { quick_calculation() });

// ✅ RIGHT
quick_calculation();  // Just do it synchronously
```

### ❌ Don't Mix std::sync and tokio channels
```rust
// ❌ WRONG (mixing sync channel with tokio task)
tokio::spawn(async {
    std_rx.recv();  // Blocks tokio executor!
});

// ✅ RIGHT
tokio::spawn(async {
    tokio_rx.recv().await;  // Cooperative async
});
```

---

## VERIFICATION

```bash
# Verify zero std::thread::spawn in src/
cd packages/candle && rg "std::thread::spawn" --type rust src/ | wc -l
# Expected: 0

# Verify tokio::spawn usage
cd packages/candle && rg "tokio::spawn" --type rust src/ | wc -l
# Expected: 9+

# Verify compilation
cargo check --package paraphym_candle
# Expected: Success
```

---

## DEFINITION OF DONE

- ✅ Zero `std::thread::spawn` in `src/` directory
- ✅ All background tasks use `tokio::spawn`
- ✅ All loops use `tokio::time::interval` or `tokio::time::sleep`
- ✅ All channels are `tokio::sync::mpsc` (or use `spawn_blocking` if unavoidable)
- ✅ Shared runtime pattern removed (use `Handle::current()` or direct `tokio::spawn`)
- ✅ Code compiles successfully
- ✅ No nested runtime warnings
- ✅ 100% tokio async runtime

---

## REFERENCES

- Main entry point: [src/main.rs](../packages/candle/src/main.rs) - Uses `#[tokio::main]`
- Async stream helper: [src/async_stream.rs](../packages/candle/src/async_stream.rs)
- Tokio documentation: https://docs.rs/tokio/latest/tokio/
- Tokio time: https://docs.rs/tokio/latest/tokio/time/
