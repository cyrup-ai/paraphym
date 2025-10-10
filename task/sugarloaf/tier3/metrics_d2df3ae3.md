# `packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/metrics.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: sugarloaf
- **File Hash**: d2df3ae3  
- **Timestamp**: 2025-10-10T02:15:59.425406+00:00  
- **Lines of Code**: 560

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 560 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 3 Evaluations


- Line 101
  - actual
  - 

```rust
            cap_height: Some(metrics.cap_height as f64),
            ex_height: Some(metrics.x_height as f64),
            // Measure actual CJK character width from the font
            ic_width: Self::measure_cjk_character_width(font_ref),
        }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 268
  - fallback
  - 

```rust
            .unwrap_or(primary_face.cell_width * 2.0);

        // Get fallback font metrics
        let fallback_ex = fallback_face.ex_height.unwrap_or(
            fallback_face
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 280
  - fallback
  - 

```rust
            .unwrap_or(fallback_face.cell_width * 2.0);

        // If the fallback font has an ic_width, prefer that for normalization
        // of CJK font sizes when mixed with Latin fonts
        if fallback_face.ic_width.is_some() && fallback_ic > 0.0 && primary_ic > 0.0 {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 286
  - Fall back
  - 

```rust
            Some(primary_ic / fallback_ic)
        } else if primary_ex > 0.0 && fallback_ex > 0.0 {
            // Fall back to ex_height ratio for general font size matching
            Some(primary_ex / fallback_ex)
        } else {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 557
  - fallback
  - 

```rust
        let metrics = Metrics::calc(face);

        // Should calculate fallback values without panicking
        assert!(metrics.underline_thickness >= 1);
        assert!(metrics.strikethrough_thickness >= 1);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 569
  - fallback
  - 

```rust
    #[test]
    fn test_mixed_font_scenario() {
        // Simulate a real-world scenario with Latin primary and CJK fallback
        let cascadia_face = FaceMetrics {
            cell_width: 9.6,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 691: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // Should use ic_width ratio: 16.0 / 32.0 = 0.5
        assert!(adjustment.is_some());
        let ratio = adjustment.unwrap();
        assert!((ratio - 0.5).abs() < 0.001);
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 768: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // Should use estimated ic_width (cell_width * 2) vs provided ic_width
        // (10.0 * 2.0) / 20.0 = 1.0
        let ratio = adjustment_estimated.unwrap();
        assert!((ratio - 1.0).abs() < 0.001);
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


### Line 335: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/metrics.rs` (line 335)
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
  


### Line 339: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/metrics.rs` (line 339)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_primary_font_metrics() {
        let primary_face = FaceMetrics {
            cell_width: 10.0,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 360: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/metrics.rs` (line 360)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_secondary_font_uses_primary_dimensions() {
        let primary_face = FaceMetrics {
            cell_width: 10.0,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 403: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/metrics.rs` (line 403)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_cjk_font_metrics_consistency() {
        // Test typical CJK font metrics (wider characters, different proportions)
        let latin_face = FaceMetrics {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 451: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/metrics.rs` (line 451)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_baseline_calculation_rio_format() {
        // Test that baseline calculation works correctly with Rio's positive descent
        let face = FaceMetrics {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 483: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/metrics.rs` (line 483)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_underline_positioning() {
        let face = FaceMetrics {
            cell_width: 10.0,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 514: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/metrics.rs` (line 514)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_strikethrough_positioning() {
        let face = FaceMetrics {
            cell_width: 10.0,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 539: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/metrics.rs` (line 539)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_fallback_values() {
        // Test metrics calculation with missing optional values
        let face = FaceMetrics {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 568: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/metrics.rs` (line 568)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_mixed_font_scenario() {
        // Simulate a real-world scenario with Latin primary and CJK fallback
        let cascadia_face = FaceMetrics {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 615: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/metrics.rs` (line 615)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_line_height_calculation_edge_cases() {
        // Test with zero line gap
        let face_no_gap = FaceMetrics {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 656: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/metrics.rs` (line 656)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_cjk_font_size_adjustment() {
        // Test CJK font size adjustment calculation
        let latin_face = FaceMetrics {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 696: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/metrics.rs` (line 696)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_baseline_adjustment() {
        let face = FaceMetrics {
            cell_width: 10.0,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 731: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/metrics.rs` (line 731)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_cjk_character_width_estimation() {
        // Test that ic_width is properly estimated when not provided
        let face_without_ic = FaceMetrics {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 773: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/metrics.rs` (line 773)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_rio_style_baseline_calculation() {
        // Test that baseline calculation matches Rio's approach
        // Rio: cell_baseline = half_line_gap + descent (with positive descent)
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym