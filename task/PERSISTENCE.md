# Task: Automatic Cognitive State Persistence

**Status**: Ready for Execution
**Priority**: Medium
**Complexity**: Low

## Overview

Restore automatic cognitive state persistence that was disabled. The `CognitiveState` already has Serialize/Deserialize derives (state.rs:43-44). This task adds automatic save/load using the same path pattern as the database (memory_ops.rs:10-13) - NO environment variables, everything "just works".

## Objective

Enable transparent state persistence by:
1. Auto-load state from cache directory on startup
2. Auto-save state to cache directory on shutdown
3. Use `dirs::cache_dir()` with automatic fallback (same as database)
4. NO environment variables
5. NO user configuration
6. Persistence happens automatically in lifecycle methods

## Key Principle: FOLLOW DATABASE PATH PATTERN

From memory/vector/memory_ops.rs lines 10-13:
```rust
let db_path = dirs::cache_dir()
    .unwrap_or_else(|| std::path::PathBuf::from("."))
    .join("cyrup")
    .join("agent.db");
```

This shows the pattern:
- Use `dirs::cache_dir()` for platform-specific cache location
- Automatic fallback to current directory if unavailable
- Join "cyrup" subdirectory
- Join specific filename
- NO environment variables

We'll follow this EXACT pattern for cognitive state persistence.

## Background: What's Already Built

From domain/memory/cognitive/types/state.rs lines 43-44:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveState {
```

CognitiveState ALREADY has:
- Serialize/Deserialize derives
- All nested types serializable (QuantumSignature, AlignedActivationPattern, AttentionWeights)
- Ready for JSON persistence

All we need to do is:
1. Determine automatic path using dirs::cache_dir()
2. Load on startup in lifecycle.rs
3. Save on shutdown in lifecycle.rs

## Technical Details

### File: packages/candle/src/memory/core/manager/coordinator/lifecycle.rs

**Location 1: Add automatic path helper (after line 19)**

```rust
/// Get default cognitive state persistence path
///
/// Uses platform-specific cache directory with automatic fallback:
/// - Linux: `~/.cache/cyrup/cognitive-state.json`
/// - macOS: `~/Library/Caches/cyrup/cognitive-state.json`
/// - Windows: `%LOCALAPPDATA%\cyrup\cognitive-state.json`
/// - Fallback: `./cyrup/cognitive-state.json`
fn default_cognitive_state_path() -> std::path::PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("cyrup")
        .join("cognitive-state.json")
}
```

**Location 2: Auto-load state on startup (replace line 92)**

Current code (line 92):
```rust
cognitive_state: Arc::new(RwLock::new(CognitiveState::new())),
```

Replace with:
```rust
cognitive_state: Arc::new(RwLock::new({
    let path = default_cognitive_state_path();
    match std::fs::read_to_string(&path) {
        Ok(json) => match serde_json::from_str::<CognitiveState>(&json) {
            Ok(state) => {
                log::info!("Loaded cognitive state from {:?}", path);
                state
            }
            Err(e) => {
                log::warn!("Failed to parse cognitive state from {:?}: {}", path, e);
                CognitiveState::new()
            }
        },
        Err(_) => {
            log::debug!("No existing cognitive state at {:?}, using fresh state", path);
            CognitiveState::new()
        }
    }
})),
```

**Location 3: Auto-save state on shutdown (replace lines 132-142)**

Current code (lines 132-142):
```rust
/// Shutdown all cognitive worker tasks gracefully
pub fn shutdown_workers(&mut self) {
    // Flush any pending batches before shutdown
    if let Err(e) = self.cognitive_queue.flush_batches() {
        log::warn!("Failed to flush batches during shutdown: {}", e);
    }

    // Note: Tokio tasks will be cancelled when runtime shuts down
    // We don't await them here since this method is sync
    // The queue channel will be dropped, causing workers to exit their loops
    log::info!("Cognitive workers will shut down when queue is closed");
}
```

Replace with:
```rust
/// Shutdown all cognitive worker tasks gracefully
pub fn shutdown_workers(&mut self) {
    // Flush any pending batches before shutdown
    if let Err(e) = self.cognitive_queue.flush_batches() {
        log::warn!("Failed to flush batches during shutdown: {}", e);
    }

    // Auto-save cognitive state before shutdown
    let path = default_cognitive_state_path();
    
    // Ensure directory exists
    if let Some(parent) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            log::warn!("Failed to create cognitive state directory: {}", e);
        }
    }

    // Save state (sync version - block_in_place for async read)
    let save_result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            let state = self.cognitive_state.read().await;
            match serde_json::to_string_pretty(&*state) {
                Ok(json) => std::fs::write(&path, json)
                    .map_err(|e| format!("Failed to write: {}", e)),
                Err(e) => Err(format!("Failed to serialize: {}", e)),
            }
        })
    });

    match save_result {
        Ok(()) => log::info!("Saved cognitive state to {:?}", path),
        Err(e) => log::warn!("Failed to save cognitive state: {}", e),
    }

    // Note: Tokio tasks will be cancelled when runtime shuts down
    // We don't await them here since this method is sync
    // The queue channel will be dropped, causing workers to exit their loops
    log::info!("Cognitive workers will shut down when queue is closed");
}
```

## Architecture Flow

```
Application Startup
        │
        ▼
MemoryCoordinator::new()
        │
        ├──> default_cognitive_state_path()
        │         │
        │         └──> dirs::cache_dir().join("cyrup/cognitive-state.json")
        │
        ├──> Try read from path
        │         │
        │         ├──> File exists? Parse JSON → CognitiveState
        │         │
        │         └──> File missing? Use CognitiveState::new()
        │
        └──> Cognitive state ready (loaded or fresh)

Application Shutdown
        │
        ▼
coordinator.shutdown_workers()
        │
        ├──> Flush pending batches
        │
        ├──> default_cognitive_state_path()
        │
        ├──> Create directory if needed
        │
        ├──> Read cognitive_state (async)
        │
        ├──> Serialize to JSON
        │
        └──> Write to path
```

## Implementation Checklist

### Phase 1: Add Path Helper
- [ ] Add default_cognitive_state_path() function after line 19
- [ ] Use dirs::cache_dir() with fallback pattern
- [ ] Join "cyrup" and "cognitive-state.json"

### Phase 2: Auto-Load on Startup
- [ ] Find CognitiveState initialization at line 92
- [ ] Replace with auto-load logic
- [ ] Try read from default path
- [ ] Parse JSON if file exists
- [ ] Fall back to CognitiveState::new() if missing/invalid
- [ ] Add appropriate log messages

### Phase 3: Auto-Save on Shutdown
- [ ] Find shutdown_workers() method (lines 132-142)
- [ ] Add cognitive state save logic BEFORE worker shutdown
- [ ] Create directory if needed (parent path)
- [ ] Use block_in_place for async read
- [ ] Serialize state to pretty JSON
- [ ] Write to default path
- [ ] Add error logging

### Phase 4: Verification
- [ ] Run `cargo check -p paraphym_candle --color=never`
- [ ] Verify no new fields added
- [ ] Verify no public API changes
- [ ] Test startup with missing state file (creates fresh)
- [ ] Test startup with existing state file (loads)
- [ ] Test shutdown (saves state)

## Success Criteria

✅ State auto-loads from cache directory on startup
✅ State auto-saves to cache directory on shutdown
✅ Uses platform-specific cache location (dirs::cache_dir())
✅ Automatic fallback to current directory
✅ NO environment variables required
✅ NO user configuration needed
✅ Directory created automatically if missing
✅ Graceful degradation (fresh state if load fails)
✅ Same path pattern as database (memory_ops.rs:10-13)

## Non-Goals (Out of Scope)

❌ Adding tests or benchmarks
❌ Writing documentation
❌ Periodic auto-save during runtime
❌ Multiple save locations
❌ State versioning or migration
❌ Compression or encryption
❌ Configuration of save path

## Notes

- Path follows EXACT pattern from memory_ops.rs:10-13
- Platform-specific cache locations:
  - Linux: `~/.cache/cyrup/cognitive-state.json`
  - macOS: `~/Library/Caches/cyrup/cognitive-state.json`
  - Windows: `%LOCALAPPDATA%\cyrup\cognitive-state.json`
- Fallback: `./cyrup/cognitive-state.json` if cache_dir unavailable
- State saved as pretty-printed JSON for human readability
- Load errors are non-fatal (fall back to fresh state)
- Save errors are logged but don't block shutdown
- This enables cognitive state to persist across application restarts
- Stats counters (AtomicU64) are persisted as their current values
- The dirs crate is already a dependency (used in memory_ops.rs)
