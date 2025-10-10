# `packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: rioterm
- **File Hash**: 92fd122d  
- **Timestamp**: 2025-10-10T02:15:58.901811+00:00  
- **Lines of Code**: 1199

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 1199 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 2 (possible) Infractions 


- Line 1392
  - Mock
  - 

```rust
    // #[test]
    // fn binding_trigger_input() {
    //     let binding = MockBinding { trigger: 13, ..MockBinding::default() };

    //     let mods = binding.mods;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1403
  - Mock
  - 

```rust
    // #[test]
    // fn binding_trigger_mods() {
    //     let binding = MockBinding {
    //         mods: ModifiersState::ALT | ModifiersState::SUPER,
    //         ..MockBinding::default()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1405
  - Mock
  - 

```rust
    //     let binding = MockBinding {
    //         mods: ModifiersState::ALT | ModifiersState::SUPER,
    //         ..MockBinding::default()
    //     };

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 1149
  - actual
  - 

```rust
        "n", ModifiersState::CONTROL | ModifiersState::SHIFT; Action::WindowCreateNew;
        ",", ModifiersState::CONTROL | ModifiersState::SHIFT; Action::ConfigEditor;
        // This is actually a Windows Powershell shortcut
        // https://github.com/alacritty/alacritty/issues/2930
        // https://github.com/raphamorim/rio/issues/220#issuecomment-1761651339
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 273: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        }

        let re = regex::Regex::new(r"selecttab\(([^()]+)\)").unwrap();
        for capture in re.captures_iter(&action) {
            if let Some(matched) = capture.get(1) {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 282: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        }

        let re = regex::Regex::new(r"run\(([^()]+)\)").unwrap();
        for capture in re.captures_iter(&action) {
            if let Some(matched) = capture.get(1) {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 306: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        }

        let re = regex::Regex::new(r"scroll\(([^()]+)\)").unwrap();
        for capture in re.captures_iter(&action) {
            if let Some(matched) = capture.get(1) {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 974: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 974)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
// Macos
#[cfg(all(target_os = "macos", not(test)))]
pub fn platform_key_bindings(
    use_navigation_key_bindings: bool,
    use_splits: bool,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1065: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1065)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
// Not Windows, Macos
#[cfg(not(any(target_os = "macos", target_os = "windows", test)))]
pub fn platform_key_bindings(
    use_navigation_key_bindings: bool,
    use_splits: bool,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1129: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1129)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
// Windows
#[cfg(all(target_os = "windows", not(test)))]
pub fn platform_key_bindings(
    use_navigation_key_bindings: bool,
    use_splits: bool,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1199: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1199)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
pub fn platform_key_bindings(_: bool, _: bool, _: ConfigKeyboard) -> Vec<KeyBinding> {
    vec![]
}
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1204: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1204)
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
  


### Line 1224: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1224)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn binding_matches_itself() {
        let binding = MockBinding::default();
        let identical_binding = MockBinding::default();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1233: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1233)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn binding_matches_different_action() {
        let binding = MockBinding::default();
        let different_action = MockBinding {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1245: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1245)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn mods_binding_requires_strict_match() {
        let superset_mods = MockBinding {
            mods: ModifiersState::all(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1260: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1260)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn binding_matches_identical_mode() {
        let b1 = MockBinding {
            mode: BindingMode::ALT_SCREEN,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1275: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1275)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn binding_without_mode_matches_any_mode() {
        let b1 = MockBinding::default();
        let b2 = MockBinding {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1287: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1287)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn binding_with_mode_matches_empty_mode() {
        let b1 = MockBinding {
            mode: BindingMode::APP_KEYPAD,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1300: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1300)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn binding_matches_modes() {
        let b1 = MockBinding {
            mode: BindingMode::ALT_SCREEN | BindingMode::APP_KEYPAD,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1315: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1315)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn binding_matches_partial_intersection() {
        let b1 = MockBinding {
            mode: BindingMode::ALT_SCREEN | BindingMode::APP_KEYPAD,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1330: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1330)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn binding_mismatches_notmode() {
        let b1 = MockBinding {
            mode: BindingMode::ALT_SCREEN,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1345: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1345)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn binding_mismatches_unrelated() {
        let b1 = MockBinding {
            mode: BindingMode::ALT_SCREEN,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1360: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1360)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn binding_matches_notmodes() {
        let subset_notmodes = MockBinding {
            notmode: BindingMode::VI | BindingMode::APP_CURSOR,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1375: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1375)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn binding_matches_mode_notmode() {
        let b1 = MockBinding {
            mode: BindingMode::VI,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1420: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1420)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn binding_trigger_modes() {
        let binding = MockBinding {
            mode: BindingMode::ALT_SCREEN,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1439: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1439)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn binding_trigger_notmodes() {
        let binding = MockBinding {
            notmode: BindingMode::ALT_SCREEN,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1458: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1458)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn bindings_overwrite() {
        let bindings = bindings!(
            KeyBinding;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1480: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1480)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn bindings_conflict_resolution() {
        // Test that conflicting bindings are properly replaced
        let bindings = bindings!(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1519: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/frontends/rioterm/src/bindings/mod.rs` (line 1519)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn bindings_alt_enter_conflict_resolution() {
        // Test Windows Alt+Enter conflict resolution
        let bindings = bindings!(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym