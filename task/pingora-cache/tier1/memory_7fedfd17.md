# `forks/pingora/pingora-cache/src/memory.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-cache
- **File Hash**: 7fedfd17  
- **Timestamp**: 2025-10-10T02:16:01.412773+00:00  
- **Lines of Code**: 522

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 522 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 19
  - TODO
  - 

```rust
//! For testing only, not for production use

//TODO: Mark this module #[test] only

use super::*;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 168
  - FIXME
  - 

```rust
            if self.bytes_written.changed().await.is_err() {
                // err: sender dropped, body is finished
                // FIXME: sender could drop because of an error
                return None;
            }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 219
  - FIXME
  - 

```rust
    fn get_eviction_weight(&self) -> usize {
        match self {
            // FIXME: just body size, also track meta size
            Self::Complete(c) => c.body.len(),
            // partial read cannot be estimated since body size is unknown
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 298
  - stubby method name
  - temp_obj

```rust
}

fn hit_from_temp_obj(temp_obj: &TempObject) -> Result<Option<(CacheMeta, HitHandler)>> {
    let meta = CacheMeta::deserialize(&temp_obj.meta.0, &temp_obj.meta.1)?;
    let partial = PartialHit {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 326
  - stubby method name
  - temp_obj

```rust
            .and_then(|map| map.iter().next())
        {
            hit_from_temp_obj(temp_obj)
        } else if let Some(obj) = self.cached.read().get(&hash) {
            let meta = CacheMeta::deserialize(&obj.meta.0, &obj.meta.1)?;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 353
  - stubby method name
  - temp_obj

```rust
            .try_into()
            .expect("tag must be correct length");
        hit_from_temp_obj(
            self.temp
                .read()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 240
  - stubby variable name
  - temp_id

```rust
    // these are used only in finish() to data from temp to cache
    key: String,
    temp_id: U64WriteId,
    // key -> cache object
    cache: Arc<RwLock<HashMap<String, CacheObject>>>,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 272
  - stubby variable name
  - temp_id

```rust
            .get(&self.key)
            .unwrap()
            .get(&self.temp_id.into())
            .unwrap()
            .make_cache_object();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 280
  - stubby variable name
  - temp_id

```rust
            .write()
            .get_mut(&self.key)
            .and_then(|map| map.remove(&self.temp_id.into()));
        Ok(MissFinishType::Created(size))
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 285
  - stubby variable name
  - temp_id

```rust

    fn streaming_write_tag(&self) -> Option<&[u8]> {
        Some(self.temp_id.as_bytes())
    }
}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 294
  - stubby variable name
  - temp_id

```rust
            .write()
            .get_mut(&self.key)
            .and_then(|map| map.remove(&self.temp_id.into()));
    }
}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 298
  - stubby variable name
  - temp_obj

```rust
}

fn hit_from_temp_obj(temp_obj: &TempObject) -> Result<Option<(CacheMeta, HitHandler)>> {
    let meta = CacheMeta::deserialize(&temp_obj.meta.0, &temp_obj.meta.1)?;
    let partial = PartialHit {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 299
  - stubby variable name
  - temp_obj

```rust

fn hit_from_temp_obj(temp_obj: &TempObject) -> Result<Option<(CacheMeta, HitHandler)>> {
    let meta = CacheMeta::deserialize(&temp_obj.meta.0, &temp_obj.meta.1)?;
    let partial = PartialHit {
        body: temp_obj.body.clone(),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 301
  - stubby variable name
  - temp_obj

```rust
    let meta = CacheMeta::deserialize(&temp_obj.meta.0, &temp_obj.meta.1)?;
    let partial = PartialHit {
        body: temp_obj.body.clone(),
        bytes_written: temp_obj.bytes_written.subscribe(),
        bytes_read: 0,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 302
  - stubby variable name
  - temp_obj

```rust
    let partial = PartialHit {
        body: temp_obj.body.clone(),
        bytes_written: temp_obj.bytes_written.subscribe(),
        bytes_read: 0,
    };
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 320
  - stubby variable name
  - temp_obj

```rust
        // until it is fully updated
        // no preference on which partial read we get (if there are multiple writers)
        if let Some((_, temp_obj)) = self
            .temp
            .read()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 326
  - stubby variable name
  - temp_obj

```rust
            .and_then(|map| map.iter().next())
        {
            hit_from_temp_obj(temp_obj)
        } else if let Some(obj) = self.cached.read().get(&hash) {
            let meta = CacheMeta::deserialize(&obj.meta.0, &obj.meta.1)?;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 370
  - stubby variable name
  - temp_obj

```rust
        let hash = key.combined();
        let meta = meta.serialize()?;
        let temp_obj = TempObject::new(meta);
        let temp_id = self.last_temp_id.fetch_add(1, Ordering::Relaxed);
        let miss_handler = MemMissHandler {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 371
  - stubby variable name
  - temp_id

```rust
        let meta = meta.serialize()?;
        let temp_obj = TempObject::new(meta);
        let temp_id = self.last_temp_id.fetch_add(1, Ordering::Relaxed);
        let miss_handler = MemMissHandler {
            body: temp_obj.body.clone(),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 373
  - stubby variable name
  - temp_obj

```rust
        let temp_id = self.last_temp_id.fetch_add(1, Ordering::Relaxed);
        let miss_handler = MemMissHandler {
            body: temp_obj.body.clone(),
            bytes_written: temp_obj.bytes_written.clone(),
            key: hash.clone(),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 374
  - stubby variable name
  - temp_obj

```rust
        let miss_handler = MemMissHandler {
            body: temp_obj.body.clone(),
            bytes_written: temp_obj.bytes_written.clone(),
            key: hash.clone(),
            cache: self.cached.clone(),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 378
  - stubby variable name
  - temp_id

```rust
            cache: self.cached.clone(),
            temp: self.temp.clone(),
            temp_id: temp_id.into(),
        };
        self.temp
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 384
  - stubby variable name
  - temp_id

```rust
            .entry(hash)
            .or_default()
            .insert(miss_handler.temp_id.into(), temp_obj);
        Ok(Box::new(miss_handler))
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 397
  - stubby variable name
  - temp_removed

```rust
        // empty
        let hash = key.combined();
        let temp_removed = self.temp.write().remove(&hash).is_some();
        let cache_removed = self.cached.write().remove(&hash).is_some();
        Ok(temp_removed || cache_removed)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 399
  - stubby variable name
  - temp_removed

```rust
        let temp_removed = self.temp.write().remove(&hash).is_some();
        let cache_removed = self.cached.write().remove(&hash).is_some();
        Ok(temp_removed || cache_removed)
    }

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 607
  - stubby variable name
  - temp_obj

```rust
        );

        let temp_obj = TempObject::new(meta);
        let mut map = HashMap::new();
        map.insert(0, temp_obj);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 609
  - stubby variable name
  - temp_obj

```rust
        let temp_obj = TempObject::new(meta);
        let mut map = HashMap::new();
        map.insert(0, temp_obj);
        cache.temp.write().insert(hash.clone(), map);

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 126
  - actual
  - 

```rust
        self.range_start = start;
        if let Some(end) = end {
            // end over the actual last byte is allowed, we just need to return the actual bytes
            self.range_end = std::cmp::min(self.body.len(), end);
        }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 273: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .unwrap()
            .get(&self.temp_id.into())
            .unwrap()
            .make_cache_object();
        let size = cache_object.body.len(); // FIXME: this just body size, also track meta size
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 271: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .read()
            .get(&self.key)
            .unwrap()
            .get(&self.temp_id.into())
            .unwrap()
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 433: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

    fn gen_meta() -> CacheMeta {
        let mut header = ResponseHeader::build(200, None).unwrap();
        header.append_header("foo1", "bar1").unwrap();
        header.append_header("foo2", "bar2").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 434: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn gen_meta() -> CacheMeta {
        let mut header = ResponseHeader::build(200, None).unwrap();
        header.append_header("foo1", "bar1").unwrap();
        header.append_header("foo2", "bar2").unwrap();
        header.append_header("foo3", "bar3").unwrap();
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
- **Issue**: Can panic in production code

```rust
        let mut header = ResponseHeader::build(200, None).unwrap();
        header.append_header("foo1", "bar1").unwrap();
        header.append_header("foo2", "bar2").unwrap();
        header.append_header("foo3", "bar3").unwrap();
        header.append_header("Server", "Pingora").unwrap();
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
- **Issue**: Can panic in production code

```rust
        header.append_header("foo1", "bar1").unwrap();
        header.append_header("foo2", "bar2").unwrap();
        header.append_header("foo3", "bar3").unwrap();
        header.append_header("Server", "Pingora").unwrap();
        let internal = crate::meta::InternalMeta::default();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 437: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        header.append_header("foo2", "bar2").unwrap();
        header.append_header("foo3", "bar3").unwrap();
        header.append_header("Server", "Pingora").unwrap();
        let internal = crate::meta::InternalMeta::default();
        CacheMeta(Box::new(crate::meta::CacheMetaInner {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 452: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let key1 = CacheKey::new("", "a", "1");
        let res = MEM_CACHE.lookup(&key1, span).await.unwrap();
        assert!(res.is_none());

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 460: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .get_miss_handler(&key1, &cache_meta, span)
            .await
            .unwrap();
        miss_handler
            .write_body(b"test1"[..].into(), false)
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
            .write_body(b"test1"[..].into(), false)
            .await
            .unwrap();
        miss_handler
            .write_body(b"test2"[..].into(), false)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 468: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .write_body(b"test2"[..].into(), false)
            .await
            .unwrap();
        miss_handler.finish().await.unwrap();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 469: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .await
            .unwrap();
        miss_handler.finish().await.unwrap();

        let (cache_meta2, mut hit_handler) = MEM_CACHE.lookup(&key1, span).await.unwrap().unwrap();
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
        miss_handler.finish().await.unwrap();

        let (cache_meta2, mut hit_handler) = MEM_CACHE.lookup(&key1, span).await.unwrap().unwrap();
        assert_eq!(
            cache_meta.0.internal.fresh_until,
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
        miss_handler.finish().await.unwrap();

        let (cache_meta2, mut hit_handler) = MEM_CACHE.lookup(&key1, span).await.unwrap().unwrap();
        assert_eq!(
            cache_meta.0.internal.fresh_until,
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
- **Issue**: Can panic in production code

```rust
        );

        let data = hit_handler.read_body().await.unwrap().unwrap();
        assert_eq!("test1test2", data);
        let data = hit_handler.read_body().await.unwrap();
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
- **Issue**: Can panic in production code

```rust
        );

        let data = hit_handler.read_body().await.unwrap().unwrap();
        assert_eq!("test1test2", data);
        let data = hit_handler.read_body().await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 479: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let data = hit_handler.read_body().await.unwrap().unwrap();
        assert_eq!("test1test2", data);
        let data = hit_handler.read_body().await.unwrap();
        assert!(data.is_none());
    }
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

        let key1 = CacheKey::new("", "a", "1");
        let res = MEM_CACHE.lookup(&key1, span).await.unwrap();
        assert!(res.is_none());

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
            .get_miss_handler(&key1, &cache_meta, span)
            .await
            .unwrap();
        miss_handler
            .write_body(b"test1test2"[..].into(), false)
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
- **Issue**: Can panic in production code

```rust
            .write_body(b"test1test2"[..].into(), false)
            .await
            .unwrap();
        miss_handler.finish().await.unwrap();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 502: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .await
            .unwrap();
        miss_handler.finish().await.unwrap();

        let (cache_meta2, mut hit_handler) = MEM_CACHE.lookup(&key1, span).await.unwrap().unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 504: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        miss_handler.finish().await.unwrap();

        let (cache_meta2, mut hit_handler) = MEM_CACHE.lookup(&key1, span).await.unwrap().unwrap();
        assert_eq!(
            cache_meta.0.internal.fresh_until,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 504: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        miss_handler.finish().await.unwrap();

        let (cache_meta2, mut hit_handler) = MEM_CACHE.lookup(&key1, span).await.unwrap().unwrap();
        assert_eq!(
            cache_meta.0.internal.fresh_until,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 514: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        assert!(hit_handler.seek(5, None).is_ok());
        let data = hit_handler.read_body().await.unwrap().unwrap();
        assert_eq!("test2", data);
        let data = hit_handler.read_body().await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 514: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        assert!(hit_handler.seek(5, None).is_ok());
        let data = hit_handler.read_body().await.unwrap().unwrap();
        assert_eq!("test2", data);
        let data = hit_handler.read_body().await.unwrap();
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
- **Issue**: Can panic in production code

```rust
        let data = hit_handler.read_body().await.unwrap().unwrap();
        assert_eq!("test2", data);
        let data = hit_handler.read_body().await.unwrap();
        assert!(data.is_none());

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 520: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        assert!(hit_handler.seek(4, Some(5)).is_ok());
        let data = hit_handler.read_body().await.unwrap().unwrap();
        assert_eq!("1", data);
        let data = hit_handler.read_body().await.unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 520: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        assert!(hit_handler.seek(4, Some(5)).is_ok());
        let data = hit_handler.read_body().await.unwrap().unwrap();
        assert_eq!("1", data);
        let data = hit_handler.read_body().await.unwrap();
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
        let data = hit_handler.read_body().await.unwrap().unwrap();
        assert_eq!("1", data);
        let data = hit_handler.read_body().await.unwrap();
        assert!(data.is_none());
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 534: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let key1 = CacheKey::new("", "a", "1");
        let res = MEM_CACHE.lookup(&key1, span).await.unwrap();
        assert!(res.is_none());

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 542: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .get_miss_handler(&key1, &cache_meta, span)
            .await
            .unwrap();

        // first reader
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

        // first reader
        let (cache_meta1, mut hit_handler1) = MEM_CACHE.lookup(&key1, span).await.unwrap().unwrap();
        assert_eq!(
            cache_meta.0.internal.fresh_until,
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

        // first reader
        let (cache_meta1, mut hit_handler1) = MEM_CACHE.lookup(&key1, span).await.unwrap().unwrap();
        assert_eq!(
            cache_meta.0.internal.fresh_until,
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
            .write_body(b"test1"[..].into(), false)
            .await
            .unwrap();

        let data = hit_handler1.read_body().await.unwrap().unwrap();
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
            .unwrap();

        let data = hit_handler1.read_body().await.unwrap().unwrap();
        assert_eq!("test1", data);
        let res = hit_handler1.read_body().now_or_never();
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
            .unwrap();

        let data = hit_handler1.read_body().await.unwrap().unwrap();
        assert_eq!("test1", data);
        let res = hit_handler1.read_body().now_or_never();
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
            .write_body(b"test2"[..].into(), false)
            .await
            .unwrap();
        let data = hit_handler1.read_body().await.unwrap().unwrap();
        assert_eq!("test2", data);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 569: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .await
            .unwrap();
        let data = hit_handler1.read_body().await.unwrap().unwrap();
        assert_eq!("test2", data);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 569: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .await
            .unwrap();
        let data = hit_handler1.read_body().await.unwrap().unwrap();
        assert_eq!("test2", data);

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

        // second reader
        let (cache_meta2, mut hit_handler2) = MEM_CACHE.lookup(&key1, span).await.unwrap().unwrap();
        assert_eq!(
            cache_meta.0.internal.fresh_until,
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

        // second reader
        let (cache_meta2, mut hit_handler2) = MEM_CACHE.lookup(&key1, span).await.unwrap().unwrap();
        assert_eq!(
            cache_meta.0.internal.fresh_until,
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
        );

        let data = hit_handler2.read_body().await.unwrap().unwrap();
        assert_eq!("test1test2", data);
        let res = hit_handler2.read_body().now_or_never();
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
        );

        let data = hit_handler2.read_body().await.unwrap().unwrap();
        assert_eq!("test1test2", data);
        let res = hit_handler2.read_body().now_or_never();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 587: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert!(res.is_none());

        miss_handler.finish().await.unwrap();

        let data = hit_handler1.read_body().await.unwrap();
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
        miss_handler.finish().await.unwrap();

        let data = hit_handler1.read_body().await.unwrap();
        assert!(data.is_none());
        let data = hit_handler2.read_body().await.unwrap();
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
        let data = hit_handler1.read_body().await.unwrap();
        assert!(data.is_none());
        let data = hit_handler2.read_body().await.unwrap();
        assert!(data.is_none());
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 352: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            .expect("tag must be set during streaming write")
            .try_into()
            .expect("tag must be correct length");
        hit_from_temp_obj(
            self.temp
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 350: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let hash = key.combined();
        let write_tag: U64WriteId = streaming_write_tag
            .expect("tag must be set during streaming write")
            .try_into()
            .expect("tag must be correct length");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 358: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                .get(&hash)
                .and_then(|map| map.get(&write_tag.into()))
                .expect("must have partial write in progress"),
        )
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


### Line 427: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/memory.rs` (line 427)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod test {
    use super::*;
    use cf_rustracing::span::Span;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 447: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/memory.rs` (line 447)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_write_then_read() {
        static MEM_CACHE: Lazy<MemCache> = Lazy::new(MemCache::new);
        let span = &Span::inactive().handle();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 484: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/memory.rs` (line 484)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_read_range() {
        static MEM_CACHE: Lazy<MemCache> = Lazy::new(MemCache::new);
        let span = &Span::inactive().handle();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 527: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/memory.rs` (line 527)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_write_while_read() {
        use futures::FutureExt;

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 596: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/memory.rs` (line 596)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_purge_partial() {
        static MEM_CACHE: Lazy<MemCache> = Lazy::new(MemCache::new);
        let cache = &MEM_CACHE;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 623: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-cache/src/memory.rs` (line 623)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_purge_complete() {
        static MEM_CACHE: Lazy<MemCache> = Lazy::new(MemCache::new);
        let cache = &MEM_CACHE;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym