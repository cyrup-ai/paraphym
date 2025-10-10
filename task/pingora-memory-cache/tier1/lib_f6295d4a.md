# `forks/pingora/pingora-memory-cache/src/lib.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-memory-cache
- **File Hash**: f6295d4a  
- **Timestamp**: 2025-10-10T02:16:01.275005+00:00  
- **Lines of Code**: 305

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 305 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 178
  - for now
  - 

```rust
        let hashed_key = self.hasher.hash_one(key);
        let node = Node::new(value, ttl);
        // weight is always 1 for now
        self.store.put(hashed_key, node, 1);
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 200
  - for now
  - 

```rust
        let hashed_key = self.hasher.hash_one(key);
        let node = Node::new(value, ttl);
        // weight is always 1 for now
        self.store.force_put(hashed_key, node, 1);
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 240
  - TODO
  - 

```rust
    }

    // TODO: evict expired first
}

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tests in Source Directory


### Line 244: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-memory-cache/src/lib.rs` (line 244)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 249: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-memory-cache/src/lib.rs` (line 249)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_get() {
        let cache: MemoryCache<i32, ()> = MemoryCache::new(10);
        let (res, hit) = cache.get(&1);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 257: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-memory-cache/src/lib.rs` (line 257)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_put_get() {
        let cache: MemoryCache<i32, i32> = MemoryCache::new(10);
        let (res, hit) = cache.get(&1);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 269: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-memory-cache/src/lib.rs` (line 269)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_put_get_remove() {
        let cache: MemoryCache<i32, i32> = MemoryCache::new(10);
        let (res, hit) = cache.get(&1);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 294: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-memory-cache/src/lib.rs` (line 294)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_get_expired() {
        let cache: MemoryCache<i32, i32> = MemoryCache::new(10);
        let (res, hit) = cache.get(&1);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 307: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-memory-cache/src/lib.rs` (line 307)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_get_stale() {
        let cache: MemoryCache<i32, i32> = MemoryCache::new(10);
        let (res, hit) = cache.get(&1);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 321: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-memory-cache/src/lib.rs` (line 321)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_eviction() {
        let cache: MemoryCache<i32, i32> = MemoryCache::new(2);
        cache.put(&1, 2, None);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 338: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-memory-cache/src/lib.rs` (line 338)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_multi_get() {
        let cache: MemoryCache<i32, i32> = MemoryCache::new(10);
        cache.put(&2, -2, None);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 362: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-memory-cache/src/lib.rs` (line 362)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_get_with_mismatched_key() {
        let cache: MemoryCache<String, ()> = MemoryCache::new(10);
        let (res, hit) = cache.get("Hello");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 370: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-memory-cache/src/lib.rs` (line 370)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_put_get_with_mismatched_key() {
        let cache: MemoryCache<String, i32> = MemoryCache::new(10);
        let (res, hit) = cache.get("1");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym