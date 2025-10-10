# `forks/surrealdb/crates/core/src/fnc/crypto.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: 3f60cd03  
- **Timestamp**: 2025-10-10T02:16:00.689730+00:00  
- **Lines of Code**: 227

---## Tier 1 Infractions 


- Line 141
  - FIXME
  - 

```rust
			Value::Bool(false)
		} else {
			// FIXME: If base64 dependency is added, can avoid parsing the HashParts twice,
			// once above and once in verity, by using bcrypt::bcrypt.
			bcrypt::verify(pass, &hash).unwrap_or(false).into()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 115: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let algo = Argon2::default();
		let salt = SaltString::generate(&mut OsRng);
		let hash = algo.hash_password(pass.as_ref(), &salt).unwrap().to_string();
		Ok(hash.into())
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 148: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

	pub fn r#gen((pass,): (String,)) -> Result<Value> {
		let hash = bcrypt::hash(pass, bcrypt::DEFAULT_COST).unwrap();
		Ok(hash.into())
	}
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
	pub fn r#gen((pass,): (String,)) -> Result<Value> {
		let salt = SaltString::generate(&mut OsRng);
		let hash = Pbkdf2.hash_password(pass.as_ref(), &salt).unwrap().to_string();
		Ok(hash.into())
	}
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
	pub fn r#gen((pass,): (String,)) -> Result<Value> {
		let salt = SaltString::generate(&mut OsRng);
		let hash = Scrypt.hash_password(pass.as_ref(), &salt).unwrap().to_string();
		Ok(hash.into())
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


### Line 267: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/crypto.rs` (line 267)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
	#[cfg(test)]
	#[allow(clippy::unreadable_literal)]
	mod tests {
		use super::*;

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 271: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/crypto.rs` (line 271)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

		#[test]
		fn test() {
			assert_eq!(hash_bytes(b""), 0);
			assert_eq!(hash_bytes(b"a"), 0xCA2E9442);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `blake3()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/crypto.rs` (line 8)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
use crate::val::Value;

pub fn blake3((arg,): (String,)) -> Result<Value> {
	Ok(blake3::hash(arg.as_bytes()).to_string().into())
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `sha1()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/crypto.rs` (line 24)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub fn sha1((arg,): (String,)) -> Result<Value> {
	let mut hasher = Sha1::new();
	hasher.update(arg.as_str());
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `md5()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/crypto.rs` (line 16)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub fn md5((arg,): (String,)) -> Result<Value> {
	let mut hasher = Md5::new();
	hasher.update(arg.as_str());
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `sha512()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/crypto.rs` (line 40)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub fn sha512((arg,): (String,)) -> Result<Value> {
	let mut hasher = Sha512::new();
	hasher.update(arg.as_str());
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `sha256()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/crypto.rs` (line 32)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub fn sha256((arg,): (String,)) -> Result<Value> {
	let mut hasher = Sha256::new();
	hasher.update(arg.as_str());
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `joaat()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/fnc/crypto.rs` (line 12)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub fn joaat((arg,): (String,)) -> Result<Value> {
	Ok(joaat::hash_bytes(arg.as_bytes()).into())
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