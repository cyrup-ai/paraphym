# `forks/pingora/pingora-core/src/protocols/http/conditional_filter.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-core
- **File Hash**: 851aa274  
- **Timestamp**: 2025-10-10T02:16:01.211463+00:00  
- **Lines of Code**: 196

---## Tier 1 Infractions 


- Line 35
  - TODO
  - 

```rust
    // https://datatracker.ietf.org/doc/html/rfc9111#name-handling-a-received-validat

    // TODO: If-Match and If-Unmodified-Since, and returning 412 Precondition Failed
    // Note that this function is currently used only for proxy cache,
    // and the current RFCs have some conflicting opinions as to whether
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 74
  - TODO
  - 

```rust
// Trim ASCII whitespace bytes from the start of the slice.
// This is pretty much copied from the nightly API.
// TODO: use `trim_ascii_start` when it stabilizes https://doc.rust-lang.org/std/primitive.slice.html#method.trim_ascii_start
fn trim_ascii_start(mut bytes: &[u8]) -> &[u8] {
    while let [first, rest @ ..] = bytes {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 87
  - hardcoded IP address
  - 

```rust

/// Search for an ETag matching `target_etag` from the input header, using
/// [weak comparison](https://datatracker.ietf.org/doc/html/rfc9110#section-8.8.3.2).
/// Multiple ETags can exist in the header as a comma-separated list.
///
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 92
  - hardcoded IP address
  - 

```rust
/// Returns true if a matching ETag exists.
pub fn weak_validate_etag(input_etag_header: &[u8], target_etag: &[u8]) -> bool {
    // ETag comparison: https://datatracker.ietf.org/doc/html/rfc9110#section-8.8.3.2
    fn strip_weak_prefix(etag: &[u8]) -> &[u8] {
        // Weak ETags are prefaced with `W/`
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 43
  - actual
  - 

```rust
    // https://datatracker.ietf.org/doc/html/rfc9110#name-precedence-of-preconditions
    // If-None-Match should be handled before If-Modified-Since.
    // XXX: In nginx, IMS is actually checked first, which may cause compatibility issues
    // for certain origins/clients.

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 103
  - legacy
  - 

```rust

    // The RFC defines ETags here: https://datatracker.ietf.org/doc/html/rfc9110#section-8.8.3
    // The RFC requires ETags to be wrapped in double quotes, though some legacy origins or clients
    // don't adhere to this.
    // Unfortunately by allowing non-quoted etags, parsing becomes a little more complicated.
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 298
  - legacy
  - 

```rust
    #[test]
    fn test_weak_validate_etag_unquoted() {
        // legacy unquoted etag
        let target_unquoted = b"xyzzy";
        assert!(weak_validate_etag(b"*", target_unquoted));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 199: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    // "A server may send content-length in 304", but no common web server does it
    // So we drop both content-length and content-type for consistency/less surprise
    resp.set_status(StatusCode::NOT_MODIFIED).unwrap();
    resp.remove_header(&CONTENT_LENGTH);
    resp.remove_header(&CONTENT_TYPE);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 219: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn test_if_modified_since() {
        fn build_req(if_modified_since: &[u8]) -> RequestHeader {
            let mut req = RequestHeader::build("GET", b"/", None).unwrap();
            req.insert_header("If-Modified-Since", if_modified_since)
                .unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 221: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            let mut req = RequestHeader::build("GET", b"/", None).unwrap();
            req.insert_header("If-Modified-Since", if_modified_since)
                .unwrap();
            req
        }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 226: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        fn build_resp(last_modified: &[u8]) -> ResponseHeader {
            let mut resp = ResponseHeader::build(200, None).unwrap();
            resp.insert_header("Last-Modified", last_modified).unwrap();
            resp
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 227: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        fn build_resp(last_modified: &[u8]) -> ResponseHeader {
            let mut resp = ResponseHeader::build(200, None).unwrap();
            resp.insert_header("Last-Modified", last_modified).unwrap();
            resp
        }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 213: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/conditional_filter.rs` (line 213)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use super::*;

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 217: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/conditional_filter.rs` (line 217)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_if_modified_since() {
        fn build_req(if_modified_since: &[u8]) -> RequestHeader {
            let mut req = RequestHeader::build("GET", b"/", None).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 249: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/conditional_filter.rs` (line 249)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_weak_validate_etag() {
        let target_weak_etag = br#"W/"xyzzy""#;
        let target_etag = br#""xyzzy""#;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 297: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/conditional_filter.rs` (line 297)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_weak_validate_etag_unquoted() {
        // legacy unquoted etag
        let target_unquoted = b"xyzzy";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `to_304()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/conditional_filter.rs` (line 194)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Utility function to convert the input response header to a 304 Not Modified response.
pub fn to_304(resp: &mut ResponseHeader) {
    // https://datatracker.ietf.org/doc/html/rfc9110#name-304-not-modified
    // XXX: https://datatracker.ietf.org/doc/html/rfc9110#name-content-length
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `not_modified_filter()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/conditional_filter.rs` (line 25)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
///
/// Returns true if the request should receive 304 Not Modified.
pub fn not_modified_filter(req: &RequestHeader, resp: &ResponseHeader) -> bool {
    // https://datatracker.ietf.org/doc/html/rfc9110#name-304-not-modified
    // 304 can only validate 200
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym