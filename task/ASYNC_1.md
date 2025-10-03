# ASYNC_1: Remove Runtime Blocking in Tool Execution

## OBJECTIVE
Refactor tool execution from synchronous blocking to proper async/await pattern to eliminate thread blocking and improve concurrency. The blocking was previously approved temporarily but needs to be removed now.

## CORE PROBLEM

The `execute_native_tool()` method in `unified.rs` uses `runtime.block_on()` to bridge async code, which blocks the current thread unnecessarily. This is a **false bridge** - the underlying `SweetMcpRouter::call_tool()` is already async, so the blocking is completely avoidable.

### Why The Blocking Exists

Current code at [unified.rs:286-310](../packages/candle/src/domain/tool/unified.rs):
- Line 293: `runtime.block_on(self.native_router.read())` - blocks to read async RwLock
- Line 299-301: `runtime.block_on(router.call_tool(...))` - blocks on async router call

The `SweetMcpRouter::call_tool()` signature ([router.rs:136](../packages/candle/src/domain/tool/router.rs)):
```rust
pub async fn call_tool(&self, tool_name: &str, args: JsonValue) -> Result<Value, RouterError>
```

Since the router's method is already async, we can simply await it instead of blocking.

## LOCATION
`packages/candle/src/domain/tool/unified.rs`

## IMPLEMENTATION PLAN

### STEP 1: Convert execute_native_tool to async

**Current (Line 286):**
```rust
fn execute_native_tool(&self, tool_info: &ToolInfo, args: JsonValue) -> Result<Response, ToolError> {
```

**Change to:**
```rust
async fn execute_native_tool(&self, tool_info: &ToolInfo, args: JsonValue) -> Result<Response, ToolError> {
```

### STEP 2: Replace blocking RwLock read with await

**Current (Line 293):**
```rust
let router_guard = runtime.block_on(self.native_router.read());
```

**Change to:**
```rust
let router_guard = self.native_router.read().await;
```

### STEP 3: Replace blocking router call with await

**Current (Lines 289-301):**
```rust
// Get router through RwLock
let runtime = crate::runtime::shared_runtime()
    .ok_or_else(|| ToolError::Other(anyhow::anyhow!("Runtime unavailable")))?;

let router_guard = runtime.block_on(self.native_router.read());

let router = router_guard.as_ref()
    .ok_or_else(|| ToolError::Other(anyhow::anyhow!("Native router not initialized")))?;

// BLOCKING CODE APPROVED BY DAVID ON 2025-01-29: Using shared_runtime().block_on() for router call
let result = runtime
    .block_on(router.call_tool(&tool_info.name, args))
    .map_err(|e| ToolError::CyloError(e.to_string()))?;
```

**Change to:**
```rust
// Get router through RwLock
let router_guard = self.native_router.read().await;

let router = router_guard.as_ref()
    .ok_or_else(|| ToolError::Other(anyhow::anyhow!("Native router not initialized")))?;

// Now properly async - no blocking
let result = router
    .call_tool(&tool_info.name, args)
    .await
    .map_err(|e| ToolError::CyloError(e.to_string()))?;
```

### STEP 4: Update caller to await the async function

**Current (Line 211 in unified.rs - inside async fn call_tool):**
```rust
if Self::is_mcp_tool(tool_info) {
    self.execute_mcp_tool(tool_name, args).await
} else {
    self.execute_native_tool(tool_info, args)  // <- Missing await
}
```

**Change to:**
```rust
if Self::is_mcp_tool(tool_info) {
    self.execute_mcp_tool(tool_name, args).await
} else {
    self.execute_native_tool(tool_info, args).await  // <- Add await
}
```

## CALL CHAIN ANALYSIS

The async conversion is safe because:

1. **execute_native_tool** is only called from one location:
   - Line 211 in `call_tool()` method (which is already async)

2. **call_tool** is async:
   - Line 199: `pub async fn call_tool(&self, tool_name: &str, args: JsonValue) -> Result<Response, ToolError>`
   - Can naturally await execute_native_tool

3. **SweetMcpRouter::call_tool** is already async:
   - Defined at [router.rs:136](../packages/candle/src/domain/tool/router.rs)
   - Returns a Future that can be awaited

## COMPLETE DIFF

### Before (Lines 286-310):
```rust
fn execute_native_tool(&self, tool_info: &ToolInfo, args: JsonValue) -> Result<Response, ToolError> {
    use crate::domain::agent::role::convert_serde_to_sweet_json;
    
    // Get router through RwLock
    let runtime = crate::runtime::shared_runtime()
        .ok_or_else(|| ToolError::Other(anyhow::anyhow!("Runtime unavailable")))?;
    
    let router_guard = runtime.block_on(self.native_router.read());
    
    let router = router_guard.as_ref()
        .ok_or_else(|| ToolError::Other(anyhow::anyhow!("Native router not initialized")))?;

    // BLOCKING CODE APPROVED BY DAVID ON 2025-01-29: Using shared_runtime().block_on() for router call
    let result = runtime
        .block_on(router.call_tool(&tool_info.name, args))
        .map_err(|e| ToolError::CyloError(e.to_string()))?;

    // Convert serde_json::Value result to Response
    let response_data = convert_serde_to_sweet_json(result);
    
    let native_id = uuid::Uuid::new_v4();
    Ok(Response {
        id: sweet_mcp_type::RequestId::Str(format!("native_{native_id}")),
        result: Some(response_data),
        error: None,
    })
}
```

### After (Lines 286-307):
```rust
async fn execute_native_tool(&self, tool_info: &ToolInfo, args: JsonValue) -> Result<Response, ToolError> {
    use crate::domain::agent::role::convert_serde_to_sweet_json;
    
    // Get router through RwLock
    let router_guard = self.native_router.read().await;
    
    let router = router_guard.as_ref()
        .ok_or_else(|| ToolError::Other(anyhow::anyhow!("Native router not initialized")))?;

    // Now properly async - no blocking
    let result = router
        .call_tool(&tool_info.name, args)
        .await
        .map_err(|e| ToolError::CyloError(e.to_string()))?;

    // Convert serde_json::Value result to Response
    let response_data = convert_serde_to_sweet_json(result);
    
    let native_id = uuid::Uuid::new_v4();
    Ok(Response {
        id: sweet_mcp_type::RequestId::Str(format!("native_{native_id}")),
        result: Some(response_data),
        error: None,
    })
}
```

## FILES TO MODIFY

1. **[packages/candle/src/domain/tool/unified.rs](../packages/candle/src/domain/tool/unified.rs)**
   - Line 286: Change `fn execute_native_tool` to `async fn execute_native_tool`
   - Lines 289-291: Remove `runtime` and `shared_runtime()` call
   - Line 293: Change `runtime.block_on(self.native_router.read())` to `self.native_router.read().await`
   - Lines 298-301: Change `runtime.block_on(router.call_tool(...))` to `router.call_tool(...).await`
   - Line 211: Add `.await` to `self.execute_native_tool(tool_info, args)`

## RELATED CODE

- **[packages/candle/src/domain/tool/router.rs:136](../packages/candle/src/domain/tool/router.rs)** - Shows SweetMcpRouter::call_tool is already async
- **[packages/candle/src/domain/tool/unified.rs:199](../packages/candle/src/domain/tool/unified.rs)** - Shows UnifiedToolExecutor::call_tool is already async

## DEFINITION OF DONE

- [x] No `runtime.block_on()` calls remain in execute_native_tool
- [x] No `shared_runtime()` dependency in execute_native_tool  
- [x] `execute_native_tool()` method signature is `async fn`
- [x] All router operations use `.await` instead of blocking
- [x] Caller at line 211 properly awaits execute_native_tool
- [x] Code compiles without errors or warnings
- [x] Async chain flows naturally from call_tool → execute_native_tool → router.call_tool
