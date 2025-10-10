# `forks/pingora/pingora-cache/src/eviction/lru.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-cache
- **File Hash**: 90c70c5c  
- **Timestamp**: 2025-10-10T02:16:01.413761+00:00  
- **Lines of Code**: 367

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 367 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 248
  - TODO
  - 

```rust

    async fn load(&self, dir_path: &str) -> Result<()> {
        // TODO: check the saved shards so that we load all the save files
        let mut loaded_shards = 0;
        for i in 0..N {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 223
  - stubby variable name
  - temp_path

```rust
                // create a temporary filename using a randomized u32 hash to minimize the chance of multiple writers writing to the same tmp file
                let random_suffix: u32 = rand::thread_rng().gen();
                let temp_path =
                    dir_path.join(format!("{}.{i}.{:08x}.tmp", FILE_NAME, random_suffix));
                let mut file = File::create(&temp_path)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 225
  - stubby variable name
  - temp_path

```rust
                let temp_path =
                    dir_path.join(format!("{}.{i}.{:08x}.tmp", FILE_NAME, random_suffix));
                let mut file = File::create(&temp_path)
                    .or_err_with(InternalError, || err_str_path("fail to create", &temp_path))?;
                file.write_all(&data).or_err_with(InternalError, || {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 226
  - stubby variable name
  - temp_path

```rust
                    dir_path.join(format!("{}.{i}.{:08x}.tmp", FILE_NAME, random_suffix));
                let mut file = File::create(&temp_path)
                    .or_err_with(InternalError, || err_str_path("fail to create", &temp_path))?;
                file.write_all(&data).or_err_with(InternalError, || {
                    err_str_path("fail to write to", &temp_path)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 228
  - stubby variable name
  - temp_path

```rust
                    .or_err_with(InternalError, || err_str_path("fail to create", &temp_path))?;
                file.write_all(&data).or_err_with(InternalError, || {
                    err_str_path("fail to write to", &temp_path)
                })?;
                file.flush().or_err_with(InternalError, || {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 231
  - stubby variable name
  - temp_path

```rust
                })?;
                file.flush().or_err_with(InternalError, || {
                    err_str_path("fail to flush temp file", &temp_path)
                })?;
                rename(&temp_path, &final_path).or_err_with(InternalError, || {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 233
  - stubby variable name
  - temp_path

```rust
                    err_str_path("fail to flush temp file", &temp_path)
                })?;
                rename(&temp_path, &final_path).or_err_with(InternalError, || {
                    format!(
                        "Failed to rename file from {} to {}",
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 236
  - stubby variable name
  - temp_path

```rust
                    format!(
                        "Failed to rename file from {} to {}",
                        temp_path.display(),
                        final_path.display(),
                    )
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 84: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .or_err(InternalError, "fail to serialize node")?;
        for node in nodes {
            seq.serialize_element(&node).unwrap(); // write to vec, safe
        }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 453: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        // load lru2 with lru's data
        let ser = lru.serialize_shard(0).unwrap();
        let lru2 = Manager::<1>::with_capacity(4, 10);
        lru2.deserialize_shard(&ser).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 455: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let ser = lru.serialize_shard(0).unwrap();
        let lru2 = Manager::<1>::with_capacity(4, 10);
        lru2.deserialize_shard(&ser).unwrap();

        let key4 = CacheKey::new("", "d", "1").to_compact();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 476: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        // load lru2 with lru's data
        lru.save("/tmp/test_lru_save").await.unwrap();
        let lru2 = Manager::<2>::with_capacity(4, 10);
        lru2.load("/tmp/test_lru_save").await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 478: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        lru.save("/tmp/test_lru_save").await.unwrap();
        let lru2 = Manager::<2>::with_capacity(4, 10);
        lru2.load("/tmp/test_lru_save").await.unwrap();

        let ser0 = lru.serialize_shard(0).unwrap();
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
- **Issue**: Can panic in production code

```rust
        lru2.load("/tmp/test_lru_save").await.unwrap();

        let ser0 = lru.serialize_shard(0).unwrap();
        let ser1 = lru.serialize_shard(1).unwrap();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 481: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let ser0 = lru.serialize_shard(0).unwrap();
        let ser1 = lru.serialize_shard(1).unwrap();

        assert_eq!(ser0, lru2.serialize_shard(0).unwrap());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 289: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/eviction/lru.rs` (line 289)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod test {
    use super::*;
    use crate::CacheKey;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 296: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/eviction/lru.rs` (line 296)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_admission() {
        let lru = Manager::<1>::with_capacity(4, 10);
        let key1 = CacheKey::new("", "a", "1").to_compact();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 320: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/eviction/lru.rs` (line 320)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_access() {
        let lru = Manager::<1>::with_capacity(4, 10);
        let key1 = CacheKey::new("", "a", "1").to_compact();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 345: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/eviction/lru.rs` (line 345)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_remove() {
        let lru = Manager::<1>::with_capacity(4, 10);
        let key1 = CacheKey::new("", "a", "1").to_compact();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 370: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/eviction/lru.rs` (line 370)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_access_add() {
        let lru = Manager::<1>::with_capacity(4, 10);
        let until = SystemTime::now(); // unused value as a placeholder
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 390: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/eviction/lru.rs` (line 390)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_admit_update() {
        let lru = Manager::<1>::with_capacity(4, 10);
        let key1 = CacheKey::new("", "a", "1").to_compact();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 421: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/eviction/lru.rs` (line 421)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_peek() {
        let lru = Manager::<1>::with_capacity(4, 10);
        let until = SystemTime::now(); // unused value as a placeholder
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 434: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/eviction/lru.rs` (line 434)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_serde() {
        let lru = Manager::<1>::with_capacity(4, 10);
        let key1 = CacheKey::new("", "a", "1").to_compact();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 464: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/eviction/lru.rs` (line 464)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_save_to_disk() {
        let until = SystemTime::now(); // unused value as a placeholder
        let lru = Manager::<2>::with_capacity(10, 10);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym