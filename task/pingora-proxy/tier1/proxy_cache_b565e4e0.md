# `forks/pingora/pingora-proxy/src/proxy_cache.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-proxy
- **File Hash**: b565e4e0  
- **Timestamp**: 2025-10-10T02:16:01.366596+00:00  
- **Lines of Code**: 1657

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 1657 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 42
  - TODO
  - 

```rust
        // Cache logic request phase
        if let Err(e) = self.inner.request_cache_filter(session, ctx) {
            // TODO: handle this error
            warn!(
                "Fail to request_cache_filter: {e}, {}",
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 56
  - TODO
  - 

```rust
                }
                Err(e) => {
                    // TODO: handle this error
                    session.cache.disable(NoCacheReason::StorageError);
                    warn!(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 82
  - TODO
  - 

```rust
        // cache lookup logic
        loop {
            // for cache lock, TODO: cap the max number of loops
            match session.cache.cache_lookup().await {
                Ok(res) => {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 117
  - TODO
  - 

```rust

                        // hit
                        // TODO: maybe round and/or cache now()
                        let is_fresh = meta.is_fresh(SystemTime::now());
                        // check if we should force expire or force miss
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 246
  - TODO
  - 

```rust
                    // Allow cache miss to fill cache even if cache lookup errors
                    // this is mostly to support backward incompatible metadata update
                    // TODO: check error types
                    // session.cache.disable();
                    self.inner.cache_miss(session, ctx);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 303
  - TODO
  - 

```rust
        let header_only = header_only || matches!(range_type, RangeType::Invalid);

        // TODO: use ProxyUseCache to replace the logic below
        match self.inner.response_filter(session, &mut header, ctx).await {
            Ok(_) => {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 363
  - TODO
  - 

```rust
                }
                RangeType::Multi(_) => {
                    // TODO: seek hit handler for multipart
                    let mut range_filter = RangeBodyFilter::new();
                    range_filter.set(range_type.clone());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 454
  - TODO
  - 

```rust
        SV: ProxyHttp,
    {
        // TODO: range
        let req = session.req_header();

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 481
  - TODO
  - 

```rust
    }

    // TODO: cache upstream header filter to add/remove headers

    pub(crate) async fn cache_http_task(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 501
  - for now
  - 

```rust
            HttpTask::Header(header, end_stream) => {
                // decide if cacheable and create cache meta
                // for now, skip 1xxs (should not affect response cache decisions)
                // However 101 is an exception because it is the final response header
                if header.status.is_informational()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 604
  - TODO
  - 

```rust
                Some(d) => {
                    if session.cache.enabled() {
                        // TODO: do this async
                        // fail if writing the body would exceed the max_file_size_bytes
                        let body_size_allowed =
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 647
  - TODO
  - 

```rust
            }
            HttpTask::Failed(_) => {
                // TODO: handle this failure: delete the temp files?
            }
        }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 693
  - for now
  - 

```rust
                        {
                            Ok(Cacheable(mut meta)) => {
                                // For simplicity, ignore changes to variance over 304 for now.
                                // Note this means upstream can only update variance via 2xx
                                // (expired response).
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 697
  - TODO
  - 

```rust
                                // (expired response).
                                //
                                // TODO: if we choose to respect changing Vary / variance over 304,
                                // then there are a few cases to consider. See `update_variance` in
                                // the `pingora-cache` module.
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 722
  - TODO
  - 

```rust
                                // (downstream may have a different cacheability assessment and could cache the 304)

                                //TODO: log more
                                warn!("Uncacheable {reason:?} 304 received");
                                session.cache.response_became_uncacheable(reason);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 742
  - TODO
  - 

```rust
                        true
                    } else {
                        //TODO: log more
                        warn!("304 received without cached asset, disable caching");
                        let reason = NoCacheReason::Custom("304 on miss");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 750
  - for now
  - 

```rust
                    }
                } else if resp.status.is_server_error() {
                    // stale if error logic, 5xx only for now

                    // this is response header filter, response_written should always be None?
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 847
  - TODO
  - 

```rust
            // the request is uncacheable, go ahead to fetch from the origin
            LockStatus::GiveUp => {
                // TODO: It will be nice for the writer to propagate the real reason
                session.cache.disable(NoCacheReason::CacheLockGiveUp);
                // not cacheable, just go to the origin.
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 954
  - TODO
  - 

```rust
        for _ in ranges_str.split(',') {
            range_count += 1;
            // TODO: make configurable
            const MAX_RANGES: usize = 100;
            if range_count >= MAX_RANGES {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 957
  - for now
  - 

```rust
            const MAX_RANGES: usize = 100;
            if range_count >= MAX_RANGES {
                // If we get more than MAX_RANGES ranges, return None for now to save parsing time
                return RangeType::None;
            }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1009
  - For now
  - 

```rust
                }
            };
            // For now we stick to non-overlapping, ascending ranges for simplicity
            // and parity with nginx
            if range.start < last_range_end {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1313
  - TODO
  - 

```rust
        }

        // TODO: we can also check Accept-Range header from resp. Nginx gives uses the option
        // see proxy_force_ranges

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1362
  - TODO
  - 

```rust
                resp.insert_header(&CONTENT_LENGTH, HeaderValue::from_static("0"))
                    .unwrap();
                // TODO: remove other headers like content-encoding
                resp.remove_header(&CONTENT_TYPE);
                resp.insert_header(&CONTENT_RANGE, format!("bytes */{content_length}"))
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2057
  - TODO
  - 

```rust
        if !cache.enabled() {
            // Cache is disabled due to internal error
            // TODO: if nothing is sent to eyeball yet, figure out a way to recovery by
            // fetching from upstream
            return Error::e_explain(InternalError, "Cache disabled");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2149
  - for now
  - 

```rust
            RangeType::Multi(_) => {
                // For multipart ranges, we will handle the seeking in
                // the body filter per part for now.
                // TODO: implement seek for multipart range
            }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 2150
  - TODO
  - 

```rust
                // For multipart ranges, we will handle the seeking in
                // the body filter per part for now.
                // TODO: implement seek for multipart range
            }
            _ => {}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 207
  - actual
  - 

```rust
                                && self.inner.should_serve_stale(session, ctx, None);
                            if will_serve_stale {
                                // create a background thread to do the actual update
                                // the subrequest handle is only None by this phase in unit tests
                                // that don't go through process_new_http
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 936
  - actual
  - 

```rust
        };

        // Split into "bytes=" and the actual range(s)
        let mut parts = range_str.splitn(2, "=");

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 950
  - actual
  - 

```rust
        };

        // Get the actual range string (e.g."100-200,300-400")
        let mut range_count = 0;
        for _ in ranges_str.split(',') {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1712
  - actual
  - 

```rust
                        ));
                    }
                    // Emit the actual data bytes
                    result.extend_from_slice(&sliced);
                    if self.current_chunk_includes_range_end(range, current, data_len) {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 585: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                                    .cache
                                    .miss_handler()
                                    .unwrap() // safe, it is set above
                                    .write_body(Bytes::new(), true)
                                    .await?;
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
                        // this will panic if more data is sent after we see end_stream
                        // but should be impossible in real world
                        let miss_handler = session.cache.miss_handler().unwrap();

                        miss_handler.write_body(d.clone(), *end_stream).await?;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 700: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                                // then there are a few cases to consider. See `update_variance` in
                                // the `pingora-cache` module.
                                let old_meta = session.cache.maybe_cache_meta().unwrap(); // safe, checked above
                                if let Some(old_variance) = old_meta.variance() {
                                    meta.set_variance(old_variance);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 895: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    if !cache.upstream_used() {
        let age = cache.cache_meta().age().as_secs();
        header.insert_header(http::header::AGE, age).unwrap();
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 906: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        header
            .insert_header(http::header::TRANSFER_ENCODING, "chunked")
            .unwrap();
    }
    header
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 928: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // Match individual range parts, (e.g. "0-100", "-5", "1-")
        static RE_SINGLE_RANGE_PART: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?i)^\s*(?P<start>\d*)-(?P<end>\d*)\s*$").unwrap());

        // Convert bytes to UTF-8 string
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
            RangeType::Single(r) => {
                // 206 response
                resp.set_status(StatusCode::PARTIAL_CONTENT).unwrap();
                resp.insert_header(&CONTENT_LENGTH, r.end - r.start)
                    .unwrap();
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
- **Issue**: Can panic in production code

```rust
                resp.set_status(StatusCode::PARTIAL_CONTENT).unwrap();
                resp.insert_header(&CONTENT_LENGTH, r.end - r.start)
                    .unwrap();
                resp.insert_header(
                    &CONTENT_RANGE,
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
- **Issue**: Can panic in production code

```rust
                    format!("bytes {}-{}/{content_length}", r.start, r.end - 1), // range end is inclusive
                )
                .unwrap()
            }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1344: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                let total_length = multi_range_info.calculate_multipart_length();

                resp.set_status(StatusCode::PARTIAL_CONTENT).unwrap();
                resp.insert_header(CONTENT_LENGTH, total_length).unwrap();
                resp.insert_header(
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
- **Issue**: Can panic in production code

```rust

                resp.set_status(StatusCode::PARTIAL_CONTENT).unwrap();
                resp.insert_header(CONTENT_LENGTH, total_length).unwrap();
                resp.insert_header(
                    CONTENT_TYPE,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1353: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                    ), // RFC 2046
                )
                .unwrap();
                resp.remove_header(&CONTENT_RANGE);
            }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1358: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            RangeType::Invalid => {
                // 416 response
                resp.set_status(StatusCode::RANGE_NOT_SATISFIABLE).unwrap();
                // empty body for simplicity
                resp.insert_header(&CONTENT_LENGTH, HeaderValue::from_static("0"))
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
                // empty body for simplicity
                resp.insert_header(&CONTENT_LENGTH, HeaderValue::from_static("0"))
                    .unwrap();
                // TODO: remove other headers like content-encoding
                resp.remove_header(&CONTENT_TYPE);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1365: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                resp.remove_header(&CONTENT_TYPE);
                resp.insert_header(&CONTENT_RANGE, format!("bytes */{content_length}"))
                    .unwrap()
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


### Line 1375: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn test_range_filter_single() {
        fn gen_req() -> RequestHeader {
            RequestHeader::build(http::Method::GET, b"/", Some(1)).unwrap()
        }
        fn gen_resp() -> ResponseHeader {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1378: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        }
        fn gen_resp() -> ResponseHeader {
            let mut resp = ResponseHeader::build(200, Some(1)).unwrap();
            resp.append_header("Content-Length", "10").unwrap();
            resp
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1379: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        fn gen_resp() -> ResponseHeader {
            let mut resp = ResponseHeader::build(200, Some(1)).unwrap();
            resp.append_header("Content-Length", "10").unwrap();
            resp
        }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1391: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // regular range
        let mut req = gen_req();
        req.insert_header("Range", "bytes=0-1").unwrap();
        let mut resp = gen_resp();
        assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1406: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // bad range
        let mut req = gen_req();
        req.insert_header("Range", "bytes=1-0").unwrap();
        let mut resp = gen_resp();
        assert_eq!(RangeType::Invalid, range_header_filter(&req, &mut resp));
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
        fn gen_req() -> RequestHeader {
            let mut req: RequestHeader =
                RequestHeader::build(http::Method::GET, b"/", Some(1)).unwrap();
            req.append_header("Range", "bytes=0-1,3-4,6-7").unwrap();
            req
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1423: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            let mut req: RequestHeader =
                RequestHeader::build(http::Method::GET, b"/", Some(1)).unwrap();
            req.append_header("Range", "bytes=0-1,3-4,6-7").unwrap();
            req
        }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1428: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        fn gen_req_overlap_range() -> RequestHeader {
            let mut req: RequestHeader =
                RequestHeader::build(http::Method::GET, b"/", Some(1)).unwrap();
            req.append_header("Range", "bytes=0-3,2-5,7-8").unwrap();
            req
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1429: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            let mut req: RequestHeader =
                RequestHeader::build(http::Method::GET, b"/", Some(1)).unwrap();
            req.append_header("Range", "bytes=0-3,2-5,7-8").unwrap();
            req
        }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1433: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        }
        fn gen_resp() -> ResponseHeader {
            let mut resp = ResponseHeader::build(200, Some(1)).unwrap();
            resp.append_header("Content-Length", "10").unwrap();
            resp
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1434: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        fn gen_resp() -> ResponseHeader {
            let mut resp = ResponseHeader::build(200, Some(1)).unwrap();
            resp.append_header("Content-Length", "10").unwrap();
            resp
        }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1476: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut req = gen_req();
        req.insert_header("Range", "bytes=1-0, 12-9, 50-40")
            .unwrap();
        let mut resp = gen_resp();
        let result = range_header_filter(&req, &mut resp);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1489: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        fn gen_req() -> RequestHeader {
            let mut req = RequestHeader::build(http::Method::GET, b"/", Some(1)).unwrap();
            req.append_header("Range", "bytes=0-1").unwrap();
            req
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1490: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        fn gen_req() -> RequestHeader {
            let mut req = RequestHeader::build(http::Method::GET, b"/", Some(1)).unwrap();
            req.append_header("Range", "bytes=0-1").unwrap();
            req
        }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1494: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        }
        fn get_multipart_req() -> RequestHeader {
            let mut req = RequestHeader::build(http::Method::GET, b"/", Some(1)).unwrap();
            _ = req.append_header("Range", "bytes=0-1,3-4,6-7");
            req
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
- **Issue**: Can panic in production code

```rust
        }
        fn gen_resp() -> ResponseHeader {
            let mut resp = ResponseHeader::build(200, Some(1)).unwrap();
            resp.append_header("Content-Length", "10").unwrap();
            resp.append_header("Last-Modified", DATE).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1500: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        fn gen_resp() -> ResponseHeader {
            let mut resp = ResponseHeader::build(200, Some(1)).unwrap();
            resp.append_header("Content-Length", "10").unwrap();
            resp.append_header("Last-Modified", DATE).unwrap();
            resp.append_header("ETag", ETAG).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1501: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            let mut resp = ResponseHeader::build(200, Some(1)).unwrap();
            resp.append_header("Content-Length", "10").unwrap();
            resp.append_header("Last-Modified", DATE).unwrap();
            resp.append_header("ETag", ETAG).unwrap();
            resp
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1502: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            resp.append_header("Content-Length", "10").unwrap();
            resp.append_header("Last-Modified", DATE).unwrap();
            resp.append_header("ETag", ETAG).unwrap();
            resp
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
- **Issue**: Can panic in production code

```rust
        // matching Last-Modified date
        let mut req = gen_req();
        req.insert_header("If-Range", DATE).unwrap();
        let mut resp = gen_resp();
        assert_eq!(
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
        let mut req = gen_req();
        req.insert_header("If-Range", "Fri, 07 Jul 2023 22:03:25 GMT")
            .unwrap();
        let mut resp = gen_resp();
        assert_eq!(RangeType::None, range_header_filter(&req, &mut resp));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1524: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // match ETag
        let mut req = gen_req();
        req.insert_header("If-Range", ETAG).unwrap();
        let mut resp = gen_resp();
        assert_eq!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1533: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // non-matching ETags do not result in range
        let mut req = gen_req();
        req.insert_header("If-Range", "\"4567\"").unwrap();
        let mut resp = gen_resp();
        assert_eq!(RangeType::None, range_header_filter(&req, &mut resp));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1538: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let mut req = gen_req();
        req.insert_header("If-Range", "1234").unwrap();
        let mut resp = gen_resp();
        assert_eq!(RangeType::None, range_header_filter(&req, &mut resp));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1544: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // multipart range with If-Range
        let mut req = get_multipart_req();
        req.insert_header("If-Range", DATE).unwrap();
        let mut resp = gen_resp();
        let result = range_header_filter(&req, &mut resp);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1560: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // multipart with non-matching If-Range
        let mut req = get_multipart_req();
        req.insert_header("If-Range", "\"wrong\"").unwrap();
        let mut resp = gen_resp();
        assert_eq!(RangeType::None, range_header_filter(&req, &mut resp));
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1786: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        // Pass the whole body in one chunk
        let output = body_filter.filter_body(Some(data)).unwrap();
        let footer = body_filter.finalize(&multi_range_info.boundary).unwrap();

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
- **Issue**: Can panic in production code

```rust
        // Pass the whole body in one chunk
        let output = body_filter.filter_body(Some(data)).unwrap();
        let footer = body_filter.finalize(&multi_range_info.boundary).unwrap();

        // Convert to String so that we can inspect whole response
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1790: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        // Convert to String so that we can inspect whole response
        let output_str = str::from_utf8(&output).unwrap();
        let final_boundary = str::from_utf8(&footer).unwrap();
        let boundary = &multi_range_info.boundary;
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
        // Convert to String so that we can inspect whole response
        let output_str = str::from_utf8(&output).unwrap();
        let final_boundary = str::from_utf8(&footer).unwrap();
        let boundary = &multi_range_info.boundary;

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1854: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        }

        let output_str = str::from_utf8(&collected_bytes).unwrap();
        let boundary = multi_range_info.boundary;

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1918: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        }

        let output_str = str::from_utf8(&collected_bytes).unwrap();
        let boundary = &multi_range_info.boundary;

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
- **Issue**: Can panic in production code

```rust
                }
                // safety: called of enable_miss() call it only if the async_body_reader exist
                if let Some(b) = cache.miss_body_reader().unwrap().read_body().await? {
                    Ok(HttpTask::Body(Some(b), false)) // false for now
                } else {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2114: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        if let RangeType::Single(range) = &range_filter.range {
            // safety: called only if the async_body_reader exists
            if cache.miss_body_reader().unwrap().can_seek() {
                cache
                    .miss_body_reader()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2118: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                    .miss_body_reader()
                    // safety: called only if the async_body_reader exists
                    .unwrap()
                    .seek(range.start, Some(range.end))
                    .or_err(InternalError, "cannot seek miss handler")?;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 173: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                    // Safe because an empty hit status would have broken out
                    // in the block above
                    let hit_status = hit_status_opt.expect("None case handled as miss");

                    if !hit_status.is_fresh() {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1694: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            };

            let multipart_idx = self.multipart_idx.expect("must be set on multirange");
            let final_range = multi_part_info.ranges.last()?;

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1722: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                        }
                        // done with this range
                        self.multipart_idx = Some(self.multipart_idx.expect("must be set") + 1);
                    }
                } else {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1783: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            .get_multirange_info()
            .cloned()
            .expect("Multipart Ranges should have MultiPartInfo struct");

        // Pass the whole body in one chunk
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1836: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            .get_multirange_info()
            .cloned()
            .expect("Multipart Ranges should have MultiPartInfo struct");

        // Split the body into 4 chunks
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1900: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            .get_multirange_info()
            .cloned()
            .expect("Multipart Ranges should have MultiPartInfo struct");

        // Split the body into 4 chunks
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 1037: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/src/proxy_cache.rs` (line 1037)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    }
    #[test]
    fn test_parse_range() {
        assert_eq!(
            parse_range_header(b"bytes=0-1", 10),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1070: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/src/proxy_cache.rs` (line 1070)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    // Add some tests for multi-range too
    #[test]
    fn test_parse_range_header_multi() {
        assert_eq!(
            parse_range_header(b"bytes=0-1,4-5", 10)
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1373: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/src/proxy_cache.rs` (line 1373)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_range_filter_single() {
        fn gen_req() -> RequestHeader {
            RequestHeader::build(http::Method::GET, b"/", Some(1)).unwrap()
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1419: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/src/proxy_cache.rs` (line 1419)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    // Multipart Tests
    #[test]
    fn test_range_filter_multipart() {
        fn gen_req() -> RequestHeader {
            let mut req: RequestHeader =
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1484: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/src/proxy_cache.rs` (line 1484)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_if_range() {
        const DATE: &str = "Fri, 07 Jul 2023 22:03:29 GMT";
        const ETAG: &str = "\"1234\"";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1740: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/src/proxy_cache.rs` (line 1740)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_range_body_filter_single() {
        let mut body_filter = RangeBodyFilter::new();
        assert_eq!(body_filter.filter_body(Some("123".into())).unwrap(), "123");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1767: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/src/proxy_cache.rs` (line 1767)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_range_body_filter_multipart() {
        // Test #1 - Test multipart ranges from 1 chunk
        let data = Bytes::from("0123456789");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym