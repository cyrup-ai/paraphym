# `packages/candle/src/util/json_util.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: candle
- **File Hash**: cbe21222  
- **Timestamp**: 2025-10-10T02:15:58.149325+00:00  
- **Lines of Code**: 272

---## Tests in Source Directory


### Line 369: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/json_util.rs` (line 369)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
// ============================================================================
#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 381: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/json_util.rs` (line 381)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    // ----- merge -----------------------------------------------------------
    #[test]
    fn merge_by_value() {
        let a = serde_json::json!({"k1":"v1"});
        let b = serde_json::json!({"k2":"v2"});
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 388: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/json_util.rs` (line 388)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn merge_in_place() {
        let mut a = serde_json::json!({"k1":"v1"});
        merge_inplace(&mut a, serde_json::json!({"k2":"v2"}));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 396: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/json_util.rs` (line 396)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    // ----- stringified JSON -----------------------------------------------
    #[test]
    fn stringified_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let original = Dummy {
            data: serde_json::json!({"k":"v"})};
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 408: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/json_util.rs` (line 408)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    // ----- string_or_vec ---------------------------------------------------
    #[test]
    fn str_or_array_deserialise() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Wrapper {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 427: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/json_util.rs` (line 427)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    // ----- null_or_vec -----------------------------------------------------
    #[test]
    fn null_or_array_deserialise() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Wrapper {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 443: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/json_util.rs` (line 443)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    // ----- utility functions -----------------------------------------------
    #[test]
    fn test_ensure_object_and_merge() {
        let mut target = serde_json::json!("not an object");
        let source = serde_json::json!({"key": "value"});
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 452: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/json_util.rs` (line 452)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_is_empty_value() {
        assert!(is_empty_value(&serde_json::json!(null)));
        assert!(is_empty_value(&serde_json::json!({})));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 463: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/json_util.rs` (line 463)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_merge_multiple() {
        let values = vec![
            serde_json::json!({"a": 1}),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `null_or_vec()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/json_util.rs` (line 231)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// ```
#[inline]
pub fn null_or_vec<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    T: Deserialize<'de>,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `insert_or_create()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/json_util.rs` (line 314)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Performance: Zero allocation if target is already an object
#[inline]
pub fn insert_or_create(target: &mut serde_json::Value, key: String, value: serde_json::Value) {
    if let Some(map) = ensure_object_map(target) {
        map.insert(key, value);
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `is_empty_value()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/json_util.rs` (line 342)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Performance: Inlined, no allocations
#[inline]
pub fn is_empty_value(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Null => true,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `to_pretty_string()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/json_util.rs` (line 361)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Performance: Single allocation for output string, optimized formatting
#[inline]
pub fn to_pretty_string(value: &serde_json::Value) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(value)
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `to_compact_string()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/json_util.rs` (line 354)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Performance: Single allocation for output string
#[inline]
pub fn to_compact_string(value: &serde_json::Value) -> String {
    value.to_string() // serde_json uses compact format by default
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `string_or_vec()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/json_util.rs` (line 145)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// ```
#[inline]
pub fn string_or_vec<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    T: Deserialize<'de> + FromStr,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym