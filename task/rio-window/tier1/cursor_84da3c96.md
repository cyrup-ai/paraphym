# `packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-window/src/platform_impl/macos/cursor.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: rio-window
- **File Hash**: 84da3c96  
- **Timestamp**: 2025-10-10T02:15:58.710126+00:00  
- **Lines of Code**: 184

---## Tier 1 Infractions 


- Line 21
  - TODO
  - 

```rust

// SAFETY: NSCursor is immutable and thread-safe
// TODO(madsmtm): Put this logic in objc2-app-kit itself
unsafe impl Send for CustomCursor {}
unsafe impl Sync for CustomCursor {}
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 131
  - TODO
  - 

```rust
    let image = NSImage::initByReferencingFile(NSImage::alloc(), &pdf_path).unwrap();

    // TODO: Handle PLists better
    let info_path = cursor_path.stringByAppendingPathComponent(ns_string!("info.plist"));
    let info: Retained<NSDictionary<NSObject, NSObject>> = unsafe {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 181
  - TODO
  - 

```rust

    fn new_invisible() -> Retained<NSCursor> {
        // TODO: Consider using `dataWithBytesNoCopy:`
        let data = NSData::with_bytes(CURSOR_BYTES);
        let image = NSImage::initWithData(NSImage::alloc(), &data).unwrap();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 48: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            width as isize * 4,
            32,
        ).unwrap()
    };
    let bitmap_data =
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 129: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

    let pdf_path = cursor_path.stringByAppendingPathComponent(ns_string!("cursor.pdf"));
    let image = NSImage::initByReferencingFile(NSImage::alloc(), &pdf_path).unwrap();

    // TODO: Handle PLists better
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 183: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        // TODO: Consider using `dataWithBytesNoCopy:`
        let data = NSData::with_bytes(CURSOR_BYTES);
        let image = NSImage::initWithData(NSImage::alloc(), &data).unwrap();
        let hotspot = NSPoint::new(0.0, 0.0);
        NSCursor::initWithImage_hotSpot(NSCursor::alloc(), &image, hotspot)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Orphaned Methods


### `invisible_cursor()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-window/src/platform_impl/macos/cursor.rs` (line 168)
- **Visibility**: pub(crate)
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

pub(crate) fn invisible_cursor() -> Retained<NSCursor> {
    // 16x16 GIF data for invisible cursor
    // You can reproduce this via ImageMagick.
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `try_cursor_from_selector()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-window/src/platform_impl/macos/cursor.rs` (line 68)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

unsafe fn try_cursor_from_selector(sel: Sel) -> Option<Retained<NSCursor>> {
    let cls = NSCursor::class();
    if msg_send![cls, respondsToSelector: sel] {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym