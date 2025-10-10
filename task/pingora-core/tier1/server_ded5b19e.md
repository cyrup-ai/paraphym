# `forks/pingora/pingora-core/src/protocols/http/v1/server.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-core
- **File Hash**: ded5b19e  
- **Timestamp**: 2025-10-10T02:16:01.204490+00:00  
- **Lines of Code**: 1622

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 1622 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 91
  - TODO
  - 

```rust
    /// any other operations.
    pub fn new(underlying_stream: Stream) -> Self {
        // TODO: maybe we should put digest in the connection itself
        let digest = Box::new(Digest {
            ssl_digest: underlying_stream.get_ssl_digest(),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 156
  - FIXME
  - 

```rust
                    },
                    KeepaliveStatus::Infinite => {
                        // FIXME: this should only apply to reads between requests
                        read_event.await
                    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 487
  - TODO
  - 

```rust
            header.insert_header(header::DATE, date::get_cached_date())?;

            // TODO: make these lazy static
            let connection_value = if self.will_keepalive() {
                "keep-alive"
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 594
  - TODO
  - 

```rust
    /// Return whether the session will be keepalived for connection reuse.
    pub fn will_keepalive(&self) -> bool {
        // TODO: check self.body_writer. If it is http1.0 type then keepalive
        // cannot be used because the connection close is the signal of end body
        !matches!(self.keepalive_timeout, KeepaliveStatus::Off)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 601
  - TODO
  - 

```rust
    // `Keep-Alive: timeout=5, max=1000` => 5, 1000
    fn get_keepalive_values(&self) -> (Option<u64>, Option<usize>) {
        // TODO: implement this parsing
        (None, None)
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 640
  - TODO
  - 

```rust
            if keepalive {
                let (timeout, _max_use) = self.get_keepalive_values();
                // TODO: respect max_use
                match timeout {
                    Some(d) => self.set_keepalive(Some(d)),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 700
  - TODO
  - 

```rust
    /// to be written, e.g., writing more bytes than what the `Content-Length` header suggests
    pub async fn write_body(&mut self, buf: &[u8]) -> Result<Option<usize>> {
        // TODO: check if the response header is written
        match self.write_timeout(buf.len()) {
            Some(t) => match timeout(t, self.do_write_body(buf)).await {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 847
  - TODO
  - 

```rust

    fn get_body(&self, buf_ref: &BufRef) -> &[u8] {
        // TODO: these get_*() could panic. handle them better
        self.body_reader.get_body(buf_ref)
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1071
  - TODO
  - 

```rust
    }

    // TODO: use vectored write to avoid copying
    pub async fn response_duplex_vec(&mut self, mut tasks: Vec<HttpTask>) -> Result<bool> {
        let n_tasks = tasks.len();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1146
  - TODO
  - 

```rust

        // rebuild the entire request buf in a new buffer
        // TODO: this might be able to be done in place

        // need to be slightly bigger than the current buf;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1223
  - TODO
  - 

```rust

    // headers
    // TODO: style: make sure Server and Date headers are the first two
    resp.header_to_h1_wire(buf);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2005
  - FIXME
  - 

```rust
        }
        assert!(result.unwrap().is_complete());
        // FIXME: the order is not guaranteed
        assert_eq!(b"Foo", headers[0].name.as_bytes());
        assert_eq!(b"Bar", headers[0].value);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1247
  - stubby variable name
  - mock_io

```rust
        init_log();
        let input = b"GET / HTTP/1.1\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_request().await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1248
  - stubby variable name
  - mock_io

```rust
        let input = b"GET / HTTP/1.1\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_request().await;
        assert_eq!(input.len(), res.unwrap().unwrap());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1259
  - stubby variable name
  - mock_io

```rust
        init_log();
        let input = b"GET /\x01\xF0\x90\x80 HTTP/1.1\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_request().await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1260
  - stubby variable name
  - mock_io

```rust
        let input = b"GET /\x01\xF0\x90\x80 HTTP/1.1\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_request().await;
        assert_eq!(input.len(), res.unwrap().unwrap());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1272
  - stubby variable name
  - mock_io

```rust
        let input1 = b"GET / HTTP/1.1\r\n";
        let input2 = b"Host: pingora.org\r\n\r\n";
        let mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_request().await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1273
  - stubby variable name
  - mock_io

```rust
        let input2 = b"Host: pingora.org\r\n\r\n";
        let mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_request().await;
        assert_eq!(input1.len() + input2.len(), res.unwrap().unwrap());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1294
  - stubby variable name
  - mock_io

```rust
        let input2 = b"Host: pingora.org\r\nContent-Length: 3\r\n\r\n";
        let input3 = b"abc";
        let mock_io = Builder::new()
            .read(&input1[..])
            .read(&input2[..])
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1299
  - stubby variable name
  - mock_io

```rust
            .read(&input3[..])
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1314
  - stubby variable name
  - mock_io

```rust
        let input2 = b"Host: pingora.org\r\nContent-Length: 3\r\n\r\n";
        let input3 = b"abc";
        let mock_io = Builder::new()
            .read(&input1[..])
            .read(&input2[..])
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1320
  - stubby variable name
  - mock_io

```rust
            .read(&input3[..])
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_timeout = Some(Duration::from_secs(1));
        http_stream.read_request().await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1333
  - stubby variable name
  - mock_io

```rust
        let input1 = b"GET / HTTP/1.1\r\n";
        let input2 = b"Host: pingora.org\r\nContent-Length: 3\r\n\r\nabc";
        let mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1334
  - stubby variable name
  - mock_io

```rust
        let input2 = b"Host: pingora.org\r\nContent-Length: 3\r\n\r\nabc";
        let mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1349
  - stubby variable name
  - mock_io

```rust
        let input3 = b"a";
        let input4 = b""; // simulating close
        let mock_io = Builder::new()
            .read(&input1[..])
            .read(&input2[..])
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1355
  - stubby variable name
  - mock_io

```rust
            .read(&input4[..])
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1374
  - stubby variable name
  - mock_io

```rust
        let input3 = b"b";
        let input4 = b""; // simulating close
        let mock_io = Builder::new()
            .read(&input1[..])
            .read(&input2[..])
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1380
  - stubby variable name
  - mock_io

```rust
            .read(&input4[..])
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1399
  - stubby variable name
  - mock_io

```rust
        let input1 = b"GET / HTTP/1.1\r\n";
        let input2 = b"Host: pingora.org\r\n\r\n";
        let mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1400
  - stubby variable name
  - mock_io

```rust
        let input2 = b"Host: pingora.org\r\n\r\n";
        let mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1414
  - stubby variable name
  - mock_io

```rust
        let input2 = b"Host: pingora.org\r\nTransfer-Encoding: chunked\r\n\r\n";
        let input3 = b"0\r\n";
        let mock_io = Builder::new()
            .read(&input1[..])
            .read(&input2[..])
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1419
  - stubby variable name
  - mock_io

```rust
            .read(&input3[..])
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        assert!(http_stream.is_chunked_encoding());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1434
  - stubby variable name
  - mock_io

```rust
        let input2 = b"Host: pingora.org\r\nTransfer-Encoding: chunked\r\n\r\n1\r\na\r\n";
        let input3 = b"0\r\n\r\n";
        let mock_io = Builder::new()
            .read(&input1[..])
            .read(&input2[..])
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1439
  - stubby variable name
  - mock_io

```rust
            .read(&input3[..])
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        assert!(http_stream.is_chunked_encoding());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1480
  - stubby variable name
  - mock_io

```rust

        input2 += "\r\n3e\r\na\r\n";
        let mock_io = Builder::new()
            .read(&input1[..])
            .read(input2.as_bytes())
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1484
  - stubby variable name
  - mock_io

```rust
            .read(input2.as_bytes())
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let _ = http_stream.read_request().await.unwrap();

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1508
  - stubby variable name
  - mock_io

```rust
        let input1 = b"GET / HTP/1.1\r\n";
        let input2 = b"Host: pingora.org\r\n\r\n";
        let mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_request().await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1509
  - stubby variable name
  - mock_io

```rust
        let input2 = b"Host: pingora.org\r\n\r\n";
        let mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_request().await;
        assert_eq!(&InvalidHTTPHeader, res.unwrap_err().etype());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1516
  - stubby variable name
  - mock_io

```rust
    async fn build_req(upgrade: &str, conn: &str) -> HttpSession {
        let input = format!("GET / HTTP/1.1\r\nHost: pingora.org\r\nUpgrade: {upgrade}\r\nConnection: {conn}\r\n\r\n");
        let mock_io = Builder::new().read(input.as_bytes()).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1517
  - stubby variable name
  - mock_io

```rust
        let input = format!("GET / HTTP/1.1\r\nHost: pingora.org\r\nUpgrade: {upgrade}\r\nConnection: {conn}\r\n\r\n");
        let mock_io = Builder::new().read(input.as_bytes()).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        http_stream
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1526
  - stubby variable name
  - mock_io

```rust
        // http 1.0
        let input = b"GET / HTTP/1.0\r\nHost: pingora.org\r\nUpgrade: websocket\r\nConnection: upgrade\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1527
  - stubby variable name
  - mock_io

```rust
        let input = b"GET / HTTP/1.0\r\nHost: pingora.org\r\nUpgrade: websocket\r\nConnection: upgrade\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        assert!(!http_stream.is_upgrade_req());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1533
  - stubby variable name
  - mock_io

```rust
        // different method
        let input = b"POST / HTTP/1.1\r\nHost: pingora.org\r\nUpgrade: websocket\r\nConnection: upgrade\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1534
  - stubby variable name
  - mock_io

```rust
        let input = b"POST / HTTP/1.1\r\nHost: pingora.org\r\nUpgrade: websocket\r\nConnection: upgrade\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        assert!(http_stream.is_upgrade_req());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1540
  - stubby variable name
  - mock_io

```rust
        // missing upgrade header
        let input = b"GET / HTTP/1.1\r\nHost: pingora.org\r\nConnection: upgrade\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1541
  - stubby variable name
  - mock_io

```rust
        let input = b"GET / HTTP/1.1\r\nHost: pingora.org\r\nConnection: upgrade\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        assert!(!http_stream.is_upgrade_req());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1547
  - stubby variable name
  - mock_io

```rust
        // no connection header
        let input = b"GET / HTTP/1.1\r\nHost: pingora.org\r\nUpgrade: WebSocket\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1548
  - stubby variable name
  - mock_io

```rust
        let input = b"GET / HTTP/1.1\r\nHost: pingora.org\r\nUpgrade: WebSocket\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        assert!(http_stream.is_upgrade_req());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1561
  - stubby variable name
  - mock_io

```rust
    async fn read_upgrade_req_with_1xx_response() {
        let input = b"GET / HTTP/1.1\r\nHost: pingora.org\r\nUpgrade: websocket\r\nConnection: upgrade\r\n\r\n";
        let mock_io = Builder::new()
            .read(&input[..])
            .write(b"HTTP/1.1 100 Continue\r\n\r\n")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1565
  - stubby variable name
  - mock_io

```rust
            .write(b"HTTP/1.1 100 Continue\r\n\r\n")
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        assert!(http_stream.is_upgrade_req());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1582
  - stubby variable name
  - mock_io

```rust
        // close
        let input = b"GET / HTTP/1.1\r\nHost: pingora.org\r\nConnection: close\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1583
  - stubby variable name
  - mock_io

```rust
        let input = b"GET / HTTP/1.1\r\nHost: pingora.org\r\nConnection: close\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        // verify close
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1593
  - stubby variable name
  - mock_io

```rust
        // explicit keep-alive
        let input = b"GET / HTTP/1.1\r\nHost: pingora.org\r\nConnection: keep-alive\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        // default is infinite for 1.1
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1594
  - stubby variable name
  - mock_io

```rust
        let input = b"GET / HTTP/1.1\r\nHost: pingora.org\r\nConnection: keep-alive\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        // default is infinite for 1.1
        http_stream.read_request().await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1607
  - stubby variable name
  - mock_io

```rust
        // not specified
        let input = b"GET / HTTP/1.1\r\nHost: pingora.org\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1608
  - stubby variable name
  - mock_io

```rust
        let input = b"GET / HTTP/1.1\r\nHost: pingora.org\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        // default is infinite for 1.1
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1623
  - stubby variable name
  - mock_io

```rust
    async fn write() {
        let wire = b"HTTP/1.1 200 OK\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1624
  - stubby variable name
  - mock_io

```rust
        let wire = b"HTTP/1.1 200 OK\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response.append_header("Foo", "Bar").unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1637
  - stubby variable name
  - mock_io

```rust
    async fn write_custom_reason() {
        let wire = b"HTTP/1.1 200 Just Fine\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1638
  - stubby variable name
  - mock_io

```rust
        let wire = b"HTTP/1.1 200 Just Fine\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response.set_reason_phrase(Some("Just Fine")).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1652
  - stubby variable name
  - mock_io

```rust
    async fn write_informational() {
        let wire = b"HTTP/1.1 100 Continue\r\n\r\nHTTP/1.1 200 OK\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let response_100 = ResponseHeader::build(StatusCode::CONTINUE, None).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1653
  - stubby variable name
  - mock_io

```rust
        let wire = b"HTTP/1.1 100 Continue\r\n\r\nHTTP/1.1 200 OK\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let response_100 = ResponseHeader::build(StatusCode::CONTINUE, None).unwrap();
        http_stream
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1671
  - stubby variable name
  - mock_io

```rust
    async fn write_informational_ignored() {
        let wire = b"HTTP/1.1 200 OK\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        // ignore the 100 Continue
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1672
  - stubby variable name
  - mock_io

```rust
        let wire = b"HTTP/1.1 200 OK\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        // ignore the 100 Continue
        http_stream.ignore_info_resp = true;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1694
  - stubby variable name
  - mock_io

```rust
        let output = b"HTTP/1.1 100 Continue\r\n\r\nHTTP/1.1 200 OK\r\nFoo: Bar\r\n\r\n";

        let mock_io = Builder::new().read(&input[..]).write(output).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1695
  - stubby variable name
  - mock_io

```rust

        let mock_io = Builder::new().read(&input[..]).write(output).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        http_stream.ignore_info_resp = true;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1718
  - stubby variable name
  - mock_io

```rust
        let output = b"HTTP/1.1 200 OK\r\nFoo: Bar\r\n\r\n";

        let mock_io = Builder::new().read(&input[..]).write(output).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1719
  - stubby variable name
  - mock_io

```rust

        let mock_io = Builder::new().read(&input[..]).write(output).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        http_stream.ignore_info_resp = true;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1741
  - stubby variable name
  - mock_io

```rust
        let wire = b"HTTP/1.1 101 Switching Protocols\r\nFoo: Bar\r\n\r\n";
        let wire_body = b"nPAYLOAD";
        let mock_io = Builder::new().write(wire).write(wire_body).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut response_101 =
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1742
  - stubby variable name
  - mock_io

```rust
        let wire_body = b"nPAYLOAD";
        let mock_io = Builder::new().write(wire).write(wire_body).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut response_101 =
            ResponseHeader::build(StatusCode::SWITCHING_PROTOCOLS, None).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1766
  - stubby variable name
  - mock_io

```rust
        let wire_header = b"HTTP/1.1 200 OK\r\nContent-Length: 1\r\n\r\n";
        let wire_body = b"a";
        let mock_io = Builder::new().write(wire_header).write(wire_body).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1767
  - stubby variable name
  - mock_io

```rust
        let wire_body = b"a";
        let mock_io = Builder::new().write(wire_header).write(wire_body).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response.append_header("Content-Length", "1").unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1789
  - stubby variable name
  - mock_io

```rust
        let wire_header = b"HTTP/1.1 200 OK\r\n\r\n";
        let wire_body = b"a";
        let mock_io = Builder::new().write(wire_header).write(wire_body).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1790
  - stubby variable name
  - mock_io

```rust
        let wire_body = b"a";
        let mock_io = Builder::new().write(wire_header).write(wire_body).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        http_stream.update_resp_headers = false;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1809
  - stubby variable name
  - mock_io

```rust
        let wire_body = b"1\r\na\r\n";
        let wire_end = b"0\r\n\r\n";
        let mock_io = Builder::new()
            .write(wire_header)
            .write(wire_body)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1814
  - stubby variable name
  - mock_io

```rust
            .write(wire_end)
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1840
  - stubby variable name
  - mock_io

```rust
        let input2 = b"Host: pingora.org\r\nContent-Length: 3\r\n\r\n";
        let input3 = b"abc";
        let mock_io = Builder::new()
            .read(&input1[..])
            .read(&input2[..])
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1845
  - stubby variable name
  - mock_io

```rust
            .read(&input3[..])
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        assert_eq!(http_stream.get_path(), &b"/a?q=b%20c"[..]);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1886
  - stubby variable name
  - mock_io

```rust
    async fn test_write_body_buf() {
        let wire = b"HTTP/1.1 200 OK\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1887
  - stubby variable name
  - mock_io

```rust
        let wire = b"HTTP/1.1 200 OK\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response.append_header("Foo", "Bar").unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1904
  - stubby variable name
  - mock_io

```rust
        let wire1 = b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\n\r\n";
        let wire2 = b"abc";
        let mock_io = Builder::new()
            .write(wire1)
            .wait(Duration::from_millis(500))
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1909
  - stubby variable name
  - mock_io

```rust
            .write(wire2)
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.write_timeout = Some(Duration::from_millis(100));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1926
  - stubby variable name
  - mock_io

```rust
    async fn test_write_continue_resp() {
        let wire = b"HTTP/1.1 100 Continue\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.write_continue_response().await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1927
  - stubby variable name
  - mock_io

```rust
        let wire = b"HTTP/1.1 100 Continue\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.write_continue_response().await.unwrap();
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 791
  - hardcoded URL
  - 

```rust
            }

            /* follow https://tools.ietf.org/html/rfc7230#section-3.3.3 */
            let preread_body = self.preread_body.as_ref().unwrap().get(&self.buf[..]);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 1075
  - fallback
  - 

```rust
        let n_tasks = tasks.len();
        if n_tasks == 1 {
            // fallback to single operation to avoid copy
            return self.response_duplex(tasks.pop().unwrap()).await;
        }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 535: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let mut write_buf = BytesMut::with_capacity(INIT_HEADER_BUF_SIZE);
        http_resp_header_to_buf(&header, &mut write_buf).unwrap();
        match self.underlying_stream.write_all(&write_buf).await {
            Ok(()) => {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 792: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

            /* follow https://tools.ietf.org/html/rfc7230#section-3.3.3 */
            let preread_body = self.preread_body.as_ref().unwrap().get(&self.buf[..]);

            if self.req_header().version == Version::HTTP_11 && self.is_upgrade_req() {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 887: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    /// Return the raw bytes of the request header.
    pub fn get_headers_raw_bytes(&self) -> Bytes {
        self.raw_header.as_ref().unwrap().get_bytes(&self.buf)
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1037: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            // size hint Some(0) because default is 8
            return self
                .write_response_header(Box::new(ResponseHeader::build(100, Some(0)).unwrap()))
                .await;
        }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1076: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        if n_tasks == 1 {
            // fallback to single operation to avoid copy
            return self.response_duplex(tasks.pop().unwrap()).await;
        }
        let mut end_stream = false;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1132: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
// Regex to parse request line that has illegal chars in it
static REQUEST_LINE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\w+ (?P<uri>.+) HTTP/\d(?:\.\d)?").unwrap());

// the chars httparse considers illegal in URL
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1300: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
        assert_eq!(res, input3.as_slice());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1301: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
        assert_eq!(res, input3.as_slice());
        assert_eq!(http_stream.body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1301: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
        assert_eq!(res, input3.as_slice());
        assert_eq!(http_stream.body_reader.body_state, ParseState::Complete(3));
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
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_timeout = Some(Duration::from_secs(1));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await;
        assert_eq!(http_stream.body_bytes_read(), 0);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1335: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
        assert_eq!(res, b"abc".as_slice());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1336: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
        assert_eq!(res, b"abc".as_slice());
        assert_eq!(http_stream.body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1336: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
        assert_eq!(res, b"abc".as_slice());
        assert_eq!(http_stream.body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1356: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
        assert_eq!(res, input3.as_slice());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1357: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
        assert_eq!(res, input3.as_slice());
        assert_eq!(http_stream.body_reader.body_state, ParseState::HTTP1_0(1));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1357: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
        assert_eq!(res, input3.as_slice());
        assert_eq!(http_stream.body_reader.body_state, ParseState::HTTP1_0(1));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1361: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(http_stream.body_reader.body_state, ParseState::HTTP1_0(1));
        assert_eq!(http_stream.body_bytes_read(), 1);
        let res = http_stream.read_body_bytes().await.unwrap();
        assert!(res.is_none());
        assert_eq!(http_stream.body_reader.body_state, ParseState::Complete(1));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1381: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
        assert_eq!(res, b"a".as_slice());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1382: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
        assert_eq!(res, b"a".as_slice());
        assert_eq!(http_stream.body_reader.body_state, ParseState::HTTP1_0(1));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1382: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
        assert_eq!(res, b"a".as_slice());
        assert_eq!(http_stream.body_reader.body_state, ParseState::HTTP1_0(1));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1385: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(res, b"a".as_slice());
        assert_eq!(http_stream.body_reader.body_state, ParseState::HTTP1_0(1));
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
        assert_eq!(res, b"b".as_slice());
        assert_eq!(http_stream.body_reader.body_state, ParseState::HTTP1_0(2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1385: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(res, b"a".as_slice());
        assert_eq!(http_stream.body_reader.body_state, ParseState::HTTP1_0(1));
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
        assert_eq!(res, b"b".as_slice());
        assert_eq!(http_stream.body_reader.body_state, ParseState::HTTP1_0(2));
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
- **Issue**: Can panic in production code

```rust
        assert_eq!(res, b"b".as_slice());
        assert_eq!(http_stream.body_reader.body_state, ParseState::HTTP1_0(2));
        let res = http_stream.read_body_bytes().await.unwrap();
        assert_eq!(http_stream.body_bytes_read(), 2);
        assert!(res.is_none());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1401: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap();
        assert!(res.is_none());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1402: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        let res = http_stream.read_body_bytes().await.unwrap();
        assert!(res.is_none());
        assert_eq!(http_stream.body_bytes_read(), 0);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1420: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        assert!(http_stream.is_chunked_encoding());
        let res = http_stream.read_body_bytes().await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1422: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        http_stream.read_request().await.unwrap();
        assert!(http_stream.is_chunked_encoding());
        let res = http_stream.read_body_bytes().await.unwrap();
        assert!(res.is_none());
        assert_eq!(http_stream.body_bytes_read(), 0);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1440: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        assert!(http_stream.is_chunked_encoding());
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1442: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        http_stream.read_request().await.unwrap();
        assert!(http_stream.is_chunked_encoding());
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
        assert_eq!(res, b"a".as_slice());
        assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1442: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        http_stream.read_request().await.unwrap();
        assert!(http_stream.is_chunked_encoding());
        let res = http_stream.read_body_bytes().await.unwrap().unwrap();
        assert_eq!(res, b"a".as_slice());
        assert_eq!(
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
- **Issue**: Can panic in production code

```rust
            ParseState::Chunked(1, 0, 0, 0)
        );
        let res = http_stream.read_body_bytes().await.unwrap();
        assert!(res.is_none());
        assert_eq!(http_stream.body_bytes_read(), 1);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1485: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let _ = http_stream.read_request().await.unwrap();

        match (content_length_header, transfer_encoding_header) {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1518: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().read(input.as_bytes()).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        http_stream
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1528: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        assert!(!http_stream.is_upgrade_req());

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1535: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        assert!(http_stream.is_upgrade_req());

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1542: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        assert!(!http_stream.is_upgrade_req());

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1549: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        assert!(http_stream.is_upgrade_req());

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
- **Issue**: Can panic in production code

```rust
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        assert!(http_stream.is_upgrade_req());
        let mut response = ResponseHeader::build(StatusCode::CONTINUE, None).unwrap();
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
- **Issue**: Can panic in production code

```rust
        http_stream.read_request().await.unwrap();
        assert!(http_stream.is_upgrade_req());
        let mut response = ResponseHeader::build(StatusCode::CONTINUE, None).unwrap();
        response.set_version(http::Version::HTTP_11);
        http_stream
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1573: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_response_header(Box::new(response))
            .await
            .unwrap();
        // 100 won't affect body state
        assert!(!http_stream.is_body_done());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1584: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        // verify close
        assert_eq!(http_stream.keepalive_timeout, KeepaliveStatus::Off);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1596: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        // default is infinite for 1.1
        http_stream.read_request().await.unwrap();
        assert_eq!(http_stream.keepalive_timeout, KeepaliveStatus::Infinite);
        http_stream.set_server_keepalive(Some(60));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1609: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        // default is infinite for 1.1
        assert_eq!(http_stream.keepalive_timeout, KeepaliveStatus::Infinite);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1625: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response.append_header("Foo", "Bar").unwrap();
        http_stream.update_resp_headers = false;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1626: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response.append_header("Foo", "Bar").unwrap();
        http_stream.update_resp_headers = false;
        http_stream
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1631: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_response_header_ref(&new_response)
            .await
            .unwrap();
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1639: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response.set_reason_phrase(Some("Just Fine")).unwrap();
        new_response.append_header("Foo", "Bar").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1640: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response.set_reason_phrase(Some("Just Fine")).unwrap();
        new_response.append_header("Foo", "Bar").unwrap();
        http_stream.update_resp_headers = false;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1641: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response.set_reason_phrase(Some("Just Fine")).unwrap();
        new_response.append_header("Foo", "Bar").unwrap();
        http_stream.update_resp_headers = false;
        http_stream
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
- **Issue**: Can panic in production code

```rust
            .write_response_header_ref(&new_response)
            .await
            .unwrap();
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1654: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let response_100 = ResponseHeader::build(StatusCode::CONTINUE, None).unwrap();
        http_stream
            .write_response_header_ref(&response_100)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1658: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_response_header_ref(&response_100)
            .await
            .unwrap();
        let mut response_200 = ResponseHeader::build(StatusCode::OK, None).unwrap();
        response_200.append_header("Foo", "Bar").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1659: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .await
            .unwrap();
        let mut response_200 = ResponseHeader::build(StatusCode::OK, None).unwrap();
        response_200.append_header("Foo", "Bar").unwrap();
        http_stream.update_resp_headers = false;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1660: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .unwrap();
        let mut response_200 = ResponseHeader::build(StatusCode::OK, None).unwrap();
        response_200.append_header("Foo", "Bar").unwrap();
        http_stream.update_resp_headers = false;
        http_stream
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1665: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_response_header_ref(&response_200)
            .await
            .unwrap();
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1675: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // ignore the 100 Continue
        http_stream.ignore_info_resp = true;
        let response_100 = ResponseHeader::build(StatusCode::CONTINUE, None).unwrap();
        http_stream
            .write_response_header_ref(&response_100)
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
- **Issue**: Can panic in production code

```rust
            .write_response_header_ref(&response_100)
            .await
            .unwrap();
        let mut response_200 = ResponseHeader::build(StatusCode::OK, None).unwrap();
        response_200.append_header("Foo", "Bar").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1680: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .await
            .unwrap();
        let mut response_200 = ResponseHeader::build(StatusCode::OK, None).unwrap();
        response_200.append_header("Foo", "Bar").unwrap();
        http_stream.update_resp_headers = false;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1681: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .unwrap();
        let mut response_200 = ResponseHeader::build(StatusCode::OK, None).unwrap();
        response_200.append_header("Foo", "Bar").unwrap();
        http_stream.update_resp_headers = false;
        http_stream
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1686: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_response_header_ref(&response_200)
            .await
            .unwrap();
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1696: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().read(&input[..]).write(output).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        http_stream.ignore_info_resp = true;
        // 100 Continue is not ignored due to Expect: 100-continue on request
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1699: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        http_stream.ignore_info_resp = true;
        // 100 Continue is not ignored due to Expect: 100-continue on request
        let response_100 = ResponseHeader::build(StatusCode::CONTINUE, None).unwrap();
        http_stream
            .write_response_header_ref(&response_100)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1703: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_response_header_ref(&response_100)
            .await
            .unwrap();
        let mut response_200 = ResponseHeader::build(StatusCode::OK, None).unwrap();
        response_200.append_header("Foo", "Bar").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1704: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .await
            .unwrap();
        let mut response_200 = ResponseHeader::build(StatusCode::OK, None).unwrap();
        response_200.append_header("Foo", "Bar").unwrap();
        http_stream.update_resp_headers = false;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1705: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .unwrap();
        let mut response_200 = ResponseHeader::build(StatusCode::OK, None).unwrap();
        response_200.append_header("Foo", "Bar").unwrap();
        http_stream.update_resp_headers = false;
        http_stream
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1710: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_response_header_ref(&response_200)
            .await
            .unwrap();
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1720: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().read(&input[..]).write(output).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        http_stream.ignore_info_resp = true;
        // 102 Processing is ignored
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1723: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        http_stream.ignore_info_resp = true;
        // 102 Processing is ignored
        let response_102 = ResponseHeader::build(StatusCode::PROCESSING, None).unwrap();
        http_stream
            .write_response_header_ref(&response_102)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1727: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_response_header_ref(&response_102)
            .await
            .unwrap();
        let mut response_200 = ResponseHeader::build(StatusCode::OK, None).unwrap();
        response_200.append_header("Foo", "Bar").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1728: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .await
            .unwrap();
        let mut response_200 = ResponseHeader::build(StatusCode::OK, None).unwrap();
        response_200.append_header("Foo", "Bar").unwrap();
        http_stream.update_resp_headers = false;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1729: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .unwrap();
        let mut response_200 = ResponseHeader::build(StatusCode::OK, None).unwrap();
        response_200.append_header("Foo", "Bar").unwrap();
        http_stream.update_resp_headers = false;
        http_stream
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1734: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_response_header_ref(&response_200)
            .await
            .unwrap();
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1744: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut response_101 =
            ResponseHeader::build(StatusCode::SWITCHING_PROTOCOLS, None).unwrap();
        response_101.append_header("Foo", "Bar").unwrap();
        http_stream
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1745: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut response_101 =
            ResponseHeader::build(StatusCode::SWITCHING_PROTOCOLS, None).unwrap();
        response_101.append_header("Foo", "Bar").unwrap();
        http_stream
            .write_response_header_ref(&response_101)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1749: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_response_header_ref(&response_101)
            .await
            .unwrap();
        let n = http_stream.write_body(wire_body).await.unwrap().unwrap();
        assert_eq!(wire_body.len(), n);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1750: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .await
            .unwrap();
        let n = http_stream.write_body(wire_body).await.unwrap().unwrap();
        assert_eq!(wire_body.len(), n);
        // simulate upgrade
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1750: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .await
            .unwrap();
        let n = http_stream.write_body(wire_body).await.unwrap().unwrap();
        assert_eq!(wire_body.len(), n);
        // simulate upgrade
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1755: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        http_stream.upgraded = true;
        // this write should be ignored
        let response_502 = ResponseHeader::build(StatusCode::BAD_GATEWAY, None).unwrap();
        http_stream
            .write_response_header_ref(&response_502)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1759: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_response_header_ref(&response_502)
            .await
            .unwrap();
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1768: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().write(wire_header).write(wire_body).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response.append_header("Content-Length", "1").unwrap();
        http_stream.update_resp_headers = false;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1769: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response.append_header("Content-Length", "1").unwrap();
        http_stream.update_resp_headers = false;
        http_stream
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1774: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_response_header_ref(&new_response)
            .await
            .unwrap();
        assert_eq!(
            http_stream.body_writer.body_mode,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1779: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            BodyMode::ContentLength(1, 0)
        );
        let n = http_stream.write_body(wire_body).await.unwrap().unwrap();
        assert_eq!(wire_body.len(), n);
        let n = http_stream.finish_body().await.unwrap().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1779: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            BodyMode::ContentLength(1, 0)
        );
        let n = http_stream.write_body(wire_body).await.unwrap().unwrap();
        assert_eq!(wire_body.len(), n);
        let n = http_stream.finish_body().await.unwrap().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1781: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let n = http_stream.write_body(wire_body).await.unwrap().unwrap();
        assert_eq!(wire_body.len(), n);
        let n = http_stream.finish_body().await.unwrap().unwrap();
        assert_eq!(wire_body.len(), n);
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1781: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let n = http_stream.write_body(wire_body).await.unwrap().unwrap();
        assert_eq!(wire_body.len(), n);
        let n = http_stream.finish_body().await.unwrap().unwrap();
        assert_eq!(wire_body.len(), n);
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1791: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().write(wire_header).write(wire_body).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        http_stream.update_resp_headers = false;
        http_stream
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1796: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_response_header_ref(&new_response)
            .await
            .unwrap();
        assert_eq!(http_stream.body_writer.body_mode, BodyMode::HTTP1_0(0));
        let n = http_stream.write_body(wire_body).await.unwrap().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1798: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .unwrap();
        assert_eq!(http_stream.body_writer.body_mode, BodyMode::HTTP1_0(0));
        let n = http_stream.write_body(wire_body).await.unwrap().unwrap();
        assert_eq!(wire_body.len(), n);
        let n = http_stream.finish_body().await.unwrap().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1798: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .unwrap();
        assert_eq!(http_stream.body_writer.body_mode, BodyMode::HTTP1_0(0));
        let n = http_stream.write_body(wire_body).await.unwrap().unwrap();
        assert_eq!(wire_body.len(), n);
        let n = http_stream.finish_body().await.unwrap().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1800: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let n = http_stream.write_body(wire_body).await.unwrap().unwrap();
        assert_eq!(wire_body.len(), n);
        let n = http_stream.finish_body().await.unwrap().unwrap();
        assert_eq!(wire_body.len(), n);
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1800: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let n = http_stream.write_body(wire_body).await.unwrap().unwrap();
        assert_eq!(wire_body.len(), n);
        let n = http_stream.finish_body().await.unwrap().unwrap();
        assert_eq!(wire_body.len(), n);
    }
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
- **Issue**: Can panic in production code

```rust
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response
            .append_header("Transfer-Encoding", "chunked")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1818: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        new_response
            .append_header("Transfer-Encoding", "chunked")
            .unwrap();
        http_stream.update_resp_headers = false;
        http_stream
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1823: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_response_header_ref(&new_response)
            .await
            .unwrap();
        assert_eq!(
            http_stream.body_writer.body_mode,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1828: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            BodyMode::ChunkedEncoding(0)
        );
        let n = http_stream.write_body(b"a").await.unwrap().unwrap();
        assert_eq!(b"a".len(), n);
        let n = http_stream.finish_body().await.unwrap().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1828: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            BodyMode::ChunkedEncoding(0)
        );
        let n = http_stream.write_body(b"a").await.unwrap().unwrap();
        assert_eq!(b"a".len(), n);
        let n = http_stream.finish_body().await.unwrap().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1830: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let n = http_stream.write_body(b"a").await.unwrap().unwrap();
        assert_eq!(b"a".len(), n);
        let n = http_stream.finish_body().await.unwrap().unwrap();
        assert_eq!(b"a".len(), n);
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1830: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let n = http_stream.write_body(b"a").await.unwrap().unwrap();
        assert_eq!(b"a".len(), n);
        let n = http_stream.finish_body().await.unwrap().unwrap();
        assert_eq!(b"a".len(), n);
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1846: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_request().await.unwrap();
        assert_eq!(http_stream.get_path(), &b"/a?q=b%20c"[..]);
        let res = http_stream.read_body().await.unwrap().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1848: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        http_stream.read_request().await.unwrap();
        assert_eq!(http_stream.get_path(), &b"/a?q=b%20c"[..]);
        let res = http_stream.read_body().await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 3));
        assert_eq!(http_stream.body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1848: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        http_stream.read_request().await.unwrap();
        assert_eq!(http_stream.get_path(), &b"/a?q=b%20c"[..]);
        let res = http_stream.read_body().await.unwrap().unwrap();
        assert_eq!(res, BufRef::new(0, 3));
        assert_eq!(http_stream.body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1861: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            &b"GET /a?q=<\"b c\"> HTTP/1.1\r\nHost: pingora.org\r\nContent-Length: 3\r\n\r\n"[..],
        );
        let output = escape_illegal_request_line(&input).unwrap();
        assert_eq!(
            &output,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1871: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            &b"GET /a:\"bc\" HTTP/1.1\r\nHost: pingora.org\r\nContent-Length: 3\r\n\r\n"[..],
        );
        let output = escape_illegal_request_line(&input).unwrap();
        assert_eq!(
            &output,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1888: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response.append_header("Foo", "Bar").unwrap();
        http_stream.update_resp_headers = false;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1889: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response.append_header("Foo", "Bar").unwrap();
        http_stream.update_resp_headers = false;
        http_stream
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1894: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_response_header_ref(&new_response)
            .await
            .unwrap();
        let written = http_stream.write_body_buf().await.unwrap();
        assert!(written.is_none());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1895: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .await
            .unwrap();
        let written = http_stream.write_body_buf().await.unwrap();
        assert!(written.is_none());
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1911: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.write_timeout = Some(Duration::from_millis(100));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response.append_header("Content-Length", "3").unwrap();
        http_stream.update_resp_headers = false;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1912: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        http_stream.write_timeout = Some(Duration::from_millis(100));
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response.append_header("Content-Length", "3").unwrap();
        http_stream.update_resp_headers = false;
        http_stream
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1917: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_response_header_ref(&new_response)
            .await
            .unwrap();
        http_stream.body_write_buf = BytesMut::from(&b"abc"[..]);
        let res = http_stream.write_body_buf().await;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1928: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.write_continue_response().await.unwrap();
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1992: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn test_response_to_wire() {
        init_log();
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response.append_header("Foo", "Bar").unwrap();
        let mut wire = BytesMut::with_capacity(INIT_HEADER_BUF_SIZE);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1993: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        init_log();
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
        new_response.append_header("Foo", "Bar").unwrap();
        let mut wire = BytesMut::with_capacity(INIT_HEADER_BUF_SIZE);
        http_resp_header_to_buf(&new_response, &mut wire).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1995: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        new_response.append_header("Foo", "Bar").unwrap();
        let mut wire = BytesMut::with_capacity(INIT_HEADER_BUF_SIZE);
        http_resp_header_to_buf(&new_response, &mut wire).unwrap();
        debug!("{}", str::from_utf8(wire.as_ref()).unwrap());
        let mut headers = [httparse::EMPTY_HEADER; 128];
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2080: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(mocked_blocking_body_forever_stream());
        http_stream.read_timeout = None;
        http_stream.read_request().await.unwrap();
        let res = test_read_with_tokio_timeout(http_stream.read_body_bytes()).await;
        assert!(res.is_err()); // test timeout occurred, and not any internal Pingora timeout
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2087: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(mocked_blocking_body_forever_stream());
        http_stream.read_timeout = Some(TEST_READ_TIMEOUT);
        http_stream.read_request().await.unwrap();
        let res = test_read_with_tokio_timeout(http_stream.read_body_bytes()).await;
        assert!(res.is_ok());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 329: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        self.request_header
            .as_ref()
            .expect("Request header is not read yet")
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 338: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        self.request_header
            .as_mut()
            .expect("Request header is not read yet")
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


### Line 1231: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1231)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests_stream {
    use super::*;
    use crate::protocols::http::v1::body::{BodyMode, ParseState};
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1244: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1244)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_basic() {
        init_log();
        let input = b"GET / HTTP/1.1\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1256: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1256)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[cfg(feature = "patched_http1")]
    #[tokio::test]
    async fn read_invalid_path() {
        init_log();
        let input = b"GET /\x01\xF0\x90\x80 HTTP/1.1\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1268: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1268)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_2_buf() {
        init_log();
        let input1 = b"GET / HTTP/1.1\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1289: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1289)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_content_length() {
        init_log();
        let input1 = b"GET / HTTP/1.1\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1309: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1309)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[tokio::test]
    #[should_panic(expected = "There is still data left to read.")]
    async fn read_with_body_timeout() {
        init_log();
        let input1 = b"GET / HTTP/1.1\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1329: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1329)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_content_length_single_read() {
        init_log();
        let input1 = b"GET / HTTP/1.1\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1343: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1343)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_http10() {
        init_log();
        let input1 = b"GET / HTTP/1.0\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1368: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1368)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_http10_single_read() {
        init_log();
        let input1 = b"GET / HTTP/1.0\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1395: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1395)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_http11_default_no_body() {
        init_log();
        let input1 = b"GET / HTTP/1.1\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1409: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1409)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_chunked_0() {
        init_log();
        let input1 = b"GET / HTTP/1.1\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1429: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1429)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_body_chunked_single_read() {
        init_log();
        let input1 = b"GET / HTTP/1.1\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1464: `#[rstest]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1464)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[case(None, Some("content-length"))]
    #[tokio::test]
    async fn transfer_encoding_and_content_length_disallowed(
        #[case] transfer_encoding_header: Option<&str>,
        #[case] content_length_header: Option<&str>,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1464: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1464)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[case(None, Some("content-length"))]
    #[tokio::test]
    async fn transfer_encoding_and_content_length_disallowed(
        #[case] transfer_encoding_header: Option<&str>,
        #[case] content_length_header: Option<&str>,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1505: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1505)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[tokio::test]
    #[should_panic(expected = "There is still data left to read.")]
    async fn read_invalid() {
        let input1 = b"GET / HTP/1.1\r\n";
        let input2 = b"Host: pingora.org\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1523: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1523)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_upgrade_req() {
        // http 1.0
        let input = b"GET / HTTP/1.0\r\nHost: pingora.org\r\nUpgrade: websocket\r\nConnection: upgrade\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1559: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1559)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_upgrade_req_with_1xx_response() {
        let input = b"GET / HTTP/1.1\r\nHost: pingora.org\r\nUpgrade: websocket\r\nConnection: upgrade\r\n\r\n";
        let mock_io = Builder::new()
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1579: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1579)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn set_server_keepalive() {
        // close
        let input = b"GET / HTTP/1.1\r\nHost: pingora.org\r\nConnection: close\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1621: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1621)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn write() {
        let wire = b"HTTP/1.1 200 OK\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1635: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1635)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn write_custom_reason() {
        let wire = b"HTTP/1.1 200 Just Fine\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1650: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1650)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn write_informational() {
        let wire = b"HTTP/1.1 100 Continue\r\n\r\nHTTP/1.1 200 OK\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1669: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1669)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn write_informational_ignored() {
        let wire = b"HTTP/1.1 200 OK\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1690: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1690)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn write_informational_100_not_ignored_if_expect_continue() {
        let input = b"GET / HTTP/1.1\r\nExpect: 100-continue\r\n\r\n";
        let output = b"HTTP/1.1 100 Continue\r\n\r\nHTTP/1.1 200 OK\r\nFoo: Bar\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1714: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1714)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn write_informational_1xx_ignored_if_expect_continue() {
        let input = b"GET / HTTP/1.1\r\nExpect: 100-continue\r\n\r\n";
        let output = b"HTTP/1.1 200 OK\r\nFoo: Bar\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1738: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1738)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn write_101_switching_protocol() {
        let wire = b"HTTP/1.1 101 Switching Protocols\r\nFoo: Bar\r\n\r\n";
        let wire_body = b"nPAYLOAD";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1763: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1763)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn write_body_cl() {
        let wire_header = b"HTTP/1.1 200 OK\r\nContent-Length: 1\r\n\r\n";
        let wire_body = b"a";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1786: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1786)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn write_body_http10() {
        let wire_header = b"HTTP/1.1 200 OK\r\n\r\n";
        let wire_body = b"a";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1805: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1805)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn write_body_chunk() {
        let wire_header = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n";
        let wire_body = b"1\r\na\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1835: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1835)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_with_illegal() {
        init_log();
        let input1 = b"GET /a?q=b c HTTP/1.1\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1855: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1855)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn escape_illegal() {
        init_log();
        // in query string
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1884: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1884)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_write_body_buf() {
        let wire = b"HTTP/1.1 200 OK\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1901: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1901)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[tokio::test]
    #[should_panic(expected = "There is still data left to write.")]
    async fn test_write_body_buf_write_timeout() {
        let wire1 = b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\n\r\n";
        let wire2 = b"abc";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1924: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1924)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_write_continue_resp() {
        let wire = b"HTTP/1.1 100 Continue\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1932: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1932)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_get_write_timeout() {
        let mut http_stream = HttpSession::new(Box::new(Builder::new().build()));
        let expected = Duration::from_secs(5);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1941: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1941)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_get_write_timeout_none() {
        let http_stream = HttpSession::new(Box::new(Builder::new().build()));
        assert!(http_stream.write_timeout(50).is_none());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1947: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1947)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_get_write_timeout_min_send_rate_zero() {
        let mut http_stream = HttpSession::new(Box::new(Builder::new().build()));
        http_stream.set_min_send_rate(Some(0));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1958: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1958)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_get_write_timeout_min_send_rate_overrides_write_timeout() {
        let mut http_stream = HttpSession::new(Box::new(Builder::new().build()));
        let expected = Duration::from_millis(29800);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1969: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1969)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_get_write_timeout_min_send_rate_max_zero_buf() {
        let mut http_stream = HttpSession::new(Box::new(Builder::new().build()));
        let expected = Duration::from_secs(1);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1979: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1979)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod test_sync {
    use super::*;
    use http::StatusCode;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1990: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 1990)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_response_to_wire() {
        init_log();
        let mut new_response = ResponseHeader::build(StatusCode::OK, None).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2012: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 2012)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod test_timeouts {
    use super::*;
    use std::future::IntoFuture;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2060: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 2060)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_read_http_request_headers_timeout_for_read_request() {
        // confirm that a `read_timeout` of `None` would've waited "indefinitely"
        let mut http_stream = HttpSession::new(mocked_blocking_headers_forever_stream());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 2076: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/server.rs` (line 2076)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_read_http_body_timeout_for_read_body_bytes() {
        // confirm that a `read_timeout` of `None` would've waited "indefinitely"
        let mut http_stream = HttpSession::new(mocked_blocking_body_forever_stream());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym