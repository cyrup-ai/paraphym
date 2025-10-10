# `forks/pingora/pingora-cache/src/eviction/simple_lru.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-cache
- **File Hash**: baba775e  
- **Timestamp**: 2025-10-10T02:16:01.413046+00:00  
- **Lines of Code**: 421

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 421 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 279
  - stubby variable name
  - temp_file_path

```rust
            // create a temporary filename using a randomized u32 hash to minimize the chance of multiple writers writing to the same tmp file
            let random_suffix: u32 = rand::thread_rng().gen();
            let temp_file_path = dir_path.join(format!("{}.{:08x}.tmp", FILE_NAME, random_suffix));
            let mut file = File::create(&temp_file_path).or_err_with(InternalError, || {
                format!("fail to create temporary file {}", temp_file_path.display())
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 280
  - stubby variable name
  - temp_file_path

```rust
            let random_suffix: u32 = rand::thread_rng().gen();
            let temp_file_path = dir_path.join(format!("{}.{:08x}.tmp", FILE_NAME, random_suffix));
            let mut file = File::create(&temp_file_path).or_err_with(InternalError, || {
                format!("fail to create temporary file {}", temp_file_path.display())
            })?;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 281
  - stubby variable name
  - temp_file_path

```rust
            let temp_file_path = dir_path.join(format!("{}.{:08x}.tmp", FILE_NAME, random_suffix));
            let mut file = File::create(&temp_file_path).or_err_with(InternalError, || {
                format!("fail to create temporary file {}", temp_file_path.display())
            })?;
            file.write_all(&data).or_err_with(InternalError, || {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 284
  - stubby variable name
  - temp_file_path

```rust
            })?;
            file.write_all(&data).or_err_with(InternalError, || {
                format!("fail to write to {}", temp_file_path.display())
            })?;
            file.flush().or_err_with(InternalError, || {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 287
  - stubby variable name
  - temp_file_path

```rust
            })?;
            file.flush().or_err_with(InternalError, || {
                format!("fail to flush temp file {}", temp_file_path.display())
            })?;
            std::fs::rename(&temp_file_path, &final_file_path).or_err_with(InternalError, || {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 289
  - stubby variable name
  - temp_file_path

```rust
                format!("fail to flush temp file {}", temp_file_path.display())
            })?;
            std::fs::rename(&temp_file_path, &final_file_path).or_err_with(InternalError, || {
                format!(
                    "fail to rename temporary file {} to {}",
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 292
  - stubby variable name
  - temp_file_path

```rust
                format!(
                    "fail to rename temporary file {} to {}",
                    temp_file_path.display(),
                    final_file_path.display()
                )
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 163: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .or_err(InternalError, "fail to serialize node")?;
        for item in lru.iter() {
            seq.serialize_element(item.1).unwrap(); // write to vec, safe
        }
        seq.end().or_err(InternalError, "when serializing LRU")?;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 469: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        // load lru2 with lru's data
        let ser = lru.serialize().unwrap();
        let lru2 = Manager::new(4);
        lru2.deserialize(&ser).unwrap();
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
- **Issue**: Can panic in production code

```rust
        let ser = lru.serialize().unwrap();
        let lru2 = Manager::new(4);
        lru2.deserialize(&ser).unwrap();

        let key4 = CacheKey::new("", "d", "1").to_compact();
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
- **Issue**: Can panic in production code

```rust

        // load lru2 with lru's data
        lru.save("/tmp/test_simple_lru_save").await.unwrap();
        let lru2 = Manager::new(4);
        lru2.load("/tmp/test_simple_lru_save").await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 501: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        lru.save("/tmp/test_simple_lru_save").await.unwrap();
        let lru2 = Manager::new(4);
        lru2.load("/tmp/test_simple_lru_save").await.unwrap();

        let key4 = CacheKey::new("", "d", "1").to_compact();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 320: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/eviction/simple_lru.rs` (line 320)
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
  


### Line 325: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/eviction/simple_lru.rs` (line 325)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_admission() {
        let lru = Manager::new(4);
        let key1 = CacheKey::new("", "a", "1").to_compact();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 349: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/eviction/simple_lru.rs` (line 349)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_access() {
        let lru = Manager::new(4);
        let key1 = CacheKey::new("", "a", "1").to_compact();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 374: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/eviction/simple_lru.rs` (line 374)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_remove() {
        let lru = Manager::new(4);
        let key1 = CacheKey::new("", "a", "1").to_compact();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 399: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/eviction/simple_lru.rs` (line 399)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_access_add() {
        let lru = Manager::new(4);
        let until = SystemTime::now(); // unused value as a placeholder
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 419: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/eviction/simple_lru.rs` (line 419)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_admit_update() {
        let lru = Manager::new(4);
        let key1 = CacheKey::new("", "a", "1").to_compact();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 450: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/eviction/simple_lru.rs` (line 450)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_serde() {
        let lru = Manager::new(4);
        let key1 = CacheKey::new("", "a", "1").to_compact();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 480: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/eviction/simple_lru.rs` (line 480)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_save_to_disk() {
        let lru = Manager::new(4);
        let key1 = CacheKey::new("", "a", "1").to_compact();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 510: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/eviction/simple_lru.rs` (line 510)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_watermark_eviction() {
        const SIZE_LIMIT: usize = usize::MAX / 2;
        let lru = Manager::new_with_watermark(SIZE_LIMIT, Some(4));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym