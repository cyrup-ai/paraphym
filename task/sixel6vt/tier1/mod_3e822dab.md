# `packages/sweetmcp/packages/sixel6vt/packages/sixel6vt/src/bundle/mod.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: sixel6vt
- **File Hash**: 3e822dab  
- **Timestamp**: 2025-10-10T02:15:58.394938+00:00  
- **Lines of Code**: 373

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 373 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 221
  - hardcoded URL
  - 

```rust
            identity: HeaplessString::new(),
            keychain_path: None,
            timestamp_url: HeaplessString::from_str("http://timestamp.apple.com/ts01").unwrap(),
            hardened_runtime: true,
            notarize: false,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 329
  - fallback
  - 

```rust
}

/// Resilient signer with automatic retry and fallback
#[derive(Debug)]
pub struct ResilientSigner<T> {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 366
  - fallback
  - 

```rust
    }
    
    /// Sign with automatic retry and fallback
    pub async fn sign_with_retry(
        &self,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 380
  - fallback
  - 

```rust
                Ok(result) => return Ok(result),
                Err(e) if attempts >= self.retry_config.max_attempts => {
                    // Try fallback if available
                    if let Some(ref fallback) = self.fallback {
                        return fallback.sign_bundle(bundle_path, config).await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 221: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            identity: HeaplessString::new(),
            keychain_path: None,
            timestamp_url: HeaplessString::from_str("http://timestamp.apple.com/ts01").unwrap(),
            hardened_runtime: true,
            notarize: false,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 300: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        manifest.signature = [0u8; 64]; // Zero out signature for verification
        
        bincode::serialize(&manifest).unwrap()
    }
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Orphaned Methods


### `file_stem_str()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/packages/sixel6vt/src/bundle/mod.rs` (line 450)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
    /// Get file stem without allocation
    #[inline]
    pub fn file_stem_str(path: &Path) -> Option<&str> {
        path.file_stem()?.to_str()
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