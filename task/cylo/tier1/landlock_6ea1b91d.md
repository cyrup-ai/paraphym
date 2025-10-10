# `packages/cylo/src/backends/landlock.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: cylo
- **File Hash**: 6ea1b91d  
- **Timestamp**: 2025-10-10T02:15:57.752906+00:00  
- **Lines of Code**: 704

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 704 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 915
  - stubby method name
  - temp_dir

```rust
    fn jail_path_validation() {
        // Valid absolute path should pass
        let temp_dir = std::env::temp_dir().join("cylo_test_jail");
        assert!(LandLockBackend::validate_jail_path(&temp_dir).is_ok());
        let _ = fs::remove_dir_all(&temp_dir);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 962
  - stubby method name
  - temp_dir

```rust
    fn backend_creation() {
        let config = BackendConfig::new("test_landlock");
        let temp_jail = std::env::temp_dir().join("cylo_test_jail");

        let result = LandLockBackend::new(temp_jail.display().to_string(), config);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 985
  - stubby method name
  - temp_dir

```rust
    fn supported_languages() {
        let config = BackendConfig::new("test");
        let temp_jail = std::env::temp_dir().join("cylo_test_jail2");

        if let Ok(backend) = LandLockBackend::new(temp_jail.display().to_string(), config) {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 915
  - stubby variable name
  - temp_dir

```rust
    fn jail_path_validation() {
        // Valid absolute path should pass
        let temp_dir = std::env::temp_dir().join("cylo_test_jail");
        assert!(LandLockBackend::validate_jail_path(&temp_dir).is_ok());
        let _ = fs::remove_dir_all(&temp_dir);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 916
  - stubby variable name
  - temp_dir

```rust
        // Valid absolute path should pass
        let temp_dir = std::env::temp_dir().join("cylo_test_jail");
        assert!(LandLockBackend::validate_jail_path(&temp_dir).is_ok());
        let _ = fs::remove_dir_all(&temp_dir);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 917
  - stubby variable name
  - temp_dir

```rust
        let temp_dir = std::env::temp_dir().join("cylo_test_jail");
        assert!(LandLockBackend::validate_jail_path(&temp_dir).is_ok());
        let _ = fs::remove_dir_all(&temp_dir);

        // Relative path should fail
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 962
  - stubby variable name
  - temp_jail

```rust
    fn backend_creation() {
        let config = BackendConfig::new("test_landlock");
        let temp_jail = std::env::temp_dir().join("cylo_test_jail");

        let result = LandLockBackend::new(temp_jail.display().to_string(), config);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 964
  - stubby variable name
  - temp_jail

```rust
        let temp_jail = std::env::temp_dir().join("cylo_test_jail");

        let result = LandLockBackend::new(temp_jail.display().to_string(), config);

        #[cfg(target_os = "linux")]
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 979
  - stubby variable name
  - temp_jail

```rust
        }

        let _ = fs::remove_dir_all(&temp_jail);
    }

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 985
  - stubby variable name
  - temp_jail

```rust
    fn supported_languages() {
        let config = BackendConfig::new("test");
        let temp_jail = std::env::temp_dir().join("cylo_test_jail2");

        if let Ok(backend) = LandLockBackend::new(temp_jail.display().to_string(), config) {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 987
  - stubby variable name
  - temp_jail

```rust
        let temp_jail = std::env::temp_dir().join("cylo_test_jail2");

        if let Ok(backend) = LandLockBackend::new(temp_jail.display().to_string(), config) {
            assert!(backend.supports_language("python"));
            assert!(backend.supports_language("rust"));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 994
  - stubby variable name
  - temp_jail

```rust
        }

        let _ = fs::remove_dir_all(&temp_jail);
    }
}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 928: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let exec_dir = PathBuf::from("/tmp/test");

        let (prog, args) = LandLockBackend::prepare_execution_command("python", &exec_dir).unwrap();
        assert_eq!(prog, "python3");
        assert_eq!(args, vec!["main.py"]);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 932: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(args, vec!["main.py"]);

        let (prog, args) = LandLockBackend::prepare_execution_command("rust", &exec_dir).unwrap();
        assert_eq!(prog, "bash");
        assert!(args[1].contains("rustc"));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 945: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert!(features.is_ok());

        let features = features.unwrap();
        #[cfg(target_os = "linux")]
        {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 908: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/landlock.rs` (line 908)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backends::BackendConfig;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 913: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/landlock.rs` (line 913)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn jail_path_validation() {
        // Valid absolute path should pass
        let temp_dir = std::env::temp_dir().join("cylo_test_jail");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 925: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/landlock.rs` (line 925)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn execution_command_preparation() {
        let exec_dir = PathBuf::from("/tmp/test");

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 941: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/landlock.rs` (line 941)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn landlock_feature_detection() {
        let features = LandLockBackend::detect_landlock_features();
        assert!(features.is_ok());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 960: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/landlock.rs` (line 960)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn backend_creation() {
        let config = BackendConfig::new("test_landlock");
        let temp_jail = std::env::temp_dir().join("cylo_test_jail");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 983: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/landlock.rs` (line 983)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn supported_languages() {
        let config = BackendConfig::new("test");
        let temp_jail = std::env::temp_dir().join("cylo_test_jail2");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `get_disk_read_stats()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/landlock.rs` (line 739)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

#[cfg(not(target_os = "linux"))]
fn get_disk_read_stats(_pid: u32) -> Result<u64, std::io::Error> {
    Ok(0)
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `get_disk_io_stats()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/landlock.rs` (line 708)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

#[cfg(not(target_os = "linux"))]
fn get_disk_io_stats(_pid: u32) -> Result<u64, std::io::Error> {
    Ok(0)
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `get_process_cpu_time()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/landlock.rs` (line 635)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

#[cfg(not(target_os = "linux"))]
fn get_process_cpu_time(_pid: u32) -> Result<u64, std::io::Error> {
    Ok(0)
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `get_memory_usage()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/landlock.rs` (line 771)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

#[cfg(not(target_os = "linux"))]
fn get_memory_usage(_pid: u32) -> Result<u64, std::io::Error> {
    Ok(0)
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym