# `packages/candle/src/core/model_config.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: candle
- **File Hash**: 54db26be  
- **Timestamp**: 2025-10-10T02:15:58.151472+00:00  
- **Lines of Code**: 325

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 325 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 335
  - stubby variable name
  - temp_dir

```rust
    #[test]
    fn test_model_config_creation() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let model_path = temp_dir.path().join("model.safetensors");
        let tokenizer_path = temp_dir.path().join("tokenizer.json");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 336
  - stubby variable name
  - temp_dir

```rust
    fn test_model_config_creation() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let model_path = temp_dir.path().join("model.safetensors");
        let tokenizer_path = temp_dir.path().join("tokenizer.json");

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 337
  - stubby variable name
  - temp_dir

```rust
        let temp_dir = tempdir()?;
        let model_path = temp_dir.path().join("model.safetensors");
        let tokenizer_path = temp_dir.path().join("tokenizer.json");

        // Create dummy files
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 2 (possible) Infractions 


- Line 339
  - dummy
  - 

```rust
        let tokenizer_path = temp_dir.path().join("tokenizer.json");

        // Create dummy files
        File::create(&model_path)?;
        File::create(&tokenizer_path)?;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tests in Source Directory


### Line 326: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/core/model_config.rs` (line 326)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use std::fs::File;

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 334: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/core/model_config.rs` (line 334)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_model_config_creation() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let model_path = temp_dir.path().join("model.safetensors");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 378: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/core/model_config.rs` (line 378)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_special_token_identification() {
        let tokens = SpecialTokenIds::default();

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 395: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/core/model_config.rs` (line 395)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_architecture_defaults() {
        let llama_arch = ModelArchitecture::Llama(LlamaConfig {
            vocab_size: 32000,
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym