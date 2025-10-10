# `forks/pingora/pingora-core/src/utils/tls/rustls.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-core
- **File Hash**: eeaf842e  
- **Timestamp**: 2025-10-10T02:16:01.215068+00:00  
- **Lines of Code**: 154

---## Panic-Prone Code


### Line 176: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    /// Return the serial from the leaf certificate.
    pub fn serial(&self) -> String {
        get_serial(self.leaf()).unwrap()
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


### Line 62: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
pub fn get_organization_serial_bytes(cert: &[u8]) -> Result<(Option<String>, String)> {
    let (_, x509cert) = x509_parser::certificate::X509Certificate::from_der(cert)
        .expect("Failed to parse certificate from DER format.");

    get_organization_serial_x509(&x509cert)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 119: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
{
    X509Certificate::from_der(raw_cert.as_ref())
        .expect("Failed to parse certificate from DER format.")
        .1
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


### `parse_x509()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/utils/tls/rustls.rs` (line 114)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

fn parse_x509<C>(raw_cert: &C) -> X509Certificate<'_>
where
    C: AsRef<[u8]>,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `get_not_after()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/utils/tls/rustls.rs` (line 93)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Get the `not_after` field for the valid time period for the given cert
/// see https://en.wikipedia.org/wiki/X.509#Structure_of_a_certificate
pub fn get_not_after(x509cert: &WrappedX509) -> String {
    x509cert.borrow_cert().validity.not_after.to_string()
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `get_organization_serial()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/utils/tls/rustls.rs` (line 23)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Get the organization and serial number associated with the given certificate
/// see https://en.wikipedia.org/wiki/X.509#Structure_of_a_certificate
pub fn get_organization_serial(x509cert: &WrappedX509) -> Result<(Option<String>, String)> {
    let serial = get_serial(x509cert)?;
    Ok((get_organization(x509cert), serial))
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym