# `packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/positioning_tests.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: sugarloaf
- **File Hash**: 2e78200e  
- **Timestamp**: 2025-10-10T02:15:59.424712+00:00  
- **Lines of Code**: 825

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 825 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 3 Evaluations


- Line 524
  - actual
  - 

```rust
            // Now that baseline = py - descent, the relationships are:
            // - glyph_y = py + padding_y (glyphs positioned relative to py)
            // - cursor baseline = py - descent (actual text baseline)
            let expected_glyph_baseline = glyph_y - with_cursor.padding_y; // This should be py
            let cursor_baseline_in_line_coords = with_cursor.baseline; // This should be py - descent
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 707
  - actual
  - 

```rust

            // Verify padding_y calculation is correct
            // The actual calculation is: if line_height_mod > 1.0 then (line_height - line_height_without_mod) / 2.0 else 0.0
            // where line_height = line_height_without_mod * line_height_mod
            // and line_height_without_mod = ascent + descent + leading (leading is usually 0)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 918
  - fallback
  - 

```rust
        }

        // Test with x-height fallback (no font strikeout offset)
        let style_with_x_height = TextRunStyle {
            strikeout_offset: 0.0, // No font-provided offset
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 935
  - fallback
  - 

```rust
        }

        // Test with final fallback (no font offset or x-height)
        let style_fallback = TextRunStyle {
            strikeout_offset: 0.0,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tests in Source Directory


### Line 198: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/positioning_tests.rs` (line 198)
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
  


### Line 202: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/positioning_tests.rs` (line 202)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_positioning_consistency_without_cursor() {
        let mut helper = PositioningTestHelper::new();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 283: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/positioning_tests.rs` (line 283)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_cursor_positioning_consistency() {
        let mut helper = PositioningTestHelper::new();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 354: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/positioning_tests.rs` (line 354)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_drawable_character_cached_vs_non_cached() {
        let mut helper = PositioningTestHelper::new();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 406: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/positioning_tests.rs` (line 406)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_padding_y_effects() {
        let mut helper = PositioningTestHelper::new();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 473: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/positioning_tests.rs` (line 473)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_glyph_vs_cursor_positioning_relationship() {
        let mut helper = PositioningTestHelper::new();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 546: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/positioning_tests.rs` (line 546)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_drawable_character_positioning() {
        let mut helper = PositioningTestHelper::new();
        let font_id = 0;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 594: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/positioning_tests.rs` (line 594)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_cursor_positioning_relationships() {
        let mut helper = PositioningTestHelper::new();
        let font_id = 0;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 671: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/positioning_tests.rs` (line 671)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_glyph_and_cursor_baseline_consistency() {
        let mut helper = PositioningTestHelper::new();
        let font_id = 0;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 724: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/positioning_tests.rs` (line 724)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_cached_vs_non_cached_positioning_consistency() {
        let mut helper = PositioningTestHelper::new();
        let font_id = 0;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 797: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/positioning_tests.rs` (line 797)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_positioning_invariants() {
        let mut helper = PositioningTestHelper::new();
        let font_id = 0;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 869: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/positioning_tests.rs` (line 869)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_strikethrough_positioning() {
        use crate::components::rich_text::compositor::Compositor;
        use crate::components::rich_text::text::TextRunStyle;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 954: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/positioning_tests.rs` (line 954)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_cursor_font_based_sizing() {
        // Test that cursor size is based on font metrics, not line height
        let _font_size = 16.0;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 988: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/positioning_tests.rs` (line 988)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_cursor_positioning_with_different_font_metrics() {
        let _font_size = 16.0;
        let _line_height = 24.0; // Larger than font size
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1040: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/components/rich_text/positioning_tests.rs` (line 1040)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_underline_cursor_positioning() {
        let _font_size = 16.0;
        let _line_height = 24.0;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym