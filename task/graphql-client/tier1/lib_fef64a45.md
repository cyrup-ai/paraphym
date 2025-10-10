# `packages/sweetmcp/packages/graphql-client/src/lib.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: graphql-client
- **File Hash**: fef64a45  
- **Timestamp**: 2025-10-10T02:15:58.357909+00:00  
- **Lines of Code**: 304

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 304 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 444
  - hardcoded URL
  - 

```rust
    #[tokio::test]
    async fn test_client_creation() {
        let client = GraphQLClient::new("https://localhost:8443").await.unwrap();
        assert!(!client.get_schema_sdl().is_empty());
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 457
  - hardcoded URL
  - 

```rust
    #[tokio::test]
    async fn test_schema_generation() {
        let client = GraphQLClient::new("https://localhost:8443").await.unwrap();
        let schema_sdl = client.get_schema_sdl();
        
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 444: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    #[tokio::test]
    async fn test_client_creation() {
        let client = GraphQLClient::new("https://localhost:8443").await.unwrap();
        assert!(!client.get_schema_sdl().is_empty());
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 450: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    #[test]
    fn test_invalid_url() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(GraphQLClient::new("not-a-url"));
        assert!(result.is_err());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 457: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
    #[tokio::test]
    async fn test_schema_generation() {
        let client = GraphQLClient::new("https://localhost:8443").await.unwrap();
        let schema_sdl = client.get_schema_sdl();
        
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 439: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/graphql-client/src/lib.rs` (line 439)
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
  


### Line 443: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/graphql-client/src/lib.rs` (line 443)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust
    
    #[tokio::test]
    async fn test_client_creation() {
        let client = GraphQLClient::new("https://localhost:8443").await.unwrap();
        assert!(!client.get_schema_sdl().is_empty());
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 449: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/graphql-client/src/lib.rs` (line 449)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_invalid_url() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(GraphQLClient::new("not-a-url"));
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 456: `#[tokio :: test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/graphql-client/src/lib.rs` (line 456)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[tokio::test]
    async fn test_schema_generation() {
        let client = GraphQLClient::new("https://localhost:8443").await.unwrap();
        let schema_sdl = client.get_schema_sdl();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym