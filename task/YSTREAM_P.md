# YSTREAM_P: Chat Commands & Execution

## OBJECTIVE
Convert AsyncStream usage to tokio Stream in chat command execution modules, completing the async stream migration for the command execution subsystem.

## FILES TO CONVERT

### 1. `packages/candle/src/domain/chat/commands/execution.rs`
**AsyncStream Usage:** 4 occurrences  
**Locations:**
- Line 101: `execute_streaming()` return type and implementation
- Line 483: `execute_config_streaming()` return type and implementation  
- Line 550: `execute_search_streaming()` return type and implementation
- Line 739: `parse_and_execute()` return type and implementation

**Pattern:** All use `AsyncStream::with_channel` with `std::thread::spawn` and `ystream::emit!` macros  
**Crossbeam:** Line 8 has `crossbeam_utils::CachePadded` - **DO NOT CHANGE** (only removing crossbeam_queue)

### 2. `packages/candle/src/domain/chat/commands/mod.rs`
**AsyncStream Usage:** 0 occurrences  
**Status:** ✅ Already converted or no AsyncStream usage

### 3. `packages/candle/src/domain/chat/commands/types/mod.rs`
**AsyncStream Usage:** ~24 occurrences  
**Locations:**
- Line 123: `DomainCommandExecutor::execute()` trait method return type
- Line 189: `DomainCommandExecutorEnum::execute()` return type
- Lines 654-1200: 20+ `AsyncStream::with_channel` implementations in all domain executor impls

**Pattern:** Trait signature + many simple `AsyncStream::with_channel(|sender| { ... })` implementations

## TECHNICAL CONTEXT

### Current AsyncStream Helper (Reference)
See [`packages/candle/src/async_stream.rs`](../packages/candle/src/async_stream.rs):
```rust
pub fn spawn_stream<T, F, Fut>(f: F) -> impl Stream<Item = T>
where
    T: Send + 'static,
    F: FnOnce(mpsc::UnboundedSender<T>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    let (tx, rx) = mpsc::unbounded_channel();
    tokio::spawn(f(tx));
    UnboundedReceiverStream::new(rx)
}
```

### Related Conversions (Already Complete)
- ✅ YSTREAM_M: Chat core modules (config, conversation, formatting, macros, message_processing)
- ✅ YSTREAM_N: CLI runner and agent role builder
- ✅ YSTREAM_O: Builder files (agent_builder, audio, completion_response_builder, image)

## CONVERSION PATTERNS

### Pattern 1: Method with Thread Spawn + emit! (execution.rs)

These methods spawn std::thread and use ystream::emit! macro to send events.

**BEFORE:**
```rust
use ystream::AsyncStream;

pub fn execute_streaming(
    &self,
    _execution_id: u64,
    command: ImmutableChatCommand,
) -> AsyncStream<CommandEvent> {
    let self_clone = self.clone();
    
    AsyncStream::with_channel(move |sender| {
        std::thread::spawn(move || {
            // ... logic ...
            ystream::emit!(sender, CommandEvent::Started { /* ... */ });
            ystream::emit!(sender, CommandEvent::Output { /* ... */ });
            ystream::emit!(sender, CommandEvent::Completed { /* ... */ });
        });
    })
}
```

**AFTER:**
```rust
use std::pin::Pin;
use tokio_stream::Stream;

pub fn execute_streaming(
    &self,
    _execution_id: u64,
    command: ImmutableChatCommand,
) -> Pin<Box<dyn Stream<Item = CommandEvent> + Send>> {
    let self_clone = self.clone();
    
    Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
        // Convert std::thread::spawn to tokio::spawn
        tokio::spawn(async move {
            // ... logic ...
            let _ = sender.send(CommandEvent::Started { /* ... */ });
            let _ = sender.send(CommandEvent::Output { /* ... */ });
            let _ = sender.send(CommandEvent::Completed { /* ... */ });
        });
    }))
}
```

**Key Changes:**
1. Return type: `AsyncStream<T>` → `Pin<Box<dyn Stream<Item = T> + Send>>`
2. Replace `AsyncStream::with_channel` with `Box::pin(crate::async_stream::spawn_stream(...))`
3. Add `async move` to outer closure
4. Convert `std::thread::spawn` to `tokio::spawn(async move { ... })`
5. Replace `ystream::emit!(sender, event)` with `let _ = sender.send(event)`
6. Update imports

### Pattern 2: Simple Synchronous Channel (types/mod.rs executors)

Many executor implementations have simple sync closures that just send a result.

**BEFORE:**
```rust
fn execute(&self, _context: &CommandExecutionContext) -> AsyncStream<CommandExecutionResult> {
    AsyncStream::with_channel(|sender| {
        let result = CommandExecutionResult::Success("Command executed".to_string());
        let _ = sender.send(result);
    })
}
```

**AFTER:**
```rust
use std::pin::Pin;
use tokio_stream::Stream;

fn execute(&self, _context: &CommandExecutionContext) -> Pin<Box<dyn Stream<Item = CommandExecutionResult> + Send>> {
    Box::pin(crate::async_stream::spawn_stream(|sender| async move {
        let result = CommandExecutionResult::Success("Command executed".to_string());
        let _ = sender.send(result);
    }))
}
```

**Key Changes:**
1. Return type updated
2. Replace `AsyncStream::with_channel` with `Box::pin(crate::async_stream::spawn_stream(...))`
3. Add `async move` to closure (even though body is sync)

### Pattern 3: Trait Definition Return Type (types/mod.rs)

Trait method signatures need return type update.

**BEFORE:**
```rust
pub trait DomainCommandExecutor: Send + Sync + 'static {
    fn execute(&self, context: &CommandExecutionContext) -> AsyncStream<CommandExecutionResult>;
}
```

**AFTER:**
```rust
use std::pin::Pin;
use tokio_stream::Stream;

pub trait DomainCommandExecutor: Send + Sync + 'static {
    fn execute(&self, context: &CommandExecutionContext) -> Pin<Box<dyn Stream<Item = CommandExecutionResult> + Send>>;
}
```

## SPECIFIC CHANGES REQUIRED

### File: `execution.rs`

**Lines 1-15:** Update imports
```rust
// REMOVE these lines (if present)
use ystream::AsyncStream;

// KEEP these existing imports
use std::pin::Pin;
use tokio_stream::Stream;

// Note: Keep crossbeam_utils::CachePadded on line 8 - DO NOT REMOVE
```

**Line 101:** Convert execute_streaming() signature
```rust
// BEFORE
pub fn execute_streaming(
    &self,
    _execution_id: u64,
    command: ImmutableChatCommand,
) -> AsyncStream<CommandEvent> {

// AFTER
pub fn execute_streaming(
    &self,
    _execution_id: u64,
    command: ImmutableChatCommand,
) -> Pin<Box<dyn Stream<Item = CommandEvent> + Send>> {
```

**Lines 105-320:** Convert execute_streaming() body
```rust
// BEFORE
AsyncStream::with_channel(move |sender| {
    std::thread::spawn(move || {
        // ... execution logic with ystream::emit! calls ...
        ystream::emit!(sender, CommandEvent::Started { /* ... */ });
        // ... many more emit! calls ...
    });
})

// AFTER
Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
    tokio::spawn(async move {
        // ... same execution logic but replace emit! with send ...
        let _ = sender.send(CommandEvent::Started { /* ... */ });
        // ... convert all ystream::emit!(sender, event) to let _ = sender.send(event) ...
    });
}))
```

**Important:** Replace ALL `ystream::emit!(sender, event)` calls with `let _ = sender.send(event)` throughout the method.

**Line 483:** Convert execute_config_streaming()
```rust
// BEFORE
pub fn execute_config_streaming( /* ... */ ) -> AsyncStream<CommandEvent> {
    AsyncStream::with_channel(move |sender| {
        std::thread::spawn(move || {
            ystream::emit!(sender, CommandEvent::Started { /* ... */ });
            // ... more emit! calls ...
        });
    })
}

// AFTER  
pub fn execute_config_streaming( /* ... */ ) -> Pin<Box<dyn Stream<Item = CommandEvent> + Send>> {
    Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
        tokio::spawn(async move {
            let _ = sender.send(CommandEvent::Started { /* ... */ });
            // ... convert all emit! calls ...
        });
    }))
}
```

**Line 550:** Convert execute_search_streaming()  
- Same pattern as execute_config_streaming()
- Replace return type, AsyncStream::with_channel, std::thread::spawn, and emit! calls

**Line 739:** Convert parse_and_execute()
```rust
// BEFORE
pub fn parse_and_execute(&self, input: &str) -> AsyncStream<CommandEvent> {
    // ...
    AsyncStream::with_channel(move |sender| {
        // No thread spawn here - direct emit! calls
        ystream::emit!(sender, CommandEvent::Started { /* ... */ });
        ystream::emit!(sender, CommandEvent::Output { /* ... */ });
    })
}

// AFTER
pub fn parse_and_execute(&self, input: &str) -> Pin<Box<dyn Stream<Item = CommandEvent> + Send>> {
    // ...
    Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
        // No tokio spawn needed - direct send calls  
        let _ = sender.send(CommandEvent::Started { /* ... */ });
        let _ = sender.send(CommandEvent::Output { /* ... */ });
    }))
}
```

### File: `types/mod.rs`

**Top of file:** Update imports
```rust
// ADD these imports if not present
use std::pin::Pin;
use tokio_stream::Stream;
```

**Line 123:** Update trait method signature
```rust
// BEFORE
pub trait DomainCommandExecutor: Send + Sync + 'static {
    fn execute(&self, context: &CommandExecutionContext) -> AsyncStream<CommandExecutionResult>;
    // ... other methods ...
}

// AFTER
pub trait DomainCommandExecutor: Send + Sync + 'static {
    fn execute(&self, context: &CommandExecutionContext) -> Pin<Box<dyn Stream<Item = CommandExecutionResult> + Send>>;
    // ... other methods unchanged ...
}
```

**Line 189:** Update enum execute() signature
```rust
// BEFORE
pub fn execute(
    &self,
    context: &CommandExecutionContext,
) -> AsyncStream<CommandExecutionResult> {

// AFTER
pub fn execute(
    &self,
    context: &CommandExecutionContext,
) -> Pin<Box<dyn Stream<Item = CommandExecutionResult> + Send>> {
```

**Lines 654-1200:** Convert ALL executor implementations (20+ occurrences)

Each executor follows this pattern:
```rust
// BEFORE
impl DomainCommandExecutor for DomainHelpExecutor {
    fn execute(&self, _context: &CommandExecutionContext) -> AsyncStream<CommandExecutionResult> {
        AsyncStream::with_channel(|sender| {
            let result = CommandExecutionResult::Success("Help text".to_string());
            let _ = sender.send(result);
        })
    }
    // ... other methods ...
}

// AFTER
impl DomainCommandExecutor for DomainHelpExecutor {
    fn execute(&self, _context: &CommandExecutionContext) -> Pin<Box<dyn Stream<Item = CommandExecutionResult> + Send>> {
        Box::pin(crate::async_stream::spawn_stream(|sender| async move {
            let result = CommandExecutionResult::Success("Help text".to_string());
            let _ = sender.send(result);
        }))
    }
    // ... other methods unchanged ...
}
```

**Executors to convert:** DomainHelpExecutor, DomainClearExecutor, DomainExportExecutor, DomainConfigExecutor, DomainTemplateExecutor, DomainMacroExecutor, DomainSearchExecutor, DomainBranchExecutor, DomainSessionExecutor, DomainToolExecutor, DomainStatsExecutor, DomainThemeExecutor, DomainDebugExecutor, DomainHistoryExecutor, DomainSaveExecutor, DomainLoadExecutor, DomainImportExecutor, DomainSettingsExecutor, DomainCustomExecutor, DomainCopyExecutor, DomainRetryExecutor, DomainUndoExecutor, DomainChatExecutor

## IMPLEMENTATION CHECKLIST

### execution.rs
- [ ] Update imports (remove ystream, ensure Pin and Stream imports)
- [ ] Keep crossbeam_utils::CachePadded import
- [ ] Convert execute_streaming() return type
- [ ] Convert execute_streaming() body (AsyncStream → spawn_stream, thread::spawn → tokio::spawn, emit! → send)
- [ ] Convert execute_config_streaming() return type and body
- [ ] Convert execute_search_streaming() return type and body
- [ ] Convert parse_and_execute() return type and body
- [ ] Replace ALL ystream::emit! calls with sender.send()

### types/mod.rs
- [ ] Add Pin and Stream imports
- [ ] Update DomainCommandExecutor trait execute() return type
- [ ] Update DomainCommandExecutorEnum execute() return type
- [ ] Convert DomainHelpExecutor::execute()
- [ ] Convert DomainClearExecutor::execute()
- [ ] Convert DomainExportExecutor::execute()
- [ ] Convert DomainConfigExecutor::execute()
- [ ] Convert DomainTemplateExecutor::execute()
- [ ] Convert DomainMacroExecutor::execute()
- [ ] Convert DomainSearchExecutor::execute()
- [ ] Convert DomainBranchExecutor::execute()
- [ ] Convert DomainSessionExecutor::execute()
- [ ] Convert DomainToolExecutor::execute()
- [ ] Convert DomainStatsExecutor::execute()
- [ ] Convert DomainThemeExecutor::execute()
- [ ] Convert DomainDebugExecutor::execute()
- [ ] Convert DomainHistoryExecutor::execute()
- [ ] Convert DomainSaveExecutor::execute()
- [ ] Convert DomainLoadExecutor::execute()
- [ ] Convert DomainImportExecutor::execute()
- [ ] Convert DomainSettingsExecutor::execute()
- [ ] Convert DomainCustomExecutor::execute()
- [ ] Convert DomainCopyExecutor::execute()
- [ ] Convert DomainRetryExecutor::execute()
- [ ] Convert DomainUndoExecutor::execute()
- [ ] Convert DomainChatExecutor::execute()

### Global Verification
- [ ] Run `cargo check` to verify compilation
- [ ] Verify no AsyncStream references remain
- [ ] Verify no ystream imports remain
- [ ] Confirm all ystream::emit! macros replaced

## IMPORTANT NOTES

### What NOT to Change

1. **crossbeam_utils in execution.rs:**
   ```rust
   use crossbeam_utils::CachePadded;  // ← KEEP THIS
   ```
   We only remove `crossbeam_queue`, not `crossbeam_utils`.

2. **Synchronous logic:**
   All the command execution logic (validation, event creation) remains synchronous. We just wrap it in `async move` for the spawn_stream closure.

3. **std::thread::sleep in execution.rs:**
   ```rust
   std::thread::sleep(std::time::Duration::from_millis(250));  // ← Can keep for now
   ```
   While this should ideally be `tokio::time::sleep`, it's inside a `tokio::spawn` so it won't block the main executor. Can be optimized later.

### Thread Spawn → Tokio Spawn

**Pattern in execution.rs:**
```rust
// BEFORE
AsyncStream::with_channel(move |sender| {
    std::thread::spawn(move || {
        // logic here
    });
})

// AFTER  
Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
    tokio::spawn(async move {
        // same logic here
    });
}))
```

The outer `spawn_stream` already uses `tokio::spawn`, but the inner spawn also needs conversion from `std::thread::spawn` to `tokio::spawn(async move { ... })`.

### ystream::emit! Macro Conversion

The `ystream::emit!` macro is shorthand for `let _ = sender.send(value);`

**Global replacement pattern:**
- Find: `ystream::emit!(sender, event)`
- Replace: `let _ = sender.send(event)`

## DEFINITION OF DONE

- ✅ All AsyncStream usage removed from execution.rs (4 occurrences)
- ✅ All AsyncStream usage removed from types/mod.rs (~24 occurrences)
- ✅ mod.rs verified (already has no AsyncStream)
- ✅ All ystream::emit! macros replaced with sender.send()
- ✅ All ystream imports removed
- ✅ All trait return types updated to Pin<Box<dyn Stream>>
- ✅ All std::thread::spawn converted to tokio::spawn
- ✅ Project compiles with `cargo check` (0 errors)
- ✅ No stubs or placeholder code introduced
- ✅ crossbeam_utils::CachePadded remains in execution.rs

## VERIFICATION COMMANDS

```bash
# Verify no AsyncStream references remain
rg "AsyncStream" packages/candle/src/domain/chat/commands/execution.rs
rg "AsyncStream" packages/candle/src/domain/chat/commands/types/mod.rs

# Verify no ystream imports or macros remain
rg "use ystream" packages/candle/src/domain/chat/commands/
rg "ystream::emit!" packages/candle/src/domain/chat/commands/

# Verify compilation
cd packages/candle && cargo check

# Verify proper patterns are present
rg "Box::pin\(crate::async_stream::spawn_stream" packages/candle/src/domain/chat/commands/execution.rs
rg "Pin<Box<dyn Stream<Item = CommandEvent> \+ Send>>" packages/candle/src/domain/chat/commands/execution.rs
rg "tokio::spawn\(async move" packages/candle/src/domain/chat/commands/execution.rs

# Check that crossbeam_utils is still present (should find it)
rg "crossbeam_utils::CachePadded" packages/candle/src/domain/chat/commands/execution.rs

# Count conversions (should be 0 when done)
rg "AsyncStream" packages/candle/src/domain/chat/commands/ | wc -l
```

## REFERENCES

- **async_stream helper:** [`packages/candle/src/async_stream.rs`](../packages/candle/src/async_stream.rs)
- **Previous conversion examples:**
  - YSTREAM_M: [`packages/candle/src/domain/chat/config.rs`](../packages/candle/src/domain/chat/config.rs)
  - YSTREAM_N: [`packages/candle/src/cli/runner.rs`](../packages/candle/src/cli/runner.rs)
  - YSTREAM_O: [`packages/candle/src/builders/`](../packages/candle/src/builders/)

## TASK SCOPE SUMMARY

**Converting 2 command files (~28 total AsyncStream occurrences):**
- execution.rs: 4 methods with AsyncStream + ystream::emit! + std::thread::spawn
- mod.rs: 0 occurrences (already clean)
- types/mod.rs: 1 trait signature + 1 enum method + ~23 executor implementations

**Key patterns:**
1. Thread spawn pattern (execution.rs) - requires tokio::spawn conversion
2. Simple sync channel pattern (types/mod.rs) - straightforward conversion
3. Trait signature updates (types/mod.rs) - return type changes

**Complexity:** Medium-high due to large number of executor implementations, but each follows same simple pattern.
