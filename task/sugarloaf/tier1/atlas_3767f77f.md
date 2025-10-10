# `packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/image_cache/atlas.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: sugarloaf
- **File Hash**: 3767f77f  
- **Timestamp**: 2025-10-10T02:15:59.433152+00:00  
- **Lines of Code**: 199

---## Tier 1 Infractions 


- Line 147
  - in practice
  - 

```rust
    }

    /// Deallocates a rectangle (simplified - in practice this is complex)
    pub fn deallocate(&mut self, _x: u16, _y: u16, _width: u16) {
        // For now, we don't implement deallocation as it's complex
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 149
  - For now
  - 

```rust
    /// Deallocates a rectangle (simplified - in practice this is complex)
    pub fn deallocate(&mut self, _x: u16, _y: u16, _width: u16) {
        // For now, we don't implement deallocation as it's complex
        // In a full implementation, you'd need to track allocated rectangles
        // and merge adjacent free spaces when deallocating
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tests in Source Directory


### Line 188: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/image_cache/atlas.rs` (line 188)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use super::*;

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 192: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/image_cache/atlas.rs` (line 192)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_basic_allocation() {
        let mut atlas = AtlasAllocator::new(100, 100);

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 209: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/image_cache/atlas.rs` (line 209)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_new_shelf_creation() {
        let mut atlas = AtlasAllocator::new(100, 100);

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 221: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/image_cache/atlas.rs` (line 221)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_height_matching() {
        let mut atlas = AtlasAllocator::new(100, 100);

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 238: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/image_cache/atlas.rs` (line 238)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_oversized_allocation() {
        let mut atlas = AtlasAllocator::new(100, 100);

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 251: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/image_cache/atlas.rs` (line 251)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_atlas_full() {
        let mut atlas = AtlasAllocator::new(20, 20);

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 264: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/image_cache/atlas.rs` (line 264)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_clear() {
        let mut atlas = AtlasAllocator::new(100, 100);

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 279: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/image_cache/atlas.rs` (line 279)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_utilization_stats() {
        let mut atlas = AtlasAllocator::new(100, 100);

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym