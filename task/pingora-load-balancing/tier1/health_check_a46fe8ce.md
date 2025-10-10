# `forks/pingora/pingora-load-balancing/src/health_check.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-load-balancing
- **File Hash**: a46fe8ce  
- **Timestamp**: 2025-10-10T02:16:01.442168+00:00  
- **Lines of Code**: 362

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 362 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 388
  - hardcoded IP address
  - 

```rust

        let backend = Backend {
            addr: SocketAddr::Inet("1.1.1.1:80".parse().unwrap()),
            weight: 1,
            ext: Extensions::new(),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 396
  - hardcoded IP address
  - 

```rust

        let backend = Backend {
            addr: SocketAddr::Inet("1.1.1.1:79".parse().unwrap()),
            weight: 1,
            ext: Extensions::new(),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 409
  - hardcoded IP address
  - 

```rust
        let tls_check = TcpHealthCheck::new_tls("one.one.one.one");
        let backend = Backend {
            addr: SocketAddr::Inet("1.1.1.1:443".parse().unwrap()),
            weight: 1,
            ext: Extensions::new(),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 423
  - hardcoded IP address
  - 

```rust

        let backend = Backend {
            addr: SocketAddr::Inet("1.1.1.1:443".parse().unwrap()),
            weight: 1,
            ext: Extensions::new(),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 446
  - hardcoded IP address
  - 

```rust

        let backend = Backend {
            addr: SocketAddr::Inet("1.1.1.1:80".parse().unwrap()),
            weight: 1,
            ext: Extensions::new(),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 2 (possible) Infractions 


- Line 74
  - placeholder
  - 

```rust
    ///
    /// This field defines settings like the connect timeout and src IP to bind.
    /// The SocketAddr of `peer_template` is just a placeholder which will be replaced by the
    /// actual address of the backend when the health check runs.
    ///
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 159
  - placeholder
  - 

```rust
    ///
    /// This field defines settings like the connect timeout and src IP to bind.
    /// The SocketAddr of `peer_template` is just a placeholder which will be replaced by the
    /// actual address of the backend when the health check runs.
    ///
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 75
  - actual
  - 

```rust
    /// This field defines settings like the connect timeout and src IP to bind.
    /// The SocketAddr of `peer_template` is just a placeholder which will be replaced by the
    /// actual address of the backend when the health check runs.
    ///
    /// By default, this check will try to establish a TCP connection. When the `sni` field is
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 160
  - actual
  - 

```rust
    /// This field defines settings like the connect timeout and src IP to bind.
    /// The SocketAddr of `peer_template` is just a placeholder which will be replaced by the
    /// actual address of the backend when the health check runs.
    ///
    /// Set the `scheme` field to use HTTPs.
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 178
  - actual
  - 

```rust
    /// If not set, any response with a `200 OK` is considered a successful check.
    pub validator: Option<Validator>,
    /// Sometimes the health check endpoint lives one a different port than the actual backend.
    /// Setting this option allows the health check to perform on the given port of the backend IP.
    pub port_override: Option<u16>,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 197: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    /// * validator: `None`, any 200 response is considered successful
    pub fn new(host: &str, tls: bool) -> Self {
        let mut req = RequestHeader::build("GET", b"/", None).unwrap();
        req.append_header("Host", host).unwrap();
        let sni = if tls { host.into() } else { String::new() };
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 198: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    pub fn new(host: &str, tls: bool) -> Self {
        let mut req = RequestHeader::build("GET", b"/", None).unwrap();
        req.append_header("Host", host).unwrap();
        let sni = if tls { host.into() } else { String::new() };
        let mut peer_template = HttpPeer::new("0.0.0.0:1", tls, sni);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 388: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let backend = Backend {
            addr: SocketAddr::Inet("1.1.1.1:80".parse().unwrap()),
            weight: 1,
            ext: Extensions::new(),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 396: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let backend = Backend {
            addr: SocketAddr::Inet("1.1.1.1:79".parse().unwrap()),
            weight: 1,
            ext: Extensions::new(),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 409: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let tls_check = TcpHealthCheck::new_tls("one.one.one.one");
        let backend = Backend {
            addr: SocketAddr::Inet("1.1.1.1:443".parse().unwrap()),
            weight: 1,
            ext: Extensions::new(),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 423: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let backend = Backend {
            addr: SocketAddr::Inet("1.1.1.1:443".parse().unwrap()),
            weight: 1,
            ext: Extensions::new(),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 446: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let backend = Backend {
            addr: SocketAddr::Inet("1.1.1.1:80".parse().unwrap()),
            weight: 1,
            ext: Extensions::new(),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 451: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        };

        http_check.check(&backend).await.unwrap();

        assert!(http_check.check(&backend).await.is_ok());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 470: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        }

        let good_backend = Backend::new("127.0.0.1:79").unwrap();
        let new_good_backends = || -> (BTreeSet<Backend>, HashMap<u64, bool>) {
            let mut healthy = HashMap::new();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 259: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        session.read_response_header().await?;

        let resp = session.response_header().expect("just read");

        if let Some(validator) = self.validator.as_ref() {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 372: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-load-balancing/src/health_check.rs` (line 372)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod test {
    use std::{
        collections::{BTreeSet, HashMap},
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 384: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-load-balancing/src/health_check.rs` (line 384)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_tcp_check() {
        let tcp_check = TcpHealthCheck::default();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 406: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-load-balancing/src/health_check.rs` (line 406)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[cfg(feature = "any_tls")]
    #[tokio::test]
    async fn test_tls_check() {
        let tls_check = TcpHealthCheck::new_tls("one.one.one.one");
        let backend = Backend {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 419: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-load-balancing/src/health_check.rs` (line 419)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[cfg(feature = "any_tls")]
    #[tokio::test]
    async fn test_https_check() {
        let https_check = HttpHealthCheck::new("one.one.one.one", true);

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 432: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-load-balancing/src/health_check.rs` (line 432)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_http_custom_check() {
        let mut http_check = HttpHealthCheck::new("one.one.one.one", false);
        http_check.validator = Some(Box::new(|resp: &ResponseHeader| {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 457: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-load-balancing/src/health_check.rs` (line 457)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_health_observe() {
        struct Observe {
            unhealthy_count: Arc<AtomicU16>,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym