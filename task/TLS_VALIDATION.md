# TLS: Comprehensive Certificate Validation Disconnection

## Status
**DISCONNECTED** - Full validation pipeline exists but bypassed

## Problem
Comprehensive certificate validation system built but never called. OCSP, CRL, chain validation, time/constraints/key usage checks all implemented but skipped.

## Disconnected Components

### 1. OCSP Validation (42 items dead)
**File**: `tls/ocsp.rs`
- `OcspCache.nonce_pool` field exists but unused
- `perform_ocsp_check()` method never called (line 190)
- `generate_nonce()` method never called (line 207)

### 2. CRL Validation
**File**: `tls/crl_cache.rs`
- `CrlCache.http_client` field exists but unused (line 30)
- `cache_crl()` never called (line 182)
- `download_and_parse_crl()` never called (line 194)
- `parse_crl_data()` never called (line 206)

### 3. Bootstrap HTTP Client
**File**: `tls/bootstrap_client.rs`
- `BootstrapHttpClient.client` field exists (line 19)
- `execute()` method never called (line 48)
- `get()` method never called (line 56)

### 4. Comprehensive Validation Functions
**File**: `tls/tls_manager.rs` or validation modules
- `verify_peer_certificate()` never called
- `verify_peer_certificate_comprehensive()` never called
- `verify_hostname()` never called
- `match_hostname()` never called
- `validate_certificate_chain()` never called
- `validate_certificate_time()` + `_internal()` never called
- `validate_basic_constraints()` + `_internal()` never called
- `validate_key_usage()` + `_internal()` never called

### 5. Certificate Generation
**File**: `tls/builder/certificate/`
- `generate_ca()` never called
- `generate_server_cert()` never called
- `generate_wildcard_certificate()` never called
- `validate_existing_wildcard_cert()` never called
- `load_ca()` never called
- `parse_certificate_from_der()` never called

### 6. TlsManager Integration
**File**: `tls/tls_manager.rs`
- `TlsManager.ocsp_cache` field exists but OCSP never invoked (check field usage)

## Root Cause
TLS validation takes simpler path. Instead of calling comprehensive validation, code uses basic rustls verification.

## Reconnection Points

### Find Current Validation
Search for where TLS connections are established:
1. Check `TlsManager::create_client_config()` or similar
2. Look for where certificates are verified
3. Find rustls config setup

### Reconnect OCSP
1. In certificate validation callback, call `ocsp_cache.perform_ocsp_check()`
2. Use `BootstrapHttpClient` to fetch OCSP responses
3. Check revocation status before accepting cert

### Reconnect CRL
1. Extract CRL distribution points from certificate
2. Call `crl_cache.download_and_parse_crl()`
3. Check certificate serial against CRL

### Reconnect Comprehensive Validation
Replace simple validation with:
```rust
verify_peer_certificate_comprehensive(
    cert_chain,
    server_name,
    &self.ocsp_cache,
    &self.crl_cache,
)
```

### Reconnect Certificate Builders
1. Find where certificates are loaded
2. If not found, call `generate_server_cert()` or `generate_wildcard_certificate()`
3. Use CA generation for self-signed development certs

## Investigation Required

### 1. Find TLS Handshake Code
```bash
grep -r "rustls::ServerConfig\|rustls::ClientConfig" src/
grep -r "certificate_verifier\|custom_certificate_verifier" src/
```

### 2. Find Certificate Loading
```bash
grep -r "load_certs\|pemfile::certs" src/
grep -r "X509Certificate\|CertificateDer" src/
```

### 3. Check TlsManager Usage
```bash
grep -r "TlsManager::" src/
grep -r "\.ocsp_cache\|\.crl_cache" src/
```

## Files to Modify
- `tls/tls_manager.rs` - Wire up OCSP/CRL checks
- `tls/ocsp.rs` - Call perform_ocsp_check from validation
- `tls/crl_cache.rs` - Call download_and_parse_crl from validation
- `tls/bootstrap_client.rs` - Use client for OCSP/CRL HTTP requests
- Certificate verification callback - Add comprehensive checks

## Expected Behavior After Reconnection
1. ✅ Client cert validated with OCSP stapling
2. ✅ CRL checked if OCSP unavailable
3. ✅ Certificate chain validated
4. ✅ Time validity enforced
5. ✅ Basic constraints checked
6. ✅ Key usage validated
7. ✅ Hostname matching verified
8. ✅ Auto-generation of missing certs in dev mode
