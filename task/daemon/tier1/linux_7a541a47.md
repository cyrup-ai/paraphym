# `packages/sweetmcp/packages/daemon/src/install/linux.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: daemon
- **File Hash**: 7a541a47  
- **Timestamp**: 2025-10-10T02:15:59.687809+00:00  
- **Lines of Code**: 480

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 480 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 130
  - stubby method name
  - temp_dir

```rust

        // Create unique helper path in temp directory
        let temp_dir = std::env::temp_dir();
        let helper_name = format!("sweetmcp-helper-{}", std::process::id());
        let helper_path = temp_dir.join(helper_name);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 130
  - stubby variable name
  - temp_dir

```rust

        // Create unique helper path in temp directory
        let temp_dir = std::env::temp_dir();
        let helper_name = format!("sweetmcp-helper-{}", std::process::id());
        let helper_path = temp_dir.join(helper_name);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 132
  - stubby variable name
  - temp_dir

```rust
        let temp_dir = std::env::temp_dir();
        let helper_name = format!("sweetmcp-helper-{}", std::process::id());
        let helper_path = temp_dir.join(helper_name);

        // Extract embedded helper executable
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 636
  - stubby variable name
  - temp_path

```rust
    /// Write file atomically to prevent corruption
    fn write_file_atomic(path: &Path, content: &str) -> Result<(), InstallerError> {
        let temp_path = path.with_extension("tmp");

        {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 639
  - stubby variable name
  - temp_path

```rust

        {
            let mut file = fs::File::create(&temp_path).map_err(|e| {
                InstallerError::System(format!("Failed to create temp file: {}", e))
            })?;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 650
  - stubby variable name
  - temp_path

```rust
        }

        fs::rename(&temp_path, path)
            .map_err(|e| InstallerError::System(format!("Failed to rename temp file: {}", e)))?;

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 237
  - hardcoded URL
  - 

```rust
        content.push_str("[Unit]\n");
        content.push_str(&format!("Description={}\n", config.description));
        content.push_str("Documentation=https://github.com/cyrup/sweetmcp\n");

        if config.wants_network {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym