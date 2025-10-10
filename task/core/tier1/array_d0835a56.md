# `forks/surrealdb/crates/core/src/fnc/array.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: d0835a56  
- **Timestamp**: 2025-10-10T02:16:00.663191+00:00  
- **Lines of Code**: 728

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 728 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 59
  - TODO
  - 

```rust
			if let Some(opt) = opt {
				for arg in array.into_iter() {
					// TODO: Don't clone the closure every time the function is called.
					if closure.compute(stk, ctx, opt, doc, vec![arg]).await?.is_truthy() {
						continue;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 84
  - TODO
  - 

```rust
			if let Some(opt) = opt {
				for arg in array.into_iter() {
					// TODO: Don't clone the closure every time the function is called.
					if closure.compute(stk, ctx, opt, doc, vec![arg]).await?.is_truthy() {
						return Ok(Value::Bool(true));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 307
  - TODO
  - 

```rust
			if let Some(opt) = opt {
				for (i, arg) in array.into_iter().enumerate() {
					// TODO: Don't clone the closure every time the function is called.
					if closure.compute(stk, ctx, opt, doc, vec![arg]).await?.is_truthy() {
						return Ok(i.into());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 350
  - TODO
  - 

```rust
		let mut accum = init;
		for (i, val) in array.into_iter().enumerate() {
			// TODO: Don't clone the closure every time the function is called.
			accum = mapper.compute(stk, ctx, opt, doc, vec![accum, val, i.into()]).await?
		}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 489
  - TODO
  - 

```rust
		let mut res = Vec::with_capacity(array.len());
		for (i, arg) in array.into_iter().enumerate() {
			// TODO: Don't clone the closure every time the function is called.
			res.push(mapper.compute(stk, ctx, opt, doc, vec![arg, i.into()]).await?);
		}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 592
  - TODO
  - 

```rust

pub fn repeat((value, count): (Value, i64)) -> Result<Value> {
	// TODO: Fix signed to unsigned casting here.
	let count = count as usize;
	limit("array::repeat", size_of_val(&value).saturating_mul(count))?;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tests in Source Directory


### Line 750: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 750)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
	use super::{at, first, join, last, slice};
	use crate::fnc::args::Optional;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 756: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 756)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn array_slice() {
		fn test(initial: &[u8], beg: Option<i64>, lim: Option<i64>, expected: &[u8]) {
			let initial_values =
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 779: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 779)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn array_join() {
		fn test(arr: Array, sep: &str, expected: &str) {
			assert_eq!(join((arr, sep.to_string())).unwrap(), Value::from(expected));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 801: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 801)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn array_first() {
		fn test(arr: Array, expected: Value) {
			assert_eq!(first((arr,)).unwrap(), expected);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 811: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 811)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn array_last() {
		fn test(arr: Array, expected: Value) {
			assert_eq!(last((arr,)).unwrap(), expected);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 821: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 821)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn array_at() {
		fn test(arr: Array, i: i64, expected: Value) {
			assert_eq!(at((arr, i)).unwrap(), expected);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `boolean_and()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 114)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub fn boolean_and((lh, rh): (Array, Array)) -> Result<Value> {
	let longest_length = lh.len().max(rh.len());
	let mut results = Array::with_capacity(longest_length);
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `boolean_not()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 126)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub fn boolean_not((mut array,): (Array,)) -> Result<Value> {
	array.iter_mut().for_each(|v| *v = (!v.is_truthy()).into());
	Ok(array.into())
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `prepend()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 514)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub fn prepend((mut array, value): (Array, Value)) -> Result<Value> {
	array.insert(0, value);
	Ok(array.into())
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `distinct()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 187)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub fn distinct((array,): (Array,)) -> Result<Value> {
	Ok(array.uniq().into())
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `logical_xor()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 458)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub fn logical_xor((lh, rh): (Array, Array)) -> Result<Value> {
	let mut result_arr = Array::with_capacity(lh.len().max(rh.len()));
	let mut iters = (lh.into_iter(), rh.into_iter());
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `boolean_or()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 131)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub fn boolean_or((lh, rh): (Array, Array)) -> Result<Value> {
	let longest_length = lh.len().max(rh.len());
	let mut results = Array::with_capacity(longest_length);
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `sort_natural()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 654)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub fn sort_natural((mut array, Optional(order)): (Array, Optional<Value>)) -> Result<Value> {
	if sort_as_asc(&order) {
		array.sort_unstable_by(|a, b| a.natural_cmp(b).unwrap_or(Ordering::Equal));
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `sort_lexical()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 664)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub fn sort_lexical((mut array, Optional(order)): (Array, Optional<Value>)) -> Result<Value> {
	if sort_as_asc(&order) {
		array.sort_unstable_by(|a, b| a.lexical_cmp(b).unwrap_or(Ordering::Equal));
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `filter_index()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 245)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn filter_index(
	(stk, ctx, opt, doc): (&mut Stk, &Context, Option<&Options>, Option<&CursorDoc>),
	(array, value): (Array, Value),
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `logical_or()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 434)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub fn logical_or((lh, rh): (Array, Array)) -> Result<Value> {
	let mut result_arr = Array::with_capacity(lh.len().max(rh.len()));
	let mut iters = (lh.into_iter(), rh.into_iter());
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `sort_natural_lexical()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 674)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub fn sort_natural_lexical(
	(mut array, Optional(order)): (Array, Optional<Value>),
) -> Result<Value> {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `find_index()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 299)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn find_index(
	(stk, ctx, opt, doc): (&mut Stk, &Context, Option<&Options>, Option<&CursorDoc>),
	(array, value): (Array, Value),
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `asc()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 738)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
	use crate::val::{Array, Value};

	pub fn asc((mut array,): (Array,)) -> Result<Value> {
		array.sort_unstable();
		Ok(array.into())
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `boolean_xor()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 143)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub fn boolean_xor((lh, rh): (Array, Array)) -> Result<Value> {
	let longest_length = lh.len().max(rh.len());
	let mut results = Array::with_capacity(longest_length);
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `logical_and()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 410)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub fn logical_and((lh, rh): (Array, Array)) -> Result<Value> {
	let mut result_arr = Array::with_capacity(lh.len().max(rh.len()));
	let mut iters = (lh.into_iter(), rh.into_iter());
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `desc()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/array.rs` (line 743)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
	}

	pub fn desc((mut array,): (Array,)) -> Result<Value> {
		array.sort_unstable_by(|a, b| b.cmp(a));
		Ok(array.into())
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym