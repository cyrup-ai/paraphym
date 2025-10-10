# `forks/pingora/pingora-core/src/protocols/http/v1/client.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-core
- **File Hash**: d06b9c2f  
- **Timestamp**: 2025-10-10T02:16:01.205251+00:00  
- **Lines of Code**: 1033

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 1033 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 63
  - TODO
  - 

```rust
    /// Create a new http client session from an established (TCP or TLS) [`Stream`].
    pub fn new(stream: Stream) -> Self {
        // TODO: maybe we should put digest in the connection itself
        let digest = Box::new(Digest {
            ssl_digest: stream.get_ssl_digest(),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 91
  - TODO
  - 

```rust
    /// sending request body if any.
    pub async fn write_request_header(&mut self, req: Box<RequestHeader>) -> Result<usize> {
        // TODO: make sure this can only be called once
        // init body writer
        self.init_req_body_writer(&req);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 139
  - TODO
  - 

```rust
    /// Content-Length or the last chunk is already sent
    pub async fn write_body(&mut self, buf: &[u8]) -> Result<Option<usize>> {
        // TODO: verify that request header is sent already
        match self.write_timeout {
            Some(t) => match timeout(t, self.do_write_body(buf)).await {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 386
  - TODO
  - 

```rust

    pub(super) fn get_headers_raw(&self) -> &[u8] {
        // TODO: these get_*() could panic. handle them better
        self.raw_header.as_ref().unwrap().get(&self.buf[..])
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 428
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


- Line 445
  - TODO
  - 

```rust
    // Whether this session will be kept alive
    pub fn will_keepalive(&self) -> bool {
        // TODO: check self.body_writer. If it is http1.0 type then keepalive
        // cannot be used because the connection close is the signal of end body
        !matches!(self.keepalive_timeout, KeepaliveStatus::Off)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 503
  - TODO
  - 

```rust
    /// returned.
    pub async fn reuse(mut self) -> Option<Stream> {
        // TODO: this function is unnecessarily slow for keepalive case
        // because that case does not need async
        match self.keepalive_timeout {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 652
  - TODO
  - 

```rust
            Ok(HttpTask::Body(body, end_of_body))
        }
        // TODO: support h1 trailer
    }

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 717
  - TODO
  - 

```rust
}

// TODO: change it to to_buf
#[inline]
pub(crate) fn http_req_header_to_wire(req: &RequestHeader) -> Option<BytesMut> {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1281
  - FIXME
  - 

```rust
        }
        assert!(result.unwrap().is_complete());
        // FIXME: the order is not guaranteed
        assert_eq!("/", req.path.unwrap());
        assert_eq!(b"Foo", headers[0].name.as_bytes());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 768
  - stubby variable name
  - mock_io

```rust
        init_log();
        let input = b"HTTP/1.1 200 OK\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_response().await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 769
  - stubby variable name
  - mock_io

```rust
        let input = b"HTTP/1.1 200 OK\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_response().await;
        assert_eq!(input.len(), res.unwrap());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 779
  - stubby variable name
  - mock_io

```rust
        init_log();
        let input = b"HTTP/1.1 200 Just Fine\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_response().await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 780
  - stubby variable name
  - mock_io

```rust
        let input = b"HTTP/1.1 200 Just Fine\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_response().await;
        assert_eq!(input.len(), res.unwrap());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 795
  - stubby variable name
  - mock_io

```rust
        let input_body = b"abc";
        let input_close = b""; // simulating close
        let mock_io = Builder::new()
            .read(&input_header[..])
            .read(&input_body[..])
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 800
  - stubby variable name
  - mock_io

```rust
            .read(&input_close[..])
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_response().await;
        assert_eq!(input_header.len(), res.unwrap());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 817
  - stubby variable name
  - mock_io

```rust
        let input_header2 = b"Content-Length: 2\r\n\r\n";
        let input_body = b"abc";
        let mock_io = Builder::new()
            .read(&input_header[..])
            .read(&input_header2[..])
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 822
  - stubby variable name
  - mock_io

```rust
            .read(&input_body[..])
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_response().await;
        assert_eq!(input_header.len() + input_header2.len(), res.unwrap());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 839
  - stubby variable name
  - mock_io

```rust
        init_log();
        let input = b"HTTP/1.1 200 OK\r\nServer : pingora\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_response().await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 840
  - stubby variable name
  - mock_io

```rust
        let input = b"HTTP/1.1 200 OK\r\nServer : pingora\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_response().await;
        assert_eq!(input.len(), res.unwrap());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 852
  - stubby variable name
  - mock_io

```rust
        init_log();
        let input = "HTTP/1.1 200 OK\r\nServer👍: pingora\r\n\r\n".as_bytes();
        let mock_io = Builder::new().read(input).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let resp = http_stream.read_resp_header_parts().await.unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 853
  - stubby variable name
  - mock_io

```rust
        let input = "HTTP/1.1 200 OK\r\nServer👍: pingora\r\n\r\n".as_bytes();
        let mock_io = Builder::new().read(input).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let resp = http_stream.read_resp_header_parts().await.unwrap();
        assert_eq!(1, http_stream.resp_header().unwrap().headers.len());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 865
  - stubby variable name
  - mock_io

```rust
        init_log();
        let input = b"HTTP/1.1 200 OK\r\n\r\n";
        let mock_io = Builder::new()
            .wait(Duration::from_secs(2))
            .read(&input[..])
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 869
  - stubby variable name
  - mock_io

```rust
            .read(&input[..])
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.read_timeout = Some(Duration::from_secs(1));
        let res = http_stream.read_response().await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 880
  - stubby variable name
  - mock_io

```rust
        let input1 = b"HTTP/1.1 200 OK\r\n";
        let input2 = b"Server: pingora\r\n\r\n";
        let mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_response().await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 881
  - stubby variable name
  - mock_io

```rust
        let input2 = b"Server: pingora\r\n\r\n";
        let mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_response().await;
        assert_eq!(input1.len() + input2.len(), res.unwrap());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 900
  - stubby variable name
  - mock_io

```rust
        let input1 = b"HTP/1.1 200 OK\r\n";
        let input2 = b"Server: pingora\r\n\r\n";
        let mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_response().await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 901
  - stubby variable name
  - mock_io

```rust
        let input2 = b"Server: pingora\r\n\r\n";
        let mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_response().await;
        assert_eq!(&ErrorType::InvalidHTTPHeader, res.unwrap_err().etype());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 909
  - stubby variable name
  - mock_io

```rust
    async fn write() {
        let wire = b"GET /test HTTP/1.1\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_request = RequestHeader::build("GET", b"/test", None).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 910
  - stubby variable name
  - mock_io

```rust
        let wire = b"GET /test HTTP/1.1\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_request = RequestHeader::build("GET", b"/test", None).unwrap();
        new_request.insert_header("Foo", "Bar").unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 924
  - stubby variable name
  - mock_io

```rust
    async fn write_timeout() {
        let wire = b"GET /test HTTP/1.1\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new()
            .wait(Duration::from_secs(2))
            .write(wire)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 928
  - stubby variable name
  - mock_io

```rust
            .write(wire)
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.write_timeout = Some(Duration::from_secs(1));
        let mut new_request = RequestHeader::build("GET", b"/test", None).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 943
  - stubby variable name
  - mock_io

```rust
        let header = b"POST /test HTTP/1.1\r\n\r\n";
        let body = b"abc";
        let mock_io = Builder::new()
            .write(&header[..])
            .wait(Duration::from_secs(2))
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 948
  - stubby variable name
  - mock_io

```rust
            .write(&body[..])
            .build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.write_timeout = Some(Duration::from_secs(1));

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 964
  - stubby variable name
  - mock_io

```rust
    async fn write_invalid_path() {
        let wire = b"GET /\x01\xF0\x90\x80 HTTP/1.1\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_request = RequestHeader::build("GET", b"/\x01\xF0\x90\x80", None).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 965
  - stubby variable name
  - mock_io

```rust
        let wire = b"GET /\x01\xF0\x90\x80 HTTP/1.1\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_request = RequestHeader::build("GET", b"/\x01\xF0\x90\x80", None).unwrap();
        new_request.insert_header("Foo", "Bar").unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 980
  - stubby variable name
  - mock_io

```rust
        let input1 = b"HTTP/1.1 100 Continue\r\n\r\n";
        let input2 = b"HTTP/1.1 204 OK\r\nServer: pingora\r\n\r\n";
        let mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 981
  - stubby variable name
  - mock_io

```rust
        let input2 = b"HTTP/1.1 204 OK\r\nServer: pingora\r\n\r\n";
        let mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));

        // read 100 header first
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1013
  - stubby variable name
  - mock_io

```rust
        let wire =
            b"GET / HTTP/1.1\r\nConnection: Upgrade\r\nUpgrade: WS\r\nContent-Length: 0\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_request = RequestHeader::build("GET", b"/", None).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1014
  - stubby variable name
  - mock_io

```rust
            b"GET / HTTP/1.1\r\nConnection: Upgrade\r\nUpgrade: WS\r\nContent-Length: 0\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_request = RequestHeader::build("GET", b"/", None).unwrap();
        new_request.insert_header("Connection", "Upgrade").unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1032
  - stubby variable name
  - mock_io

```rust
        let input1 = b"HTTP/1.1 101 Continue\r\n\r\n";
        let input2 = b"PAYLOAD";
        let mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1033
  - stubby variable name
  - mock_io

```rust
        let input2 = b"PAYLOAD";
        let mock_io = Builder::new().read(&input1[..]).read(&input2[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));

        // read 100 header first
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1077
  - stubby variable name
  - mock_io

```rust
        init_log();
        let input = b"HTTP/1.1 200 OK\r\nServer : pingora\r\n Foo: Bar\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_response().await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1078
  - stubby variable name
  - mock_io

```rust
        let input = b"HTTP/1.1 200 OK\r\nServer : pingora\r\n Foo: Bar\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_response().await;
        assert_eq!(input.len(), res.unwrap());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1089
  - stubby variable name
  - mock_io

```rust

        let input = b"HTTP/1.1 200 OK\r\nServer : pingora\r\n\t  Fizz: Buzz\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_response().await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1090
  - stubby variable name
  - mock_io

```rust
        let input = b"HTTP/1.1 200 OK\r\nServer : pingora\r\n\t  Fizz: Buzz\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_response().await;
        assert_eq!(input.len(), res.unwrap());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1105
  - stubby variable name
  - mock_io

```rust
        init_log();
        let input = b"HTTP/1.1 200 OK\r\n;\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_response().await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1106
  - stubby variable name
  - mock_io

```rust
        let input = b"HTTP/1.1 200 OK\r\n;\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().read(&input[..]).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let res = http_stream.read_response().await;
        assert_eq!(input.len(), res.unwrap());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1119
  - stubby variable name
  - mock_io

```rust
        async fn build_resp_with_keepalive(conn: &str) -> HttpSession {
            let input = format!("HTTP/1.1 200 OK\r\nConnection: {conn}\r\n\r\n");
            let mock_io = Builder::new().read(input.as_bytes()).build();
            let mut http_stream = HttpSession::new(Box::new(mock_io));
            let res = http_stream.read_response().await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1120
  - stubby variable name
  - mock_io

```rust
            let input = format!("HTTP/1.1 200 OK\r\nConnection: {conn}\r\n\r\n");
            let mock_io = Builder::new().read(input.as_bytes()).build();
            let mut http_stream = HttpSession::new(Box::new(mock_io));
            let res = http_stream.read_response().await;
            assert_eq!(input.len(), res.unwrap());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1202
  - stubby variable name
  - mock_io

```rust
        async fn build_resp_with_keepalive_values(keep_alive: &str) -> HttpSession {
            let input = format!("HTTP/1.1 200 OK\r\nKeep-Alive: {keep_alive}\r\n\r\n");
            let mock_io = Builder::new().read(input.as_bytes()).build();
            let mut http_stream = HttpSession::new(Box::new(mock_io));
            let res = http_stream.read_response().await;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1203
  - stubby variable name
  - mock_io

```rust
            let input = format!("HTTP/1.1 200 OK\r\nKeep-Alive: {keep_alive}\r\n\r\n");
            let mock_io = Builder::new().read(input.as_bytes()).build();
            let mut http_stream = HttpSession::new(Box::new(mock_io));
            let res = http_stream.read_response().await;
            assert_eq!(input.len(), res.unwrap());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 517
  - hardcoded URL
  - 

```rust
    fn init_body_reader(&mut self) {
        if self.body_reader.need_init() {
            /* follow https://tools.ietf.org/html/rfc7230#section-3.3.3 */
            let preread_body = self.preread_body.as_ref().unwrap().get(&self.buf[..]);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 95: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        self.init_req_body_writer(&req);

        let to_wire = http_req_header_to_wire(&req).unwrap();
        trace!("Writing request header: {to_wire:?}");

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

                    let mut response_header = Box::new(ResponseHeader::build(
                        resp.code.unwrap(),
                        Some(resp.headers.len()),
                    )?);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 323: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        self.read_response().await?;
        // safe to unwrap because it is just read
        Ok(Box::new(self.resp_header().unwrap().clone()))
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
- **Issue**: Can panic in production code

```rust
    pub(super) fn get_headers_raw(&self) -> &[u8] {
        // TODO: these get_*() could panic. handle them better
        self.raw_header.as_ref().unwrap().get(&self.buf[..])
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 392: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    /// Get the raw response header bytes
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


### Line 518: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        if self.body_reader.need_init() {
            /* follow https://tools.ietf.org/html/rfc7230#section-3.3.3 */
            let preread_body = self.preread_body.as_ref().unwrap().get(&self.buf[..]);

            if let Some(req) = self.request_written.as_ref() {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 803: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let res = http_stream.read_response().await;
        assert_eq!(input_header.len(), res.unwrap());
        let res = http_stream.read_body_ref().await.unwrap();
        assert_eq!(res.unwrap(), input_body);
        assert_eq!(http_stream.body_reader.body_state, ParseState::HTTP1_0(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 806: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(res.unwrap(), input_body);
        assert_eq!(http_stream.body_reader.body_state, ParseState::HTTP1_0(3));
        let res = http_stream.read_body_ref().await.unwrap();
        assert_eq!(res, None);
        assert_eq!(http_stream.body_reader.body_state, ParseState::Complete(3));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 825: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let res = http_stream.read_response().await;
        assert_eq!(input_header.len() + input_header2.len(), res.unwrap());
        let res = http_stream.read_body_ref().await.unwrap();
        assert_eq!(res.unwrap(), &input_body[..2]);
        assert_eq!(http_stream.body_reader.body_state, ParseState::Complete(2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 828: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(res.unwrap(), &input_body[..2]);
        assert_eq!(http_stream.body_reader.body_state, ParseState::Complete(2));
        let res = http_stream.read_body_ref().await.unwrap();
        assert_eq!(res, None);
        assert_eq!(http_stream.body_reader.body_state, ParseState::Complete(2));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 854: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().read(input).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let resp = http_stream.read_resp_header_parts().await.unwrap();
        assert_eq!(1, http_stream.resp_header().unwrap().headers.len());
        assert_eq!(http_stream.get_header("Server👍").unwrap(), "pingora");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 911: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_request = RequestHeader::build("GET", b"/test", None).unwrap();
        new_request.insert_header("Foo", "Bar").unwrap();
        let n = http_stream
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 912: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_request = RequestHeader::build("GET", b"/test", None).unwrap();
        new_request.insert_header("Foo", "Bar").unwrap();
        let n = http_stream
            .write_request_header(Box::new(new_request))
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
- **Issue**: Can panic in production code

```rust
            .write_request_header(Box::new(new_request))
            .await
            .unwrap();
        assert_eq!(wire.len(), n);
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 930: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        http_stream.write_timeout = Some(Duration::from_secs(1));
        let mut new_request = RequestHeader::build("GET", b"/test", None).unwrap();
        new_request.insert_header("Foo", "Bar").unwrap();
        let res = http_stream
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 931: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        http_stream.write_timeout = Some(Duration::from_secs(1));
        let mut new_request = RequestHeader::build("GET", b"/test", None).unwrap();
        new_request.insert_header("Foo", "Bar").unwrap();
        let res = http_stream
            .write_request_header(Box::new(new_request))
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 951: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        http_stream.write_timeout = Some(Duration::from_secs(1));

        let new_request = RequestHeader::build("POST", b"/test", None).unwrap();
        http_stream
            .write_request_header(Box::new(new_request))
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
- **Issue**: Can panic in production code

```rust
            .write_request_header(Box::new(new_request))
            .await
            .unwrap();
        let res = http_stream.write_body(body).await;
        assert_eq!(res.unwrap_err().etype(), &WriteTimedout);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 966: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_request = RequestHeader::build("GET", b"/\x01\xF0\x90\x80", None).unwrap();
        new_request.insert_header("Foo", "Bar").unwrap();
        let n = http_stream
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 967: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_request = RequestHeader::build("GET", b"/\x01\xF0\x90\x80", None).unwrap();
        new_request.insert_header("Foo", "Bar").unwrap();
        let n = http_stream
            .write_request_header(Box::new(new_request))
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 971: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_request_header(Box::new(new_request))
            .await
            .unwrap();
        assert_eq!(wire.len(), n);
    }
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
- **Issue**: Can panic in production code

```rust

        // read 100 header first
        let task = http_stream.read_response_task().await.unwrap();
        match task {
            HttpTask::Header(h, eob) => {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 995: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        }
        // read 200 header next
        let task = http_stream.read_response_task().await.unwrap();
        match task {
            HttpTask::Header(h, eob) => {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1015: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mock_io = Builder::new().write(wire).build();
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_request = RequestHeader::build("GET", b"/", None).unwrap();
        new_request.insert_header("Connection", "Upgrade").unwrap();
        new_request.insert_header("Upgrade", "WS").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1016: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut http_stream = HttpSession::new(Box::new(mock_io));
        let mut new_request = RequestHeader::build("GET", b"/", None).unwrap();
        new_request.insert_header("Connection", "Upgrade").unwrap();
        new_request.insert_header("Upgrade", "WS").unwrap();
        // CL is ignored when Upgrade presents
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
- **Issue**: Can panic in production code

```rust
        let mut new_request = RequestHeader::build("GET", b"/", None).unwrap();
        new_request.insert_header("Connection", "Upgrade").unwrap();
        new_request.insert_header("Upgrade", "WS").unwrap();
        // CL is ignored when Upgrade presents
        new_request.insert_header("Content-Length", "0").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1019: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        new_request.insert_header("Upgrade", "WS").unwrap();
        // CL is ignored when Upgrade presents
        new_request.insert_header("Content-Length", "0").unwrap();
        let _ = http_stream
            .write_request_header(Box::new(new_request))
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1023: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_request_header(Box::new(new_request))
            .await
            .unwrap();
        assert_eq!(http_stream.body_writer.body_mode, BodyMode::HTTP1_0(0));
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1036: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        // read 100 header first
        let task = http_stream.read_response_task().await.unwrap();
        match task {
            HttpTask::Header(h, eob) => {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1047: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        }
        // read body
        let task = http_stream.read_response_task().await.unwrap();
        match task {
            HttpTask::Body(b, eob) => {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1058: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        }
        // read body
        let task = http_stream.read_response_task().await.unwrap();
        match task {
            HttpTask::Body(b, eob) => {
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
- **Issue**: Can panic in production code

```rust
    #[test]
    fn test_request_to_wire() {
        let mut new_request = RequestHeader::build("GET", b"/", None).unwrap();
        new_request.insert_header("Foo", "Bar").unwrap();
        let wire = http_req_header_to_wire(&new_request).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1271: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn test_request_to_wire() {
        let mut new_request = RequestHeader::build("GET", b"/", None).unwrap();
        new_request.insert_header("Foo", "Bar").unwrap();
        let wire = http_req_header_to_wire(&new_request).unwrap();
        let mut headers = [httparse::EMPTY_HEADER; 128];
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1272: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut new_request = RequestHeader::build("GET", b"/", None).unwrap();
        new_request.insert_header("Foo", "Bar").unwrap();
        let wire = http_req_header_to_wire(&new_request).unwrap();
        let mut headers = [httparse::EMPTY_HEADER; 128];
        let mut req = httparse::Request::new(&mut headers);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 175: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            .response_header
            .as_ref()
            .expect("response header must be read");

        // ad-hoc checks
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 754: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 754)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests_stream {
    use super::*;
    use crate::protocols::http::v1::body::ParseState;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 765: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 765)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_basic_response() {
        init_log();
        let input = b"HTTP/1.1 200 OK\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 776: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 776)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_response_custom_reason() {
        init_log();
        let input = b"HTTP/1.1 200 Just Fine\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 790: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 790)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_response_default() {
        init_log();
        let input_header = b"HTTP/1.1 200 OK\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 812: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 812)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_response_overread() {
        init_log();
        let input_header = b"HTTP/1.1 200 OK\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 836: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 836)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_resp_header_with_space() {
        init_log();
        let input = b"HTTP/1.1 200 OK\r\nServer : pingora\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 849: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 849)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[cfg(feature = "patched_http1")]
    #[tokio::test]
    async fn read_resp_header_with_utf8() {
        init_log();
        let input = "HTTP/1.1 200 OK\r\nServer👍: pingora\r\n\r\n".as_bytes();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 862: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 862)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[tokio::test]
    #[should_panic(expected = "There is still data left to read.")]
    async fn read_timeout() {
        init_log();
        let input = b"HTTP/1.1 200 OK\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 876: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 876)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_2_buf() {
        init_log();
        let input1 = b"HTTP/1.1 200 OK\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 897: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 897)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[tokio::test]
    #[should_panic(expected = "There is still data left to read.")]
    async fn read_invalid() {
        let input1 = b"HTP/1.1 200 OK\r\n";
        let input2 = b"Server: pingora\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 907: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 907)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn write() {
        let wire = b"GET /test HTTP/1.1\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 922: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 922)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[tokio::test]
    #[should_panic(expected = "There is still data left to write.")]
    async fn write_timeout() {
        let wire = b"GET /test HTTP/1.1\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new()
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 940: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 940)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[tokio::test]
    #[should_panic(expected = "There is still data left to write.")]
    async fn write_body_timeout() {
        let header = b"POST /test HTTP/1.1\r\n\r\n";
        let body = b"abc";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 962: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 962)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[cfg(feature = "patched_http1")]
    #[tokio::test]
    async fn write_invalid_path() {
        let wire = b"GET /\x01\xF0\x90\x80 HTTP/1.1\r\nFoo: Bar\r\n\r\n";
        let mock_io = Builder::new().write(wire).build();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 976: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 976)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_informational() {
        init_log();
        let input1 = b"HTTP/1.1 100 Continue\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1008: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 1008)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn init_body_for_upgraded_req() {
        use crate::protocols::http::v1::body::BodyMode;

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1028: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 1028)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_switching_protocol() {
        init_log();
        let input1 = b"HTTP/1.1 101 Continue\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1074: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 1074)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    // reserved
    #[tokio::test]
    async fn read_obsolete_multiline_headers() {
        init_log();
        let input = b"HTTP/1.1 200 OK\r\nServer : pingora\r\n Foo: Bar\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1102: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 1102)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    #[cfg(feature = "patched_http1")]
    #[tokio::test]
    async fn read_headers_skip_invalid_line() {
        init_log();
        let input = b"HTTP/1.1 200 OK\r\n;\r\nFoo: Bar\r\n\r\n";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1114: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 1114)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn read_keepalive_headers() {
        init_log();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1264: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 1264)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod test_sync {
    use super::*;
    use log::error;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1269: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 1269)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_request_to_wire() {
        let mut new_request = RequestHeader::build("GET", b"/", None).unwrap();
        new_request.insert_header("Foo", "Bar").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `build_resp_with_keepalive_values()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 1200)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
        );

        async fn build_resp_with_keepalive_values(keep_alive: &str) -> HttpSession {
            let input = format!("HTTP/1.1 200 OK\r\nKeep-Alive: {keep_alive}\r\n\r\n");
            let mock_io = Builder::new().read(input.as_bytes()).build();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `build_resp_with_keepalive()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/v1/client.rs` (line 1117)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
        init_log();

        async fn build_resp_with_keepalive(conn: &str) -> HttpSession {
            let input = format!("HTTP/1.1 200 OK\r\nConnection: {conn}\r\n\r\n");
            let mock_io = Builder::new().read(input.as_bytes()).build();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym