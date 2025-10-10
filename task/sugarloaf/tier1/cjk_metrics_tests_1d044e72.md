# `packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: sugarloaf
- **File Hash**: 1d044e72  
- **Timestamp**: 2025-10-10T02:15:59.424195+00:00  
- **Lines of Code**: 783

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 783 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 309
  - In practice
  - 

```rust
        // primary font's cell dimensions. This is expected behavior - the underline
        // calculation is font-specific but constrained by primary font's cell size.
        // In practice, the renderer would clamp this to reasonable bounds.

        // Verify that both fonts use the same cell dimensions (the key requirement)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 406
  - actual
  - 

```rust

        // Line height: 11.7 + 2.3 + (0.9 * 2.0) = 16.8, ceiled to 17
        // But let's check the actual calculation
        let expected_height =
            (font.ascent + font.descent + (font.line_gap * 2.0)).ceil() as u32;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 489
  - fallback
  - 

```rust
        };

        // CJK font (fallback font that was causing issues)
        let cjk_font = FaceMetrics {
            cell_width: 18.0, // Double width
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 530
  - actual
  - 

```rust
        // Verify that the terminal can calculate scroll distances correctly
        // Before the fix: terminal would assume 4 Latin lines = 4 * latin_height
        // But actual content height would be different due to CJK line heights
        let lines_to_scroll = 4;
        let expected_scroll_distance = lines_to_scroll * latin_metrics.cell_height;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 582
  - actual
  - 

```rust
        // Simulate the scenario: terminal displays long CJK text and needs to scroll
        // The issue was that terminal would calculate scroll based on Latin line height
        // but actual content would have different height due to CJK metrics

        let terminal_rows = 24; // Typical terminal height
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 653
  - actual
  - 

```rust

        // Before fix: terminal would calculate scroll distance based on Latin metrics
        // but actual content height would be based on CJK metrics, causing mismatch
        let expected_scroll_distance = printf_output_lines * latin_metrics.cell_height;
        let actual_content_height = printf_output_lines * cjk_metrics.cell_height;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 773
  - fallback
  - 

```rust
        };

        // CJK fallback font (causing the original issue)
        let cjk_font = FaceMetrics {
            cell_width: 20.0,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 745: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        );

        let adjustment_ratio = size_adjustment.unwrap();
        // Should use ic_width ratio: 18.0 / 36.0 = 0.5
        assert!((adjustment_ratio - 0.5).abs() < 0.001);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 13: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 13)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use crate::font::metrics::{FaceMetrics, Metrics};

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 139: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 139)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_cjk_font_consistency_with_latin_primary() {
        let latin_font = TestFontData::cascadia_code();
        let cjk_fonts = vec![
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 197: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 197)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_multiple_latin_fonts_consistency() {
        let fonts = vec![
            TestFontData::cascadia_code(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 228: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 228)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_cjk_primary_font_scenario() {
        // Test scenario where CJK font is the primary font
        let cjk_font = TestFontData::noto_sans_cjk();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 251: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 251)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_rich_text_format_consistency() {
        let latin_font = TestFontData::cascadia_code();
        let cjk_font = TestFontData::noto_sans_cjk();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 293: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 293)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_underline_positioning_across_fonts() {
        let latin_font = TestFontData::cascadia_code();
        let cjk_font = TestFontData::noto_sans_cjk();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 318: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 318)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_strikethrough_positioning_across_fonts() {
        let latin_font = TestFontData::cascadia_code();
        let cjk_font = TestFontData::noto_sans_cjk();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 340: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 340)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_extreme_font_metrics() {
        // Test with extreme font metrics to ensure robustness
        let tiny_font = FaceMetrics {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 387: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 387)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_line_height_calculation_precision() {
        // Test that line height calculation maintains precision correctly
        let font = FaceMetrics {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 416: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 416)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_baseline_consistency_across_font_combinations() {
        let fonts = vec![
            TestFontData::cascadia_code(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 472: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 472)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    /// causing incorrect scrolling calculations
    #[test]
    fn test_issue_1071_cjk_latin_line_height_consistency() {
        // Simulate the scenario described in issue #1071
        // Latin font (like the user's primary font)
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 545: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 545)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    /// "after a long CJK text was output in rio terminal, it run into a strange status"
    #[test]
    fn test_issue_1071_long_cjk_text_scrolling() {
        // Simulate fonts similar to those that would cause the issue
        let primary_font = FaceMetrics {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 615: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 615)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    /// Test the printf scenario specifically mentioned in the issue
    #[test]
    fn test_issue_1071_printf_command_scrolling() {
        // The issue mentioned: "terminal scrolls less than 4 lines down after the printf command executed"
        let latin_font = FaceMetrics {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 671: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 671)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    /// Test baseline adjustment functionality that helps with the scrolling issue
    #[test]
    fn test_issue_1071_baseline_adjustment_consistency() {
        let font = FaceMetrics {
            cell_width: 10.0,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 706: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 706)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    /// Test CJK font size adjustment that prevents the scrolling issue
    #[test]
    fn test_issue_1071_cjk_font_size_normalization() {
        // Test the font size adjustment that normalizes CJK fonts with Latin fonts
        let latin_font = FaceMetrics {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 755: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 755)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    /// Integration test that verifies the complete fix for issue #1071
    #[test]
    fn test_issue_1071_complete_fix_integration() {
        // This test simulates the complete scenario described in issue #1071

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 834: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 834)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    /// Test edge case: mixed content with alternating Latin and CJK characters
    #[test]
    fn test_issue_1071_mixed_content_consistency() {
        let latin_font = FaceMetrics {
            cell_width: 8.0,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 891: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 891)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    /// Test edge cases for font metrics calculations
    #[test]
    fn test_edge_cases() {
        // Test extremely small font size
        let tiny_font = FaceMetrics {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 982: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font/cjk_metrics_tests.rs` (line 982)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    /// Test mixed script rendering (Latin + CJK + Emoji)
    #[test]
    fn test_mixed_script_rendering() {
        let latin_font = TestFontData::cascadia_code();
        let cjk_font = TestFontData::noto_sans_cjk();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym