# `forks/surrealdb/crates/core/src/fnc/string.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: cd05b2a1  
- **Timestamp**: 2025-10-10T02:16:00.666785+00:00  
- **Lines of Code**: 544

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 544 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 51
  - FIXME
  - 

```rust
	)?;

	// FIXME: Use intersperse to avoid intermediate allocation once stable
	// https://github.com/rust-lang/rust/issues/79524
	Ok(strings.join(&chr).into())
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 66
  - TODO
  - 

```rust

pub fn repeat((val, num): (String, i64)) -> Result<Value> {
	//TODO: Deal with truncation of neg:
	let num = num as usize;
	limit("string::repeat", val.len().saturating_mul(num))?;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 166
  - TODO
  - 

```rust

pub fn split((val, chr): (String, String)) -> Result<Value> {
	// TODO: Null byte validity
	Ok(val
		.split(&chr)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 528
  - TODO
  - 

```rust

		pub fn major((version, value): (String, i64)) -> Result<Value> {
			// TODO: Deal with negative trunc:
			let value = value as u64;
			parse_version(&version, "string::semver::set::major", "Invalid semantic version").map(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 539
  - TODO
  - 

```rust

		pub fn minor((version, value): (String, i64)) -> Result<Value> {
			// TODO: Deal with negative trunc:
			let value = value as u64;
			parse_version(&version, "string::semver::set::minor", "Invalid semantic version").map(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 550
  - TODO
  - 

```rust

		pub fn patch((version, value): (String, i64)) -> Result<Value> {
			// TODO: Deal with negative trunc:
			let value = value as u64;

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 169: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	Ok(val
		.split(&chr)
		.map(|x| Value::from(Strand::new(x.to_owned()).unwrap()))
		.collect::<Vec<_>>()
		.into())
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 189: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	Ok(string
		.split_whitespace()
		.map(|v| Value::from(Strand::new(v.to_owned()).unwrap()))
		.collect::<Vec<_>>()
		.into())
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 281: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	use crate::val::{Datetime, Value};

	#[rustfmt::skip] static LATITUDE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("^[-+]?([1-8]?\\d(\\.\\d+)?|90(\\.0+)?)$").unwrap());
	#[rustfmt::skip] static LONGITUDE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("^[-+]?(180(\\.0+)?|((1[0-7]\\d)|([1-9]?\\d))(\\.\\d+)?)$").unwrap());

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 282: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

	#[rustfmt::skip] static LATITUDE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("^[-+]?([1-8]?\\d(\\.\\d+)?|90(\\.0+)?)$").unwrap());
	#[rustfmt::skip] static LONGITUDE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("^[-+]?(180(\\.0+)?|((1[0-7]\\d)|([1-9]?\\d))(\\.\\d+)?)$").unwrap());

	pub fn alphanum((arg,): (String,)) -> Result<Value> {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 609: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		}

		test("foo bar", Value::Regex("foo".parse().unwrap()), "bar", "bar bar");
		test("foo bar", "bar".into(), "foo", "foo foo");
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 634: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn html_encode() {
		let value = super::html::encode((String::from("<div>Hello world!</div>"),)).unwrap();
		assert_eq!(value, Value::Strand("&lt;div&gt;Hello&#32;world!&lt;&#47;div&gt;".into()));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 637: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		assert_eq!(value, Value::Strand("&lt;div&gt;Hello&#32;world!&lt;&#47;div&gt;".into()));

		let value = super::html::encode((String::from("SurrealDB"),)).unwrap();
		assert_eq!(value, Value::Strand("SurrealDB".into()));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 643: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn html_sanitize() {
		let value = super::html::sanitize((String::from("<div>Hello world!</div>"),)).unwrap();
		assert_eq!(value, Value::Strand("<div>Hello world!</div>".into()));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 646: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		assert_eq!(value, Value::Strand("<div>Hello world!</div>".into()));

		let value = super::html::sanitize((String::from("XSS<script>attack</script>"),)).unwrap();
		assert_eq!(value, Value::Strand("XSS".into()));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 652: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn semver_compare() {
		let value = super::semver::compare((String::from("1.2.3"), String::from("1.0.0"))).unwrap();
		assert_eq!(value, Value::from(1));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 655: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		assert_eq!(value, Value::from(1));

		let value = super::semver::compare((String::from("1.2.3"), String::from("1.2.3"))).unwrap();
		assert_eq!(value, Value::from(0));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 658: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		assert_eq!(value, Value::from(0));

		let value = super::semver::compare((String::from("1.0.0"), String::from("1.2.3"))).unwrap();
		assert_eq!(value, Value::from(-1));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 664: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn semver_extract() {
		let value = super::semver::major((String::from("1.2.3"),)).unwrap();
		assert_eq!(value, Value::from(1));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 667: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		assert_eq!(value, Value::from(1));

		let value = super::semver::minor((String::from("1.2.3"),)).unwrap();
		assert_eq!(value, Value::from(2));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 670: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		assert_eq!(value, Value::from(2));

		let value = super::semver::patch((String::from("1.2.3"),)).unwrap();
		assert_eq!(value, Value::from(3));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 676: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn semver_increment() {
		let value = super::semver::inc::major((String::from("1.2.3"),)).unwrap();
		assert_eq!(value, Value::from("2.0.0"));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 679: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		assert_eq!(value, Value::from("2.0.0"));

		let value = super::semver::inc::minor((String::from("1.2.3"),)).unwrap();
		assert_eq!(value, Value::from("1.3.0"));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 682: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		assert_eq!(value, Value::from("1.3.0"));

		let value = super::semver::inc::patch((String::from("1.2.3"),)).unwrap();
		assert_eq!(value, Value::from("1.2.4"));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 688: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn semver_set() {
		let value = super::semver::set::major((String::from("1.2.3"), 9)).unwrap();
		assert_eq!(value, Value::from("9.2.3"));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 691: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		assert_eq!(value, Value::from("9.2.3"));

		let value = super::semver::set::minor((String::from("1.2.3"), 9)).unwrap();
		assert_eq!(value, Value::from("1.9.3"));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 694: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		assert_eq!(value, Value::from("1.9.3"));

		let value = super::semver::set::patch((String::from("1.2.3"), 9)).unwrap();
		assert_eq!(value, Value::from("1.2.9"));
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


### Line 564: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 564)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
	use super::{matches, replace, slice};
	use crate::fnc::args::{Cast, Optional};
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 570: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 570)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn string_slice() {
		#[track_caller]
		fn test(initial: &str, beg: Option<i64>, end: Option<i64>, expected: &str) {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 596: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 596)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn string_replace() {
		#[track_caller]
		fn test(base: &str, pattern: Value, replacement: &str, expected: &str) {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 614: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 614)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn string_matches() {
		#[track_caller]
		fn test(base: &str, regex: &str, expected: bool) {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 633: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 633)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn html_encode() {
		let value = super::html::encode((String::from("<div>Hello world!</div>"),)).unwrap();
		assert_eq!(value, Value::Strand("&lt;div&gt;Hello&#32;world!&lt;&#47;div&gt;".into()));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 642: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 642)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn html_sanitize() {
		let value = super::html::sanitize((String::from("<div>Hello world!</div>"),)).unwrap();
		assert_eq!(value, Value::Strand("<div>Hello world!</div>".into()));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 651: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 651)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn semver_compare() {
		let value = super::semver::compare((String::from("1.2.3"), String::from("1.0.0"))).unwrap();
		assert_eq!(value, Value::from(1));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 663: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 663)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn semver_extract() {
		let value = super::semver::major((String::from("1.2.3"),)).unwrap();
		assert_eq!(value, Value::from(1));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 675: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 675)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn semver_increment() {
		let value = super::semver::inc::major((String::from("1.2.3"),)).unwrap();
		assert_eq!(value, Value::from("2.0.0"));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 687: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 687)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn semver_set() {
		let value = super::semver::set::major((String::from("1.2.3"), 9)).unwrap();
		assert_eq!(value, Value::from("9.2.3"));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `ipv4()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 335)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
	}

	pub fn ipv4((arg,): (String,)) -> Result<Value> {
		Ok(arg.parse::<Ipv4Addr>().is_ok().into())
	}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `words()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 186)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub fn words((string,): (String,)) -> Result<Value> {
	Ok(string
		.split_whitespace()
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `numeric()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 351)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
	}

	pub fn numeric((arg,): (String,)) -> Result<Value> {
		if arg.is_empty() {
			Ok(Value::Bool(false))
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `fuzzy()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 410)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
	use strsim;

	pub fn fuzzy(arg: (String, String)) -> Result<Value> {
		smithwaterman(arg)
	}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `hexadecimal()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 323)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
	}

	pub fn hexadecimal((arg,): (String,)) -> Result<Value> {
		if arg.is_empty() {
			Ok(Value::Bool(false))
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `alphanum()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 284)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
	#[rustfmt::skip] static LONGITUDE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("^[-+]?(180(\\.0+)?|((1[0-7]\\d)|([1-9]?\\d))(\\.\\d+)?)$").unwrap());

	pub fn alphanum((arg,): (String,)) -> Result<Value> {
		if arg.is_empty() {
			Ok(Value::Bool(false))
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `email()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 319)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
	}

	pub fn email((arg,): (String,)) -> Result<Value> {
		Ok(addr::parse_email_address(arg.as_str()).is_ok().into())
	}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `latitude()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 343)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
	}

	pub fn latitude((arg,): (String,)) -> Result<Value> {
		Ok(LATITUDE_RE.is_match(arg.as_str()).into())
	}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `alpha()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 292)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
	}

	pub fn alpha((arg,): (String,)) -> Result<Value> {
		if arg.is_empty() {
			Ok(Value::Bool(false))
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `longitude()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 347)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
	}

	pub fn longitude((arg,): (String,)) -> Result<Value> {
		Ok(LONGITUDE_RE.is_match(arg.as_str()).into())
	}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `ipv6()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 339)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
	}

	pub fn ipv6((arg,): (String,)) -> Result<Value> {
		Ok(arg.parse::<Ipv6Addr>().is_ok().into())
	}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `semver()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/string.rs` (line 359)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
	}

	pub fn semver((arg,): (String,)) -> Result<Value> {
		Ok(Version::parse(arg.as_str()).is_ok().into())
	}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym