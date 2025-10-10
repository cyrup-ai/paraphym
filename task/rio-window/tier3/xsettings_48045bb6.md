# `packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-window/src/platform_impl/linux/x11/xsettings.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: rio-window
- **File Hash**: 48045bb6  
- **Timestamp**: 2025-10-10T02:15:58.709498+00:00  
- **Lines of Code**: 251

---## Panic-Prone Code


### Line 217: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    fn i16(&mut self) -> Result<i16> {
        self.advance(2).map(|s| {
            let bytes: &[u8; 2] = s.try_into().unwrap();
            match self.endianness {
                Endianness::Big => i16::from_be_bytes(*bytes),
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
    fn i32(&mut self) -> Result<i32> {
        self.advance(4).map(|s| {
            let bytes: &[u8; 4] = s.try_into().unwrap();
            match self.endianness {
                Endianness::Big => i32::from_be_bytes(*bytes),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 274: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
impl ParserError {
    fn ran_out(expected: usize, found: usize) -> ParserError {
        let expected = NonZeroUsize::new(expected).unwrap();
        Self::NoMoreBytes { expected, found }
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 308: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .split(',')
            .map(|tok| {
                let val = tok.strip_prefix("0x").unwrap();
                u8::from_str_radix(val, 16).unwrap()
            })
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 309: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .map(|tok| {
                let val = tok.strip_prefix("0x").unwrap();
                u8::from_str_radix(val, 16).unwrap()
            })
            .collect::<Vec<_>>();
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
            .unwrap()
            .collect::<Result<Vec<_>>>()
            .unwrap();

        let dpi = settings.iter().find(|s| s.name == b"Xft/DPI").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 314: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        let settings = read_settings(&data)
            .unwrap()
            .collect::<Result<Vec<_>>>()
            .unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 318: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .unwrap();

        let dpi = settings.iter().find(|s| s.name == b"Xft/DPI").unwrap();
        assert_int(&dpi.data, 96 * 1024);
        let hinting = settings.iter().find(|s| s.name == b"Xft/Hinting").unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 320: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        let dpi = settings.iter().find(|s| s.name == b"Xft/DPI").unwrap();
        assert_int(&dpi.data, 96 * 1024);
        let hinting = settings.iter().find(|s| s.name == b"Xft/Hinting").unwrap();
        assert_int(&hinting.data, 1);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 323: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        assert_int(&hinting.data, 1);

        let rgba = settings.iter().find(|s| s.name == b"Xft/RGBA").unwrap();
        assert_string(&rgba.data, "rgb");
        let lcd = settings
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 328: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            .iter()
            .find(|s| s.name == b"Xft/Lcdfilter")
            .unwrap();
        assert_string(&lcd.data, "lcddefault");
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


### Line 281: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-window/src/platform_impl/linux/x11/xsettings.rs` (line 281)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
#[cfg(test)]
/// Tests for the XSETTINGS parser.
mod tests {
    use super::*;

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 287: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-window/src/platform_impl/linux/x11/xsettings.rs` (line 287)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn empty() {
        let err = match read_settings(&[]) {
            Ok(_) => panic!(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 303: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-window/src/platform_impl/linux/x11/xsettings.rs` (line 303)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn parse_xsettings() {
        let data = XSETTINGS
            .trim()
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym