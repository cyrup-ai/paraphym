# `packages/candle/src/util/input_resolver.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: candle
- **File Hash**: c44b8c59  
- **Timestamp**: 2025-10-10T02:15:58.169061+00:00  
- **Lines of Code**: 57

---## Tier 1 Infractions 


- Line 43
  - hardcoded URL
  - 

```rust
            .await
            .context(format!("Failed to read file: {}", input))
    } else if input.starts_with("http://") || input.starts_with("https://") {
        // URL - fetch from web with timeout and retries
        let client = reqwest::Client::builder()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 102
  - hardcoded URL
  - 

```rust
    async fn test_resolve_url() {
        // URL format should be recognized
        let input = "https://example.com/doc.txt";
        let result = resolve_smart_input(input).await;
        // We don't test actual fetch, just that it's recognized as URL
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 104
  - actual
  - 

```rust
        let input = "https://example.com/doc.txt";
        let result = resolve_smart_input(input).await;
        // We don't test actual fetch, just that it's recognized as URL
        // The result may fail due to network, which is expected
        assert!(result.is_ok() || result.is_err());
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 66: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        }
        
        Err(last_error.unwrap().into())
    } else {
        // Literal text - use as-is
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 89: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/input_resolver.rs` (line 89)
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
  


### Line 93: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/input_resolver.rs` (line 93)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_resolve_literal_text() {
        let result = resolve_smart_input("Hello, world!").await;
        assert!(result.is_ok());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 100: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/input_resolver.rs` (line 100)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_resolve_url() {
        // URL format should be recognized
        let input = "https://example.com/doc.txt";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 110: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/util/input_resolver.rs` (line 110)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_sync_literal() {
        let result = resolve_smart_input_sync("Test content");
        assert!(result.is_ok());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym