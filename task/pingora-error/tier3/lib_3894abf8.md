# `forks/pingora/pingora-error/src/lib.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-error
- **File Hash**: 3894abf8  
- **Timestamp**: 2025-10-10T02:16:01.256381+00:00  
- **Lines of Code**: 483

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 483 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tests in Source Directory


### Line 588: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-error/src/lib.rs` (line 588)
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
  


### Line 592: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-error/src/lib.rs` (line 592)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_chain_of_error() {
        let e1 = Error::new(ErrorType::InternalError);
        let mut e2 = Error::new(ErrorType::HTTPStatus(400));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 609: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-error/src/lib.rs` (line 609)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_error_context() {
        let mut e1 = Error::new(ErrorType::InternalError);
        e1.set_context(format!("{} {}", "my", "context"));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 616: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-error/src/lib.rs` (line 616)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_context_trait() {
        let e1: Result<(), BError> = Err(Error::new(ErrorType::InternalError));
        let e2 = e1.err_context(|| "another");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 626: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-error/src/lib.rs` (line 626)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_cause_trait() {
        let e1: Result<(), BError> = Err(Error::new(ErrorType::InternalError));
        let e2 = e1.or_err(ErrorType::HTTPStatus(400), "another");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 636: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-error/src/lib.rs` (line 636)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_option_some_ok() {
        let m = Some(2);
        let o = m.or_err(ErrorType::InternalError, "some is not an error!");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 646: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-error/src/lib.rs` (line 646)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_option_none_err() {
        let m: Option<i32> = None;
        let e1 = m.or_err(ErrorType::InternalError, "none is an error!");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 662: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-error/src/lib.rs` (line 662)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_into() {
        fn other_error() -> Result<(), &'static str> {
            Err("oops")
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym