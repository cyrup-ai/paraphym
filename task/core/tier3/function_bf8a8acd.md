# `forks/surrealdb/crates/core/src/syn/parser/function.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: bf8a8acd  
- **Timestamp**: 2025-10-10T02:16:00.682439+00:00  
- **Lines of Code**: 321

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 321 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Panic-Prone Code


### Line 132: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn function_single() {
		let sql = "count()";
		let out = syn::expr(sql).unwrap();
		assert_eq!("count()", format!("{}", out));
		let Expr::FunctionCall(f) = out else {
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
	fn function_single_not() {
		let sql = "not(10)";
		let out = syn::expr(sql).unwrap();
		assert_eq!("not(10)", format!("{}", out));
		let Expr::FunctionCall(f) = out else {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 156: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn function_module() {
		let sql = "rand::uuid()";
		let out = syn::expr(sql).unwrap();
		assert_eq!("rand::uuid()", format!("{}", out));
		let Expr::FunctionCall(f) = out else {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 168: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn function_arguments() {
		let sql = "string::is::numeric(null)";
		let out = syn::expr(sql).unwrap();
		assert_eq!("string::is::numeric(NULL)", format!("{}", out));
		let Expr::FunctionCall(f) = out else {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 180: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn function_simple_together() {
		let sql = "function() { return 'test'; }";
		let out = syn::expr(sql).unwrap();
		assert_eq!("function() { return 'test'; }", format!("{}", out));
		let Expr::FunctionCall(f) = out else {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 192: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn function_simple_whitespace() {
		let sql = "function () { return 'test'; }";
		let out = syn::expr(sql).unwrap();
		assert_eq!("function() { return 'test'; }", format!("{}", out));
		let Expr::FunctionCall(f) = out else {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 204: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn function_script_expression() {
		let sql = "function() { return this.tags.filter(t => { return t.length > 3; }); }";
		let out = syn::expr(sql).unwrap();
		assert_eq!(
			"function() { return this.tags.filter(t => { return t.length > 3; }); }",
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 229: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
 		})
 		"#;
		let out = syn::expr(sql).unwrap();
		assert_eq!(
			"ml::insurance::prediction<1.0.0>({ age: 18, disposable_income: 'yes', purchased_before: true })",
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 248: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
 			}) AS likely_to_buy FROM person:tobie;
 		";
		let out = syn::parse(sql).unwrap();
		assert_eq!(
			"SELECT name, age, ml::insurance::prediction<1.0.0>({ age: age, disposable_income: math::round(income), purchased_before: array::len(->purchased->property) > 0 }) AS likely_to_buy FROM person:tobie;",
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 258: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn ml_model_with_mutiple_arguments() {
		let sql = "ml::insurance::prediction<1.0.0>(1,2,3,4,)";
		let out = syn::expr(sql).unwrap();
		assert_eq!("ml::insurance::prediction<1.0.0>(1, 2, 3, 4)", out.to_string());
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 265: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn script_basic() {
		let sql = "function(){return true;}";
		let out = syn::expr(sql).unwrap();
		assert_eq!("function() {return true;}", format!("{}", out));
		let Expr::FunctionCall(f) = out else {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 277: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn script_object() {
		let sql = "function(){return { test: true, something: { other: true } };}";
		let out = syn::expr(sql).unwrap();
		assert_eq!(
			"function() {return { test: true, something: { other: true } };}",
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 298: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn script_closure() {
		let sql = "function(){return this.values.map(v => `This value is ${Number(v * 3)}`);}";
		let out = syn::expr(sql).unwrap();
		assert_eq!(
			"function() {return this.values.map(v => `This value is ${Number(v * 3)}`);}",
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 319: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn script_complex() {
		let sql = r#"function(){return { test: true, some: { object: "some text with uneven {{{ {} \" brackets", else: false } };}"#;
		let out = syn::expr(sql).unwrap();
		assert_eq!(
			r#"function() {return { test: true, some: { object: "some text with uneven {{{ {} \" brackets", else: false } };}"#,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 359: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
 			"#;
		let sql = "function() {".to_owned() + body + "}";
		let out = syn::expr(&sql).unwrap();

		assert_eq!(sql, format!("{}", out));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 125: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/function.rs` (line 125)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod test {
	use super::*;
	use crate::{sql, syn};
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 130: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/function.rs` (line 130)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn function_single() {
		let sql = "count()";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 142: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/function.rs` (line 142)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn function_single_not() {
		let sql = "not(10)";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 154: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/function.rs` (line 154)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn function_module() {
		let sql = "rand::uuid()";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 166: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/function.rs` (line 166)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn function_arguments() {
		let sql = "string::is::numeric(null)";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 178: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/function.rs` (line 178)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn function_simple_together() {
		let sql = "function() { return 'test'; }";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 190: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/function.rs` (line 190)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn function_simple_whitespace() {
		let sql = "function () { return 'test'; }";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 202: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/function.rs` (line 202)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn function_script_expression() {
		let sql = "function() { return this.tags.filter(t => { return t.length > 3; }); }";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 222: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/function.rs` (line 222)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn ml_model_example() {
		let sql = r#"ml::insurance::prediction<1.0.0>({
 			age: 18,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 237: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/function.rs` (line 237)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn ml_model_example_in_select() {
		let sql = r"
 			SELECT
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 256: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/function.rs` (line 256)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn ml_model_with_mutiple_arguments() {
		let sql = "ml::insurance::prediction<1.0.0>(1,2,3,4,)";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 263: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/function.rs` (line 263)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn script_basic() {
		let sql = "function(){return true;}";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 275: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/function.rs` (line 275)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn script_object() {
		let sql = "function(){return { test: true, something: { other: true } };}";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 296: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/function.rs` (line 296)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn script_closure() {
		let sql = "function(){return this.values.map(v => `This value is ${Number(v * 3)}`);}";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 317: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/function.rs` (line 317)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn script_complex() {
		let sql = r#"function(){return { test: true, some: { object: "some text with uneven {{{ {} \" brackets", else: false } };}"#;
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 336: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/function.rs` (line 336)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn script_advanced() {
		let body = r#"
 			// {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym