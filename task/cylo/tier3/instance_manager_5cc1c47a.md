# `packages/cylo/src/instance_manager.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: cylo
- **File Hash**: 5cc1c47a  
- **Timestamp**: 2025-10-10T02:15:57.753785+00:00  
- **Lines of Code**: 450

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 450 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Panic-Prone Code


### Line 585: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let instances = manager
            .list_instances()
            .expect("Failed to list instances in test");
        assert!(instances.is_empty());
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 648: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let initial_list = manager
            .list_instances()
            .expect("Failed to get initial instance list in test");
        assert!(initial_list.is_empty());

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 660: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            let updated_list = manager
                .list_instances()
                .expect("Failed to get updated instance list in test");
            assert!(updated_list.contains(&instance.id()));
        }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 674: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            .await
            .expect("Failed to join async task in test")
            .expect("Failed to check health of all instances in test");
        assert!(health_results.is_empty());
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 673: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            .health_check_all()
            .await
            .expect("Failed to join async task in test")
            .expect("Failed to check health of all instances in test");
        assert!(health_results.is_empty());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 686: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            .await
            .expect("Failed to join async task in test")
            .expect("Failed to cleanup idle instances in test");
        assert_eq!(cleaned_count, 0);
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 685: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            .cleanup_idle_instances()
            .await
            .expect("Failed to join async task in test")
            .expect("Failed to cleanup idle instances in test");
        assert_eq!(cleaned_count, 0);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 703: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let instances = manager
            .list_instances()
            .expect("Failed to list instances from global manager in test");
        assert!(instances.is_empty());
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


### Line 572: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/instance_manager.rs` (line 572)
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
  


### Line 580: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/instance_manager.rs` (line 580)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn instance_manager_creation() {
        let manager = InstanceManager::new();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 590: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/instance_manager.rs` (line 590)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn instance_registration_and_retrieval() {
        let manager = InstanceManager::new();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 622: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/instance_manager.rs` (line 622)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn instance_not_found() {
        let manager = InstanceManager::new();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 643: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/instance_manager.rs` (line 643)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn instance_list() {
        let manager = InstanceManager::new();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 667: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/instance_manager.rs` (line 667)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn health_check_all() {
        let manager = InstanceManager::new();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 679: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/instance_manager.rs` (line 679)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn cleanup_idle_instances() {
        let manager = InstanceManager::new();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 691: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/instance_manager.rs` (line 691)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn shutdown() {
        let manager = InstanceManager::new();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 699: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/instance_manager.rs` (line 699)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn global_instance_manager_access() {
        let manager = global_instance_manager();
        let instances = manager
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 708: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/instance_manager.rs` (line 708)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn custom_configuration() {
        let config = BackendConfig::new("custom").with_timeout(Duration::from_secs(120));
        let manager =
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `init_global_instance_manager()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/instance_manager.rs` (line 559)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// # Returns
/// Result indicating success or if already initialized
pub fn init_global_instance_manager(
    config: BackendConfig,
    health_check_interval: Duration,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym