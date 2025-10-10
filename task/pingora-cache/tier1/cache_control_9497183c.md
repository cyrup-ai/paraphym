# `forks/pingora/pingora-cache/src/cache_control.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-cache
- **File Hash**: 9497183c  
- **Timestamp**: 2025-10-10T02:16:01.411651+00:00  
- **Lines of Code**: 601

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 601 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 253
  - hardcoded IP address
  - 

```rust
    // whereas it MAY store the remainder of the response."
    // It must be a boolean form (no value) to apply to the whole response.
    // https://datatracker.ietf.org/doc/html/rfc7234#section-5.2.2.6
    pub fn private(&self) -> bool {
        self.has_key_without_value("private")
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 401
  - hardcoded IP address
  - 

```rust
        // ("MUST NOT be sent in the response to a subsequent request
        // without successful revalidation with the origin server").
        // https://datatracker.ietf.org/doc/html/rfc7234#section-5.2.2.2
        if let Some(headers) = self.no_cache_field_names() {
            strip_listed_headers(resp_header, headers);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 409
  - actual
  - 

```rust

/// `InterpretCacheControl` provides a meaningful interface to the parsed `CacheControl`.
/// These functions actually interpret the parsed cache-control directives to return
/// the freshness or other cache meta values that cache-control is signaling.
///
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 167: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    // `(?:=((?:[^\x00-\x20\(\)<>@,;:\\"/\[\]\?=\{\}\x7F]+|(?:"(?:[^"\\]|\\.)*"))))`: token OR quoted-string (directive value capture-group)
    Lazy::new(|| {
        Regex::new(r#"(?-u)(?:^|(?:\s*[,;]\s*))([^\x00-\x20\(\)<>@,;:\\"/\[\]\?=\{\}\x7F]+)(?:=((?:[^\x00-\x20\(\)<>@,;:\\"/\[\]\?=\{\}\x7F]+|(?:"(?:[^"\\]|\\.)*"))))?"#).unwrap()
    });

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
                    .get(2)
                    .map(|cap| DirectiveValue(cap.as_bytes().to_vec()));
                directives.insert(key.unwrap(), value);
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


### Line 467: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .header(cc_key, cc_value)
            .body(())
            .unwrap()
            .into_parts();
        parts
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
- **Issue**: Can panic in production code

```rust
    fn test_simple_cache_control() {
        let resp = build_response(CACHE_CONTROL, "public, max-age=10000");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert!(cc.public());
        assert_eq!(cc.max_age().unwrap().unwrap(), 10000);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 483: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn test_private_cache_control() {
        let resp = build_response(CACHE_CONTROL, "private");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();

        assert!(cc.private());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 495: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .header("cache-Control", "max-age=10000")
            .body(())
            .unwrap()
            .into_parts();
        let cc = CacheControl::from_resp_headers(&parts).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 497: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .unwrap()
            .into_parts();
        let cc = CacheControl::from_resp_headers(&parts).unwrap();

        assert!(cc.public());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 506: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn test_recognizes_semicolons_as_delimiters() {
        let resp = build_response(CACHE_CONTROL, "public; max-age=0");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();

        assert!(cc.public());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 515: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn test_unknown_directives() {
        let resp = build_response(CACHE_CONTROL, "public,random1=random2, rand3=\"\"");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        let mut directive_iter = cc.directives.iter();

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
        let mut directive_iter = cc.directives.iter();

        let first = directive_iter.next().unwrap();
        assert_eq!(first.0, &"public");
        assert!(first.1.is_none());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 522: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert!(first.1.is_none());

        let second = directive_iter.next().unwrap();
        assert_eq!(second.0, &"random1");
        assert_eq!(second.1.as_ref().unwrap().0, "random2".as_bytes());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 526: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(second.1.as_ref().unwrap().0, "random2".as_bytes());

        let third = directive_iter.next().unwrap();
        assert_eq!(third.0, &"rand3");
        assert_eq!(third.1.as_ref().unwrap().0, "\"\"".as_bytes());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 539: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            "Public=\"something\", mAx-AGe=\"10000\", foo=cRaZyCaSe, bAr=\"inQuotes\"",
        );
        let cc = CacheControl::from_resp_headers(&resp).unwrap();

        assert!(cc.public());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 545: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let mut directive_iter = cc.directives.iter();
        let first = directive_iter.next().unwrap();
        assert_eq!(first.0, &"public");
        assert_eq!(first.1.as_ref().unwrap().0, "\"something\"".as_bytes());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 549: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(first.1.as_ref().unwrap().0, "\"something\"".as_bytes());

        let second = directive_iter.next().unwrap();
        assert_eq!(second.0, &"max-age");
        assert_eq!(second.1.as_ref().unwrap().0, "\"10000\"".as_bytes());
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
- **Issue**: Can panic in production code

```rust

        // values are still stored with casing
        let third = directive_iter.next().unwrap();
        assert_eq!(third.0, &"foo");
        assert_eq!(third.1.as_ref().unwrap().0, "cRaZyCaSe".as_bytes());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 558: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(third.1.as_ref().unwrap().0, "cRaZyCaSe".as_bytes());

        let fourth = directive_iter.next().unwrap();
        assert_eq!(fourth.0, &"bar");
        assert_eq!(fourth.1.as_ref().unwrap().0, "\"inQuotes\"".as_bytes());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 568: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn test_non_ascii() {
        let resp = build_response(CACHE_CONTROL, "püblic=💖, max-age=\"💯\"");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();

        // Not considered valid registered directive keys / values
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

        let mut directive_iter = cc.directives.iter();
        let first = directive_iter.next().unwrap();
        assert_eq!(first.0, &"püblic");
        assert_eq!(first.1.as_ref().unwrap().0, "💖".as_bytes());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 582: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(first.1.as_ref().unwrap().0, "💖".as_bytes());

        let second = directive_iter.next().unwrap();
        assert_eq!(second.0, &"max-age");
        assert_eq!(second.1.as_ref().unwrap().0, "\"💯\"".as_bytes());
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
    #[test]
    fn test_non_utf8_key() {
        let mut resp = response::Builder::new().body(()).unwrap();
        resp.headers_mut().insert(
            CACHE_CONTROL,
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
        resp.headers_mut().insert(
            CACHE_CONTROL,
            HeaderValue::from_bytes(b"bar\xFF=\"baz\", a=b").unwrap(),
        );
        let (parts, _) = resp.into_parts();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 597: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        );
        let (parts, _) = resp.into_parts();
        let cc = CacheControl::from_resp_headers(&parts).unwrap();

        // invalid bytes for key
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
- **Issue**: Can panic in production code

```rust
        // invalid bytes for key
        let mut directive_iter = cc.directives.iter();
        let first = directive_iter.next().unwrap();
        assert_eq!(first.0, &"a");
        assert_eq!(first.1.as_ref().unwrap().0, "b".as_bytes());
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
- **Issue**: Can panic in production code

```rust
    fn test_non_utf8_value() {
        // RFC 7230: 0xFF is part of obs-text and is officially considered a valid octet in quoted-strings
        let mut resp = response::Builder::new().body(()).unwrap();
        resp.headers_mut().insert(
            CACHE_CONTROL,
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
        resp.headers_mut().insert(
            CACHE_CONTROL,
            HeaderValue::from_bytes(b"max-age=ba\xFFr, bar=\"baz\xFF\", a=b").unwrap(),
        );
        let (parts, _) = resp.into_parts();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 617: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        );
        let (parts, _) = resp.into_parts();
        let cc = CacheControl::from_resp_headers(&parts).unwrap();

        assert_eq!(
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
        let mut directive_iter = cc.directives.iter();

        let first = directive_iter.next().unwrap();
        assert_eq!(first.0, &"max-age");
        assert_eq!(first.1.as_ref().unwrap().0, b"ba\xFFr");
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
- **Issue**: Can panic in production code

```rust
        assert_eq!(first.1.as_ref().unwrap().0, b"ba\xFFr");

        let second = directive_iter.next().unwrap();
        assert_eq!(second.0, &"bar");
        assert_eq!(second.1.as_ref().unwrap().0, b"\"baz\xFF\"");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 634: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(second.1.as_ref().unwrap().0, b"\"baz\xFF\"");

        let third = directive_iter.next().unwrap();
        assert_eq!(third.0, &"a");
        assert_eq!(third.1.as_ref().unwrap().0, "b".as_bytes());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 647: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            "max-age=-99999999999999999999999999, s-maxage=99999999999999999999999999",
        );
        let cc = CacheControl::from_resp_headers(&resp).unwrap();

        assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 663: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn test_fresh_sec() {
        let resp = build_response(CACHE_CONTROL, "");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert!(cc.fresh_duration().is_none());

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
- **Issue**: Can panic in production code

```rust

        let resp = build_response(CACHE_CONTROL, "max-age=12345");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert_eq!(cc.fresh_duration().unwrap(), Duration::from_secs(12345));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 671: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let resp = build_response(CACHE_CONTROL, "max-age=99999,s-maxage=123");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        // prefer s-maxage over max-age
        assert_eq!(cc.fresh_duration().unwrap(), Duration::from_secs(123));
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
- **Issue**: Can panic in production code

```rust
    fn test_cacheability() {
        let resp = build_response(CACHE_CONTROL, "");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert_eq!(cc.is_cacheable(), Cacheable::Default);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 684: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // uncacheable
        let resp = build_response(CACHE_CONTROL, "private, max-age=12345");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert_eq!(cc.is_cacheable(), Cacheable::No);

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
- **Issue**: Can panic in production code

```rust

        let resp = build_response(CACHE_CONTROL, "no-store, max-age=12345");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert_eq!(cc.is_cacheable(), Cacheable::No);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 693: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // cacheable
        let resp = build_response(CACHE_CONTROL, "public");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert_eq!(cc.is_cacheable(), Cacheable::Yes);

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

        let resp = build_response(CACHE_CONTROL, "max-age=0");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert_eq!(cc.is_cacheable(), Cacheable::Yes);
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 704: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn test_no_cache() {
        let resp = build_response(CACHE_CONTROL, "no-cache, max-age=12345");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert_eq!(cc.is_cacheable(), Cacheable::Yes);
        assert_eq!(cc.fresh_duration().unwrap(), Duration::ZERO);
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
- **Issue**: Can panic in production code

```rust
    fn test_no_cache_field_names() {
        let resp = build_response(CACHE_CONTROL, "no-cache=\"set-cookie\", max-age=12345");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert!(!cc.private());
        assert_eq!(cc.is_cacheable(), Cacheable::Yes);
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
- **Issue**: Can panic in production code

```rust
        assert_eq!(cc.is_cacheable(), Cacheable::Yes);
        assert_eq!(cc.fresh_duration().unwrap(), Duration::from_secs(12345));
        let mut field_names = cc.no_cache_field_names().unwrap();
        assert_eq!(
            str::from_utf8(field_names.next().unwrap()).unwrap(),
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
- **Issue**: Can panic in production code

```rust
        assert!(field_names.next().is_none());

        let mut resp = response::Builder::new().body(()).unwrap();
        resp.headers_mut().insert(
            CACHE_CONTROL,
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
                b"private=\"\", no-cache=\"a\xFF, set-cookie, Baz\x09 , c,d  ,, \"",
            )
            .unwrap(),
        );
        let (parts, _) = resp.into_parts();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 732: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        );
        let (parts, _) = resp.into_parts();
        let cc = CacheControl::from_resp_headers(&parts).unwrap();
        let mut field_names = cc.private_field_names().unwrap();
        assert_eq!(str::from_utf8(field_names.next().unwrap()).unwrap(), "");
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
- **Issue**: Can panic in production code

```rust
        let (parts, _) = resp.into_parts();
        let cc = CacheControl::from_resp_headers(&parts).unwrap();
        let mut field_names = cc.private_field_names().unwrap();
        assert_eq!(str::from_utf8(field_names.next().unwrap()).unwrap(), "");
        assert!(field_names.next().is_none());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 736: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_eq!(str::from_utf8(field_names.next().unwrap()).unwrap(), "");
        assert!(field_names.next().is_none());
        let mut field_names = cc.no_cache_field_names().unwrap();
        assert!(str::from_utf8(field_names.next().unwrap()).is_err());
        assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 752: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    #[test]
    fn test_strip_private_headers() {
        let mut resp = ResponseHeader::build(200, None).unwrap();
        resp.append_header(
            CACHE_CONTROL,
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
- **Issue**: Can panic in production code

```rust
            "no-cache=\"x-private-header\", max-age=12345",
        )
        .unwrap();
        resp.append_header("X-Private-Header", "dropped").unwrap();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 758: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        )
        .unwrap();
        resp.append_header("X-Private-Header", "dropped").unwrap();

        let cc = CacheControl::from_resp_headers(&resp).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 760: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        resp.append_header("X-Private-Header", "dropped").unwrap();

        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        cc.strip_private_headers(&mut resp);
        assert!(!resp.headers.contains_key("X-Private-Header"));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 768: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn test_stale_while_revalidate() {
        let resp = build_response(CACHE_CONTROL, "max-age=12345, stale-while-revalidate=5");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert_eq!(cc.stale_while_revalidate().unwrap().unwrap(), 5);
        assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 780: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn test_stale_if_error() {
        let resp = build_response(CACHE_CONTROL, "max-age=12345, stale-if-error=3600");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert_eq!(cc.stale_if_error().unwrap().unwrap(), 3600);
        assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 795: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            "max-age=12345, stale-while-revalidate=60, stale-if-error=30, must-revalidate",
        );
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert!(cc.must_revalidate());
        assert_eq!(cc.stale_while_revalidate().unwrap().unwrap(), 60);
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
- **Issue**: Can panic in production code

```rust
            "max-age=12345, stale-while-revalidate=60, stale-if-error=30, proxy-revalidate",
        );
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert!(cc.proxy_revalidate());
        assert_eq!(cc.stale_while_revalidate().unwrap().unwrap(), 60);
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
- **Issue**: Can panic in production code

```rust
            "s-maxage=0, stale-while-revalidate=60, stale-if-error=30",
        );
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert_eq!(cc.stale_while_revalidate().unwrap().unwrap(), 60);
        assert_eq!(cc.stale_if_error().unwrap().unwrap(), 30);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 842: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn test_authorized_request() {
        let resp = build_response(CACHE_CONTROL, "max-age=10");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert!(!cc.allow_caching_authorized_req());

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

        let resp = build_response(CACHE_CONTROL, "s-maxage=10");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert!(cc.allow_caching_authorized_req());

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 850: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let resp = build_response(CACHE_CONTROL, "public");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert!(cc.allow_caching_authorized_req());

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

        let resp = build_response(CACHE_CONTROL, "must-revalidate, max-age=0");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert!(cc.allow_caching_authorized_req());

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 858: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let resp = build_response(CACHE_CONTROL, "");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
        assert!(!cc.allow_caching_authorized_req());
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 866: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .header(cc_key, cc_value)
            .body(())
            .unwrap()
            .into_parts();
        parts
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 874: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn test_request_only_if_cached() {
        let req = build_request(CACHE_CONTROL, "only-if-cached=1");
        let cc = CacheControl::from_req_headers(&req).unwrap();
        assert!(cc.only_if_cached())
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


### Line 458: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 458)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use super::*;
    use http::header::CACHE_CONTROL;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 473: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 473)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_simple_cache_control() {
        let resp = build_response(CACHE_CONTROL, "public, max-age=10000");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 481: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 481)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_private_cache_control() {
        let resp = build_response(CACHE_CONTROL, "private");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 490: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 490)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_directives_across_header_lines() {
        let (parts, _) = response::Builder::new()
            .header(CACHE_CONTROL, "public,")
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 504: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 504)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_recognizes_semicolons_as_delimiters() {
        let resp = build_response(CACHE_CONTROL, "public; max-age=0");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 513: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 513)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_unknown_directives() {
        let resp = build_response(CACHE_CONTROL, "public,random1=random2, rand3=\"\"");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 534: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 534)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_case_insensitive_directive_keys() {
        let resp = build_response(
            CACHE_CONTROL,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 566: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 566)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_non_ascii() {
        let resp = build_response(CACHE_CONTROL, "püblic=💖, max-age=\"💯\"");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 590: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 590)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_non_utf8_key() {
        let mut resp = response::Builder::new().body(()).unwrap();
        resp.headers_mut().insert(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 609: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 609)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_non_utf8_value() {
        // RFC 7230: 0xFF is part of obs-text and is officially considered a valid octet in quoted-strings
        let mut resp = response::Builder::new().body(()).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 642: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 642)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_age_overflow() {
        let resp = build_response(
            CACHE_CONTROL,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 661: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 661)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_fresh_sec() {
        let resp = build_response(CACHE_CONTROL, "");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 677: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 677)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_cacheability() {
        let resp = build_response(CACHE_CONTROL, "");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 702: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 702)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_no_cache() {
        let resp = build_response(CACHE_CONTROL, "no-cache, max-age=12345");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 710: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 710)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_no_cache_field_names() {
        let resp = build_response(CACHE_CONTROL, "no-cache=\"set-cookie\", max-age=12345");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 751: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 751)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_strip_private_headers() {
        let mut resp = ResponseHeader::build(200, None).unwrap();
        resp.append_header(
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 766: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 766)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_stale_while_revalidate() {
        let resp = build_response(CACHE_CONTROL, "max-age=12345, stale-while-revalidate=5");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 778: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 778)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_stale_if_error() {
        let resp = build_response(CACHE_CONTROL, "max-age=12345, stale-if-error=3600");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 790: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 790)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_must_revalidate() {
        let resp = build_response(
            CACHE_CONTROL,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 807: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 807)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_proxy_revalidate() {
        let resp = build_response(
            CACHE_CONTROL,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 824: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 824)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_s_maxage_stale() {
        let resp = build_response(
            CACHE_CONTROL,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 840: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 840)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_authorized_request() {
        let resp = build_response(CACHE_CONTROL, "max-age=10");
        let cc = CacheControl::from_resp_headers(&resp).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 872: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 872)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_request_only_if_cached() {
        let req = build_request(CACHE_CONTROL, "only-if-cached=1");
        let cc = CacheControl::from_req_headers(&req).unwrap();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `not_ows()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/cache_control.rs` (line 130)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
// optional whitespace OWS = *(SP / HTAB); SP = 0x20, HTAB = 0x09
fn trim_ows(bytes: &[u8]) -> &[u8] {
    fn not_ows(b: &u8) -> bool {
        b != &b'\x20' && b != &b'\x09'
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