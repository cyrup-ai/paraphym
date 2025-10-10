# `forks/surrealdb/crates/core/src/idx/trees/mtree.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: a0b99e39  
- **Timestamp**: 2025-10-10T02:16:00.648286+00:00  
- **Lines of Code**: 1992

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 1992 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Panic-Prone Code


### Line 1504: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

	async fn get_db(ds: &Datastore) -> Arc<DatabaseDefinition> {
		let tx = ds.transaction(TransactionType::Write, Optimistic).await.unwrap();
		tx.ensure_ns_db("myns", "mydb", false).await.unwrap()
	}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1505: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	async fn get_db(ds: &Datastore) -> Arc<DatabaseDefinition> {
		let tx = ds.transaction(TransactionType::Write, Optimistic).await.unwrap();
		tx.ensure_ns_db("myns", "mydb", false).await.unwrap()
	}

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1514: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		cache_size: usize,
	) -> (Context, TreeStore<MTreeNode>) {
		let tx = ds.transaction(tt, Optimistic).await.unwrap().enclose();
		let st = tx
			.index_caches()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1519: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			.get_store_mtree(TreeNodeProvider::Debug, t.state.generation, tt, cache_size)
			.await
			.unwrap();
		let mut ctx = MutableContext::default();
		ctx.set_transaction(tx);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1785: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
				)
				.await
				.unwrap();

				let map = if collection.len() < 1000 {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 1481: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/trees/mtree.rs` (line 1481)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
	use std::collections::VecDeque;
	use std::sync::Arc;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1810: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/trees/mtree.rs` (line 1810)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
	#[test(tokio::test)]
	#[ignore]
	async fn test_mtree_unique_xs() -> Result<()> {
		let mut stack = reblessive::tree::TreeStack::new();
		stack
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1843: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/trees/mtree.rs` (line 1843)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
	#[test(tokio::test)]
	#[ignore]
	async fn test_mtree_unique_xs_full_cache() -> Result<()> {
		let mut stack = reblessive::tree::TreeStack::new();
		stack
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1876: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/trees/mtree.rs` (line 1876)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
	#[test(tokio::test(flavor = "multi_thread"))]
	#[ignore]
	async fn test_mtree_unique_small() -> Result<()> {
		let mut stack = reblessive::tree::TreeStack::new();
		stack
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1900: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/trees/mtree.rs` (line 1900)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test(tokio::test(flavor = "multi_thread"))]
	async fn test_mtree_unique_normal() -> Result<()> {
		let mut stack = reblessive::tree::TreeStack::new();
		stack
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1924: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/trees/mtree.rs` (line 1924)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test(tokio::test(flavor = "multi_thread"))]
	async fn test_mtree_unique_normal_full_cache() -> Result<()> {
		let mut stack = reblessive::tree::TreeStack::new();
		stack
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1948: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/trees/mtree.rs` (line 1948)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test(tokio::test(flavor = "multi_thread"))]
	async fn test_mtree_unique_normal_small_cache() -> Result<()> {
		let mut stack = reblessive::tree::TreeStack::new();
		stack
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1973: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/trees/mtree.rs` (line 1973)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
	#[test(tokio::test)]
	#[ignore]
	async fn test_mtree_random_xs() -> Result<()> {
		let mut stack = reblessive::tree::TreeStack::new();
		stack
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2012: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/trees/mtree.rs` (line 2012)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
	#[test(tokio::test(flavor = "multi_thread"))]
	#[ignore]
	async fn test_mtree_random_small() -> Result<()> {
		let mut stack = reblessive::tree::TreeStack::new();
		stack
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2036: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/src/idx/trees/mtree.rs` (line 2036)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

	#[test(tokio::test(flavor = "multi_thread"))]
	async fn test_mtree_random_normal() -> Result<()> {
		let mut stack = reblessive::tree::TreeStack::new();
		stack
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym