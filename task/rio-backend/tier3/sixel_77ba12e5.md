# `packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/ansi/sixel.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: rio-backend
- **File Hash**: 77ba12e5  
- **Timestamp**: 2025-10-10T02:15:59.534788+00:00  
- **Lines of Code**: 600

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 600 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Panic-Prone Code


### Line 868: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                let mut path = images_dir.join(test_image);
                path.set_extension("sixel");
                fs::read(path).unwrap()
            };

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 872: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

            // Remove DCS sequence from Sixel data.
            let dcs_end = sixel.iter().position(|&byte| byte == b'q').unwrap();
            sixel.drain(..=dcs_end);

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 884: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            let mut parser = Parser::default();
            for byte in sixel {
                parser.put(byte).unwrap();
            }

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 887: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
            }

            let graphics = parser.finish().unwrap().0;

            assert_eq!(graphics.width, 64);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 897: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                let mut path = images_dir.join(test_image);
                path.set_extension("rgba");
                fs::read(path).unwrap()
            };

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 651: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/ansi/sixel.rs` (line 651)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 666: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/ansi/sixel.rs` (line 666)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn parse_command_parameters() {
        let mut command_parser = CommandParser::new(SixelCommand::ColorIntroducer);
        put_bytes!(command_parser, "65535;1;2;3;4;5");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 679: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/ansi/sixel.rs` (line 679)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn set_color_registers() {
        let mut parser = Parser::default();
        put_bytes!(parser, "#1;2;30;100;0#200;1;20;75;50.");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 706: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/ansi/sixel.rs` (line 706)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn convert_hls_colors() {
        // This test converts some values from HLS to RBG, and compares those
        // results with the values generated by the libsixel implementation
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 758: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/ansi/sixel.rs` (line 758)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn resize_picture() -> Result<(), Error> {
        let mut parser = Parser {
            background: REG_TRANSPARENT,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 827: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/ansi/sixel.rs` (line 827)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn sixel_height() {
        assert_eq!(Sixel(0b000000).height(), 0);
        assert_eq!(Sixel(0b000001).height(), 1);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 836: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/ansi/sixel.rs` (line 836)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn sixel_positions() {
        macro_rules! dots {
            ($sixel:expr) => {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 854: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/rio-backend/src/ansi/sixel.rs` (line 854)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn load_sixel_files() {
        let images_dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sixel"));

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym