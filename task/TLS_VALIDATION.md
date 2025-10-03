# TLS CRL Download: Fix Compilation Errors

## QA Rating: 4/10
**Status:** COMPILATION BLOCKED - Two critical bugs prevent compilation

## Summary

The CRL download implementation is structurally complete and follows all requirements, but has **2 critical compilation errors** that must be fixed:

1. ❌ **Wrong hyper API usage** in `crl_cache.rs:223`
2. ❌ **Missing `.await`** in caller `tls_manager.rs:465`

## Completed ✅ (DO NOT MODIFY)

- ✅ download_and_parse_crl() uses self.http_client 
- ✅ check_against_crl() made async and downloads on cache miss
- ✅ Downloaded CRLs cached and returned by get_rustls_crls()
- ✅ Soft-fail error handling (returns false on download error)
- ✅ Proper logging throughout
- ✅ No unwrap()/expect() used
- ✅ Misleading comments removed

## Critical Bug 1: Wrong Hyper API Usage

**File:** `packages/sweetmcp/packages/pingora/src/tls/crl_cache.rs:223`

**Current (BROKEN):**
```rust
let body_bytes = hyper::body::to_bytes(response.into_body()).await
    .map_err(|e| TlsError::CrlValidation(format!("Failed to read CRL body: {}", e)))?;
```

**Error:**
```
error[E0599]: no function or associated item named `to_bytes` found in module `hyper::body`
```

**Root Cause:** 
- `hyper::body::to_bytes()` doesn't exist in hyper 1.x
- Need `http_body_util::BodyExt` trait for `.collect()`

**Fix Required:**

1. Add import at top of file (after line 13):
```rust
use http_body_util::BodyExt;
```

2. Replace line 223 with:
```rust
let body_bytes = response.into_body()
    .collect()
    .await
    .map_err(|e| TlsError::CrlValidation(format!("Failed to read CRL body: {}", e)))?
    .to_bytes();
```

## Critical Bug 2: Missing .await in Caller

**File:** `packages/sweetmcp/packages/pingora/src/tls/tls_manager.rs:465`

**Current (BROKEN):**
```rust
match self
    .crl_cache
    .check_certificate_status(&parsed_cert.serial_number, crl_url)
{
    crate::tls::crl_cache::CrlStatus::Valid => { ... }
    crate::tls::crl_cache::CrlStatus::Revoked => { ... }
    crate::tls::crl_cache::CrlStatus::Unknown => { ... }
}
```

**Error:**
```
error[E0308]: mismatched types
expected future `impl std::future::Future<Output = CrlStatus>`
found enum `CrlStatus`
```

**Root Cause:**
- `check_certificate_status()` is now async but caller doesn't await it
- Matching on Future instead of CrlStatus enum

**Fix Required:**

Add `.await` after line 465:
```rust
match self
    .crl_cache
    .check_certificate_status(&parsed_cert.serial_number, crl_url)
    .await  // ← ADD THIS
{
    crate::tls::crl_cache::CrlStatus::Valid => { ... }
    crate::tls::crl_cache::CrlStatus::Revoked => { ... }
    crate::tls::crl_cache::CrlStatus::Unknown => { ... }
}
```

## Implementation Steps

### Step 1: Fix Hyper API (2 minutes)
1. Add `use http_body_util::BodyExt;` import to crl_cache.rs
2. Replace line 223 with `.collect().await?.to_bytes()`

### Step 2: Fix Missing .await (1 minute)  
1. Add `.await` after `check_certificate_status()` call in tls_manager.rs:465

### Step 3: Verify Compilation
```bash
cargo check --lib --color=never
```

## Definition of Done

1. ✅ `cargo check` passes with no errors in tls module
2. ✅ CRL download implementation compiles
3. ✅ All async functions properly awaited

## Files to Modify

1. **`packages/sweetmcp/packages/pingora/src/tls/crl_cache.rs`**
   - Line 13: Add `use http_body_util::BodyExt;`
   - Line 223: Replace with `.collect().await?.to_bytes()`

2. **`packages/sweetmcp/packages/pingora/src/tls/tls_manager.rs`**  
   - Line 465: Add `.await` after `check_certificate_status(...)`

**Estimated fix time:** 3 minutes
**Complexity:** Trivial - simple API corrections
