# Issue: Unsafe Memory-Mapped Files Without Validation

## Severity: MEDIUM
**Impact**: Potential crashes, security risk from corrupted files

## Location
- `/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/stella/base.rs:153-165`
- `/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/stella/loaded.rs:154-166`

## Problem Description

Model weights are loaded using `unsafe` memory-mapped files without validation:

```rust
let base_vb = unsafe {
    VarBuilder::from_mmaped_safetensors(&[base_weights], dtype, &device)
        .map_err(|e| format!("Failed to load base model weights: {}", e))?
};

let embed_vb = unsafe {
    VarBuilder::from_mmaped_safetensors(
        &[projection_head],
        DType::F32,
        &device,
    )
    .map_err(|e| format!("Failed to load projection head weights: {}", e))?
};
```

## Why It's Unsafe

Memory-mapping (`mmap`) directly maps file contents into process memory:
- **No validation**: File could be corrupted, truncated, or malicious
- **Trust required**: Assumes file format is valid SafeTensors
- **Crash risk**: Invalid data can cause segfaults during tensor operations

## Potential Issues

1. **Corrupted Downloads**: If HuggingFace download was interrupted, file might be incomplete
2. **Disk Corruption**: Bit flips, filesystem errors
3. **Malicious Files**: If cache is compromised, attacker could inject invalid data
4. **Version Mismatches**: Old cached file with incompatible format

## Current Error Handling

The `map_err` only catches errors during mmap setup, not during actual tensor access:

```rust
// This catches mmap errors:
.map_err(|e| format!("Failed to load base model weights: {}", e))?

// But this can still segfault later:
let embeddings = model.forward_norm(&input_ids, &attention_mask)?;
//                    ↑ Accesses mmapped memory - no validation!
```

## Safer Approach

### Option 1: Validate SafeTensors Format

```rust
// Before mmapping, validate the file
let base_vb = unsafe {
    // First, check if file is valid SafeTensors
    validate_safetensors_file(&base_weights)?;
    
    VarBuilder::from_mmaped_safetensors(&[base_weights], dtype, &device)
        .map_err(|e| format!("Failed to load base model weights: {}", e))?
};

fn validate_safetensors_file(path: &Path) -> Result<(), String> {
    // Read header to verify format
    let file = std::fs::File::open(path)
        .map_err(|e| format!("Cannot open file: {}", e))?;
    
    // SafeTensors has 8-byte header with JSON length
    let mut header = [0u8; 8];
    file.read_exact(&mut header)
        .map_err(|e| format!("Cannot read header: {}", e))?;
    
    let json_len = u64::from_le_bytes(header);
    if json_len > 100_000_000 {  // 100MB is suspiciously large for metadata
        return Err("Invalid SafeTensors header".to_string());
    }
    
    Ok(())
}
```

### Option 2: Use Non-Unsafe Loading

```rust
// Load into memory instead of mmap (slower but safer)
let base_vb = VarBuilder::from_safetensors(&[base_weights], dtype, &device)
    .map_err(|e| format!("Failed to load base model weights: {}", e))?;
```

**Pros**:
- No unsafe code
- Full validation during load
- Catches corruption early

**Cons**:
- Slower (copies entire file into memory)
- Higher memory usage (2x during load)

### Option 3: Checksum Validation

```rust
// Verify file integrity before mmap
verify_file_checksum(&base_weights, expected_sha256)?;

let base_vb = unsafe {
    VarBuilder::from_mmaped_safetensors(&[base_weights], dtype, &device)?
};
```

## Recommendation

**Combine Option 1 + Option 3**:
1. Validate SafeTensors header format
2. Verify SHA256 checksum (HuggingFace provides these)
3. Only then use unsafe mmap

This provides defense-in-depth:
- Header validation catches format errors
- Checksum catches corruption/tampering
- Mmap provides performance

## Related Security Consideration

The `huggingface_file()` method should also verify checksums during download. Check if it already does this.
