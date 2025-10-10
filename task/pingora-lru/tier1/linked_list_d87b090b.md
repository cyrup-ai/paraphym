# `forks/pingora/pingora-lru/src/linked_list.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-lru
- **File Hash**: d87b090b  
- **Timestamp**: 2025-10-10T02:16:01.083802+00:00  
- **Lines of Code**: 313

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 313 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 80
  - hopeful
  - 

```rust
        // The amortized growth cost is O(n) beyond the max of the initially reserved capacity and
        // the cap. But this list is for limited sized LRU and we recycle released node, so
        // hopefully insertions are rare beyond certain sizes
        if self.data_nodes.capacity() > VEC_EXP_GROWTH_CAP
            && self.data_nodes.capacity() - self.data_nodes.len() < 2
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 164
  - TODO
  - 

```rust
    fn valid_index(&self, index: Index) -> bool {
        index != HEAD && index != TAIL && index < self.nodes.len() + OFFSET
        // TODO: check node prev/next not NULL
        // TODO: debug_check index not in self.free
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 165
  - TODO
  - 

```rust
        index != HEAD && index != TAIL && index < self.nodes.len() + OFFSET
        // TODO: check node prev/next not NULL
        // TODO: debug_check index not in self.free
    }

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tests in Source Directory


### Line 356: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-lru/src/linked_list.rs` (line 356)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod test {
    use super::*;

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 371: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-lru/src/linked_list.rs` (line 371)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_insert() {
        let mut list = LinkedList::with_capacity(10);
        assert_eq!(list.len(), 0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 395: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-lru/src/linked_list.rs` (line 395)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_pop() {
        let mut list = LinkedList::with_capacity(10);
        list.push_head(2);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 408: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-lru/src/linked_list.rs` (line 408)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_promote() {
        let mut list = LinkedList::with_capacity(10);
        let index2 = list.push_head(2);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 426: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-lru/src/linked_list.rs` (line 426)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_exist_near_head() {
        let mut list = LinkedList::with_capacity(10);
        list.push_head(2);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym