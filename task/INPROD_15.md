# INPROD_15: Document Context Provider Implementation

## SEVERITY: HIGH

## OBJECTIVE
Implement actual file reading and document creation in the context provider instead of returning placeholder documents.

## LOCATION
- **Primary File:** `packages/candle/src/domain/context/provider.rs`
- **Function:** `load_file_document` (Line 653-683)
- **Key Line:** Line 658 - `// For now, create a basic document structure`

## CURRENT STATE
The `load_file_document` function currently returns a hardcoded placeholder document with:
- Fake data: `format!("Content from file: {}", context.path)`
- Hardcoded format: Always `CandleContentFormat::Text`
- Hardcoded media type: Always `CandleDocumentMediaType::TXT`
- Properly populated metadata (id, path, size, hash)

## EXISTING CODE PATTERNS TO LEVERAGE

### File Reading Pattern (from line 860)
```rust
if let Ok(content) = std::fs::read_to_string(&entry) {
    let document = Document {
        data: content,
        format: Some(crate::domain::context::CandleContentFormat::Text),
        media_type: Some(crate::domain::context::CandleDocumentMediaType::TXT),
        // ... additional props
    };
}
```

### Format Detection (from [builders/document.rs:802](../packages/candle/src/builders/document.rs))
```rust
fn detect_format(content: &str, data: &DocumentBuilderData) -> ContentFormat {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("md") | Some("markdown") => ContentFormat::Markdown,
        Some("html") | Some("htm") => ContentFormat::Html,
        Some("json") => ContentFormat::Json,
        Some("xml") => ContentFormat::Xml,
        Some("yaml") | Some("yml") => ContentFormat::Yaml,
        Some("csv") => ContentFormat::Csv,
        _ => ContentFormat::Text,
    }
}
```

### Media Type Detection (from [builders/document.rs:852](../packages/candle/src/builders/document.rs))
```rust
fn detect_media_type(format: &ContentFormat, data: &DocumentBuilderData) -> DocumentMediaType {
    match format {
        ContentFormat::Json => DocumentMediaType::Json,
        ContentFormat::Html => DocumentMediaType::Html,
        ContentFormat::Markdown => DocumentMediaType::Markdown,
        ContentFormat::Xml => DocumentMediaType::Xml,
        ContentFormat::Csv => DocumentMediaType::Csv,
        ContentFormat::Yaml => DocumentMediaType::Yaml,
        ContentFormat::Base64 => /* binary file detection */,
        _ => DocumentMediaType::PlainText,
    }
}
```

## IMPLEMENTATION PLAN

### Core Implementation for `load_file_document`
```rust
fn load_file_document(
    context: &CandleImmutableFileContext,
) -> Document {
    use std::path::Path;
    
    let path = Path::new(&context.path);
    
    // Attempt to read file as UTF-8 text first
    let (content, format, media_type) = match std::fs::read_to_string(path) {
        Ok(text_content) => {
            // Detect format based on file extension
            let format = match path.extension().and_then(|ext| ext.to_str()) {
                Some("md") | Some("markdown") => CandleContentFormat::Markdown,
                Some("html") | Some("htm") => CandleContentFormat::Html,
                Some("json") => CandleContentFormat::Json,
                Some("xml") => CandleContentFormat::Xml,
                Some("yaml") | Some("yml") => CandleContentFormat::Yaml,
                Some("csv") => CandleContentFormat::Csv,
                _ => CandleContentFormat::Text,
            };
            
            // Detect media type based on format
            let media_type = match format {
                CandleContentFormat::Json => CandleDocumentMediaType::Json,
                CandleContentFormat::Html => CandleDocumentMediaType::Html,
                CandleContentFormat::Markdown => CandleDocumentMediaType::Markdown,
                CandleContentFormat::Xml => CandleDocumentMediaType::Xml,
                CandleContentFormat::Csv => CandleDocumentMediaType::Csv,
                CandleContentFormat::Yaml => CandleDocumentMediaType::Yaml,
                _ => CandleDocumentMediaType::TXT,
            };
            
            (text_content, format, media_type)
        }
        Err(_) => {
            // If text reading fails, try reading as binary
            match std::fs::read(path) {
                Ok(binary_content) => {
                    // Encode binary as base64
                    use base64::{Engine as _, engine::general_purpose};
                    let encoded = general_purpose::STANDARD.encode(&binary_content);
                    
                    // Detect binary file type
                    let media_type = match path.extension().and_then(|ext| ext.to_str()) {
                        Some("pdf") => CandleDocumentMediaType::PDF,
                        Some("doc") | Some("docx") => CandleDocumentMediaType::DOCX,
                        Some("jpg") | Some("jpeg") | Some("png") | Some("gif") => {
                            CandleDocumentMediaType::Image
                        }
                        _ => CandleDocumentMediaType::Binary,
                    };
                    
                    (encoded, CandleContentFormat::Base64, media_type)
                }
                Err(e) => {
                    // If all reading attempts fail, return error information
                    (
                        format!("Error reading file: {}", e),
                        CandleContentFormat::Text,
                        CandleDocumentMediaType::TXT,
                    )
                }
            }
        }
    };
    
    Document {
        data: content,
        format: Some(format),
        media_type: Some(media_type),
        additional_props: {
            let mut props = HashMap::new();
            props.insert(
                "id".to_string(),
                serde_json::Value::String(Uuid::new_v4().to_string()),
            );
            props.insert(
                "path".to_string(),
                serde_json::Value::String(context.path.clone()),
            );
            props.insert(
                "size".to_string(),
                serde_json::Value::String(context.size_bytes.to_string()),
            );
            props.insert(
                "hash".to_string(),
                serde_json::Value::String(context.content_hash.clone()),
            );
            props
        },
    }
}
```

## REQUIRED CHANGES

1. **Add base64 dependency if not present** in `packages/candle/Cargo.toml`
   - Check if `base64` crate is already included
   - If not, add: `base64 = "0.21"`

2. **Import statements** at top of provider.rs if needed:
   ```rust
   use std::path::Path;
   use base64::{Engine as _, engine::general_purpose};
   ```

3. **Replace the entire `load_file_document` function** (lines 653-683) with the implementation above

## FILE TYPE SUPPORT

### Text Files (UTF-8 readable)
- `.txt` → Text/TXT
- `.md`, `.markdown` → Markdown/Markdown
- `.html`, `.htm` → Html/Html
- `.json` → Json/Json
- `.xml` → Xml/Xml
- `.yaml`, `.yml` → Yaml/Yaml
- `.csv` → Csv/Csv

### Binary Files (Base64 encoded)
- `.pdf` → Base64/PDF
- `.doc`, `.docx` → Base64/DOCX
- `.jpg`, `.jpeg`, `.png`, `.gif` → Base64/Image
- Other binary → Base64/Binary

## ERROR HANDLING
- Primary: Try `std::fs::read_to_string` for UTF-8 text
- Fallback: Try `std::fs::read` for binary data, encode as base64
- Final fallback: Return error message in document data field

## DEFINITION OF DONE
- [x] File at `context.path` is actually read from disk
- [x] Document contains real file content (text or base64-encoded binary)
- [x] File extension properly maps to ContentFormat
- [x] ContentFormat properly maps to DocumentMediaType
- [x] UTF-8 text files are read as strings
- [x] Binary files are read and base64-encoded
- [x] File reading errors are gracefully handled
- [x] Placeholder comment "For now, create a basic document structure" is removed
- [x] Hardcoded format string is replaced with actual file content

## CONSTRAINTS
- NO test code implementation
- NO benchmark code implementation
- NO documentation files
- Focus solely on ./src implementation