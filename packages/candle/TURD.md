# Technical Debt & Unfinished Requirements Document (TURD)

## Production-Blocking Issues

### 1. DEAD CODE: Extractor Builder References Non-Existent Types
**File**: `src/builders/extractor.rs`  
**Lines**: 1-191 (ENTIRE FILE)  
**Severity**: CRITICAL - DELETE FILE  
**Status**: ORPHANED DEAD CODE

**Issue**:
The entire `extractor.rs` file is **dead code that doesn't compile**:

```rust
// Line 7: Imports non-existent type
use crate::domain::CandleModels as Models;

// Line 165: References non-existent variant
let agent = Agent::new(Models::Gpt35Turbo, "");
```

**Evidence of Dead Code**:
1. `CandleModels` enum does not exist in the codebase (0 results)
2. `Gpt35Turbo` variant does not exist in the codebase (0 results)
3. `extractor` module is **NOT exported** in `src/builders/mod.rs`:
   ```rust
   pub mod agent_role;
   pub mod completion;
   pub mod document;
   pub mod image;
   // ❌ NO pub mod extractor;
   ```
4. Code never compiles, never runs, never tested

**Architecture Violation**:
References OpenAI's GPT-3.5-Turbo in a **local-only Candle library** that uses capability traits, not enum-based model selection.

**Resolution**:
```bash
rm -f src/builders/extractor.rs
```

**DELETE THE ENTIRE FILE**. If extractor functionality is needed in the future, implement it properly using:
- Capability traits (`TextToTextCapable`)
- Registry-based model selection
- No hardcoded model references
- No non-existent type imports

---

### 2. Stubbed Temporal Context Maintenance
**File**: `src/memory/core/cognitive_worker.rs`  
**Lines**: 980-1005  
**Severity**: HIGH  
**Status**: PLACEHOLDER STUB

**Issue**:
The `maintain_temporal_context()` method is completely stubbed out with a placeholder implementation that only logs a debug message. The actual temporal decay logic is commented out and non-functional.

```rust
async fn maintain_temporal_context(&self) -> Result<(), String> {
    // ARCHITECTURE NOTE: This is a placeholder until temporal_context has RwLock wrapper
    log::debug!(
        "Temporal decay maintenance placeholder - awaiting RwLock wrapper on temporal_context"
    );
    Ok(())
}
```

**Resolution**:
1. Add `RwLock` wrapper to `temporal_context` field in `CognitiveState` struct
2. Implement the commented-out logic:
```rust
async fn maintain_temporal_context(&self) -> Result<(), String> {
    let cognitive_mem = self.cognitive_memory.read().await;
    let state = cognitive_mem.state();
    
    // Acquire write lock on temporal context
    let mut temporal_ctx = state.temporal_context.write().await;
    
    // Apply exponential decay to history embedding
    temporal_ctx.slide_window();
    
    log::debug!(
        "Applied temporal decay: window_start={:?}, decay_rate={}, history_dim={}",
        temporal_ctx.window_start,
        temporal_ctx.temporal_decay,
        temporal_ctx.history_embedding.len()
    );
    
    Ok(())
}
```
3. Integrate into periodic maintenance system
4. Add metrics for decay effectiveness

---

### 3. Placeholder Tool Completion Implementation
**File**: `src/memory/api/sdk.rs`  
**Lines**: 254-266  
**Severity**: MEDIUM  
**Status**: INCOMPLETE IMPLEMENTATION

**Issue**:
```rust
/// Generate completion with tools (placeholder implementation)
pub async fn generate_completion_with_tools(
    &self,
    messages: Vec<std::collections::HashMap<String, String>>,
    _tools: Vec<std::collections::HashMap<String, String>>,  // Unused!
) -> Result<std::collections::HashMap<String, String>> {
    let completion = self.generate_completion(messages).await?;
    
    let mut result = std::collections::HashMap::new();
    result.insert("content".to_string(), completion);
    result.insert("tool_calls".to_string(), "[]".to_string()); // Placeholder
    
    Ok(result)
}
```

The method claims to support tool calling but:
1. Ignores the `_tools` parameter entirely
2. Always returns empty tool_calls array
3. Doesn't actually invoke any tool functionality

**Resolution**:
1. Remove underscore prefix from `tools` parameter to indicate it's used
2. Implement actual tool calling:
```rust
pub async fn generate_completion_with_tools(
    &self,
    messages: Vec<std::collections::HashMap<String, String>>,
    tools: Vec<std::collections::HashMap<String, String>>,
) -> Result<std::collections::HashMap<String, String>> {
    // Convert tools to proper format
    let tool_definitions = tools.iter()
        .map(|t| self.convert_tool_definition(t))
        .collect::<Result<Vec<_>>>()?;
    
    // Generate completion with tool awareness
    let response = self.agent.generate_with_tools(
        &messages,
        &tool_definitions
    ).await?;
    
    // Extract and format tool calls
    let tool_calls = response.tool_calls.iter()
        .map(|tc| serde_json::to_string(tc))
        .collect::<Result<Vec<_>, _>>()?;
    
    let mut result = std::collections::HashMap::new();
    result.insert("content".to_string(), response.content);
    result.insert("tool_calls".to_string(), format!("[{}]", tool_calls.join(",")));
    
    Ok(result)
}
```

---

### 4. Placeholder File Size in Command Execution
**File**: `src/domain/chat/commands/execution.rs`  
**Line**: 447  
**Severity**: MEDIUM  
**Status**: INCOMPLETE IMPLEMENTATION

**Issue**:
```rust
let result = CommandExecutionResult::File {
    path: output_str,
    size_bytes: 1024, // Placeholder size
    mime_type: from_ext(&format).first_or_text_plain().to_string(),
};
```

Hardcodes file size as 1024 bytes instead of getting actual file size from filesystem.

**Resolution**:
```rust
let metadata = tokio::fs::metadata(&output_str)
    .await
    .map_err(|e| format!("Failed to get file metadata: {}", e))?;

let result = CommandExecutionResult::File {
    path: output_str,
    size_bytes: metadata.len(),
    mime_type: from_ext(&format).first_or_text_plain().to_string(),
};
```

---

### 5. Missing Duration Calculation in Command Completion
**File**: `src/domain/chat/commands/execution.rs`  
**Line**: 655  
**Severity**: LOW  
**Status**: INCOMPLETE IMPLEMENTATION

**Issue**:
```rust
result: CommandExecutionResult::Success(
    "Command completed".to_string()
),
duration_us: 0, // TODO: Calculate actual duration
```

Duration is hardcoded to 0 instead of calculating elapsed time.

**Resolution**:
1. Add `start_time` capture at command start
2. Calculate actual duration:
```rust
let start_time = std::time::Instant::now();

// ... command execution ...

#[allow(clippy::cast_possible_truncation)]
let duration_us = start_time.elapsed().as_micros().min(u128::from(u64::MAX)) as u64;

result: CommandExecutionResult::Success(
    "Command completed".to_string()
),
duration_us,
```

---

### 6. Placeholder Memory Count in API Handler
**File**: `src/memory/api/handlers.rs`  
**Line**: 240  
**Severity**: LOW  
**Status**: INCOMPLETE IMPLEMENTATION

**Issue**:
```rust
// Total count placeholder (would need proper implementation to count all memories)
```

Comment indicates total memory count is not properly implemented.

**Resolution**:
Implement proper count query:
```rust
let total_count = manager
    .count_memories(&query_params)
    .await
    .map_err(|e| {
        error!("Failed to count memories: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
```

Add corresponding `count_memories()` method to memory manager with proper SQL COUNT query.

---

## Code Quality Issues (Non-Blocking)

### 7. Unwrap Without Context in Error Path
**File**: `src/util/input_resolver.rs`  
**Line**: 69  
**Severity**: LOW  
**Status**: POOR ERROR HANDLING

**Issue**:
```rust
Err(last_error.unwrap().into())
```

While logically safe (loop ensures last_error is Some), unwrap() without context message is poor style.

**Resolution**:
```rust
Err(last_error.expect("last_error should be set after retry loop").into())
```

---

## Language Corrections Needed

### 8. "Dummy" in Test Struct Names
**File**: `src/domain/util/json_util.rs`  
**Line**: 401  
**Context**: Test code

This is a **FALSE POSITIVE** - "Dummy" is an appropriate name for test data structures. No action needed.

### 9. "block_in_place" Pattern
**File**: `src/cli/runner.rs`  
**Line**: 178  
**Context**: Legitimate async bridge pattern

This is a **FALSE POSITIVE** - `tokio::task::block_in_place` is a legitimate tokio pattern for calling async code from non-async contexts. This is production-grade code from Task 026 implementation. No action needed.

### 10. "expect()" in Worker Runtime Builders
**Files**: 
- `src/capability/text_to_image/flux_schnell.rs:121`
- `src/capability/text_to_image/stable_diffusion_35_turbo/worker.rs:73`
- `src/capability/vision/llava.rs:215`

**Context**: Worker thread initialization

These are **ACCEPTABLE** - Runtime builder failures during worker thread initialization are unrecoverable errors. Using `expect()` here is appropriate as these failures indicate system-level problems that prevent the worker from functioning. The error messages clearly indicate what failed.

### 11. "expect()" in Test Code
**File**: `src/domain/context/provider.rs`  
**Lines**: Multiple (1528-1699)  
**Context**: Test helper functions

These are **FALSE POSITIVES** - expect() in test code is standard practice. Tests should panic on setup failures. No action needed.

---

## Summary

**Production-Blocking Issues**: 6  
**Code Quality Issues**: 1  
**False Positives Documented**: 4  

**Priority Order for Resolution**:
1. Fix extractor builder model configuration (HIGH)
2. Implement temporal context maintenance (HIGH)
3. Implement tool completion properly (MEDIUM)
4. Fix file size calculation (MEDIUM)
5. Add duration calculation (LOW)
6. Implement memory count (LOW)
7. Improve unwrap() error message (LOW)
