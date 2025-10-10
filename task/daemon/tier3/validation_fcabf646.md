# `packages/sweetmcp/packages/daemon/src/service/sse/bridge/validation.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: daemon
- **File Hash**: fcabf646  
- **Timestamp**: 2025-10-10T02:15:59.690148+00:00  
- **Lines of Code**: 330

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 330 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 3 Evaluations


- Line 302
  - fallback
  - 

```rust

    // If parsing fails, try to extract ID with regex
    // This is a fallback for malformed JSON
    if let Some(captures) = regex::Regex::new(r#""id"\s*:\s*([^,}]+)"#)
        .ok()?
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 280: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        error
            .as_object_mut()
            .unwrap()
            .insert("data".to_string(), data_value);
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Orphaned Methods


### `extract_request_id()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/daemon/src/service/sse/bridge/validation.rs` (line 293)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Extract request ID from potentially malformed JSON (part of complete JSON-RPC validation API)
#[allow(dead_code)]
pub fn extract_request_id(request_text: &str) -> Option<Value> {
    // Try to parse as JSON first
    if let Ok(json) = serde_json::from_str::<Value>(request_text) {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `create_internal_error_response()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/daemon/src/service/sse/bridge/validation.rs` (line 252)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Create internal error response (part of complete JSON-RPC validation API)
#[allow(dead_code)]
pub fn create_internal_error_response(id: Option<Value>, details: Option<&str>) -> Value {
    serde_json::json!({
        "jsonrpc": "2.0",
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `validate_security()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/daemon/src/service/sse/bridge/validation.rs` (line 396)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Check for common JSON-RPC security issues
#[allow(dead_code)] // Part of comprehensive validation API
pub fn validate_security(request: &Value) -> Result<(), anyhow::Error> {
    if let Some(obj) = request.as_object() {
        // Check for suspicious method names
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `validate_json_rpc_response()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/daemon/src/service/sse/bridge/validation.rs` (line 73)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Validate JSON-RPC response structure (part of complete JSON-RPC validation API)
#[allow(dead_code)]
pub fn validate_json_rpc_response(response: &Value) -> Result<(), anyhow::Error> {
    if !response.is_object() {
        return Err(anyhow::anyhow!("Response must be a JSON object"));
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `sanitize_error_message()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/daemon/src/service/sse/bridge/validation.rs` (line 333)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Sanitize error message to prevent information leakage (part of complete JSON-RPC validation API)
#[allow(dead_code)]
pub fn sanitize_error_message(message: &str) -> String {
    // Remove potentially sensitive information
    let sanitized = message
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `validate_batch_requests()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/daemon/src/service/sse/bridge/validation.rs` (line 350)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Batch validate multiple JSON-RPC requests
#[allow(dead_code)] // Part of comprehensive validation API
pub fn validate_batch_requests(requests: &[Value]) -> Vec<Result<(), anyhow::Error>> {
    if requests.is_empty() {
        return vec![Err(anyhow::anyhow!("Batch cannot be empty"))];
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `create_invalid_params_response()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/daemon/src/service/sse/bridge/validation.rs` (line 238)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Create invalid params error response (part of complete JSON-RPC validation API)
#[allow(dead_code)]
pub fn create_invalid_params_response(id: Option<Value>, details: &str) -> Value {
    serde_json::json!({
        "jsonrpc": "2.0",
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `get_error_code_name()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/daemon/src/service/sse/bridge/validation.rs` (line 366)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Get error code name for debugging (part of complete JSON-RPC validation API)
#[allow(dead_code)]
pub fn get_error_code_name(code: i64) -> &'static str {
    match code {
        -32700 => "Parse error",
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `create_method_not_found_response()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/daemon/src/service/sse/bridge/validation.rs` (line 224)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Create method not found error response (part of complete JSON-RPC validation API)
#[allow(dead_code)]
pub fn create_method_not_found_response(id: Option<Value>, method: &str) -> Value {
    serde_json::json!({
        "jsonrpc": "2.0",
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `create_parse_error_response()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/daemon/src/service/sse/bridge/validation.rs` (line 197)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Create standardized error responses (part of complete JSON-RPC validation API)
#[allow(dead_code)]
pub fn create_parse_error_response(id: Option<Value>) -> Value {
    serde_json::json!({
        "jsonrpc": "2.0",
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `create_server_error_response()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/daemon/src/service/sse/bridge/validation.rs` (line 266)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Create server error response (part of complete JSON-RPC validation API)
#[allow(dead_code)]
pub fn create_server_error_response(
    id: Option<Value>,
    code: i64,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `validate_request_size()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/daemon/src/service/sse/bridge/validation.rs` (line 380)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Validate request size limits
#[allow(dead_code)] // Part of comprehensive validation API
pub fn validate_request_size(request_text: &str) -> Result<(), anyhow::Error> {
    const MAX_REQUEST_SIZE: usize = 1024 * 1024; // 1MB

```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym