# `forks/surrealdb/crates/core/src/fnc/vector.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: c2ed64b6  
- **Timestamp**: 2025-10-10T02:16:00.697327+00:00  
- **Lines of Code**: 155

---## Tests in Source Directory


### Line 146: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/vector.rs` (line 146)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
	use rust_decimal::Decimal;

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 153: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/vector.rs` (line 153)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn vector_scale_int() {
		let input_vector: Vec<Number> = vec![1, 2, 3, 4].into_iter().map(Number::Int).collect();
		let scalar_int = Number::Int(2);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 166: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/vector.rs` (line 166)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn vector_scale_float() {
		let input_vector: Vec<Number> = vec![1, 2, 3, 4].into_iter().map(Number::Int).collect();
		let scalar_float = Number::Float(1.51);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 180: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/vector.rs` (line 180)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn vector_scale_decimal() {
		let input_vector: Vec<Number> = vec![1, 2, 3, 4].into_iter().map(Number::Int).collect();
		let scalar_decimal = Number::Decimal(Decimal::new(3141, 3));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `spearman()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/vector.rs` (line 138)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
	}

	pub fn spearman((_, _): (Vec<Number>, Vec<Number>)) -> Result<Value> {
		Err(anyhow::Error::new(Error::Unimplemented(
			"vector::similarity::spearman() function".to_string(),
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `cosine()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/vector.rs` (line 126)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
	use crate::val::{Number, Value};

	pub fn cosine((a, b): (Vec<Number>, Vec<Number>)) -> Result<Value> {
		Ok(a.cosine_similarity(&b)?.into())
	}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `jaccard()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/vector.rs` (line 130)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
	}

	pub fn jaccard((a, b): (Vec<Number>, Vec<Number>)) -> Result<Value> {
		Ok(a.jaccard_similarity(&b)?.into())
	}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `mahalanobis()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/vector.rs` (line 103)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
	}

	pub fn mahalanobis((_, _): (Vec<Number>, Vec<Number>)) -> Result<Value> {
		Err(anyhow::Error::new(Error::Unimplemented(
			"vector::distance::mahalanobis() function".to_string(),
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym