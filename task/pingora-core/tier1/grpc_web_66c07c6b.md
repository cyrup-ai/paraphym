# `forks/pingora/pingora-core/src/protocols/http/bridge/grpc_web.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-core
- **File Hash**: 66c07c6b  
- **Timestamp**: 2025-10-10T02:16:01.211703+00:00  
- **Lines of Code**: 231

---## Tier 1 Infractions 


- Line 137
  - TODO
  - 

```rust
            |                    Headers                    |
        */
        // TODO compressed trailer?
        // grpc-web trailers frame head
        const GRPC_WEB_TRAILER: u8 = 0x80;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 191
  - hardcoded URL
  - 

```rust
    #[test]
    fn non_grpc_web_request_ignored() {
        let request = Request::get("https://pingora.dev/")
            .header(CONTENT_TYPE, "application/grpc-we")
            .version(Version::HTTP_2) // only set this to verify send_end_stream is configured
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 211
  - hardcoded URL
  - 

```rust
    #[test]
    fn grpc_web_request_module_disabled_ignored() {
        let request = Request::get("https://pingora.dev/")
            .header(CONTENT_TYPE, "application/grpc-web")
            .version(Version::HTTP_2) // only set this to verify send_end_stream is configured
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 231
  - hardcoded URL
  - 

```rust
    #[test]
    fn grpc_web_request_upgrade() {
        let request = Request::get("https://pingora.org/")
            .header(CONTENT_TYPE, "application/gRPC-web+thrift")
            .version(Version::HTTP_2) // only set this to verify send_end_stream is configured
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 195: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .version(Version::HTTP_2) // only set this to verify send_end_stream is configured
            .body(())
            .unwrap();
        let mut request = request.into_parts().0.into();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 215: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .version(Version::HTTP_2) // only set this to verify send_end_stream is configured
            .body(())
            .unwrap();
        let mut request = request.into_parts().0.into();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 235: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .version(Version::HTTP_2) // only set this to verify send_end_stream is configured
            .body(())
            .unwrap();
        let mut request = request.into_parts().0.into();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 258: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .header(CONTENT_LENGTH, "10")
            .body(())
            .unwrap();
        let mut response = response.into_parts().0.into();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 275: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .header(CONTENT_TYPE, "application/grpc")
            .body(())
            .unwrap();
        let mut response = response.into_parts().0.into();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 292: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .header(CONTENT_LENGTH, "0")
            .body(())
            .unwrap();
        let mut response = response.into_parts().0.into();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 310: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    #[test]
    fn grpc_response_informational_proxied() {
        let response = Response::builder().status(100).body(()).unwrap();
        let mut response = response.into_parts().0.into();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 324: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .header("grpc-message", "OK")
            .body(())
            .unwrap();
        let response = response.headers_mut();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 328: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let mut filter = GrpcWebCtx::Trailers;
        let buf = filter.response_trailer_filter(response).unwrap().unwrap();
        assert_eq!(filter, GrpcWebCtx::Done);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 328: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let mut filter = GrpcWebCtx::Trailers;
        let buf = filter.response_trailer_filter(response).unwrap().unwrap();
        assert_eq!(filter, GrpcWebCtx::Done);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 68: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        // change content type to grpc
        let ct = content_type.to_lowercase().replace(GRPC_WEB, GRPC);
        req.insert_header(CONTENT_TYPE, ct).expect("insert header");

        // The 'te' request header is used to detect incompatible proxies
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 74: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        // This header is required by gRPC over h2 protocol.
        // https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md
        req.insert_header("te", "trailers").expect("insert header");

        // For gRPC requests, EOS (end-of-stream) is indicated by the presence of the
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 114: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        // change content type to gRPC-web
        let ct = content_type.replace(GRPC, GRPC_WEB);
        resp.insert_header(CONTENT_TYPE, ct).expect("insert header");

        // always use chunked for gRPC-web
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
        resp.remove_header(&CONTENT_LENGTH);
        resp.insert_header(TRANSFER_ENCODING, "chunked")
            .expect("insert header");

        *self = Self::Trailers
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 185: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/bridge/grpc_web.rs` (line 185)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use super::*;
    use http::{request::Request, response::Response, Version};
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 190: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/bridge/grpc_web.rs` (line 190)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn non_grpc_web_request_ignored() {
        let request = Request::get("https://pingora.dev/")
            .header(CONTENT_TYPE, "application/grpc-we")
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 210: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/bridge/grpc_web.rs` (line 210)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn grpc_web_request_module_disabled_ignored() {
        let request = Request::get("https://pingora.dev/")
            .header(CONTENT_TYPE, "application/grpc-web")
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 230: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/bridge/grpc_web.rs` (line 230)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn grpc_web_request_upgrade() {
        let request = Request::get("https://pingora.org/")
            .header(CONTENT_TYPE, "application/gRPC-web+thrift")
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 253: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/bridge/grpc_web.rs` (line 253)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn non_grpc_response_ignored() {
        let response = Response::builder()
            .header(CONTENT_TYPE, "text/html")
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 271: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/bridge/grpc_web.rs` (line 271)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn grpc_response_module_disabled_ignored() {
        let response = Response::builder()
            .header(CONTENT_TYPE, "application/grpc")
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 287: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/bridge/grpc_web.rs` (line 287)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn grpc_response_upgrade() {
        let response = Response::builder()
            .header(CONTENT_TYPE, "application/grpc+proto")
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 309: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/bridge/grpc_web.rs` (line 309)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn grpc_response_informational_proxied() {
        let response = Response::builder().status(100).body(()).unwrap();
        let mut response = response.into_parts().0.into();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 319: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/bridge/grpc_web.rs` (line 319)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn grpc_response_trailer_headers_convert_to_byte_buf() {
        let mut response = Response::builder()
            .header("grpc-status", "0")
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym