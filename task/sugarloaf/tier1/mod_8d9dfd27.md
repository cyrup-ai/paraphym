# `packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font_introspector/scale/bitmap/mod.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: sugarloaf
- **File Hash**: 8d9dfd27  
- **Timestamp**: 2025-10-10T02:15:59.432039+00:00  
- **Lines of Code**: 397

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 397 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 212
  - stubby variable name
  - tmp_width

```rust
    Filter: Fn(f32) -> f32,
{
    let tmp_width = target_width;
    let tmp_height = height;
    let s = 1. / 255.;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 213
  - stubby variable name
  - tmp_height

```rust
{
    let tmp_width = target_width;
    let tmp_height = height;
    let s = 1. / 255.;
    if channels == 1 {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 223
  - stubby variable name
  - tmp_width

```rust
            filter,
            support,
            &mut |x, y, p| scratch[(y * tmp_width + x) as usize] = (p[3] * 255.) as u8,
        );
        sample_dir(
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 226
  - stubby variable name
  - tmp_width

```rust
        );
        sample_dir(
            &|y, x| [0., 0., 0., scratch[(y * tmp_width + x) as usize] as f32 * s],
            tmp_height,
            tmp_width,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 227
  - stubby variable name
  - tmp_height

```rust
        sample_dir(
            &|y, x| [0., 0., 0., scratch[(y * tmp_width + x) as usize] as f32 * s],
            tmp_height,
            tmp_width,
            target_height,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 228
  - stubby variable name
  - tmp_width

```rust
            &|y, x| [0., 0., 0., scratch[(y * tmp_width + x) as usize] as f32 * s],
            tmp_height,
            tmp_width,
            target_height,
            filter,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 261
  - stubby variable name
  - tmp_width

```rust
        sample_dir(
            &|y, x| {
                let row = (y * tmp_width * channels + x * channels) as usize;
                [
                    scratch[row] as f32 * s,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 269
  - stubby variable name
  - tmp_height

```rust
                ]
            },
            tmp_height,
            tmp_width,
            target_height,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 270
  - stubby variable name
  - tmp_width

```rust
            },
            tmp_height,
            tmp_width,
            target_height,
            filter,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Orphaned Methods


### `bilinear()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font_introspector/scale/bitmap/mod.rs` (line 368)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

fn bilinear(x: f32) -> f32 {
    let x = x.abs();
    if x < 1. {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `nearest()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font_introspector/scale/bitmap/mod.rs` (line 407)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

fn nearest(_x: f32) -> f32 {
    1.
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `mitchell()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font_introspector/scale/bitmap/mod.rs` (line 396)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

fn mitchell(x: f32) -> f32 {
    let x = x.abs();
    if x < 1. {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `bicubic()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font_introspector/scale/bitmap/mod.rs` (line 377)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

fn bicubic(x: f32) -> f32 {
    let a = x.abs();
    let b = 0.;
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `lanczos3()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/sugarloaf/src/font_introspector/scale/bitmap/mod.rs` (line 360)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

fn lanczos3(x: f32) -> f32 {
    if x.abs() < 3. {
        (sinc(x) * sinc(x / 3.)).abs()
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym