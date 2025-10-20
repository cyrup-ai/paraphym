# FIX_DOCUMENT_BAD_CHUNK: Fix Document Type Alias in processor.rs

## OBJECTIVE
Fix 5 compilation errors where `Document::bad_chunk` is called but Document type doesn't have that method. The correct type is `CandleDocument::bad_chunk`.

## STATUS
❌ **BLOCKED** - 5 compilation errors

## ERRORS TO FIX

### Error Locations
All in `packages/candle/src/domain/context/provider/processor.rs`:
- Line 224: `Document::bad_chunk(format!("Path is not a file: {}", context.path))`
- Line 230: `Document::bad_chunk(format!("Failed to access file: {e}"))`
- Line 241: `Document::bad_chunk(format!("File too large: {} bytes (max {} bytes)", ...))`
- Line 319: `Document::bad_chunk(format!("Failed to read file: {e}"))`
- Line 347: `Document::bad_chunk(format!("..."))`

**Error Message**:
```
error[E0599]: no function or associated item named `bad_chunk` found for struct `domain::context::document::CandleDocument` in the current scope
   --> packages/candle/src/domain/context/provider/processor.rs:224:38
    |
224 |                     return Document::bad_chunk(format!("Path is not a file: {}", context.path));
    |                                      ^^^^^^^^^ function or associated item not found in `CandleDocument`
```

## ROOT CAUSE

CandleDocument DOES have a bad_chunk method (implemented via MessageChunk trait), but it's being called through a type alias `Document` that may not be properly imported or configured.

From document.rs:
```rust
impl MessageChunk for CandleDocument {
    fn bad_chunk(error: String) -> Self {
        CandleDocument {
            data: format!("ERROR: {error}"),
            format: Some(CandleContentFormat::Text),
            media_type: Some(CandleDocumentMediaType::TXT),
            additional_props: HashMap::new(),
        }
    }
    // ...
}
```

## IMPLEMENTATION PLAN

### Solution: Replace Document with CandleDocument

**Find and replace** in processor.rs at these 5 locations:
- Line 224
- Line 230  
- Line 241
- Line 319
- Line 347

**CHANGE FROM**:
```rust
return Document::bad_chunk(format!("..."));
```

**CHANGE TO**:
```rust
return CandleDocument::bad_chunk(format!("..."));
```

## SPECIFIC CHANGES

### Line 224
```rust
// BEFORE:
return Document::bad_chunk(format!("Path is not a file: {}", context.path));

// AFTER:
return CandleDocument::bad_chunk(format!("Path is not a file: {}", context.path));
```

### Line 230
```rust
// BEFORE:
return Document::bad_chunk(format!("Failed to access file: {e}"));

// AFTER:
return CandleDocument::bad_chunk(format!("Failed to access file: {e}"));
```

### Line 241
```rust
// BEFORE:
return Document::bad_chunk(format!(
    "File too large: {} bytes (max {} bytes)",
    metadata.len(),
    MAX_FILE_SIZE
));

// AFTER:
return CandleDocument::bad_chunk(format!(
    "File too large: {} bytes (max {} bytes)",
    metadata.len(),
    MAX_FILE_SIZE
));
```

### Line 319
```rust
// BEFORE:
return Document::bad_chunk(format!("Failed to read file: {e}"));

// AFTER:
return CandleDocument::bad_chunk(format!("Failed to read file: {e}"));
```

### Line 347
```rust
// BEFORE:
return Document::bad_chunk(format!(...));

// AFTER:
return CandleDocument::bad_chunk(format!(...));
```

## ALTERNATIVE SOLUTION

If `Document` is meant to be a type alias, ensure it's properly defined at the top of processor.rs:

```rust
use crate::domain::context::CandleDocument as Document;
```

However, using the full type name `CandleDocument` is clearer and more explicit.

## VERIFICATION

After fixes:
```bash
cargo check -p paraphym_candle 2>&1 | grep "processor.rs.*bad_chunk"
```

Should return empty (no errors).

## DEFINITION OF DONE

- [ ] Line 224: Document::bad_chunk → CandleDocument::bad_chunk
- [ ] Line 230: Document::bad_chunk → CandleDocument::bad_chunk
- [ ] Line 241: Document::bad_chunk → CandleDocument::bad_chunk
- [ ] Line 319: Document::bad_chunk → CandleDocument::bad_chunk
- [ ] Line 347: Document::bad_chunk → CandleDocument::bad_chunk
- [ ] All 5 bad_chunk errors resolved
- [ ] Code compiles: `cargo check -p paraphym_candle`
