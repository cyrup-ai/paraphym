# `forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: sdk
- **File Hash**: 456c1d4b  
- **Timestamp**: 2025-10-10T02:16:00.938875+00:00  
- **Lines of Code**: 320

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 320 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 74
  - TODO
  - 

```rust
		},
		Content::Char(v) => Ok(v.to_string().into()),
		// TODO: Null byte validity
		Content::String(v) => Ok(unsafe { val::Strand::new_unchecked(v.into_owned()) }.into()),
		Content::Bytes(v) => Ok(val::Bytes::from(v.into_owned()).into()),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 155: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn number() {
		let number = val::Number::Int(Default::default());
		let value = to_value(number).unwrap();
		let expected = val::Value::Number(number);
		assert_eq!(value, expected);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 161: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

		let number = val::Number::Float(Default::default());
		let value = to_value(number).unwrap();
		let expected = val::Value::Number(number);
		assert_eq!(value, expected);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 167: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

		let number = val::Number::Decimal(Default::default());
		let value = to_value(number).unwrap();
		let expected = val::Value::Number(number);
		assert_eq!(value, expected);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 175: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn strand() {
		let strand = val::Strand::new("foobar".to_owned()).unwrap();
		let value = to_value(strand.clone()).unwrap();
		let expected = val::Value::Strand(strand);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 176: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn strand() {
		let strand = val::Strand::new("foobar".to_owned()).unwrap();
		let value = to_value(strand.clone()).unwrap();
		let expected = val::Value::Strand(strand);
		assert_eq!(value, expected);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 182: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

		let strand = "foobar".to_owned();
		let value = to_value(strand.clone()).unwrap();
		let expected = val::Value::Strand(strand.into());
		assert_eq!(value, expected);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 188: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

		let strand = "foobar";
		let value = to_value(strand).unwrap();
		let expected = val::Value::Strand(strand.into());
		assert_eq!(value, expected);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 197: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn duration() {
		let duration = val::Duration::default();
		let value = to_value(duration).unwrap();
		let expected = val::Value::Duration(duration);
		assert_eq!(value, expected);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 206: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn datetime() {
		let datetime = val::Datetime::now();
		let value = to_value(datetime.clone()).unwrap();
		let expected = val::Value::Datetime(datetime);
		assert_eq!(value, expected);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 215: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn uuid() {
		let uuid = val::Uuid::default();
		let value = to_value(uuid).unwrap();
		let expected = val::Value::Uuid(uuid);
		assert_eq!(value, expected);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 224: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn array() {
		let array = val::Array::default();
		let value = to_value(array.clone()).unwrap();
		let expected = val::Value::Array(array);
		assert_eq!(value, expected);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 233: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn object() {
		let object = val::Object::default();
		let value = to_value(object.clone()).unwrap();
		let expected = val::Value::Object(object);
		assert_eq!(value, expected);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 242: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn geometry() {
		let geometry = val::Geometry::Collection(Vec::new());
		let value = to_value(geometry.clone()).unwrap();
		let expected = val::Value::Geometry(geometry);
		assert_eq!(value, expected);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 251: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn bytes() {
		let bytes = val::Bytes::from("foobar".as_bytes().to_owned());
		let value = to_value(bytes.clone()).unwrap();
		let expected = val::Value::Bytes(bytes);
		assert_eq!(value, expected);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 259: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn table() {
		let table = val::Table::new("foo".to_owned()).unwrap();
		let value = to_value(table.clone()).unwrap();
		let expected = val::Value::Table(table);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 260: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn table() {
		let table = val::Table::new("foo".to_owned()).unwrap();
		let value = to_value(table.clone()).unwrap();
		let expected = val::Value::Table(table);
		assert_eq!(value, expected);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 268: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn thing() {
		let record_id = syn::record_id("foo:bar").unwrap();
		let value = to_value(record_id.clone()).unwrap();
		let expected = val::Value::RecordId(record_id);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 269: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn thing() {
		let record_id = syn::record_id("foo:bar").unwrap();
		let value = to_value(record_id.clone()).unwrap();
		let expected = val::Value::RecordId(record_id);
		assert_eq!(value, expected);
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
	#[test]
	fn regex() {
		let regex = "abc".parse::<Regex>().unwrap();
		let value = to_value(regex.clone()).unwrap();
		let expected = val::Value::Regex(regex);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 278: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn regex() {
		let regex = "abc".parse::<Regex>().unwrap();
		let value = to_value(regex.clone()).unwrap();
		let expected = val::Value::Regex(regex);
		assert_eq!(value, expected);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 287: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn range() {
		let range = val::Range {
			start: Bound::Included(val::Strand::new("foo".to_owned()).unwrap().into()),
			end: Bound::Unbounded,
		};
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 290: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			end: Bound::Unbounded,
		};
		let value = to_value(range.clone()).unwrap();
		let expected = val::Value::Range(Box::new(range));
		assert_eq!(value, expected);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 310: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			foo: foo.to_owned(),
		};
		let value = to_value(foo_bar).unwrap();
		let expected = val::Value::Object(
			map! {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 325: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn none() {
		let option: Option<val::Value> = None;
		let serialized = to_value(option).unwrap();
		assert_eq!(val::Value::None, serialized);
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 332: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn some() {
		let option = Some(val::Value::Bool(true));
		let serialized = to_value(option).unwrap();
		assert_eq!(val::Value::Bool(true), serialized);
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 339: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn empty_map() {
		let map: BTreeMap<String, val::Value> = Default::default();
		let serialized = to_value(map).unwrap();
		assert_eq!(val::Value::Object(Default::default()), serialized);
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 348: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			String::from("foo") => val::Value::from("bar"),
		};
		let serialized = to_value(map.clone()).unwrap();
		assert_eq!(serialized, val::Value::from(map));
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 355: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn empty_vec() {
		let vec: Vec<val::Value> = Vec::new();
		let serialized = to_value(vec).unwrap();
		assert_eq!(val::Value::Array(Default::default()), serialized);
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 362: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn vec() {
		let vec = vec![val::Value::default()];
		let serialized = to_value(vec).unwrap();
		assert_eq!(val::Value::Array(vec![val::Value::None].into()), serialized);
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


### Line 115: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 115)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
	use std::collections::BTreeMap;
	use std::ops::Bound;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 126: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 126)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn value_none() {
		let expected = val::Value::None;
		assert_eq!(expected, to_value(None::<u32>).unwrap());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 133: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 133)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn null() {
		let expected = val::Value::Null;
		assert_eq!(expected.clone(), to_value(expected).unwrap());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 139: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 139)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn r#false() {
		let expected = val::Value::Bool(false);
		assert_eq!(expected, to_value(false).unwrap());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 146: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 146)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn r#true() {
		let expected = val::Value::Bool(true);
		assert_eq!(expected, to_value(true).unwrap());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 153: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 153)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn number() {
		let number = val::Number::Int(Default::default());
		let value = to_value(number).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 174: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 174)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn strand() {
		let strand = val::Strand::new("foobar".to_owned()).unwrap();
		let value = to_value(strand.clone()).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 195: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 195)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn duration() {
		let duration = val::Duration::default();
		let value = to_value(duration).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 204: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 204)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn datetime() {
		let datetime = val::Datetime::now();
		let value = to_value(datetime.clone()).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 213: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 213)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn uuid() {
		let uuid = val::Uuid::default();
		let value = to_value(uuid).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 222: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 222)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn array() {
		let array = val::Array::default();
		let value = to_value(array.clone()).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 231: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 231)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn object() {
		let object = val::Object::default();
		let value = to_value(object.clone()).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 240: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 240)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn geometry() {
		let geometry = val::Geometry::Collection(Vec::new());
		let value = to_value(geometry.clone()).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 249: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 249)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn bytes() {
		let bytes = val::Bytes::from("foobar".as_bytes().to_owned());
		let value = to_value(bytes.clone()).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 258: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 258)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn table() {
		let table = val::Table::new("foo".to_owned()).unwrap();
		let value = to_value(table.clone()).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 267: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 267)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn thing() {
		let record_id = syn::record_id("foo:bar").unwrap();
		let value = to_value(record_id.clone()).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 276: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 276)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn regex() {
		let regex = "abc".parse::<Regex>().unwrap();
		let value = to_value(regex.clone()).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 285: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 285)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn range() {
		let range = val::Range {
			start: Bound::Included(val::Strand::new("foo".to_owned()).unwrap().into()),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 297: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 297)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn unknown_struct() {
		#[derive(Debug, Serialize)]
		struct FooBar {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 323: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 323)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn none() {
		let option: Option<val::Value> = None;
		let serialized = to_value(option).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 330: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 330)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn some() {
		let option = Some(val::Value::Bool(true));
		let serialized = to_value(option).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 337: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 337)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn empty_map() {
		let map: BTreeMap<String, val::Value> = Default::default();
		let serialized = to_value(map).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 344: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 344)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn map() {
		let map = map! {
			String::from("foo") => val::Value::from("bar"),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 353: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 353)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn empty_vec() {
		let vec: Vec<val::Value> = Vec::new();
		let serialized = to_value(vec).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 360: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/value/convert/ser/mod.rs` (line 360)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn vec() {
		let vec = vec![val::Value::default()];
		let serialized = to_value(vec).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym