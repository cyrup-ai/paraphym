# `packages/sweetmcp/packages/daemon/src/install/macos.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: daemon
- **File Hash**: e1f9a5a5  
- **Timestamp**: 2025-10-10T02:15:59.686967+00:00  
- **Lines of Code**: 593

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 593 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 155
  - stubby method name
  - temp_dir

```rust
        };

        let helper_dir = std::env::temp_dir()
            .join("sweetmcp_helper")
            .join(format!("v{:016x}", version_hash));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 27
  - stubby variable name
  - temp_path

```rust

        // First, copy the binary to /tmp so elevated context can access it
        let temp_path = format!("/tmp/{}", b.label);
        std::fs::copy(&b.program, &temp_path)
            .map_err(|e| InstallerError::System(format!("Failed to copy binary to temp: {}", e)))?;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 28
  - stubby variable name
  - temp_path

```rust
        // First, copy the binary to /tmp so elevated context can access it
        let temp_path = format!("/tmp/{}", b.label);
        std::fs::copy(&b.program, &temp_path)
            .map_err(|e| InstallerError::System(format!("Failed to copy binary to temp: {}", e)))?;

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 42
  - stubby variable name
  - temp_path

```rust

        let cp_cmd =
            CommandBuilder::new("cp").args([&temp_path, &format!("/usr/local/bin/{}", b.label)]);

        let chown_cmd = CommandBuilder::new("chown")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 50
  - stubby variable name
  - temp_path

```rust
            CommandBuilder::new("chmod").args(["755", &format!("/usr/local/bin/{}", b.label)]);

        let rm_cmd = CommandBuilder::new("rm").args(["-f", &temp_path]);

        // Write files to temp location first, then move them in elevated context
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 53
  - stubby variable name
  - temp_plist

```rust

        // Write files to temp location first, then move them in elevated context
        let temp_plist = format!("/tmp/{}.plist", b.label);
        std::fs::write(&temp_plist, &plist_content)
            .map_err(|e| InstallerError::System(format!("Failed to write temp plist: {}", e)))?;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 54
  - stubby variable name
  - temp_plist

```rust
        // Write files to temp location first, then move them in elevated context
        let temp_plist = format!("/tmp/{}.plist", b.label);
        std::fs::write(&temp_plist, &plist_content)
            .map_err(|e| InstallerError::System(format!("Failed to write temp plist: {}", e)))?;

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 64
  - stubby variable name
  - temp_plist

```rust
        script.push_str(&format!(" && {}", Self::command_to_script(&chmod_cmd)));
        script.push_str(&format!(" && {}", Self::command_to_script(&rm_cmd)));
        script.push_str(&format!(" && mv {} {}", temp_plist, plist_file));

        // Set plist permissions
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 93
  - stubby variable name
  - temp_service

```rust

                // Write service file to temp first
                let temp_service = format!("/tmp/{}.toml", service.name);
                std::fs::write(&temp_service, &service_toml).map_err(|e| {
                    InstallerError::System(format!("Failed to write temp service: {}", e))
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 94
  - stubby variable name
  - temp_service

```rust
                // Write service file to temp first
                let temp_service = format!("/tmp/{}.toml", service.name);
                std::fs::write(&temp_service, &service_toml).map_err(|e| {
                    InstallerError::System(format!("Failed to write temp service: {}", e))
                })?;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 99
  - stubby variable name
  - temp_service

```rust

                let service_file = format!("/etc/cyrupd/services/{}.toml", service.name);
                script.push_str(&format!(" && mv {} {}", temp_service, service_file));

                // Set service file permissions using CommandBuilder
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 155
  - stubby variable name
  - temp_dir

```rust
        };

        let helper_dir = std::env::temp_dir()
            .join("sweetmcp_helper")
            .join(format!("v{:016x}", version_hash));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 680: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let mut buf = Vec::new();
        plist::to_writer_xml(&mut buf, &Value::Dictionary(plist.into_iter().collect()))
            .expect("plist generation failed");
        String::from_utf8(buf).expect("valid utf8")
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 681: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        plist::to_writer_xml(&mut buf, &Value::Dictionary(plist.into_iter().collect()))
            .expect("plist generation failed");
        String::from_utf8(buf).expect("valid utf8")
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym