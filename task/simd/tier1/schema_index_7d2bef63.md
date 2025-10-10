# `packages/simd/src/logits/constraints/schema_index.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: simd
- **File Hash**: 7d2bef63  
- **Timestamp**: 2025-10-10T02:15:58.221689+00:00  
- **Lines of Code**: 511

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 511 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 689
  - would need
  - 

```rust
        let state = constraint.new_state();

        // Test allowed values (note: this is simplified, real enum would need quoted strings)
        assert!(constraint.get_allowed_tokens(&state).is_some());
    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 640
  - stubby method name
  - mock_vocabulary

```rust
    use super::*;

    fn mock_vocabulary() -> SchemaVocabulary {
        let token_to_bytes = vec![
            b"hello".to_vec(),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 657
  - stubby method name
  - mock_vocabulary

```rust
    #[test]
    fn test_boolean_constraint() {
        let vocab = Arc::new(mock_vocabulary());
        let constraint = match utils::boolean_constraint(vocab) {
            Ok(c) => c,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 680
  - stubby method name
  - mock_vocabulary

```rust
    #[test]
    fn test_enum_constraint() {
        let vocab = Arc::new(mock_vocabulary());
        let values = ["hello", "world"];
        let constraint = match utils::enum_constraint(&values, vocab) {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 695
  - stubby method name
  - mock_vocabulary

```rust
    #[test]
    fn test_deterministic_sequence() {
        let vocab = Arc::new(mock_vocabulary());
        let constraint = match utils::null_constraint(vocab) {
            Ok(c) => c,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 713
  - stubby method name
  - mock_vocabulary

```rust
    #[test]
    fn test_index_stats() {
        let vocab = Arc::new(mock_vocabulary());
        let constraint = match utils::boolean_constraint(vocab) {
            Ok(c) => c,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 640
  - stubby variable name
  - mock_vocabulary

```rust
    use super::*;

    fn mock_vocabulary() -> SchemaVocabulary {
        let token_to_bytes = vec![
            b"hello".to_vec(),
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 657
  - stubby variable name
  - mock_vocabulary

```rust
    #[test]
    fn test_boolean_constraint() {
        let vocab = Arc::new(mock_vocabulary());
        let constraint = match utils::boolean_constraint(vocab) {
            Ok(c) => c,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 680
  - stubby variable name
  - mock_vocabulary

```rust
    #[test]
    fn test_enum_constraint() {
        let vocab = Arc::new(mock_vocabulary());
        let values = ["hello", "world"];
        let constraint = match utils::enum_constraint(&values, vocab) {
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 695
  - stubby variable name
  - mock_vocabulary

```rust
    #[test]
    fn test_deterministic_sequence() {
        let vocab = Arc::new(mock_vocabulary());
        let constraint = match utils::null_constraint(vocab) {
            Ok(c) => c,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 713
  - stubby variable name
  - mock_vocabulary

```rust
    #[test]
    fn test_index_stats() {
        let vocab = Arc::new(mock_vocabulary());
        let constraint = match utils::boolean_constraint(vocab) {
            Ok(c) => c,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 77
  - Fallback
  - 

```rust
    fn build_transitions(&mut self, dfa: &DFA<Vec<u32>>, vocabulary: &SchemaVocabulary) -> AnyResult<()> {
        let initial_state_id = dfa.universal_start_state(Anchored::Yes).unwrap_or_else(|| {
            // Fallback to creating a basic start state
            AutomataStateId::from_ne_bytes([0, 0, 0, 0]).unwrap_or(AutomataStateId::ZERO)
        });
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 310
  - Fallback
  - 

```rust
        }

        // Fallback: use token ID 0 or search for likely candidates
        tokenizer.get_vocab(false)
            .iter()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tests in Source Directory


### Line 637: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/logits/constraints/schema_index.rs` (line 637)
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
  


### Line 656: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/logits/constraints/schema_index.rs` (line 656)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_boolean_constraint() {
        let vocab = Arc::new(mock_vocabulary());
        let constraint = match utils::boolean_constraint(vocab) {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 679: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/logits/constraints/schema_index.rs` (line 679)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_enum_constraint() {
        let vocab = Arc::new(mock_vocabulary());
        let values = ["hello", "world"];
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 694: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/logits/constraints/schema_index.rs` (line 694)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_deterministic_sequence() {
        let vocab = Arc::new(mock_vocabulary());
        let constraint = match utils::null_constraint(vocab) {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 712: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/logits/constraints/schema_index.rs` (line 712)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_index_stats() {
        let vocab = Arc::new(mock_vocabulary());
        let constraint = match utils::boolean_constraint(vocab) {
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `string_pattern_constraint()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/logits/constraints/schema_index.rs` (line 578)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

    /// Create a simple string pattern constraint
    pub fn string_pattern_constraint(
        pattern: &str,
        vocabulary: Arc<SchemaVocabulary>,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `numeric_range_constraint()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/simd/src/logits/constraints/schema_index.rs` (line 604)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

    /// Create a numeric range constraint
    pub fn numeric_range_constraint(
        min: Option<i64>,
        max: Option<i64>,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym