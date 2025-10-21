# IMPORT_1: Implement Memory Import Format Support

## OBJECTIVE

Implement proper memory import format handling beyond the current "simple JSON import for now" stub.

## BACKGROUND

Memory import is hardcoded to JSON format with a "for now" comment. The `ImportFormat` parameter is ignored, preventing support for multiple import formats.

## SUBTASK 1: Define Import Format Enum

**Location:** `packages/candle/src/memory/core/manager/surreal/manager.rs:266`

**Current State:**
```rust
pub async fn import_memories(&self, path: &Path, _format: ImportFormat) -> Result<()> {
    // Simple JSON import for now
```

**Required Changes:**
- Remove "for now" comment
- Define `ImportFormat` enum if not exists: JSON, YAML, CSV, Binary
- Remove underscore from `_format` parameter (actually use it)
- Implement format detection from file extension as fallback
- Document supported formats

**Why:** Multiple import formats are needed for different use cases and integrations.

## SUBTASK 2: Implement JSON Import

**Location:** Same file

**Required Changes:**
- Extract current JSON import logic into dedicated function
- Add proper JSON validation and error handling
- Support both memory nodes and relationships
- Handle large files with streaming JSON parsing
- Add progress reporting for large imports

**Why:** Proper JSON import with validation and error handling.

## SUBTASK 3: Implement Additional Formats

**Location:** Same file

**Required Changes:**
- Add YAML import support (similar structure to JSON)
- Add CSV import for tabular memory data
- Add binary format for efficient bulk imports
- Route to appropriate parser based on `ImportFormat` parameter
- Share common validation logic across formats

**Why:** Different formats serve different import scenarios.

## SUBTASK 4: Add Import Validation

**Location:** Same file

**Required Changes:**
- Validate schema before import
- Check for duplicate memory IDs
- Verify relationship references exist
- Validate embedding dimensions if present
- Return detailed error messages for invalid imports

**Why:** Invalid imports corrupt the memory database.

## DEFINITION OF DONE

- [ ] No "for now" comments in import code
- [ ] `ImportFormat` parameter actually used (no underscore prefix)
- [ ] Support for JSON, YAML, CSV, and binary formats
- [ ] Format-specific validation and error handling
- [ ] Large file support with streaming where appropriate
- [ ] Documentation explains each format's structure
- [ ] NO test code written (separate team responsibility)
- [ ] NO benchmark code written (separate team responsibility)

## RESEARCH NOTES

### Import Format Use Cases
- JSON: human-readable, good for manual editing
- YAML: more readable than JSON, supports comments
- CSV: tabular data, spreadsheet exports
- Binary: efficient bulk imports, backups

### Schema Structure
- Memory nodes: id, content, metadata, embeddings
- Relationships: from_id, to_id, relationship_type, metadata
- Validate all required fields present
- Validate data types correct

### Integration Points
- `ImportFormat` enum definition (likely in same module)
- SurrealDB import APIs
- File reading and parsing libraries
- Progress reporting for UI/logging

### Streaming Parsing
- Use `serde_json::Deserializer::from_reader()` for JSON
- Use `serde_yaml` for YAML
- Use `csv` crate for CSV
- Process in chunks to handle large files

## CONSTRAINTS

- DO NOT write unit tests (separate team handles testing)
- DO NOT write benchmarks (separate team handles benchmarks)
- Handle large files without loading entirely into memory
- Maintain transaction semantics (all or nothing import)
