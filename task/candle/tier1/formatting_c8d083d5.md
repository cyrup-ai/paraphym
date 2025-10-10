# `packages/candle/src/domain/chat/formatting.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: candle
- **File Hash**: c8d083d5  
- **Timestamp**: 2025-10-10T02:15:58.134776+00:00  
- **Lines of Code**: 844

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 844 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 708
  - TODO
  - 

```rust
            });
        }
        // TODO: Validate regex pattern syntax
        Ok(())
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 1131
  - Legacy
  - 

```rust
}

/// Legacy compatibility type aliases (deprecated)
///
/// Deprecated alias for `ImmutableMessageContent` - use `ImmutableMessageContent` instead for zero-allocation streaming
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1149
  - Legacy
  - 

```rust
pub type CustomFormatRule = ImmutableCustomFormatRule;

/// Legacy compatibility alias for `StreamingMessageFormatter`
#[deprecated(note = "Use StreamingMessageFormatter instead for zero-allocation streaming")]
pub type MessageFormatter = StreamingMessageFormatter;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tests in Source Directory


### Line 1154: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/chat/formatting.rs` (line 1154)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use super::*;

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1158: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/chat/formatting.rs` (line 1158)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_message_content_validation() {
        let content = ImmutableMessageContent::Plain {
            text: "Hello, world!".to_string(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1168: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/chat/formatting.rs` (line 1168)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_format_style_creation() -> Result<(), Box<dyn std::error::Error>> {
        let style = FormatStyle::new(0, 5, StyleType::Bold)?;
        assert_eq!(style.length(), 5);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1178: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/chat/formatting.rs` (line 1178)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_color_scheme_validation() {
        let valid_scheme = ImmutableColorScheme::default();
        assert!(valid_scheme.validate().is_ok());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1190: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/chat/formatting.rs` (line 1190)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_formatter_creation() -> Result<(), Box<dyn std::error::Error>> {
        let options = ImmutableFormatOptions::default();
        let formatter = StreamingMessageFormatter::new(options)?;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1199: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/chat/formatting.rs` (line 1199)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_output_format_capabilities() {
        assert!(OutputFormat::Html.supports_styling());
        assert!(OutputFormat::Html.supports_colors());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym