# `packages/sweetmcp/packages/pingora/src/tls/certificate/parser_clean.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora
- **File Hash**: 39ebc061  
- **Timestamp**: 2025-10-10T02:15:59.792688+00:00  
- **Lines of Code**: 199

---## Tier 1 Infractions 


- Line 21
  - hardcoded IP address
  - 

```rust

    // Common OIDs for DN components
    const OID_CN: &str = "2.5.4.3"; // commonName
    const OID_O: &str = "2.5.4.10"; // organizationName
    const OID_OU: &str = "2.5.4.11"; // organizationalUnitName
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 22
  - hardcoded IP address
  - 

```rust
    // Common OIDs for DN components
    const OID_CN: &str = "2.5.4.3"; // commonName
    const OID_O: &str = "2.5.4.10"; // organizationName
    const OID_OU: &str = "2.5.4.11"; // organizationalUnitName
    const OID_C: &str = "2.5.4.6"; // countryName
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 23
  - hardcoded IP address
  - 

```rust
    const OID_CN: &str = "2.5.4.3"; // commonName
    const OID_O: &str = "2.5.4.10"; // organizationName
    const OID_OU: &str = "2.5.4.11"; // organizationalUnitName
    const OID_C: &str = "2.5.4.6"; // countryName
    const OID_ST: &str = "2.5.4.8"; // stateOrProvinceName
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 24
  - hardcoded IP address
  - 

```rust
    const OID_O: &str = "2.5.4.10"; // organizationName
    const OID_OU: &str = "2.5.4.11"; // organizationalUnitName
    const OID_C: &str = "2.5.4.6"; // countryName
    const OID_ST: &str = "2.5.4.8"; // stateOrProvinceName
    const OID_L: &str = "2.5.4.7"; // localityName
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 25
  - hardcoded IP address
  - 

```rust
    const OID_OU: &str = "2.5.4.11"; // organizationalUnitName
    const OID_C: &str = "2.5.4.6"; // countryName
    const OID_ST: &str = "2.5.4.8"; // stateOrProvinceName
    const OID_L: &str = "2.5.4.7"; // localityName

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 26
  - hardcoded IP address
  - 

```rust
    const OID_C: &str = "2.5.4.6"; // countryName
    const OID_ST: &str = "2.5.4.8"; // stateOrProvinceName
    const OID_L: &str = "2.5.4.7"; // localityName

    // Iterate through RDNs (Relative Distinguished Names)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 6
  - actual
  - 

```rust
use der::{Decode, Encode, Reader};
use x509_cert::Certificate as X509CertCert;
// Using available const_oid constants based on actual const_oid 0.9 API
use const_oid::db::rfc5912::{SECP_224_R_1, SECP_256_R_1, SECP_384_R_1, SECP_521_R_1, ID_EC_PUBLIC_KEY};
use const_oid::db::rfc8410::{ID_X_25519, ID_X_448, ID_ED_25519, ID_ED_448};
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 88
  - actual
  - 

```rust
    match der::asn1::OctetString::from_der(ext_data) {
        Ok(octet_string) => {
            // Now parse the actual SubjectAltName SEQUENCE
            let san_bytes = octet_string.as_bytes();
            let mut reader = SliceReader::new(san_bytes).ok();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 155
  - Fall back
  - 

```rust
        }
        Err(_) => {
            // Fall back to simple string search if ASN.1 parsing fails
            let ext_string = String::from_utf8_lossy(ext_data);
            if ext_string.contains("DNS:") {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Orphaned Methods


### `extract_key_usage()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/pingora/src/tls/certificate/parser_clean.rs` (line 208)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Extract Key Usage from certificate extension
fn extract_key_usage(ext: &x509_cert::ext::Extension) -> Vec<String> {
    let mut key_usage = Vec::new();

```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `extract_basic_constraints()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/pingora/src/tls/certificate/parser_clean.rs` (line 172)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Extract Basic Constraints from certificate extension
fn extract_basic_constraints(ext: &x509_cert::ext::Extension) -> bool {
    // Parse BasicConstraints extension
    // Structure: SEQUENCE { cA BOOLEAN DEFAULT FALSE, ... }
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `extract_validity_dates()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/pingora/src/tls/certificate/parser_clean.rs` (line 262)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Extract validity dates from certificate
fn extract_validity_dates(cert: &X509CertCert) -> (SystemTime, SystemTime) {
    let validity = &cert.tbs_certificate.validity;
    let not_before = validity.not_before.to_system_time();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `extract_subject_alt_names()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/pingora/src/tls/certificate/parser_clean.rs` (line 74)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Extract Subject Alternative Names from certificate extension
fn extract_subject_alt_names(ext: &x509_cert::ext::Extension) -> (Vec<String>, Vec<std::net::IpAddr>) {
    let mut san_dns_names = Vec::new();
    let mut san_ip_addresses = Vec::new();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym