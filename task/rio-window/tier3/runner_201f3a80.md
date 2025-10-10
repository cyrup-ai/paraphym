# `packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-window/src/platform_impl/web/event_loop/runner.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: rio-window
- **File Hash**: 201f3a80  
- **Timestamp**: 2025-10-10T02:15:58.698807+00:00  
- **Lines of Code**: 673

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 673 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Panic-Prone Code


### Line 147: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
    pub fn new() -> Self {
        let main_thread =
            MainThreadMarker::new().expect("only callable from inside the `Window`");
        #[allow(clippy::disallowed_methods)]
        let window = web_sys::window().expect("only callable from inside the `Window`");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 149: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            MainThreadMarker::new().expect("only callable from inside the `Window`");
        #[allow(clippy::disallowed_methods)]
        let window = web_sys::window().expect("only callable from inside the `Window`");
        #[allow(clippy::disallowed_methods)]
        let document = window.document().expect("Failed to obtain document");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 151: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let window = web_sys::window().expect("only callable from inside the `Window`");
        #[allow(clippy::disallowed_methods)]
        let document = window.document().expect("Failed to obtain document");

        Shared(Rc::<Execution>::new_cyclic(|weak| {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 161: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                    }
                })
                .expect("`EventLoop` has to be created in the main thread");

            Execution {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 365: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust

                let button = backend::event::mouse_button(&event)
                    .expect("no mouse button pressed");
                runner.send_event(Event::DeviceEvent {
                    device_id: RootDeviceId(DeviceId(event.pointer_id())),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 389: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust

                let button = backend::event::mouse_button(&event)
                    .expect("no mouse button pressed");
                runner.send_event(Event::DeviceEvent {
                    device_id: RootDeviceId(DeviceId(event.pointer_id())),
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