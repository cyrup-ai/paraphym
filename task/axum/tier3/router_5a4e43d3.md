# `packages/sweetmcp/packages/axum/src/router.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: axum
- **File Hash**: 5a4e43d3  
- **Timestamp**: 2025-10-10T02:15:59.625658+00:00  
- **Lines of Code**: 526

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 526 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 3 Evaluations


- Line 13
  - actual
  - 

```rust
};

// Only import what's actually used
use crate::resource::{resource_read, resource_subscribe_handler, resource_unsubscribe_handler};
use crate::{
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Orphaned Methods


### `roots_list()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/router.rs` (line 528)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Handler for roots/list method
pub async fn roots_list(_request: Option<ListRootsRequest>) -> HandlerResult<ListRootsResult> {
    debug!("Listing available roots");
    let response = ListRootsResult {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `create_socket_listener()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/router.rs` (line 547)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Create and run Unix domain socket listener
pub async fn create_socket_listener(
    plugin_manager: PluginManager,
    socket_path: &std::path::Path,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `logging_set_level()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/router.rs` (line 521)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Handler for logging/setLevel method
pub async fn logging_set_level(request: SetLevelRequest) -> HandlerResult<LoggingResponse> {
    info!("Setting log level to: {}", request.level);
    // Implementation for changing log level would go here
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `ping()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/router.rs` (line 515)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Handler for the ping method
pub async fn ping(_request: PingRequest) -> HandlerResult<EmptyResult> {
    debug!("Received ping request");
    Ok(EmptyResult {})
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `run_http_server()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/router.rs` (line 251)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Run the server using HTTP binding
pub async fn run_http_server(plugin_manager: PluginManager, bind_addr: &str) -> Result<()> {
    info!("Starting MCP JSON-RPC server (HTTP mode on {})", bind_addr);

```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym