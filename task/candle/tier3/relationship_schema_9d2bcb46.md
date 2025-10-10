# `packages/candle/src/memory/schema/relationship_schema.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: candle
- **File Hash**: 9d2bcb46  
- **Timestamp**: 2025-10-10T02:15:58.151688+00:00  
- **Lines of Code**: 299

---## Tests in Source Directory


### Line 208: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/schema/relationship_schema.rs` (line 208)
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
  


### Line 212: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/schema/relationship_schema.rs` (line 212)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_relationship_creation() {
        let relationship = Relationship::new("source-id", "target-id", "related_to");

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 231: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/schema/relationship_schema.rs` (line 231)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_relationship_with_id() {
        let relationship =
            Relationship::new_with_id("rel-id", "source-id", "target-id", "related_to");
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 245: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/schema/relationship_schema.rs` (line 245)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_relationship_builder_pattern() {
        let relationship = Relationship::new("source-id", "target-id", "related_to")
            .with_metadata(serde_json::json!({"key": "value"}))
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 263: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/schema/relationship_schema.rs` (line 263)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_relationship_update_methods() {
        let mut relationship = Relationship::new("source-id", "target-id", "related_to");
        let original_created_at = relationship.created_at;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 292: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/schema/relationship_schema.rs` (line 292)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_relationship_metadata() -> Result<(), Box<dyn std::error::Error>> {
        let mut relationship = Relationship::new("source-id", "target-id", "related_to");

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 316: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/schema/relationship_schema.rs` (line 316)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_relationship_additional_fields() -> Result<(), Box<dyn std::error::Error>> {
        let mut relationship = Relationship::new("source-id", "target-id", "related_to");

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 346: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/schema/relationship_schema.rs` (line 346)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_relationship_timestamp_iso8601() {
        let relationship = Relationship::new("source-id", "target-id", "related_to");

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 358: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/schema/relationship_schema.rs` (line 358)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_relationship_between_and_involves() {
        let relationship = Relationship::new("memory1", "memory2", "related_to");

```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 375: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/schema/relationship_schema.rs` (line 375)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_relationship_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let relationship = Relationship::new("source-id", "target-id", "related_to")
            .with_metadata(serde_json::json!({"key": "value"}))
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym