# `forks/surrealdb/crates/sdk/src/api/engine/any/mod.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: sdk
- **File Hash**: f1292f3b  
- **Timestamp**: 2025-10-10T02:16:00.935822+00:00  
- **Lines of Code**: 162

---## Tier 1 Infractions 


- Line 22
  - would need
  - 

```rust
//! you are trying to use is enabled. If not, your code won't compile. This is
//! awesome but it strongly couples your application to the engine you are
//! using. In order to change an engine you would need to update your code to
//! the new scheme and endpoint you need to use and recompile it. This is where
//! the `any` engine comes in. We will call it `Surreal<Any>` (the type it
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 66
  - would need
  - 

```rust
//!     # Unconditionally enables the in-memory store.
//!     # Also not necessary but this will make `cargo run` just work.
//!     # Without it, you would need `cargo run --features surrealdb/kv-mem` during development. If you use a build
//!     # tool like `make` or `cargo make`, however, you can put that in your build step and avoid typing it manually.
//!     features = ["kv-mem"],
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 199: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	fn into_endpoint(self) -> Result<Endpoint> {
		let (url, path) = match self {
			"memory" | "mem://" => (Url::parse("mem://").unwrap(), "memory".to_owned()),
			url if url.starts_with("ws") | url.starts_with("http") | url.starts_with("tikv") => {
				(Url::parse(url).map_err(|_| Error::InvalidUrl(self.to_owned()))?, String::new())
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 344: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	async fn local_engine_without_auth() {
		// Instantiate an in-memory instance without root credentials
		let db = connect("memory").await.unwrap();
		db.use_ns("N").use_db("D").await.unwrap();
		// The client has access to everything
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 345: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		// Instantiate an in-memory instance without root credentials
		let db = connect("memory").await.unwrap();
		db.use_ns("N").use_db("D").await.unwrap();
		// The client has access to everything
		assert!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 361: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

		// There are no users in the datastore
		let mut res = db.query("INFO FOR ROOT").await.unwrap();
		let users: Value = res.take("users").unwrap();

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
		// There are no users in the datastore
		let mut res = db.query("INFO FOR ROOT").await.unwrap();
		let users: Value = res.take("users").unwrap();

		assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 380: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let db = connect(("memory", Config::new().user(creds).capabilities(Capabilities::all())))
			.await
			.unwrap();
		db.use_ns("N").use_db("D").await.unwrap();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 381: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			.await
			.unwrap();
		db.use_ns("N").use_db("D").await.unwrap();

		// The client needs to sign in before it can access anything
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 333: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/engine/any/mod.rs` (line 333)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(all(test, feature = "kv-mem"))]
mod tests {

	use super::*;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 342: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/engine/any/mod.rs` (line 342)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn local_engine_without_auth() {
		// Instantiate an in-memory instance without root credentials
		let db = connect("memory").await.unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 372: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/engine/any/mod.rs` (line 372)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[tokio::test]
	async fn local_engine_with_auth() {
		// Instantiate an in-memory instance with root credentials
		let creds = Root {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `__into_endpoint()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/engine/any/mod.rs` (line 181)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
#[doc(hidden)]
/// Internal API
pub fn __into_endpoint(path: impl IntoEndpoint) -> Result<Endpoint> {
	into_endpoint::Sealed::into_endpoint(path)
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