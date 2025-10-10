# `packages/sweetmcp/packages/pingora/src/main.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora
- **File Hash**: b5a9da2a  
- **Timestamp**: 2025-10-10T02:15:59.789154+00:00  
- **Lines of Code**: 603

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 603 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 102
  - stubby method name
  - temp_dir

```rust
    // Create and configure shutdown coordinator
    let mut shutdown_coordinator = shutdown::ShutdownCoordinator::new(
        std::env::temp_dir().join("sweetmcp")
    );
    shutdown_coordinator.set_local_port(local_port);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 102
  - stubby variable name
  - temp_dir

```rust
    // Create and configure shutdown coordinator
    let mut shutdown_coordinator = shutdown::ShutdownCoordinator::new(
        std::env::temp_dir().join("sweetmcp")
    );
    shutdown_coordinator.set_local_port(local_port);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 309
  - hardcoded URL
  - 

```rust
    log::info!("  MCP HTTP: {}", cfg.mcp_bind);
    log::info!("  UDS: {}", cfg.uds_path);
    log::info!("  Metrics: http://{}/metrics", cfg.metrics_bind);

    // Run the server - this never returns
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 138
  - Fallback
  - 

```rust
        server.add_service(dns_service);
    } else {
        // Fallback: mDNS for local network discovery
        let mdns_discovery = mdns_discovery::MdnsDiscovery::new(peer_registry.clone(), local_port);
        let mdns_service = background_service(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 357: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let rx = unsafe {
            let this = self as *const Self as *mut Self;
            (*this).rx.take().unwrap()
        };

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Orphaned Methods


### `check_certificate_expiry()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/pingora/src/main.rs` (line 715)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Check certificate expiry and return days until expiration
fn check_certificate_expiry(cert_path: &std::path::Path) -> anyhow::Result<i64> {
    use x509_parser::prelude::*;
    
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `renew_server_certificate()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/pingora/src/main.rs` (line 735)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Renew server certificate using existing CA
async fn renew_server_certificate(
    authority: &tls::CertificateAuthority,
    domain: &str,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym