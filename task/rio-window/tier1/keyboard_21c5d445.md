# `packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-window/src/platform_impl/windows/keyboard.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: rio-window
- **File Hash**: 21c5d445  
- **Timestamp**: 2025-10-10T02:15:58.696635+00:00  
- **Lines of Code**: 1116

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 1116 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 151
  - stubby variable name
  - is_fake

```rust
                        } else {
                            let (_, layout) = layouts.get_current_layout();
                            let is_fake = {
                                let curr_event = finished_event_info.as_ref().unwrap();
                                is_current_fake(curr_event, next_msg, layout)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 153
  - stubby variable name
  - is_current_fake

```rust
                            let is_fake = {
                                let curr_event = finished_event_info.as_ref().unwrap();
                                is_current_fake(curr_event, next_msg, layout)
                            };
                            if is_fake {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 155
  - stubby variable name
  - is_fake

```rust
                                is_current_fake(curr_event, next_msg, layout)
                            };
                            if is_fake {
                                finished_event_info = None;
                            }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 315
  - stubby variable name
  - is_fake

```rust
                        let mut layouts = LAYOUT_CACHE.lock().unwrap();
                        let (_, layout) = layouts.get_current_layout();
                        let is_fake = {
                            let event_info = valid_event_info.as_ref().unwrap();
                            is_current_fake(event_info, next_msg, layout)
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 317
  - stubby variable name
  - is_current_fake

```rust
                        let is_fake = {
                            let event_info = valid_event_info.as_ref().unwrap();
                            is_current_fake(event_info, next_msg, layout)
                        };
                        if is_fake {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 319
  - stubby variable name
  - is_fake

```rust
                            is_current_fake(event_info, next_msg, layout)
                        };
                        if is_fake {
                            valid_event_info = None;
                        }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 767
  - stubby variable name
  - is_current_fake

```rust
/// the next event is a right Alt (AltGr) event. If this is the case, the current event must be the
/// fake Ctrl event.
fn is_current_fake(
    curr_info: &PartialKeyEventInfo,
    next_msg: MSG,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 2 (possible) Infractions 


- Line 153
  - stubby method name
  - is_current_fake

```rust
                            let is_fake = {
                                let curr_event = finished_event_info.as_ref().unwrap();
                                is_current_fake(curr_event, next_msg, layout)
                            };
                            if is_fake {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 317
  - stubby method name
  - is_current_fake

```rust
                        let is_fake = {
                            let event_info = valid_event_info.as_ref().unwrap();
                            is_current_fake(event_info, next_msg, layout)
                        };
                        if is_fake {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 767
  - stubby method name
  - is_current_fake

```rust
/// the next event is a right Alt (AltGr) event. If this is the case, the current event must be the
/// fake Ctrl event.
fn is_current_fake(
    curr_info: &PartialKeyEventInfo,
    next_msg: MSG,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 520
  - fall back
  - 

```rust
    /// Use the text provided by the WM_CHAR messages and report that as a `Character` variant. If
    /// the text consists of multiple grapheme clusters (user-precieved characters) that means that
    /// dead key could not be combined with the second input, and in that case we should fall back
    /// to using what would have without a dead-key input.
    TextOr(Key),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 129: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                    let next_msg = next_kbd_msg(hwnd);

                    let mut layouts = LAYOUT_CACHE.lock().unwrap();
                    let mut finished_event_info =
                        Some(PartialKeyEventInfo::from_message(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 137: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                            &mut layouts,
                        ));
                    let mut event_info = self.event_info.lock().unwrap();
                    *event_info = None;
                    if let Some(next_msg) = next_msg {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 152: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                            let (_, layout) = layouts.get_current_layout();
                            let is_fake = {
                                let curr_event = finished_event_info.as_ref().unwrap();
                                is_current_fake(curr_event, next_msg, layout)
                            };
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 179: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                    // At this point, we know that there isn't going to be any more events related
                    // to this key press
                    let event_info = self.event_info.lock().unwrap().take().unwrap();
                    let ev = event_info.finalize();
                    MatchResult::MessagesToDispatch(self.pending.complete_pending(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 179: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                    // At this point, we know that there isn't going to be any more events related
                    // to this key press
                    let event_info = self.event_info.lock().unwrap().take().unwrap();
                    let ev = event_info.finalize();
                    MatchResult::MessagesToDispatch(self.pending.complete_pending(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 190: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                }
                WM_CHAR | WM_SYSCHAR => {
                    let mut event_info = self.event_info.lock().unwrap();
                    if event_info.is_none() {
                        trace!(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 242: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                        MatchResult::TokenToRemove(pending_token)
                    } else {
                        let mut event_info = self.event_info.lock().unwrap();
                        let mut event_info = match event_info.take() {
                            Some(ev_info) => ev_info,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 250: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                            }
                        };
                        let mut layouts = LAYOUT_CACHE.lock().unwrap();
                        // It's okay to call `ToUnicode` here, because at this point the dead key
                        // is already consumed by the character.
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 298: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                    *result = ProcResult::Value(0);

                    let mut layouts = LAYOUT_CACHE.lock().unwrap();
                    let event_info = PartialKeyEventInfo::from_message(
                        wparam,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 313: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                    let mut valid_event_info = Some(event_info);
                    if let Some(next_msg) = next_msg {
                        let mut layouts = LAYOUT_CACHE.lock().unwrap();
                        let (_, layout) = layouts.get_current_layout();
                        let is_fake = {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 316: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                        let (_, layout) = layouts.get_current_layout();
                        let is_fake = {
                            let event_info = valid_event_info.as_ref().unwrap();
                            is_current_fake(event_info, next_msg, layout)
                        };
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 357: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let mut key_events = Vec::new();

        let mut layouts = LAYOUT_CACHE.lock().unwrap();
        let (locale_id, _) = layouts.get_current_layout();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 480: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            WindowsModifiers::empty()
        };
        let layout = layouts.layouts.get(&(locale_id as u64)).unwrap();
        let logical_key = layout.get_key(mods, num_lock_on, vk, &physical_key);
        let key_without_modifiers =
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 821: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    pub fn add_pending(&self) -> PendingMessageToken {
        let token = self.next_token();
        let mut pending = self.pending.lock().unwrap();
        pending.push(IdentifiedPendingMessage {
            token,
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 835: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    /// See also: `add_pending`
    pub fn complete_pending(&self, token: PendingMessageToken, msg: T) -> Vec<T> {
        let mut pending = self.pending.lock().unwrap();
        let mut target_is_first = false;
        for (i, pending_msg) in pending.iter_mut().enumerate() {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 856: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

    pub fn complete_multi(&self, msgs: Vec<T>) -> Vec<T> {
        let mut pending = self.pending.lock().unwrap();
        if pending.is_empty() {
            return msgs;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 876: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    /// See also: `add_pending`
    pub fn remove_pending(&self, token: PendingMessageToken) -> Vec<T> {
        let mut pending = self.pending.lock().unwrap();
        let mut was_first = false;
        if let Some(m) = pending.first() {
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