# `packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: rio-backend
- **File Hash**: acaa7734  
- **Timestamp**: 2025-10-10T02:15:59.532399+00:00  
- **Lines of Code**: 3064

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 3064 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 455
  - hardcoded URL
  - 

```rust
        let term_colors = TermColors::default();
        // Regex used for the default URL hint.
        let _url_regex: &str = "(ipfs:|ipns:|magnet:|mailto:|gemini://|gopher://|https://|http://|news:|file:|git://|ssh:|ftp://)\
                         [^\u{0000}-\u{001F}\u{007F}-\u{009F}<>\"\\s{-}\\^⟨⟩`\\\\]+";

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 2 (possible) Infractions 


- Line 2148
  - placeholder
  - 

```rust
            if self.grid.cursor.pos.col + 1 >= columns {
                if self.mode.contains(Mode::LINE_WRAP) {
                    // Insert placeholder before wide char if glyph does not fit in this row.
                    self.grid
                        .cursor
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3876
  - mock
  - 

```rust

        // The report_keyboard_mode function sends an event through event_proxy
        // We can't easily test the exact output without mocking the event system,
        // but we can verify the current mode is correctly retrieved
        let current_mode = term.keyboard_mode_stack[term.keyboard_mode_idx];
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 253
  - actual
  - 

```rust
#[derive(Debug, Clone)]
struct TermDamageState {
    /// Hint whether terminal should be damaged entirely regardless of the actual damage changes.
    full: bool,

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 921
  - actual
  - 

```rust
    }

    /// Check if any rendering is actually needed
    #[inline]
    pub fn needs_render(&self) -> bool {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2577
  - Fallback
  - 

```rust
    #[inline]
    fn set_scrolling_region(&mut self, top: usize, bottom: Option<usize>) {
        // Fallback to the last line as default.
        let bottom = bottom.unwrap_or_else(|| self.grid.screen_lines());

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 3224
  - actual
  - 

```rust
        }

        // Verify the line was actually cleared
        let cursor_line = cw.grid.cursor.pos.row;
        for col in 0..test_text.len() {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tests in Source Directory


### Line 3064: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3064)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crosswords::pos::{Column, Line, Pos, Side};
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3071: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3071)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn scroll_up() {
        let size = CrosswordsSize::new(1, 10);
        let window_id = crate::event::WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3105: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3105)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_linefeed() {
        let size = CrosswordsSize::new(1, 1);
        let window_id = crate::event::WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3118: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3118)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_linefeed_moving_cursor() {
        let size = CrosswordsSize::new(1, 3);

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3145: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3145)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_input() {
        let size = CrosswordsSize::new(5, 10);
        let window_id = crate::event::WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3167: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3167)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_damage_tracking_after_control_c() {
        let size = CrosswordsSize::new(80, 24);
        let window_id = crate::event::WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3236: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3236)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_damage_tracking_cursor_movement() {
        let size = CrosswordsSize::new(80, 24);
        let window_id = crate::event::WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3266: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3266)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_damage_tracking_clear_operations() {
        let size = CrosswordsSize::new(80, 24);
        let window_id = crate::event::WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3310: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3310)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_damage_tracking_prompt_redraw() {
        let size = CrosswordsSize::new(80, 24);
        let window_id = crate::event::WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3358: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3358)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn simple_selection_works() {
        let size = CrosswordsSize::new(5, 5);
        let window_id = crate::event::WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3433: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3433)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn line_selection_works() {
        let size = CrosswordsSize::new(5, 1);
        let window_id = crate::event::WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3460: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3460)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn block_selection_works() {
        let size = CrosswordsSize::new(5, 5);
        let window_id = crate::event::WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3535: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3535)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn parse_cargo_version() {
        assert_eq!(version_number("0.0.1-nightly"), 1);
        assert_eq!(version_number("0.1.2-nightly"), 1_02);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3543: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3543)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_cursor_damage_after_clear() {
        use crate::ansi::CursorShape;
        use crate::crosswords::CrosswordsSize;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3621: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3621)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_line_damage_approach() {
        use crate::ansi::CursorShape;
        use crate::crosswords::CrosswordsSize;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3693: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3693)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_keyboard_mode_push_pop() {
        let size = CrosswordsSize::new(10, 10);
        let window_id = WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3734: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3734)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_keyboard_mode_stack_wraparound() {
        let size = CrosswordsSize::new(10, 10);
        let window_id = WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3759: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3759)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_keyboard_mode_pop_excessive() {
        let size = CrosswordsSize::new(10, 10);
        let window_id = WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3781: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3781)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_keyboard_mode_set_replace() {
        let size = CrosswordsSize::new(10, 10);
        let window_id = WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3811: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3811)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_keyboard_mode_set_union() {
        let size = CrosswordsSize::new(10, 10);
        let window_id = WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3837: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3837)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_keyboard_mode_set_difference() {
        let size = CrosswordsSize::new(10, 10);
        let window_id = WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3866: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3866)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_keyboard_mode_report() {
        let size = CrosswordsSize::new(10, 10);
        let window_id = WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3883: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3883)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_keyboard_mode_reset() {
        let size = CrosswordsSize::new(10, 10);
        let window_id = WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 3907: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/mod.rs` (line 3907)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_keyboard_mode_stack_underflow_protection() {
        let size = CrosswordsSize::new(10, 10);
        let window_id = WindowId::from(0);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym