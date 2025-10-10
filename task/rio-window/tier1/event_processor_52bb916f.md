# `packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-window/src/platform_impl/linux/x11/event_processor.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: rio-window
- **File Hash**: 52bb916f  
- **Timestamp**: 2025-10-10T02:15:58.692827+00:00  
- **Lines of Code**: 1631

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 1631 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 779
  - hack
  - 

```rust
            let hittest = shared_state_lock.cursor_hittest;

            // This is a hack to ensure that the DPI adjusted resize is actually
            // applied on all WMs. KWin doesn't need this, but Xfwm does. The hack
            // should not be run on other WMs, since tiling WMs constrain the window
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 780
  - hack
  - 

```rust

            // This is a hack to ensure that the DPI adjusted resize is actually
            // applied on all WMs. KWin doesn't need this, but Xfwm does. The hack
            // should not be run on other WMs, since tiling WMs constrain the window
            // size, making the resize fail. This would cause an endless stream of
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1146
  - In practice
  - 

```rust

            // Suppress emulated scroll wheel clicks, since we handle the real motion events for
            // those. In practice, even clicky scroll wheels appear to be reported by
            // evdev (and XInput2 in turn) as axis motion, so we don't otherwise
            // special-case these button presses.
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 720
  - stubby method name
  - is_dummy

```rust
                    .expect("Failed to find monitor for window");

                if monitor.is_dummy() {
                    // Avoid updating monitor using a dummy monitor handle
                    last_scale_factor
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 720
  - stubby variable name
  - is_dummy

```rust
                    .expect("Failed to find monitor for window");

                if monitor.is_dummy() {
                    // Avoid updating monitor using a dummy monitor handle
                    last_scale_factor
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 2 (possible) Infractions 


- Line 721
  - dummy
  - 

```rust

                if monitor.is_dummy() {
                    // Avoid updating monitor using a dummy monitor handle
                    last_scale_factor
                } else {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 472
  - actual
  - 

```rust
            // This event occurs every time the mouse moves while a file's being dragged
            // over our window. We emit HoveredFile in response; while the macOS backend
            // does that upon a drag entering, XDND doesn't have access to the actual drop
            // data until this event. For parity with other platforms, we only emit
            // `HoveredFile` the first time, though if winit's API is later extended to
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 779
  - actual
  - 

```rust
            let hittest = shared_state_lock.cursor_hittest;

            // This is a hack to ensure that the DPI adjusted resize is actually
            // applied on all WMs. KWin doesn't need this, but Xfwm does. The hack
            // should not be run on other WMs, since tiling WMs constrain the window
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 430: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                data: xproto::ClientMessageData::from({
                    let [a, b, c, d, e]: [c_long; 5] =
                        xev.data.as_longs().try_into().unwrap();
                    [a as u32, b as u32, c as u32, d as u32, e as u32]
                }),
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
                );

                let new_inner_size = *inner_size.lock().unwrap();
                drop(inner_size);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 505: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                    self.dnd
                        .send_status(window, source_window, DndState::Rejected)
                        .expect("Failed to send `XdndStatus` message.");
                }
                self.dnd.reset();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 532: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                self.dnd
                    .send_status(window, source_window, DndState::Accepted)
                    .expect("Failed to send `XdndStatus` message.");
            }
            return;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 560: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                self.dnd
                    .send_finished(window, source_window, state)
                    .expect("Failed to send `XdndFinished` message.");
            }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 718: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                    .xconn
                    .get_monitor_for_window(Some(window_rect))
                    .expect("Failed to find monitor for window");

                if monitor.is_dummy() {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 872: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            ime.borrow_mut()
                .remove_context(window as XWindow)
                .expect("Failed to destroy input context");
        }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1362: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            ime.borrow_mut()
                .focus(xev.event)
                .expect("Failed to focus input context");
        }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1433: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            ime.borrow_mut()
                .unfocus(xev.event)
                .expect("Failed to unfocus input context");
        }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1993: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        wt.xconn
            .reload_database()
            .expect("failed to reload Xft database");

        // In the future, it would be quite easy to emit monitor hotplug events.
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 2007: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            .xconn
            .available_monitors()
            .expect("Failed to get monitor list");
        for new_monitor in new_list {
            // Previous list may be empty, in case of disconnecting and
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Orphaned Methods


### `predicate()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-window/src/platform_impl/linux/x11/event_processor.rs` (line 319)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
        // global Xlib mutex.
        // XPeekEvent does not remove events from the queue.
        unsafe extern "C" fn predicate(
            _display: *mut XDisplay,
            _event: *mut XEvent,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym