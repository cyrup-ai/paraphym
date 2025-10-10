# `forks/surrealdb/crates/core/src/val/value/into_json.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: 97e8aaed  
- **Timestamp**: 2025-10-10T02:16:00.676072+00:00  
- **Lines of Code**: 395

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 395 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 10
  - TODO
  - 

```rust
	/// Converts the value into a json representation of the value.
	/// Returns None if there are non serializable values present in the value.
	// TODO: Remove the JsonValue intermediate and implement a json formatter for
	// Value.
	pub fn into_json_value(self) -> Option<JsonValue> {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 66
  - TODO
  - 

```rust
			}
			Value::RecordId(thing) => JsonValue::String(thing.to_string()),
			// TODO: Maybe remove
			Value::Regex(regex) => JsonValue::String(regex.0.to_string()),
			Value::File(file) => JsonValue::String(file.to_string()),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 70
  - TODO
  - 

```rust
			Value::File(file) => JsonValue::String(file.to_string()),
			// This kind of breaks the behaviour
			// TODO: look at the serialization here.
			Value::Range(range) => JsonValue::String(range.to_string()),
			// These Value types are un-computed values
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 420: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		#[case] expected_deserialized: Value,
	) {
		let json_value = value.into_json_value().unwrap();
		assert_eq!(json_value, expected);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 424: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

		let json_str = serde_json::to_string(&json_value).expect("Failed to serialize to JSON");
		let deserialized_sql_value = crate::syn::value_legacy_strand(&json_str).unwrap();
		let deserialized: Value = deserialized_sql_value;
		assert_eq!(deserialized, expected_deserialized);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 423: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
		assert_eq!(json_value, expected);

		let json_str = serde_json::to_string(&json_value).expect("Failed to serialize to JSON");
		let deserialized_sql_value = crate::syn::value_legacy_strand(&json_str).unwrap();
		let deserialized: Value = deserialized_sql_value;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 147: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/into_json.rs` (line 147)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
	use std::collections::BTreeMap;
	use std::time::Duration;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 415: `#[rstest]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/into_json.rs` (line 415)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
	)]

	fn test_json(
		#[case] value: Value,
		#[case] expected: Json,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `point_into_json_value()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/into_json.rs` (line 127)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

fn point_into_json_value(point: Point) -> JsonValue {
	vec![JsonValue::from(point.x()), JsonValue::from(point.y())].into()
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `line_into_json_value()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/into_json.rs` (line 131)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

fn line_into_json_value(line_string: LineString) -> JsonValue {
	line_string.points().map(point_into_json_value).collect::<Vec<_>>().into()
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `polygon_into_json_value()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/into_json.rs` (line 135)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

fn polygon_into_json_value(polygon: Polygon) -> JsonValue {
	let mut coords =
		vec![polygon.exterior().points().map(point_into_json_value).collect::<Vec<_>>()];
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym