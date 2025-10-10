# `packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: rio-backend
- **File Hash**: ed4475bc  
- **Timestamp**: 2025-10-10T02:15:59.534518+00:00  
- **Lines of Code**: 643

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 643 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 3 Evaluations


- Line 238
  - Fallback
  - 

```rust
        }

        // Fallback to the next non-empty cell.
        let mut line = pos.row;
        occupied.unwrap_or_else(|| loop {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tests in Source Directory


### Line 410: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 410)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crosswords::pos::{Column, Line};
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 430: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 430)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn motion_simple() {
        let mut term = term();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 449: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 449)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn simple_wide() {
        let mut term = term();
        term.grid[Line(0)][Column(0)].c = 'a';
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 470: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 470)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn motion_start_end() {
        let mut term = term();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 483: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 483)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn motion_first_occupied() {
        let mut term = term();
        term.grid[Line(0)][Column(0)].c = ' ';
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 504: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 504)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn motion_high_middle_low() {
        let mut term = term();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 520: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 520)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn motion_bracket() {
        let mut term = term();
        term.grid[Line(0)][Column(0)].c = '(';
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 559: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 559)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn motion_semantic_right_end() {
        let mut term = motion_semantic_term();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 587: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 587)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn motion_semantic_left_start() {
        let mut term = motion_semantic_term();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 615: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 615)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn motion_semantic_right_start() {
        let mut term = motion_semantic_term();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 643: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 643)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn motion_semantic_left_end() {
        let mut term = motion_semantic_term();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 671: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 671)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn scroll_semantic() {
        let mut term = term();
        term.grid.scroll_up(&(Line(0)..Line(20)), 5);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 695: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 695)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn semantic_wide() {
        let mut term = term();
        term.grid[Line(0)][Column(0)].c = 'a';
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 718: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 718)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn motion_word() {
        let mut term = term();
        term.grid[Line(0)][Column(0)].c = 'a';
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 749: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 749)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn scroll_word() {
        let mut term = term();
        term.grid.scroll_up(&(Line(0)..Line(20)), 5);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 773: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 773)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn word_wide() {
        let mut term = term();
        term.grid[Line(0)][Column(0)].c = 'a';
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 796: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 796)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn scroll_simple() {
        let mut term = term();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 817: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 817)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn scroll_over_top() {
        let mut term = term();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 841: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/crosswords/vi_mode.rs` (line 841)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn scroll_over_bottom() {
        let mut term = term();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym