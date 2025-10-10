# `packages/candle/src/domain/context/provider.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: candle
- **File Hash**: 38af8ea2  
- **Timestamp**: 2025-10-10T02:15:58.131130+00:00  
- **Lines of Code**: 1409

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 1409 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 1528
  - stubby variable name
  - temp_file

```rust
    #[test]
    fn test_load_text_file() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_content = "Hello, this is a test text file!";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1530
  - stubby variable name
  - temp_file

```rust
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_content = "Hello, this is a test text file!";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1531
  - stubby variable name
  - temp_file

```rust
        let test_content = "Hello, this is a test text file!";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1533
  - stubby variable name
  - temp_file

```rust
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1544
  - stubby variable name
  - temp_file

```rust
    #[test]
    fn test_load_json_file() {
        let mut temp_file = NamedTempFile::with_suffix(".json").expect("Failed to create temp file");
        let test_content = r#"{"key": "value", "number": 42}"#;
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1546
  - stubby variable name
  - temp_file

```rust
        let mut temp_file = NamedTempFile::with_suffix(".json").expect("Failed to create temp file");
        let test_content = r#"{"key": "value", "number": 42}"#;
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1547
  - stubby variable name
  - temp_file

```rust
        let test_content = r#"{"key": "value", "number": 42}"#;
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1549
  - stubby variable name
  - temp_file

```rust
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1561
  - stubby variable name
  - temp_file

```rust
    #[test]
    fn test_load_html_file() {
        let mut temp_file = NamedTempFile::with_suffix(".html").expect("Failed to create temp file");
        let test_content = "<html><body><h1>Test</h1></body></html>";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1563
  - stubby variable name
  - temp_file

```rust
        let mut temp_file = NamedTempFile::with_suffix(".html").expect("Failed to create temp file");
        let test_content = "<html><body><h1>Test</h1></body></html>";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1564
  - stubby variable name
  - temp_file

```rust
        let test_content = "<html><body><h1>Test</h1></body></html>";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1566
  - stubby variable name
  - temp_file

```rust
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1578
  - stubby variable name
  - temp_file

```rust
    #[test]
    fn test_load_markdown_file() {
        let mut temp_file = NamedTempFile::with_suffix(".md").expect("Failed to create temp file");
        let test_content = "# Heading\n\nThis is **markdown** content.";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1580
  - stubby variable name
  - temp_file

```rust
        let mut temp_file = NamedTempFile::with_suffix(".md").expect("Failed to create temp file");
        let test_content = "# Heading\n\nThis is **markdown** content.";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1581
  - stubby variable name
  - temp_file

```rust
        let test_content = "# Heading\n\nThis is **markdown** content.";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1583
  - stubby variable name
  - temp_file

```rust
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1595
  - stubby variable name
  - temp_file

```rust
    #[test]
    fn test_load_binary_file() {
        let mut temp_file = NamedTempFile::with_suffix(".pdf").expect("Failed to create temp file");
        let binary_data: Vec<u8> = vec![0x25, 0x50, 0x44, 0x46, 0x2D];
        temp_file.write_all(&binary_data).expect("Failed to write");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1597
  - stubby variable name
  - temp_file

```rust
        let mut temp_file = NamedTempFile::with_suffix(".pdf").expect("Failed to create temp file");
        let binary_data: Vec<u8> = vec![0x25, 0x50, 0x44, 0x46, 0x2D];
        temp_file.write_all(&binary_data).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1598
  - stubby variable name
  - temp_file

```rust
        let binary_data: Vec<u8> = vec![0x25, 0x50, 0x44, 0x46, 0x2D];
        temp_file.write_all(&binary_data).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1600
  - stubby variable name
  - temp_file

```rust
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1613
  - stubby variable name
  - temp_file

```rust
    #[test]
    fn test_utf8_fallback() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let invalid_utf8: Vec<u8> = vec![0xFF, 0xFE, 0xFD, 0x80, 0x81];
        temp_file.write_all(&invalid_utf8).expect("Failed to write");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1615
  - stubby variable name
  - temp_file

```rust
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let invalid_utf8: Vec<u8> = vec![0xFF, 0xFE, 0xFD, 0x80, 0x81];
        temp_file.write_all(&invalid_utf8).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1616
  - stubby variable name
  - temp_file

```rust
        let invalid_utf8: Vec<u8> = vec![0xFF, 0xFE, 0xFD, 0x80, 0x81];
        temp_file.write_all(&invalid_utf8).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1618
  - stubby variable name
  - temp_file

```rust
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1639
  - stubby variable name
  - temp_dir

```rust
    #[test]
    fn test_directory_not_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let path = temp_dir.path().to_string_lossy().to_string();
        let context = create_test_context(path);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1640
  - stubby variable name
  - temp_dir

```rust
    fn test_directory_not_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let path = temp_dir.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1650
  - stubby variable name
  - temp_file

```rust
    #[test]
    fn test_extension_fallback() {
        let mut temp_file = NamedTempFile::with_suffix(".custom").expect("Failed to create temp file");
        let test_content = "Custom file content";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1652
  - stubby variable name
  - temp_file

```rust
        let mut temp_file = NamedTempFile::with_suffix(".custom").expect("Failed to create temp file");
        let test_content = "Custom file content";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1653
  - stubby variable name
  - temp_file

```rust
        let test_content = "Custom file content";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1655
  - stubby variable name
  - temp_file

```rust
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1666
  - stubby variable name
  - temp_file

```rust
    #[test]
    fn test_metadata_preservation() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_content = "Test content for metadata";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1668
  - stubby variable name
  - temp_file

```rust
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_content = "Test content for metadata";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1669
  - stubby variable name
  - temp_file

```rust
        let test_content = "Test content for metadata";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1671
  - stubby variable name
  - temp_file

```rust
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
        let test_hash = "test_hash_456";
        let test_size = test_content.len() as u64;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1705
  - stubby variable name
  - temp_file

```rust
    #[test]
    fn test_csv_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::with_suffix(".csv")?;
        let test_content = "name,age,city\nAlice,30,NYC\nBob,25,LA";
        temp_file.write_all(test_content.as_bytes())?;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1707
  - stubby variable name
  - temp_file

```rust
        let mut temp_file = NamedTempFile::with_suffix(".csv")?;
        let test_content = "name,age,city\nAlice,30,NYC\nBob,25,LA";
        temp_file.write_all(test_content.as_bytes())?;
        temp_file.flush()?;

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1708
  - stubby variable name
  - temp_file

```rust
        let test_content = "name,age,city\nAlice,30,NYC\nBob,25,LA";
        temp_file.write_all(test_content.as_bytes())?;
        temp_file.flush()?;

        let path = temp_file.path().to_string_lossy().to_string();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1710
  - stubby variable name
  - temp_file

```rust
        temp_file.flush()?;

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1723
  - stubby variable name
  - temp_file

```rust
    #[test]
    fn test_xml_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::with_suffix(".xml")?;
        let test_content = r#"<?xml version="1.0"?><root><item>Test</item></root>"#;
        temp_file.write_all(test_content.as_bytes())?;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1725
  - stubby variable name
  - temp_file

```rust
        let mut temp_file = NamedTempFile::with_suffix(".xml")?;
        let test_content = r#"<?xml version="1.0"?><root><item>Test</item></root>"#;
        temp_file.write_all(test_content.as_bytes())?;
        temp_file.flush()?;

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1726
  - stubby variable name
  - temp_file

```rust
        let test_content = r#"<?xml version="1.0"?><root><item>Test</item></root>"#;
        temp_file.write_all(test_content.as_bytes())?;
        temp_file.flush()?;

        let path = temp_file.path().to_string_lossy().to_string();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1728
  - stubby variable name
  - temp_file

```rust
        temp_file.flush()?;

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1741
  - stubby variable name
  - temp_file

```rust
    #[test]
    fn test_yaml_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::with_suffix(".yaml")?;
        let test_content = "key: value\nlist:\n  - item1\n  - item2";
        temp_file.write_all(test_content.as_bytes())?;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1743
  - stubby variable name
  - temp_file

```rust
        let mut temp_file = NamedTempFile::with_suffix(".yaml")?;
        let test_content = "key: value\nlist:\n  - item1\n  - item2";
        temp_file.write_all(test_content.as_bytes())?;
        temp_file.flush()?;

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1744
  - stubby variable name
  - temp_file

```rust
        let test_content = "key: value\nlist:\n  - item1\n  - item2";
        temp_file.write_all(test_content.as_bytes())?;
        temp_file.flush()?;

        let path = temp_file.path().to_string_lossy().to_string();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1746
  - stubby variable name
  - temp_file

```rust
        temp_file.flush()?;

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1759
  - stubby variable name
  - temp_file

```rust
    #[test]
    fn test_image_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::with_suffix(".png")?;
        let png_header: Vec<u8> = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        temp_file.write_all(&png_header)?;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1761
  - stubby variable name
  - temp_file

```rust
        let mut temp_file = NamedTempFile::with_suffix(".png")?;
        let png_header: Vec<u8> = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        temp_file.write_all(&png_header)?;
        temp_file.flush()?;

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1762
  - stubby variable name
  - temp_file

```rust
        let png_header: Vec<u8> = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        temp_file.write_all(&png_header)?;
        temp_file.flush()?;

        let path = temp_file.path().to_string_lossy().to_string();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1764
  - stubby variable name
  - temp_file

```rust
        temp_file.flush()?;

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1778
  - stubby variable name
  - temp_file

```rust
    #[test]
    fn test_file_size_limit() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(b"small content")?;
        temp_file.flush()?;
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1779
  - stubby variable name
  - temp_file

```rust
    fn test_file_size_limit() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(b"small content")?;
        temp_file.flush()?;

```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1780
  - stubby variable name
  - temp_file

```rust
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(b"small content")?;
        temp_file.flush()?;

        let path = temp_file.path().to_string_lossy().to_string();
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 1782
  - stubby variable name
  - temp_file

```rust
        temp_file.flush()?;

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 198
  - Actual
  - 

```rust
        /// Configured threshold value that was exceeded
        threshold: f64,
        /// Actual measured value that exceeded the threshold
        actual: f64,
        /// When the threshold breach was detected
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 758
  - Fall back
  - 

```rust
                    ),
                    _ => {
                        // Fall back to extension-based detection
                        Self::detect_format_from_extension(file_path)
                    }
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.


- Line 808
  - actual
  - 

```rust
        };
        
        // Create the document with actual content
        Document {
            data,
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 1528: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
    #[test]
    fn test_load_text_file() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_content = "Hello, this is a test text file!";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1530: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_content = "Hello, this is a test text file!";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1531: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let test_content = "Hello, this is a test text file!";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1544: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
    #[test]
    fn test_load_json_file() {
        let mut temp_file = NamedTempFile::with_suffix(".json").expect("Failed to create temp file");
        let test_content = r#"{"key": "value", "number": 42}"#;
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1546: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let mut temp_file = NamedTempFile::with_suffix(".json").expect("Failed to create temp file");
        let test_content = r#"{"key": "value", "number": 42}"#;
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1547: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let test_content = r#"{"key": "value", "number": 42}"#;
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1561: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
    #[test]
    fn test_load_html_file() {
        let mut temp_file = NamedTempFile::with_suffix(".html").expect("Failed to create temp file");
        let test_content = "<html><body><h1>Test</h1></body></html>";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1563: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let mut temp_file = NamedTempFile::with_suffix(".html").expect("Failed to create temp file");
        let test_content = "<html><body><h1>Test</h1></body></html>";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1564: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let test_content = "<html><body><h1>Test</h1></body></html>";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1578: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
    #[test]
    fn test_load_markdown_file() {
        let mut temp_file = NamedTempFile::with_suffix(".md").expect("Failed to create temp file");
        let test_content = "# Heading\n\nThis is **markdown** content.";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1580: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let mut temp_file = NamedTempFile::with_suffix(".md").expect("Failed to create temp file");
        let test_content = "# Heading\n\nThis is **markdown** content.";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1581: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let test_content = "# Heading\n\nThis is **markdown** content.";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1595: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
    #[test]
    fn test_load_binary_file() {
        let mut temp_file = NamedTempFile::with_suffix(".pdf").expect("Failed to create temp file");
        let binary_data: Vec<u8> = vec![0x25, 0x50, 0x44, 0x46, 0x2D];
        temp_file.write_all(&binary_data).expect("Failed to write");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1597: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let mut temp_file = NamedTempFile::with_suffix(".pdf").expect("Failed to create temp file");
        let binary_data: Vec<u8> = vec![0x25, 0x50, 0x44, 0x46, 0x2D];
        temp_file.write_all(&binary_data).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1598: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let binary_data: Vec<u8> = vec![0x25, 0x50, 0x44, 0x46, 0x2D];
        temp_file.write_all(&binary_data).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1613: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
    #[test]
    fn test_utf8_fallback() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let invalid_utf8: Vec<u8> = vec![0xFF, 0xFE, 0xFD, 0x80, 0x81];
        temp_file.write_all(&invalid_utf8).expect("Failed to write");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1615: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let invalid_utf8: Vec<u8> = vec![0xFF, 0xFE, 0xFD, 0x80, 0x81];
        temp_file.write_all(&invalid_utf8).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1616: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let invalid_utf8: Vec<u8> = vec![0xFF, 0xFE, 0xFD, 0x80, 0x81];
        temp_file.write_all(&invalid_utf8).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1639: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
    #[test]
    fn test_directory_not_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let path = temp_dir.path().to_string_lossy().to_string();
        let context = create_test_context(path);
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1650: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
    #[test]
    fn test_extension_fallback() {
        let mut temp_file = NamedTempFile::with_suffix(".custom").expect("Failed to create temp file");
        let test_content = "Custom file content";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1652: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let mut temp_file = NamedTempFile::with_suffix(".custom").expect("Failed to create temp file");
        let test_content = "Custom file content";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1653: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let test_content = "Custom file content";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1666: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
    #[test]
    fn test_metadata_preservation() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_content = "Test content for metadata";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1668: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_content = "Test content for metadata";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 1669: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
        let test_content = "Test content for metadata";
        temp_file.write_all(test_content.as_bytes()).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 1507: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/context/provider.rs` (line 1507)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::context::{CandleContentFormat, CandleDocumentMediaType};
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1527: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/context/provider.rs` (line 1527)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_load_text_file() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_content = "Hello, this is a test text file!";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1543: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/context/provider.rs` (line 1543)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_load_json_file() {
        let mut temp_file = NamedTempFile::with_suffix(".json").expect("Failed to create temp file");
        let test_content = r#"{"key": "value", "number": 42}"#;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1560: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/context/provider.rs` (line 1560)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_load_html_file() {
        let mut temp_file = NamedTempFile::with_suffix(".html").expect("Failed to create temp file");
        let test_content = "<html><body><h1>Test</h1></body></html>";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1577: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/context/provider.rs` (line 1577)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_load_markdown_file() {
        let mut temp_file = NamedTempFile::with_suffix(".md").expect("Failed to create temp file");
        let test_content = "# Heading\n\nThis is **markdown** content.";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1594: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/context/provider.rs` (line 1594)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_load_binary_file() {
        let mut temp_file = NamedTempFile::with_suffix(".pdf").expect("Failed to create temp file");
        let binary_data: Vec<u8> = vec![0x25, 0x50, 0x44, 0x46, 0x2D];
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1612: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/context/provider.rs` (line 1612)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_utf8_fallback() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let invalid_utf8: Vec<u8> = vec![0xFF, 0xFE, 0xFD, 0x80, 0x81];
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1629: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/context/provider.rs` (line 1629)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_missing_file() {
        let context = create_test_context("/path/to/nonexistent/file.txt".to_string());
        let document = CandleStreamingContextProcessor::load_file_document(&context);
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1638: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/context/provider.rs` (line 1638)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_directory_not_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let path = temp_dir.path().to_string_lossy().to_string();
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1649: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/context/provider.rs` (line 1649)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_extension_fallback() {
        let mut temp_file = NamedTempFile::with_suffix(".custom").expect("Failed to create temp file");
        let test_content = "Custom file content";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1665: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/context/provider.rs` (line 1665)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_metadata_preservation() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_content = "Test content for metadata";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1704: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/context/provider.rs` (line 1704)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_csv_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::with_suffix(".csv")?;
        let test_content = "name,age,city\nAlice,30,NYC\nBob,25,LA";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1722: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/context/provider.rs` (line 1722)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_xml_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::with_suffix(".xml")?;
        let test_content = r#"<?xml version="1.0"?><root><item>Test</item></root>"#;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1740: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/context/provider.rs` (line 1740)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_yaml_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::with_suffix(".yaml")?;
        let test_content = "key: value\nlist:\n  - item1\n  - item2";
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1758: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/context/provider.rs` (line 1758)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_image_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::with_suffix(".png")?;
        let png_header: Vec<u8> = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  


### Line 1777: `#[test]`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/candle/src/domain/context/provider.rs` (line 1777)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

    #[test]
    fn test_file_size_limit() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(b"small content")?;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym