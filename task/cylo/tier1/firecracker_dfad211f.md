# `packages/cylo/src/backends/firecracker.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: cylo
- **File Hash**: dfad211f  
- **Timestamp**: 2025-10-10T02:15:57.748705+00:00  
- **Lines of Code**: 1174

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 1174 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 726
  - stubby method name
  - temp_dir

```rust
        );

        let socket_path = std::env::temp_dir().join(format!("{}.sock", vm_id));
        let config_path = std::env::temp_dir().join(format!("{}.json", vm_id));

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 727
  - stubby method name
  - temp_dir

```rust

        let socket_path = std::env::temp_dir().join(format!("{}.sock", vm_id));
        let config_path = std::env::temp_dir().join(format!("{}.json", vm_id));

        // Build SSH config from backend_specific
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1483
  - stubby method name
  - temp_dir

```rust

            // Clean up temporary files
            if let Ok(entries) = fs::read_dir(std::env::temp_dir()) {
                for entry in entries.filter_map(Result::ok) {
                    if let Ok(file_name) = entry.file_name().into_string() {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 726
  - stubby variable name
  - temp_dir

```rust
        );

        let socket_path = std::env::temp_dir().join(format!("{}.sock", vm_id));
        let config_path = std::env::temp_dir().join(format!("{}.json", vm_id));

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 727
  - stubby variable name
  - temp_dir

```rust

        let socket_path = std::env::temp_dir().join(format!("{}.sock", vm_id));
        let config_path = std::env::temp_dir().join(format!("{}.json", vm_id));

        // Build SSH config from backend_specific
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1483
  - stubby variable name
  - temp_dir

```rust

            // Clean up temporary files
            if let Ok(entries) = fs::read_dir(std::env::temp_dir()) {
                for entry in entries.filter_map(Result::ok) {
                    if let Ok(file_name) = entry.file_name().into_string() {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 219
  - hardcoded URL
  - 

```rust

        let request = HttpRequest::put(
            &format!("http://unix:{}/machine-config", self.socket_path.display()),
            request_body,
        )
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 263
  - hardcoded URL
  - 

```rust

        let request = HttpRequest::put(
            &format!("http://unix:{}/actions", self.socket_path.display()),
            serde_json::to_vec(&serde_json::json!({
                "action_type": "InstanceStart"
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 311
  - hardcoded URL
  - 

```rust

        let request = HttpRequest::put(
            &format!("http://unix:{}/actions", self.socket_path.display()),
            serde_json::to_vec(&serde_json::json!({
                "action_type": "SendCtrlAltDel"
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 359
  - hardcoded URL
  - 

```rust

        let request = HttpRequest::get(&format!(
            "http://unix:{}/metrics",
            self.socket_path.display()
        ))
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 735
  - hardcoded IP address
  - 

```rust
                .get("ssh_host")
                .cloned()
                .unwrap_or_else(|| "172.16.0.2".to_string());
            let port = backend_config
                .backend_specific
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 885
  - hardcoded URL
  - 

```rust
            let boot_request = HttpRequest::put(
                &format!(
                    "http://unix:{}/boot-source",
                    vm_with_pid.socket_path.display()
                ),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 917
  - hardcoded URL
  - 

```rust
            let rootfs_request = HttpRequest::put(
                &format!(
                    "http://unix:{}/drives/rootfs",
                    vm_with_pid.socket_path.display()
                ),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 950
  - hardcoded URL
  - 

```rust
                let network_request = HttpRequest::put(
                    &format!(
                        "http://unix:{}/network-interfaces/eth0",
                        vm_with_pid.socket_path.display()
                    ),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 1573: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .with_config("vcpu_count", "2");

        let fc_config = FireCrackerBackend::init_firecracker_config(&config).unwrap();
        assert_eq!(fc_config.memory_size_mb, 1024);
        assert_eq!(fc_config.vcpu_count, 2);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1543: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
    fn execution_script_preparation() {
        let request = ExecutionRequest::new("print('hello')", "python");
        let script = FireCrackerBackend::prepare_execution_script(&request).expect("Failed to prepare Python execution script");
        assert!(script.contains("python3"));
        assert!(script.contains("print('hello')"));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1548: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust

        let request = ExecutionRequest::new("console.log('hello')", "javascript");
        let script = FireCrackerBackend::prepare_execution_script(&request).expect("Failed to prepare JavaScript execution script");
        assert!(script.contains("node"));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1559: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let request = ExecutionRequest::new("test", "python");
        let backend_config = BackendConfig::default();
        let vm = FireCrackerBackend::create_vm_instance(&request, &backend_config).expect("Failed to create VM instance for test");

        assert!(vm.vm_id.starts_with("cylo-"));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 1525: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/firecracker.rs` (line 1525)
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
  


### Line 1530: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/firecracker.rs` (line 1530)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn image_format_validation() {
        assert!(FireCrackerBackend::is_valid_image_format("python:3.11"));
        assert!(FireCrackerBackend::is_valid_image_format("rust:alpine3.20"));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1541: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/firecracker.rs` (line 1541)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn execution_script_preparation() {
        let request = ExecutionRequest::new("print('hello')", "python");
        let script = FireCrackerBackend::prepare_execution_script(&request).expect("Failed to prepare Python execution script");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1556: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/firecracker.rs` (line 1556)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn vm_instance_creation() {
        let request = ExecutionRequest::new("test", "python");
        let backend_config = BackendConfig::default();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1568: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/firecracker.rs` (line 1568)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn firecracker_config_initialization() {
        let config = BackendConfig::new("test_firecracker")
            .with_config("memory_size_mb", "1024")
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1579: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/firecracker.rs` (line 1579)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn backend_creation() {
        let config = BackendConfig::new("test_firecracker");

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1592: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/cylo/src/backends/firecracker.rs` (line 1592)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn supported_languages() {
        let config = BackendConfig::new("test");
        if let Ok(backend) = FireCrackerBackend::new("python:3.11".to_string(), config) {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym