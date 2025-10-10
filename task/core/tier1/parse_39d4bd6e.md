# `forks/surrealdb/crates/core/src/fnc/parse.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: 39d4bd6e  
- **Timestamp**: 2025-10-10T02:16:00.710819+00:00  
- **Lines of Code**: 122

---## Tier 1 Infractions 


- Line 132
  - hardcoded URL
  - 

```rust
		#[test]
		fn port_default_port_specified() {
			let value = super::port(("http://www.google.com:80".to_string(),)).unwrap();
			assert_eq!(value, Value::from(80));
		}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 138
  - hardcoded URL
  - 

```rust
		#[test]
		fn port_nondefault_port_specified() {
			let value = super::port(("http://www.google.com:8080".to_string(),)).unwrap();
			assert_eq!(value, Value::from(8080));
		}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 144
  - hardcoded URL
  - 

```rust
		#[test]
		fn port_no_port_specified() {
			let value = super::port(("http://www.google.com".to_string(),)).unwrap();
			assert_eq!(value, Value::from(80));
		}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 36: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		fn host() {
			let input = (String::from("john.doe@example.com"),);
			let value = super::host(input).unwrap();
			assert_eq!(value, Value::from("example.com"));
		}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 43: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		fn user() {
			let input = (String::from("john.doe@example.com"),);
			let value = super::user(input).unwrap();
			assert_eq!(value, Value::from("john.doe"));
		}
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
		fn port_default_port_specified() {
			let value = super::port(("http://www.google.com:80".to_string(),)).unwrap();
			assert_eq!(value, Value::from(80));
		}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 138: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		#[test]
		fn port_nondefault_port_specified() {
			let value = super::port(("http://www.google.com:8080".to_string(),)).unwrap();
			assert_eq!(value, Value::from(8080));
		}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 144: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		#[test]
		fn port_no_port_specified() {
			let value = super::port(("http://www.google.com".to_string(),)).unwrap();
			assert_eq!(value, Value::from(80));
		}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 150: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		#[test]
		fn port_no_scheme_no_port_specified() {
			let value = super::port(("www.google.com".to_string(),)).unwrap();
			assert_eq!(value, Value::None);
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


### Line 30: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/parse.rs` (line 30)
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
  


### Line 34: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/parse.rs` (line 34)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

		#[test]
		fn host() {
			let input = (String::from("john.doe@example.com"),);
			let value = super::host(input).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 41: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/parse.rs` (line 41)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

		#[test]
		fn user() {
			let input = (String::from("john.doe@example.com"),);
			let value = super::user(input).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 127: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/parse.rs` (line 127)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[cfg(test)]
	mod tests {
		use crate::val::Value;

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 131: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/parse.rs` (line 131)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

		#[test]
		fn port_default_port_specified() {
			let value = super::port(("http://www.google.com:80".to_string(),)).unwrap();
			assert_eq!(value, Value::from(80));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 137: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/parse.rs` (line 137)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

		#[test]
		fn port_nondefault_port_specified() {
			let value = super::port(("http://www.google.com:8080".to_string(),)).unwrap();
			assert_eq!(value, Value::from(8080));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 143: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/parse.rs` (line 143)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

		#[test]
		fn port_no_port_specified() {
			let value = super::port(("http://www.google.com".to_string(),)).unwrap();
			assert_eq!(value, Value::from(80));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 149: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/parse.rs` (line 149)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

		#[test]
		fn port_no_scheme_no_port_specified() {
			let value = super::port(("www.google.com".to_string(),)).unwrap();
			assert_eq!(value, Value::None);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym