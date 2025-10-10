# `forks/pingora/pingora-core/src/protocols/windows.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-core
- **File Hash**: f725ff32  
- **Timestamp**: 2025-10-10T02:16:01.216877+00:00  
- **Lines of Code**: 91

---## Panic-Prone Code


### Line 99: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

    async fn assert_listener_and_stream(addr: &str) {
        let tokio_listener = tokio::net::TcpListener::bind(addr).await.unwrap();

        let listener_local_addr = tokio_listener.local_addr().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 101: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let tokio_listener = tokio::net::TcpListener::bind(addr).await.unwrap();

        let listener_local_addr = tokio_listener.local_addr().unwrap();

        let tokio_stream = tokio::net::TcpStream::connect(listener_local_addr)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 105: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let tokio_stream = tokio::net::TcpStream::connect(listener_local_addr)
            .await
            .unwrap();

        let stream_local_addr = tokio_stream.local_addr().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 107: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .unwrap();

        let stream_local_addr = tokio_stream.local_addr().unwrap();
        let stream_peer_addr = tokio_stream.peer_addr().unwrap();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 108: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let stream_local_addr = tokio_stream.local_addr().unwrap();
        let stream_peer_addr = tokio_stream.peer_addr().unwrap();

        let stream: Stream = tokio_stream.into();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 91: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/windows.rs` (line 91)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use std::os::windows::io::AsRawSocket;

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 122: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/windows.rs` (line 122)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn get_v4_addrs_from_raw_socket() {
        assert_listener_and_stream("127.0.0.1:0").await
    }
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 126: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/windows.rs` (line 126)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    }
    #[tokio::test]
    async fn get_v6_addrs_from_raw_socket() {
        assert_listener_and_stream("[::1]:0").await
    }
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym