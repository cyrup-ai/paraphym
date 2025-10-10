# `packages/candle/src/cli/args.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: candle
- **File Hash**: e55fa9bd  
- **Timestamp**: 2025-10-10T02:15:58.158210+00:00  
- **Lines of Code**: 233

---## Tests in Source Directory


### Line 199: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/cli/args.rs` (line 199)
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
  


### Line 203: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/cli/args.rs` (line 203)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_default_args() {
        let args = CliArgs::default();
        assert_eq!(args.agent_role, "CYRUP.ai");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 214: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/cli/args.rs` (line 214)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_parse_role() {
        let args = vec![
            "program".to_string(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 225: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/cli/args.rs` (line 225)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_parse_model() {
        let args = vec![
            "program".to_string(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 236: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/cli/args.rs` (line 236)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_parse_temperature() {
        let args = vec![
            "program".to_string(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 247: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/cli/args.rs` (line 247)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_parse_memory_timeout() {
        let args = vec![
            "program".to_string(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 258: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/cli/args.rs` (line 258)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_parse_multiple_tools() {
        let args = vec![
            "program".to_string(),
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 273: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/cli/args.rs` (line 273)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_validate_temperature() {
        let mut args = CliArgs::default();
        args.temperature = 2.5;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 283: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/cli/args.rs` (line 283)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_validate_memory_timeout() {
        let mut args = CliArgs::default();
        args.memory_read_timeout = 0;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym