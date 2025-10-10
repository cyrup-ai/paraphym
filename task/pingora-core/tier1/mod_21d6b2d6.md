# `forks/pingora/pingora-core/src/protocols/http/compression/mod.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-core
- **File Hash**: 21d6b2d6  
- **Timestamp**: 2025-10-10T02:16:01.207053+00:00  
- **Lines of Code**: 649

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 649 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 227
  - TODO
  - 

```rust
                if resp.status.is_informational() {
                    if resp.status == http::status::StatusCode::SWITCHING_PROTOCOLS {
                        // no transformation for websocket (TODO: cite RFC)
                        self.0 = CtxInner::BodyPhase(None);
                    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 302
  - TODO
  - 

```rust
    }

    // TODO: retire this function, replace it with the two functions above
    /// Feed the response into this ctx.
    /// This filter will mutate the response accordingly if encoding is needed.
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 336
  - TODO
  - 

```rust
    Brotli,
    Zstd,
    // TODO: Identity,
    // TODO: Deflate
    Other, // anything unknown
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 337
  - TODO
  - 

```rust
    Zstd,
    // TODO: Identity,
    // TODO: Deflate
    Other, // anything unknown
}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 423
  - TODO
  - 

```rust
                    if let sfv::ListEntry::Item(i) = item {
                        if let Some(s) = i.bare_item.as_token() {
                            // TODO: support q value
                            let algorithm = Algorithm::from(s);
                            // ignore algorithms that we don't understand ignore
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 539
  - TODO
  - 

```rust
            // https://www.rfc-editor.org/rfc/rfc9110#field.accept-encoding
            // "If the representation has no content coding, then it is acceptable by default
            // unless specifically excluded..." TODO: check the exclude case
            // TODO: we could also transcode it to a preferred encoding, e.g. br->gzip
            Action::Decompress(ce)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 540
  - TODO
  - 

```rust
            // "If the representation has no content coding, then it is acceptable by default
            // unless specifically excluded..." TODO: check the exclude case
            // TODO: we could also transcode it to a preferred encoding, e.g. br->gzip
            Action::Decompress(ce)
        }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 550
  - TODO
  - 

```rust
    } else {
        // try to compress with the first AC
        // TODO: support to configure preferred encoding
        Action::Compress(accept_encoding[0])
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 623
  - TODO
  - 

```rust

// Allow text, application, font, a few image/ MIME types and binary/octet-stream
// TODO: fine tune this list
static MIME_CHECK: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?:text/|application/|font/|image/(?:x-icon|svg\+xml|nd\.microsoft\.icon)|binary/octet-stream)")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 764
  - TODO
  - 

```rust
        resp.remove_header(&ACCEPT_RANGES);

        // we stream body now TODO: chunked is for h1 only
        resp.insert_header(&TRANSFER_ENCODING, HeaderValue::from_static("chunked"))
            .unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 445: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
#[test]
fn test_accept_encoding_req_header() {
    let mut header = RequestHeader::build("GET", b"/", None).unwrap();
    let mut ac_list = Vec::new();
    parse_accept_encoding(
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

    let mut ac_list = Vec::new();
    header.insert_header("accept-encoding", "gzip").unwrap();
    parse_accept_encoding(
        header.headers.get(http::header::ACCEPT_ENCODING),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 464: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    header
        .insert_header("accept-encoding", "what, br, gzip")
        .unwrap();
    parse_accept_encoding(
        header.headers.get(http::header::ACCEPT_ENCODING),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 488: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
#[test]
fn test_decide_on_accept_encoding() {
    let mut resp = ResponseHeader::build(200, None).unwrap();
    resp.insert_header("content-length", "50").unwrap();
    resp.insert_header("content-type", "text/html").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 489: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
fn test_decide_on_accept_encoding() {
    let mut resp = ResponseHeader::build(200, None).unwrap();
    resp.insert_header("content-length", "50").unwrap();
    resp.insert_header("content-type", "text/html").unwrap();
    resp.insert_header("content-encoding", "gzip").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 490: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    let mut resp = ResponseHeader::build(200, None).unwrap();
    resp.insert_header("content-length", "50").unwrap();
    resp.insert_header("content-type", "text/html").unwrap();
    resp.insert_header("content-encoding", "gzip").unwrap();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 491: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    resp.insert_header("content-length", "50").unwrap();
    resp.insert_header("content-type", "text/html").unwrap();
    resp.insert_header("content-encoding", "gzip").unwrap();

    // enabled
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 510: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

    // compress enabled and not compressible response => disabled
    resp.insert_header("content-type", "text/html+zip").unwrap();
    assert!(!depends_on_accept_encoding(&resp, true, &[false]));
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 560: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    use Algorithm::*;

    let header = ResponseHeader::build(200, None).unwrap();
    // no compression asked, no compression needed
    assert_eq!(decide_action(&header, &[]), Noop);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 565: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

    // already gzip, no compression needed
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-type", "text/html").unwrap();
    header.insert_header("content-encoding", "gzip").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 566: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    // already gzip, no compression needed
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-type", "text/html").unwrap();
    header.insert_header("content-encoding", "gzip").unwrap();
    assert_eq!(decide_action(&header, &[Gzip]), Noop);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 567: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-type", "text/html").unwrap();
    header.insert_header("content-encoding", "gzip").unwrap();
    assert_eq!(decide_action(&header, &[Gzip]), Noop);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 571: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

    // already gzip, no compression needed, upper case
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-encoding", "GzIp").unwrap();
    header.insert_header("content-type", "text/html").unwrap();
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
- **Issue**: Can panic in production code

```rust
    // already gzip, no compression needed, upper case
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-encoding", "GzIp").unwrap();
    header.insert_header("content-type", "text/html").unwrap();
    assert_eq!(decide_action(&header, &[Gzip]), Noop);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 573: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-encoding", "GzIp").unwrap();
    header.insert_header("content-type", "text/html").unwrap();
    assert_eq!(decide_action(&header, &[Gzip]), Noop);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 578: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    // no encoding, compression needed, accepted content-type, large enough
    // Will compress
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "20").unwrap();
    header.insert_header("content-type", "text/html").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 579: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    // Will compress
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "20").unwrap();
    header.insert_header("content-type", "text/html").unwrap();
    assert_eq!(decide_action(&header, &[Gzip]), Compress(Gzip));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 580: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "20").unwrap();
    header.insert_header("content-type", "text/html").unwrap();
    assert_eq!(decide_action(&header, &[Gzip]), Compress(Gzip));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 584: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

    // too small
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "19").unwrap();
    header.insert_header("content-type", "text/html").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 585: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    // too small
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "19").unwrap();
    header.insert_header("content-type", "text/html").unwrap();
    assert_eq!(decide_action(&header, &[Gzip]), Noop);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 586: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "19").unwrap();
    header.insert_header("content-type", "text/html").unwrap();
    assert_eq!(decide_action(&header, &[Gzip]), Noop);

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
- **Issue**: Can panic in production code

```rust

    // already compressed MIME
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "20").unwrap();
    header
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 591: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    // already compressed MIME
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "20").unwrap();
    header
        .insert_header("content-type", "text/html+zip")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 594: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    header
        .insert_header("content-type", "text/html+zip")
        .unwrap();
    assert_eq!(decide_action(&header, &[Gzip]), Noop);

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
- **Issue**: Can panic in production code

```rust

    // unsupported MIME
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "20").unwrap();
    header.insert_header("content-type", "image/jpg").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 599: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    // unsupported MIME
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "20").unwrap();
    header.insert_header("content-type", "image/jpg").unwrap();
    assert_eq!(decide_action(&header, &[Gzip]), Noop);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 600: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "20").unwrap();
    header.insert_header("content-type", "image/jpg").unwrap();
    assert_eq!(decide_action(&header, &[Gzip]), Noop);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 604: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

    // compressed, need decompress
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-encoding", "gzip").unwrap();
    assert_eq!(decide_action(&header, &[]), Decompress(Gzip));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 605: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    // compressed, need decompress
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-encoding", "gzip").unwrap();
    assert_eq!(decide_action(&header, &[]), Decompress(Gzip));

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
- **Issue**: Can panic in production code

```rust

    // accept-encoding different, need decompress
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-encoding", "gzip").unwrap();
    assert_eq!(decide_action(&header, &[Brotli]), Decompress(Gzip));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 610: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    // accept-encoding different, need decompress
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-encoding", "gzip").unwrap();
    assert_eq!(decide_action(&header, &[Brotli]), Decompress(Gzip));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 614: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

    // less preferred but no need to decompress
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-encoding", "gzip").unwrap();
    assert_eq!(decide_action(&header, &[Brotli, Gzip]), Noop);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 615: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    // less preferred but no need to decompress
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-encoding", "gzip").unwrap();
    assert_eq!(decide_action(&header, &[Brotli, Gzip]), Noop);
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 626: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
static MIME_CHECK: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?:text/|application/|font/|image/(?:x-icon|svg\+xml|nd\.microsoft\.icon)|binary/octet-stream)")
        .unwrap()
});

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 697: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    if !already_present {
        resp.append_header(&VARY, HeaderValue::from_name(value.clone()))
            .unwrap();
    }
}
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 703: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
#[test]
fn test_add_vary_header() {
    let mut header = ResponseHeader::build(200, None).unwrap();
    add_vary_header(&mut header, &http::header::ACCEPT_ENCODING);
    assert_eq!(
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
- **Issue**: Can panic in production code

```rust
    );

    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("Vary", "Accept-Language").unwrap();
    add_vary_header(&mut header, &http::header::ACCEPT_ENCODING);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 715: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("Vary", "Accept-Language").unwrap();
    add_vary_header(&mut header, &http::header::ACCEPT_ENCODING);
    assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 726: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    );

    let mut header = ResponseHeader::build(200, None).unwrap();
    header
        .insert_header("Vary", "Accept-Language, Accept-Encoding")
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
- **Issue**: Can panic in production code

```rust
    header
        .insert_header("Vary", "Accept-Language, Accept-Encoding")
        .unwrap();
    add_vary_header(&mut header, &http::header::ACCEPT_ENCODING);
    assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 740: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    );

    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("Vary", "*").unwrap();
    add_vary_header(&mut header, &http::header::ACCEPT_ENCODING);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 741: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("Vary", "*").unwrap();
    add_vary_header(&mut header, &http::header::ACCEPT_ENCODING);
    assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 766: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // we stream body now TODO: chunked is for h1 only
        resp.insert_header(&TRANSFER_ENCODING, HeaderValue::from_static("chunked"))
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


### Line 807: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        Action::Compress(a) => {
            resp.insert_header(&CONTENT_ENCODING, HeaderValue::from_static(a.as_str()))
                .unwrap();
            set_stream_headers(resp);
            if !preserve_etag {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 822: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

    // noop
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "20").unwrap();
    header.insert_header("content-encoding", "gzip").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 823: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    // noop
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "20").unwrap();
    header.insert_header("content-encoding", "gzip").unwrap();
    header.insert_header("accept-ranges", "bytes").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 824: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "20").unwrap();
    header.insert_header("content-encoding", "gzip").unwrap();
    header.insert_header("accept-ranges", "bytes").unwrap();
    header.insert_header("etag", "\"abc123\"").unwrap();
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
    header.insert_header("content-length", "20").unwrap();
    header.insert_header("content-encoding", "gzip").unwrap();
    header.insert_header("accept-ranges", "bytes").unwrap();
    header.insert_header("etag", "\"abc123\"").unwrap();
    adjust_response_header(&mut header, &Noop, false);
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
- **Issue**: Can panic in production code

```rust
    header.insert_header("content-encoding", "gzip").unwrap();
    header.insert_header("accept-ranges", "bytes").unwrap();
    header.insert_header("etag", "\"abc123\"").unwrap();
    adjust_response_header(&mut header, &Noop, false);
    assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 843: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

    // decompress gzip
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "20").unwrap();
    header.insert_header("content-encoding", "gzip").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 844: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    // decompress gzip
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "20").unwrap();
    header.insert_header("content-encoding", "gzip").unwrap();
    header.insert_header("accept-ranges", "bytes").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 845: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "20").unwrap();
    header.insert_header("content-encoding", "gzip").unwrap();
    header.insert_header("accept-ranges", "bytes").unwrap();
    header.insert_header("etag", "\"abc123\"").unwrap();
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
- **Issue**: Can panic in production code

```rust
    header.insert_header("content-length", "20").unwrap();
    header.insert_header("content-encoding", "gzip").unwrap();
    header.insert_header("accept-ranges", "bytes").unwrap();
    header.insert_header("etag", "\"abc123\"").unwrap();
    adjust_response_header(&mut header, &Decompress(Gzip), false);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 847: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    header.insert_header("content-encoding", "gzip").unwrap();
    header.insert_header("accept-ranges", "bytes").unwrap();
    header.insert_header("etag", "\"abc123\"").unwrap();
    adjust_response_header(&mut header, &Decompress(Gzip), false);
    assert!(header.headers.get("content-encoding").is_none());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 861: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    );
    // when preserve_etag on, strong etag is kept
    header.insert_header("etag", "\"abc123\"").unwrap();
    adjust_response_header(&mut header, &Decompress(Gzip), true);
    assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 869: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

    // compress
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "20").unwrap();
    header.insert_header("accept-ranges", "bytes").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 870: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    // compress
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "20").unwrap();
    header.insert_header("accept-ranges", "bytes").unwrap();
    // try invalid etag, should be cleared
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 871: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    let mut header = ResponseHeader::build(200, None).unwrap();
    header.insert_header("content-length", "20").unwrap();
    header.insert_header("accept-ranges", "bytes").unwrap();
    // try invalid etag, should be cleared
    header.insert_header("etag", "abc123").unwrap();
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
- **Issue**: Can panic in production code

```rust
    header.insert_header("accept-ranges", "bytes").unwrap();
    // try invalid etag, should be cleared
    header.insert_header("etag", "abc123").unwrap();
    adjust_response_header(&mut header, &Compress(Gzip), false);
    assert_eq!(
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
    assert!(header.headers.get("etag").is_none());
    // when preserve_etag on, etag is kept
    header.insert_header("etag", "abc123").unwrap();
    adjust_response_header(&mut header, &Compress(Gzip), true);
    assert_eq!(header.headers.get("etag").unwrap().as_bytes(), b"abc123");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 784: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                // strong ETag, weaken since we are changing the byte representation
                let weakened_etag = HeaderValue::from_bytes(&[b"W/", etag_bytes].concat())
                    .expect("valid header value prefixed with \"W/\" should remain valid");
                resp.insert_header(&ETAG, weakened_etag)
                    .expect("can insert weakened etag when etag was already valid");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 786: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                    .expect("valid header value prefixed with \"W/\" should remain valid");
                resp.insert_header(&ETAG, weakened_etag)
                    .expect("can insert weakened etag when etag was already valid");
            } else {
                // invalid strong ETag, just clear it
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 444: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/compression/mod.rs` (line 444)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn test_accept_encoding_req_header() {
    let mut header = RequestHeader::build("GET", b"/", None).unwrap();
    let mut ac_list = Vec::new();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 487: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/compression/mod.rs` (line 487)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn test_decide_on_accept_encoding() {
    let mut resp = ResponseHeader::build(200, None).unwrap();
    resp.insert_header("content-length", "50").unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 556: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/compression/mod.rs` (line 556)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn test_decide_action() {
    use Action::*;
    use Algorithm::*;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 702: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/compression/mod.rs` (line 702)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn test_add_vary_header() {
    let mut header = ResponseHeader::build(200, None).unwrap();
    add_vary_header(&mut header, &http::header::ACCEPT_ENCODING);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 817: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-core/src/protocols/http/compression/mod.rs` (line 817)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[test]
fn test_adjust_response_header() {
    use Action::*;
    use Algorithm::*;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym