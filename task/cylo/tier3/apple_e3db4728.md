# `packages/cylo/src/backends/apple.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: cylo
- **File Hash**: e3db4728  
- **Timestamp**: 2025-10-10T02:15:57.755051+00:00  
- **Lines of Code**: 432

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 432 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Panic-Prone Code


### Line 573: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn execution_command_preparation() {
        let python_cmd =
            AppleBackend::prepare_execution_command("python", "print('hello')").unwrap();
        assert_eq!(python_cmd, vec!["python3", "-c", "print('hello')"]);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 577: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let js_cmd =
            AppleBackend::prepare_execution_command("javascript", "console.log('hello')").unwrap();
        assert_eq!(js_cmd, vec!["node", "-e", "console.log('hello')"]);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 580: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(js_cmd, vec!["node", "-e", "console.log('hello')"]);

        let bash_cmd = AppleBackend::prepare_execution_command("bash", "echo hello").unwrap();
        assert_eq!(bash_cmd, vec!["sh", "-c", "echo hello"]);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 548: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/apple.rs` (line 548)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use std::time::Duration;

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 555: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/apple.rs` (line 555)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn image_format_validation() {
        assert!(AppleBackend::is_valid_image_format("python:3.11"));
        assert!(AppleBackend::is_valid_image_format("rust:alpine3.20"));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 571: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/apple.rs` (line 571)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn execution_command_preparation() {
        let python_cmd =
            AppleBackend::prepare_execution_command("python", "print('hello')").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 588: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/apple.rs` (line 588)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn backend_creation() {
        let config = BackendConfig::new("test_apple").with_timeout(Duration::from_secs(60));

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 601: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/apple.rs` (line 601)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn supported_languages() {
        let config = BackendConfig::new("test");
        if let Ok(backend) = AppleBackend::new("python:3.11".to_string(), config) {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym