# `forks/pingora/pingora-openssl/src/ext.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-openssl
- **File Hash**: a8814be1  
- **Timestamp**: 2025-10-10T02:16:01.428879+00:00  
- **Lines of Code**: 147

---## Panic-Prone Code


### Line 221: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    #[test]
    fn test_ssl_set_groups_list() {
        let ctx_builder = SslContextBuilder::new(SslMethod::tls()).unwrap();
        let ssl = Ssl::new(&ctx_builder.build()).unwrap();
        let ssl_ref = unsafe { ssl_mut(&ssl) };
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 222: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn test_ssl_set_groups_list() {
        let ctx_builder = SslContextBuilder::new(SslMethod::tls()).unwrap();
        let ssl = Ssl::new(&ctx_builder.build()).unwrap();
        let ssl_ref = unsafe { ssl_mut(&ssl) };

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 215: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-openssl/src/ext.rs` (line 215)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use super::*;
    use openssl::ssl::{SslContextBuilder, SslMethod};
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 220: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-openssl/src/ext.rs` (line 220)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_ssl_set_groups_list() {
        let ctx_builder = SslContextBuilder::new(SslMethod::tls()).unwrap();
        let ssl = Ssl::new(&ctx_builder.build()).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `ssl_add_chain_cert()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-openssl/src/ext.rs` (line 118)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// See [SSL_add1_chain_cert](https://www.openssl.org/docs/man1.1.1/man3/SSL_add1_chain_cert.html)
pub fn ssl_add_chain_cert(ssl: &mut SslRef, cert: &X509Ref) -> Result<(), ErrorStack> {
    const SSL_CTRL_CHAIN_CERT: i32 = 89;
    unsafe {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `ssl_use_certificate()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-openssl/src/ext.rs` (line 95)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// See [SSL_use_certificate](https://www.openssl.org/docs/man1.1.1/man3/SSL_use_certificate.html).
pub fn ssl_use_certificate(ssl: &mut SslRef, cert: &X509Ref) -> Result<(), ErrorStack> {
    unsafe {
        cvt(SSL_use_certificate(ssl.as_ptr(), cert.as_ptr()) as c_long)?;
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `suspend_when_need_ssl_cert()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-openssl/src/ext.rs` (line 181)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// The caller should set the certificate and then call [unblock_ssl_cert()] before continue the
/// handshake on the tls connection.
pub fn suspend_when_need_ssl_cert(ssl: &mut SslRef) {
    unsafe {
        SSL_set_cert_cb(ssl.as_ptr(), Some(raw_cert_block), std::ptr::null_mut());
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `raw_cert_block()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-openssl/src/ext.rs` (line 197)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

// Just block the handshake
extern "C" fn raw_cert_block(_ssl: *mut openssl_sys::SSL, _arg: *mut c_void) -> c_int {
    -1
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `ssl_use_private_key()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-openssl/src/ext.rs` (line 105)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// See [SSL_use_certificate](https://www.openssl.org/docs/man1.1.1/man3/SSL_use_PrivateKey.html).
pub fn ssl_use_private_key<T>(ssl: &mut SslRef, key: &PKeyRef<T>) -> Result<(), ErrorStack>
where
    T: HasPrivate,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `ssl_from_acceptor()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-openssl/src/ext.rs` (line 172)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// this function is to unify the interface between this crate and [`pingora-boringssl`](https://docs.rs/pingora-boringssl)
pub fn ssl_from_acceptor(acceptor: &SslAcceptor) -> Result<Ssl, ErrorStack> {
    Ssl::new(acceptor.context())
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `is_suspended_for_cert()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-openssl/src/ext.rs` (line 202)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Whether the TLS error is SSL_ERROR_WANT_X509_LOOKUP
pub fn is_suspended_for_cert(error: &openssl::ssl::Error) -> bool {
    error.code().as_raw() == openssl_sys::SSL_ERROR_WANT_X509_LOOKUP
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `ssl_use_second_key_share()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-openssl/src/ext.rs` (line 158)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// This function is specific to BoringSSL. This function is noop for OpenSSL.
pub fn ssl_use_second_key_share(_ssl: &mut SslRef, _enabled: bool) {}

/// Clear the error stack
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `add_host()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-openssl/src/ext.rs` (line 60)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// See [X509_VERIFY_PARAM_set1_host](https://www.openssl.org/docs/man3.1/man3/X509_VERIFY_PARAM_set1_host.html).
pub fn add_host(verify_param: &mut X509VerifyParamRef, host: &str) -> Result<(), ErrorStack> {
    if host.is_empty() {
        return Ok(());
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `ssl_set_groups_list()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-openssl/src/ext.rs` (line 139)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// See [set_groups_list](https://www.openssl.org/docs/manmaster/man3/SSL_CTX_set1_curves.html).
pub fn ssl_set_groups_list(ssl: &mut SslRef, groups: &str) -> Result<(), ErrorStack> {
    if groups.contains('\0') {
        return Err(ErrorStack::get());
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `ssl_set_renegotiate_mode_freely()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-openssl/src/ext.rs` (line 134)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// This function is specific to BoringSSL. This function is noop for OpenSSL.
pub fn ssl_set_renegotiate_mode_freely(_ssl: &mut SslRef) {}

/// Set the curves/groups of `ssl`
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `clear_error_stack()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-openssl/src/ext.rs` (line 165)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// This causes the next unrelated SSL call to fail due to the leftover errors. This function allows
/// caller to clear the error stack before performing SSL calls to avoid this issue.
pub fn clear_error_stack() {
    let _ = ErrorStack::get();
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `unblock_ssl_cert()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-openssl/src/ext.rs` (line 190)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// The user should continue to call tls handshake after this function is called.
pub fn unblock_ssl_cert(ssl: &mut SslRef) {
    unsafe {
        SSL_set_cert_cb(ssl.as_ptr(), None, std::ptr::null_mut());
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `ssl_set_verify_cert_store()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-openssl/src/ext.rs` (line 77)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// See [SSL_set1_verify_cert_store](https://www.openssl.org/docs/man1.1.1/man3/SSL_set1_verify_cert_store.html).
pub fn ssl_set_verify_cert_store(
    ssl: &mut SslRef,
    cert_store: &X509StoreRef,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym