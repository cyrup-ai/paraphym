# `forks/surrealdb/crates/core/src/val/mod.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: a57164cc  
- **Timestamp**: 2025-10-10T02:16:00.656935+00:00  
- **Lines of Code**: 921

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 921 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 151
  - TODO
  - 

```rust
			Value::Number(v) => v.is_truthy(),
			Value::Duration(v) => v.as_nanos() > 0,
			// TODO: Table, range, bytes and closure should probably also have certain truthy
			// values.
			_ => false,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 309
  - TODO
  - 

```rust
	/// don't accidentally use it where it can return an invalid value.
	pub fn kind_of(&self) -> &'static str {
		// TODO: Look at this function, there are a whole bunch of options for which
		// this returns "incorrect type" which might sneak into the results where it
		// shouldn.t
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 338
  - TODO
  - 

```rust
			Self::Range(_) => "range",
			Self::RecordId(_) => "thing",
			// TODO: Dubious types
			Self::Table(_) => "table",
		}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 386
  - TODO
  - 

```rust
			Value::RecordId(v) => match other {
				Value::RecordId(w) => v == w,
				// TODO(3.0.0): Decide if we want to keep this behavior.
				//Value::Regex(w) => w.regex().is_match(v.to_raw().as_str()),
				_ => false,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 397
  - TODO
  - 

```rust
			Value::Regex(v) => match other {
				Value::Regex(w) => v == w,
				// TODO(3.0.0): Decide if we want to keep this behavior.
				//Value::RecordId(w) => v.regex().is_match(w.to_raw().as_str()),
				Value::Strand(w) => v.regex().is_match(w.as_str()),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 933
  - TODO
  - 

```rust
}

// TODO: Remove these implementations
// They truncate by default and therefore should not be implement for value.
impl From<i128> for Value {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 1101: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn check_serialize() {
		let enc: Vec<u8> = revision::to_vec(&Value::None).unwrap();
		assert_eq!(2, enc.len());
		let enc: Vec<u8> = revision::to_vec(&Value::Null).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1103: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let enc: Vec<u8> = revision::to_vec(&Value::None).unwrap();
		assert_eq!(2, enc.len());
		let enc: Vec<u8> = revision::to_vec(&Value::Null).unwrap();
		assert_eq!(2, enc.len());
		let enc: Vec<u8> = revision::to_vec(&Value::Bool(true)).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1105: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let enc: Vec<u8> = revision::to_vec(&Value::Null).unwrap();
		assert_eq!(2, enc.len());
		let enc: Vec<u8> = revision::to_vec(&Value::Bool(true)).unwrap();
		assert_eq!(3, enc.len());
		let enc: Vec<u8> = revision::to_vec(&Value::Bool(false)).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1107: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let enc: Vec<u8> = revision::to_vec(&Value::Bool(true)).unwrap();
		assert_eq!(3, enc.len());
		let enc: Vec<u8> = revision::to_vec(&Value::Bool(false)).unwrap();
		assert_eq!(3, enc.len());
		let enc: Vec<u8> = revision::to_vec(&Value::from("test")).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1109: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let enc: Vec<u8> = revision::to_vec(&Value::Bool(false)).unwrap();
		assert_eq!(3, enc.len());
		let enc: Vec<u8> = revision::to_vec(&Value::from("test")).unwrap();
		assert_eq!(8, enc.len());
		let enc: Vec<u8> = revision::to_vec(&syn::value("{ hello: 'world' }").unwrap()).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1111: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let enc: Vec<u8> = revision::to_vec(&Value::from("test")).unwrap();
		assert_eq!(8, enc.len());
		let enc: Vec<u8> = revision::to_vec(&syn::value("{ hello: 'world' }").unwrap()).unwrap();
		assert_eq!(19, enc.len());
		let enc: Vec<u8> =
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1111: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let enc: Vec<u8> = revision::to_vec(&Value::from("test")).unwrap();
		assert_eq!(8, enc.len());
		let enc: Vec<u8> = revision::to_vec(&syn::value("{ hello: 'world' }").unwrap()).unwrap();
		assert_eq!(19, enc.len());
		let enc: Vec<u8> =
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1114: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		assert_eq!(19, enc.len());
		let enc: Vec<u8> =
			revision::to_vec(&syn::value("{ compact: true, schema: 0 }").unwrap()).unwrap();
		assert_eq!(27, enc.len());
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1114: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		assert_eq!(19, enc.len());
		let enc: Vec<u8> =
			revision::to_vec(&syn::value("{ compact: true, schema: 0 }").unwrap()).unwrap();
		assert_eq!(27, enc.len());
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1123: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			"{ test: { something: [1, 'two', null, test:tobie, { trueee: false, noneee: null }] } }",
		)
		.unwrap();
		let res = syn::value(
			"{ test: { something: [1, 'two', null, test:tobie, { trueee: false, noneee: null }] } }",
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1127: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			"{ test: { something: [1, 'two', null, test:tobie, { trueee: false, noneee: null }] } }",
		)
		.unwrap();
		let enc: Vec<u8> = revision::to_vec(&val).unwrap();
		let dec: Value = revision::from_slice(&enc).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1128: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		)
		.unwrap();
		let enc: Vec<u8> = revision::to_vec(&val).unwrap();
		let dec: Value = revision::from_slice(&enc).unwrap();
		assert_eq!(res, dec);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1129: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		.unwrap();
		let enc: Vec<u8> = revision::to_vec(&val).unwrap();
		let dec: Value = revision::from_slice(&enc).unwrap();
		assert_eq!(res, dec);
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


### Line 1020: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/mod.rs` (line 1020)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
	use chrono::TimeZone;

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1027: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/mod.rs` (line 1027)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn check_none() {
		assert!(Value::None.is_none());
		assert!(!Value::Null.is_none());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1034: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/mod.rs` (line 1034)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn check_null() {
		assert!(Value::Null.is_null());
		assert!(!Value::None.is_null());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1041: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/mod.rs` (line 1041)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn check_true() {
		assert!(!Value::None.is_true());
		assert!(Value::Bool(true).is_true());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1050: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/mod.rs` (line 1050)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn check_false() {
		assert!(!Value::None.is_false());
		assert!(!Value::Bool(true).is_false());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1059: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/mod.rs` (line 1059)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn convert_truthy() {
		assert!(!Value::None.is_truthy());
		assert!(!Value::Null.is_truthy());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1078: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/mod.rs` (line 1078)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn convert_string() {
		assert_eq!(String::from("NONE"), Value::None.as_raw_string());
		assert_eq!(String::from("NULL"), Value::Null.as_raw_string());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1095: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/mod.rs` (line 1095)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn check_size() {
		assert!(64 >= std::mem::size_of::<Value>(), "size of value too big");
	}
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1100: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/mod.rs` (line 1100)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn check_serialize() {
		let enc: Vec<u8> = revision::to_vec(&Value::None).unwrap();
		assert_eq!(2, enc.len());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1119: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/mod.rs` (line 1119)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn serialize_deserialize() {
		let val = syn::value(
			"{ test: { something: [1, 'two', null, test:tobie, { trueee: false, noneee: null }] } }",
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym