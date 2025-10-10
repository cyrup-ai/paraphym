# `packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/src/sys/fuchsia/selector.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: corcovado
- **File Hash**: 3e0f0522  
- **Timestamp**: 2025-10-10T02:15:58.458990+00:00  
- **Lines of Code**: 283

---## Tier 1 Infractions 


- Line 313
  - stubby variable name
  - temp_handle

```rust
        }

        let temp_handle = unsafe { zircon::Handle::from_raw(handle) };

        let res = temp_handle.wait_async_handle(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 315
  - stubby variable name
  - temp_handle

```rust
        let temp_handle = unsafe { zircon::Handle::from_raw(handle) };

        let res = temp_handle.wait_async_handle(
            &self.port,
            key_from_token_and_type(token, RegType::Handle)?,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 322
  - stubby variable name
  - temp_handle

```rust
        );

        mem::forget(temp_handle);

        Ok(res?)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 328
  - stubby variable name
  - temp_handle

```rust

    pub fn deregister_handle(&self, handle: zx_handle_t, token: Token) -> io::Result<()> {
        let temp_handle = unsafe { zircon::Handle::from_raw(handle) };
        let res = self.port.cancel(
            &temp_handle,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 330
  - stubby variable name
  - temp_handle

```rust
        let temp_handle = unsafe { zircon::Handle::from_raw(handle) };
        let res = self.port.cancel(
            &temp_handle,
            key_from_token_and_type(token, RegType::Handle)?,
        );
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 334
  - stubby variable name
  - temp_handle

```rust
        );

        mem::forget(temp_handle);

        Ok(res?)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 127: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // written before the store using `Ordering::Release`.
        if self.has_tokens_to_rereg.load(Ordering::Acquire) {
            let mut tokens = self.tokens_to_rereg.lock().unwrap();
            let token_to_fd = self.token_to_fd.lock().unwrap();
            for token in tokens.drain(0..) {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 128: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        if self.has_tokens_to_rereg.load(Ordering::Acquire) {
            let mut tokens = self.tokens_to_rereg.lock().unwrap();
            let token_to_fd = self.token_to_fd.lock().unwrap();
            for token in tokens.drain(0..) {
                if let Some(eventedfd) = token_to_fd.get(&token).and_then(|h| h.upgrade())
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
- **Issue**: Can panic in production code

```rust
                        .token_to_fd
                        .lock()
                        .unwrap()
                        .get(&token)
                        .and_then(|h| h.upgrade())
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
- **Issue**: Can panic in production code

```rust
                    // If necessary, queue to be reregistered before next port_await
                    let needs_to_rereg = {
                        let registration_lock = handle.registration().lock().unwrap();

                        registration_lock
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 228: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                    if needs_to_rereg {
                        let mut tokens_to_rereg_lock =
                            self.tokens_to_rereg.lock().unwrap();
                        tokens_to_rereg_lock.push(token);
                        // We use `Ordering::Release` to make sure that we see all `tokens_to_rereg`
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 253: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    ) -> io::Result<()> {
        {
            let mut token_to_fd = self.token_to_fd.lock().unwrap();
            match token_to_fd.entry(token) {
                hash_map::Entry::Occupied(_) => {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 275: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        if wait_res.is_err() {
            self.token_to_fd.lock().unwrap().remove(&token);
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
- **Issue**: Can panic in production code

```rust
    /// Deregister event interests for the given IO handle with the OS
    pub fn deregister_fd(&self, handle: &zircon::Handle, token: Token) -> io::Result<()> {
        self.token_to_fd.lock().unwrap().remove(&token);

        // We ignore NotFound errors since oneshots are automatically deregistered,
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