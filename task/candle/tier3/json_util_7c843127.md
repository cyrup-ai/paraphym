# `packages/candle/src/domain/util/json_util.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: candle
- **File Hash**: 7c843127  
- **Timestamp**: 2025-10-10T02:15:58.148169+00:00  
- **Lines of Code**: 282

---## Tests in Source Directory


### Line 395: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/util/json_util.rs` (line 395)
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
  


### Line 408: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/util/json_util.rs` (line 408)
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
  


### Line 415: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/util/json_util.rs` (line 415)
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
  


### Line 423: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/util/json_util.rs` (line 423)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    // ----- stringified JSON -----------------------------------------------
    #[test]
    fn stringified_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let original = Dummy {
            data: serde_json::json!({"k":"v"}),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 436: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/util/json_util.rs` (line 436)
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
  


### Line 456: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/util/json_util.rs` (line 456)
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
  


### Line 473: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/util/json_util.rs` (line 473)
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
  


### Line 482: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/util/json_util.rs` (line 482)
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
  


### Line 493: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/util/json_util.rs` (line 493)
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

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/util/json_util.rs` (line 249)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Returns error if deserialization fails
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

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/util/json_util.rs` (line 332)
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

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/util/json_util.rs` (line 362)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
#[inline]
#[must_use]
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

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/util/json_util.rs` (line 387)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Returns error if JSON serialization fails
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

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/util/json_util.rs` (line 376)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
#[inline]
#[must_use]
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

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/util/json_util.rs` (line 159)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Returns error if deserialization or string parsing fails
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