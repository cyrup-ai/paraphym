# `forks/pingora/pingora-ketama/src/lib.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-ketama
- **File Hash**: f35181ee  
- **Timestamp**: 2025-10-10T02:16:01.261921+00:00  
- **Lines of Code**: 273

---## Tier 1 Infractions 


- Line 72
  - TODO
  - 

```rust
pub struct Bucket {
    // The node name.
    // TODO: UDS
    node: SocketAddr,

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 157
  - TODO
  - 

```rust
            // - The hash input is as follows "HOST EMPTY PORT PREVIOUS_HASH". Spaces are only added
            //   for readability.
            // TODO: remove this logic and hash the literal SocketAddr once we no longer
            // need backwards compatibility

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 35
  - hardcoded IP address
  - 

```rust
//!     let mut buckets = vec![];
//!     buckets.push(Bucket::new("127.0.0.1:12345".parse().unwrap(), 1));
//!     buckets.push(Bucket::new("127.0.0.2:12345".parse().unwrap(), 2));
//!     buckets.push(Bucket::new("127.0.0.3:12345".parse().unwrap(), 3));
//!     let ring = Continuum::new(&buckets);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 36
  - hardcoded IP address
  - 

```rust
//!     buckets.push(Bucket::new("127.0.0.1:12345".parse().unwrap(), 1));
//!     buckets.push(Bucket::new("127.0.0.2:12345".parse().unwrap(), 2));
//!     buckets.push(Bucket::new("127.0.0.3:12345".parse().unwrap(), 3));
//!     let ring = Continuum::new(&buckets);
//!
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 49
  - hardcoded IP address
  - 

```rust
//! ```bash
//! # Output:
//! some_key: 127.0.0.3:12345
//! another_key: 127.0.0.3:12345
//! last_key: 127.0.0.2:12345
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 50
  - hardcoded IP address
  - 

```rust
//! # Output:
//! some_key: 127.0.0.3:12345
//! another_key: 127.0.0.3:12345
//! last_key: 127.0.0.2:12345
//! ```
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 51
  - hardcoded IP address
  - 

```rust
//! some_key: 127.0.0.3:12345
//! another_key: 127.0.0.3:12345
//! last_key: 127.0.0.2:12345
//! ```
//!
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 270
  - hardcoded IP address
  - 

```rust
        fn assert_hosts(c: &Continuum) {
            assert_eq!(c.node(b"a"), Some(get_sockaddr("127.0.0.10:6443")));
            assert_eq!(c.node(b"b"), Some(get_sockaddr("127.0.0.5:6443")));
        }

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 323
  - hardcoded IP address
  - 

```rust
    fn matches_nginx_sample_data() {
        let upstream_hosts = [
            "10.0.0.1:443",
            "10.0.0.2:443",
            "10.0.0.3:443",
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 324
  - hardcoded IP address
  - 

```rust
        let upstream_hosts = [
            "10.0.0.1:443",
            "10.0.0.2:443",
            "10.0.0.3:443",
            "10.0.0.4:443",
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 325
  - hardcoded IP address
  - 

```rust
            "10.0.0.1:443",
            "10.0.0.2:443",
            "10.0.0.3:443",
            "10.0.0.4:443",
            "10.0.0.5:443",
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 326
  - hardcoded IP address
  - 

```rust
            "10.0.0.2:443",
            "10.0.0.3:443",
            "10.0.0.4:443",
            "10.0.0.5:443",
            "10.0.0.6:443",
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 327
  - hardcoded IP address
  - 

```rust
            "10.0.0.3:443",
            "10.0.0.4:443",
            "10.0.0.5:443",
            "10.0.0.6:443",
            "10.0.0.7:443",
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 328
  - hardcoded IP address
  - 

```rust
            "10.0.0.4:443",
            "10.0.0.5:443",
            "10.0.0.6:443",
            "10.0.0.7:443",
            "10.0.0.8:443",
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 329
  - hardcoded IP address
  - 

```rust
            "10.0.0.5:443",
            "10.0.0.6:443",
            "10.0.0.7:443",
            "10.0.0.8:443",
            "10.0.0.9:443",
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 330
  - hardcoded IP address
  - 

```rust
            "10.0.0.6:443",
            "10.0.0.7:443",
            "10.0.0.8:443",
            "10.0.0.9:443",
        ];
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 331
  - hardcoded IP address
  - 

```rust
            "10.0.0.7:443",
            "10.0.0.8:443",
            "10.0.0.9:443",
        ];
        let upstream_hosts = upstream_hosts.iter().map(|i| get_sockaddr(i));
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 98
  - actual
  - 

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
struct Point {
    // the index to the actual address
    node: u32,
    hash: u32,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 162: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            // with_capacity = max_len(ipv6)(39) + len(null)(1) + max_len(port)(5)
            let mut hash_bytes = Vec::with_capacity(39 + 1 + 5);
            write!(&mut hash_bytes, "{}", bucket.node.ip()).unwrap();
            write!(&mut hash_bytes, "\0").unwrap();
            write!(&mut hash_bytes, "{}", bucket.node.port()).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 163: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            let mut hash_bytes = Vec::with_capacity(39 + 1 + 5);
            write!(&mut hash_bytes, "{}", bucket.node.ip()).unwrap();
            write!(&mut hash_bytes, "\0").unwrap();
            write!(&mut hash_bytes, "{}", bucket.node.port()).unwrap();
            hasher.update(hash_bytes.as_ref());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 164: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            write!(&mut hash_bytes, "{}", bucket.node.ip()).unwrap();
            write!(&mut hash_bytes, "\0").unwrap();
            write!(&mut hash_bytes, "{}", bucket.node.port()).unwrap();
            hasher.update(hash_bytes.as_ref());

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 263: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

    fn get_sockaddr(ip: &str) -> SocketAddr {
        ip.parse().unwrap()
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 349: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .has_headers(false)
            .from_path(path)
            .unwrap();

        for pair in rdr.records() {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 352: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        for pair in rdr.records() {
            let pair = pair.unwrap();
            let uri = pair.get(0).unwrap();
            let upstream = pair.get(1).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 353: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        for pair in rdr.records() {
            let pair = pair.unwrap();
            let uri = pair.get(0).unwrap();
            let upstream = pair.get(1).unwrap();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 354: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            let pair = pair.unwrap();
            let uri = pair.get(0).unwrap();
            let upstream = pair.get(1).unwrap();

            let got = c.node(uri.as_bytes()).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 356: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            let upstream = pair.get(1).unwrap();

            let got = c.node(uri.as_bytes()).unwrap();
            assert_eq!(got, get_sockaddr(upstream));
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


### Line 256: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-ketama/src/lib.rs` (line 256)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::path::Path;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 267: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-ketama/src/lib.rs` (line 267)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn consistency_after_adding_host() {
        fn assert_hosts(c: &Continuum) {
            assert_eq!(c.node(b"a"), Some(get_sockaddr("127.0.0.10:6443")));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 289: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-ketama/src/lib.rs` (line 289)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn matches_nginx_sample() {
        let upstream_hosts = ["127.0.0.1:7777", "127.0.0.1:7778"];
        let upstream_hosts = upstream_hosts.iter().map(|i| get_sockaddr(i));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 321: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-ketama/src/lib.rs` (line 321)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn matches_nginx_sample_data() {
        let upstream_hosts = [
            "10.0.0.1:443",
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 362: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-ketama/src/lib.rs` (line 362)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn node_iter() {
        let upstream_hosts = ["127.0.0.1:7777", "127.0.0.1:7778", "127.0.0.1:7779"];
        let upstream_hosts = upstream_hosts.iter().map(|i| get_sockaddr(i));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 416: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-ketama/src/lib.rs` (line 416)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_empty() {
        let c = Continuum::new(&[]);
        assert!(c.node(b"doghash").is_none());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 427: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-ketama/src/lib.rs` (line 427)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_ipv6_ring() {
        let upstream_hosts = ["[::1]:7777", "[::1]:7778", "[::1]:7779"];
        let upstream_hosts = upstream_hosts.iter().map(|i| get_sockaddr(i));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym