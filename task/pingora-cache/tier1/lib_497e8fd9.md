# `forks/pingora/pingora-cache/src/lib.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-cache
- **File Hash**: 497e8fd9  
- **Timestamp**: 2025-10-10T02:16:01.411017+00:00  
- **Lines of Code**: 1064

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 1064 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 388
  - for now
  - 

```rust
                        }
                        // depends on why the proxy upstream filter declined the request,
                        // for now still allow next request try to acquire to avoid thundering herd
                        DeclinedToUpstream => LockStatus::TransientError,
                        // no need for the lock anymore
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 394
  - for now
  - 

```rust
                            LockStatus::GiveUp
                        }
                        // not sure which LockStatus make sense, we treat it as GiveUp for now
                        Custom(_) => LockStatus::GiveUp,
                        // should never happen, NeverEnabled shouldn't hold a lock
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 422
  - TODO
  - 

```rust
                if old_reason == NoCacheReason::NeverEnabled {
                    // safeguard, don't allow replacing NeverEnabled as a reason
                    // TODO: can be promoted to assertion once confirmed nothing is attempting this
                    warn!("Tried to replace cache NeverEnabled with reason: {reason:?}");
                    return;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 748
  - TODO
  - 

```rust
            inner_enabled.traces.log_meta_in_hit_span(&meta);
            if let Some(eviction) = inner_enabled.eviction {
                // TODO: make access() accept CacheKey
                let cache_key = key.to_compact();
                if hit_handler.should_count_access() {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 991
  - TODO
  - 

```rust
    pub fn set_cache_meta(&mut self, meta: CacheMeta) {
        match self.phase {
            // TODO: store the staled meta somewhere else for future use?
            CachePhase::Stale | CachePhase::Miss => {
                let inner_enabled = self.inner_enabled_mut();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 994
  - TODO
  - 

```rust
            CachePhase::Stale | CachePhase::Miss => {
                let inner_enabled = self.inner_enabled_mut();
                // TODO: have a separate expired span?
                inner_enabled.traces.log_meta_in_miss_span(&meta);
                inner_enabled.meta = Some(meta);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1017
  - TODO
  - 

```rust
                    .as_mut()
                    .expect("stale phase has cache enabled");
                // TODO: we should keep old meta in place, just use new one to update it
                // that requires cacheable_filter to take a mut header and just return InternalMeta

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1045
  - TODO
  - 

```rust

                let mut span = inner_enabled.traces.child("update_meta");
                // TODO: this call can be async
                let result = inner_enabled
                    .storage
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1098
  - for now
  - 

```rust
                // incoming request.
                //
                // For simplicity, ignore changing Vary in revalidation for now.
                // TODO: if we support vary during revalidation, there are a few edge cases to
                // consider (what if Vary header appears/disappears/changes)?
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1099
  - TODO
  - 

```rust
                //
                // For simplicity, ignore changing Vary in revalidation for now.
                // TODO: if we support vary during revalidation, there are a few edge cases to
                // consider (what if Vary header appears/disappears/changes)?
                //
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1121
  - TODO
  - 

```rust
        }
        self.phase = CachePhase::RevalidatedNoCache(reason);
        // TODO: remove this asset from cache once finished?
    }

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1183
  - TODO
  - 

```rust
                // Drop the cache lock to avoid leaving a dangling lock
                // (because we locked with the old cache key for the secondary slot)
                // TODO: maybe we should try to signal waiting readers to compete for the primary key
                // lock instead? we will not be modifying this secondary slot so it's not actually
                // ready for readers
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1204
  - TODO
  - 

```rust
    pub fn cache_meta(&self) -> &CacheMeta {
        match self.phase {
            // TODO: allow in Bypass phase?
            CachePhase::Stale
            | CachePhase::StaleUpdating
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1384
  - TODO
  - 

```rust
    /// # Panic
    /// Panics if cache lock was not originally configured for this request.
    // TODO: it may make sense to allow configuring the CacheKeyLock here too that the write permit
    // is associated with
    // (The WritePermit comes from the CacheKeyLock and should be used when releasing from the CacheKeyLock,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1431
  - TODO
  - 

```rust
                    match timeout(wait_timeout, r.wait()).await {
                        Ok(()) => r.lock_status(),
                        // TODO: need to differentiate WaitTimeout vs. Lock(Age)Timeout (expired)?
                        Err(_) => LockStatus::Timeout,
                    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1069
  - hardcoded URL
  - 

```rust
            CachePhase::Stale => {
                /*
                 * https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.5
                 * 304 response MUST generate ... would have been sent in a 200 ...
                 * - Content-Location, Date, ETag, and Vary
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 271
  - actual
  - 

```rust
    /// Check whether the hit status should be treated as a miss. A forced miss
    /// is obviously treated as a miss. A hit-filter failure is treated as a
    /// miss because we can't use the asset as an actual hit. If we treat it as
    /// expired, we still might not be able to use it even if revalidation
    /// succeeds.
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 968
  - actual
  - 

```rust
                        }
                    };
                    // actual eviction can be done async
                    let span = inner_enabled.traces.child("eviction");
                    let handle = span.handle();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1184
  - actual
  - 

```rust
                // (because we locked with the old cache key for the secondary slot)
                // TODO: maybe we should try to signal waiting readers to compete for the primary key
                // lock instead? we will not be modifying this secondary slot so it's not actually
                // ready for readers
                if let Some(lock_ctx) = inner_enabled.lock_ctx.as_mut() {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 404: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                    lock_ctx
                        .cache_lock
                        .release(inner.key.as_ref().unwrap(), permit, lock_status);
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


### Line 584: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    #[inline]
    fn inner_enabled_mut(&mut self) -> &mut HttpCacheInnerEnabled {
        self.inner.as_mut().unwrap().enabled_ctx.as_mut().unwrap()
    }

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
    #[inline]
    fn inner_enabled_mut(&mut self) -> &mut HttpCacheInnerEnabled {
        self.inner.as_mut().unwrap().enabled_ctx.as_mut().unwrap()
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 589: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    #[inline]
    fn inner_enabled(&self) -> &HttpCacheInnerEnabled {
        self.inner.as_ref().unwrap().enabled_ctx.as_ref().unwrap()
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 589: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    #[inline]
    fn inner_enabled(&self) -> &HttpCacheInnerEnabled {
        self.inner.as_ref().unwrap().enabled_ctx.as_ref().unwrap()
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 595: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    #[inline]
    fn inner_mut(&mut self) -> &mut HttpCacheInner {
        self.inner.as_mut().unwrap()
    }

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
    #[inline]
    fn inner(&self) -> &HttpCacheInner {
        self.inner.as_ref().unwrap()
    }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 794: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            | CachePhase::Revalidated
            | CachePhase::RevalidatedNoCache(_) => {
                self.inner_enabled_mut().body_reader.as_mut().unwrap()
            }
            _ => panic!("wrong phase {:?}", self.phase),
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
- **Issue**: Can panic in production code

```rust
                    return Ok(());
                }
                let body_reader = inner_enabled.body_reader.take().unwrap();
                let key = inner.key.as_ref().unwrap();
                let result = body_reader
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 839: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                }
                let body_reader = inner_enabled.body_reader.take().unwrap();
                let key = inner.key.as_ref().unwrap();
                let result = body_reader
                    .finish(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 868: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                    panic!("write handler is already set")
                }
                let meta = inner_enabled.meta.as_ref().unwrap();
                let key = inner.key.as_ref().unwrap();
                let miss_handler = inner_enabled
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
                }
                let meta = inner_enabled.meta.as_ref().unwrap();
                let key = inner.key.as_ref().unwrap();
                let miss_handler = inner_enabled
                    .storage
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 943: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                    return Ok(());
                }
                let miss_handler = inner_enabled.miss_handler.take().unwrap();
                let size = miss_handler.finish().await?;
                let key = inner
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 959: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                if let Some(eviction) = inner_enabled.eviction {
                    let cache_key = key.to_compact();
                    let meta = inner_enabled.meta.as_ref().unwrap();
                    let evicted = match size {
                        MissFinishType::Created(size) => {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1021: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

                // update new meta with old meta's created time
                let old_meta = inner_enabled.meta.take().unwrap();
                let created = old_meta.0.internal.created;
                meta.0.internal.created = created;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1049: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                    .storage
                    .update_meta(
                        inner.key.as_ref().unwrap(),
                        inner_enabled.meta.as_ref().unwrap(),
                        &span.handle(),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1050: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                    .update_meta(
                        inner.key.as_ref().unwrap(),
                        inner_enabled.meta.as_ref().unwrap(),
                        &span.handle(),
                    )
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1074: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                 * - Cache-Control and Expires...
                 */
                let mut old_header = self.inner_enabled().meta.as_ref().unwrap().0.header.clone();
                let mut clone_header = |header_name: &'static str| {
                    for (i, value) in resp.headers.get_all(header_name).iter().enumerate() {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1114: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            CachePhase::Stale => {
                // replace cache meta header
                self.inner_enabled_mut().meta.as_mut().unwrap().0.header = header;
                // upstream request done, release write lock
                self.release_write_lock(reason);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1167: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                .meta
                .as_mut()
                .unwrap()
                .set_variance_key(*variance_hash);
        } else {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1170: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                .set_variance_key(*variance_hash);
        } else {
            inner_enabled.meta.as_mut().unwrap().remove_variance();
        }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1175: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // Change the lookup `key` if necessary, in order to admit asset into the primary slot
        // instead of the secondary slot.
        let key = inner.key.as_ref().unwrap();
        if let Some(old_variance) = key.get_variance_key().as_ref() {
            // This is a secondary variant slot.
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1193: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                // Remove the `variance` from the `key`, so that we admit this asset into the
                // primary slot. (`key` is used to tell storage where to write the data.)
                inner.key.as_mut().unwrap().remove_variance_key();
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


### Line 1210: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            | CachePhase::Hit
            | CachePhase::Revalidated
            | CachePhase::RevalidatedNoCache(_) => self.inner_enabled().meta.as_ref().unwrap(),
            CachePhase::Miss => {
                // this is the async body read case, safe because body_reader is only set
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1215: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                // after meta is retrieved
                if self.inner_enabled().body_reader.is_some() {
                    self.inner_enabled().meta.as_ref().unwrap()
                } else {
                    panic!("wrong phase {:?}", self.phase);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1263: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                    .expect("Cache enabled on cache_lookup");
                let mut span = inner_enabled.traces.child("lookup");
                let key = inner.key.as_ref().unwrap(); // safe, this phase should have cache key
                let now = Instant::now();
                let result = inner_enabled.storage.lookup(key, &span.handle()).await?;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1311: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

                // update vary
                let key = inner.key.as_mut().unwrap();
                // if no variance was previously set, then this is the first cache hit
                let is_initial_cache_hit = key.get_variance_key().is_none();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1328: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                // We can recreate the "full" cache key by using the meta's variance, if needed.
                if matches_variance && is_initial_cache_hit {
                    inner.key.as_mut().unwrap().remove_variance_key();
                }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1469: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                let inner_enabled = self.inner_enabled();
                let span = inner_enabled.traces.child("purge");
                let key = inner.key.as_ref().unwrap().to_compact();
                Self::purge_impl(inner_enabled.storage, inner_enabled.eviction, &key, span).await
            }
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
        let inner_enabled = self.inner_enabled();
        let span = inner_enabled.traces.child("purge");
        let key = self.inner().key.as_ref().unwrap().to_compact();
        let storage = inner_enabled.storage;
        let eviction = inner_enabled.eviction;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 436: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                    .enabled_ctx
                    .take()
                    .expect("could remove enabled_ctx on disable");
                // log initial disable reason
                inner_enabled
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 628: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                .key
                .as_ref()
                .expect("cache key should be set (set_cache_key not called?)"),
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


### Line 725: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let inner = self.inner_mut();

        let key = inner.key.as_ref().expect("key must be set on hit");
        let inner_enabled = inner
            .enabled_ctx
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 729: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            .enabled_ctx
            .as_mut()
            .expect("cache_found must be called while cache enabled");

        // The cache lock might not be set for stale hit or hits treated as
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 833: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            | CachePhase::RevalidatedNoCache(_) => {
                let inner = self.inner_mut();
                let inner_enabled = inner.enabled_ctx.as_mut().expect("cache enabled");
                if inner_enabled.body_reader.is_none() {
                    // already finished, we allow calling this function more than once
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 864: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                    .enabled_ctx
                    .as_mut()
                    .expect("cache enabled on miss and expired");
                if inner_enabled.miss_handler.is_some() {
                    panic!("write handler is already set")
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 894: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                                .miss_handler
                                .as_ref()
                                .expect("miss handler already set")
                                .streaming_write_tag(),
                            &inner_enabled.traces.get_miss_span(),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 938: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                    .enabled_ctx
                    .as_mut()
                    .expect("cache enabled on miss and expired");
                if inner_enabled.miss_handler.is_none() {
                    // already finished, we allow calling this function more than once
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 948: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                    .key
                    .as_ref()
                    .expect("key set by miss or expired phase");
                if let Some(lock_ctx) = inner_enabled.lock_ctx.as_mut() {
                    let lock = lock_ctx.lock.take();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1016: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                    .enabled_ctx
                    .as_mut()
                    .expect("stale phase has cache enabled");
                // TODO: we should keep old meta in place, just use new one to update it
                // that requires cacheable_filter to take a mut header and just return InternalMeta
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1037: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                    if let Some(Locked::Write(permit)) = lock {
                        lock_ctx.cache_lock.release(
                            inner.key.as_ref().expect("key set by stale phase"),
                            permit,
                            LockStatus::Done,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1080: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                            old_header
                                .insert_header(header_name, value)
                                .expect("can add valid header");
                        } else {
                            old_header
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1084: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                            old_header
                                .append_header(header_name, value)
                                .expect("can add valid header");
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


### Line 1160: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            .enabled_ctx
            .as_mut()
            .expect("cache enabled on miss and expired");

        // Update the variance in the meta
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1257: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                    .inner
                    .as_mut()
                    .expect("Cache phase is checked and should have inner");
                let inner_enabled = inner
                    .enabled_ctx
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1261: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                    .enabled_ctx
                    .as_mut()
                    .expect("Cache enabled on cache_lookup");
                let mut span = inner_enabled.traces.child("lookup");
                let key = inner.key.as_ref().unwrap(); // safe, this phase should have cache key
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1307: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                    .enabled_ctx
                    .as_mut()
                    .expect("cache enabled")
                    .valid_after = Some(meta.created());

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1369: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            .lock_ctx
            .as_mut()
            .expect("take_write_lock() called without cache lock");
        let lock = lock_ctx
            .lock
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1373: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            .lock
            .take()
            .expect("take_write_lock() called without lock");
        match lock {
            Locked::Write(w) => (w, lock_ctx.cache_lock),
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