# `forks/pingora/pingora-core/src/protocols/l4/socket.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-core
- **File Hash**: 1eec0b0d  
- **Timestamp**: 2025-10-10T02:16:01.213866+00:00  
- **Lines of Code**: 234

---## Tier 1 Infractions 


- Line 77
  - for now
  - 

```rust
        }

        // TODO: don't set abstract / unnamed for now,
        // for parity with how we treat these types in TryFrom<TokioUnixSockAddr>
        Some(SocketAddr::Unix(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 77
  - TODO
  - 

```rust
        }

        // TODO: don't set abstract / unnamed for now,
        // for parity with how we treat these types in TryFrom<TokioUnixSockAddr>
        Some(SocketAddr::Unix(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 145
  - for now
  - 

```rust
                    // unnamed or abstract UDS
                    // abstract UDS name not yet exposed by std API
                    // panic for now, we can decide on the right way to hash them later
                    panic!("Unnamed and abstract UDS types not yet supported for hashing")
                }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 262
  - TODO
  - 

```rust
}

// TODO: ideally mio/tokio will start using the std version of the unix `SocketAddr`
// so we can avoid a fallible conversion
// https://github.com/tokio-rs/mio/issues/1527
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 216
  - backward compatibility
  - 

```rust
                Ok(addr) => Ok(SocketAddr::Inet(addr)),
                Err(_) => {
                    // Try to parse as UDS for backward compatibility
                    let uds_socket = StdUnixSockAddr::from_pathname(s)
                        .or_err(crate::BindError, "invalid UDS path")?;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 285: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    #[test]
    fn parse_ip() {
        let ip: SocketAddr = "127.0.0.1:80".parse().unwrap();
        assert!(ip.as_inet().is_some());
    }
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
    #[test]
    fn parse_uds() {
        let uds: SocketAddr = "/tmp/my.sock".parse().unwrap();
        assert!(uds.as_unix().is_some());
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 299: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    #[test]
    fn parse_uds_with_prefix() {
        let uds: SocketAddr = "unix:/tmp/my.sock".parse().unwrap();
        assert!(uds.as_unix().is_some());
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


### Line 280: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/l4/socket.rs` (line 280)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod test {
    use super::*;

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 284: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/l4/socket.rs` (line 284)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn parse_ip() {
        let ip: SocketAddr = "127.0.0.1:80".parse().unwrap();
        assert!(ip.as_inet().is_some());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 291: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/l4/socket.rs` (line 291)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[cfg(unix)]
    #[test]
    fn parse_uds() {
        let uds: SocketAddr = "/tmp/my.sock".parse().unwrap();
        assert!(uds.as_unix().is_some());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 298: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/l4/socket.rs` (line 298)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[cfg(unix)]
    #[test]
    fn parse_uds_with_prefix() {
        let uds: SocketAddr = "unix:/tmp/my.sock".parse().unwrap();
        assert!(uds.as_unix().is_some());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym