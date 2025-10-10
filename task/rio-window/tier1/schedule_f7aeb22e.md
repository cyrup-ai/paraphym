# `packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-window/src/platform_impl/web/web_sys/schedule.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: rio-window
- **File Hash**: f7aeb22e  
- **Timestamp**: 2025-10-10T02:15:58.710361+00:00  
- **Lines of Code**: 251

---## Tier 1 Infractions 


- Line 202
  - TODO
  - 

```rust
}

// TODO: Replace with `u32::div_ceil()` when we hit Rust v1.73.
fn duration_millis_ceil(duration: Duration) -> u32 {
    let micros = duration.subsec_micros();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 132: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        F: 'static + FnMut(),
    {
        let channel = MessageChannel::new().unwrap();
        let closure = Closure::new(f);
        let port_1 = channel.port1();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 79: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let mut options = SchedulerPostTaskOptions::new();
        let controller =
            AbortController::new().expect("Failed to create `AbortController`");
        options.signal(&controller.signal());

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 116: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let handle = window
            .request_idle_callback(closure.as_ref().unchecked_ref())
            .expect("Failed to request idle callback");

        Schedule {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 142: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            port_2
                .post_message(&JsValue::UNDEFINED)
                .expect("Failed to send message")
        });
        let handle = if let Some(duration) = duration {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 167: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            window.set_timeout_with_callback(timeout_closure.as_ref().unchecked_ref())
        }
        .expect("Failed to set timeout");

        Schedule {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 155: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                    let millis: i32 = duration_millis_ceil(duration)
                        .try_into()
                        .expect("millis are somehow bigger then 1K");
                    secs.checked_add(millis)
                })
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