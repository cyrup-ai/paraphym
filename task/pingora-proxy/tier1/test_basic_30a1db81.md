# `forks/pingora/pingora-proxy/tests/test_basic.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-proxy
- **File Hash**: 30a1db81  
- **Timestamp**: 2025-10-10T02:16:01.367576+00:00  
- **Lines of Code**: 823

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 823 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 35
  - hardcoded URL
  - 

```rust
async fn test_origin_alive() {
    init();
    let res = reqwest::get("http://127.0.0.1:8000/").await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let headers = res.headers();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 46
  - hardcoded URL
  - 

```rust
async fn test_simple_proxy() {
    init();
    let res = reqwest::get("http://127.0.0.1:6147").await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 66
  - hardcoded IP address
  - 

```rust
        .parse::<std::net::SocketAddr>()
        .unwrap();
    assert_eq!(sockaddr.ip().to_string(), "127.0.0.2");
    assert!(is_specified_port(sockaddr.port()));

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 83
  - hardcoded URL
  - 

```rust

    let res = client
        .get("https://127.0.0.1:6150")
        .header("sni", "openrusty.org")
        .send()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 109
  - hardcoded IP address
  - 

```rust
        .parse::<std::net::SocketAddr>()
        .unwrap();
    assert_eq!(sockaddr.ip().to_string(), "127.0.0.2");
    assert!(is_specified_port(sockaddr.port()));

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 126
  - hardcoded URL
  - 

```rust

    let res = client
        .get("https://127.0.0.1:6150")
        .header("sni", "openrusty.org")
        .header("x-h2", "true")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 152
  - hardcoded IP address
  - 

```rust
        .parse::<std::net::SocketAddr>()
        .unwrap();
    assert_eq!(sockaddr.ip().to_string(), "127.0.0.2");
    assert!(is_specified_port(sockaddr.port()));

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 168
  - hardcoded URL
  - 

```rust

    let mut req = hyper::Request::builder()
        .uri("http://127.0.0.1:6146")
        .body(Body::empty())
        .unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 190
  - hardcoded URL
  - 

```rust

    let mut req = hyper::Request::builder()
        .uri("http://127.0.0.1:6146")
        .body(Body::empty())
        .unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 213
  - hardcoded URL
  - 

```rust

    let res = client
        .get("https://127.0.0.1:6150")
        .header("x-h2", "true")
        .header("host-override", "test.com")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 239
  - hardcoded URL
  - 

```rust

    let res = client
        .get("https://127.0.0.1:6150/echo")
        .header("sni", "openrusty.org")
        .header("x-h2", "true")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 264
  - hardcoded URL
  - 

```rust

    let res = client
        .get("https://127.0.0.1:6150/echo")
        .header("sni", "openrusty.org")
        .body(payload)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 286
  - hardcoded URL
  - 

```rust

    let res = client
        .head("https://127.0.0.1:6150/set_content_length")
        .header("sni", "openrusty.org")
        .header("x-h2", "true")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 323
  - hardcoded IP address
  - 

```rust
        .parse::<std::net::SocketAddr>()
        .unwrap();
    assert_eq!(sockaddr.ip().to_string(), "127.0.0.2");
    assert!(is_specified_port(sockaddr.port()));

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 337
  - hardcoded URL
  - 

```rust

    let res = client
        .get("http://127.0.0.1:6147")
        .header("x-uds-peer", "1") // force upstream peer to be UDS
        .send()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 374
  - hardcoded URL
  - 

```rust
        // load conns into pool
        let res = client
            .get("http://127.0.0.1:6147")
            .header("x-port", port)
            .send()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 383
  - hardcoded URL
  - 

```rust

    let res = client
        .get("http://127.0.0.1:6147/bad_lb")
        .header("x-port", port)
        .send()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 403
  - hardcoded URL
  - 

```rust
        // load conn into pool
        let res = client
            .get("http://127.0.0.1:6147")
            .header("x-port", port)
            .send()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 412
  - hardcoded URL
  - 

```rust

    let res = client
        .post("http://127.0.0.1:6147/bad_lb")
        .header("x-port", port)
        .send()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 431
  - hardcoded URL
  - 

```rust
        // load conn into pool
        let res = client
            .get("http://127.0.0.1:6147")
            .header("x-port", port)
            .send()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 440
  - hardcoded URL
  - 

```rust

    let res = client
        .post("http://127.0.0.1:6147/bad_lb")
        .header("x-port", port)
        .body("cat!")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 461
  - hardcoded URL
  - 

```rust
        // load conn into pool
        let res = client
            .get("http://127.0.0.1:6147")
            .header("x-port", port)
            .send()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 470
  - hardcoded URL
  - 

```rust

    let res = client
        .post("http://127.0.0.1:6147/bad_lb")
        .header("x-port", port)
        .body(large_body)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 498
  - hardcoded URL
  - 

```rust
    let client = reqwest::Client::new();
    let res = client
        .get("http://127.0.0.1:6149/tls_verify")
        .send()
        .await
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 513
  - hardcoded URL
  - 

```rust

    let res = client
        .get("http://127.0.0.1:6149/tls_verify")
        .header("sni", "openrusty.org")
        .header("verify", "1")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 531
  - hardcoded URL
  - 

```rust

    let res = client
        .get("http://127.0.0.1:6149/tls_verify")
        .header("verify", "1")
        .header("verify_host", "1")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 548
  - hardcoded URL
  - 

```rust

    let res = client
        .get("http://127.0.0.1:6149/tls_verify")
        .header("sni", "openrusty.org")
        .header("verify", "1")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 566
  - hardcoded URL
  - 

```rust

    let res = client
        .get("http://127.0.0.1:6149/tls_verify")
        .header("sni", "d_g.openrusty.org")
        .header("verify", "1")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 584
  - hardcoded URL
  - 

```rust

    let res = client
        .get("http://127.0.0.1:6149/tls_verify")
        .header("sni", "open_rusty.org")
        .header("verify", "1")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 604
  - hardcoded URL
  - 

```rust

    let res = client
        .get("http://127.0.0.1:6149/tls_verify")
        .header("sni", "open_rusty.org")
        .header("alt", "openrusty.org")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 623
  - hardcoded URL
  - 

```rust

    let res = client
        .get("http://127.0.0.1:6149/tls_verify")
        .header("sni", "open_rusty.org")
        .header("alt", "d_g.openrusty.org")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 642
  - hardcoded URL
  - 

```rust

    let res = client
        .get("http://127.0.0.1:6149/tls_verify")
        .header("sni", "open_rusty.org")
        .header("alt", "open_rusty.org")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 662
  - hardcoded URL
  - 

```rust
    let client = reqwest::ClientBuilder::new().gzip(false).build().unwrap();
    let res = client
        .get("http://127.0.0.1:6147/no_compression")
        .header("accept-encoding", "gzip")
        .send()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 675
  - hardcoded URL
  - 

```rust
    let client = reqwest::ClientBuilder::new().gzip(true).build().unwrap();
    let res = client
        .get("http://127.0.0.1:6147/no_compression")
        .header("accept-encoding", "gzip")
        .send()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 693
  - hardcoded URL
  - 

```rust
    let client = reqwest::ClientBuilder::new().gzip(false).build().unwrap();
    let res = client
        .get("http://127.0.0.1:6147/no_compression")
        // tell the test proxy to use downstream compression module instead of upstream
        .header("x-downstream-compression", "1")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 708
  - hardcoded URL
  - 

```rust
    let client = reqwest::ClientBuilder::new().gzip(true).build().unwrap();
    let res = client
        .get("http://127.0.0.1:6147/no_compression")
        .header("accept-encoding", "gzip")
        .send()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 724
  - hardcoded URL
  - 

```rust
    // default keep-alive
    let client = reqwest::ClientBuilder::new().build().unwrap();
    let res = client.get("http://127.0.0.1:6147").send().await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let headers = res.headers();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 735
  - hardcoded URL
  - 

```rust
    let client = reqwest::ClientBuilder::new().build().unwrap();
    let res = client
        .get("http://127.0.0.1:6147")
        .header("connection", "close")
        .send()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 755
  - hardcoded URL
  - 

```rust

    let res = client
        .get("http://127.0.0.1:6149/tls_verify")
        .header("x-port", "8444")
        .header("sni", "openrusty.org")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 775
  - hardcoded URL
  - 

```rust

    let res = client
        .get("http://127.0.0.1:6149/tls_verify")
        .header("x-port", "8444")
        .header("sni", "openrusty.org")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 796
  - hardcoded URL
  - 

```rust

    let res = client
        .get("http://127.0.0.1:6149/")
        .header("x-port", "8444")
        .header("sni", "openrusty.org")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 824
  - hardcoded URL
  - 

```rust

    let req = client
        .get("http://127.0.0.1:6149/")
        .header("x-port", "8444")
        .header("sni", "openrusty.org")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 837
  - hardcoded URL
  - 

```rust
    // different cert no re-use
    let res = client
        .get("http://127.0.0.1:6149/")
        .header("x-port", "8444")
        .header("sni", "openrusty.org")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 859
  - hardcoded URL
  - 

```rust

    let req = client
        .get("http://127.0.0.1:6149/")
        .header("sni", "dog.openrusty.org")
        .header("verify", "1");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 868
  - hardcoded URL
  - 

```rust
    // disable 'verify' no re-use
    let res = client
        .get("http://127.0.0.1:6149/")
        .header("sni", "dog.openrusty.org")
        .header("verify", "0")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 886
  - hardcoded URL
  - 

```rust

    let req = client
        .get("http://127.0.0.1:6149/")
        .header("sni", "cat.openrusty.org")
        .header("verify", "1")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 896
  - hardcoded URL
  - 

```rust
    // disable 'verify_host' no re-use
    let res = client
        .get("http://127.0.0.1:6149/")
        .header("sni", "cat.openrusty.org")
        .header("verify", "1")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 915
  - hardcoded URL
  - 

```rust

    let req = client
        .get("http://127.0.0.1:6149/")
        .header("sni", "openrusty.org")
        .header("alt", "cat.com")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 926
  - hardcoded URL
  - 

```rust
    // use alt-cn no reuse
    let res = client
        .get("http://127.0.0.1:6149/")
        .header("sni", "openrusty.org")
        .header("alt", "dog.com")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 942
  - hardcoded URL
  - 

```rust
async fn test_error_before_headers_sent() {
    init();
    let url = "http://127.0.0.1:6146/sleep/test_error_before_headers_sent.txt";

    let tcp = TcpStream::connect("127.0.0.1:6146").await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 971
  - hardcoded URL
  - 

```rust
async fn test_error_after_headers_sent_rst_received() {
    init();
    let url = "http://127.0.0.1:6146/connection_die/test_error_after_headers_sent_rst_received.txt";

    let tcp = TcpStream::connect("127.0.0.1:6146").await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 997
  - hardcoded URL
  - 

```rust
async fn test_103() {
    init();
    let res = reqwest::get("http://127.0.0.1:6147/103").await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let headers = res.headers();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1008
  - hardcoded URL
  - 

```rust
async fn test_103_die() {
    init();
    let res = reqwest::get("http://127.0.0.1:6147/103-die").await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_GATEWAY);
}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 35: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
async fn test_origin_alive() {
    init();
    let res = reqwest::get("http://127.0.0.1:8000/").await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 39: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let headers = res.headers();
    assert_eq!(headers[header::CONTENT_LENGTH], "13");
    let body = res.text().await.unwrap();
    assert_eq!(body, "Hello World!\n");
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 46: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
async fn test_simple_proxy() {
    init();
    let res = reqwest::get("http://127.0.0.1:6147").await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 56: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .unwrap()
        .parse::<std::net::SocketAddr>()
        .unwrap();
    assert_eq!(sockaddr.ip().to_string(), "127.0.0.1");
    assert!(is_specified_port(sockaddr.port()));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 54: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let sockaddr = headers["x-client-addr"]
        .to_str()
        .unwrap()
        .parse::<std::net::SocketAddr>()
        .unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 65: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .unwrap()
        .parse::<std::net::SocketAddr>()
        .unwrap();
    assert_eq!(sockaddr.ip().to_string(), "127.0.0.2");
    assert!(is_specified_port(sockaddr.port()));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 63: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let sockaddr = headers["x-upstream-client-addr"]
        .to_str()
        .unwrap()
        .parse::<std::net::SocketAddr>()
        .unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 69: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    assert!(is_specified_port(sockaddr.port()));

    let body = res.text().await.unwrap();
    assert_eq!(body, "Hello World!\n");
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 80: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let res = client
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 87: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), reqwest::StatusCode::OK);
    assert_eq!(res.version(), reqwest::Version::HTTP_2);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 99: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .unwrap()
        .parse::<std::net::SocketAddr>()
        .unwrap();
    assert_eq!(sockaddr.ip().to_string(), "127.0.0.1");
    assert!(is_specified_port(sockaddr.port()));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 97: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let sockaddr = headers["x-client-addr"]
        .to_str()
        .unwrap()
        .parse::<std::net::SocketAddr>()
        .unwrap();
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .unwrap()
        .parse::<std::net::SocketAddr>()
        .unwrap();
    assert_eq!(sockaddr.ip().to_string(), "127.0.0.2");
    assert!(is_specified_port(sockaddr.port()));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 106: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let sockaddr = headers["x-upstream-client-addr"]
        .to_str()
        .unwrap()
        .parse::<std::net::SocketAddr>()
        .unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 112: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    assert!(is_specified_port(sockaddr.port()));

    let body = res.text().await.unwrap();
    assert_eq!(body, "Hello World!\n");
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 123: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let res = client
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 131: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), reqwest::StatusCode::OK);
    assert_eq!(res.version(), reqwest::Version::HTTP_2);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 142: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .unwrap()
        .parse::<std::net::SocketAddr>()
        .unwrap();
    assert_eq!(sockaddr.ip().to_string(), "127.0.0.1");
    assert!(is_specified_port(sockaddr.port()));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 140: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let sockaddr = headers["x-client-addr"]
        .to_str()
        .unwrap()
        .parse::<std::net::SocketAddr>()
        .unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 151: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .unwrap()
        .parse::<std::net::SocketAddr>()
        .unwrap();
    assert_eq!(sockaddr.ip().to_string(), "127.0.0.2");
    assert!(is_specified_port(sockaddr.port()));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 149: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let sockaddr = headers["x-upstream-client-addr"]
        .to_str()
        .unwrap()
        .parse::<std::net::SocketAddr>()
        .unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 155: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    assert!(is_specified_port(sockaddr.port()));

    let body = res.text().await.unwrap();
    assert_eq!(body, "Hello World!\n");
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 170: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .uri("http://127.0.0.1:6146")
        .body(Body::empty())
        .unwrap();
    req.headers_mut()
        .insert("x-h2", HeaderValue::from_bytes(b"true").unwrap());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 172: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .unwrap();
    req.headers_mut()
        .insert("x-h2", HeaderValue::from_bytes(b"true").unwrap());
    let res = client.request(req).await.unwrap();
    assert_eq!(res.status(), reqwest::StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 173: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    req.headers_mut()
        .insert("x-h2", HeaderValue::from_bytes(b"true").unwrap());
    let res = client.request(req).await.unwrap();
    assert_eq!(res.status(), reqwest::StatusCode::OK);
    assert_eq!(res.version(), reqwest::Version::HTTP_2);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 177: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    assert_eq!(res.version(), reqwest::Version::HTTP_2);

    let body = res.into_body().data().await.unwrap().unwrap();
    assert_eq!(body.as_ref(), b"Hello World!\n");
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 177: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    assert_eq!(res.version(), reqwest::Version::HTTP_2);

    let body = res.into_body().data().await.unwrap().unwrap();
    assert_eq!(body.as_ref(), b"Hello World!\n");
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 192: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .uri("http://127.0.0.1:6146")
        .body(Body::empty())
        .unwrap();
    req.headers_mut()
        .insert("x-h2", HeaderValue::from_bytes(b"true").unwrap());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 194: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .unwrap();
    req.headers_mut()
        .insert("x-h2", HeaderValue::from_bytes(b"true").unwrap());
    let res = client.request(req).await.unwrap();
    assert_eq!(res.status(), reqwest::StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 195: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    req.headers_mut()
        .insert("x-h2", HeaderValue::from_bytes(b"true").unwrap());
    let res = client.request(req).await.unwrap();
    assert_eq!(res.status(), reqwest::StatusCode::OK);
    assert_eq!(res.version(), reqwest::Version::HTTP_11);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 199: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    assert_eq!(res.version(), reqwest::Version::HTTP_11);

    let body = res.into_body().data().await.unwrap().unwrap();
    assert_eq!(body.as_ref(), b"Hello World!\n");
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 199: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    assert_eq!(res.version(), reqwest::Version::HTTP_11);

    let body = res.into_body().data().await.unwrap().unwrap();
    assert_eq!(body.as_ref(), b"Hello World!\n");
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 210: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let res = client
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 218: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), reqwest::StatusCode::OK);
    assert_eq!(res.version(), reqwest::Version::HTTP_2);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 223: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let headers = res.headers();
    assert_eq!(headers[header::CONTENT_LENGTH], "13");
    let body = res.text().await.unwrap();
    assert_eq!(body, "Hello World!\n");
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 234: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let payload = "test upload";
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 245: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), reqwest::StatusCode::OK);
    assert_eq!(res.version(), reqwest::Version::HTTP_2);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 248: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    assert_eq!(res.status(), reqwest::StatusCode::OK);
    assert_eq!(res.version(), reqwest::Version::HTTP_2);
    let body = res.text().await.unwrap();
    assert_eq!(body, payload);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 259: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let payload = "test upload";
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 269: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), reqwest::StatusCode::OK);
    assert_eq!(res.version(), reqwest::Version::HTTP_2);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 272: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    assert_eq!(res.status(), reqwest::StatusCode::OK);
    assert_eq!(res.version(), reqwest::Version::HTTP_2);
    let body = res.text().await.unwrap();
    assert_eq!(body, payload);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 283: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let res = client
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), reqwest::StatusCode::OK);
    assert_eq!(res.version(), reqwest::Version::HTTP_2);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 295: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    assert_eq!(res.status(), reqwest::StatusCode::OK);
    assert_eq!(res.version(), reqwest::Version::HTTP_2);
    let body = res.text().await.unwrap();
    // should not be any body, despite content-length
    assert_eq!(body, "");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 307: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let client = Client::unix();

    let res = client.get(url).await.unwrap();

    assert_eq!(res.status(), reqwest::StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 322: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .unwrap()
        .parse::<std::net::SocketAddr>()
        .unwrap();
    assert_eq!(sockaddr.ip().to_string(), "127.0.0.2");
    assert!(is_specified_port(sockaddr.port()));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 320: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let sockaddr = headers["x-upstream-client-addr"]
        .to_str()
        .unwrap()
        .parse::<std::net::SocketAddr>()
        .unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 326: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    assert!(is_specified_port(sockaddr.port()));

    let body = hyper::body::to_bytes(body).await.unwrap();
    assert_eq!(body.as_ref(), b"Hello World!\n");
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 341: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .unwrap()
        .parse::<std::net::SocketAddr>()
        .unwrap();
    assert_eq!(sockaddr.ip().to_string(), "127.0.0.1");
    assert!(is_specified_port(sockaddr.port()));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 350: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let sockaddr = headers["x-client-addr"]
        .to_str()
        .unwrap()
        .parse::<std::net::SocketAddr>()
        .unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 362: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    );

    let body = res.text().await.unwrap();
    assert_eq!(body, "Hello World!\n");
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 378: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 387: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();

    // retry gives 200
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 391: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    // retry gives 200
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.text().await.unwrap();
    assert_eq!(body, "dog!\n");
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 407: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 416: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 419: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

    assert_eq!(res.status(), StatusCode::OK);
    let body = res.text().await.unwrap();
    assert_eq!(body, "dog!\n");
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 435: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 445: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 448: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

    assert_eq!(res.status(), StatusCode::OK);
    let body = res.text().await.unwrap();
    assert_eq!(body, "cat!\n");
}
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let client = reqwest::Client::new();
    let port = "8001"; // special port to avoid unexpected connection reuse from other tests
    let large_body = String::from_utf8(vec![b'e'; 1024 * 64 + 1]).unwrap();

    for _ in 1..3 {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 465: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 475: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();

    // 502, body larger than buffer limit
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 501: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 518: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 536: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 554: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 572: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 590: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::BAD_GATEWAY);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 611: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 630: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 649: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::BAD_GATEWAY);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 660: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    // disable reqwest gzip support to check compression headers and body
    // otherwise reqwest will decompress and strip the headers
    let client = reqwest::ClientBuilder::new().gzip(false).build().unwrap();
    let res = client
        .get("http://127.0.0.1:6147/no_compression")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 666: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.headers().get("Content-Encoding").unwrap(), "gzip");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 669: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.headers().get("Content-Encoding").unwrap(), "gzip");
    let body = res.bytes().await.unwrap();
    assert!(body.len() < 32);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 673: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

    // Next let reqwest decompress to validate the data
    let client = reqwest::ClientBuilder::new().gzip(true).build().unwrap();
    let res = client
        .get("http://127.0.0.1:6147/no_compression")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 679: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.bytes().await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 681: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.bytes().await.unwrap();
    assert_eq!(body.as_ref(), &[b'B'; 32]);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 691: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    // disable reqwest gzip support to check compression headers and body
    // otherwise reqwest will decompress and strip the headers
    let client = reqwest::ClientBuilder::new().gzip(false).build().unwrap();
    let res = client
        .get("http://127.0.0.1:6147/no_compression")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 699: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.headers().get("Content-Encoding").unwrap(), "gzip");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 702: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.headers().get("Content-Encoding").unwrap(), "gzip");
    let body = res.bytes().await.unwrap();
    assert!(body.len() < 32);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 706: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

    // Next let reqwest decompress to validate the data
    let client = reqwest::ClientBuilder::new().gzip(true).build().unwrap();
    let res = client
        .get("http://127.0.0.1:6147/no_compression")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 712: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.bytes().await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 714: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.bytes().await.unwrap();
    assert_eq!(body.as_ref(), &[b'B'; 32]);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 723: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

    // default keep-alive
    let client = reqwest::ClientBuilder::new().build().unwrap();
    let res = client.get("http://127.0.0.1:6147").send().await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 724: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    // default keep-alive
    let client = reqwest::ClientBuilder::new().build().unwrap();
    let res = client.get("http://127.0.0.1:6147").send().await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 729: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    assert_eq!(headers[header::CONTENT_LENGTH], "13");
    assert_eq!(headers[header::CONNECTION], "keep-alive");
    let body = res.text().await.unwrap();
    assert_eq!(body, "Hello World!\n");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 733: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

    // close
    let client = reqwest::ClientBuilder::new().build().unwrap();
    let res = client
        .get("http://127.0.0.1:6147")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 739: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 744: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    assert_eq!(headers[header::CONTENT_LENGTH], "13");
    assert_eq!(headers[header::CONNECTION], "close");
    let body = res.text().await.unwrap();
    assert_eq!(body, "Hello World!\n");
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 762: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();

    // 400: because no cert
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 783: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();

    // 400: because no intermediate cert
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 805: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 811: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
#[cfg(feature = "any_tls")]
async fn assert_reuse(req: reqwest::RequestBuilder) {
    req.try_clone().unwrap().send().await.unwrap();
    let res = req.send().await.unwrap();
    let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 811: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
#[cfg(feature = "any_tls")]
async fn assert_reuse(req: reqwest::RequestBuilder) {
    req.try_clone().unwrap().send().await.unwrap();
    let res = req.send().await.unwrap();
    let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 812: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
async fn assert_reuse(req: reqwest::RequestBuilder) {
    req.try_clone().unwrap().send().await.unwrap();
    let res = req.send().await.unwrap();
    let headers = res.headers();
    assert!(headers.get("x-conn-reuse").is_some());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 846: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 873: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 902: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 933: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 944: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let url = "http://127.0.0.1:6146/sleep/test_error_before_headers_sent.txt";

    let tcp = TcpStream::connect("127.0.0.1:6146").await.unwrap();
    let (mut client, h2) = client::handshake(tcp).await.unwrap();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 945: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

    let tcp = TcpStream::connect("127.0.0.1:6146").await.unwrap();
    let (mut client, h2) = client::handshake(tcp).await.unwrap();

    tokio::spawn(async move {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 948: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

    tokio::spawn(async move {
        h2.await.unwrap();
    });

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 956: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .header("x-abort", "true")
        .body(())
        .unwrap();

    let (response, mut _stream) = client.send_request(request, true).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 958: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .unwrap();

    let (response, mut _stream) = client.send_request(request, true).unwrap();

    let response = response.await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 960: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let (response, mut _stream) = client.send_request(request, true).unwrap();

    let response = response.await.unwrap();
    let mut body = response.into_body();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 973: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let url = "http://127.0.0.1:6146/connection_die/test_error_after_headers_sent_rst_received.txt";

    let tcp = TcpStream::connect("127.0.0.1:6146").await.unwrap();
    let (mut client, h2) = client::handshake(tcp).await.unwrap();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 974: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

    let tcp = TcpStream::connect("127.0.0.1:6146").await.unwrap();
    let (mut client, h2) = client::handshake(tcp).await.unwrap();

    tokio::spawn(async move {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 977: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

    tokio::spawn(async move {
        h2.await.unwrap();
    });

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 980: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    });

    let request = Request::builder().uri(url).body(()).unwrap();

    let (response, mut _stream) = client.send_request(request, true).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 982: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let request = Request::builder().uri(url).body(()).unwrap();

    let (response, mut _stream) = client.send_request(request, true).unwrap();

    let response = response.await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 984: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let (response, mut _stream) = client.send_request(request, true).unwrap();

    let response = response.await.unwrap();
    let mut body = response.into_body();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 987: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let mut body = response.into_body();

    let chunk = body.data().await.unwrap();
    assert_eq!(chunk.unwrap(), Bytes::from_static(b"AAAAA"));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 990: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    assert_eq!(chunk.unwrap(), Bytes::from_static(b"AAAAA"));

    let err = body.data().await.unwrap().err().unwrap();
    assert_eq!(err.reason().unwrap(), h2::Reason::CANCEL);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 990: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    assert_eq!(chunk.unwrap(), Bytes::from_static(b"AAAAA"));

    let err = body.data().await.unwrap().err().unwrap();
    assert_eq!(err.reason().unwrap(), h2::Reason::CANCEL);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 997: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
async fn test_103() {
    init();
    let res = reqwest::get("http://127.0.0.1:6147/103").await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1001: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let headers = res.headers();
    assert_eq!(headers[header::CONTENT_LENGTH], "6");
    let body = res.text().await.unwrap();
    assert_eq!(body, "123456");
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1008: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
async fn test_103_die() {
    init();
    let res = reqwest::get("http://127.0.0.1:6147/103-die").await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_GATEWAY);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Orphaned Methods


### `is_specified_port()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/tests/test_basic.rs` (line 28)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
use utils::server_utils::init;

fn is_specified_port(port: u16) -> bool {
    (1..65535).contains(&port)
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym