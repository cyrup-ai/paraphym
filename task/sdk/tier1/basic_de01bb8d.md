# `forks/surrealdb/crates/sdk/tests/api_integration/basic.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: sdk
- **File Hash**: de01bb8d  
- **Timestamp**: 2025-10-10T02:16:00.929615+00:00  
- **Lines of Code**: 1708

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 1708 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 207
  - hardcoded URL
  - 

```rust
			surrealdb::error::Api::Http(e) => assert_eq!(
				e,
				"HTTP status client error (400 Bad Request) for url (http://127.0.0.1:8000/signup)"
			),
			x => panic!("unexpected error: {x:?}"),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 238
  - hardcoded URL
  - 

```rust
			surrealdb::error::Api::Http(e) => assert_eq!(
				e,
				"HTTP status client error (400 Bad Request) for url (http://127.0.0.1:8000/signup)"
			),
			x => panic!("unexpected error: {x:?}"),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 292
  - hardcoded URL
  - 

```rust
			surrealdb::error::Api::Http(e) => assert_eq!(
				e,
				"HTTP status client error (400 Bad Request) for url (http://127.0.0.1:8000/signup)"
			),
			x => panic!("unexpected error: {x:?}"),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 326
  - hardcoded URL
  - 

```rust
			surrealdb::error::Api::Http(e) => assert_eq!(
				e,
				"HTTP status client error (400 Bad Request) for url (http://127.0.0.1:8000/signin)"
			),
			x => panic!("unexpected error: {x:?}"),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 23: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let (permit, db) = new_db.create_db().await;
	drop(permit);
	db.health().await.unwrap();
}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 33: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		panic!("{:?}", err)
	}
	db.use_ns(NS).await.unwrap();
	let err = db.create(Resource::from(item.as_str())).await.unwrap_err();
	if !err.to_string().contains("Specify a database to use") {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 38: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		panic!("{:?}", err)
	}
	db.use_db(item.as_str()).await.unwrap();
	db.create(Resource::from(item)).await.unwrap();
	drop(permit);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 39: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	}
	db.use_db(item.as_str()).await.unwrap();
	db.create(Resource::from(item)).await.unwrap();
	drop(permit);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 45: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn invalidate(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	db.invalidate().await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 47: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	db.invalidate().await.unwrap();
	let error = db.create::<Option<ApiRecordId>>(("user", "john")).await.unwrap_err();
	assert!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 59: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let (permit, db) = new_db.create_db().await;
	let database = Ulid::new().to_string();
	db.use_ns(NS).use_db(&database).await.unwrap();
	let access = Ulid::new().to_string();
	let sql = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 69: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    "
	);
	let response = db.query(sql).await.unwrap();
	drop(permit);
	response.check().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 71: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let response = db.query(sql).await.unwrap();
	drop(permit);
	response.check().unwrap();
	db.signup(RecordAccess {
		namespace: NS,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 82: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	})
	.await
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


### Line 87: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn signin_ns(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	let user = Ulid::new().to_string();
	let pass = "password123";
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let pass = "password123";
	let sql = format!("DEFINE USER `{user}` ON NAMESPACE PASSWORD '{pass}'");
	let response = db.query(sql).await.unwrap();
	drop(permit);
	response.check().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 93: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let response = db.query(sql).await.unwrap();
	drop(permit);
	response.check().unwrap();
	db.signin(Namespace {
		namespace: NS,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 100: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	})
	.await
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


### Line 106: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let (permit, db) = new_db.create_db().await;
	let database = Ulid::new().to_string();
	db.use_ns(NS).use_db(&database).await.unwrap();
	let user = Ulid::new().to_string();
	let pass = "password123";
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let pass = "password123";
	let sql = format!("DEFINE USER `{user}` ON DATABASE PASSWORD '{pass}'");
	let response = db.query(sql).await.unwrap();
	drop(permit);
	response.check().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 112: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let response = db.query(sql).await.unwrap();
	drop(permit);
	response.check().unwrap();
	db.signin(Database {
		namespace: NS,
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	})
	.await
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


### Line 126: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let (permit, db) = new_db.create_db().await;
	let database = Ulid::new().to_string();
	db.use_ns(NS).use_db(&database).await.unwrap();
	let access = Ulid::new().to_string();
	let email = format!("{access}@example.com");
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    "
	);
	let response = db.query(sql).await.unwrap();
	drop(permit);
	response.check().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 140: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let response = db.query(sql).await.unwrap();
	drop(permit);
	response.check().unwrap();
	db.signup(RecordAccess {
		namespace: NS,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 151: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	})
	.await
	.unwrap();
	db.signin(RecordAccess {
		namespace: NS,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 162: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	})
	.await
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


### Line 168: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let (permit, db) = new_db.create_db().await;
	let database = Ulid::new().to_string();
	db.use_ns(NS).use_db(&database).await.unwrap();
	let access = Ulid::new().to_string();
	let email = format!("{access}@example.com");
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    "
	);
	let response = db.query(sql).await.unwrap();
	drop(permit);
	response.check().unwrap();
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let response = db.query(sql).await.unwrap();
	drop(permit);
	response.check().unwrap();

	let err = db
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 250: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let (permit, db) = new_db.create_db().await;
	let database = Ulid::new().to_string();
	db.use_ns(NS).use_db(&database).await.unwrap();
	let access = Ulid::new().to_string();
	let email = format!("{access}@example.com");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 262: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    "
	);
	let response = db.query(sql).await.unwrap();
	drop(permit);
	response.check().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 264: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let response = db.query(sql).await.unwrap();
	drop(permit);
	response.check().unwrap();

	let err = db
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 337: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn authenticate(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	let user = Ulid::new().to_string();
	let pass = "password123";
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 341: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let pass = "password123";
	let sql = format!("DEFINE USER `{user}` ON NAMESPACE PASSWORD '{pass}'");
	let response = db.query(sql).await.unwrap();
	drop(permit);
	response.check().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 343: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let response = db.query(sql).await.unwrap();
	drop(permit);
	response.check().unwrap();
	let token = db
		.signin(Namespace {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 351: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();
	db.authenticate(token).await.unwrap();
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 352: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.await
		.unwrap();
	db.authenticate(token).await.unwrap();
}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 357: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn query(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let _ = db
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 369: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.unwrap()
		.check()
		.unwrap();
	let mut response = db.query("SELECT name FROM user:john").await.unwrap().check().unwrap();
	let Some(name): Option<String> = response.take("name").unwrap() else {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 367: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		)
		.await
		.unwrap()
		.check()
		.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 370: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.check()
		.unwrap();
	let mut response = db.query("SELECT name FROM user:john").await.unwrap().check().unwrap();
	let Some(name): Option<String> = response.take("name").unwrap() else {
		panic!("query returned no record");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 370: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.check()
		.unwrap();
	let mut response = db.query("SELECT name FROM user:john").await.unwrap().check().unwrap();
	let Some(name): Option<String> = response.take("name").unwrap() else {
		panic!("query returned no record");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 371: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.unwrap();
	let mut response = db.query("SELECT name FROM user:john").await.unwrap().check().unwrap();
	let Some(name): Option<String> = response.take("name").unwrap() else {
		panic!("query returned no record");
	};
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 379: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn query_raw(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let _ = db
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 386: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.unwrap()
		.check()
		.unwrap();
	let mut response = db
		.query(Raw::from("SELECT name FROM user:john".to_owned()))
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 384: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.query(Raw::from("CREATE user:john SET name = 'John Doe'"))
		.await
		.unwrap()
		.check()
		.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 392: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.unwrap()
		.check()
		.unwrap();
	let Some(name): Option<String> = response.take("name").unwrap() else {
		panic!("query returned no record");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 390: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.query(Raw::from("SELECT name FROM user:john".to_owned()))
		.await
		.unwrap()
		.check()
		.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 393: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.check()
		.unwrap();
	let Some(name): Option<String> = response.take("name").unwrap() else {
		panic!("query returned no record");
	};
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 401: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn query_decimals(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	let sql = "
	    DEFINE TABLE foo;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 407: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	    CREATE foo CONTENT { bar: 42.69 };
    ";
	let _ = db.query(sql).await.unwrap().check().unwrap();
	drop(permit);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 407: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	    CREATE foo CONTENT { bar: 42.69 };
    ";
	let _ = db.query(sql).await.unwrap().check().unwrap();
	drop(permit);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 413: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn query_binds(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let mut response =
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 416: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	drop(permit);
	let mut response =
		db.query("CREATE user:john SET name = $name").bind(("name", "John Doe")).await.unwrap();
	let Some(record): Option<RecordName> = response.take(0).unwrap() else {
		panic!("query returned no record");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 417: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let mut response =
		db.query("CREATE user:john SET name = $name").bind(("name", "John Doe")).await.unwrap();
	let Some(record): Option<RecordName> = response.take(0).unwrap() else {
		panic!("query returned no record");
	};
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 425: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.bind(("record_id", syn::record_id("user:john").unwrap()))
		.await
		.unwrap();
	let Some(record): Option<RecordName> = response.take(0).unwrap() else {
		panic!("query returned no record");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 423: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let mut response = db
		.query("SELECT * FROM $record_id")
		.bind(("record_id", syn::record_id("user:john").unwrap()))
		.await
		.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 426: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.await
		.unwrap();
	let Some(record): Option<RecordName> = response.take(0).unwrap() else {
		panic!("query returned no record");
	};
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();
	let Some(record): Option<RecordName> = response.take(0).unwrap() else {
		panic!("query returned no record");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 437: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.await
		.unwrap();
	let Some(record): Option<RecordName> = response.take(0).unwrap() else {
		panic!("query returned no record");
	};
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 445: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn query_with_stats(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let sql = "CREATE foo; SELECT * FROM foo";
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 448: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	drop(permit);
	let sql = "CREATE foo; SELECT * FROM foo";
	let mut response = db.query(sql).with_stats().await.unwrap();
	// First query statement
	let (stats, result) = response.take(0).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 450: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let mut response = db.query(sql).with_stats().await.unwrap();
	// First query statement
	let (stats, result) = response.take(0).unwrap();
	assert!(stats.execution_time > Some(Duration::ZERO));
	let _: Value = result.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 452: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let (stats, result) = response.take(0).unwrap();
	assert!(stats.execution_time > Some(Duration::ZERO));
	let _: Value = result.unwrap();
	// Second query statement
	let (stats, result) = response.take(1).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 454: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Value = result.unwrap();
	// Second query statement
	let (stats, result) = response.take(1).unwrap();
	assert!(stats.execution_time > Some(Duration::ZERO));
	let _: Vec<ApiRecordId> = result.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 456: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let (stats, result) = response.take(1).unwrap();
	assert!(stats.execution_time > Some(Duration::ZERO));
	let _: Vec<ApiRecordId> = result.unwrap();
}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 461: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn query_chaining(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let response = db
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 471: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.query(TopLevelExpr::Commit)
		.await
		.unwrap();
	response.check().unwrap();
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 472: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.await
		.unwrap();
	response.check().unwrap();
}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 477: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn mixed_results_query(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let sql = "CREATE bar SET baz = rand('a'); CREATE foo;";
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 480: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	drop(permit);
	let sql = "CREATE bar SET baz = rand('a'); CREATE foo;";
	let mut response = db.query(sql).await.unwrap();
	response.take::<Value>(0).unwrap_err();
	let _: Option<ApiRecordId> = response.take(1).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 482: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let mut response = db.query(sql).await.unwrap();
	response.take::<Value>(0).unwrap_err();
	let _: Option<ApiRecordId> = response.take(1).unwrap();
}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 487: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn create_record_no_id(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let _: Option<ApiRecordId> = db.create("user").await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 489: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let _: Option<ApiRecordId> = db.create("user").await.unwrap();
	let _: Value = db.create(Resource::from("user")).await.unwrap();
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 490: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	drop(permit);
	let _: Option<ApiRecordId> = db.create("user").await.unwrap();
	let _: Value = db.create(Resource::from("user")).await.unwrap();
}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 495: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn create_record_with_id(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let _: Option<ApiRecordId> = db.create(("user", "jane")).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 497: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let _: Option<ApiRecordId> = db.create(("user", "jane")).await.unwrap();
	let _: Value = db.create(Resource::from(("user", "john"))).await.unwrap();
	let _: Value = db.create(Resource::from("user:doe")).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 498: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	drop(permit);
	let _: Option<ApiRecordId> = db.create(("user", "jane")).await.unwrap();
	let _: Value = db.create(Resource::from(("user", "john"))).await.unwrap();
	let _: Value = db.create(Resource::from("user:doe")).await.unwrap();
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 499: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Option<ApiRecordId> = db.create(("user", "jane")).await.unwrap();
	let _: Value = db.create(Resource::from(("user", "john"))).await.unwrap();
	let _: Value = db.create(Resource::from("user:doe")).await.unwrap();
}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 504: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn create_record_no_id_with_content(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let _: Option<ApiRecordId> = db
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 512: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();
	let _: Value = db
		.create(Resource::from("user"))
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 519: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
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


### Line 524: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn create_record_with_id_with_content(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let record: Option<ApiRecordId> = db
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 532: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();
	assert_eq!(record.unwrap().id, "user:john".parse::<RecordId>().unwrap());
	let value: Value = db
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 540: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();
	assert_eq!(
		value.into_inner().record(),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 535: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	assert_eq!(record.unwrap().id, "user:john".parse::<RecordId>().unwrap());
	let value: Value = db
		.create(Resource::from("user:jane".parse::<RecordId>().unwrap()))
		.content(Record {
			name: "Jane Doe".to_owned(),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 562: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 572: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();
	assert_eq!(record.unwrap().id, "user:john".parse::<RecordId>().unwrap());

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 604: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();

	let _: Option<Record> = db
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 614: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
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


### Line 619: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn insert_table(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let table = "user";
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 622: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	drop(permit);
	let table = "user";
	let _: Vec<ApiRecordId> = db.insert(table).await.unwrap();
	let _: Vec<ApiRecordId> = db.insert(table).content(json!({ "foo": "bar" })).await.unwrap();
	let _: Vec<ApiRecordId> = db.insert(table).content(json!([{ "foo": "bar" }])).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 623: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let table = "user";
	let _: Vec<ApiRecordId> = db.insert(table).await.unwrap();
	let _: Vec<ApiRecordId> = db.insert(table).content(json!({ "foo": "bar" })).await.unwrap();
	let _: Vec<ApiRecordId> = db.insert(table).content(json!([{ "foo": "bar" }])).await.unwrap();
	let _: Value = db.insert(Resource::from(table)).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 624: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Vec<ApiRecordId> = db.insert(table).await.unwrap();
	let _: Vec<ApiRecordId> = db.insert(table).content(json!({ "foo": "bar" })).await.unwrap();
	let _: Vec<ApiRecordId> = db.insert(table).content(json!([{ "foo": "bar" }])).await.unwrap();
	let _: Value = db.insert(Resource::from(table)).await.unwrap();
	let _: Value = db.insert(Resource::from(table)).content(json!({ "foo": "bar" })).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 625: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Vec<ApiRecordId> = db.insert(table).content(json!({ "foo": "bar" })).await.unwrap();
	let _: Vec<ApiRecordId> = db.insert(table).content(json!([{ "foo": "bar" }])).await.unwrap();
	let _: Value = db.insert(Resource::from(table)).await.unwrap();
	let _: Value = db.insert(Resource::from(table)).content(json!({ "foo": "bar" })).await.unwrap();
	let _: Value =
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 626: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Vec<ApiRecordId> = db.insert(table).content(json!([{ "foo": "bar" }])).await.unwrap();
	let _: Value = db.insert(Resource::from(table)).await.unwrap();
	let _: Value = db.insert(Resource::from(table)).content(json!({ "foo": "bar" })).await.unwrap();
	let _: Value =
		db.insert(Resource::from(table)).content(json!([{ "foo": "bar" }])).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 628: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Value = db.insert(Resource::from(table)).content(json!({ "foo": "bar" })).await.unwrap();
	let _: Value =
		db.insert(Resource::from(table)).content(json!([{ "foo": "bar" }])).await.unwrap();
	let users: Vec<ApiRecordId> = db.insert(table).await.unwrap();
	assert!(!users.is_empty());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 629: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Value =
		db.insert(Resource::from(table)).content(json!([{ "foo": "bar" }])).await.unwrap();
	let users: Vec<ApiRecordId> = db.insert(table).await.unwrap();
	assert!(!users.is_empty());
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 635: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn insert_thing(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let table = "user";
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 638: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	drop(permit);
	let table = "user";
	let _: Option<ApiRecordId> = db.insert((table, "user1")).await.unwrap();
	let _: Option<ApiRecordId> =
		db.insert((table, "user2")).content(json!({ "foo": "bar" })).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 640: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Option<ApiRecordId> = db.insert((table, "user1")).await.unwrap();
	let _: Option<ApiRecordId> =
		db.insert((table, "user2")).content(json!({ "foo": "bar" })).await.unwrap();
	let _: Value = db.insert(Resource::from((table, "user3"))).await.unwrap();
	let _: Value =
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 641: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Option<ApiRecordId> =
		db.insert((table, "user2")).content(json!({ "foo": "bar" })).await.unwrap();
	let _: Value = db.insert(Resource::from((table, "user3"))).await.unwrap();
	let _: Value =
		db.insert(Resource::from((table, "user4"))).content(json!({ "foo": "bar" })).await.unwrap();
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Value = db.insert(Resource::from((table, "user3"))).await.unwrap();
	let _: Value =
		db.insert(Resource::from((table, "user4"))).content(json!({ "foo": "bar" })).await.unwrap();
	let user: Option<ApiRecordId> = db.insert((table, "user5")).await.unwrap();
	assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 644: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Value =
		db.insert(Resource::from((table, "user4"))).content(json!({ "foo": "bar" })).await.unwrap();
	let user: Option<ApiRecordId> = db.insert((table, "user5")).await.unwrap();
	assert_eq!(
		user,
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn insert_unspecified(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let tmp: Result<Vec<RecordId>, _> = db.insert(()).await;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 665: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.content("{id: user:user1, foo: 'bar'}".parse::<Value>().unwrap())
		.await
		.unwrap();
	assert_eq!(
		tmp,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 663: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let tmp: Vec<ApiRecordId> = db
		.insert(())
		.content("{id: user:user1, foo: 'bar'}".parse::<Value>().unwrap())
		.await
		.unwrap();
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.content("{id: user:user2, foo: 'bar'}".parse::<Value>().unwrap())
		.await
		.unwrap();
	let val = "{id: user:user2, foo: 'bar'}".parse::<Value>().unwrap();
	assert_eq!(tmp, val);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 680: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let tmp: Value = db
		.insert(Resource::from(()))
		.content("{id: user:user2, foo: 'bar'}".parse::<Value>().unwrap())
		.await
		.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 683: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.await
		.unwrap();
	let val = "{id: user:user2, foo: 'bar'}".parse::<Value>().unwrap();
	assert_eq!(tmp, val);
}
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn insert_relation_table(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let tmp: Result<Vec<ApiRecordId>, _> =
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 692: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	drop(permit);
	let tmp: Result<Vec<ApiRecordId>, _> =
		db.insert("likes").relation("{}".parse::<Value>().unwrap()).await;
	tmp.unwrap_err();
	let val = "{in: person:a, out: thing:a}".parse::<Value>().unwrap();
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		db.insert("likes").relation("{}".parse::<Value>().unwrap()).await;
	tmp.unwrap_err();
	let val = "{in: person:a, out: thing:a}".parse::<Value>().unwrap();
	let _: Vec<ApiRecordId> = db.insert("likes").relation(val).await.unwrap();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 695: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	tmp.unwrap_err();
	let val = "{in: person:a, out: thing:a}".parse::<Value>().unwrap();
	let _: Vec<ApiRecordId> = db.insert("likes").relation(val).await.unwrap();

	let vals = r#"[
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	]"#
	.parse::<Value>()
	.unwrap();
	let _: Vec<ApiRecordId> = db.insert("likes").relation(vals).await.unwrap();
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 704: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	.parse::<Value>()
	.unwrap();
	let _: Vec<ApiRecordId> = db.insert("likes").relation(vals).await.unwrap();
}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 709: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn select_table(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let table = "user";
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 712: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	drop(permit);
	let table = "user";
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Value = db.create(Resource::from(table)).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 713: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let table = "user";
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Value = db.create(Resource::from(table)).await.unwrap();
	let users: Vec<ApiRecordId> = db.select(table).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 714: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Value = db.create(Resource::from(table)).await.unwrap();
	let users: Vec<ApiRecordId> = db.select(table).await.unwrap();
	assert_eq!(users.len(), 3);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 715: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Value = db.create(Resource::from(table)).await.unwrap();
	let users: Vec<ApiRecordId> = db.select(table).await.unwrap();
	assert_eq!(users.len(), 3);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 721: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn select_record_id(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let record_id = ("user", "john");
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	drop(permit);
	let record_id = ("user", "john");
	let _: Option<ApiRecordId> = db.create(record_id).await.unwrap();
	let Some(record): Option<ApiRecordId> = db.select(record_id).await.unwrap() else {
		panic!("record not found");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 725: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let record_id = ("user", "john");
	let _: Option<ApiRecordId> = db.create(record_id).await.unwrap();
	let Some(record): Option<ApiRecordId> = db.select(record_id).await.unwrap() else {
		panic!("record not found");
	};
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 729: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	};
	assert_eq!(record.id, "user:john".parse().unwrap());
	let value: Value = db.select(Resource::from(record_id)).await.unwrap();
	assert_eq!(
		value.into_inner().record(),
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn select_record_ranges(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let table = "user";
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 741: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	drop(permit);
	let table = "user";
	let _: Option<ApiRecordId> = db.create((table, "amos")).await.unwrap();
	let _: Option<ApiRecordId> = db.create((table, "jane")).await.unwrap();
	let _: Option<ApiRecordId> = db.create((table, "john")).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 742: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let table = "user";
	let _: Option<ApiRecordId> = db.create((table, "amos")).await.unwrap();
	let _: Option<ApiRecordId> = db.create((table, "jane")).await.unwrap();
	let _: Option<ApiRecordId> = db.create((table, "john")).await.unwrap();
	let _: Value = db.create(Resource::from((table, "zoey"))).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 743: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Option<ApiRecordId> = db.create((table, "amos")).await.unwrap();
	let _: Option<ApiRecordId> = db.create((table, "jane")).await.unwrap();
	let _: Option<ApiRecordId> = db.create((table, "john")).await.unwrap();
	let _: Value = db.create(Resource::from((table, "zoey"))).await.unwrap();
	let convert = |users: Vec<ApiRecordId>| -> Vec<String> {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 744: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Option<ApiRecordId> = db.create((table, "jane")).await.unwrap();
	let _: Option<ApiRecordId> = db.create((table, "john")).await.unwrap();
	let _: Value = db.create(Resource::from((table, "zoey"))).await.unwrap();
	let convert = |users: Vec<ApiRecordId>| -> Vec<String> {
		users
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 756: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
			.collect()
	};
	let users: Vec<ApiRecordId> = db.select(table).range(..).await.unwrap();
	assert_eq!(convert(users), vec!["amos", "jane", "john", "zoey"]);
	let users: Vec<ApiRecordId> = db.select(table).range(.."john").await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 758: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let users: Vec<ApiRecordId> = db.select(table).range(..).await.unwrap();
	assert_eq!(convert(users), vec!["amos", "jane", "john", "zoey"]);
	let users: Vec<ApiRecordId> = db.select(table).range(.."john").await.unwrap();
	assert_eq!(convert(users), vec!["amos", "jane"]);
	let users: Vec<ApiRecordId> = db.select(table).range(..="john").await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 760: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let users: Vec<ApiRecordId> = db.select(table).range(.."john").await.unwrap();
	assert_eq!(convert(users), vec!["amos", "jane"]);
	let users: Vec<ApiRecordId> = db.select(table).range(..="john").await.unwrap();
	assert_eq!(convert(users), vec!["amos", "jane", "john"]);
	let users: Vec<ApiRecordId> = db.select(table).range("jane"..).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 762: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let users: Vec<ApiRecordId> = db.select(table).range(..="john").await.unwrap();
	assert_eq!(convert(users), vec!["amos", "jane", "john"]);
	let users: Vec<ApiRecordId> = db.select(table).range("jane"..).await.unwrap();
	assert_eq!(convert(users), vec!["jane", "john", "zoey"]);
	let users: Vec<ApiRecordId> = db.select(table).range("jane".."john").await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 764: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let users: Vec<ApiRecordId> = db.select(table).range("jane"..).await.unwrap();
	assert_eq!(convert(users), vec!["jane", "john", "zoey"]);
	let users: Vec<ApiRecordId> = db.select(table).range("jane".."john").await.unwrap();
	assert_eq!(convert(users), vec!["jane"]);
	let users: Vec<ApiRecordId> = db.select(table).range("jane"..="john").await.unwrap();
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let users: Vec<ApiRecordId> = db.select(table).range("jane".."john").await.unwrap();
	assert_eq!(convert(users), vec!["jane"]);
	let users: Vec<ApiRecordId> = db.select(table).range("jane"..="john").await.unwrap();
	assert_eq!(convert(users), vec!["jane", "john"]);
	let v: Value = db.select(Resource::from(table)).range("jane"..="john").await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 768: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let users: Vec<ApiRecordId> = db.select(table).range("jane"..="john").await.unwrap();
	assert_eq!(convert(users), vec!["jane", "john"]);
	let v: Value = db.select(Resource::from(table)).range("jane"..="john").await.unwrap();
	let val::Value::Array(array) = v.into_inner() else {
		panic!()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 774: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	assert_eq!(array.len(), 2);
	let users: Vec<ApiRecordId> =
		db.select(table).range((Bound::Excluded("jane"), Bound::Included("john"))).await.unwrap();
	assert_eq!(convert(users), vec!["john"]);
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn select_records_order_by_start_limit(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let sql = "
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 788: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        CREATE user:jane SET name = 'Jane';
    ";
	db.query(sql).await.unwrap().check().unwrap();

	let check_start_limit = |mut response: Response, expected: Vec<&str>| {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 788: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        CREATE user:jane SET name = 'Jane';
    ";
	db.query(sql).await.unwrap().check().unwrap();

	let check_start_limit = |mut response: Response, expected: Vec<&str>| {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 791: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

	let check_start_limit = |mut response: Response, expected: Vec<&str>| {
		let users: Vec<RecordName> = response.take(0).unwrap();
		let users: Vec<String> = users.into_iter().map(|user| user.name).collect();
		assert_eq!(users, expected);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 797: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

	let response =
		db.query("SELECT name FROM user ORDER BY name DESC START 1 LIMIT 2").await.unwrap();
	check_start_limit(response, vec!["John", "Jane"]);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 800: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	check_start_limit(response, vec!["John", "Jane"]);

	let response = db.query("SELECT name FROM user ORDER BY name DESC START 1").await.unwrap();
	check_start_limit(response, vec!["John", "Jane", "Amos"]);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 803: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	check_start_limit(response, vec!["John", "Jane", "Amos"]);

	let response = db.query("SELECT name FROM user ORDER BY name DESC START 4").await.unwrap();
	check_start_limit(response, vec![]);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 806: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	check_start_limit(response, vec![]);

	let response = db.query("SELECT name FROM user ORDER BY name DESC LIMIT 2").await.unwrap();
	check_start_limit(response, vec!["Zoey", "John"]);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 809: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	check_start_limit(response, vec!["Zoey", "John"]);

	let response = db.query("SELECT name FROM user ORDER BY name DESC LIMIT 10").await.unwrap();
	check_start_limit(response, vec!["Zoey", "John", "Jane", "Amos"]);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 815: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn select_records_order_by(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let sql = "
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 823: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        CREATE user:jane SET name = 'Jane';
    ";
	db.query(sql).await.unwrap().check().unwrap();
	let sql = "SELECT name FROM user ORDER BY name DESC";
	let mut response = db.query(sql).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 823: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        CREATE user:jane SET name = 'Jane';
    ";
	db.query(sql).await.unwrap().check().unwrap();
	let sql = "SELECT name FROM user ORDER BY name DESC";
	let mut response = db.query(sql).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 825: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	db.query(sql).await.unwrap().check().unwrap();
	let sql = "SELECT name FROM user ORDER BY name DESC";
	let mut response = db.query(sql).await.unwrap();
	let users: Vec<RecordName> = response.take(0).unwrap();
	let convert = |users: Vec<RecordName>| -> Vec<String> {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 826: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let sql = "SELECT name FROM user ORDER BY name DESC";
	let mut response = db.query(sql).await.unwrap();
	let users: Vec<RecordName> = response.take(0).unwrap();
	let convert = |users: Vec<RecordName>| -> Vec<String> {
		users.into_iter().map(|user| user.name).collect()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 835: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn select_records_fetch(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let sql = "
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 844: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		CREATE person:jaime SET tags = [tag:js];
    ";
	db.query(sql).await.unwrap().check().unwrap();

	let check_fetch = |mut response: Response, expected: &str| {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 844: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		CREATE person:jaime SET tags = [tag:js];
    ";
	db.query(sql).await.unwrap().check().unwrap();

	let check_fetch = |mut response: Response, expected: &str| {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 847: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

	let check_fetch = |mut response: Response, expected: &str| {
		let val: Value = response.take(0).unwrap();
		let exp = expected.parse().unwrap();
		assert_eq!(val, exp);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 848: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let check_fetch = |mut response: Response, expected: &str| {
		let val: Value = response.take(0).unwrap();
		let exp = expected.parse().unwrap();
		assert_eq!(val, exp);
	};
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 853: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

	let sql = "SELECT * FROM person LIMIT 1 FETCH tags;";
	let response = db.query(sql).await.unwrap();
	check_fetch(
		response,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 870: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

	let sql = "SELECT * FROM person START 1 LIMIT 1 FETCH tags;";
	let response = db.query(sql).await.unwrap();
	check_fetch(
		response,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 895: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

	let sql = "SELECT * FROM person ORDER BY id FETCH tags;";
	let response = db.query(sql).await.unwrap();
	check_fetch(
		response,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 931: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn update_table(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let table = "user";
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 934: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	drop(permit);
	let table = "user";
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Value = db.update(Resource::from(table)).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 935: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let table = "user";
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Value = db.update(Resource::from(table)).await.unwrap();
	let users: Vec<ApiRecordId> = db.update(table).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 936: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Value = db.update(Resource::from(table)).await.unwrap();
	let users: Vec<ApiRecordId> = db.update(table).await.unwrap();
	assert_eq!(users.len(), 2);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 937: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Value = db.update(Resource::from(table)).await.unwrap();
	let users: Vec<ApiRecordId> = db.update(table).await.unwrap();
	assert_eq!(users.len(), 2);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 943: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn update_record_id(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let table = "user";
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 946: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	drop(permit);
	let table = "user";
	let _: Option<ApiRecordId> = db.create((table, "john")).await.unwrap();
	let _: Option<ApiRecordId> = db.create((table, "jane")).await.unwrap();
	let users: Vec<ApiRecordId> = db.update(table).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 947: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let table = "user";
	let _: Option<ApiRecordId> = db.create((table, "john")).await.unwrap();
	let _: Option<ApiRecordId> = db.create((table, "jane")).await.unwrap();
	let users: Vec<ApiRecordId> = db.update(table).await.unwrap();
	assert_eq!(users.len(), 2);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 948: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Option<ApiRecordId> = db.create((table, "john")).await.unwrap();
	let _: Option<ApiRecordId> = db.create((table, "jane")).await.unwrap();
	let users: Vec<ApiRecordId> = db.update(table).await.unwrap();
	assert_eq!(users.len(), 2);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 954: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn update_table_with_content(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let sql = "
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 963: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    ";
	let table = "user";
	let response = db.query(sql).bind(("table", table)).await.unwrap();
	response.check().unwrap();
	let users: Vec<RecordBuf> = db
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 964: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let table = "user";
	let response = db.query(sql).bind(("table", table)).await.unwrap();
	response.check().unwrap();
	let users: Vec<RecordBuf> = db
		.update(table)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 971: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();
	let expected = &[
		RecordBuf {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 974: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let expected = &[
		RecordBuf {
			id: "user:amos".parse().unwrap(),
			name: "Doe".to_owned(),
		},
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 978: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		},
		RecordBuf {
			id: "user:jane".parse().unwrap(),
			name: "Doe".to_owned(),
		},
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 982: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		},
		RecordBuf {
			id: "user:john".parse().unwrap(),
			name: "Doe".to_owned(),
		},
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 986: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		},
		RecordBuf {
			id: "user:zoey".parse().unwrap(),
			name: "Doe".to_owned(),
		},
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 991: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	];
	assert_eq!(users, expected);
	let users: Vec<RecordBuf> = db.select(table).await.unwrap();
	assert_eq!(users, expected);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 997: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn update_record_range_with_content(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let sql = "
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1006: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    ";
	let table = "user";
	let response = db.query(sql).bind(("table", table)).await.unwrap();
	response.check().unwrap();
	let users: Vec<RecordBuf> = db
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1007: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let table = "user";
	let response = db.query(sql).bind(("table", table)).await.unwrap();
	response.check().unwrap();
	let users: Vec<RecordBuf> = db
		.update(table)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1015: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();
	assert_eq!(
		users,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1029: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		]
	);
	let users: Vec<RecordBuf> = db.select(table).await.unwrap();
	assert_eq!(
		users,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1055: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn update_record_id_with_content(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let record_id = ("user", "john");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1064: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();
	assert_eq!(user.unwrap().name, "Jane Doe");
	let user: Option<RecordName> = db
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1072: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();
	assert_eq!(user.unwrap().name, "John Doe");
	let user: Option<RecordName> = db.select(record_id).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1074: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.unwrap();
	assert_eq!(user.unwrap().name, "John Doe");
	let user: Option<RecordName> = db.select(record_id).await.unwrap();
	assert_eq!(user.unwrap().name, "John Doe");
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1095: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn update_merge_record_id(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let record_id = ("person", "jaime");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1110: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();
	assert_eq!(jaime.unwrap().id.unwrap(), "person:jaime".parse().unwrap());
	jaime = db.update(record_id).merge(json!({ "marketing": true })).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1112: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.unwrap();
	assert_eq!(jaime.unwrap().id.unwrap(), "person:jaime".parse().unwrap());
	jaime = db.update(record_id).merge(json!({ "marketing": true })).await.unwrap();
	assert!(jaime.as_ref().unwrap().marketing);
	jaime = db.select(record_id).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1114: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	jaime = db.update(record_id).merge(json!({ "marketing": true })).await.unwrap();
	assert!(jaime.as_ref().unwrap().marketing);
	jaime = db.select(record_id).await.unwrap();
	assert_eq!(
		jaime.unwrap(),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1131: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn upsert_merge_record_id(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	// Create a new record using upsert
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1147: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();
	assert_eq!(jaime.unwrap().id.unwrap(), "person:jaime".parse().unwrap());
	// Update the record using merge
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1150: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	assert_eq!(jaime.unwrap().id.unwrap(), "person:jaime".parse().unwrap());
	// Update the record using merge
	jaime = db.upsert(record_id).merge(json!({ "marketing": true })).await.unwrap();
	assert!(jaime.as_ref().unwrap().marketing);
	jaime = db.select(record_id).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1152: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	jaime = db.upsert(record_id).merge(json!({ "marketing": true })).await.unwrap();
	assert!(jaime.as_ref().unwrap().marketing);
	jaime = db.select(record_id).await.unwrap();
	assert_eq!(
		jaime,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1178: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();
	assert_eq!(
		tobie,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1192: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	);
	// Ensure the record is saved
	tobie = db.select(("person", "tobie")).await.unwrap();
	assert_eq!(
		tobie,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1216: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let id = "john";
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1226: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		}))
		.await
		.unwrap();
	let _: Option<Record> = db
		.update(("user", id))
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1233: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.patch(PatchOp::remove("/foo"))
		.await
		.unwrap();
	let value: Option<Record> = db.select(("user", id)).await.unwrap();
	assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1234: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.await
		.unwrap();
	let value: Option<Record> = db.select(("user", id)).await.unwrap();
	assert_eq!(
		value,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1254: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let id = "john";
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1265: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		}))
		.await
		.unwrap();
	// Update the record using patch
	let _: Option<Record> = db
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1271: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.patch(PatchOps::new().replace("/baz", "boo").add("/hello", ["world"]).remove("/foo"))
		.await
		.unwrap();
	let value: Option<Record> = db.select(("user", id)).await.unwrap();
	assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1272: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.await
		.unwrap();
	let value: Option<Record> = db.select(("user", id)).await.unwrap();
	assert_eq!(
		value,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1286: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.patch(PatchOps::new().replace("/baz", "boo").add("/hello", ["world"]).remove("/foo"))
		.await
		.unwrap();
	assert_eq!(
		jane,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1296: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	);
	// Ensure the record is saved
	jane = db.select(("user", "jane")).await.unwrap();
	assert_eq!(
		jane,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1316: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let id = "john";
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1326: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		}))
		.await
		.unwrap();
	let _: Option<Record> = db
		.update(("user", id))
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1331: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.patch(PatchOps::new().replace("/baz", "boo").add("/hello", ["world"]).remove("/foo"))
		.await
		.unwrap();
	let value: Option<Record> = db.select(("user", id)).await.unwrap();
	assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1332: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		.await
		.unwrap();
	let value: Option<Record> = db.select(("user", id)).await.unwrap();
	assert_eq!(
		value,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1345: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn delete_table(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let table = "user";
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1348: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	drop(permit);
	let table = "user";
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1349: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let table = "user";
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let users: Vec<ApiRecordId> = db.select(table).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1350: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let users: Vec<ApiRecordId> = db.select(table).await.unwrap();
	assert_eq!(users.len(), 3);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1351: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let _: Option<ApiRecordId> = db.create(table).await.unwrap();
	let users: Vec<ApiRecordId> = db.select(table).await.unwrap();
	assert_eq!(users.len(), 3);
	let users: Vec<ApiRecordId> = db.delete(table).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1353: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let users: Vec<ApiRecordId> = db.select(table).await.unwrap();
	assert_eq!(users.len(), 3);
	let users: Vec<ApiRecordId> = db.delete(table).await.unwrap();
	assert_eq!(users.len(), 3);
	let users: Vec<ApiRecordId> = db.select(table).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1355: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let users: Vec<ApiRecordId> = db.delete(table).await.unwrap();
	assert_eq!(users.len(), 3);
	let users: Vec<ApiRecordId> = db.select(table).await.unwrap();
	assert!(users.is_empty());
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1361: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn delete_record_id(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let record_id = ("user", "john");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1364: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	drop(permit);
	let record_id = ("user", "john");
	let _: Option<ApiRecordId> = db.create(record_id).await.unwrap();
	let _: Option<ApiRecordId> = db.select(record_id).await.unwrap();
	let john: Option<ApiRecordId> = db.delete(record_id).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1365: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let record_id = ("user", "john");
	let _: Option<ApiRecordId> = db.create(record_id).await.unwrap();
	let _: Option<ApiRecordId> = db.select(record_id).await.unwrap();
	let john: Option<ApiRecordId> = db.delete(record_id).await.unwrap();
	assert!(john.is_some());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1366: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _: Option<ApiRecordId> = db.create(record_id).await.unwrap();
	let _: Option<ApiRecordId> = db.select(record_id).await.unwrap();
	let john: Option<ApiRecordId> = db.delete(record_id).await.unwrap();
	assert!(john.is_some());
	let john: Option<ApiRecordId> = db.select(record_id).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1368: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let john: Option<ApiRecordId> = db.delete(record_id).await.unwrap();
	assert!(john.is_some());
	let john: Option<ApiRecordId> = db.select(record_id).await.unwrap();
	assert!(john.is_none());
	// non-existing user
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1371: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	assert!(john.is_none());
	// non-existing user
	let jane: Option<ApiRecordId> = db.delete(("user", "jane")).await.unwrap();
	assert!(jane.is_none());
	let value: Value = db.delete(Resource::from(("user", "jane"))).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1373: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let jane: Option<ApiRecordId> = db.delete(("user", "jane")).await.unwrap();
	assert!(jane.is_none());
	let value: Value = db.delete(Resource::from(("user", "jane"))).await.unwrap();
	assert_eq!(value.into_inner(), val::Value::None);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1379: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn delete_record_range(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let sql = "
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1388: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    ";
	let table = "user";
	let response = db.query(sql).bind(("table", table)).await.unwrap();
	response.check().unwrap();
	let users: Vec<RecordBuf> = db.delete(table).range("jane".."zoey").await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1389: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let table = "user";
	let response = db.query(sql).bind(("table", table)).await.unwrap();
	response.check().unwrap();
	let users: Vec<RecordBuf> = db.delete(table).range("jane".."zoey").await.unwrap();
	assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1390: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let response = db.query(sql).bind(("table", table)).await.unwrap();
	response.check().unwrap();
	let users: Vec<RecordBuf> = db.delete(table).range("jane".."zoey").await.unwrap();
	assert_eq!(
		users,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1404: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		]
	);
	let users: Vec<RecordBuf> = db.select(table).await.unwrap();
	assert_eq!(
		users,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1422: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn changefeed(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	// Enable change feeds
	let sql = "
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1427: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    DEFINE TABLE testuser CHANGEFEED 1h;
    ";
	let response = db.query(sql).await.unwrap();
	response.check().unwrap();
	// Create and update records
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1428: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    ";
	let response = db.query(sql).await.unwrap();
	response.check().unwrap();
	// Create and update records
	let sql = "
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1436: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    ";
	let table = "testuser";
	let response = db.query(sql).await.unwrap();
	response.check().unwrap();
	let users: Vec<RecordBuf> = db
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1437: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let table = "testuser";
	let response = db.query(sql).await.unwrap();
	response.check().unwrap();
	let users: Vec<RecordBuf> = db
		.update(table)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1444: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		})
		.await
		.unwrap();
	let expected = &[
		RecordBuf {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1447: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let expected = &[
		RecordBuf {
			id: "testuser:amos".parse().unwrap(),
			name: "Doe".to_owned(),
		},
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1451: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		},
		RecordBuf {
			id: "testuser:jane".parse().unwrap(),
			name: "Doe".to_owned(),
		},
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1456: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	];
	assert_eq!(users, expected);
	let users: Vec<RecordBuf> = db.select(table).await.unwrap();
	assert_eq!(users, expected);
	let sql = "
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1461: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        SHOW CHANGES FOR TABLE testuser SINCE 0 LIMIT 10;
    ";
	let mut response = db.query(sql).await.unwrap();
	drop(permit);
	let v: Value = response.take(0).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1463: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let mut response = db.query(sql).await.unwrap();
	drop(permit);
	let v: Value = response.take(0).unwrap();
	let val::Value::Array(array) = v.into_inner() else {
		panic!()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1469: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	assert_eq!(array.len(), 5);
	// DEFINE TABLE
	let a = array.first().unwrap();
	let val::Value::Object(a) = a.clone() else {
		unreachable!()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1473: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		unreachable!()
	};
	let val::Value::Number(_versionstamp1) = a.get("versionstamp").unwrap() else {
		unreachable!()
	};
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1476: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		unreachable!()
	};
	let changes = a.get("changes").unwrap().clone().clone();
	assert_eq!(
		Value::from_inner(changes),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1509: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		unreachable!()
	};
	let val::Value::Number(versionstamp1) = a.get("versionstamp").unwrap() else {
		unreachable!()
	};
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1512: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		unreachable!()
	};
	let changes = a.get("changes").unwrap().to_owned();
	assert_eq!(
		Value::from_inner(changes),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1531: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		unreachable!()
	};
	let val::Value::Number(versionstamp2) = a.get("versionstamp").unwrap().clone() else {
		unreachable!()
	};
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1535: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	};
	assert!(*versionstamp1 < versionstamp2);
	let changes = a.get("changes").unwrap().to_owned();
	assert_eq!(
		Value::from_inner(changes),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1554: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		unreachable!()
	};
	let val::Value::Number(versionstamp3) = a.get("versionstamp").unwrap() else {
		unreachable!()
	};
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1558: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	};
	assert!(versionstamp2 < *versionstamp3);
	let changes = a.get("changes").unwrap().to_owned();
	assert_eq!(
		Value::from_inner(changes),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1577: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		unreachable!()
	};
	let val::Value::Number(versionstamp4) = a.get("versionstamp").unwrap() else {
		unreachable!()
	};
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1581: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	};
	assert!(versionstamp3 < versionstamp4);
	let changes = a.get("changes").unwrap().to_owned();
	assert_eq!(
		Value::from_inner(changes),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1606: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let (permit, db) = new_db.create_db().await;
	drop(permit);
	db.version().await.unwrap();
}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1611: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn set_unset(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let (key, value) = ("name", "Doe");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1615: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let (key, value) = ("name", "Doe");
	let sql = "RETURN $name";
	db.set(key, value).await.unwrap();
	let mut response = db.query(sql).await.unwrap();
	let Some(name): Option<String> = response.take(0).unwrap() else {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1616: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let sql = "RETURN $name";
	db.set(key, value).await.unwrap();
	let mut response = db.query(sql).await.unwrap();
	let Some(name): Option<String> = response.take(0).unwrap() else {
		panic!("record not found");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1617: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	db.set(key, value).await.unwrap();
	let mut response = db.query(sql).await.unwrap();
	let Some(name): Option<String> = response.take(0).unwrap() else {
		panic!("record not found");
	};
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1624: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	db.set("token", value).await.unwrap_err();
	// make sure we can still run queries after trying to set a protected variable
	db.query("RETURN true").await.unwrap().check().unwrap();
	db.unset(key).await.unwrap();
	let mut response = db.query(sql).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1624: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	db.set("token", value).await.unwrap_err();
	// make sure we can still run queries after trying to set a protected variable
	db.query("RETURN true").await.unwrap().check().unwrap();
	db.unset(key).await.unwrap();
	let mut response = db.query(sql).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1625: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	// make sure we can still run queries after trying to set a protected variable
	db.query("RETURN true").await.unwrap().check().unwrap();
	db.unset(key).await.unwrap();
	let mut response = db.query(sql).await.unwrap();
	let name: Option<String> = response.take(0).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1626: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	db.query("RETURN true").await.unwrap().check().unwrap();
	db.unset(key).await.unwrap();
	let mut response = db.query(sql).await.unwrap();
	let name: Option<String> = response.take(0).unwrap();
	assert!(name.is_none());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1627: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	db.unset(key).await.unwrap();
	let mut response = db.query(sql).await.unwrap();
	let name: Option<String> = response.take(0).unwrap();
	assert!(name.is_none());
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1633: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn return_bool(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	let mut response = db.query("RETURN true").await.unwrap();
	drop(permit);
	let Some(boolean): Option<bool> = response.take(0).unwrap() else {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1635: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let mut response = db.query("RETURN true").await.unwrap();
	drop(permit);
	let Some(boolean): Option<bool> = response.take(0).unwrap() else {
		panic!("record not found");
	};
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1639: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	};
	assert!(boolean);
	let mut response = db.query("RETURN false").await.unwrap();
	let value: Value = response.take(0).unwrap();
	assert_eq!(value.into_inner(), val::Value::Bool(false));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1640: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	assert!(boolean);
	let mut response = db.query("RETURN false").await.unwrap();
	let value: Value = response.take(0).unwrap();
	assert_eq!(value.into_inner(), val::Value::Bool(false));
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1646: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn run(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);
	let sql = "
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1660: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	};
	";
	let _ = db.query(sql).await.unwrap();

	let tmp: i32 = db.run("fn::foo").await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1662: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let _ = db.query(sql).await.unwrap();

	let tmp: i32 = db.run("fn::foo").await.unwrap();
	assert_eq!(tmp, 42);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1673: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	assert!(tmp.to_string().contains("The function 'fn::idnotexist' does not exist"));

	let tmp: usize = db.run("count").args(vec![1, 2, 3]).await.unwrap();
	assert_eq!(tmp, 3);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1676: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	assert_eq!(tmp, 3);

	let tmp: Option<RecordId> = db.run("fn::bar").args(7).await.unwrap();
	assert_eq!(tmp, None);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1679: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	assert_eq!(tmp, None);

	let tmp: i32 = db.run("fn::baz").await.unwrap();
	assert_eq!(tmp, 7);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1685: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn multi_take(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1688: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	drop(permit);

	db.query("INSERT INTO user {name: 'John', address: 'USA'};").await.unwrap();
	db.query("INSERT INTO user {name: 'Adam', address: 'UK'};").await.unwrap();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1689: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

	db.query("INSERT INTO user {name: 'John', address: 'USA'};").await.unwrap();
	db.query("INSERT INTO user {name: 'Adam', address: 'UK'};").await.unwrap();

	let mut response = db.query("SELECT * FROM user").await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1691: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	db.query("INSERT INTO user {name: 'Adam', address: 'UK'};").await.unwrap();

	let mut response = db.query("SELECT * FROM user").await.unwrap();

	let mut names: Vec<String> = response.take("name").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1693: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let mut response = db.query("SELECT * FROM user").await.unwrap();

	let mut names: Vec<String> = response.take("name").unwrap();
	names.sort();
	assert_eq!(names, vec!["Adam".to_owned(), "John".to_owned()]);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1697: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	assert_eq!(names, vec!["Adam".to_owned(), "John".to_owned()]);

	let mut addresses: Vec<String> = response.take("address").unwrap();
	addresses.sort();
	assert_eq!(addresses, vec!["UK".to_owned(), "USA".to_owned()]);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1704: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
pub async fn field_and_index_methods(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
	drop(permit);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1708: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

	let mut response =
		db.query("SELECT b1 FROM CREATE something SET b1.total_peers = 74").await.unwrap();
	let as_value: Value = response.take::<Value>(0).unwrap();
	let inside = as_value.get(0).get("b1").get("total_peers");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1709: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let mut response =
		db.query("SELECT b1 FROM CREATE something SET b1.total_peers = 74").await.unwrap();
	let as_value: Value = response.take::<Value>(0).unwrap();
	let inside = as_value.get(0).get("b1").get("total_peers");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1717: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

	let mut response =
		db.query("SELECT b1 FROM CREATE something SET b1.total_peers = 74").await.unwrap();
	let as_value: Value = response.take::<Value>(0).unwrap();
	// Second .get() is a non-existent field
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1718: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	let mut response =
		db.query("SELECT b1 FROM CREATE something SET b1.total_peers = 74").await.unwrap();
	let as_value: Value = response.take::<Value>(0).unwrap();
	// Second .get() is a non-existent field
	let inside = as_value.get(0).get("b1111111").get("total_peers");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Orphaned Methods


### `update_table()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 929)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn update_table(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `delete_table()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 1343)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn delete_table(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `set_unset()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 1609)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn set_unset(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `mixed_results_query()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 475)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn mixed_results_query(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `insert_relation_table()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 687)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn insert_relation_table(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `yuse()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 26)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn yuse(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	let item = Ulid::new().to_string();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `select_record_id()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 719)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn select_record_id(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `select_records_order_by()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 813)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn select_records_order_by(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `changefeed()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 1420)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn changefeed(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `query_with_stats()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 443)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn query_with_stats(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `return_bool()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 1631)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn return_bool(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	let mut response = db.query("RETURN true").await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `query_binds()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 411)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn query_binds(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `select_table()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 707)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn select_table(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `update_merge_record_id()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 1093)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn update_merge_record_id(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `upsert_merge_record_id()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 1129)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn upsert_merge_record_id(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `update_record_id_with_content()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 1053)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn update_record_id_with_content(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `update_record_id()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 941)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn update_record_id(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `select_record_ranges()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 736)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn select_record_ranges(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `signin_record()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 123)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn signin_record(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	let database = Ulid::new().to_string();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `record_access_throws_error()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 165)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn record_access_throws_error(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	let database = Ulid::new().to_string();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `signin_ns()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 85)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn signin_ns(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `delete_record_range()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 1377)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn delete_record_range(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `multi_take()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 1683)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn multi_take(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `upsert_patch_record_id()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 1245)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn upsert_patch_record_id(new_db: impl CreateDb) {
	#[derive(Debug, Deserialize, PartialEq)]
	struct Record {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `update_table_with_content()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 952)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn update_table_with_content(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `field_and_index_methods()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 1702)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn field_and_index_methods(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `select_records_order_by_start_limit()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 778)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn select_records_order_by_start_limit(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `create_record_with_id()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 493)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn create_record_with_id(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `create_record_with_id_with_content()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 522)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn create_record_with_id_with_content(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `signin_db()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 103)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn signin_db(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	let database = Ulid::new().to_string();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `create_record_no_id()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 485)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn create_record_no_id(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `query_raw()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 377)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn query_raw(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `insert_unspecified()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 653)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn insert_unspecified(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `record_access_invalid_query()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 247)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn record_access_invalid_query(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	let database = Ulid::new().to_string();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `create_record_no_id_with_content()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 502)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn create_record_no_id_with_content(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `update_record_range_with_content()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 995)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn update_record_range_with_content(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `patch_record_id()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 1207)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn patch_record_id(new_db: impl CreateDb) {
	#[derive(Debug, Deserialize, PartialEq)]
	struct Record {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `insert_table()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 617)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn insert_table(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `query_chaining()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 459)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn query_chaining(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `create_record_with_id_in_content()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 547)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn create_record_with_id_in_content(new_db: impl CreateDb) {
	#[derive(Debug, Serialize, Deserialize)]
	pub struct Person {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `delete_record_id()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 1359)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn delete_record_id(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `insert_thing()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 633)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn insert_thing(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `query_decimals()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 399)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn query_decimals(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `patch_record_id_ops()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 1307)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn patch_record_id_ops(new_db: impl CreateDb) {
	#[derive(Debug, Deserialize, PartialEq)]
	struct Record {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `select_records_fetch()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 833)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn select_records_fetch(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	db.use_ns(NS).use_db(Ulid::new().to_string()).await.unwrap();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `signup_record()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/tests/api_integration/basic.rs` (line 56)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub async fn signup_record(new_db: impl CreateDb) {
	let (permit, db) = new_db.create_db().await;
	let database = Ulid::new().to_string();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym