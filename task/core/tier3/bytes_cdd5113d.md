# `forks/surrealdb/crates/core/src/sql/bytes.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: cdd5113d  
- **Timestamp**: 2025-10-10T02:16:00.717249+00:00  
- **Lines of Code**: 89

---## Panic-Prone Code


### Line 91: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn serialize() {
		let val = SqlValue::Bytes(Bytes(vec![1, 2, 3, 5]));
		let serialized: Vec<u8> = revision::to_vec(&val).unwrap();
		println!("{serialized:?}");
		let deserialized: SqlValue = revision::from_slice(&serialized).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 93: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let serialized: Vec<u8> = revision::to_vec(&val).unwrap();
		println!("{serialized:?}");
		let deserialized: SqlValue = revision::from_slice(&serialized).unwrap();
		assert_eq!(val, deserialized);
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 100: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn json_roundtrip() {
		let val = Bytes::from(vec![1, 2, 3, 5]);
		let json = serde_json::to_string(&val).unwrap();
		let deserialized = serde_json::from_str(&json).unwrap();
		assert_eq!(val, deserialized);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 101: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let val = Bytes::from(vec![1, 2, 3, 5]);
		let json = serde_json::to_string(&val).unwrap();
		let deserialized = serde_json::from_str(&json).unwrap();
		assert_eq!(val, deserialized);
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 85: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/sql/bytes.rs` (line 85)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
	use crate::sql::{Bytes, SqlValue};

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 89: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/sql/bytes.rs` (line 89)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn serialize() {
		let val = SqlValue::Bytes(Bytes(vec![1, 2, 3, 5]));
		let serialized: Vec<u8> = revision::to_vec(&val).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 98: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/sql/bytes.rs` (line 98)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn json_roundtrip() {
		let val = Bytes::from(vec![1, 2, 3, 5]);
		let json = serde_json::to_string(&val).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym