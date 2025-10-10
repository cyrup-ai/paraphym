# `packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-window/src/platform_impl/web/cursor.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: rio-window
- **File Hash**: ec18d0c9  
- **Timestamp**: 2025-10-10T02:15:58.700626+00:00  
- **Lines of Code**: 698

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 698 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Panic-Prone Code


### Line 141: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                let this = weak
                    .upgrade()
                    .expect("`CursorHandler` invalidated without aborting");
                let mut this = this.get(main_thread).borrow_mut();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 212: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            .state
            .take()
            .expect("`CustomCursorFuture` polled after completion");

        Poll::Ready(result.map(|_| CustomCursor {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 293: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                            async move {
                                let _ = notified.await;
                                let handler = weak.upgrade().expect(
                                    "`CursorHandler` invalidated without aborting",
                                );
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 549: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
    fn drop(&mut self) {
        Url::revoke_object_url(&self.0)
            .expect("unexpected exception in `URL.revokeObjectURL()`");
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


### Line 602: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        image.width as u32,
    );
    let image_data = result.expect("found wrong image size");

    // 2. Create an `ImageBitmap` from the `ImageData`.
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 616: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
                &options,
            )
            .expect("unexpected exception in `createImageBitmap()`"),
    );

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 629: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let bitmap: ImageBitmap = bitmap
            .await
            .expect("found invalid state in `ImageData`")
            .unchecked_into();

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 634: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let canvas: HtmlCanvasElement = document
            .create_element("canvas")
            .expect("invalid tag name")
            .unchecked_into();
        #[allow(clippy::disallowed_methods)]
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 645: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
            .get_context("bitmaprenderer")
            .expect("unexpected exception in `HTMLCanvasElement.getContext()`")
            .expect("`bitmaprenderer` context unsupported")
            .unchecked_into();
        context.transfer_from_image_bitmap(&bitmap);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 644: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let context: ImageBitmapRenderingContext = canvas
            .get_context("bitmaprenderer")
            .expect("unexpected exception in `HTMLCanvasElement.getContext()`")
            .expect("`bitmaprenderer` context unsupported")
            .unchecked_into();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 671: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        canvas
            .to_blob(callback.as_ref().unchecked_ref())
            .expect("failed with `SecurityError` despite only source coming from memory");
        let blob = future::poll_fn(|cx| {
            if let Some(blob) = value.borrow_mut().take() {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 689: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        // 5. Create an object URL from the `Blob`.
        let url = Url::create_object_url_with_blob(&blob)
            .expect("unexpected exception in `URL.createObjectURL()`");
        let url = UrlType::Object(ObjectUrl(url));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 703: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
    // 6. Decode the image on an `HTMLImageElement` from the URL.
    let image =
        HtmlImageElement::new().expect("unexpected exception in `new HtmlImageElement`");
    image.set_src(url.url());
    let result = JsFuture::from(image.decode()).await;
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