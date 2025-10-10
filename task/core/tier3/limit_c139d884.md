# `forks/surrealdb/crates/core/src/syn/parser/test/limit.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: c139d884  
- **Timestamp**: 2025-10-10T02:16:00.700846+00:00  
- **Lines of Code**: 273

---## Panic-Prone Code


### Line 31: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
		.enter(|stk| parser.parse_query(stk))
		.finish()
		.expect("recursion limit of 5 couldn't parse 5 deep object");

	let source = r#"
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 77: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
		.enter(|stk| parser.parse_query(stk))
		.finish()
		.expect("recursion limit of 5 couldn't parse 5 deep object");

	let source = r#"
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 131: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
		.enter(|stk| parser.parse_query(stk))
		.finish()
		.expect("recursion limit of 5 couldn't parse 5 deep object");

	let mut stack = Stack::new();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 189: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
		.enter(|stk| parser.parse_query(stk))
		.finish()
		.expect("recursion limit of 5 couldn't parse 5 deep query");

	let source = r#"
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 232: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
		.enter(|stk| parser.parse_query(stk))
		.finish()
		.expect("recursion limit of 5 couldn't parse 5 deep query");

	let source = r#"
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 277: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
		.enter(|stk| parser.parse_query(stk))
		.finish()
		.expect("recursion limit of 5 couldn't parse 5 deep query");

	let source = r#"
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 6: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/limit.rs` (line 6)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn object_depth() {
	let mut stack = Stack::new();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 61: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/limit.rs` (line 61)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn array_depth() {
	let mut stack = Stack::new();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 96: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/limit.rs` (line 96)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn object_depth_succeed_then_fail() {
	let mut stack = Stack::new();
	let source = r#"
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 173: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/limit.rs` (line 173)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn query_depth_subquery() {
	let mut stack = Stack::new();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 208: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/limit.rs` (line 208)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn query_depth_block() {
	let mut stack = Stack::new();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 261: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/test/limit.rs` (line 261)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn query_depth_if() {
	let mut stack = Stack::new();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym