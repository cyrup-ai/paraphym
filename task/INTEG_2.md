# INTEG_2: Export/Import Data Integrity Issues

## 🟡 QA RATING: 6/10 - CRITICAL DATA INTEGRITY BUGS

**Status**: Partial implementation with data corruption and stubs

**Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/core/manager/surreal.rs`

---

## ✅ COMPLETED: None Handling Bug (Original Issue)

The original INTEG_2 issue about silent record dropping when `.create()` returns `Ok(None)` has been **FIXED**:

- ✅ Node insertion (lines 512-514): Returns `Error::NotFound` on None
- ✅ Relationship insertion (lines 530-532): Returns `Error::NotFound` on None
- ✅ Matches established pattern from create_memory/create_relationship
- ✅ Error messages include record IDs

**This specific issue is complete.**

---

## 🔴 CRITICAL ISSUE 1: Binary Import Stub

**Location**: Lines 489-492

### Problem
Binary import is **NOT IMPLEMENTED** - just returns a hardcoded error:

```rust
ImportFormat::Binary => {
    return Err(Error::Migration(
        "Binary import not supported - ExportData doesn't implement bincode::Decode".to_string()
    ));
}
```

### Impact
- `ImportFormat` enum advertises `Binary` as an option
- Users can select Binary format but it always fails
- This is a stub/placeholder, not a production implementation

### Required Fix
**Option A**: Implement binary import properly
- Add `bincode::Decode` derives to ExportData, MemoryNode, MemoryRelationship
- Use `DataImporter::import_binary()` (which exists at importer.rs:89-100)

**Option B**: Remove Binary variant from ImportFormat enum
- Be honest about supported formats
- Remove the stub

---

## 🔴 CRITICAL ISSUE 2: Relationship Timestamp Corruption

**Location**: Lines 59-70 (`RelationshipCreateContent::from`)

### Problem
Import **OVERWRITES** original timestamps with current time:

```rust
impl From<&MemoryRelationship> for RelationshipCreateContent {
    fn from(relationship: &MemoryRelationship) -> Self {
        Self {
            source_id: relationship.source_id.clone(),
            target_id: relationship.target_id.clone(),
            relationship_type: relationship.relationship_type.clone(),
            metadata: relationship.metadata.clone().unwrap_or_else(...),
            created_at: crate::memory::utils::current_timestamp_ms(),  // BUG!
            updated_at: crate::memory::utils::current_timestamp_ms(),  // BUG!
            strength: 1.0,
        }
    }
}
```

### Impact
- Historical timestamps are **DESTROYED** during import
- All relationships get today's timestamp, not their original creation time
- Data integrity violation - temporal data is lost
- Breaks audit trails and time-based queries

### Root Cause
`MemoryRelationship` (domain model) lacks timestamp fields:
```rust
pub struct MemoryRelationship {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    pub relationship_type: String,
    pub metadata: Option<Value>,
    // Missing: created_at, updated_at, strength
}
```

But `RelationshipSchema` (database) has them:
```rust
pub struct Relationship {  // aka RelationshipSchema
    pub id: RecordId,
    pub source_id: String,
    pub target_id: String,
    pub relationship_type: String,
    pub metadata: Value,
    pub created_at: u64,      // ← Missing in domain model
    pub updated_at: u64,      // ← Missing in domain model
    pub strength: f32,        // ← Missing in domain model
    // ...
}
```

### Required Fix
**Option A**: Add timestamp fields to MemoryRelationship
```rust
pub struct MemoryRelationship {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    pub relationship_type: String,
    pub metadata: Option<Value>,
    pub created_at: Option<u64>,     // ← Add
    pub updated_at: Option<u64>,     // ← Add
    pub strength: Option<f32>,       // ← Add
}
```

Then update conversion to preserve timestamps:
```rust
created_at: relationship.created_at.unwrap_or_else(|| current_timestamp_ms()),
updated_at: relationship.updated_at.unwrap_or_else(|| current_timestamp_ms()),
strength: relationship.strength.unwrap_or(1.0),
```

**Option B**: Create separate types for export/import
- `ExportRelationship` with all fields
- Keep `MemoryRelationship` for runtime use
- Update `ExportData` to use `ExportRelationship`

---

## 🔴 CRITICAL ISSUE 3: Relationship Data Loss on Export

**Location**: Lines 432-437 (export_memories)

### Problem
Export **DROPS timestamp and strength fields** during deserialization:

```rust
// Query all relationships from 'memory_relationship' table
let rels_query = "SELECT * FROM memory_relationship";
let mut rels_response = self.db.query(rels_query).await
    .map_err(|e| Error::Database(format!("Failed to query relationships: {}", e)))?;

let relationships: Vec<MemoryRelationship> = rels_response.take(0)  // ← BUG!
    .map_err(|e| Error::Database(format!("Failed to parse relationships: {}", e)))?;
```

### Impact
- Database returns `RelationshipSchema` with created_at, updated_at, strength
- Deserializes into `MemoryRelationship` which lacks these fields
- Serde silently **IGNORES** extra fields (created_at, updated_at, strength)
- Export file is **MISSING** critical data
- Import then generates fake timestamps (Issue #2 above)

**Complete data corruption cycle:**
1. Export drops timestamps/strength from database → export file
2. Import reads file (no timestamps) → generates new timestamps
3. Result: All temporal data is lost in export/import round-trip

### Required Fix
Same as Issue #2 - add fields to MemoryRelationship OR use separate export types

---

## 🟡 ISSUE 4: Export Array Wrapping Hack

**Location**: Lines 448-450 (export_memories)

### Problem
Wraps single item in array due to API mismatch:

```rust
// Convert to slice for DataExporter (it expects &[T])
let export_slice = std::slice::from_ref(&export_data);
```

The comment admits: "export creates array of 1 element"

Import then unwraps it:
```rust
// Extract the single ExportData element (export creates array of 1 element)
let import_data = import_vec
    .into_iter()
    .next()
    .ok_or_else(|| Error::Migration("Import file is empty".to_string()))?;
```

### Impact
- Not a data loss issue, but poor API design
- DataExporter expects `&[T]` for batch operations
- Export/import always uses single item
- Unnecessary complexity

### Suggested Fix
Either:
- Update DataExporter to accept `&T` for single items
- Or document that this is intentional for future batch support

---

## 🟡 ISSUE 5: Memory Export Bypasses Conversion

**Location**: Line 427 (export_memories)

### Problem
Export deserializes directly instead of using helper:

```rust
let nodes: Vec<MemoryNode> = nodes_response.take(0)  // ← Bypasses from_schema()
    .map_err(|e| Error::Database(format!("Failed to parse memory nodes: {}", e)))?;
```

But there's a proper conversion function:
```rust
fn from_schema(schema: MemoryNodeSchema) -> MemoryNode {  // Line 328
    // Proper field mapping
}
```

### Impact
- May produce incorrect data if structures don't align
- Inconsistent with how other code converts schema → domain model
- Bypasses established conversion logic

### Suggested Fix
```rust
let node_schemas: Vec<MemoryNodeSchema> = nodes_response.take(0)?;
let nodes: Vec<MemoryNode> = node_schemas
    .into_iter()
    .map(SurrealDBMemoryManager::from_schema)
    .collect();
```

---

## DEFINITION OF DONE

- [ ] Binary import either implemented OR removed from ImportFormat enum
- [ ] MemoryRelationship includes timestamp and strength fields (OR separate export type created)
- [ ] RelationshipCreateContent preserves original timestamps instead of generating new ones
- [ ] Export properly captures all database fields (no data loss)
- [ ] Memory export uses from_schema() for consistency
- [ ] Export/import round-trip preserves all data with no corruption
- [ ] Code passes `cargo check -p paraphym_candle`
- [ ] Code passes `cargo clippy` with no warnings

---

## VERIFICATION STEPS

1. **Compilation check:**
   ```bash
   cargo check -p paraphym_candle
   ```

2. **Clippy check:**
   ```bash
   cargo clippy -p paraphym_candle -- -D warnings
   ```

3. **Data integrity test:**
   - Export a relationship with known timestamps
   - Import it back
   - Verify timestamps are preserved (not current time)

---

## FILES TO MODIFY

1. `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/core/primitives/relationship.rs`
   - Add created_at, updated_at, strength fields to MemoryRelationship

2. `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/core/manager/surreal.rs`
   - Fix RelationshipCreateContent conversion (lines 59-70)
   - Fix export to preserve all fields (lines 432-437)
   - Fix binary import stub (lines 489-492)
   - Use from_schema for memory export (line 427)

3. `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/migration/importer.rs`
   - Either implement binary import OR remove from enum
