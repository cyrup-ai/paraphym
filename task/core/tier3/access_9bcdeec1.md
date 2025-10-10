# `forks/surrealdb/crates/core/src/expr/statements/access.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: 9bcdeec1  
- **Timestamp**: 2025-10-10T02:16:00.656637+00:00  
- **Lines of Code**: 896

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 896 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Panic-Prone Code


### Line 235: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
pub fn access_object_from_grant(grant: &catalog::AccessGrant) -> Object {
	let mut res = Object::default();
	res.insert("id".to_owned(), Value::from(Strand::new(grant.id.clone()).unwrap()));
	res.insert("ac".to_owned(), Value::from(Strand::new(grant.ac.clone()).unwrap()));
	res.insert("type".to_owned(), Value::from(grant.grant.variant()));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 236: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	let mut res = Object::default();
	res.insert("id".to_owned(), Value::from(Strand::new(grant.id.clone()).unwrap()));
	res.insert("ac".to_owned(), Value::from(Strand::new(grant.ac.clone()).unwrap()));
	res.insert("type".to_owned(), Value::from(grant.grant.variant()));
	res.insert("creation".to_owned(), Value::from(grant.creation.clone()));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 305: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
				ac: access.to_string(),
				// The namespace is expected above so this unwrap should not be able to trigger
				ns: opt.ns.as_deref().unwrap().to_owned(),
			})?
		}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 314: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
				// The namespace and database is expected above so these unwraps should not be able
				// to trigger
				ns: opt.ns.as_deref().unwrap().to_owned(),
				db: opt.db.as_deref().unwrap().to_owned(),
			})?
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 315: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
				// to trigger
				ns: opt.ns.as_deref().unwrap().to_owned(),
				db: opt.db.as_deref().unwrap().to_owned(),
			})?
		}
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
									// We just retrieved the ns_id above so we should have a
									// namespace.
									ns: opt.ns().unwrap().to_owned(),
								}
							})?
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 435: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
									// We just retrieved the ns_id and db_id above so we should have
									// a namespace and database.
									ns: opt.ns().unwrap().to_owned(),
									db: opt.db().unwrap().to_owned(),
								}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 436: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
									// a namespace and database.
									ns: opt.ns().unwrap().to_owned(),
									db: opt.db().unwrap().to_owned(),
								}
							})?
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym