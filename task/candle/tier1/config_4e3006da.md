# `packages/candle/src/cli/config.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: candle
- **File Hash**: 4e3006da  
- **Timestamp**: 2025-10-10T02:15:58.166039+00:00  
- **Lines of Code**: 115

---## Tier 1 Infractions 


- Line 155
  - stubby method name
  - temp_dir

```rust
    #[test]
    fn test_save_and_load() {
        let temp_dir = env::temp_dir();
        let config_path = temp_dir.join("test-candle-config.json");

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 155
  - stubby variable name
  - temp_dir

```rust
    #[test]
    fn test_save_and_load() {
        let temp_dir = env::temp_dir();
        let config_path = temp_dir.join("test-candle-config.json");

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 156
  - stubby variable name
  - temp_dir

```rust
    fn test_save_and_load() {
        let temp_dir = env::temp_dir();
        let config_path = temp_dir.join("test-candle-config.json");

        // Clean up if exists
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 166: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        // Save
        config.save(Some(&config_path)).unwrap();

        // Load
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 169: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

        // Load
        let loaded = CliConfig::load(Some(&config_path)).unwrap();
        assert_eq!(loaded.get_last_model(), Some("phi4"));
        assert_eq!(loaded.get_history().len(), 1);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 128: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/cli/config.rs` (line 128)
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
  


### Line 134: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/cli/config.rs` (line 134)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_default_config() {
        let config = CliConfig::default();
        assert_eq!(config.default_temperature, 0.7);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 141: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/cli/config.rs` (line 141)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_history_management() {
        let mut config = CliConfig::new();
        config.add_to_history("Hello".to_string());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 154: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/cli/config.rs` (line 154)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_save_and_load() {
        let temp_dir = env::temp_dir();
        let config_path = temp_dir.join("test-candle-config.json");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym