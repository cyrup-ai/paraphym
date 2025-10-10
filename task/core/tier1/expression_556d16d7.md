# `forks/surrealdb/crates/core/src/syn/parser/expression.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: 556d16d7  
- **Timestamp**: 2025-10-10T02:16:00.661672+00:00  
- **Lines of Code**: 665

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 665 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 78
  - TODO
  - 

```rust
	/// module are left associative or have no defined associativity.
	fn infix_binding_power(&mut self, token: TokenKind) -> Option<BindingPower> {
		// TODO: Look at ordering of operators.
		match token {
			// assigment operators have the lowest binding power.
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 350
  - TODO
  - 

```rust

	fn expr_is_range(expr: &Expr) -> bool {
		//TODO(EXPR): Prefix and Postfix range
		match expr {
			Expr::Binary {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 384
  - TODO
  - 

```rust
		let token = self.next();
		let operator = match token.kind {
			// TODO: change operator name?
			t!("||") | t!("OR") => BinaryOperator::Or,
			t!("&&") | t!("AND") => BinaryOperator::And,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 675: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn cast_int() {
		let sql = "<int>1.2345";
		let out = syn::expr(sql).unwrap();
		assert_eq!("<int> 1.2345f", format!("{}", out));
		assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 689: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn cast_string() {
		let sql = "<string>1.2345";
		let out = syn::expr(sql).unwrap();
		assert_eq!("<string> 1.2345f", format!("{}", out));
		assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 703: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn expression_statement() {
		let sql = "true AND false";
		let out = syn::expr(sql).unwrap();
		assert_eq!("true AND false", format!("{}", out));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 710: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn expression_left_opened() {
		let sql = "3 * 3 * 3 = 27";
		let out = syn::expr(sql).unwrap();
		assert_eq!("3 * 3 * 3 = 27", format!("{}", out));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 717: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn expression_left_closed() {
		let sql = "(3 * 3 * 3) = 27";
		let out = syn::expr(sql).unwrap();
		assert_eq!("3 * 3 * 3 = 27", format!("{}", out));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 724: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn expression_right_opened() {
		let sql = "27 = 3 * 3 * 3";
		let out = syn::expr(sql).unwrap();
		assert_eq!("27 = 3 * 3 * 3", format!("{}", out));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 731: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn expression_right_closed() {
		let sql = "27 = (3 * 3 * 3)";
		let out = syn::expr(sql).unwrap();
		assert_eq!("27 = 3 * 3 * 3", format!("{}", out));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 738: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn expression_both_opened() {
		let sql = "3 * 3 * 3 = 3 * 3 * 3";
		let out = syn::expr(sql).unwrap();
		assert_eq!("3 * 3 * 3 = 3 * 3 * 3", format!("{}", out));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 745: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn expression_both_closed() {
		let sql = "(3 * 3 * 3) = (3 * 3 * 3)";
		let out = syn::expr(sql).unwrap();
		assert_eq!("3 * 3 * 3 = 3 * 3 * 3", format!("{}", out));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 752: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn expression_closed_required() {
		let sql = "(3 + 3) * 3";
		let out = syn::expr(sql).unwrap();
		assert_eq!("(3 + 3) * 3", format!("{}", out));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 759: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn range_closed_required() {
		let sql = "(1..2)..3";
		let out = syn::expr(sql).unwrap();
		assert_eq!("(1..2)..3", format!("{}", out));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 766: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn expression_unary() {
		let sql = "-a";
		let out = syn::expr(sql).unwrap();
		assert_eq!(sql, format!("{}", out));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 773: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn expression_with_unary() {
		let sql = "-(5) + 5";
		let out = syn::expr(sql).unwrap();
		assert_eq!("-5 + 5", format!("{}", out));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 780: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn expression_left_associative() {
		let sql = "1 - 1 - 1";
		let out = syn::expr(sql).unwrap();
		let one = Expr::Literal(Literal::Integer(1));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 668: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/expression.rs` (line 668)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod test {
	use crate::sql::{BinaryOperator, Expr, Kind, Literal, PrefixOperator};
	use crate::syn;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 673: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/expression.rs` (line 673)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn cast_int() {
		let sql = "<int>1.2345";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 687: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/expression.rs` (line 687)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn cast_string() {
		let sql = "<string>1.2345";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 701: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/expression.rs` (line 701)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn expression_statement() {
		let sql = "true AND false";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 708: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/expression.rs` (line 708)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn expression_left_opened() {
		let sql = "3 * 3 * 3 = 27";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 715: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/expression.rs` (line 715)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn expression_left_closed() {
		let sql = "(3 * 3 * 3) = 27";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 722: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/expression.rs` (line 722)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn expression_right_opened() {
		let sql = "27 = 3 * 3 * 3";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 729: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/expression.rs` (line 729)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn expression_right_closed() {
		let sql = "27 = (3 * 3 * 3)";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 736: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/expression.rs` (line 736)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn expression_both_opened() {
		let sql = "3 * 3 * 3 = 3 * 3 * 3";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 743: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/expression.rs` (line 743)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn expression_both_closed() {
		let sql = "(3 * 3 * 3) = (3 * 3 * 3)";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 750: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/expression.rs` (line 750)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn expression_closed_required() {
		let sql = "(3 + 3) * 3";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 757: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/expression.rs` (line 757)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn range_closed_required() {
		let sql = "(1..2)..3";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 764: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/expression.rs` (line 764)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn expression_unary() {
		let sql = "-a";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 771: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/expression.rs` (line 771)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn expression_with_unary() {
		let sql = "-(5) + 5";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 778: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/syn/parser/expression.rs` (line 778)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn expression_left_associative() {
		let sql = "1 - 1 - 1";
		let out = syn::expr(sql).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym