# `forks/surrealdb/crates/core/src/val/value/compare.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: 08966dd2  
- **Timestamp**: 2025-10-10T02:16:00.690570+00:00  
- **Lines of Code**: 182

---## Tier 1 Infractions 


- Line 55
  - TODO
  - 

```rust
						(_, _) => Some(Ordering::Equal),
					},
					//TODO: It is kind of weird that a[1] works but `a[+(1)]` or `let $b = 1;
					// a[$b]` for example doesn't as
					x => {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 101: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn compare_none() {
		let idi: Idiom = Default::default();
		let one = syn::value("{ test: { other: null, something: 456 } }").unwrap();
		let two = syn::value("{ test: { other: null, something: 123 } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 102: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let idi: Idiom = Default::default();
		let one = syn::value("{ test: { other: null, something: 456 } }").unwrap();
		let two = syn::value("{ test: { other: null, something: 123 } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
		assert_eq!(res, Some(Ordering::Greater));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 109: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn compare_basic() {
		let idi: Idiom = syn::idiom("test.something").unwrap().into();
		let one = syn::value("{ test: { other: null, something: 456 } }").unwrap();
		let two = syn::value("{ test: { other: null, something: 123 } }").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 110: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn compare_basic() {
		let idi: Idiom = syn::idiom("test.something").unwrap().into();
		let one = syn::value("{ test: { other: null, something: 456 } }").unwrap();
		let two = syn::value("{ test: { other: null, something: 123 } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 111: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let idi: Idiom = syn::idiom("test.something").unwrap().into();
		let one = syn::value("{ test: { other: null, something: 456 } }").unwrap();
		let two = syn::value("{ test: { other: null, something: 123 } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
		assert_eq!(res, Some(Ordering::Greater));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 118: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn compare_basic_missing_left() {
		let idi: Idiom = syn::idiom("test.something").unwrap().into();
		let one = syn::value("{ test: { other: null } }").unwrap();
		let two = syn::value("{ test: { other: null, something: 123 } }").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 119: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn compare_basic_missing_left() {
		let idi: Idiom = syn::idiom("test.something").unwrap().into();
		let one = syn::value("{ test: { other: null } }").unwrap();
		let two = syn::value("{ test: { other: null, something: 123 } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 120: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let idi: Idiom = syn::idiom("test.something").unwrap().into();
		let one = syn::value("{ test: { other: null } }").unwrap();
		let two = syn::value("{ test: { other: null, something: 123 } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
		assert_eq!(res, Some(Ordering::Less));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 127: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn compare_basic_missing_right() {
		let idi: Idiom = syn::idiom("test.something").unwrap().into();
		let one = syn::value("{ test: { other: null, something: 456 } }").unwrap();
		let two = syn::value("{ test: { other: null } }").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 128: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn compare_basic_missing_right() {
		let idi: Idiom = syn::idiom("test.something").unwrap().into();
		let one = syn::value("{ test: { other: null, something: 456 } }").unwrap();
		let two = syn::value("{ test: { other: null } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 129: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let idi: Idiom = syn::idiom("test.something").unwrap().into();
		let one = syn::value("{ test: { other: null, something: 456 } }").unwrap();
		let two = syn::value("{ test: { other: null } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
		assert_eq!(res, Some(Ordering::Greater));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 136: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn compare_array() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [4, 5, 6] } }").unwrap();
		let two = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 137: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn compare_array() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [4, 5, 6] } }").unwrap();
		let two = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
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
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [4, 5, 6] } }").unwrap();
		let two = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
		assert_eq!(res, Some(Ordering::Greater));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 145: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn compare_array_longer_left() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [1, 2, 3, 4, 5, 6] } }").unwrap();
		let two = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 146: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn compare_array_longer_left() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [1, 2, 3, 4, 5, 6] } }").unwrap();
		let two = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 147: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [1, 2, 3, 4, 5, 6] } }").unwrap();
		let two = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
		assert_eq!(res, Some(Ordering::Greater));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 154: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn compare_array_longer_right() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
		let two = syn::value("{ test: { other: null, something: [1, 2, 3, 4, 5, 6] } }").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 155: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn compare_array_longer_right() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
		let two = syn::value("{ test: { other: null, something: [1, 2, 3, 4, 5, 6] } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
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
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
		let two = syn::value("{ test: { other: null, something: [1, 2, 3, 4, 5, 6] } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
		assert_eq!(res, Some(Ordering::Less));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 163: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn compare_array_missing_left() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: null } }").unwrap();
		let two = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 164: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn compare_array_missing_left() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: null } }").unwrap();
		let two = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 165: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: null } }").unwrap();
		let two = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
		assert_eq!(res, Some(Ordering::Less));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 172: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn compare_array_missing_right() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [4, 5, 6] } }").unwrap();
		let two = syn::value("{ test: { other: null, something: null } }").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 173: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn compare_array_missing_right() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [4, 5, 6] } }").unwrap();
		let two = syn::value("{ test: { other: null, something: null } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 174: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [4, 5, 6] } }").unwrap();
		let two = syn::value("{ test: { other: null, something: null } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
		assert_eq!(res, Some(Ordering::Greater));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 181: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn compare_array_missing_value_left() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [1, null, 3] } }").unwrap();
		let two = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
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
	fn compare_array_missing_value_left() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [1, null, 3] } }").unwrap();
		let two = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 183: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [1, null, 3] } }").unwrap();
		let two = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
		assert_eq!(res, Some(Ordering::Less));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 190: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn compare_array_missing_value_right() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
		let two = syn::value("{ test: { other: null, something: [1, null, 3] } }").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 191: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn compare_array_missing_value_right() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
		let two = syn::value("{ test: { other: null, something: [1, null, 3] } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
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
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
		let two = syn::value("{ test: { other: null, something: [1, null, 3] } }").unwrap();
		let res = one.compare(&two, &idi, false, false);
		assert_eq!(res, Some(Ordering::Greater));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 199: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	#[test]
	fn compare_last() {
		let idi: Idiom = syn::idiom("test[$]").unwrap().into();
		let one = syn::value("{ test: [1,5] }").unwrap();
		let two = syn::value("{ test: [2,4] }").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 200: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn compare_last() {
		let idi: Idiom = syn::idiom("test[$]").unwrap().into();
		let one = syn::value("{ test: [1,5] }").unwrap();
		let two = syn::value("{ test: [2,4] }").unwrap();
		let res = one.compare(&two, &idi, false, false);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 201: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let idi: Idiom = syn::idiom("test[$]").unwrap().into();
		let one = syn::value("{ test: [1,5] }").unwrap();
		let two = syn::value("{ test: [2,4] }").unwrap();
		let res = one.compare(&two, &idi, false, false);
		assert_eq!(res, Some(Ordering::Greater))
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 92: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/compare.rs` (line 92)
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
  


### Line 99: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/compare.rs` (line 99)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn compare_none() {
		let idi: Idiom = Default::default();
		let one = syn::value("{ test: { other: null, something: 456 } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 108: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/compare.rs` (line 108)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn compare_basic() {
		let idi: Idiom = syn::idiom("test.something").unwrap().into();
		let one = syn::value("{ test: { other: null, something: 456 } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 117: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/compare.rs` (line 117)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn compare_basic_missing_left() {
		let idi: Idiom = syn::idiom("test.something").unwrap().into();
		let one = syn::value("{ test: { other: null } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 126: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/compare.rs` (line 126)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn compare_basic_missing_right() {
		let idi: Idiom = syn::idiom("test.something").unwrap().into();
		let one = syn::value("{ test: { other: null, something: 456 } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 135: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/compare.rs` (line 135)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn compare_array() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [4, 5, 6] } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 144: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/compare.rs` (line 144)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn compare_array_longer_left() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [1, 2, 3, 4, 5, 6] } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 153: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/compare.rs` (line 153)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn compare_array_longer_right() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 162: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/compare.rs` (line 162)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn compare_array_missing_left() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: null } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 171: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/compare.rs` (line 171)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn compare_array_missing_right() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [4, 5, 6] } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 180: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/compare.rs` (line 180)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn compare_array_missing_value_left() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [1, null, 3] } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 189: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/compare.rs` (line 189)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn compare_array_missing_value_right() {
		let idi: Idiom = syn::idiom("test.something.*").unwrap().into();
		let one = syn::value("{ test: { other: null, something: [1, 2, 3] } }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 198: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/val/value/compare.rs` (line 198)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn compare_last() {
		let idi: Idiom = syn::idiom("test[$]").unwrap().into();
		let one = syn::value("{ test: [1,5] }").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym