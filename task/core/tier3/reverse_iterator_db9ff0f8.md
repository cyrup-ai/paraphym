# `forks/surrealdb/crates/core/src/kvs/tests/reverse_iterator.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: db9ff0f8  
- **Timestamp**: 2025-10-10T02:16:00.696762+00:00  
- **Lines of Code**: 267

---## Panic-Prone Code


### Line 14: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
async fn test(new_ds: impl CreateDs, index: &str) -> Vec<Response> {
	// Create a new datastore
	let node_id = Uuid::parse_str("056804f2-b379-4397-9ceb-af8ebd527beb").unwrap();
	let clock = Arc::new(SizedClock::Fake(FakeClock::new(Timestamp::default())));
	let (ds, _) = new_ds.create_ds(node_id, clock).await;
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
	);

	let mut r = ds.execute(&sql, &Session::owner(), None).await.unwrap();
	assert_eq!(r.len(), 10);
	// Check the first statements are successful
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 35: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	// Check the first statements are successful
	for _ in 0..4 {
		r.remove(0).result.unwrap();
	}
	r
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 41: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

fn check(r: &mut Vec<Response>, tmp: &str) {
	let tmp = syn::value(tmp).unwrap();
	let val = match r.remove(0).result {
		Ok(v) => v,
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
		]",
	);
	check_array_is_sorted(&r.remove(0).result.unwrap(), 3);
	check(
		r,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 114: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		]",
	);
	check_array_is_sorted(&r.remove(0).result.unwrap(), 3);
	check(
		r,
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
		]",
	);
	check_array_is_sorted(&r.remove(0).result.unwrap(), 1500);
}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 171: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		]",
	);
	check_array_is_sorted(&r.remove(0).result.unwrap(), 3);
	check(
		r,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 194: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		]",
	);
	check_array_is_sorted(&r.remove(0).result.unwrap(), 3);
	check(
		r,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 216: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		]",
	);
	check_array_is_sorted(&r.remove(0).result.unwrap(), 1500);
}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 221: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
pub async fn range(new_ds: impl CreateDs) {
	// Create a new datastore
	let node_id = Uuid::parse_str("056804f2-b379-4397-9ceb-af8ebd527beb").unwrap();
	let clock = Arc::new(SizedClock::Fake(FakeClock::new(Timestamp::default())));
	let (ds, _) = new_ds.create_ds(node_id, clock).await;
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
		SELECT * FROM t:[500]..=[550] ORDER BY id DESC LIMIT 3 EXPLAIN;
	";
	let mut r = ds.execute(sql, &Session::owner(), None).await.unwrap();
	//Check the result
	for _ in 0..3 {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Orphaned Methods


### `standard()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/kvs/tests/reverse_iterator.rs` (line 59)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn standard(new_ds: impl CreateDs) {
	let r = &mut (test(new_ds, "DEFINE INDEX idx ON TABLE i COLUMNS v").await);
	check(
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `unique()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/kvs/tests/reverse_iterator.rs` (line 139)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn unique(new_ds: impl CreateDs) {
	let r = &mut (test(new_ds, "DEFINE INDEX idx ON TABLE i COLUMNS v UNIQUE").await);
	check(
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym