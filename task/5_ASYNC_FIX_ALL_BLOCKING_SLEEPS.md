# Task: Replace ALL Blocking std::thread::sleep with tokio::time::sleep

## Problem
**Multiple blocking sleep calls throughout codebase will block tokio worker threads**

Found 13+ locations using `std::thread::sleep()` instead of async `tokio::time::sleep().await`:

## Blocking Sleep Locations

### 1. Pool Core
**File**: `src/pool/core/pool.rs:314`
```rust
std::thread::sleep(Duration::from_millis(50));  // ❌ In wait_for_workers loop
```
**Context**: Hot path during worker spawn wait
**Priority**: 🔥 CRITICAL

### 2. Pool Shutdown  
**File**: `src/pool/shutdown.rs:60`
```rust
std::thread::sleep(Duration::from_millis(100));
```
**Context**: Shutdown polling loop
**Priority**: 🟡 MEDIUM

### 3. Document Builder
**File**: `src/builders/document.rs:726`
```rust
std::thread::sleep(std::time::Duration::from_millis(...));
```
**File**: `src/builders/document.rs:785`
```rust
std::thread::sleep(std::time::Duration::from_millis(...));
```
**Context**: Document processing loops
**Priority**: 🔴 HIGH

### 4. Generation Stats
**File**: `src/core/generation/stats.rs:214`
```rust
thread::sleep(Duration::from_millis(10));
```
**Context**: Stats collection loop
**Priority**: 🟡 MEDIUM

### 5. Memory Schema
**File**: `src/memory/schema/relationship_schema.rs:269`
```rust
std::thread::sleep(std::time::Duration::from_millis(10));
```
**Context**: Schema operations
**Priority**: 🟡 MEDIUM

### 6. Memory Cache
**File**: `src/domain/memory/cache.rs:60`
```rust
std::thread::sleep(std::time::Duration::from_millis(100));
```
**Context**: Cache polling
**Priority**: 🟡 MEDIUM

### 7. Realtime Typing
**File**: `src/domain/chat/realtime/typing.rs:319`
```rust
std::thread::sleep(cleanup_interval);
```
**Context**: Typing indicator cleanup loop
**Priority**: 🟡 MEDIUM

### 8. Realtime Connection
**File**: `src/domain/chat/realtime/connection.rs:368`
```rust
std::thread::sleep(Duration::from_secs(1));
```
**Context**: Connection monitoring
**Priority**: 🟡 MEDIUM

### 9. Command Execution (2 locations)
**File**: `src/domain/chat/commands/execution.rs:172`
```rust
std::thread::sleep(std::time::Duration::from_millis(250));
```
**File**: `src/domain/chat/commands/execution.rs:224`
```rust
std::thread::sleep(std::time::Duration::from_millis(150));
```
**Context**: Command execution delays
**Priority**: 🟡 MEDIUM

### 10. Chat Macros (2 locations)
**File**: `src/domain/chat/macros.rs:1057`
```rust
std::thread::sleep(duration);
```
**File**: `src/domain/chat/macros.rs:1742`
```rust
std::thread::sleep(duration);
```
**Context**: Macro execution delays
**Priority**: 🟡 MEDIUM

---

## Solution Pattern

**Before (blocks thread)**:
```rust
std::thread::sleep(Duration::from_millis(50));
```

**After (yields to scheduler)**:
```rust
tokio::time::sleep(Duration::from_millis(50)).await;
```

## Requirements

1. Function containing sleep MUST be `async fn`
2. All callers must add `.await` to the call
3. If in a loop, ensure loop is in async context
4. If in a spawned task, verify it's `tokio::spawn` not `std::thread::spawn`

## Steps

For EACH location:
1. Change `std::thread::sleep(duration)` → `tokio::time::sleep(duration).await`
2. Make containing function `async fn` (if not already)
3. Update all call sites to add `.await`
4. Verify no `std::thread::spawn` wrapping the async code
5. Test functionality

## Testing

For each fixed location:
- Verify functionality still works
- Check logs for any errors
- Monitor thread pool under load
- Confirm no runtime blocking

## Priority
🔥 **CRITICAL** for hot paths (pool, document builders)
🟡 **MEDIUM** for background tasks (monitoring, cleanup)

## Status
⏳ TODO - 13 locations to fix (all blocking sleeps must be converted)
