# `forks/pingora/pingora-core/src/connectors/mod.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-core
- **File Hash**: a1b4a746  
- **Timestamp**: 2025-10-10T02:16:01.209825+00:00  
- **Lines of Code**: 410

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 410 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 336
  - TODO
  - 

```rust

struct PreferredHttpVersion {
    // TODO: shard to avoid the global lock
    versions: RwLock<HashMap<u64, u8>>, // <hash of peer, version>
}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 340
  - TODO
  - 

```rust
}

// TODO: limit the size of this

impl PreferredHttpVersion {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 437
  - stubby method name
  - mock_connect_server

```rust
    // one-off mock server
    #[cfg(unix)]
    async fn mock_connect_server() {
        let _ = std::fs::remove_file(MOCK_UDS_PATH);
        let listener = UnixListener::bind(MOCK_UDS_PATH).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 450
  - stubby method name
  - mock_connect_server

```rust
    async fn test_connect_uds() {
        tokio::spawn(async {
            mock_connect_server().await;
        });
        // create a new service at /tmp
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 437
  - stubby variable name
  - mock_connect_server

```rust
    // one-off mock server
    #[cfg(unix)]
    async fn mock_connect_server() {
        let _ = std::fs::remove_file(MOCK_UDS_PATH);
        let listener = UnixListener::bind(MOCK_UDS_PATH).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 450
  - stubby variable name
  - mock_connect_server

```rust
    async fn test_connect_uds() {
        tokio::spawn(async {
            mock_connect_server().await;
        });
        // create a new service at /tmp
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 403
  - hardcoded IP address
  - 

```rust
    use tokio::net::UnixListener;

    // 192.0.2.1 is effectively a black hole
    const BLACK_HOLE: &str = "192.0.2.1:79";

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 404
  - hardcoded IP address
  - 

```rust

    // 192.0.2.1 is effectively a black hole
    const BLACK_HOLE: &str = "192.0.2.1:79";

    #[tokio::test]
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 409
  - hardcoded IP address
  - 

```rust
    async fn test_connect() {
        let connector = TransportConnector::new(None);
        let peer = BasicPeer::new("1.1.1.1:80");
        // make a new connection to 1.1.1.1
        let stream = connector.new_stream(&peer).await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 410
  - hardcoded IP address
  - 

```rust
        let connector = TransportConnector::new(None);
        let peer = BasicPeer::new("1.1.1.1:80");
        // make a new connection to 1.1.1.1
        let stream = connector.new_stream(&peer).await.unwrap();
        connector.release_stream(stream, peer.reuse_hash(), None);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 421
  - hardcoded IP address
  - 

```rust
    async fn test_connect_tls() {
        let connector = TransportConnector::new(None);
        let mut peer = BasicPeer::new("1.1.1.1:443");
        // BasicPeer will use tls when SNI is set
        peer.sni = "one.one.one.one".to_string();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 424
  - hardcoded IP address
  - 

```rust
        // BasicPeer will use tls when SNI is set
        peer.sni = "one.one.one.one".to_string();
        // make a new connection to https://1.1.1.1
        let stream = connector.new_stream(&peer).await.unwrap();
        connector.release_stream(stream, peer.reuse_hash(), None);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 492
  - hardcoded IP address
  - 

```rust
    async fn test_connector_bind_to() {
        // connect to remote while bind to localhost will fail
        let peer = BasicPeer::new("240.0.0.1:80");
        let mut conf = ConnectorOptions::new(1);
        conf.bind_to_v4.push("127.0.0.1:0".parse().unwrap());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 2 (possible) Infractions 


- Line 435
  - mock
  - 

```rust
    const MOCK_UDS_PATH: &str = "/tmp/test_unix_transport_connector.sock";

    // one-off mock server
    #[cfg(unix)]
    async fn mock_connect_server() {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 455
  - mock
  - 

```rust
        let connector = TransportConnector::new(None);
        let peer = BasicPeer::new_uds(MOCK_UDS_PATH).unwrap();
        // make a new connection to mock uds
        let mut stream = connector.new_stream(&peer).await.unwrap();
        let mut buf = [0; 9];
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 295
  - actual
  - 

```rust
}

// Perform the actual L4 and tls connection steps while respecting the peer's
// connection timeout if there is one
async fn do_connect<P: Peer + Send + Sync>(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 319
  - actual
  - 

```rust
}

// Perform the actual L4 and tls connection steps with no timeout
async fn do_connect_inner<P: Peer + Send + Sync>(
    peer: &P,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 92: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .iter()
            .map(|v4| {
                let ip = v4.parse().unwrap();
                SocketAddr::new(ip, 0)
            })
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
            .iter()
            .map(|v6| {
                let ip = v6.parse().unwrap();
                SocketAddr::new(ip, 0)
            })
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 260: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        debug!("Try to keepalive client session");
        let stream = Arc::new(Mutex::new(stream));
        let locked_stream = stream.clone().try_lock_owned().unwrap(); // safe as we just created it
        let (notify_close, watch_use) = self.connection_pool.put(&meta, stream);
        let pool = self.connection_pool.clone(); //clone the arc
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 411: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let peer = BasicPeer::new("1.1.1.1:80");
        // make a new connection to 1.1.1.1
        let stream = connector.new_stream(&peer).await.unwrap();
        connector.release_stream(stream, peer.reuse_hash(), None);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 414: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        connector.release_stream(stream, peer.reuse_hash(), None);

        let (_, reused) = connector.get_stream(&peer).await.unwrap();
        assert!(reused);
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 425: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        peer.sni = "one.one.one.one".to_string();
        // make a new connection to https://1.1.1.1
        let stream = connector.new_stream(&peer).await.unwrap();
        connector.release_stream(stream, peer.reuse_hash(), None);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 428: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        connector.release_stream(stream, peer.reuse_hash(), None);

        let (_, reused) = connector.get_stream(&peer).await.unwrap();
        assert!(reused);
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 439: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    async fn mock_connect_server() {
        let _ = std::fs::remove_file(MOCK_UDS_PATH);
        let listener = UnixListener::bind(MOCK_UDS_PATH).unwrap();
        if let Ok((mut stream, _addr)) = listener.accept().await {
            stream.write_all(b"it works!").await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 441: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let listener = UnixListener::bind(MOCK_UDS_PATH).unwrap();
        if let Ok((mut stream, _addr)) = listener.accept().await {
            stream.write_all(b"it works!").await.unwrap();
            // wait a bit so that the client can read
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 454: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // create a new service at /tmp
        let connector = TransportConnector::new(None);
        let peer = BasicPeer::new_uds(MOCK_UDS_PATH).unwrap();
        // make a new connection to mock uds
        let mut stream = connector.new_stream(&peer).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 456: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let peer = BasicPeer::new_uds(MOCK_UDS_PATH).unwrap();
        // make a new connection to mock uds
        let mut stream = connector.new_stream(&peer).await.unwrap();
        let mut buf = [0; 9];
        let _ = stream.read(&mut buf).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 458: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut stream = connector.new_stream(&peer).await.unwrap();
        let mut buf = [0; 9];
        let _ = stream.read(&mut buf).await.unwrap();
        assert_eq!(&buf, b"it works!");
        connector.release_stream(stream, peer.reuse_hash(), None);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 462: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        connector.release_stream(stream, peer.reuse_hash(), None);

        let (_, reused) = connector.get_stream(&peer).await.unwrap();
        assert!(reused);
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 494: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let peer = BasicPeer::new("240.0.0.1:80");
        let mut conf = ConnectorOptions::new(1);
        conf.bind_to_v4.push("127.0.0.1:0".parse().unwrap());
        let connector = TransportConnector::new(Some(conf));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 393: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/connectors/mod.rs` (line 393)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
#[cfg(test)]
#[cfg(feature = "any_tls")]
mod tests {
    use pingora_error::ErrorType;
    use tls::Connector;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 407: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/connectors/mod.rs` (line 407)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_connect() {
        let connector = TransportConnector::new(None);
        let peer = BasicPeer::new("1.1.1.1:80");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 419: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/connectors/mod.rs` (line 419)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_connect_tls() {
        let connector = TransportConnector::new(None);
        let mut peer = BasicPeer::new("1.1.1.1:443");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 448: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/connectors/mod.rs` (line 448)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn test_connect_uds() {
        tokio::spawn(async {
            mock_connect_server().await;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 478: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/connectors/mod.rs` (line 478)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_conn_timeout() {
        do_test_conn_timeout(None).await;
    }
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 483: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/connectors/mod.rs` (line 483)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_conn_timeout_with_offload() {
        let mut conf = ConnectorOptions::new(8);
        conf.offload_threadpool = Some((2, 2));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 490: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/connectors/mod.rs` (line 490)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_connector_bind_to() {
        // connect to remote while bind to localhost will fail
        let peer = BasicPeer::new("240.0.0.1:80");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 522: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/connectors/mod.rs` (line 522)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_do_connect_with_total_timeout() {
        let mut peer = BasicPeer::new(BLACK_HOLE);
        peer.options.total_connection_timeout = Some(std::time::Duration::from_millis(1));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 531: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/connectors/mod.rs` (line 531)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_tls_connect_timeout_supersedes_total() {
        let mut peer = BasicPeer::new(BLACK_HOLE);
        peer.options.total_connection_timeout = Some(std::time::Duration::from_millis(10));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 541: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/connectors/mod.rs` (line 541)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_do_connect_without_total_timeout() {
        let peer = BasicPeer::new(BLACK_HOLE);
        let (etype, context) = get_do_connect_failure_with_peer(&peer).await;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym