# `forks/surrealdb/crates/core/src/dbs/options.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: 1975f8e2  
- **Timestamp**: 2025-10-10T02:16:00.674221+00:00  
- **Lines of Code**: 333

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 333 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Panic-Prone Code


### Line 453: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

			// When a root resource is targeted, it succeeds
			opts.is_allowed(Action::View, ResourceKind::Any, &Base::Root).unwrap();
			// When a NS resource is targeted and NS was provided, it succeeds
			opts.clone()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 458: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
				.with_ns(Some("ns".into()))
				.is_allowed(Action::View, ResourceKind::Any, &Base::Ns)
				.unwrap();
			// When a DB resource is targeted and NS and DB was provided, it succeeds
			opts.clone()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 464: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
				.with_db(Some("db".into()))
				.is_allowed(Action::View, ResourceKind::Any, &Base::Db)
				.unwrap();
		}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 483: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

			// When a root resource is targeted, it succeeds
			opts.is_allowed(Action::View, ResourceKind::Any, &Base::Root).unwrap();
			// When a NS resource is targeted and NS was provided, it succeeds
			opts.clone()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 488: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
				.with_ns(Some("ns".into()))
				.is_allowed(Action::View, ResourceKind::Any, &Base::Ns)
				.unwrap();
			// When a DB resource is targeted and NS and DB was provided, it succeeds
			opts.clone()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 494: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
				.with_db(Some("db".into()))
				.is_allowed(Action::View, ResourceKind::Any, &Base::Db)
				.unwrap();
		}
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


### Line 432: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/dbs/options.rs` (line 432)
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
  


### Line 438: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/dbs/options.rs` (line 438)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test]
	fn is_allowed() {
		// With auth disabled
		{
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym