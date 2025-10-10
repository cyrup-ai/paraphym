# `forks/surrealdb/crates/core/src/val/value/changed.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: c82f6dd5  
- **Timestamp**: 2025-10-10T02:16:00.715790+00:00  
- **Lines of Code**: 80

---## Tier 1 Infractions 


- Line 14
  - TODO
  - 

```rust
				for (key, _) in a.iter() {
					if !b.contains_key(key) {
						// TODO: null byte validity.
						let path = Idiom::field(Ident::new(key.clone()).unwrap());
						chg.put(&path, Value::None);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 24
  - TODO
  - 

```rust
						// Key did not exist
						None => {
							// TODO: null byte validity.
							let path = Idiom::field(Ident::new(key.clone()).unwrap());
							chg.put(&path, val.clone());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 30
  - TODO
  - 

```rust
						Some(old) => {
							if old != val {
								// TODO: null byte validity.
								let path = Idiom::field(Ident::new(key.clone()).unwrap());
								chg.put(&path, old.changed(val));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 15: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
					if !b.contains_key(key) {
						// TODO: null byte validity.
						let path = Idiom::field(Ident::new(key.clone()).unwrap());
						chg.put(&path, Value::None);
					}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 25: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
						None => {
							// TODO: null byte validity.
							let path = Idiom::field(Ident::new(key.clone()).unwrap());
							chg.put(&path, val.clone());
						}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 31: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
							if old != val {
								// TODO: null byte validity.
								let path = Idiom::field(Ident::new(key.clone()).unwrap());
								chg.put(&path, old.changed(val));
							}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 51: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn changed_none() {
		let old = syn::value("{ test: true, text: 'text', other: { something: true } }").unwrap();
		let now = syn::value("{ test: true, text: 'text', other: { something: true } }").unwrap();
		let res = syn::value("{}").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 52: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn changed_none() {
		let old = syn::value("{ test: true, text: 'text', other: { something: true } }").unwrap();
		let now = syn::value("{ test: true, text: 'text', other: { something: true } }").unwrap();
		let res = syn::value("{}").unwrap();
		assert_eq!(res, old.changed(&now));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 53: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let old = syn::value("{ test: true, text: 'text', other: { something: true } }").unwrap();
		let now = syn::value("{ test: true, text: 'text', other: { something: true } }").unwrap();
		let res = syn::value("{}").unwrap();
		assert_eq!(res, old.changed(&now));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 59: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn changed_add() {
		let old = syn::value("{ test: true }").unwrap();
		let now = syn::value("{ test: true, other: 'test' }").unwrap();
		let res = syn::value("{ other: 'test' }").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 60: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn changed_add() {
		let old = syn::value("{ test: true }").unwrap();
		let now = syn::value("{ test: true, other: 'test' }").unwrap();
		let res = syn::value("{ other: 'test' }").unwrap();
		assert_eq!(res, old.changed(&now));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 61: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let old = syn::value("{ test: true }").unwrap();
		let now = syn::value("{ test: true, other: 'test' }").unwrap();
		let res = syn::value("{ other: 'test' }").unwrap();
		assert_eq!(res, old.changed(&now));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 67: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn changed_remove() {
		let old = syn::value("{ test: true, other: 'test' }").unwrap();
		let now = syn::value("{ test: true }").unwrap();
		let res = syn::value("{ other: NONE }").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 68: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn changed_remove() {
		let old = syn::value("{ test: true, other: 'test' }").unwrap();
		let now = syn::value("{ test: true }").unwrap();
		let res = syn::value("{ other: NONE }").unwrap();
		assert_eq!(res, old.changed(&now));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 69: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let old = syn::value("{ test: true, other: 'test' }").unwrap();
		let now = syn::value("{ test: true }").unwrap();
		let res = syn::value("{ other: NONE }").unwrap();
		assert_eq!(res, old.changed(&now));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 75: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn changed_add_array() {
		let old = syn::value("{ test: [1,2,3] }").unwrap();
		let now = syn::value("{ test: [1,2,3,4] }").unwrap();
		let res = syn::value("{ test: [1,2,3,4] }").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 76: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn changed_add_array() {
		let old = syn::value("{ test: [1,2,3] }").unwrap();
		let now = syn::value("{ test: [1,2,3,4] }").unwrap();
		let res = syn::value("{ test: [1,2,3,4] }").unwrap();
		assert_eq!(res, old.changed(&now));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 77: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let old = syn::value("{ test: [1,2,3] }").unwrap();
		let now = syn::value("{ test: [1,2,3,4] }").unwrap();
		let res = syn::value("{ test: [1,2,3,4] }").unwrap();
		assert_eq!(res, old.changed(&now));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 83: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn changed_replace_embedded() {
		let old = syn::value("{ test: { other: 'test' } }").unwrap();
		let now = syn::value("{ test: { other: false } }").unwrap();
		let res = syn::value("{ test: { other: false } }").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 84: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn changed_replace_embedded() {
		let old = syn::value("{ test: { other: 'test' } }").unwrap();
		let now = syn::value("{ test: { other: false } }").unwrap();
		let res = syn::value("{ test: { other: false } }").unwrap();
		assert_eq!(res, old.changed(&now));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 85: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let old = syn::value("{ test: { other: 'test' } }").unwrap();
		let now = syn::value("{ test: { other: false } }").unwrap();
		let res = syn::value("{ test: { other: false } }").unwrap();
		assert_eq!(res, old.changed(&now));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 91: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn changed_change_text() {
		let old = syn::value("{ test: { other: 'test' } }").unwrap();
		let now = syn::value("{ test: { other: 'text' } }").unwrap();
		let res = syn::value("{ test: { other: 'text' } }").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 92: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn changed_change_text() {
		let old = syn::value("{ test: { other: 'test' } }").unwrap();
		let now = syn::value("{ test: { other: 'text' } }").unwrap();
		let res = syn::value("{ test: { other: 'text' } }").unwrap();
		assert_eq!(res, old.changed(&now));
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
		let old = syn::value("{ test: { other: 'test' } }").unwrap();
		let now = syn::value("{ test: { other: 'text' } }").unwrap();
		let res = syn::value("{ test: { other: 'text' } }").unwrap();
		assert_eq!(res, old.changed(&now));
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


### Line 46: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/changed.rs` (line 46)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
	use crate::syn;

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 50: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/changed.rs` (line 50)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn changed_none() {
		let old = syn::value("{ test: true, text: 'text', other: { something: true } }").unwrap();
		let now = syn::value("{ test: true, text: 'text', other: { something: true } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 58: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/changed.rs` (line 58)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn changed_add() {
		let old = syn::value("{ test: true }").unwrap();
		let now = syn::value("{ test: true, other: 'test' }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 66: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/changed.rs` (line 66)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn changed_remove() {
		let old = syn::value("{ test: true, other: 'test' }").unwrap();
		let now = syn::value("{ test: true }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 74: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/changed.rs` (line 74)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn changed_add_array() {
		let old = syn::value("{ test: [1,2,3] }").unwrap();
		let now = syn::value("{ test: [1,2,3,4] }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 82: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/changed.rs` (line 82)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn changed_replace_embedded() {
		let old = syn::value("{ test: { other: 'test' } }").unwrap();
		let now = syn::value("{ test: { other: false } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 90: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/changed.rs` (line 90)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn changed_change_text() {
		let old = syn::value("{ test: { other: 'test' } }").unwrap();
		let now = syn::value("{ test: { other: 'text' } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym