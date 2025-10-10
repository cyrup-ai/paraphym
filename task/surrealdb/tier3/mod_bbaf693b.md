# `forks/surrealdb/src/cli/validator/mod.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: surrealdb
- **File Hash**: bbaf693b  
- **Timestamp**: 2025-10-10T02:16:01.061282+00:00  
- **Lines of Code**: 257

---## Tests in Source Directory


### Line 192: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/src/cli/validator/mod.rs` (line 192)
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
  


### Line 196: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/src/cli/validator/mod.rs` (line 196)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn test_func_targets() {
		assert_eq!(func_targets("*").unwrap(), Targets::<FuncTarget>::All);
		assert_eq!(func_targets("").unwrap(), Targets::<FuncTarget>::All);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 212: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/src/cli/validator/mod.rs` (line 212)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn test_net_targets() {
		assert_eq!(net_targets("*").unwrap(), Targets::<NetTarget>::All);
		assert_eq!(net_targets("").unwrap(), Targets::<NetTarget>::All);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 237: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/src/cli/validator/mod.rs` (line 237)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn test_method_targets() {
		assert_eq!(method_targets("*").unwrap(), Targets::<MethodTarget>::All);
		assert_eq!(method_targets("").unwrap(), Targets::<MethodTarget>::All);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 255: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/src/cli/validator/mod.rs` (line 255)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn test_route_targets() {
		assert_eq!(route_targets("*").unwrap(), Targets::<RouteTarget>::All);
		assert_eq!(route_targets("").unwrap(), Targets::<RouteTarget>::All);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 271: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/src/cli/validator/mod.rs` (line 271)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn test_arbitrary_query_targets() {
		assert_eq!(query_arbitrary_targets("*").unwrap(), Targets::<ArbitraryQueryTarget>::All);
		assert_eq!(query_arbitrary_targets("").unwrap(), Targets::<ArbitraryQueryTarget>::All);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `func_targets()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/src/cli/validator/mod.rs` (line 107)
- **Visibility**: pub(crate)
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub(crate) fn func_targets(value: &str) -> Result<Targets<FuncTarget>, String> {
	if ["*", ""].contains(&value) {
		return Ok(Targets::All);
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `file_exists()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/src/cli/validator/mod.rs` (line 36)
- **Visibility**: pub(crate)
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub(crate) fn file_exists(path: &str) -> Result<PathBuf, String> {
	let path = path_exists(path)?;
	if !path.is_file() {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `net_targets()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/src/cli/validator/mod.rs` (line 93)
- **Visibility**: pub(crate)
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub(crate) fn net_targets(value: &str) -> Result<Targets<NetTarget>, String> {
	if ["*", ""].contains(&value) {
		return Ok(Targets::All);
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `key_valid()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/src/cli/validator/mod.rs` (line 80)
- **Visibility**: pub(crate)
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub(crate) fn key_valid(v: &str) -> Result<String, String> {
	match v.len() {
		16 => Ok(v.to_string()),
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `export_tables()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/src/cli/validator/mod.rs` (line 179)
- **Visibility**: pub(crate)
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub(crate) fn export_tables(value: &str) -> Result<TableConfig, String> {
	if ["*", "", "true"].contains(&value) {
		return Ok(TableConfig::All);
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `experimental_targets()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/src/cli/validator/mod.rs` (line 121)
- **Visibility**: pub(crate)
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub(crate) fn experimental_targets(value: &str) -> Result<Targets<ExperimentalTarget>, String> {
	if ["*", ""].contains(&value) {
		return Ok(Targets::All);
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `endpoint_valid()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/src/cli/validator/mod.rs` (line 52)
- **Visibility**: pub(crate)
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub(crate) fn endpoint_valid(v: &str) -> Result<String, String> {
	fn split_endpoint(v: &str) -> (&str, &str) {
		match v {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `query_arbitrary_targets()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/src/cli/validator/mod.rs` (line 135)
- **Visibility**: pub(crate)
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub(crate) fn query_arbitrary_targets(
	value: &str,
) -> Result<Targets<ArbitraryQueryTarget>, String> {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `dir_exists()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/src/cli/validator/mod.rs` (line 44)
- **Visibility**: pub(crate)
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub(crate) fn dir_exists(path: &str) -> Result<PathBuf, String> {
	let path = path_exists(path)?;
	if !path.is_dir() {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `path_valid()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/src/cli/validator/mod.rs` (line 15)
- **Visibility**: pub(crate)
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
pub(crate) mod parser;

pub(crate) fn path_valid(v: &str) -> Result<String, String> {
	match v {
		"memory" => Ok(v.to_string()),
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `route_targets()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/src/cli/validator/mod.rs` (line 165)
- **Visibility**: pub(crate)
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub(crate) fn route_targets(value: &str) -> Result<Targets<RouteTarget>, String> {
	if ["*", ""].contains(&value) {
		return Ok(Targets::All);
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `method_targets()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/src/cli/validator/mod.rs` (line 151)
- **Visibility**: pub(crate)
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub(crate) fn method_targets(value: &str) -> Result<Targets<MethodTarget>, String> {
	if ["*", ""].contains(&value) {
		return Ok(Targets::All);
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym