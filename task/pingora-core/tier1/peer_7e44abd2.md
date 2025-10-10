# `forks/pingora/pingora-core/src/upstreams/peer.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-core
- **File Hash**: 7e44abd2  
- **Timestamp**: 2025-10-10T02:16:01.210383+00:00  
- **Lines of Code**: 481

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 481 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 277
  - TODO
  - 

```rust
    }

    // TODO: change connection pool to accept u64 instead of String
    fn reuse_hash(&self) -> u64 {
        let mut hasher = AHasher::default();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 563
  - TODO
  - 

```rust
    }

    // TODO: change connection pool to accept u64 instead of String
    fn reuse_hash(&self) -> u64 {
        self.peer_hash()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 609
  - for now
  - 

```rust
}

/// The proxy settings to connect to the remote server, CONNECT only for now
#[derive(Debug, Hash, Clone)]
pub struct Proxy {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 231: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    /// Create a new [`BasicPeer`].
    pub fn new(address: &str) -> Self {
        let addr = SocketAddr::Inet(address.parse().unwrap()); // TODO: check error
        Self::new_from_sockaddr(addr)
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 471: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    /// Create a new [`HttpPeer`] with the given socket address and TLS settings.
    pub fn new<A: ToInetSocketAddrs>(address: A, tls: bool, sni: String) -> Self {
        let mut addrs_iter = address.to_socket_addrs().unwrap(); //TODO: handle error
        let addr = addrs_iter.next().unwrap();
        Self::new_from_sockaddr(SocketAddr::Inet(addr), tls, sni)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 472: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    pub fn new<A: ToInetSocketAddrs>(address: A, tls: bool, sni: String) -> Self {
        let mut addrs_iter = address.to_socket_addrs().unwrap(); //TODO: handle error
        let addr = addrs_iter.next().unwrap();
        Self::new_from_sockaddr(SocketAddr::Inet(addr), tls, sni)
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym