# `forks/pingora/pingora-proxy/tests/test_upstream.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-proxy
- **File Hash**: d1e23a78  
- **Timestamp**: 2025-10-10T02:16:01.365194+00:00  
- **Lines of Code**: 2426

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 2426 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 2727
  - TODO
  - 

```rust
        let url = "http://127.0.0.1:6148/unique/test_cache_max_file_size_chunked_1/test3";
        let res = send_max_file_size_req(url, 1, None).await;
        // TODO: this can currently break with 500 when body arrives alongside header
        assert!(matches!(
            res.status(),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 874
  - stubby method name
  - mock_1xx_server

```rust
        // set up a one-off mock server
        // (warp / hyper don't have custom 1xx sending capabilities yet)
        async fn mock_1xx_server(port: u16, cc_header: &str) {
            use tokio::io::AsyncWriteExt;

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 894
  - stubby method name
  - mock_1xx_server

```rust

        tokio::spawn(async {
            mock_1xx_server(6151, "max-age=5").await;
        });
        // wait for server to start
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 926
  - stubby method name
  - mock_1xx_server

```rust

        tokio::spawn(async {
            mock_1xx_server(6152, "private, no-store").await;
        });
        // wait for server to start
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 945
  - stubby method name
  - mock_1xx_server

```rust
        sleep(Duration::from_millis(100)).await;
        tokio::spawn(async {
            mock_1xx_server(6152, "private, no-store").await;
        });
        // wait for server to start
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 874
  - stubby variable name
  - mock_1xx_server

```rust
        // set up a one-off mock server
        // (warp / hyper don't have custom 1xx sending capabilities yet)
        async fn mock_1xx_server(port: u16, cc_header: &str) {
            use tokio::io::AsyncWriteExt;

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 894
  - stubby variable name
  - mock_1xx_server

```rust

        tokio::spawn(async {
            mock_1xx_server(6151, "max-age=5").await;
        });
        // wait for server to start
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 926
  - stubby variable name
  - mock_1xx_server

```rust

        tokio::spawn(async {
            mock_1xx_server(6152, "private, no-store").await;
        });
        // wait for server to start
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 945
  - stubby variable name
  - mock_1xx_server

```rust
        sleep(Duration::from_millis(100)).await;
        tokio::spawn(async {
            mock_1xx_server(6152, "private, no-store").await;
        });
        // wait for server to start
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 29
  - hardcoded URL
  - 

```rust
async fn test_ip_binding() {
    init();
    let res = reqwest::get("http://127.0.0.1:6147/client_ip")
        .await
        .unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 34
  - hardcoded IP address
  - 

```rust
    assert_eq!(res.status(), StatusCode::OK);
    let headers = res.headers();
    assert_eq!(headers["x-client-ip"], "127.0.0.2");
}

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 44
  - hardcoded URL
  - 

```rust
    let client = reqwest::Client::new();
    let res = client
        .post("http://127.0.0.1:6147/duplex/")
        .body("b".repeat(1024 * 1024)) // 1 MB upload
        .timeout(Duration::from_secs(5))
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 60
  - hardcoded URL
  - 

```rust
async fn test_connection_die() {
    init();
    let res = reqwest::get("http://127.0.0.1:6147/connection_die")
        .await
        .unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 74
  - hardcoded URL
  - 

```rust
    let client = reqwest::Client::new();
    let res = client
        .post("http://127.0.0.1:6147/upload_connection_die/")
        .body("b".repeat(15 * 1024 * 1024)) // 15 MB upload
        .timeout(Duration::from_secs(5))
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 87
  - hardcoded URL
  - 

```rust
    let client = reqwest::Client::new();
    let res = client
        .post("http://127.0.0.1:6147/upload_connection_die/")
        .body("b".repeat(15 * 1024 * 1024)) // 15 MB upload
        .timeout(Duration::from_secs(5))
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 104
  - hardcoded URL
  - 

```rust
    let client = reqwest::Client::new();
    let res = client
        .post("http://127.0.0.1:6147/upload/")
        .body("b".repeat(15 * 1024 * 1024)) // 15 MB upload
        .timeout(Duration::from_secs(5))
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 120
  - hardcoded URL
  - 

```rust
    let client = reqwest::Client::new();
    let res = client
        .post("http://127.0.0.1:6147/test2")
        .header("x-close-on-response-before-downstream-finish", "1")
        .body("b".repeat(15 * 1024 * 1024)) // 15 MB upload
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 194
  - hardcoded URL
  - 

```rust

    let client = hyper::Client::new();
    let uri: hyper::Uri = "http://127.0.0.1:6147/download/".parse().unwrap();
    let req = hyper::Request::builder()
        .uri(uri)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 220
  - hardcoded URL
  - 

```rust

    let client = hyper::Client::new();
    let uri: hyper::Uri = "http://127.0.0.1:6147/download/".parse().unwrap();
    let req = hyper::Request::builder()
        .uri(uri)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 249
  - hardcoded URL
  - 

```rust
    async fn test_basic_caching() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_basic_caching/now";

        let res = reqwest::get(url).await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 282
  - hardcoded URL
  - 

```rust
    async fn test_purge() {
        init();
        let res = reqwest::get("http://127.0.0.1:6148/unique/test_purge/test2")
            .await
            .unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 290
  - hardcoded URL
  - 

```rust
        assert_eq!(res.text().await.unwrap(), "hello world");

        let res = reqwest::get("http://127.0.0.1:6148/unique/test_purge/test2")
            .await
            .unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 303
  - hardcoded URL
  - 

```rust
            .request(
                reqwest::Method::from_bytes(b"PURGE").unwrap(),
                "http://127.0.0.1:6148/unique/test_purge/test2",
            )
            .send()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 316
  - hardcoded URL
  - 

```rust
            .request(
                reqwest::Method::from_bytes(b"PURGE").unwrap(),
                "http://127.0.0.1:6148/unique/test_purge/test2",
            )
            .send()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 324
  - hardcoded URL
  - 

```rust
        assert_eq!(res.text().await.unwrap(), "");

        let res = reqwest::get("http://127.0.0.1:6148/unique/test_purge/test2")
            .await
            .unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 340
  - hardcoded URL
  - 

```rust
        let client = reqwest::Client::new();
        let res = client
            .get("http://127.0.0.1:6148/unique/test_cache_miss_convert/no_if_headers")
            .header("if-modified-since", "Wed, 19 Jan 2022 18:39:12 GMT")
            .send()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 354
  - hardcoded URL
  - 

```rust
        let client = reqwest::Client::new();
        let res = client
            .get("http://127.0.0.1:6148/unique/test_cache_miss_convert2/no_if_headers")
            .header("Range", "bytes=0-1")
            .send()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 374
  - hardcoded URL
  - 

```rust
        let client = reqwest::ClientBuilder::new().gzip(false).build().unwrap();
        let res = client
            .get("http://127.0.0.1:6148/unique/test_cache_downstream_compression/no_compression")
            .header("x-downstream-compression", "1")
            .header("accept-encoding", "gzip")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 390
  - hardcoded URL
  - 

```rust
        let client = reqwest::ClientBuilder::new().gzip(false).build().unwrap();
        let res = client
            .get("http://127.0.0.1:6148/unique/test_cache_downstream_compression/no_compression")
            .header("x-downstream-compression", "1")
            .header("accept-encoding", "gzip")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 412
  - hardcoded URL
  - 

```rust
        let client = reqwest::ClientBuilder::new().gzip(false).build().unwrap();
        let res = client
            .get("http://127.0.0.1:6148/unique/test_cache_downstream_decompression/gzip/index.html")
            .header("x-downstream-decompression", "1")
            .header("x-upstream-accept-encoding", "gzip")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 431
  - hardcoded URL
  - 

```rust
        let client = reqwest::ClientBuilder::new().gzip(false).build().unwrap();
        let res = client
            .get("http://127.0.0.1:6148/unique/test_cache_downstream_decompression/gzip/index.html")
            .header("x-downstream-decompression", "1")
            .header("x-upstream-accept-encoding", "gzip")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 450
  - hardcoded URL
  - 

```rust
        let client = reqwest::ClientBuilder::new().gzip(false).build().unwrap();
        let res = client
            .get("http://127.0.0.1:6148/unique/test_cache_downstream_decompression/gzip/index.html")
            .header("x-downstream-decompression", "1")
            .header("x-upstream-accept-encoding", "gzip")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 468
  - hardcoded URL
  - 

```rust
    async fn test_network_error_mid_response() {
        init();
        let url = "http://127.0.0.1:6148/sleep/test_network_error_mid_response.txt";

        let res = reqwest::Client::new()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 506
  - hardcoded URL
  - 

```rust
    async fn test_cache_upstream_revalidation() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_upstream_revalidation/revalidate_now";

        let res = reqwest::get(url).await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 543
  - hardcoded URL
  - 

```rust
    async fn test_cache_upstream_revalidation_appends_headers() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_cache_upstream_revalidation_appends_headers/cache_control";

        let res = reqwest::Client::new()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 596
  - hardcoded URL
  - 

```rust
    async fn test_force_miss() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_froce_miss/revalidate_now";

        let res = reqwest::get(url).await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 633
  - hardcoded URL
  - 

```rust
    async fn test_force_miss_stale() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_froce_miss_stale/revalidate_now";

        let res = reqwest::get(url).await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 675
  - hardcoded URL
  - 

```rust
    async fn test_cache_downstream_revalidation_etag() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_downstream_revalidation_etag/revalidate_now";
        let client = reqwest::Client::new();

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 744
  - hardcoded URL
  - 

```rust
    async fn test_cache_downstream_revalidation_last_modified() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_downstream_revalidation_last_modified/revalidate_now";
        let client = reqwest::Client::new();

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 813
  - hardcoded URL
  - 

```rust
    async fn test_cache_downstream_head() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_downstream_head/revalidate_now";
        let client = reqwest::Client::new();

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 858
  - hardcoded URL
  - 

```rust
            .request(
                reqwest::Method::from_bytes(b"PURGE").unwrap(),
                "http://127.0.0.1:6148/",
            )
            .header("x-bypass-cache", "1") // not to cache this one
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 881
  - hardcoded URL
  - 

```rust
                .unwrap();
            if let Ok((mut stream, _addr)) = listener.accept().await {
                stream.write_all(b"HTTP/1.1 103 Early Hints\r\nLink: <https://foo.bar>; rel=preconnect\r\n\r\n").await.unwrap();
                // wait a bit so that the client can read
                sleep(Duration::from_millis(100)).await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 891
  - hardcoded URL
  - 

```rust
        init();

        let url = "http://127.0.0.1:6148/unique/test_1xx_caching";

        tokio::spawn(async {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 923
  - hardcoded URL
  - 

```rust

        // 1xx shouldn't interfere with bypass
        let url = "http://127.0.0.1:6148/unique/test_1xx_bypass";

        tokio::spawn(async {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 966
  - hardcoded URL
  - 

```rust
        init();

        let url = "http://127.0.0.1:6148/unique/test_bypassed/cache_control";

        let res = reqwest::Client::new()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1010
  - hardcoded URL
  - 

```rust
        init();

        let url = "http://127.0.0.1:6148/unique/test_bypassed_304/cache_control";

        let res = reqwest::Client::new()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1067
  - hardcoded URL
  - 

```rust
        init();

        let url = "http://127.0.0.1:6148/unique/test_bypassed_private_304/cache_control";

        // cache fill
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1115
  - hardcoded URL
  - 

```rust
        init();

        let url = "http://127.0.0.1:6148/unique/test_bypassed_head/cache_control";

        // uncacheable, should bypass
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1157
  - hardcoded URL
  - 

```rust
    async fn test_eviction() {
        init();
        let url = "http://127.0.0.1:6148/file_maker/test_eviction".to_owned();

        // admit asset 1
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1241
  - hardcoded URL
  - 

```rust
    async fn test_cache_lock_miss_hit() {
        init();
        let url = "http://127.0.0.1:6148/sleep/test_cache_lock_miss_hit.txt";

        // no lock, parallel fetches to a slow origin are all misses
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1277
  - hardcoded URL
  - 

```rust

        // try with lock
        let url = "http://127.0.0.1:6148/sleep/test_cache_lock_miss_hit2.txt";
        let task1 = tokio::spawn(async move {
            let res = reqwest::Client::new()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1337
  - hardcoded URL
  - 

```rust
    async fn test_cache_lock_expired() {
        init();
        let url = "http://127.0.0.1:6148/sleep/test_cache_lock_expired.txt";

        // cache one
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1403
  - hardcoded URL
  - 

```rust
    async fn test_cache_lock_network_error() {
        init();
        let url = "http://127.0.0.1:6148/sleep/test_cache_lock_network_error.txt";

        let task1 = tokio::spawn(async move {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1463
  - hardcoded URL
  - 

```rust
    async fn test_cache_lock_uncacheable() {
        init();
        let url = "http://127.0.0.1:6148/sleep/test_cache_lock_uncacheable.txt";

        let task1 = tokio::spawn(async move {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1514
  - hardcoded URL
  - 

```rust
    async fn test_cache_lock_timeout() {
        init();
        let url = "http://127.0.0.1:6148/sleep/test_cache_lock_timeout.txt";

        let task1 = tokio::spawn(async move {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1585
  - hardcoded URL
  - 

```rust
    async fn test_cache_lock_wait_timeout() {
        init();
        let url = "http://127.0.0.1:6148/sleep/test_cache_lock_wait_timeout.txt";

        let mut handles = vec![];
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1625
  - hardcoded URL
  - 

```rust
    async fn test_cache_serve_stale_network_error() {
        init();
        let url = "http://127.0.0.1:6148/sleep/test_cache_serve_stale_network_error.txt";

        let res = reqwest::Client::new()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1657
  - hardcoded URL
  - 

```rust
        init();
        let url =
            "http://127.0.0.1:6148/sleep/test_cache_serve_stale_network_error_mid_response.txt";

        let res = reqwest::Client::new()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1690
  - hardcoded URL
  - 

```rust
    async fn test_cache_serve_stale_on_500() {
        init();
        let url = "http://127.0.0.1:6148/sleep/test_cache_serve_stale_on_500.txt";

        let res = reqwest::Client::new()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1721
  - hardcoded URL
  - 

```rust
    async fn test_stale_while_revalidate_many_readers() {
        init();
        let url = "http://127.0.0.1:6148/sleep/test_stale_while_revalidate_many_readers.txt";

        // cache one
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1779
  - hardcoded URL
  - 

```rust
    async fn test_stale_while_revalidate_single_request() {
        init();
        let url = "http://127.0.0.1:6148/sleep/test_stale_while_revalidate_single_request.txt";

        // cache one
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1825
  - hardcoded URL
  - 

```rust
    async fn test_cache_streaming_partial_body() {
        init();
        let url = "http://127.0.0.1:6148/slow_body/test_cache_streaming_partial_body.txt";
        let task1 = tokio::spawn(async move {
            let res = reqwest::Client::new()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1891
  - hardcoded URL
  - 

```rust
        // multiple streaming writers don't conflict
        init();
        let url = "http://127.0.0.1:6148/slow_body/test_cache_streaming_multiple_writers.txt";
        let task1 = tokio::spawn(async move {
            let res = reqwest::Client::new()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1927
  - hardcoded URL
  - 

```rust
    async fn test_range_request() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_range_request/now";

        let res = reqwest::Client::new()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2001
  - hardcoded URL
  - 

```rust
    async fn test_multipart_range_request() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_multipart_range_request/now";

        // 1st multipart range request - uncached
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2195
  - hardcoded URL
  - 

```rust
    async fn test_single_then_mutltipart_range_request() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_single_then_multipart_range_request/now";

        // 1st request - single range request
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2272
  - hardcoded URL
  - 

```rust
    async fn test_caching_when_downstream_bails() {
        init();
        let url = "http://127.0.0.1:6148/slow_body/test_caching_when_downstream_bails/";

        tokio::spawn(async move {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2352
  - hardcoded URL
  - 

```rust
    async fn test_vary_caching() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_vary_caching/vary";
        let vary_field = "Accept";

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2392
  - hardcoded URL
  - 

```rust
    async fn test_vary_caching_ignored_vary_header() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_vary_caching_ignored_vary_header/vary";
        let vary_field = "Some-Ignored-Vary-Header";

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2428
  - hardcoded URL
  - 

```rust
    async fn test_vary_some_ignored() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_vary_some_ignored/vary";
        let vary_header = "Accept, SomeIgnoredVaryHeader";

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2542
  - hardcoded URL
  - 

```rust
    async fn test_vary_dup_header_some_ignored() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_vary_dup_header_some_ignored/vary";
        let first_vary_header = "SomeIgnoredVaryHeader";
        let dup_vary_header = "Accept";
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2605
  - hardcoded URL
  - 

```rust
    async fn test_vary_purge() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_vary_purge/vary";
        let vary_field = "Accept";
        let opt1 = "image/png";
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2665
  - hardcoded URL
  - 

```rust
    async fn test_cache_max_file_size() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_cache_max_file_size_100/now";

        let res = send_max_file_size_req(url, 100, None).await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2680
  - hardcoded URL
  - 

```rust
        assert_eq!(res.text().await.unwrap(), "hello world");

        let url = "http://127.0.0.1:6148/unique/test_cache_max_file_size_1/now";
        let res = send_max_file_size_req(url, 1, None).await;
        assert_eq!(res.status(), StatusCode::OK);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2710
  - hardcoded URL
  - 

```rust
    async fn test_cache_max_file_size_chunked() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_cache_max_file_size_chunked_100/test3";

        let res = send_max_file_size_req(url, 100, None).await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2725
  - hardcoded URL
  - 

```rust
        assert_eq!(res.text().await.unwrap(), "hello world");

        let url = "http://127.0.0.1:6148/unique/test_cache_max_file_size_chunked_1/test3";
        let res = send_max_file_size_req(url, 1, None).await;
        // TODO: this can currently break with 500 when body arrives alongside header
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2767
  - hardcoded URL
  - 

```rust
    async fn test_cache_max_file_size_range() {
        init();
        let url = "http://127.0.0.1:6148/unique/test_cache_max_file_size_range_100/now";

        let res = send_max_file_size_req(url, 100, Some((1, 4))).await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2785
  - hardcoded URL
  - 

```rust

        // disable downstream ranging on max file size exceeded
        let url = "http://127.0.0.1:6148/unique/test_cache_max_file_size_range_1/now";
        let res = send_max_file_size_req(url, 1, Some((1, 4))).await;
        assert_eq!(res.status(), StatusCode::OK);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2806
  - hardcoded URL
  - 

```rust
    async fn test_cache_h2_premature_end() {
        init();
        let url = "http://127.0.0.1:6148/set_content_length/test_cache_h2_premature_end.txt";
        // try to fill cache
        reqwest::Client::new()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2837
  - hardcoded URL
  - 

```rust
        init();
        let test_url =
            "http://127.0.0.1:6148/unique/test_cache_bypass_downstream_range/cache_control/";

        let res = reqwest::Client::new()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2875
  - hardcoded URL
  - 

```rust

        let test_url =
            "http://127.0.0.1:6148/unique/test_cache_downstream_head_miss_conn_close/sleep/";

        let res = reqwest::Client::new()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2902
  - hardcoded URL
  - 

```rust

        let test_url =
            "https://127.0.0.1:6153/unique/test_cache_downstream_head_miss_conn_close/sleep/";

        let client = reqwest::Client::builder()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 2 (possible) Infractions 


- Line 872
  - mock
  - 

```rust
        // 1xx shouldn't interfere with HTTP caching

        // set up a one-off mock server
        // (warp / hyper don't have custom 1xx sending capabilities yet)
        async fn mock_1xx_server(port: u16, cc_header: &str) {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 31: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let res = reqwest::get("http://127.0.0.1:6147/client_ip")
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


### Line 49: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    let headers = res.headers();
    assert_eq!(headers["Connection"], "keep-alive");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 53: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    assert_eq!(headers["Connection"], "keep-alive");
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.text().await.unwrap();
    assert_eq!(body.len(), 64 * 5);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 62: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let res = reqwest::get("http://127.0.0.1:6147/connection_die")
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.text().await;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 79: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    // should get 200 status before connection dies
    assert_eq!(res.status(), StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 93: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    // should get 200 status before connection dies
    assert_eq!(res.status(), StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 109: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.text().await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 111: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.text().await.unwrap();
    assert_eq!(body.len(), 64 * 5);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 126: `.unwrap()`

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


### Line 130: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let headers = res.headers();
    assert_eq!(headers["Connection"], "close");
    let body = res.text().await.unwrap();
    assert_eq!(body.len(), 11);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 141: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    // server gracefully closes connection

    let mut req = "ws://127.0.0.1:6147".into_client_request().unwrap();
    req.headers_mut()
        .insert("x-port", HeaderValue::from_static("9283"));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 145: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .insert("x-port", HeaderValue::from_static("9283"));

    let (mut ws_stream, _) = tokio_tungstenite::connect_async(req).await.unwrap();
    // gracefully close connection
    ws_stream.send("test".into()).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 147: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let (mut ws_stream, _) = tokio_tungstenite::connect_async(req).await.unwrap();
    // gracefully close connection
    ws_stream.send("test".into()).await.unwrap();
    ws_stream.next().await.unwrap().unwrap();
    ws_stream.send("graceful".into()).await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 148: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    // gracefully close connection
    ws_stream.send("test".into()).await.unwrap();
    ws_stream.next().await.unwrap().unwrap();
    ws_stream.send("graceful".into()).await.unwrap();
    let msg = ws_stream.next().await.unwrap().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 148: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    // gracefully close connection
    ws_stream.send("test".into()).await.unwrap();
    ws_stream.next().await.unwrap().unwrap();
    ws_stream.send("graceful".into()).await.unwrap();
    let msg = ws_stream.next().await.unwrap().unwrap();
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
    ws_stream.send("test".into()).await.unwrap();
    ws_stream.next().await.unwrap().unwrap();
    ws_stream.send("graceful".into()).await.unwrap();
    let msg = ws_stream.next().await.unwrap().unwrap();
    // assert graceful close
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 150: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    ws_stream.next().await.unwrap().unwrap();
    ws_stream.send("graceful".into()).await.unwrap();
    let msg = ws_stream.next().await.unwrap().unwrap();
    // assert graceful close
    assert!(matches!(msg, Message::Close(None)));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 150: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    ws_stream.next().await.unwrap().unwrap();
    ws_stream.send("graceful".into()).await.unwrap();
    let msg = ws_stream.next().await.unwrap().unwrap();
    // assert graceful close
    assert!(matches!(msg, Message::Close(None)));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 158: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    // server abruptly closes connection

    let mut req = "ws://127.0.0.1:6147".into_client_request().unwrap();
    req.headers_mut()
        .insert("x-port", HeaderValue::from_static("9283"));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 162: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .insert("x-port", HeaderValue::from_static("9283"));

    let (mut ws_stream, _) = tokio_tungstenite::connect_async(req).await.unwrap();
    // abrupt close connection
    ws_stream.send("close".into()).await.unwrap();
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    let (mut ws_stream, _) = tokio_tungstenite::connect_async(req).await.unwrap();
    // abrupt close connection
    ws_stream.send("close".into()).await.unwrap();
    // test will hang here if downstream doesn't close when upstream does
    assert!(ws_stream.next().await.unwrap().is_err());
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
    // client gracefully closes connection

    let mut req = "ws://127.0.0.1:6147".into_client_request().unwrap();
    req.headers_mut()
        .insert("x-port", HeaderValue::from_static("9283"));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 174: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .insert("x-port", HeaderValue::from_static("9283"));

    let (mut ws_stream, _) = tokio_tungstenite::connect_async(req).await.unwrap();
    ws_stream.send("test".into()).await.unwrap();
    // sender initiates close
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 175: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

    let (mut ws_stream, _) = tokio_tungstenite::connect_async(req).await.unwrap();
    ws_stream.send("test".into()).await.unwrap();
    // sender initiates close
    ws_stream.close(None).await.unwrap();
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
    ws_stream.send("test".into()).await.unwrap();
    // sender initiates close
    ws_stream.close(None).await.unwrap();
    let msg = ws_stream.next().await.unwrap().unwrap();
    // assert echo
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 178: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    // sender initiates close
    ws_stream.close(None).await.unwrap();
    let msg = ws_stream.next().await.unwrap().unwrap();
    // assert echo
    assert_eq!("test", msg.into_text().unwrap());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 178: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    // sender initiates close
    ws_stream.close(None).await.unwrap();
    let msg = ws_stream.next().await.unwrap().unwrap();
    // assert echo
    assert_eq!("test", msg.into_text().unwrap());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 181: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    // assert echo
    assert_eq!("test", msg.into_text().unwrap());
    let msg = ws_stream.next().await.unwrap().unwrap();
    // assert graceful close
    assert!(matches!(msg, Message::Close(None)));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 181: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
    // assert echo
    assert_eq!("test", msg.into_text().unwrap());
    let msg = ws_stream.next().await.unwrap().unwrap();
    // assert graceful close
    assert!(matches!(msg, Message::Close(None)));
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

    let client = hyper::Client::new();
    let uri: hyper::Uri = "http://127.0.0.1:6147/download/".parse().unwrap();
    let req = hyper::Request::builder()
        .uri(uri)
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
        .header("x-write-timeout", "1")
        .body(hyper::Body::empty())
        .unwrap();
    let mut res = client.request(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 200: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .body(hyper::Body::empty())
        .unwrap();
    let mut res = client.request(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 220: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

    let client = hyper::Client::new();
    let uri: hyper::Uri = "http://127.0.0.1:6147/download/".parse().unwrap();
    let req = hyper::Request::builder()
        .uri(uri)
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .header("x-min-rate", "10000")
        .body(hyper::Body::empty())
        .unwrap();
    let mut res = client.request(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .body(hyper::Body::empty())
        .unwrap();
    let mut res = client.request(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 251: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let url = "http://127.0.0.1:6148/unique/test_basic_caching/now";

        let res = reqwest::get(url).await.unwrap();
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


### Line 254: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 254: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(res.text().await.unwrap(), "hello world");
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.text().await.unwrap(), "hello world");

        let res = reqwest::get(url).await.unwrap();
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


### Line 261: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert_eq!(res.text().await.unwrap(), "hello world");
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert_eq!(res.text().await.unwrap(), "hello world");
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
        sleep(Duration::from_millis(1100)).await; // ttl is 1

        let res = reqwest::get(url).await.unwrap();
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


### Line 272: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_expired_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "expired");
        assert_eq!(res.text().await.unwrap(), "hello world");
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
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_expired_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "expired");
        assert_eq!(res.text().await.unwrap(), "hello world");
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
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let res = reqwest::get("http://127.0.0.1:6148/unique/test_purge/test2")
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


### Line 292: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let res = reqwest::get("http://127.0.0.1:6148/unique/test_purge/test2")
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


### Line 307: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await.unwrap(), "");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 300: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let res = reqwest::Client::builder()
            .build()
            .unwrap()
            .request(
                reqwest::Method::from_bytes(b"PURGE").unwrap(),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 302: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .unwrap()
            .request(
                reqwest::Method::from_bytes(b"PURGE").unwrap(),
                "http://127.0.0.1:6148/unique/test_purge/test2",
            )
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
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        assert_eq!(res.text().await.unwrap(), "");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 313: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let res = reqwest::Client::builder()
            .build()
            .unwrap()
            .request(
                reqwest::Method::from_bytes(b"PURGE").unwrap(),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 315: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .unwrap()
            .request(
                reqwest::Method::from_bytes(b"PURGE").unwrap(),
                "http://127.0.0.1:6148/unique/test_purge/test2",
            )
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
        let res = reqwest::get("http://127.0.0.1:6148/unique/test_purge/test2")
            .await
            .unwrap();
        let headers = res.headers();
        assert_eq!(res.status(), StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 344: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        // 200 because last-modified not returned from upstream
        assert_eq!(res.status(), StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 358: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        // we have not implemented downstream range yet, it should be 206 once we have it
        assert_eq!(res.status(), StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 372: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        // disable reqwest gzip support to check compression headers and body
        // otherwise reqwest will decompress and strip the headers
        let client = reqwest::ClientBuilder::new().gzip(false).build().unwrap();
        let res = client
            .get("http://127.0.0.1:6148/unique/test_cache_downstream_compression/no_compression")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 379: `.unwrap()`

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


### Line 384: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(headers["Content-Encoding"], "gzip");
        assert_eq!(headers["x-cache-status"], "miss");
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


### Line 388: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

        // should also apply on hit
        let client = reqwest::ClientBuilder::new().gzip(false).build().unwrap();
        let res = client
            .get("http://127.0.0.1:6148/unique/test_cache_downstream_compression/no_compression")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 395: `.unwrap()`

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


### Line 400: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(headers["Content-Encoding"], "gzip");
        assert_eq!(headers["x-cache-status"], "hit");
        let body = res.bytes().await.unwrap();
        assert!(body.len() < 32);
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 410: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        // disable reqwest gzip support to check compression headers and body
        // otherwise reqwest will decompress and strip the headers
        let client = reqwest::ClientBuilder::new().gzip(false).build().unwrap();
        let res = client
            .get("http://127.0.0.1:6148/unique/test_cache_downstream_decompression/gzip/index.html")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 417: `.unwrap()`

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


### Line 425: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert!(headers.get("Content-Encoding").is_none());
        assert_eq!(headers["x-cache-status"], "miss");
        let body = res.bytes().await.unwrap();
        assert_eq!(body, "Hello World!\n");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 429: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

        // should also apply on hit
        let client = reqwest::ClientBuilder::new().gzip(false).build().unwrap();
        let res = client
            .get("http://127.0.0.1:6148/unique/test_cache_downstream_decompression/gzip/index.html")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 436: `.unwrap()`

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


### Line 442: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert!(headers.get("Content-Encoding").is_none());
        assert_eq!(headers["x-cache-status"], "hit");
        let body = res.bytes().await.unwrap();
        assert_eq!(body, "Hello World!\n");

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

        // should also apply on revalidated
        let client = reqwest::ClientBuilder::new().gzip(false).build().unwrap();
        let res = client
            .get("http://127.0.0.1:6148/unique/test_cache_downstream_decompression/gzip/index.html")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 455: `.unwrap()`

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


### Line 461: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert!(headers.get("Content-Encoding").is_none());
        assert_eq!(headers["x-cache-status"], "revalidated");
        let body = res.bytes().await.unwrap();
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


### Line 477: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), 200);
        // sleep just a little to make sure the req above gets the cache lock
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 493: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), 200);
        // sleep just a little to make sure the req above gets the cache lock
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 508: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let url = "http://127.0.0.1:6148/unique/test_upstream_revalidation/revalidate_now";

        let res = reqwest::get(url).await.unwrap();
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


### Line 511: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(headers["x-upstream-status"], "200");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 511: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(headers["x-upstream-status"], "200");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 516: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.text().await.unwrap(), "hello world");

        let res = reqwest::get(url).await.unwrap();
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


### Line 519: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert!(headers.get("x-upstream-status").is_none());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 519: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert!(headers.get("x-upstream-status").is_none());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 528: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        sleep(Duration::from_millis(1100)).await; // ttl is 1

        let res = reqwest::get(url).await.unwrap();
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


### Line 531: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_expired_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "revalidated");
        assert_eq!(headers["x-upstream-status"], "304");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 531: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_expired_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "revalidated");
        assert_eq!(headers["x-upstream-status"], "304");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 550: `.unwrap()`

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


### Line 564: `.unwrap()`

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


### Line 581: `.unwrap()`

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


### Line 598: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let url = "http://127.0.0.1:6148/unique/test_froce_miss/revalidate_now";

        let res = reqwest::get(url).await.unwrap();
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


### Line 601: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(headers["x-upstream-status"], "200");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 601: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(headers["x-upstream-status"], "200");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 606: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.text().await.unwrap(), "hello world");

        let res = reqwest::get(url).await.unwrap();
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


### Line 609: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert!(headers.get("x-upstream-status").is_none());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 609: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert!(headers.get("x-upstream-status").is_none());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 621: `.unwrap()`

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


### Line 635: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let url = "http://127.0.0.1:6148/unique/test_froce_miss_stale/revalidate_now";

        let res = reqwest::get(url).await.unwrap();
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


### Line 638: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(headers["x-upstream-status"], "200");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 638: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(headers["x-upstream-status"], "200");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 643: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.text().await.unwrap(), "hello world");

        let res = reqwest::get(url).await.unwrap();
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


### Line 646: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert!(headers.get("x-upstream-status").is_none());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 646: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert!(headers.get("x-upstream-status").is_none());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 661: `.unwrap()`

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


### Line 667: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(headers["x-upstream-status"], "200");
        let cache_miss_epoch2 = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert!(cache_miss_epoch != cache_miss_epoch2);
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 667: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(headers["x-upstream-status"], "200");
        let cache_miss_epoch2 = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert!(cache_miss_epoch != cache_miss_epoch2);
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 685: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 688: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
        let cache_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(res.text().await.unwrap(), ""); // 304 no body
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 688: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
        let cache_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(res.text().await.unwrap(), ""); // 304 no body
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 698: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 701: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert_eq!(res.text().await.unwrap(), ""); // 304 no body
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 701: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert_eq!(res.text().await.unwrap(), ""); // 304 no body
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 713: `.unwrap()`

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


### Line 716: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 716: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 730: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
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
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
        let cache_expired_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "revalidated");
        assert_eq!(res.text().await.unwrap(), ""); // 304 no body
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
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
        let cache_expired_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "revalidated");
        assert_eq!(res.text().await.unwrap(), ""); // 304 no body
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 754: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 757: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
        let cache_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(res.text().await.unwrap(), ""); // 304 no body
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 757: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
        let cache_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(res.text().await.unwrap(), ""); // 304 no body
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 767: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 770: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert_eq!(res.text().await.unwrap(), ""); // 304 no body
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 770: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert_eq!(res.text().await.unwrap(), ""); // 304 no body
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 782: `.unwrap()`

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


### Line 785: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 785: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 799: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 802: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
        let cache_expired_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "revalidated");
        assert_eq!(res.text().await.unwrap(), ""); // 304 no body
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 802: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
        let cache_expired_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "revalidated");
        assert_eq!(res.text().await.unwrap(), ""); // 304 no body
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 817: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

        // MISS + HEAD
        let res = client.head(url).send().await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 821: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(res.text().await.unwrap(), ""); // HEAD no body
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 821: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(res.text().await.unwrap(), ""); // HEAD no body
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 826: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

        // HIT + HEAD
        let res = client.head(url).send().await.unwrap();
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


### Line 829: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert_eq!(res.text().await.unwrap(), ""); // HEAD no body
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 829: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert_eq!(res.text().await.unwrap(), ""); // HEAD no body
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 838: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

        // revalidated + HEAD
        let res = client.head(url).send().await.unwrap();
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


### Line 841: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_expired_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "revalidated");
        assert_eq!(res.text().await.unwrap(), ""); // HEAD no body
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 841: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_expired_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "revalidated");
        assert_eq!(res.text().await.unwrap(), ""); // HEAD no body
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 863: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(res.text().await.unwrap(), "");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 855: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let res = reqwest::Client::builder()
            .build()
            .unwrap()
            .request(
                reqwest::Method::from_bytes(b"PURGE").unwrap(),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 857: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .unwrap()
            .request(
                reqwest::Method::from_bytes(b"PURGE").unwrap(),
                "http://127.0.0.1:6148/",
            )
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 879: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
                .await
                .unwrap();
            if let Ok((mut stream, _addr)) = listener.accept().await {
                stream.write_all(b"HTTP/1.1 103 Early Hints\r\nLink: <https://foo.bar>; rel=preconnect\r\n\r\n").await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 881: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
                .unwrap();
            if let Ok((mut stream, _addr)) = listener.accept().await {
                stream.write_all(b"HTTP/1.1 103 Early Hints\r\nLink: <https://foo.bar>; rel=preconnect\r\n\r\n").await.unwrap();
                // wait a bit so that the client can read
                sleep(Duration::from_millis(100)).await;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 884: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
                // wait a bit so that the client can read
                sleep(Duration::from_millis(100)).await;
                stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Length: 5\r\nCache-Control: {}\r\n\r\nhello", cc_header).as_bytes()).await.unwrap();
                sleep(Duration::from_millis(100)).await;
            }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 905: `.unwrap()`

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


### Line 916: `.unwrap()`

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


### Line 936: `.unwrap()`

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


### Line 955: `.unwrap()`

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


### Line 973: `.unwrap()`

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


### Line 976: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cc = headers.get("Cache-Control").unwrap();
        assert_eq!(cc, "private, max-age=0");
        assert_eq!(headers["x-cache-status"], "no-cache");
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


### Line 999: `.unwrap()`

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


### Line 1017: `.unwrap()`

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


### Line 1020: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cc = headers.get("Cache-Control").unwrap();
        assert_eq!(cc, "private, max-age=0");
        assert_eq!(headers["x-cache-status"], "no-cache");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1033: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1044: `.unwrap()`

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


### Line 1056: `.unwrap()`

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


### Line 1075: `.unwrap()`

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


### Line 1078: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cc = headers.get("Cache-Control").unwrap();
        assert_eq!(cc, "public, max-age=0");
        assert_eq!(headers["x-cache-status"], "miss");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1091: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        // should see the response body because we didn't send conditional headers
        assert_eq!(res.status(), StatusCode::OK);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1105: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1123: `.unwrap()`

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


### Line 1135: `.unwrap()`

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


### Line 1147: `.unwrap()`

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


### Line 1166: `.unwrap()`

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


### Line 1179: `.unwrap()`

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


### Line 1192: `.unwrap()`

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


### Line 1205: `.unwrap()`

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


### Line 1218: `.unwrap()`

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


### Line 1231: `.unwrap()`

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


### Line 1245: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        // no lock, parallel fetches to a slow origin are all misses
        tokio::spawn(async move {
            let res = reqwest::Client::new().get(url).send().await.unwrap();
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


### Line 1253: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

        tokio::spawn(async move {
            let res = reqwest::Client::new().get(url).send().await.unwrap();
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


### Line 1268: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        })
        .await
        .unwrap(); // wait for at least one of them to finish

        let res = reqwest::Client::new().get(url).send().await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1261: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

        tokio::spawn(async move {
            let res = reqwest::Client::new().get(url).send().await.unwrap();
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


### Line 1270: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        .unwrap(); // wait for at least one of them to finish

        let res = reqwest::Client::new().get(url).send().await.unwrap();
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


### Line 1284: `.unwrap()`

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


### Line 1298: `.unwrap()`

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


### Line 1306: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
                .unwrap()
                .parse()
                .unwrap();
            assert!(lock_time_ms > 900 && lock_time_ms < 1000);
            assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1304: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            let lock_time_ms: u32 = headers["x-cache-lock-time-ms"]
                .to_str()
                .unwrap()
                .parse()
                .unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1316: `.unwrap()`

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


### Line 1324: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
                .unwrap()
                .parse()
                .unwrap();
            assert!(lock_time_ms > 900 && lock_time_ms < 1000);
            assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1322: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            let lock_time_ms: u32 = headers["x-cache-lock-time-ms"]
                .to_str()
                .unwrap()
                .parse()
                .unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1329: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        });

        task1.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1330: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

        task1.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1331: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        task1.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1345: `.unwrap()`

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


### Line 1360: `.unwrap()`

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


### Line 1375: `.unwrap()`

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


### Line 1388: `.unwrap()`

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


### Line 1395: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        });

        task1.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1396: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

        task1.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1397: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        task1.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1413: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
                .send()
                .await
                .unwrap();
            assert_eq!(res.status(), 502); // error happened
        });
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1425: `.unwrap()`

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


### Line 1438: `.unwrap()`

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


### Line 1446: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        });

        task1.await.unwrap();
        let status2 = task2.await.unwrap();
        let status3 = task3.await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1447: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

        task1.await.unwrap();
        let status2 = task2.await.unwrap();
        let status3 = task3.await.unwrap();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1448: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        task1.await.unwrap();
        let status2 = task2.await.unwrap();
        let status3 = task3.await.unwrap();

        let mut count_miss = 0;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1472: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
                .send()
                .await
                .unwrap();
            assert_eq!(res.status(), 200);
            let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1487: `.unwrap()`

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


### Line 1499: `.unwrap()`

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


### Line 1506: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        });

        task1.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1507: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

        task1.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1508: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        task1.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1523: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
                .send()
                .await
                .unwrap();
            assert_eq!(res.status(), 200);
            let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1539: `.unwrap()`

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


### Line 1557: `.unwrap()`

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


### Line 1566: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        });

        task1.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1567: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

        task1.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1568: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        task1.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();

        let res = reqwest::Client::new()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1575: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), 200);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1600: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
                    .send()
                    .await
                    .unwrap();
                assert_eq!(res.status(), 502);
                let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1610: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let mut waited_count = 0;
        for handle in handles {
            let lock_time_ms = handle.await.unwrap();
            if let Some(lock_time_ms) = lock_time_ms {
                // should not have waited more than 2s for each response
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1632: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), 200);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1646: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), 200);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1664: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), 200);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1679: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), 200);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1697: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), 200);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1711: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), 200);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1724: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

        // cache one
        let res = reqwest::Client::new().get(url).send().await.unwrap();
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


### Line 1738: `.unwrap()`

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


### Line 1752: `.unwrap()`

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


### Line 1764: `.unwrap()`

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


### Line 1771: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        });

        task1.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1772: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

        task1.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1773: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        task1.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1787: `.unwrap()`

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


### Line 1801: `.unwrap()`

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


### Line 1815: `.unwrap()`

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


### Line 1832: `.unwrap()`

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


### Line 1847: `.unwrap()`

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


### Line 1855: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
                .unwrap()
                .parse()
                .unwrap();
            // the entire body should need 2 extra seconds, here the test shows that
            // only the header is under cache lock and the body should be streamed
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1853: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            let lock_time_ms: u32 = headers["x-cache-lock-time-ms"]
                .to_str()
                .unwrap()
                .parse()
                .unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1867: `.unwrap()`

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


### Line 1875: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
                .unwrap()
                .parse()
                .unwrap();
            // the entire body should need 2 extra seconds, here the test shows that
            // only the header is under cache lock and the body should be streamed
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1873: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            let lock_time_ms: u32 = headers["x-cache-lock-time-ms"]
                .to_str()
                .unwrap()
                .parse()
                .unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1882: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        });

        task1.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1883: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

        task1.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1884: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        task1.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1898: `.unwrap()`

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


### Line 1913: `.unwrap()`

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


### Line 1920: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        });

        task1.await.unwrap();
        task2.await.unwrap();
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1921: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

        task1.await.unwrap();
        task2.await.unwrap();
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1934: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1937: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
        let headers = res.headers();
        let cache_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(res.text().await.unwrap(), "he");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1937: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
        let headers = res.headers();
        let cache_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(res.text().await.unwrap(), "he");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1942: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust

        // full body is cached
        let res = reqwest::get(url).await.unwrap();
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


### Line 1945: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1945: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1956: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1967: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::RANGE_NOT_SATISFIABLE);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1978: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1991: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2010: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2014: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
        let headers = res.headers().clone();
        let content_type = headers.get("content-type").unwrap().to_str().unwrap();
        // Grab boundary to verify full response
        let boundary = content_type
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2014: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
        let headers = res.headers().clone();
        let content_type = headers.get("content-type").unwrap().to_str().unwrap();
        // Grab boundary to verify full response
        let boundary = content_type
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2026: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(headers["x-cache-status"], "miss");

        let body = res.text().await.unwrap();

        let expected_body = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2051: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2055: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
        let headers = res.headers().clone();
        let content_type = headers.get("content-type").unwrap().to_str().unwrap();
        let boundary = content_type
            .split("boundary=")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2055: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
        let headers = res.headers().clone();
        let content_type = headers.get("content-type").unwrap().to_str().unwrap();
        let boundary = content_type
            .split("boundary=")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2066: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(headers["x-cache-status"], "hit");

        let body = res.text().await.unwrap();

        let expected_body = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2091: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2095: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
        let headers = res.headers().clone();
        let content_type = headers.get("content-type").unwrap().to_str().unwrap();
        let boundary = content_type
            .split("boundary=")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2095: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
        let headers = res.headers().clone();
        let content_type = headers.get("content-type").unwrap().to_str().unwrap();
        let boundary = content_type
            .split("boundary=")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2106: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(headers["x-cache-status"], "hit");

        let body = res.text().await.unwrap();

        let expected_body = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2136: `.unwrap()`

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


### Line 2143: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(headers["x-cache-status"], "hit");

        let body = res.text().await.unwrap();
        assert_eq!(body, "hello world");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2153: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2159: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(headers["x-cache-status"], "hit");

        let body = res.text().await.unwrap();
        assert_eq!(body, "hel");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2169: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::RANGE_NOT_SATISFIABLE);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2182: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2188: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(headers["x-cache-status"], "hit");

        let body = res.text().await.unwrap();
        assert_eq!(body, "");
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2204: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2210: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(headers["x-cache-status"], "miss");

        let body = res.text().await.unwrap();
        assert_eq!(body, "hello");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2220: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2224: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
        let headers = res.headers().clone();
        let content_type = headers.get("content-type").unwrap().to_str().unwrap();
        let boundary = content_type
            .split("boundary=")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2224: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
        let headers = res.headers().clone();
        let content_type = headers.get("content-type").unwrap().to_str().unwrap();
        let boundary = content_type
            .split("boundary=")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2235: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(headers["x-cache-status"], "hit");

        let body = res.text().await.unwrap();

        let expected_body = format!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2259: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2262: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
        let headers = res.headers();
        let content_type = headers.get("content-type").unwrap().to_str().unwrap();
        assert!(!content_type.contains("multipart/byteranges; boundary="));
        assert_eq!(headers["x-cache-status"], "hit");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2262: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
        let headers = res.headers();
        let content_type = headers.get("content-type").unwrap().to_str().unwrap();
        assert!(!content_type.contains("multipart/byteranges; boundary="));
        assert_eq!(headers["x-cache-status"], "hit");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2280: `.unwrap()`

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


### Line 2294: `.unwrap()`

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


### Line 2302: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .unwrap()
            .parse()
            .unwrap();
        // the entire body should need 2 extra seconds, here the test shows that
        // only the header is under cache lock and the body should be streamed
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2300: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let lock_time_ms: u32 = headers["x-cache-lock-time-ms"]
            .to_str()
            .unwrap()
            .parse()
            .unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2319: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .map(|(name, value)| {
                (
                    HeaderName::from_str(name).unwrap(),
                    HeaderValue::from_str(value).unwrap(),
                )
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2320: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
                (
                    HeaderName::from_str(name).unwrap(),
                    HeaderValue::from_str(value).unwrap(),
                )
            })
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2334: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        }

        req_builder.send().await.unwrap()
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2358: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_a_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2358: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_a_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2365: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2365: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2374: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_b_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2374: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_b_miss_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2381: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2381: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let cache_hit_epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "hit");
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2399: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2399: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        assert_eq!(res.status(), StatusCode::OK);
        let headers = res.headers();
        let epoch = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(headers["x-cache-status"], "miss");
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2444: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let headers = res.headers();
        assert_eq!(headers["x-cache-status"], "miss");
        let epoch1 = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(res.text().await.unwrap(), "hello world");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2444: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let headers = res.headers();
        assert_eq!(headers["x-cache-status"], "miss");
        let epoch1 = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(res.text().await.unwrap(), "hello world");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2498: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let headers = res.headers();
        assert_eq!(headers["x-cache-status"], "miss");
        let epoch2 = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_ne!(epoch1, epoch2);
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2498: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let headers = res.headers();
        assert_eq!(headers["x-cache-status"], "miss");
        let epoch2 = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_ne!(epoch1, epoch2);
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2560: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let headers = res.headers();
        assert_eq!(headers["x-cache-status"], "miss");
        let epoch1 = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(res.text().await.unwrap(), "hello world");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2560: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let headers = res.headers();
        assert_eq!(headers["x-cache-status"], "miss");
        let epoch1 = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_eq!(res.text().await.unwrap(), "hello world");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2597: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let headers = res.headers();
        assert_eq!(headers["x-cache-status"], "miss");
        let epoch2 = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_ne!(epoch1, epoch2);
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2597: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let headers = res.headers();
        assert_eq!(headers["x-cache-status"], "miss");
        let epoch2 = headers["x-epoch"].to_str().unwrap().parse::<f64>().unwrap();
        assert_ne!(epoch1, epoch2);
        assert_eq!(res.text().await.unwrap(), "hello world");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2630: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await.unwrap(), "");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2626: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        let res = reqwest::Client::builder()
            .build()
            .unwrap()
            .request(reqwest::Method::from_bytes(b"PURGE").unwrap(), url)
            .send()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2627: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .build()
            .unwrap()
            .request(reqwest::Method::from_bytes(b"PURGE").unwrap(), url)
            .send()
            .await
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2659: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            req = req.header("Range", format!("bytes={}-{}", r.0, r.1));
        }
        req.send().await.unwrap()
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2815: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        // h2 protocol error with content length mismatch

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2826: `.unwrap()`

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


### Line 2848: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2863: `.unwrap()`

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


### Line 2882: `.unwrap()`

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


### Line 2889: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
        // closed connection does not impact next cache fill

        let res = reqwest::Client::new().get(test_url).send().await.unwrap();
        assert_eq!(res.status(), 200);
        let headers = res.headers();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2907: `.unwrap()`

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


### Line 2914: `.unwrap()`

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


### Line 2924: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        let res = client.get(test_url).send().await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2926: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
            .unwrap();

        let res = client.get(test_url).send().await.unwrap();
        assert_eq!(res.status(), 200);
        let headers = res.headers();
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