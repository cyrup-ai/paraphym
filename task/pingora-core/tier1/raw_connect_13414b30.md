# `forks/pingora/pingora-core/src/protocols/raw_connect.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-core
- **File Hash**: 13414b30  
- **Timestamp**: 2025-10-10T02:16:01.213012+00:00  
- **Lines of Code**: 209

---## Tier 1 Infractions 


- Line 52
  - TODO
  - 

```rust
        .or_err(WriteError, "while flushing request headers")?;

    // TODO: set http.read_timeout
    let resp_header = http.read_resp_header_parts().await?;
    Ok((
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 70
  - TODO
  - 

```rust
    H: Iterator<Item = (S, &'a Vec<u8>)>,
{
    // TODO: valid that host doesn't have port

    let authority = if host.parse::<std::net::Ipv6Addr>().is_ok() {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 273
  - stubby variable name
  - mock_io

```rust
    async fn test_connect_write_request() {
        let wire = b"CONNECT pingora.org:123 HTTP/1.1\r\nhost: pingora.org:123\r\n\r\n";
        let mock_io = Box::new(Builder::new().write(wire).build());

        let headers: BTreeMap<String, Vec<u8>> = BTreeMap::new();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 278
  - stubby variable name
  - mock_io

```rust
        let req = generate_connect_header("pingora.org", 123, headers.iter()).unwrap();
        // ConnectionClosed
        assert!(connect(mock_io, &req).await.is_err());

        let to_wire = b"CONNECT pingora.org:123 HTTP/1.1\r\nhost: pingora.org:123\r\n\r\n";
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 282
  - stubby variable name
  - mock_io

```rust
        let to_wire = b"CONNECT pingora.org:123 HTTP/1.1\r\nhost: pingora.org:123\r\n\r\n";
        let from_wire = b"HTTP/1.1 200 OK\r\n\r\n";
        let mock_io = Box::new(Builder::new().write(to_wire).read(from_wire).build());

        let req = generate_connect_header("pingora.org", 123, headers.iter()).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 285
  - stubby variable name
  - mock_io

```rust

        let req = generate_connect_header("pingora.org", 123, headers.iter()).unwrap();
        let result = connect(mock_io, &req).await;
        assert!(result.is_ok());
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 241
  - hardcoded URL
  - 

```rust
        let new_request = http::Request::builder()
            .method("CONNECT")
            .uri("https://pingora.org:123/")
            .header("Foo", "Bar")
            .body(())
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 217: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut headers = BTreeMap::new();
        headers.insert(String::from("foo"), b"bar".to_vec());
        let req = generate_connect_header("pingora.org", 123, headers.iter()).unwrap();

        assert_eq!(req.method, http::method::Method::CONNECT);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 229: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut headers = BTreeMap::new();
        headers.insert(String::from("foo"), b"bar".to_vec());
        let req = generate_connect_header("::1", 123, headers.iter()).unwrap();

        assert_eq!(req.method, http::method::Method::CONNECT);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 244: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .header("Foo", "Bar")
            .body(())
            .unwrap();
        let (new_request, _) = new_request.into_parts();
        let wire = http_req_header_to_wire_auth_form(&new_request);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 255: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    #[test]
    fn test_validate_connect_response() {
        let resp = ResponseHeader::build(200, None).unwrap();
        validate_connect_response(Box::new(resp)).unwrap();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 256: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn test_validate_connect_response() {
        let resp = ResponseHeader::build(200, None).unwrap();
        validate_connect_response(Box::new(resp)).unwrap();

        let resp = ResponseHeader::build(404, None).unwrap();
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
        validate_connect_response(Box::new(resp)).unwrap();

        let resp = ResponseHeader::build(404, None).unwrap();
        assert!(validate_connect_response(Box::new(resp)).is_err());

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 261: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert!(validate_connect_response(Box::new(resp)).is_err());

        let mut resp = ResponseHeader::build(200, None).unwrap();
        resp.append_header("content-length", 0).unwrap();
        assert!(validate_connect_response(Box::new(resp)).is_ok());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 262: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let mut resp = ResponseHeader::build(200, None).unwrap();
        resp.append_header("content-length", 0).unwrap();
        assert!(validate_connect_response(Box::new(resp)).is_ok());

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 265: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert!(validate_connect_response(Box::new(resp)).is_ok());

        let mut resp = ResponseHeader::build(200, None).unwrap();
        resp.append_header("transfer-encoding", 0).unwrap();
        assert!(validate_connect_response(Box::new(resp)).is_err());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 266: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let mut resp = ResponseHeader::build(200, None).unwrap();
        resp.append_header("transfer-encoding", 0).unwrap();
        assert!(validate_connect_response(Box::new(resp)).is_err());
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 276: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let headers: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        let req = generate_connect_header("pingora.org", 123, headers.iter()).unwrap();
        // ConnectionClosed
        assert!(connect(mock_io, &req).await.is_err());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 284: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Box::new(Builder::new().write(to_wire).read(from_wire).build());

        let req = generate_connect_header("pingora.org", 123, headers.iter()).unwrap();
        let result = connect(mock_io, &req).await;
        assert!(result.is_ok());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 208: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/raw_connect.rs` (line 208)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod test_sync {
    use super::*;
    use std::collections::BTreeMap;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 214: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/raw_connect.rs` (line 214)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_generate_connect_header() {
        let mut headers = BTreeMap::new();
        headers.insert(String::from("foo"), b"bar".to_vec());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 226: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/raw_connect.rs` (line 226)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_generate_connect_header_ipv6() {
        let mut headers = BTreeMap::new();
        headers.insert(String::from("foo"), b"bar".to_vec());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 238: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/raw_connect.rs` (line 238)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_request_to_wire_auth_form() {
        let new_request = http::Request::builder()
            .method("CONNECT")
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 254: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/raw_connect.rs` (line 254)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_validate_connect_response() {
        let resp = ResponseHeader::build(200, None).unwrap();
        validate_connect_response(Box::new(resp)).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 271: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/raw_connect.rs` (line 271)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_connect_write_request() {
        let wire = b"CONNECT pingora.org:123 HTTP/1.1\r\nhost: pingora.org:123\r\n\r\n";
        let mock_io = Box::new(Builder::new().write(wire).build());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym