# `forks/surrealdb/crates/core/src/idx/ft/offset.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: 63e1e44b  
- **Timestamp**: 2025-10-10T02:16:00.709133+00:00  
- **Lines of Code**: 99

---## Panic-Prone Code


### Line 103: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			Offset::new(1, 1, 3, 4),
		]);
		let v: Val = o.clone().kv_encode_value().unwrap();
		let o2 = OffsetRecords::kv_decode_value(v).unwrap();
		assert_eq!(o, o2)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 104: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		]);
		let v: Val = o.clone().kv_encode_value().unwrap();
		let o2 = OffsetRecords::kv_decode_value(v).unwrap();
		assert_eq!(o, o2)
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 111: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn test_migrate_v1_offset_records() {
		let decompressed = vec![3u32, 0, 0, 1, 1, 3, 11, 22, 1, 4];
		let v = bincode::serialize(&decompressed).unwrap();
		let o: OffsetRecords = OffsetRecords::kv_decode_value(v).unwrap();
		assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 112: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let decompressed = vec![3u32, 0, 0, 1, 1, 3, 11, 22, 1, 4];
		let v = bincode::serialize(&decompressed).unwrap();
		let o: OffsetRecords = OffsetRecords::kv_decode_value(v).unwrap();
		assert_eq!(
			o,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 92: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/offset.rs` (line 92)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
	use crate::idx::ft::offset::{Offset, OffsetRecords};
	use crate::kvs::{KVValue, Val};
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 97: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/offset.rs` (line 97)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn test_offset_records() {
		let o = OffsetRecords(vec![
			Offset::new(0, 1, 2, 3),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 109: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/ft/offset.rs` (line 109)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn test_migrate_v1_offset_records() {
		let decompressed = vec![3u32, 0, 0, 1, 1, 3, 11, 22, 1, 4];
		let v = bincode::serialize(&decompressed).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym