# `packages/cylo/src/execution_env.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: cylo
- **File Hash**: 95862f59  
- **Timestamp**: 2025-10-10T02:15:57.755857+00:00  
- **Lines of Code**: 375

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 375 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tests in Source Directory


### Line 497: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/execution_env.rs` (line 497)
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
  


### Line 501: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/execution_env.rs` (line 501)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn cylo_landlock_creation() {
        let cylo = Cylo::LandLock("/tmp/sandbox".to_string());
        assert_eq!(cylo.backend_type(), "LandLock");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 508: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/execution_env.rs` (line 508)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn cylo_firecracker_creation() {
        let cylo = Cylo::FireCracker("rust:alpine3.20".to_string());
        assert_eq!(cylo.backend_type(), "FireCracker");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 515: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/execution_env.rs` (line 515)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn cylo_apple_creation() {
        let cylo = Cylo::Apple("python:alpine3.20".to_string());
        assert_eq!(cylo.backend_type(), "Apple");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 522: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/execution_env.rs` (line 522)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn instance_creation() {
        let instance = Cylo::Apple("python:alpine3.20".to_string()).instance("test_env");
        assert_eq!(instance.name, "test_env");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 530: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/execution_env.rs` (line 530)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn landlock_path_validation() {
        // Valid absolute path
        let valid = Cylo::LandLock("/tmp/sandbox".to_string());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 545: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/execution_env.rs` (line 545)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn image_validation() {
        // Valid image with tag
        let valid = Cylo::FireCracker("rust:alpine3.20".to_string());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 560: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/execution_env.rs` (line 560)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn instance_name_validation() {
        let env = Cylo::Apple("python:alpine3.20".to_string());

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 577: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/execution_env.rs` (line 577)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn display_formatting() {
        let cylo = Cylo::Apple("python:alpine3.20".to_string());
        let instance = cylo.instance("test_env");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `validate_instance_name()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/execution_env.rs` (line 390)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// * `Ok(())` if the name is valid
/// * `Err(CyloError)` if the name is invalid
pub fn validate_instance_name(name: &str) -> CyloResult<()> {
    if name.is_empty() {
        return Err(CyloError::validation("Instance name cannot be empty"));
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `validate_environment_spec()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/execution_env.rs` (line 445)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// * `Ok(())` if the environment specification is valid
/// * `Err(CyloError)` if the specification is invalid
pub fn validate_environment_spec(env: &Cylo) -> CyloResult<()> {
    match env {
        Cylo::LandLock(path) => {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym