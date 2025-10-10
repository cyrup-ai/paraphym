# `packages/candle/src/domain/chat/orchestration.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: candle
- **File Hash**: 5c5514a5  
- **Timestamp**: 2025-10-10T02:15:58.158978+00:00  
- **Lines of Code**: 169

---## Tests in Source Directory


### Line 166: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/chat/orchestration.rs` (line 166)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
pub fn get_selected_tool_schemas(
    selected_names: &[String],
    available_tools: &[ToolInfo]
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 190: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/chat/orchestration.rs` (line 190)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use super::*;
    
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 194: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/chat/orchestration.rs` (line 194)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    
    #[test]
    fn test_format_tools_for_selection() -> Result<(), Box<dyn std::error::Error>> {
        // Convert serde_json to simd_json by serializing and deserializing
        let schema_json = serde_json::to_string(&json!({"type": "object"}))?;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 220: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/chat/orchestration.rs` (line 220)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    
    #[test]
    fn test_get_selected_tool_schemas() -> Result<(), Box<dyn std::error::Error>> {
        // Convert serde_json to simd_json
        let schema_json = serde_json::to_string(&json!({}))?;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `collect_stream_to_string()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/chat/orchestration.rs` (line 179)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Helper to collect `AsyncStream` into String
#[must_use]
pub fn collect_stream_to_string(
    stream: &ystream::AsyncStream<crate::domain::context::chunk::CandleStringChunk>
) -> String {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `parse_tool_selection_response()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/chat/orchestration.rs` (line 140)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// Returns error if JSON parsing fails
pub fn parse_tool_selection_response(json_str: &str) -> Result<ToolSelectionResponse> {
    serde_json::from_str(json_str)
        .context("Failed to parse tool selection response")
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `render_stage4_prompt()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/chat/orchestration.rs` (line 115)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// Returns error if template rendering or JSON serialization fails
pub fn render_stage4_prompt(
    user_message: &str,
    tool_calls: &[super::types::responses::ToolCall],
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `parse_final_response()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/chat/orchestration.rs` (line 160)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// Returns error if JSON parsing fails
pub fn parse_final_response(json_str: &str) -> Result<FinalResponse> {
    serde_json::from_str(json_str)
        .context("Failed to parse final response")
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `render_stage2_prompt()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/chat/orchestration.rs` (line 101)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// Returns error if template rendering or tool formatting fails
pub fn render_stage2_prompt(user_input: &str, selected_tools: &[ToolInfo]) -> Result<String> {
    let mut variables = HashMap::new();
    variables.insert("user_input".to_string(), user_input.to_string());
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `render_stage1_prompt()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/chat/orchestration.rs` (line 87)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// Returns error if template rendering fails
pub fn render_stage1_prompt(user_input: &str, available_tools: &[ToolInfo]) -> Result<String> {
    let mut variables = HashMap::new();
    variables.insert("user_input".to_string(), user_input.to_string());
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `parse_function_call_response()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/chat/orchestration.rs` (line 150)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// Returns error if JSON parsing fails
pub fn parse_function_call_response(json_str: &str) -> Result<OpenAIFunctionCallResponse> {
    serde_json::from_str(json_str)
        .context("Failed to parse function call response")
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym