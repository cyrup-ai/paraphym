# `forks/surrealdb/crates/core/src/val/value/pick.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: dd3d391b  
- **Timestamp**: 2025-10-10T02:16:00.705361+00:00  
- **Lines of Code**: 128

---## Panic-Prone Code


### Line 73: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn pick_none() {
		let idi: Idiom = SqlIdiom::default().into();
		let val = syn::value("{ test: { other: null, something: 123 } }").unwrap();
		let res = val.pick(&idi);
		assert_eq!(res, val);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 80: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn pick_basic() {
		let idi: Idiom = syn::idiom("test.something").unwrap().into();
		let val = syn::value("{ test: { other: null, something: 123 } }").unwrap();
		let res = val.pick(&idi);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 81: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn pick_basic() {
		let idi: Idiom = syn::idiom("test.something").unwrap().into();
		let val = syn::value("{ test: { other: null, something: 123 } }").unwrap();
		let res = val.pick(&idi);
		assert_eq!(res, Value::from(123));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 88: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn pick_thing() {
		let idi: Idiom = syn::idiom("test.other").unwrap().into();
		let val = syn::value("{ test: { other: test:tobie, something: 123 } }").unwrap();
		let res = val.pick(&idi);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 89: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn pick_thing() {
		let idi: Idiom = syn::idiom("test.other").unwrap().into();
		let val = syn::value("{ test: { other: test:tobie, something: 123 } }").unwrap();
		let res = val.pick(&idi);
		assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 102: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn pick_array() {
		let idi: Idiom = syn::idiom("test.something[1]").unwrap().into();
		let val = syn::value("{ test: { something: [123, 456, 789] } }").unwrap();
		let res = val.pick(&idi);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 103: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn pick_array() {
		let idi: Idiom = syn::idiom("test.something[1]").unwrap().into();
		let val = syn::value("{ test: { something: [123, 456, 789] } }").unwrap();
		let res = val.pick(&idi);
		assert_eq!(res, Value::from(456));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 110: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn pick_array_thing() {
		let idi: Idiom = syn::idiom("test.something[1]").unwrap().into();
		let val = syn::value("{ test: { something: [test:tobie, test:jaime] } }").unwrap();
		let res = val.pick(&idi);
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
	fn pick_array_thing() {
		let idi: Idiom = syn::idiom("test.something[1]").unwrap().into();
		let val = syn::value("{ test: { something: [test:tobie, test:jaime] } }").unwrap();
		let res = val.pick(&idi);
		assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 124: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn pick_array_field() {
		let idi: Idiom = syn::idiom("test.something[1].age").unwrap().into();
		let val = syn::value("{ test: { something: [{ age: 34 }, { age: 36 }] } }").unwrap();
		let res = val.pick(&idi);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 125: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn pick_array_field() {
		let idi: Idiom = syn::idiom("test.something[1].age").unwrap().into();
		let val = syn::value("{ test: { something: [{ age: 34 }, { age: 36 }] } }").unwrap();
		let res = val.pick(&idi);
		assert_eq!(res, Value::from(36));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 132: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn pick_array_fields() {
		let idi: Idiom = syn::idiom("test.something[*].age").unwrap().into();
		let val = syn::value("{ test: { something: [{ age: 34 }, { age: 36 }] } }").unwrap();
		let res = val.pick(&idi);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 133: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn pick_array_fields() {
		let idi: Idiom = syn::idiom("test.something[*].age").unwrap().into();
		let val = syn::value("{ test: { something: [{ age: 34 }, { age: 36 }] } }").unwrap();
		let res = val.pick(&idi);
		assert_eq!(res, [Value::from(34i64), Value::from(36i64)].into_iter().collect::<Value>());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 140: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn pick_array_fields_flat() {
		let idi: Idiom = syn::idiom("test.something.age").unwrap().into();
		let val = syn::value("{ test: { something: [{ age: 34 }, { age: 36 }] } }").unwrap();
		let res = val.pick(&idi);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 141: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn pick_array_fields_flat() {
		let idi: Idiom = syn::idiom("test.something.age").unwrap().into();
		let val = syn::value("{ test: { something: [{ age: 34 }, { age: 36 }] } }").unwrap();
		let res = val.pick(&idi);
		assert_eq!(res, [Value::from(34i64), Value::from(36i64)].into_iter().collect::<Value>());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 62: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/pick.rs` (line 62)
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
  


### Line 71: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/pick.rs` (line 71)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn pick_none() {
		let idi: Idiom = SqlIdiom::default().into();
		let val = syn::value("{ test: { other: null, something: 123 } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 79: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/pick.rs` (line 79)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn pick_basic() {
		let idi: Idiom = syn::idiom("test.something").unwrap().into();
		let val = syn::value("{ test: { other: null, something: 123 } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 87: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/pick.rs` (line 87)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn pick_thing() {
		let idi: Idiom = syn::idiom("test.other").unwrap().into();
		let val = syn::value("{ test: { other: test:tobie, something: 123 } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 101: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/pick.rs` (line 101)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn pick_array() {
		let idi: Idiom = syn::idiom("test.something[1]").unwrap().into();
		let val = syn::value("{ test: { something: [123, 456, 789] } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 109: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/pick.rs` (line 109)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn pick_array_thing() {
		let idi: Idiom = syn::idiom("test.something[1]").unwrap().into();
		let val = syn::value("{ test: { something: [test:tobie, test:jaime] } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 123: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/pick.rs` (line 123)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn pick_array_field() {
		let idi: Idiom = syn::idiom("test.something[1].age").unwrap().into();
		let val = syn::value("{ test: { something: [{ age: 34 }, { age: 36 }] } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 131: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/pick.rs` (line 131)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn pick_array_fields() {
		let idi: Idiom = syn::idiom("test.something[*].age").unwrap().into();
		let val = syn::value("{ test: { something: [{ age: 34 }, { age: 36 }] } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 139: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/pick.rs` (line 139)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn pick_array_fields_flat() {
		let idi: Idiom = syn::idiom("test.something.age").unwrap().into();
		let val = syn::value("{ test: { something: [{ age: 34 }, { age: 36 }] } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym