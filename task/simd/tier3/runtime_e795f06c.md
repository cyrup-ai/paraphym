# `packages/simd/src/runtime.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: simd
- **File Hash**: e795f06c  
- **Timestamp**: 2025-10-10T02:15:58.222921+00:00  
- **Lines of Code**: 333

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 333 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 3 Evaluations


- Line 123
  - fallback
  - 

```rust
    /// ARM NEON optimized temperature scaling function
    pub neon: Option<TemperatureScaleFn>,
    /// Scalar fallback temperature scaling function
    pub scalar: TemperatureScaleFn,
}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 137
  - fallback
  - 

```rust
    /// ARM NEON optimized softmax function
    pub neon: Option<SoftmaxFn>,
    /// Scalar fallback softmax function
    pub scalar: SoftmaxFn,
}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 151
  - fallback
  - 

```rust
    /// ARM NEON optimized argmax function
    pub neon: Option<ArgmaxFn>,
    /// Scalar fallback argmax function
    pub scalar: ArgmaxFn,
}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tests in Source Directory


### Line 373: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/runtime.rs` (line 373)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
/// Force CPU feature detection (for testing)
#[cfg(test)]
pub fn force_feature_detection() -> CpuFeatures {
    CPU_FEATURES.store(0xFF, Ordering::Relaxed);
    get_cpu_features()
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 379: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/runtime.rs` (line 379)
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
  


### Line 383: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/runtime.rs` (line 383)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_cpu_feature_detection() {
        let features = get_cpu_features();
        // Should detect some valid feature set
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 397: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/runtime.rs` (line 397)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_vector_width() {
        assert_eq!(CpuFeatures::Scalar.vector_width(), 1);
        assert_eq!(CpuFeatures::Neon.vector_width(), 4);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 406: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/runtime.rs` (line 406)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_should_use_simd() {
        // Small arrays should use scalar
        assert!(!should_use_simd(1));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 420: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/runtime.rs` (line 420)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_cpu_info() {
        let info = get_cpu_info();
        assert!(info.vector_width >= 1);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 428: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/runtime.rs` (line 428)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_caching() {
        // Multiple calls should return same result
        let features1 = get_cpu_features();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `get_optimal_chunk_size()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/runtime.rs` (line 329)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
#[inline]
#[must_use]
pub fn get_optimal_chunk_size() -> usize {
    get_cpu_features().chunk_size()
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `force_feature_detection()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/runtime.rs` (line 373)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Force CPU feature detection (for testing)
#[cfg(test)]
pub fn force_feature_detection() -> CpuFeatures {
    CPU_FEATURES.store(0xFF, Ordering::Relaxed);
    get_cpu_features()
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym