# Task 011: Remove Dead Code from Qwen3 Module

## Priority: LOW
## Status: NOT STARTED
## Created: 2025-10-19

## Problem Statement

The qwen3_quantized.rs module contains unused structs and functions that should be removed to improve code clarity and maintainability.

## Dead Code

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/text_to_text/qwen3_quantized.rs`

### 1. CandleQwenCompletionRequest (Lines 493-502)

```rust
/// Qwen3 Quantized completion request format for HTTP API compatibility
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]  // ⚠️ Never used anywhere
struct CandleQwenCompletionRequest {
    prompt: String,
    temperature: f64,
    max_tokens: u64,
    stream: bool,
    model: String,
}
```

**Issue**: This struct is never instantiated, never referenced, and has no purpose.

### 2. validate_model_path (Lines 504-521)

```rust
/// Validate that the model path exists and is accessible
///
/// # Errors
/// Returns error if the path does not exist or is not accessible
#[allow(dead_code)]  // ⚠️ Never called anywhere
fn validate_model_path(path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let model_path = Path::new(path);

    if !model_path.exists() {
        return Err(format!("Model path does not exist: {}", path).into());
    }

    if !model_path.is_dir() && !model_path.is_file() {
        return Err(format!("Model path is neither file nor directory: {}", path).into());
    }

    Ok(())
}
```

**Issue**: 
- Function is never called
- We don't store model paths anymore (struct has no path fields)
- Validation happens naturally when files are opened

## Why This Exists

Likely **leftover** from earlier implementation that:
1. Had an HTTP API layer (hence the request struct)
2. Stored paths in struct fields (hence the validation)

Both were removed during refactoring but the code wasn't cleaned up.

## Fix

**Delete lines 493-521** completely.

## Impact

**Before**:
- 29 lines of dead code
- Confusing for maintainers ("why is this here?")
- Compiler warnings suppressed with `#[allow(dead_code)]`

**After**:
- Cleaner module
- Clear intent
- No confusion

## Verification

After removal:
1. Code still compiles
2. All tests pass
3. No references to `CandleQwenCompletionRequest` or `validate_model_path`

Search for usage:
```bash
rg "CandleQwenCompletionRequest" packages/candle/
rg "validate_model_path" packages/candle/
```

Should return **zero results** after checking this is truly unused.

## Priority Justification

**LOW priority** because:
- No functional impact
- No performance impact
- Just code cleanliness

Can be done as part of general cleanup pass.
