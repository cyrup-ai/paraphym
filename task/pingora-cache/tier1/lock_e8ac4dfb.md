# `forks/pingora/pingora-cache/src/lock.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-cache
- **File Hash**: e8ac4dfb  
- **Timestamp**: 2025-10-10T02:16:01.413479+00:00  
- **Lines of Code**: 382

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 382 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 45
  - for now
  - 

```rust
pub struct CacheLock {
    lock_table: ConcurrentHashTable<LockStub, N_SHARDS>,
    // fixed lock timeout values for now
    age_timeout_default: Duration,
}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 246
  - for now
  - 

```rust
        }

        // FIXME: for now it is the awkward responsibility of the ReadLock to set the
        // timeout status on the lock itself because the write permit cannot lock age
        // timeout on its own
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 246
  - FIXME
  - 

```rust
        }

        // FIXME: for now it is the awkward responsibility of the ReadLock to set the
        // timeout status on the lock itself because the write permit cannot lock age
        // timeout on its own
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 249
  - TODO
  - 

```rust
        // timeout status on the lock itself because the write permit cannot lock age
        // timeout on its own
        // TODO: need to be careful not to wake everyone up at the same time
        // (maybe not an issue because regular cache lock release behaves that way)
        if let Some(duration) = self.0.age_timeout.checked_sub(self.0.lock_start.elapsed()) {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 380: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        });
        permit.unlock(LockStatus::Done);
        handle.await.unwrap(); // check lock is unlocked and the task is returned
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 405: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        tokio::time::sleep(Duration::from_millis(2100)).await;

        handle.await.unwrap(); // check lock is timed out

        // expired lock - we will be able to install a new lock instead
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
- **Issue**: Can panic in production code

```rust
        permit.unlock(LockStatus::Done);
        permit2.unlock(LockStatus::Done);
        handle.await.unwrap();
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 449: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        tokio::time::sleep(Duration::from_millis(1100)).await; // let lock age time out
        handle.await.unwrap(); // check lock is timed out

        // writer finally finishes
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 473: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        permit.unlock(LockStatus::Done);
        handle.await.unwrap();
    }

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
- **Issue**: Can panic in production code

```rust

        for handle in handles {
            handle.await.unwrap();
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


### Line 340: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/lock.rs` (line 340)
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
  


### Line 345: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/lock.rs` (line 345)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_get_release() {
        let cache_lock = CacheLock::new_boxed(Duration::from_secs(1000));
        let key1 = CacheKey::new("", "a", "1");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 363: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/lock.rs` (line 363)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_lock() {
        let cache_lock = CacheLock::new_boxed(Duration::from_secs(1000));
        let key1 = CacheKey::new("", "a", "1");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 384: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/lock.rs` (line 384)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_lock_timeout() {
        let cache_lock = CacheLock::new_boxed(Duration::from_secs(1));
        let key1 = CacheKey::new("", "a", "1");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 429: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/lock.rs` (line 429)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_lock_expired_release() {
        let cache_lock = CacheLock::new_boxed(Duration::from_secs(1));
        let key1 = CacheKey::new("", "a", "1");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 477: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/lock.rs` (line 477)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_lock_expired_no_reader() {
        let cache_lock = CacheLock::new_boxed(Duration::from_secs(1));
        let key1 = CacheKey::new("", "a", "1");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 501: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/lock.rs` (line 501)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_lock_concurrent() {
        let _ = env_logger::builder().is_test(true).try_init();
        // Test that concurrent attempts to compete for a lock run without issues
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym